use super::video_codec_common::{VideoCodecType, VideoFrameType, VideoRotation};
use super::video_codec_specifics::{
    RTPVideoHeaderCodecSpecifics, RTPVideoHeaderH264, RTPVideoHeaderVP8, RTPVideoHeaderVP9,
};
use crate::ref_count::{FrameTransformerHandle, TransformedFrameCallbackHandle};
use crate::{CxxString, Result, ScopedRef, ffi};
use std::collections::HashMap;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::sync::Mutex;

/// ビデオコンテンツ種別を表す。
///
/// libwebrtc の `VideoContentType` に対応する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoContentType {
    /// 未指定。
    Unspecified,
    /// スクリーン共有。
    Screenshare,
    Unknown(i32),
}

impl VideoContentType {
    pub(crate) fn from_raw(value: i32) -> Self {
        if value == unsafe { ffi::webrtc_VideoContentType_UNSPECIFIED } {
            Self::Unspecified
        } else if value == unsafe { ffi::webrtc_VideoContentType_SCREENSHARE } {
            Self::Screenshare
        } else {
            Self::Unknown(value)
        }
    }

    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Unspecified => unsafe { ffi::webrtc_VideoContentType_UNSPECIFIED },
            Self::Screenshare => unsafe { ffi::webrtc_VideoContentType_SCREENSHARE },
            Self::Unknown(v) => v,
        }
    }
}

/// デコードターゲットへの依存種別を表す。
///
/// libwebrtc の `DecodeTargetIndication` に対応する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeTargetIndication {
    /// このデコードターゲットには存在しない。
    NotPresent,
    /// 破棄可能。
    Discardable,
    /// 切替可能。
    Switch,
    /// 必須。
    Required,
    Unknown(i32),
}

impl DecodeTargetIndication {
    pub(crate) fn from_raw(value: i32) -> Self {
        if value == unsafe { ffi::webrtc_DecodeTargetIndication_NotPresent } {
            Self::NotPresent
        } else if value == unsafe { ffi::webrtc_DecodeTargetIndication_Discardable } {
            Self::Discardable
        } else if value == unsafe { ffi::webrtc_DecodeTargetIndication_Switch } {
            Self::Switch
        } else if value == unsafe { ffi::webrtc_DecodeTargetIndication_Required } {
            Self::Required
        } else {
            Self::Unknown(value)
        }
    }

    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::NotPresent => unsafe { ffi::webrtc_DecodeTargetIndication_NotPresent },
            Self::Discardable => unsafe { ffi::webrtc_DecodeTargetIndication_Discardable },
            Self::Switch => unsafe { ffi::webrtc_DecodeTargetIndication_Switch },
            Self::Required => unsafe { ffi::webrtc_DecodeTargetIndication_Required },
            Self::Unknown(v) => v,
        }
    }
}

/// RTP timestamp の種別を表す。
///
/// libwebrtc の `RtpTimestampInfo` (std::variant<RtpTimestampWithOffset,
/// RtpTimestampWithoutOffset>) に対応する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtpTimestampInfo {
    /// 完全な RTP timestamp が既知の場合。
    WithOffset(u32),
    /// RTP timestamp のオフセットが不明な場合。
    WithoutOffset(u32),
}

/// フレームの方向を表す。
///
/// libwebrtc の `TransformableFrameInterface::Direction` に対応する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformableFrameDirection {
    /// 方向が不明。
    Unknown,
    /// 受信側のフレーム。
    Receiver,
    /// 送信側のフレーム。
    Sender,
}

impl TransformableFrameDirection {
    fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Receiver,
            2 => Self::Sender,
            _ => Self::Unknown,
        }
    }
}

/// エンコード済みフレームの変換処理を実装するトレイト。
///
/// [FrameTransformer::new_with_handler] に渡し、送信時はエンコーダーと
/// パケタイザーの間、受信時はデパケタイザーとデコーダーの間で
/// エンコード済みフレームを加工できる。このトレイトの実装は
/// libwebrtc の呼び出しスレッド上で同期実行されるため、
/// 重い処理は呼び出しスレッド (エンコーダー・ネットワークスレッド) を
/// ブロックすることに注意する。
///
/// 同一の [FrameTransformer] を複数の送受信ストリームで共有すると、
/// libwebrtc は `transform` を並行に呼び得る (送信はエンコーダースレッド、
/// 受信はネットワークスレッド)。そのためこのトレイトは `Sync` を要求し、
/// `transform` は `&self` で呼ばれる。共有状態を持つ場合はハンドラ側で
/// `Mutex` や `Atomic` などにより同期すること。
pub trait FrameTransformerHandler: Send + Sync {
    /// エンコード済みフレームを変換する。
    ///
    /// `frame` は所有権を持って渡され、データは [TransformableFrame::get_data] で
    /// 取得し [TransformableFrame::set_data] で書き換える。
    /// 戻り値が `Some` の場合は変換後フレームが送信され、
    /// `None` の場合はフレームがドロップされる。
    fn transform(&self, frame: TransformableFrame) -> Option<TransformableFrame>;
}

/// 登録された TransformedFrameCallback の集合。
struct FrameTransformerCallbacks {
    /// ssrc ごとに登録された delegate。
    sink_callbacks: HashMap<u32, ScopedRef<TransformedFrameCallbackHandle>>,
    /// ssrc 非対応の単一 delegate。
    callback: Option<ScopedRef<TransformedFrameCallbackHandle>>,
}

impl FrameTransformerCallbacks {
    fn new() -> Self {
        Self {
            sink_callbacks: HashMap::new(),
            callback: None,
        }
    }
}

struct FrameTransformerHandlerState {
    handler: Box<dyn FrameTransformerHandler>,
    // 送信側では delegate の登録 (Register/Unregister) がセットアップスレッド、
    // フレームの配送 (Transform) がエンコーダースレッドで実行され、
    // 同一 transformer を複数ストリームで共有すると並行呼び出しとなり得る。
    // callback の登録状態は Mutex で保護する。
    callbacks: Mutex<FrameTransformerCallbacks>,
}

// コールバックは共有参照 (&) で state にアクセスし、handler は &self
// (ハンドラ側で同期を要求)、callbacks は Mutex で保護されるため
// Send / Sync として扱う。内部の ScopedRef はロック下でのみアクセスされ、
// webrtc の参照カウントはアトミックでスレッドセーフである。
unsafe impl Send for FrameTransformerHandlerState {}
unsafe impl Sync for FrameTransformerHandlerState {}

unsafe extern "C" fn frame_transformer_transform(
    frame: *mut ffi::webrtc_TransformableFrameInterface_unique,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "frame_transformer_transform: user_data is null"
    );
    // frame の所有権は C++ 側から引き継いでいる (unique_ptr を release 済み)。
    let state = unsafe { &*(user_data as *const FrameTransformerHandlerState) };
    let frame = unsafe {
        TransformableFrame::from_unique_ptr(NonNull::new(frame).expect("BUG: frame が null"))
    };
    if let Some(frame) = state.handler.transform(frame) {
        deliver_transformed_frame(state, frame);
    }
    // None の場合は frame が破棄される (Drop)。
}

/// 変換後フレームを ssrc に対応する delegate へ配送する。
///
/// 対応する delegate が無い場合はフレームが破棄される (Drop)。
fn deliver_transformed_frame(state: &FrameTransformerHandlerState, frame: TransformableFrame) {
    let ssrc = frame.get_ssrc();
    let callback = {
        let guard = state.callbacks.lock().expect("callbacks lock poisoned");
        guard
            .sink_callbacks
            .get(&ssrc)
            .cloned()
            .or_else(|| guard.callback.clone())
    };
    match callback {
        Some(callback) => unsafe {
            ffi::webrtc_TransformedFrameCallback_OnTransformedFrame(
                callback.as_ptr(),
                frame.into_raw_unique(),
            )
        },
        None => {
            // 対応する delegate が無い場合は frame が破棄される (Drop)。
        }
    }
}

unsafe extern "C" fn frame_transformer_register_transformed_frame_callback(
    callback: *mut ffi::webrtc_TransformedFrameCallback_refcounted,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "frame_transformer_register_transformed_frame_callback: user_data is null"
    );
    let state = unsafe { &*(user_data as *const FrameTransformerHandlerState) };
    let callback = NonNull::new(callback).expect("BUG: callback が null");
    let callback = ScopedRef::<TransformedFrameCallbackHandle>::from_raw(callback);
    state
        .callbacks
        .lock()
        .expect("callbacks lock poisoned")
        .callback = Some(callback);
}

unsafe extern "C" fn frame_transformer_register_transformed_frame_sink_callback(
    callback: *mut ffi::webrtc_TransformedFrameCallback_refcounted,
    ssrc: u32,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "frame_transformer_register_transformed_frame_sink_callback: user_data is null"
    );
    let state = unsafe { &*(user_data as *const FrameTransformerHandlerState) };
    let callback = NonNull::new(callback).expect("BUG: callback が null");
    let callback = ScopedRef::<TransformedFrameCallbackHandle>::from_raw(callback);
    state
        .callbacks
        .lock()
        .expect("callbacks lock poisoned")
        .sink_callbacks
        .insert(ssrc, callback);
}

unsafe extern "C" fn frame_transformer_unregister_transformed_frame_callback(
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "frame_transformer_unregister_transformed_frame_callback: user_data is null"
    );
    let state = unsafe { &*(user_data as *const FrameTransformerHandlerState) };
    state
        .callbacks
        .lock()
        .expect("callbacks lock poisoned")
        .callback = None;
}

unsafe extern "C" fn frame_transformer_unregister_transformed_frame_sink_callback(
    ssrc: u32,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "frame_transformer_unregister_transformed_frame_sink_callback: user_data is null"
    );
    let state = unsafe { &*(user_data as *const FrameTransformerHandlerState) };
    state
        .callbacks
        .lock()
        .expect("callbacks lock poisoned")
        .sink_callbacks
        .remove(&ssrc);
}

unsafe extern "C" fn frame_transformer_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "frame_transformer_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut FrameTransformerHandlerState) };
}

/// webrtc::FrameTransformerInterface のラッパー。
///
/// エンコード済みフレームを [FrameTransformerHandler] で変換して
/// libwebrtc へ返す。生成したフレーム変換は
/// [RtpSender::set_frame_transformer] または [RtpReceiver::set_frame_transformer]
/// で適用する。
pub struct FrameTransformer {
    raw_ref: ScopedRef<FrameTransformerHandle>,
}

unsafe impl Send for FrameTransformer {}

impl FrameTransformer {
    /// [FrameTransformerHandler] を実行する FrameTransformer を生成する。
    pub fn new_with_handler(handler: Box<dyn FrameTransformerHandler>) -> Self {
        let state = Box::new(FrameTransformerHandlerState {
            handler,
            callbacks: Mutex::new(FrameTransformerCallbacks::new()),
        });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_FrameTransformerInterface_cbs {
            Transform: Some(frame_transformer_transform),
            RegisterTransformedFrameCallback: Some(
                frame_transformer_register_transformed_frame_callback,
            ),
            RegisterTransformedFrameSinkCallback: Some(
                frame_transformer_register_transformed_frame_sink_callback,
            ),
            UnregisterTransformedFrameCallback: Some(
                frame_transformer_unregister_transformed_frame_callback,
            ),
            UnregisterTransformedFrameSinkCallback: Some(
                frame_transformer_unregister_transformed_frame_sink_callback,
            ),
            OnDestroy: Some(frame_transformer_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_FrameTransformerInterface_new(&cbs, user_data) };
        let raw = NonNull::new(raw)
            .expect("BUG: webrtc_FrameTransformerInterface_new が null を返しました");
        let raw_ref = ScopedRef::<FrameTransformerHandle>::from_raw(raw);
        Self { raw_ref }
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_FrameTransformerInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }
}

/// webrtc::TransformableFrameInterface の所有ラッパー。
///
/// エンコード済みフレームの所有権 (unique_ptr) を保持する。
/// [FrameTransformerHandler] にはこの型を渡し、変換後の配送と破棄は
/// libwebrtc 側で行う。
pub struct TransformableFrame {
    raw_unique: NonNull<ffi::webrtc_TransformableFrameInterface_unique>,
}

unsafe impl Send for TransformableFrame {}

impl TransformableFrame {
    /// # Safety
    /// `raw_unique` は有効な `webrtc_TransformableFrameInterface_unique` を指し、
    /// 呼び出し元が所有権を持っている必要があります。
    unsafe fn from_unique_ptr(
        raw_unique: NonNull<ffi::webrtc_TransformableFrameInterface_unique>,
    ) -> Self {
        Self { raw_unique }
    }

    fn as_ptr(&self) -> *mut ffi::webrtc_TransformableFrameInterface {
        unsafe { ffi::webrtc_TransformableFrameInterface_unique_get(self.raw_unique.as_ptr()) }
    }

    fn into_raw_unique(self) -> *mut ffi::webrtc_TransformableFrameInterface_unique {
        let ptr = self.raw_unique.as_ptr();
        std::mem::forget(self);
        ptr
    }

    /// フレームのペイロードデータを返す。
    ///
    /// 返されたスライスは次の非 const メソッド呼び出しまで有効なため、
    /// 借用の範囲内でのみ利用すること。
    pub fn get_data(&self) -> &[u8] {
        let mut data: *const u8 = std::ptr::null();
        let mut len = 0;
        unsafe {
            ffi::webrtc_TransformableFrameInterface_GetData(self.as_ptr(), &mut data, &mut len)
        };
        if len == 0 {
            return &[];
        }
        // ライフタイムは &self に束縛され、次の非 const メソッド呼び出しを
        // 借用規則で防止する。
        unsafe { std::slice::from_raw_parts(data, len) }
    }

    /// フレームのペイロードデータを書き換える。
    pub fn set_data(&mut self, data: &[u8]) {
        unsafe {
            ffi::webrtc_TransformableFrameInterface_SetData(
                self.as_ptr(),
                data.as_ptr(),
                data.len(),
            )
        };
    }

    /// ペイロードタイプを返す。
    pub fn get_payload_type(&self) -> u8 {
        unsafe { ffi::webrtc_TransformableFrameInterface_GetPayloadType(self.as_ptr()) }
    }

    /// ペイロードタイプを変更できるかどうかを返す。
    pub fn can_set_payload_type(&self) -> bool {
        unsafe { ffi::webrtc_TransformableFrameInterface_CanSetPayloadType(self.as_ptr()) != 0 }
    }

    /// ペイロードタイプを設定する。
    pub fn set_payload_type(&mut self, payload_type: u8) {
        unsafe {
            ffi::webrtc_TransformableFrameInterface_SetPayloadType(self.as_ptr(), payload_type)
        };
    }

    /// このフレームの SSRC を返す。
    pub fn get_ssrc(&self) -> u32 {
        unsafe { ffi::webrtc_TransformableFrameInterface_GetSsrc(self.as_ptr()) }
    }

    /// RTP timestamp を返す。
    ///
    /// libwebrtc の `GetTimestamp` は deprecated のため、
    /// オフセットの有無を区別できる `GetRtpTimestampInfo` を使用する。
    pub fn get_rtp_timestamp_info(&self) -> RtpTimestampInfo {
        let raw_unique =
            unsafe { ffi::webrtc_TransformableFrameInterface_GetRtpTimestampInfo(self.as_ptr()) };
        let raw_unique = NonNull::new(raw_unique).expect(
            "BUG: webrtc_TransformableFrameInterface_GetRtpTimestampInfo が null を返しました",
        );
        let raw = unsafe { ffi::webrtc_RtpTimestampInfo_unique_get(raw_unique.as_ptr()) };
        let raw = NonNull::new(raw)
            .expect("BUG: webrtc_RtpTimestampInfo_unique_get が null を返しました");
        let index = unsafe { ffi::webrtc_RtpTimestampInfo_index(raw.as_ptr()) };
        let value = if index == 0 {
            let alt =
                unsafe { ffi::webrtc_RtpTimestampInfo_get_RtpTimestampWithOffset(raw.as_ptr()) };
            let alt = NonNull::new(alt)
                .expect("BUG: index が RtpTimestampWithOffset なのにアクセサが null を返しました");
            unsafe { ffi::webrtc_RtpTimestampWithOffset_get_value(alt.as_ptr()) }
        } else {
            let alt =
                unsafe { ffi::webrtc_RtpTimestampInfo_get_RtpTimestampWithoutOffset(raw.as_ptr()) };
            let alt = NonNull::new(alt).expect(
                "BUG: index が RtpTimestampWithoutOffset なのにアクセサが null を返しました",
            );
            unsafe { ffi::webrtc_RtpTimestampWithoutOffset_get_value(alt.as_ptr()) }
        };
        unsafe { ffi::webrtc_RtpTimestampInfo_unique_delete(raw_unique.as_ptr()) };
        if index == 0 {
            RtpTimestampInfo::WithOffset(value)
        } else {
            RtpTimestampInfo::WithoutOffset(value)
        }
    }

    /// RTP timestamp を設定する。
    pub fn set_rtp_timestamp(&mut self, rtp_timestamp_with_offset: u32) {
        unsafe {
            ffi::webrtc_TransformableFrameInterface_SetRTPTimestamp(
                self.as_ptr(),
                rtp_timestamp_with_offset,
            )
        };
    }

    /// フレームの方向を返す。
    pub fn get_direction(&self) -> TransformableFrameDirection {
        let value = unsafe { ffi::webrtc_TransformableFrameInterface_GetDirection(self.as_ptr()) };
        TransformableFrameDirection::from_raw(value)
    }

    /// フレームの MIME type を返す。
    ///
    /// 例: `"video/VP8"`。
    pub fn get_mime_type(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_TransformableFrameInterface_GetMimeType(self.as_ptr()) };
        let raw = NonNull::new(ptr)
            .expect("BUG: webrtc_TransformableFrameInterface_GetMimeType が null を返しました");
        CxxString::from_unique(raw).to_string()
    }

    /// パケットがネットワークで最初に観測されたタイムスタンプ (マイクロ秒) を返す。
    ///
    /// 受信フレームでのみ定義される。
    pub fn receive_time(&self) -> Option<i64> {
        let mut has = 0;
        let mut timestamp_us = 0;
        unsafe {
            ffi::webrtc_TransformableFrameInterface_ReceiveTime(
                self.as_ptr(),
                &mut has,
                &mut timestamp_us,
            )
        };
        if has == 0 { None } else { Some(timestamp_us) }
    }

    /// プレゼンテーション用のタイムスタンプ (マイクロ秒) を返す。
    ///
    /// deprecated の `GetCaptureTimeIdentifier` の後継。
    pub fn get_presentation_timestamp(&self) -> Option<i64> {
        let mut has = 0;
        let mut timestamp_us = 0;
        unsafe {
            ffi::webrtc_TransformableFrameInterface_GetPresentationTimestamp(
                self.as_ptr(),
                &mut has,
                &mut timestamp_us,
            )
        };
        if has == 0 { None } else { Some(timestamp_us) }
    }

    /// キャプチャシステム内でフレームがキャプチャされた時刻 (マイクロ秒) を返す。
    pub fn capture_time(&self) -> Option<i64> {
        let mut has = 0;
        let mut timestamp_us = 0;
        unsafe {
            ffi::webrtc_TransformableFrameInterface_CaptureTime(
                self.as_ptr(),
                &mut has,
                &mut timestamp_us,
            )
        };
        if has == 0 { None } else { Some(timestamp_us) }
    }

    /// キャプチャ時間を変更できるかどうかを返す。
    pub fn can_set_capture_time(&self) -> bool {
        unsafe { ffi::webrtc_TransformableFrameInterface_CanSetCaptureTime(self.as_ptr()) != 0 }
    }

    /// キャプチャシステム内でフレームがキャプチャされた時刻 (マイクロ秒) を設定する。
    ///
    /// `None` を指定するとキャプチャ時間を未設定にする。
    pub fn set_capture_time(&mut self, capture_time: Option<i64>) {
        let (has, timestamp_us) = match capture_time {
            Some(v) => (1, v),
            None => (0, 0),
        };
        unsafe {
            ffi::webrtc_TransformableFrameInterface_SetCaptureTime(self.as_ptr(), has, timestamp_us)
        };
    }

    /// 送信側システムとキャプチャ側システムのクロックオフセット (マイクロ秒) を返す。
    ///
    /// absolute capture timestamp ヘッダー拡張が有効な場合のみ利用できる。
    pub fn sender_capture_time_offset(&self) -> Option<i64> {
        let mut has = 0;
        let mut delta_us = 0;
        unsafe {
            ffi::webrtc_TransformableFrameInterface_SenderCaptureTimeOffset(
                self.as_ptr(),
                &mut has,
                &mut delta_us,
            )
        };
        if has == 0 { None } else { Some(delta_us) }
    }
}

impl Drop for TransformableFrame {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_TransformableFrameInterface_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::TransformableVideoFrameInterface の所有ラッパー。
///
/// [TransformableFrame] から [TryFrom] で変換する。MIME type が `video/` で
/// 始まる場合のみ変換でき、それ以外の場合はフレームをそのまま返す。
pub struct TransformableVideoFrame {
    base: TransformableFrame,
}

unsafe impl Send for TransformableVideoFrame {}

impl TransformableVideoFrame {
    fn as_video_ptr(&self) -> *mut ffi::webrtc_TransformableVideoFrameInterface {
        self.base.as_ptr() as *mut ffi::webrtc_TransformableVideoFrameInterface
    }

    /// 基底フレームへ戻す。
    pub fn into_base(self) -> TransformableFrame {
        self.base
    }

    /// フレームのペイロードデータを返す。
    ///
    /// 返されたスライスは次の非 const メソッド呼び出しまで有効なため、
    /// 借用の範囲内でのみ利用すること。
    pub fn get_data(&self) -> &[u8] {
        self.base.get_data()
    }

    /// フレームのペイロードデータを書き換える。
    pub fn set_data(&mut self, data: &[u8]) {
        self.base.set_data(data);
    }

    /// ペイロードタイプを返す。
    pub fn get_payload_type(&self) -> u8 {
        self.base.get_payload_type()
    }

    /// ペイロードタイプを変更できるかどうかを返す。
    pub fn can_set_payload_type(&self) -> bool {
        self.base.can_set_payload_type()
    }

    /// ペイロードタイプを設定する。
    pub fn set_payload_type(&mut self, payload_type: u8) {
        self.base.set_payload_type(payload_type);
    }

    /// このフレームの SSRC を返す。
    pub fn get_ssrc(&self) -> u32 {
        self.base.get_ssrc()
    }

    /// RTP timestamp を返す。
    pub fn get_rtp_timestamp_info(&self) -> RtpTimestampInfo {
        self.base.get_rtp_timestamp_info()
    }

    /// RTP timestamp を設定する。
    pub fn set_rtp_timestamp(&mut self, rtp_timestamp_with_offset: u32) {
        self.base.set_rtp_timestamp(rtp_timestamp_with_offset);
    }

    /// フレームの方向を返す。
    pub fn get_direction(&self) -> TransformableFrameDirection {
        self.base.get_direction()
    }

    /// フレームの MIME type を返す。
    pub fn get_mime_type(&self) -> Result<String> {
        self.base.get_mime_type()
    }

    /// パケットがネットワークで最初に観測されたタイムスタンプ (マイクロ秒) を返す。
    pub fn receive_time(&self) -> Option<i64> {
        self.base.receive_time()
    }

    /// プレゼンテーション用のタイムスタンプ (マイクロ秒) を返す。
    pub fn get_presentation_timestamp(&self) -> Option<i64> {
        self.base.get_presentation_timestamp()
    }

    /// キャプチャシステム内でフレームがキャプチャされた時刻 (マイクロ秒) を返す。
    pub fn capture_time(&self) -> Option<i64> {
        self.base.capture_time()
    }

    /// キャプチャ時間を変更できるかどうかを返す。
    pub fn can_set_capture_time(&self) -> bool {
        self.base.can_set_capture_time()
    }

    /// キャプチャシステム内でフレームがキャプチャされた時刻 (マイクロ秒) を設定する。
    ///
    /// `None` を指定するとキャプチャ時間を未設定にする。
    pub fn set_capture_time(&mut self, capture_time: Option<i64>) {
        self.base.set_capture_time(capture_time);
    }

    /// 送信側システムとキャプチャ側システムのクロックオフセット (マイクロ秒) を返す。
    pub fn sender_capture_time_offset(&self) -> Option<i64> {
        self.base.sender_capture_time_offset()
    }

    /// このフレームがキーフレームかどうかを返す。
    pub fn is_key_frame(&self) -> bool {
        unsafe { ffi::webrtc_TransformableVideoFrameInterface_IsKeyFrame(self.as_video_ptr()) != 0 }
    }

    /// RID (RTP Stream ID) を返す。
    pub fn rid(&self) -> Option<Result<String>> {
        let mut has = 0;
        let mut ptr: *mut ffi::std_string_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_TransformableVideoFrameInterface_Rid(
                self.as_video_ptr(),
                &mut has,
                &mut ptr,
            )
        };
        if has == 0 {
            return None;
        }
        let raw = NonNull::new(ptr).expect(
            "BUG: has が 1 なのに webrtc_TransformableVideoFrameInterface_Rid が null を返しました",
        );
        Some(CxxString::from_unique(raw).to_string())
    }

    /// フレームのメタデータを返す。
    pub fn metadata(&self) -> VideoFrameMetadata {
        let raw =
            unsafe { ffi::webrtc_TransformableVideoFrameInterface_Metadata(self.as_video_ptr()) };
        let raw = NonNull::new(raw)
            .expect("BUG: webrtc_TransformableVideoFrameInterface_Metadata が null を返しました");
        VideoFrameMetadata::from_raw(raw)
    }

    /// フレームのメタデータを設定する。
    ///
    /// [VideoFrameMetadata::new] で生成して setter で書き換えたメタデータを
    /// 反映する。SVC のレイヤー切替などで利用する。
    pub fn set_metadata(&mut self, metadata: &VideoFrameMetadata) {
        unsafe {
            ffi::webrtc_TransformableVideoFrameInterface_SetMetadata(
                self.as_video_ptr(),
                metadata.raw.as_ptr(),
            )
        };
    }
}

impl TryFrom<TransformableFrame> for TransformableVideoFrame {
    type Error = TransformableFrame;

    fn try_from(frame: TransformableFrame) -> std::result::Result<Self, Self::Error> {
        let mime = frame
            .get_mime_type()
            .expect("BUG: MIME type の取得に失敗しました");
        if mime.starts_with("video/") {
            Ok(TransformableVideoFrame { base: frame })
        } else {
            Err(frame)
        }
    }
}

/// webrtc::VideoFrameMetadata のラッパー。
///
/// [TransformableVideoFrame::metadata] が返す所有型。
pub struct VideoFrameMetadata {
    raw: NonNull<ffi::webrtc_VideoFrameMetadata>,
}

unsafe impl Send for VideoFrameMetadata {}

impl VideoFrameMetadata {
    /// 新しく生成する。
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoFrameMetadata_new() })
            .expect("BUG: webrtc_VideoFrameMetadata_new が null を返しました");
        Self { raw }
    }

    fn from_raw(raw: NonNull<ffi::webrtc_VideoFrameMetadata>) -> Self {
        Self { raw }
    }

    /// フレームタイプを返す。
    pub fn frame_type(&self) -> VideoFrameType {
        let value = unsafe { ffi::webrtc_VideoFrameMetadata_GetFrameType(self.raw.as_ptr()) };
        VideoFrameType::from_raw(value)
    }

    /// フレームタイプを設定する。
    pub fn set_frame_type(&mut self, frame_type: VideoFrameType) {
        unsafe {
            ffi::webrtc_VideoFrameMetadata_SetFrameType(self.raw.as_ptr(), frame_type.to_raw())
        };
    }

    /// 幅を返す。
    pub fn width(&self) -> u16 {
        unsafe { ffi::webrtc_VideoFrameMetadata_GetWidth(self.raw.as_ptr()) }
    }

    /// 幅を設定する。
    pub fn set_width(&mut self, width: u16) {
        unsafe { ffi::webrtc_VideoFrameMetadata_SetWidth(self.raw.as_ptr(), width) };
    }

    /// 高さを返す。
    pub fn height(&self) -> u16 {
        unsafe { ffi::webrtc_VideoFrameMetadata_GetHeight(self.raw.as_ptr()) }
    }

    /// 高さを設定する。
    pub fn set_height(&mut self, height: u16) {
        unsafe { ffi::webrtc_VideoFrameMetadata_SetHeight(self.raw.as_ptr(), height) };
    }

    /// フレーム ID を返す。
    pub fn frame_id(&self) -> Option<i64> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_VideoFrameMetadata_GetFrameId(self.raw.as_ptr(), &mut has, &mut value)
        };
        if has == 0 { None } else { Some(value) }
    }

    /// フレーム ID を設定する。
    pub fn set_frame_id(&mut self, frame_id: Option<i64>) {
        match frame_id {
            Some(v) => unsafe {
                ffi::webrtc_VideoFrameMetadata_SetFrameId(self.raw.as_ptr(), 1, &v)
            },
            None => unsafe {
                ffi::webrtc_VideoFrameMetadata_SetFrameId(self.raw.as_ptr(), 0, std::ptr::null())
            },
        }
    }

    /// 空間レイヤーインデックスを返す。
    pub fn spatial_index(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrameMetadata_GetSpatialIndex(self.raw.as_ptr()) }
    }

    /// 空間レイヤーインデックスを設定する。
    pub fn set_spatial_index(&mut self, spatial_index: i32) {
        unsafe { ffi::webrtc_VideoFrameMetadata_SetSpatialIndex(self.raw.as_ptr(), spatial_index) };
    }

    /// 時間レイヤーインデックスを返す。
    pub fn temporal_index(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrameMetadata_GetTemporalIndex(self.raw.as_ptr()) }
    }

    /// 時間レイヤーインデックスを設定する。
    pub fn set_temporal_index(&mut self, temporal_index: i32) {
        unsafe {
            ffi::webrtc_VideoFrameMetadata_SetTemporalIndex(self.raw.as_ptr(), temporal_index)
        };
    }

    /// フレームの依存関係 (参照フレーム ID の一覧) を返す。
    pub fn dependencies(&self) -> Option<&[i64]> {
        let mut has = 0;
        let mut data: *const i64 = std::ptr::null();
        let mut len = 0;
        unsafe {
            ffi::webrtc_VideoFrameMetadata_GetDependencies(
                self.raw.as_ptr(),
                &mut has,
                &mut data,
                &mut len,
            )
        };
        if has == 0 {
            return None;
        }
        if len == 0 {
            return Some(&[]);
        }
        // ライフタイムは &self に束縛される。
        Some(unsafe { std::slice::from_raw_parts(data, len) })
    }

    /// フレームの依存関係 (参照フレーム ID の一覧) を設定する。
    pub fn set_dependencies(&mut self, dependencies: Option<&[i64]>) {
        match dependencies {
            Some(v) => unsafe {
                ffi::webrtc_VideoFrameMetadata_SetDependencies(
                    self.raw.as_ptr(),
                    1,
                    v.as_ptr(),
                    v.len(),
                )
            },
            None => unsafe {
                ffi::webrtc_VideoFrameMetadata_SetDependencies(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                    0,
                )
            },
        }
    }

    /// ピクチャ内で最後のフレームかどうかを返す。
    pub fn is_last_frame_in_picture(&self) -> bool {
        unsafe { ffi::webrtc_VideoFrameMetadata_GetIsLastFrameInPicture(self.raw.as_ptr()) != 0 }
    }

    /// ピクチャ内で最後のフレームかどうかを設定する。
    pub fn set_is_last_frame_in_picture(&mut self, is_last_frame_in_picture: bool) {
        unsafe {
            ffi::webrtc_VideoFrameMetadata_SetIsLastFrameInPicture(
                self.raw.as_ptr(),
                if is_last_frame_in_picture { 1 } else { 0 },
            )
        };
    }

    /// サイマルキャストレイヤーのインデックスを返す。
    pub fn simulcast_idx(&self) -> u8 {
        unsafe { ffi::webrtc_VideoFrameMetadata_GetSimulcastIdx(self.raw.as_ptr()) }
    }

    /// サイマルキャストレイヤーのインデックスを設定する。
    pub fn set_simulcast_idx(&mut self, simulcast_idx: u8) {
        unsafe { ffi::webrtc_VideoFrameMetadata_SetSimulcastIdx(self.raw.as_ptr(), simulcast_idx) };
    }

    /// コーデックを返す。
    pub fn codec(&self) -> VideoCodecType {
        let value = unsafe { ffi::webrtc_VideoFrameMetadata_GetCodec(self.raw.as_ptr()) };
        VideoCodecType::from_raw(value)
    }

    /// コーデックを設定する。
    pub fn set_codec(&mut self, codec: VideoCodecType) {
        unsafe { ffi::webrtc_VideoFrameMetadata_SetCodec(self.raw.as_ptr(), codec.to_raw()) };
    }

    /// SSRC を返す。
    pub fn ssrc(&self) -> u32 {
        unsafe { ffi::webrtc_VideoFrameMetadata_GetSsrc(self.raw.as_ptr()) }
    }

    /// SSRC を設定する。
    pub fn set_ssrc(&mut self, ssrc: u32) {
        unsafe { ffi::webrtc_VideoFrameMetadata_SetSsrc(self.raw.as_ptr(), ssrc) };
    }

    /// 回転角度を返す。
    pub fn rotation(&self) -> VideoRotation {
        let value = unsafe { ffi::webrtc_VideoFrameMetadata_GetRotation(self.raw.as_ptr()) };
        VideoRotation::from_raw(value)
    }

    /// 回転角度を設定する。
    pub fn set_rotation(&mut self, rotation: VideoRotation) {
        unsafe { ffi::webrtc_VideoFrameMetadata_SetRotation(self.raw.as_ptr(), rotation.to_raw()) };
    }

    /// コンテンツ種別を返す。
    pub fn content_type(&self) -> VideoContentType {
        let value = unsafe { ffi::webrtc_VideoFrameMetadata_GetContentType(self.raw.as_ptr()) };
        VideoContentType::from_raw(value)
    }

    /// コンテンツ種別を設定する。
    pub fn set_content_type(&mut self, content_type: VideoContentType) {
        unsafe {
            ffi::webrtc_VideoFrameMetadata_SetContentType(self.raw.as_ptr(), content_type.to_raw())
        };
    }

    /// デコードターゲットへの依存種別の一覧を返す。
    ///
    /// 値はコピーして返す。
    pub fn decode_target_indications(&self) -> Vec<DecodeTargetIndication> {
        let mut data: *const i32 = std::ptr::null();
        let mut len = 0;
        unsafe {
            ffi::webrtc_VideoFrameMetadata_GetDecodeTargetIndications(
                self.raw.as_ptr(),
                &mut data,
                &mut len,
            )
        };
        (0..len)
            .map(|i| DecodeTargetIndication::from_raw(unsafe { *data.add(i) }))
            .collect()
    }

    /// デコードターゲットへの依存種別の一覧を設定する。
    pub fn set_decode_target_indications(&mut self, indications: &[DecodeTargetIndication]) {
        let data: Vec<i32> = indications.iter().map(|v| v.to_raw()).collect();
        unsafe {
            ffi::webrtc_VideoFrameMetadata_SetDecodeTargetIndications(
                self.raw.as_ptr(),
                data.as_ptr(),
                data.len(),
            )
        };
    }

    /// CSRC の一覧を返す。
    ///
    /// 値はコピーして返す。
    pub fn csrcs(&self) -> Vec<u32> {
        let raw = unsafe { ffi::webrtc_VideoFrameMetadata_GetCsrcs(self.raw.as_ptr()) };
        let raw = NonNull::new(raw)
            .expect("BUG: webrtc_VideoFrameMetadata_GetCsrcs が null を返しました");
        let len = unsafe { ffi::webrtc_uint32_vector_size(raw.as_ptr()) };
        let mut result = Vec::new();
        for index in 0..len {
            let elem = unsafe { ffi::webrtc_uint32_vector_get(raw.as_ptr(), index) };
            let elem =
                NonNull::new(elem).expect("BUG: webrtc_uint32_vector_get が null を返しました");
            result.push(unsafe { ffi::webrtc_uint32_value(elem.as_ptr()) });
        }
        unsafe { ffi::webrtc_uint32_vector_delete(raw.as_ptr()) };
        result
    }

    /// CSRC の一覧を設定する。
    pub fn set_csrcs(&mut self, csrcs: &[u32]) {
        let raw = NonNull::new(unsafe { ffi::webrtc_uint32_vector_new(0) })
            .expect("BUG: webrtc_uint32_vector_new が null を返しました");
        for value in csrcs {
            unsafe { ffi::webrtc_uint32_vector_push_back_value(raw.as_ptr(), *value) };
        }
        unsafe { ffi::webrtc_VideoFrameMetadata_SetCsrcs(self.raw.as_ptr(), raw.as_ptr()) };
        unsafe { ffi::webrtc_uint32_vector_delete(raw.as_ptr()) };
    }

    /// コーデック固有の RTP ビデオヘッダー情報を返す。
    pub fn rtp_video_header_codec_specifics(&self) -> RTPVideoHeaderCodecSpecifics {
        let raw_unique = unsafe {
            ffi::webrtc_VideoFrameMetadata_GetRTPVideoHeaderCodecSpecifics(self.raw.as_ptr())
        };
        let raw_unique = NonNull::new(raw_unique).expect(
            "BUG: webrtc_VideoFrameMetadata_GetRTPVideoHeaderCodecSpecifics が null を返しました",
        );
        let raw =
            unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_unique_get(raw_unique.as_ptr()) };
        let index = unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_index(raw) };
        let value = match index {
            0 => RTPVideoHeaderCodecSpecifics::None,
            1 => {
                let vp8 =
                    unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderVP8(raw) };
                let vp8 = NonNull::new(vp8)
                    .expect("BUG: index が RTPVideoHeaderVP8 なのにアクセサが null を返しました");
                RTPVideoHeaderCodecSpecifics::VP8(unsafe {
                    RTPVideoHeaderVP8::copy_from_raw(vp8.as_ptr())
                })
            }
            2 => {
                let vp9 =
                    unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderVP9(raw) };
                let vp9 = NonNull::new(vp9)
                    .expect("BUG: index が RTPVideoHeaderVP9 なのにアクセサが null を返しました");
                RTPVideoHeaderCodecSpecifics::VP9(unsafe {
                    RTPVideoHeaderVP9::copy_from_raw(vp9.as_ptr())
                })
            }
            3 => {
                let h264 =
                    unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderH264(raw) };
                let h264 = NonNull::new(h264)
                    .expect("BUG: index が RTPVideoHeaderH264 なのにアクセサが null を返しました");
                RTPVideoHeaderCodecSpecifics::H264(unsafe {
                    RTPVideoHeaderH264::copy_from_raw(h264.as_ptr())
                })
            }
            _ => unreachable!("BUG: 未知の RTPVideoHeaderCodecSpecifics index: {}", index),
        };
        unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_unique_delete(raw_unique.as_ptr()) };
        value
    }

    /// コーデック固有の RTP ビデオヘッダー情報を設定する。
    pub fn set_rtp_video_header_codec_specifics(&mut self, value: RTPVideoHeaderCodecSpecifics) {
        let raw_unique = match value {
            RTPVideoHeaderCodecSpecifics::None => unsafe {
                ffi::webrtc_RTPVideoHeaderCodecSpecifics_new_monostate()
            },
            RTPVideoHeaderCodecSpecifics::VP8(header) => unsafe {
                ffi::webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderVP8(header.as_ptr())
            },
            RTPVideoHeaderCodecSpecifics::VP9(header) => unsafe {
                ffi::webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderVP9(header.as_ptr())
            },
            RTPVideoHeaderCodecSpecifics::H264(header) => unsafe {
                ffi::webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderH264(header.as_ptr())
            },
        };
        let raw_unique = NonNull::new(raw_unique)
            .expect("BUG: webrtc_RTPVideoHeaderCodecSpecifics_new_* が null を返しました");
        let raw =
            unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_unique_get(raw_unique.as_ptr()) };
        unsafe {
            ffi::webrtc_VideoFrameMetadata_SetRTPVideoHeaderCodecSpecifics(self.raw.as_ptr(), raw)
        };
        unsafe { ffi::webrtc_RTPVideoHeaderCodecSpecifics_unique_delete(raw_unique.as_ptr()) };
    }
}

impl Default for VideoFrameMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VideoFrameMetadata {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoFrameMetadata_delete(self.raw.as_ptr()) };
    }
}
