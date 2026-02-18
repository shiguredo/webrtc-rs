use crate::ref_count::{
    AudioTrackHandle, AudioTrackSourceHandle, DataChannelHandle, PeerConnectionFactoryHandle,
    PeerConnectionHandle, RtpReceiverHandle, RtpSenderHandle, RtpTransceiverHandle,
    SetLocalDescriptionObserverHandle, SetRemoteDescriptionObserverHandle, VideoTrackHandle,
};
use crate::{
    AudioDecoderFactory, AudioDeviceModule, AudioEncoderFactory, AudioProcessingBuilder,
    AudioTrack, AudioTrackSource, CxxString, DataChannel, DataChannelInit, Error, IceCandidate,
    IceCandidateRef, MediaStreamTrack, MediaType, RTCStatsReport, Result, RtcError,
    RtcEventLogFactory, RtpCapabilities, RtpReceiver, RtpSender, RtpTransceiver,
    RtpTransceiverInit, ScopedRef, SessionDescription, StringVector, Thread, VideoDecoderFactory,
    VideoEncoderFactory, VideoTrack, VideoTrackSource, ffi,
};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;

/// PeerConnectionFactoryDependencies のラッパー。
/// スレッドやファクトリーのライフサイクルは呼び出し側で管理する。
pub struct PeerConnectionFactoryDependencies {
    raw: NonNull<ffi::webrtc_PeerConnectionFactoryDependencies>,
}

impl Default for PeerConnectionFactoryDependencies {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerConnectionFactoryDependencies {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_PeerConnectionFactoryDependencies_new() })
            .expect("BUG: webrtc_PeerConnectionFactoryDependencies_new が null を返しました");
        Self { raw }
    }

    /// start 済みの Thread を設定する。ライフサイクル管理は呼び出し側で行う。
    pub fn set_network_thread(&mut self, thread: &Thread) {
        let raw = thread.raw();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_network_thread(
                self.raw.as_ptr(),
                raw.as_ptr(),
            );
        }
    }

    /// start 済みの Thread を設定する。ライフサイクル管理は呼び出し側で行う。
    pub fn set_worker_thread(&mut self, thread: &Thread) {
        let raw = thread.raw();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_worker_thread(
                self.raw.as_ptr(),
                raw.as_ptr(),
            );
        }
    }

    /// start 済みの Thread を設定する。ライフサイクル管理は呼び出し側で行う。
    pub fn set_signaling_thread(&mut self, thread: &Thread) {
        let raw = thread.raw();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_signaling_thread(
                self.raw.as_ptr(),
                raw.as_ptr(),
            );
        }
    }

    pub fn set_audio_encoder_factory(&mut self, factory: &AudioEncoderFactory) {
        let raw_ref = factory.as_refcounted_ptr();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_audio_encoder_factory(
                self.raw.as_ptr(),
                raw_ref,
            );
        }
    }

    pub fn set_audio_decoder_factory(&mut self, factory: &AudioDecoderFactory) {
        let raw_ref = factory.as_refcounted_ptr();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_audio_decoder_factory(
                self.raw.as_ptr(),
                raw_ref,
            );
        }
    }

    pub fn set_audio_processing_builder(&mut self, builder: AudioProcessingBuilder) {
        let raw = builder.into_raw();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_audio_processing_builder(
                self.raw.as_ptr(),
                raw,
            );
        }
    }

    pub fn set_event_log_factory(&mut self, factory: RtcEventLogFactory) {
        let raw = factory.into_raw();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_event_log_factory(
                self.raw.as_ptr(),
                raw,
            );
        }
    }

    pub fn set_video_encoder_factory(&mut self, factory: VideoEncoderFactory) {
        let raw = factory.into_raw();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_video_encoder_factory(
                self.raw.as_ptr(),
                raw,
            );
        }
    }

    pub fn set_video_decoder_factory(&mut self, factory: VideoDecoderFactory) {
        let raw = factory.into_raw();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_video_decoder_factory(
                self.raw.as_ptr(),
                raw,
            );
        }
    }

    pub fn set_audio_device_module(&mut self, adm: &AudioDeviceModule) {
        let raw_ref = adm.as_refcounted_ptr();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryDependencies_set_adm(self.raw.as_ptr(), raw_ref);
        }
    }

    pub fn enable_media(&mut self) {
        unsafe { ffi::webrtc_EnableMedia(self.raw.as_ptr()) };
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionFactoryDependencies {
        self.raw.as_ptr()
    }
}

impl Drop for PeerConnectionFactoryDependencies {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_PeerConnectionFactoryDependencies_delete(self.raw.as_ptr()) };
    }
}

/// PeerConnectionFactoryInterface::Options のラッパー。
pub struct PeerConnectionFactoryOptions {
    raw: NonNull<ffi::webrtc_PeerConnectionFactoryInterface_Options>,
}

impl Default for PeerConnectionFactoryOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerConnectionFactoryOptions {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_PeerConnectionFactoryInterface_Options_new() })
            .expect("BUG: webrtc_PeerConnectionFactoryInterface_Options_new が null を返しました");
        Self { raw }
    }

    pub fn set_disable_encryption(&mut self, disable: bool) {
        unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_Options_set_disable_encryption(
                self.raw.as_ptr(),
                if disable { 1 } else { 0 },
            );
        }
    }

    pub fn set_ssl_max_version(&mut self, version: i32) {
        unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_Options_set_ssl_max_version(
                self.raw.as_ptr(),
                version,
            );
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionFactoryInterface_Options {
        self.raw.as_ptr()
    }
}

impl Drop for PeerConnectionFactoryOptions {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_PeerConnectionFactoryInterface_Options_delete(self.raw.as_ptr()) };
    }
}

/// PeerConnectionFactoryInterface のラッパー。
pub struct PeerConnectionFactory {
    raw_ref: ScopedRef<PeerConnectionFactoryHandle>,
}

impl PeerConnectionFactory {
    /// CreateModularPeerConnectionFactory 相当を生成する。
    pub fn create_modular(deps: &mut PeerConnectionFactoryDependencies) -> Result<Self> {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_CreateModularPeerConnectionFactory(deps.as_ptr()) })
                .ok_or(Error::NullPointer(
                    "webrtc_CreateModularPeerConnectionFactory が null を返しました",
                ))?;
        let raw_ref = ScopedRef::<PeerConnectionFactoryHandle>::from_raw(raw);
        Ok(Self { raw_ref })
    }

    pub fn set_options(&self, options: &PeerConnectionFactoryOptions) {
        unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_SetOptions(self.as_ptr(), options.as_ptr())
        };
    }

    pub fn get_rtp_sender_capabilities(&self, media_type: MediaType) -> RtpCapabilities {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_GetRtpSenderCapabilities(
                self.as_ptr(),
                media_type.to_int(),
            )
        }).expect("BUG: webrtc_PeerConnectionFactoryInterface_GetRtpSenderCapabilities が null を返しました");
        RtpCapabilities::from_raw(raw)
    }

    pub fn create_video_track(
        &self,
        source: &VideoTrackSource,
        track_id: &str,
    ) -> Result<VideoTrack> {
        let mut out = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_CreateVideoTrack(
                self.raw_ref.as_ptr(),
                source.as_refcounted_ptr(),
                track_id.as_ptr() as *const _,
                track_id.len(),
                &mut out,
            );
        }
        let out = NonNull::new(out).ok_or(Error::NullPointer(
            "webrtc_PeerConnectionFactoryInterface_CreateVideoTrack が null を返しました",
        ))?;
        let raw_ref = ScopedRef::<VideoTrackHandle>::from_raw(out);
        Ok(VideoTrack::from_scoped_ref(raw_ref))
    }

    pub fn create_audio_source(&self) -> Result<AudioTrackSource> {
        let mut out = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_CreateAudioSource(
                self.raw_ref.as_ptr(),
                &mut out,
            );
        }
        let out = NonNull::new(out).ok_or(Error::NullPointer(
            "webrtc_PeerConnectionFactoryInterface_CreateAudioSource が null を返しました",
        ))?;
        let raw_ref = ScopedRef::<AudioTrackSourceHandle>::from_raw(out);
        Ok(AudioTrackSource::from_scoped_ref(raw_ref))
    }

    pub fn create_audio_track(
        &self,
        source: &AudioTrackSource,
        track_id: &str,
    ) -> Result<AudioTrack> {
        let mut out = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_CreateAudioTrack(
                self.raw_ref.as_ptr(),
                source.as_refcounted_ptr(),
                track_id.as_ptr() as *const _,
                track_id.len(),
                &mut out,
            );
        }
        let out = NonNull::new(out).ok_or(Error::NullPointer(
            "webrtc_PeerConnectionFactoryInterface_CreateAudioTrack が null を返しました",
        ))?;
        let raw_ref = ScopedRef::<AudioTrackHandle>::from_raw(out);
        Ok(AudioTrack::from_scoped_ref(raw_ref))
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionFactoryInterface {
        self.raw_ref.as_ptr()
    }
}

unsafe impl Send for PeerConnectionFactory {}
// ここで生成する PeerConnectionFactoryInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/peer_connection_factory_proxy.h;l=32-59;drc=ef55be496e45889ace33ace4b05094ca19cb499b
unsafe impl Sync for PeerConnectionFactory {}

/// PeerConnectionInterface::RTCConfiguration のラッパー。
pub struct PeerConnectionRtcConfiguration {
    raw: NonNull<ffi::webrtc_PeerConnectionInterface_RTCConfiguration>,
}

impl Default for PeerConnectionRtcConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerConnectionRtcConfiguration {
    pub fn new() -> Self {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_PeerConnectionInterface_RTCConfiguration_new() })
                .expect(
                    "BUG: webrtc_PeerConnectionInterface_RTCConfiguration_new が null を返しました",
                );
        Self { raw }
    }

    pub fn set_type(&mut self, ice_type: IceTransportsType) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCConfiguration_set_type(
                self.raw.as_ptr(),
                ice_type.to_int(),
            );
        }
    }

    /// servers への可変参照を取得する。寿命は self に束縛される。
    pub fn servers(&mut self) -> IceServerVectorRef<'_> {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCConfiguration_get_servers(self.raw.as_ptr())
        })
        .expect(
            "BUG: webrtc_PeerConnectionInterface_RTCConfiguration_get_servers が null を返しました",
        );
        IceServerVectorRef::from_raw(raw)
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionInterface_RTCConfiguration {
        self.raw.as_ptr()
    }
}

impl Drop for PeerConnectionRtcConfiguration {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_PeerConnectionInterface_RTCConfiguration_delete(self.raw.as_ptr()) };
    }
}

/// IceTransportsType のラッパー。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IceTransportsType {
    Relay,
    Unknown(i32),
}

impl IceTransportsType {
    pub fn to_int(self) -> i32 {
        match self {
            IceTransportsType::Relay => unsafe {
                ffi::webrtc_PeerConnectionInterface_IceTransportsType_kRelay
            },
            IceTransportsType::Unknown(v) => v,
        }
    }
}

/// PeerConnectionInterface::IceServer のラッパー。
pub struct IceServer {
    raw: NonNull<ffi::webrtc_PeerConnectionInterface_IceServer>,
}

impl Default for IceServer {
    fn default() -> Self {
        Self::new()
    }
}

impl IceServer {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_PeerConnectionInterface_IceServer_new() })
            .expect("BUG: webrtc_PeerConnectionInterface_IceServer_new が null を返しました");
        Self { raw }
    }

    pub fn add_url(&mut self, url: &str) {
        self.as_ref().add_url(url);
    }

    pub fn set_username(&mut self, username: &str) {
        self.as_ref().set_username(username);
    }

    pub fn set_password(&mut self, password: &str) {
        self.as_ref().set_password(password);
    }

    pub fn as_ref(&self) -> IceServerRef<'_> {
        IceServerRef::from_raw(self.raw)
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionInterface_IceServer {
        self.raw.as_ptr()
    }
}

impl Drop for IceServer {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_PeerConnectionInterface_IceServer_delete(self.raw.as_ptr()) };
    }
}

/// IceServer への借用ラッパー。
pub struct IceServerRef<'a> {
    raw: NonNull<ffi::webrtc_PeerConnectionInterface_IceServer>,
    _marker: PhantomData<&'a mut ffi::webrtc_PeerConnectionInterface_IceServer_vector>,
}

impl<'a> IceServerRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_PeerConnectionInterface_IceServer>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionInterface_IceServer {
        self.raw.as_ptr()
    }

    pub fn add_url(&self, url: &str) {
        let urls =
            unsafe { ffi::webrtc_PeerConnectionInterface_IceServer_get_urls(self.raw.as_ptr()) };
        let cxx = CxxString::from_str(url);
        unsafe { ffi::std_string_vector_push_back(urls, cxx.as_ptr()) };
    }

    pub fn set_username(&self, username: &str) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_IceServer_set_username(
                self.raw.as_ptr(),
                username.as_ptr() as *const _,
                username.len(),
            );
        }
    }

    pub fn set_password(&self, password: &str) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_IceServer_set_password(
                self.raw.as_ptr(),
                password.as_ptr() as *const _,
                password.len(),
            );
        }
    }
}

/// PeerConnectionInterface::IceServer_vector の所有ラッパー。
pub struct IceServerVector {
    raw: NonNull<ffi::webrtc_PeerConnectionInterface_IceServer_vector>,
}

impl IceServerVector {
    pub fn new(size: i32) -> Self {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_PeerConnectionInterface_IceServer_vector_new(size) })
                .expect(
                    "BUG: webrtc_PeerConnectionInterface_IceServer_vector_new が null を返しました",
                );
        Self { raw }
    }

    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    pub fn get(&self, index: usize) -> Option<IceServerRef<'_>> {
        self.as_ref().get(index)
    }

    pub fn push(&mut self, server: &IceServer) {
        self.as_ref().push(server);
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionInterface_IceServer_vector {
        self.raw.as_ptr()
    }

    pub fn as_ref(&self) -> IceServerVectorRef<'_> {
        IceServerVectorRef::from_raw(self.raw)
    }
}

impl Drop for IceServerVector {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_PeerConnectionInterface_IceServer_vector_delete(self.raw.as_ptr()) };
    }
}

/// RTCConfiguration 内部の servers を借用するためのラッパー。
pub struct IceServerVectorRef<'a> {
    raw: NonNull<ffi::webrtc_PeerConnectionInterface_IceServer_vector>,
    _marker: PhantomData<&'a mut ffi::webrtc_PeerConnectionInterface_RTCConfiguration>,
}

impl<'a> IceServerVectorRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_PeerConnectionInterface_IceServer_vector>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let len =
            unsafe { ffi::webrtc_PeerConnectionInterface_IceServer_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<IceServerRef<'a>> {
        let len = self.len();
        if index >= len {
            return None;
        }
        let raw = NonNull::new(unsafe {
            ffi::webrtc_PeerConnectionInterface_IceServer_vector_get(
                self.raw.as_ptr(),
                index as i32,
            )
        })
        .expect("BUG: webrtc_PeerConnectionInterface_IceServer_vector_get が null を返しました");
        Some(IceServerRef::from_raw(raw))
    }

    pub fn push(&self, server: &IceServer) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_IceServer_vector_push_back(
                self.raw.as_ptr(),
                server.as_ptr(),
            );
        }
    }
}

/// RTCOfferAnswerOptions のラッパー。
pub struct PeerConnectionOfferAnswerOptions {
    raw: NonNull<ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions>,
}

impl Default for PeerConnectionOfferAnswerOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerConnectionOfferAnswerOptions {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_new()
        })
        .expect(
            "BUG: webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_new が null を返しました",
        );
        Self { raw }
    }

    pub fn offer_to_receive_video(&self) -> i32 {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_video(
                self.raw.as_ptr(),
            )
        }
    }

    pub fn set_offer_to_receive_video(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_video(
                self.raw.as_ptr(),
                value,
            );
        }
    }

    pub fn offer_to_receive_audio(&self) -> i32 {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_audio(
                self.raw.as_ptr(),
            )
        }
    }

    pub fn set_offer_to_receive_audio(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_audio(
                self.raw.as_ptr(),
                value,
            );
        }
    }

    pub fn voice_activity_detection(&self) -> bool {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_voice_activity_detection(
                self.raw.as_ptr(),
            ) != 0
        }
    }

    pub fn set_voice_activity_detection(&mut self, enable: bool) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_voice_activity_detection(
                self.raw.as_ptr(),
                if enable { 1 } else { 0 },
            );
        }
    }

    pub fn ice_restart(&self) -> bool {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_ice_restart(
                self.raw.as_ptr(),
            ) != 0
        }
    }

    pub fn set_ice_restart(&mut self, enable: bool) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_ice_restart(
                self.raw.as_ptr(),
                if enable { 1 } else { 0 },
            );
        }
    }

    pub fn use_rtp_mux(&self) -> bool {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_rtp_mux(
                self.raw.as_ptr(),
            ) != 0
        }
    }

    pub fn set_use_rtp_mux(&mut self, enable: bool) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_rtp_mux(
                self.raw.as_ptr(),
                if enable { 1 } else { 0 },
            );
        }
    }

    pub fn raw_packetization_for_video(&self) -> bool {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_raw_packetization_for_video(
                self.raw.as_ptr(),
            ) != 0
        }
    }

    pub fn set_raw_packetization_for_video(&mut self, enable: bool) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_raw_packetization_for_video(
                self.raw.as_ptr(),
                if enable { 1 } else { 0 },
            );
        }
    }

    pub fn num_simulcast_layers(&self) -> i32 {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_num_simulcast_layers(
                self.raw.as_ptr(),
            )
        }
    }

    pub fn set_num_simulcast_layers(&mut self, num: i32) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_num_simulcast_layers(
                self.raw.as_ptr(),
                num,
            );
        }
    }

    pub fn use_obsolete_sctp_sdp(&self) -> bool {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_obsolete_sctp_sdp(
                self.raw.as_ptr(),
            ) != 0
        }
    }

    pub fn set_use_obsolete_sctp_sdp(&mut self, enable: bool) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_obsolete_sctp_sdp(
                self.raw.as_ptr(),
                if enable { 1 } else { 0 },
            );
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions {
        self.raw.as_ptr()
    }
}

impl Drop for PeerConnectionOfferAnswerOptions {
    fn drop(&mut self) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_delete(self.raw.as_ptr())
        };
    }
}

/// PeerConnectionState のラッパー。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerConnectionState {
    New,
    Connecting,
    Connected,
    Disconnected,
    Failed,
    Closed,
    Unknown(i32),
}

impl PeerConnectionState {
    pub fn from_int(v: i32) -> Self {
        unsafe {
            if v == ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kNew {
                PeerConnectionState::New
            } else if v == ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kConnecting {
                PeerConnectionState::Connecting
            } else if v == ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kConnected {
                PeerConnectionState::Connected
            } else if v == ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kDisconnected {
                PeerConnectionState::Disconnected
            } else if v == ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kFailed {
                PeerConnectionState::Failed
            } else if v == ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kClosed {
                PeerConnectionState::Closed
            } else {
                PeerConnectionState::Unknown(v)
            }
        }
    }

    pub fn to_int(self) -> i32 {
        match self {
            PeerConnectionState::New => unsafe {
                ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kNew
            },
            PeerConnectionState::Connecting => unsafe {
                ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kConnecting
            },
            PeerConnectionState::Connected => unsafe {
                ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kConnected
            },
            PeerConnectionState::Disconnected => unsafe {
                ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kDisconnected
            },
            PeerConnectionState::Failed => unsafe {
                ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kFailed
            },
            PeerConnectionState::Closed => unsafe {
                ffi::webrtc_PeerConnectionInterface_PeerConnectionState_kClosed
            },
            PeerConnectionState::Unknown(v) => v,
        }
    }
}

type IceCandidateCallback = Option<Box<dyn FnMut(IceCandidateRef) + Send + 'static>>;

struct ObserverCallbacks {
    on_connection_change: Option<Box<dyn FnMut(PeerConnectionState) + Send + 'static>>,
    on_track: Option<Box<dyn FnMut(RtpTransceiver) + Send + 'static>>,
    on_remove_track: Option<Box<dyn FnMut(RtpReceiver) + Send + 'static>>,
    on_ice_candidate: IceCandidateCallback,
    on_data_channel: Option<Box<dyn FnMut(DataChannel) + Send + 'static>>,
}

/// PeerConnectionObserver 用のコールバック設定。
pub struct PeerConnectionObserverBuilder {
    on_connection_change: Option<Box<dyn FnMut(PeerConnectionState) + Send + 'static>>,
    on_track: Option<Box<dyn FnMut(RtpTransceiver) + Send + 'static>>,
    on_remove_track: Option<Box<dyn FnMut(RtpReceiver) + Send + 'static>>,
    on_ice_candidate: IceCandidateCallback,
    on_data_channel: Option<Box<dyn FnMut(DataChannel) + Send + 'static>>,
}

impl Default for PeerConnectionObserverBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerConnectionObserverBuilder {
    pub fn new() -> Self {
        Self {
            on_connection_change: None,
            on_track: None,
            on_remove_track: None,
            on_ice_candidate: None,
            on_data_channel: None,
        }
    }

    pub fn on_connection_change<F>(mut self, on_connection_change: F) -> Self
    where
        F: FnMut(PeerConnectionState) + Send + 'static,
    {
        self.on_connection_change = Some(Box::new(on_connection_change));
        self
    }

    pub fn on_track<T>(mut self, on_track: T) -> Self
    where
        T: FnMut(RtpTransceiver) + Send + 'static,
    {
        self.on_track = Some(Box::new(on_track));
        self
    }

    pub fn on_remove_track<R>(mut self, on_remove_track: R) -> Self
    where
        R: FnMut(RtpReceiver) + Send + 'static,
    {
        self.on_remove_track = Some(Box::new(on_remove_track));
        self
    }

    pub fn on_ice_candidate<C>(mut self, on_ice_candidate: C) -> Self
    where
        C: FnMut(IceCandidateRef) + Send + 'static,
    {
        self.on_ice_candidate = Some(Box::new(on_ice_candidate));
        self
    }

    pub fn on_data_channel<D>(mut self, on_data_channel: D) -> Self
    where
        D: FnMut(DataChannel) + Send + 'static,
    {
        self.on_data_channel = Some(Box::new(on_data_channel));
        self
    }

    pub fn build(self) -> PeerConnectionObserver {
        PeerConnectionObserver::new(self)
    }
}

unsafe extern "C" fn observer_on_connection_change(new_state: i32, user_data: *mut c_void) {
    assert!(!user_data.is_null());
    let callbacks = unsafe { &mut *(user_data as *mut ObserverCallbacks) };
    if let Some(cb) = callbacks.on_connection_change.as_mut() {
        cb(PeerConnectionState::from_int(new_state));
    }
}

unsafe extern "C" fn observer_on_track(
    transceiver: *mut ffi::webrtc_RtpTransceiverInterface_refcounted,
    user_data: *mut c_void,
) {
    assert!(!user_data.is_null());
    let callbacks = unsafe { &mut *(user_data as *mut ObserverCallbacks) };
    let raw_ref = ScopedRef::<RtpTransceiverHandle>::from_raw(
        NonNull::new(transceiver).expect("BUG: transceiver が null"),
    );
    let transceiver = RtpTransceiver::from_scoped_ref(raw_ref);
    if let Some(cb) = callbacks.on_track.as_mut() {
        cb(transceiver);
    }
}

unsafe extern "C" fn observer_on_ice_candidate(
    candidate: *const ffi::webrtc_IceCandidate,
    user_data: *mut c_void,
) {
    assert!(!user_data.is_null());
    let callbacks = unsafe { &mut *(user_data as *mut ObserverCallbacks) };
    let candidate =
        NonNull::new(candidate as *mut ffi::webrtc_IceCandidate).expect("BUG: candidate が null");
    let candidate = IceCandidateRef::from_raw(candidate);
    if let Some(cb) = callbacks.on_ice_candidate.as_mut() {
        cb(candidate);
    }
}

unsafe extern "C" fn observer_on_remove_track(
    receiver: *mut ffi::webrtc_RtpReceiverInterface_refcounted,
    user_data: *mut c_void,
) {
    assert!(!user_data.is_null());
    let callbacks = unsafe { &mut *(user_data as *mut ObserverCallbacks) };
    let raw_ref = ScopedRef::<RtpReceiverHandle>::from_raw(
        NonNull::new(receiver).expect("BUG: receiver が null"),
    );
    let receiver = RtpReceiver::from_scoped_ref(raw_ref);
    if let Some(cb) = callbacks.on_remove_track.as_mut() {
        cb(receiver);
    }
}

unsafe extern "C" fn observer_on_data_channel(
    data_channel: *mut ffi::webrtc_DataChannelInterface_refcounted,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callbacks = unsafe { &mut *(user_data as *mut ObserverCallbacks) };
    let raw_ref = ScopedRef::<DataChannelHandle>::from_raw(
        NonNull::new(data_channel).expect("BUG: data_channel が null"),
    );
    let data_channel = DataChannel::from_scoped_ref(raw_ref);
    if let Some(cb) = callbacks.on_data_channel.as_mut() {
        cb(data_channel);
    }
}

/// PeerConnectionObserver のラッパー。
pub struct PeerConnectionObserver {
    raw: NonNull<ffi::webrtc_PeerConnectionObserver>,
    _cbs: Box<ffi::webrtc_PeerConnectionObserver_cbs>,
    _user_data: Box<ObserverCallbacks>,
}

impl PeerConnectionObserver {
    fn new(handlers: PeerConnectionObserverBuilder) -> Self {
        let PeerConnectionObserverBuilder {
            on_connection_change,
            on_track,
            on_remove_track,
            on_ice_candidate,
            on_data_channel,
        } = handlers;
        let has_on_connection_change = on_connection_change.is_some();
        let has_on_track = on_track.is_some();
        let has_on_remove_track = on_remove_track.is_some();
        let has_on_ice_candidate = on_ice_candidate.is_some();
        let has_on_data_channel = on_data_channel.is_some();
        let mut callbacks = Box::new(ObserverCallbacks {
            on_connection_change,
            on_track,
            on_remove_track,
            on_ice_candidate,
            on_data_channel,
        });
        let user_data = callbacks.as_mut() as *mut ObserverCallbacks as *mut c_void;
        let mut cbs = Box::new(ffi::webrtc_PeerConnectionObserver_cbs {
            OnConnectionChange: if has_on_connection_change {
                Some(observer_on_connection_change)
            } else {
                None
            },
            OnIceCandidate: if has_on_ice_candidate {
                Some(observer_on_ice_candidate)
            } else {
                None
            },
            OnTrack: if has_on_track {
                Some(observer_on_track)
            } else {
                None
            },
            OnRemoveTrack: if has_on_remove_track {
                Some(observer_on_remove_track)
            } else {
                None
            },
            OnDataChannel: if has_on_data_channel {
                Some(observer_on_data_channel)
            } else {
                None
            },
        });
        let cbs_ptr = cbs.as_mut() as *mut ffi::webrtc_PeerConnectionObserver_cbs;
        let raw = unsafe { ffi::webrtc_PeerConnectionObserver_new(cbs_ptr, user_data) };
        let raw =
            NonNull::new(raw).expect("BUG: webrtc_PeerConnectionObserver_new が null を返しました");
        Self {
            raw,
            _cbs: cbs,
            _user_data: callbacks,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionObserver {
        self.raw.as_ptr()
    }

    #[cfg(test)]
    pub(crate) fn invoke_connection_change_for_test(&mut self, state: PeerConnectionState) {
        let user_data = self._user_data.as_mut() as *mut ObserverCallbacks as *mut c_void;
        unsafe { observer_on_connection_change(state.to_int(), user_data) };
    }
}

impl Drop for PeerConnectionObserver {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_PeerConnectionObserver_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for PeerConnectionObserver {}

/// PeerConnectionDependencies のラッパー。
pub struct PeerConnectionDependencies {
    raw: NonNull<ffi::webrtc_PeerConnectionDependencies>,
}

impl PeerConnectionDependencies {
    pub fn new(observer: &PeerConnectionObserver) -> Self {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_PeerConnectionDependencies_new(observer.as_ptr()) })
                .expect("BUG: webrtc_PeerConnectionDependencies_new が null を返しました");
        Self { raw }
    }

    pub fn as_ptr(&mut self) -> *mut ffi::webrtc_PeerConnectionDependencies {
        self.raw.as_ptr()
    }
}

impl Drop for PeerConnectionDependencies {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_PeerConnectionDependencies_delete(self.raw.as_ptr()) };
    }
}

struct PeerConnectionStatsCallbackState {
    on_stats: Box<dyn FnOnce(RTCStatsReport) + Send + 'static>,
    cbs: ffi::webrtc_RTCStatsCollectorCallback_cbs,
}

unsafe extern "C" fn peer_connection_on_stats(
    report: *const ffi::webrtc_RTCStatsReport_refcounted,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let state = unsafe { Box::from_raw(user_data as *mut PeerConnectionStatsCallbackState) };
    let Some(report) = NonNull::new(report as *mut ffi::webrtc_RTCStatsReport_refcounted) else {
        return;
    };
    let report = RTCStatsReport::from_refcounted_ptr(report);
    (state.on_stats)(report);
}

struct CreateSessionDescriptionCallbacks {
    on_success: Box<dyn FnMut(SessionDescription) + Send + 'static>,
    on_failure: Box<dyn FnMut(RtcError) + Send + 'static>,
}

unsafe extern "C" fn csd_on_success(
    desc: *mut ffi::webrtc_SessionDescriptionInterface_unique,
    user_data: *mut c_void,
) {
    let callbacks = unsafe { &mut *(user_data as *mut CreateSessionDescriptionCallbacks) };
    let desc =
        SessionDescription::from_unique_ptr(NonNull::new(desc).expect("BUG: desc が null です"));
    (callbacks.on_success)(desc);
}

unsafe extern "C" fn csd_on_failure(
    error: *mut ffi::webrtc_RTCError_unique,
    user_data: *mut c_void,
) {
    let callbacks = unsafe { &mut *(user_data as *mut CreateSessionDescriptionCallbacks) };
    let err = RtcError::from_unique_ptr(NonNull::new(error).expect("BUG: error が null です"));
    (callbacks.on_failure)(err);
}

/// CreateSessionDescriptionObserver のラッパー。
pub struct CreateSessionDescriptionObserver {
    raw: NonNull<ffi::webrtc_CreateSessionDescriptionObserver>,
    _cbs: Box<ffi::webrtc_CreateSessionDescriptionObserver_cbs>,
    _user_data: Box<CreateSessionDescriptionCallbacks>,
}

impl CreateSessionDescriptionObserver {
    pub fn new<S, F>(on_success: S, on_failure: F) -> Self
    where
        S: FnMut(SessionDescription) + Send + 'static,
        F: FnMut(RtcError) + Send + 'static,
    {
        let mut callbacks = Box::new(CreateSessionDescriptionCallbacks {
            on_success: Box::new(on_success),
            on_failure: Box::new(on_failure),
        });
        let user_data = callbacks.as_mut() as *mut CreateSessionDescriptionCallbacks as *mut c_void;
        let mut cbs = Box::new(ffi::webrtc_CreateSessionDescriptionObserver_cbs {
            OnSuccess: Some(csd_on_success),
            OnFailure: Some(csd_on_failure),
        });
        let cbs_ptr = cbs.as_mut() as *mut ffi::webrtc_CreateSessionDescriptionObserver_cbs;
        let raw = unsafe {
            ffi::webrtc_CreateSessionDescriptionObserver_make_ref_counted(cbs_ptr, user_data)
        };
        Self {
            raw: NonNull::new(raw).expect("BUG: raw が null です"),
            _cbs: cbs,
            _user_data: callbacks,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_CreateSessionDescriptionObserver {
        self.raw.as_ptr()
    }
}

impl Drop for CreateSessionDescriptionObserver {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_CreateSessionDescriptionObserver_Release(self.raw.as_ptr()) };
    }
}

unsafe impl Send for CreateSessionDescriptionObserver {}

struct SetLocalDescriptionCallbacks {
    on_complete: Box<dyn FnMut(RtcError) + Send + 'static>,
}

unsafe extern "C" fn sld_on_complete(
    error: *mut ffi::webrtc_RTCError_unique,
    user_data: *mut c_void,
) {
    let callbacks = unsafe { &mut *(user_data as *mut SetLocalDescriptionCallbacks) };
    let err = RtcError::from_unique_ptr(NonNull::new(error).expect("BUG: error が null です"));
    (callbacks.on_complete)(err);
}

/// SetLocalDescriptionObserverInterface のラッパー。
pub struct SetLocalDescriptionObserver {
    raw_ref: ScopedRef<SetLocalDescriptionObserverHandle>,
    _cbs: Box<ffi::webrtc_SetLocalDescriptionObserverInterface_cbs>,
    _user_data: Box<SetLocalDescriptionCallbacks>,
}

impl SetLocalDescriptionObserver {
    pub fn new<F>(on_complete: F) -> Self
    where
        F: FnMut(RtcError) + Send + 'static,
    {
        let mut callbacks = Box::new(SetLocalDescriptionCallbacks {
            on_complete: Box::new(on_complete),
        });
        let user_data = callbacks.as_mut() as *mut SetLocalDescriptionCallbacks as *mut c_void;
        let mut cbs = Box::new(ffi::webrtc_SetLocalDescriptionObserverInterface_cbs {
            OnSetLocalDescriptionComplete: Some(sld_on_complete),
        });
        let cbs_ptr = cbs.as_mut() as *mut ffi::webrtc_SetLocalDescriptionObserverInterface_cbs;
        let raw = NonNull::new(unsafe {
            ffi::webrtc_SetLocalDescriptionObserverInterface_make_ref_counted(cbs_ptr, user_data)
        }).expect("BUG: webrtc_SetLocalDescriptionObserverInterface_make_ref_counted が null を返しました");
        let raw_ref = ScopedRef::<SetLocalDescriptionObserverHandle>::from_raw(raw);
        Self {
            raw_ref,
            _cbs: cbs,
            _user_data: callbacks,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_SetLocalDescriptionObserverInterface {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(
        &self,
    ) -> *mut ffi::webrtc_SetLocalDescriptionObserverInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }
}

unsafe impl Send for SetLocalDescriptionObserver {}

struct SetRemoteDescriptionCallbacks {
    on_complete: Box<dyn FnMut(RtcError) + Send + 'static>,
}

unsafe extern "C" fn srd_on_complete(
    error: *mut ffi::webrtc_RTCError_unique,
    user_data: *mut c_void,
) {
    let callbacks = unsafe { &mut *(user_data as *mut SetRemoteDescriptionCallbacks) };
    let err = RtcError::from_unique_ptr(NonNull::new(error).expect("BUG: error が null"));
    (callbacks.on_complete)(err);
}

/// SetRemoteDescriptionObserverInterface のラッパー。
pub struct SetRemoteDescriptionObserver {
    raw_ref: ScopedRef<SetRemoteDescriptionObserverHandle>,
    _cbs: Box<ffi::webrtc_SetRemoteDescriptionObserverInterface_cbs>,
    _user_data: Box<SetRemoteDescriptionCallbacks>,
}

impl SetRemoteDescriptionObserver {
    pub fn new<F>(on_complete: F) -> Self
    where
        F: FnMut(RtcError) + Send + 'static,
    {
        let mut callbacks = Box::new(SetRemoteDescriptionCallbacks {
            on_complete: Box::new(on_complete),
        });
        let user_data = callbacks.as_mut() as *mut SetRemoteDescriptionCallbacks as *mut c_void;
        let mut cbs = Box::new(ffi::webrtc_SetRemoteDescriptionObserverInterface_cbs {
            OnSetRemoteDescriptionComplete: Some(srd_on_complete),
        });
        let cbs_ptr = cbs.as_mut() as *mut ffi::webrtc_SetRemoteDescriptionObserverInterface_cbs;
        let raw = NonNull::new(unsafe {
            ffi::webrtc_SetRemoteDescriptionObserverInterface_make_ref_counted(cbs_ptr, user_data)
        })
        .expect("BUG: raw が null です");
        let raw_ref = ScopedRef::<SetRemoteDescriptionObserverHandle>::from_raw(raw);
        Self {
            raw_ref,
            _cbs: cbs,
            _user_data: callbacks,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_SetRemoteDescriptionObserverInterface {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(
        &self,
    ) -> *mut ffi::webrtc_SetRemoteDescriptionObserverInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }
}

unsafe impl Send for SetRemoteDescriptionObserver {}

/// PeerConnectionInterface のラッパー。
pub struct PeerConnection {
    raw_ref: ScopedRef<PeerConnectionHandle>,
}

impl PeerConnection {
    pub fn create(
        factory: &PeerConnectionFactory,
        config: &mut PeerConnectionRtcConfiguration,
        deps: &mut PeerConnectionDependencies,
    ) -> Result<Self> {
        let mut out_pc: *mut ffi::webrtc_PeerConnectionInterface_refcounted = std::ptr::null_mut();
        let mut out_error: *mut ffi::webrtc_RTCError_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError(
                factory.as_ptr(),
                config.as_ptr(),
                deps.as_ptr(),
                &mut out_pc,
                &mut out_error,
            );
        }
        if !out_error.is_null() {
            let err = RtcError::from_unique_ptr(NonNull::new(out_error).unwrap());
            return Err(Error::RtcError(err));
        }

        let pc = NonNull::new(out_pc).expect("BUG: out_pc と out_error が両方 null です");
        let raw_ref = ScopedRef::<PeerConnectionHandle>::from_raw(pc);
        Ok(Self { raw_ref })
    }

    pub fn create_offer(
        &self,
        observer: &mut CreateSessionDescriptionObserver,
        options: &mut PeerConnectionOfferAnswerOptions,
    ) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_CreateOffer(
                self.raw_ref.as_ptr(),
                observer.as_ptr(),
                options.as_ptr(),
            )
        };
    }

    pub fn create_answer(
        &self,
        observer: &mut CreateSessionDescriptionObserver,
        options: &mut PeerConnectionOfferAnswerOptions,
    ) {
        unsafe {
            ffi::webrtc_PeerConnectionInterface_CreateAnswer(
                self.raw_ref.as_ptr(),
                observer.as_ptr(),
                options.as_ptr(),
            )
        };
    }

    pub fn set_local_description(
        &self,
        desc: SessionDescription,
        observer: &SetLocalDescriptionObserver,
    ) {
        let raw_desc = desc.into_raw();
        unsafe {
            ffi::webrtc_PeerConnectionInterface_SetLocalDescription(
                self.raw_ref.as_ptr(),
                raw_desc,
                observer.as_refcounted_ptr(),
            )
        };
    }

    pub fn set_remote_description(
        &self,
        desc: SessionDescription,
        observer: &SetRemoteDescriptionObserver,
    ) {
        let raw_desc = desc.into_raw();
        unsafe {
            ffi::webrtc_PeerConnectionInterface_SetRemoteDescription(
                self.raw_ref.as_ptr(),
                raw_desc,
                observer.as_refcounted_ptr(),
            )
        };
    }

    pub fn add_ice_candidate(&self, candidate: &IceCandidate) -> Result<()> {
        let ok = unsafe {
            ffi::webrtc_PeerConnectionInterface_AddIceCandidate(
                self.raw_ref.as_ptr(),
                candidate.as_ptr(),
            )
        };
        if ok == 0 {
            return Err(Error::InvalidIceCandidate);
        }
        Ok(())
    }

    pub fn set_configuration(&self, config: &mut PeerConnectionRtcConfiguration) -> Result<()> {
        let mut out_error: *mut ffi::webrtc_RTCError_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionInterface_SetConfiguration(
                self.raw_ref.as_ptr(),
                config.as_ptr(),
                &mut out_error,
            );
        }
        if !out_error.is_null() {
            let err = RtcError::from_unique_ptr(NonNull::new(out_error).unwrap());
            return Err(Error::RtcError(err));
        }
        Ok(())
    }

    pub fn create_data_channel(
        &self,
        label: &str,
        init: &mut DataChannelInit,
    ) -> Result<DataChannel> {
        let mut out_dc: *mut ffi::webrtc_DataChannelInterface_refcounted = std::ptr::null_mut();
        let mut out_error: *mut ffi::webrtc_RTCError_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionInterface_CreateDataChannelOrError(
                self.raw_ref.as_ptr(),
                label.as_ptr() as *const _,
                label.len(),
                init.as_ptr(),
                &mut out_dc,
                &mut out_error,
            );
        }
        if !out_error.is_null() {
            let err = RtcError::from_unique_ptr(NonNull::new(out_error).unwrap());
            return Err(Error::RtcError(err));
        }
        assert!(!out_dc.is_null());
        let raw_ref = ScopedRef::<DataChannelHandle>::from_raw(NonNull::new(out_dc).unwrap());
        Ok(DataChannel::from_scoped_ref(raw_ref))
    }

    pub fn add_transceiver(
        &self,
        media_type: MediaType,
        init: &mut RtpTransceiverInit,
    ) -> Result<RtpTransceiver> {
        let mut out_transceiver: *mut ffi::webrtc_RtpTransceiverInterface_refcounted =
            std::ptr::null_mut();
        let mut out_error: *mut ffi::webrtc_RTCError_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionInterface_AddTransceiver(
                self.raw_ref.as_ptr(),
                media_type.to_int(),
                init.as_ptr(),
                &mut out_transceiver,
                &mut out_error,
            );
        }
        if !out_error.is_null() {
            let err = RtcError::from_unique_ptr(NonNull::new(out_error).unwrap());
            return Err(Error::RtcError(err));
        }
        assert!(!out_transceiver.is_null());
        let raw_ref =
            ScopedRef::<RtpTransceiverHandle>::from_raw(NonNull::new(out_transceiver).unwrap());
        Ok(RtpTransceiver::from_scoped_ref(raw_ref))
    }

    pub fn add_transceiver_with_track(
        &self,
        track: &VideoTrack,
        init: &mut RtpTransceiverInit,
    ) -> Result<RtpTransceiver> {
        let mut out_transceiver: *mut ffi::webrtc_RtpTransceiverInterface_refcounted =
            std::ptr::null_mut();
        let mut out_error: *mut ffi::webrtc_RTCError_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionInterface_AddTransceiverWithTrack(
                self.raw_ref.as_ptr(),
                track.as_refcounted_ptr(),
                init.as_ptr(),
                &mut out_transceiver,
                &mut out_error,
            );
        }
        if !out_error.is_null() {
            let err = RtcError::from_unique_ptr(NonNull::new(out_error).unwrap());
            return Err(Error::RtcError(err));
        }
        assert!(!out_transceiver.is_null());
        let raw_ref =
            ScopedRef::<RtpTransceiverHandle>::from_raw(NonNull::new(out_transceiver).unwrap());
        Ok(RtpTransceiver::from_scoped_ref(raw_ref))
    }

    pub fn add_track(
        &self,
        track: &MediaStreamTrack,
        stream_ids: &StringVector,
    ) -> Result<RtpSender> {
        let mut out_sender: *mut ffi::webrtc_RtpSenderInterface_refcounted = std::ptr::null_mut();
        let mut out_error: *mut ffi::webrtc_RTCError_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_PeerConnectionInterface_AddTrack(
                self.raw_ref.as_ptr(),
                track.as_refcounted_ptr(),
                stream_ids.as_ptr(),
                &mut out_sender,
                &mut out_error,
            );
        }
        if !out_error.is_null() {
            let err = RtcError::from_unique_ptr(NonNull::new(out_error).unwrap());
            return Err(Error::RtcError(err));
        }
        assert!(!out_sender.is_null());
        let raw_ref = ScopedRef::<RtpSenderHandle>::from_raw(NonNull::new(out_sender).unwrap());
        Ok(RtpSender::from_scoped_ref(raw_ref))
    }

    pub fn get_stats<F>(&self, on_stats: F)
    where
        F: FnOnce(RTCStatsReport) + Send + 'static,
    {
        let mut state = Box::new(PeerConnectionStatsCallbackState {
            on_stats: Box::new(on_stats),
            cbs: ffi::webrtc_RTCStatsCollectorCallback_cbs {
                OnStatsDelivered: Some(peer_connection_on_stats),
            },
        });
        let cbs_ptr = &mut state.cbs as *mut ffi::webrtc_RTCStatsCollectorCallback_cbs;
        let user_data = Box::into_raw(state) as *mut c_void;
        unsafe {
            ffi::webrtc_PeerConnectionInterface_GetStats(self.raw_ref.as_ptr(), cbs_ptr, user_data)
        };
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_PeerConnectionInterface {
        self.raw_ref.as_ptr()
    }
}

unsafe impl Send for PeerConnection {}
// SAFETY: PeerConnectionInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/peer_connection_proxy.h;l=56-204;drc=ef55be496e45889ace33ace4b05094ca19cb499b
unsafe impl Sync for PeerConnection {}
