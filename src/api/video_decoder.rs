use super::video_codec_common::{
    EncodedImageRef, SdpVideoFormat, SdpVideoFormatRef, VideoCodecStatus, VideoCodecType,
    VideoFrameRef,
};
use crate::{CxxString, EnvironmentRef, Result, ffi};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub struct VideoDecoderDecoderInfo {
    raw_unique: NonNull<ffi::webrtc_VideoDecoder_DecoderInfo_unique>,
}

unsafe impl Send for VideoDecoderDecoderInfo {}

impl VideoDecoderDecoderInfo {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_new() })
            .expect("webrtc_VideoDecoder_DecoderInfo_new が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn implementation_name(&self) -> Result<String> {
        let raw =
            unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_get_implementation_name(self.as_ptr()) };
        let raw = NonNull::new(raw)
            .expect("webrtc_VideoDecoder_DecoderInfo_get_implementation_name が null を返しました");
        CxxString::from_unique(raw).to_string()
    }

    pub fn set_implementation_name(&mut self, name: &str) {
        let name = CxxString::from_str(name);
        unsafe {
            ffi::webrtc_VideoDecoder_DecoderInfo_set_implementation_name(
                self.as_ptr(),
                name.into_raw(),
            );
        }
    }

    pub fn is_hardware_accelerated(&self) -> bool {
        unsafe {
            ffi::webrtc_VideoDecoder_DecoderInfo_get_is_hardware_accelerated(self.as_ptr()) != 0
        }
    }

    pub fn set_is_hardware_accelerated(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_VideoDecoder_DecoderInfo_set_is_hardware_accelerated(
                self.as_ptr(),
                if value { 1 } else { 0 },
            );
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoder_DecoderInfo {
        unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoDecoder_DecoderInfo_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Default for VideoDecoderDecoderInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VideoDecoderDecoderInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_unique_delete(self.raw_unique.as_ptr()) };
    }
}

pub struct VideoDecoderSettingsRef<'a> {
    raw: NonNull<ffi::webrtc_VideoDecoder_Settings>,
    _marker: PhantomData<&'a ffi::webrtc_VideoDecoder_Settings>,
}

unsafe impl<'a> Send for VideoDecoderSettingsRef<'a> {}

impl<'a> VideoDecoderSettingsRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoDecoder_Settings` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoDecoder_Settings>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn number_of_cores(&self) -> i32 {
        unsafe { ffi::webrtc_VideoDecoder_Settings_number_of_cores(self.raw.as_ptr()) }
    }

    pub fn codec_type(&self) -> VideoCodecType {
        let value = unsafe { ffi::webrtc_VideoDecoder_Settings_codec_type(self.raw.as_ptr()) };
        VideoCodecType::from_raw(value)
    }

    pub fn buffer_pool_size(&self) -> Option<i32> {
        if unsafe { ffi::webrtc_VideoDecoder_Settings_has_buffer_pool_size(self.raw.as_ptr()) } == 0
        {
            return None;
        }
        Some(unsafe { ffi::webrtc_VideoDecoder_Settings_buffer_pool_size(self.raw.as_ptr()) })
    }

    pub fn max_render_resolution_width(&self) -> i32 {
        unsafe { ffi::webrtc_VideoDecoder_Settings_max_render_resolution_width(self.raw.as_ptr()) }
    }

    pub fn max_render_resolution_height(&self) -> i32 {
        unsafe { ffi::webrtc_VideoDecoder_Settings_max_render_resolution_height(self.raw.as_ptr()) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoder_Settings {
        self.raw.as_ptr()
    }
}

#[allow(dead_code)]
pub struct VideoDecoderDecodedImageCallbackRef<'a> {
    raw: NonNull<ffi::webrtc_VideoDecoder_DecodedImageCallback>,
    _marker: PhantomData<&'a ffi::webrtc_VideoDecoder_DecodedImageCallback>,
}

unsafe impl<'a> Send for VideoDecoderDecodedImageCallbackRef<'a> {}

impl<'a> VideoDecoderDecodedImageCallbackRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoDecoder_DecodedImageCallback` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoDecoder_DecodedImageCallback>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoder_DecodedImageCallback {
        self.raw.as_ptr()
    }

    pub fn decoded(&self, decoded_image: VideoFrameRef<'_>) {
        unsafe {
            ffi::webrtc_VideoDecoder_DecodedImageCallback_Decoded(
                self.raw.as_ptr(),
                decoded_image.as_ptr(),
            )
        };
    }
}

#[derive(Clone, Copy)]
pub struct VideoDecoderDecodedImageCallbackPtr {
    raw: NonNull<ffi::webrtc_VideoDecoder_DecodedImageCallback>,
}

unsafe impl Send for VideoDecoderDecodedImageCallbackPtr {}

impl VideoDecoderDecodedImageCallbackPtr {
    /// # Safety
    /// `callback` が指すオブジェクトは有効であり続ける必要があります。
    pub unsafe fn from_ref(callback: VideoDecoderDecodedImageCallbackRef<'_>) -> Self {
        Self { raw: callback.raw }
    }

    /// # Safety
    /// `raw` は有効な `webrtc_VideoDecoder_DecodedImageCallback` を指し、
    /// 呼び出し時点でも破棄されていない必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoDecoder_DecodedImageCallback>) -> Self {
        Self { raw }
    }

    /// # Safety
    /// `self` が保持するポインタは有効である必要があります。
    /// `register` の再呼び出しや `release` 後に使ってはいけません。
    pub unsafe fn decoded(&self, decoded_image: VideoFrameRef<'_>) {
        unsafe {
            ffi::webrtc_VideoDecoder_DecodedImageCallback_Decoded(
                self.raw.as_ptr(),
                decoded_image.as_ptr(),
            )
        };
    }
}

pub trait VideoDecoderHandler: Send {
    #[expect(unused_variables)]
    fn configure(&mut self, settings: VideoDecoderSettingsRef<'_>) -> bool {
        true
    }

    #[expect(unused_variables)]
    fn decode(
        &mut self,
        input_image: EncodedImageRef<'_>,
        render_time_ms: i64,
    ) -> VideoCodecStatus {
        VideoCodecStatus::Ok
    }

    #[expect(unused_variables)]
    fn register_decode_complete_callback(
        &mut self,
        callback: Option<VideoDecoderDecodedImageCallbackPtr>,
    ) -> VideoCodecStatus {
        VideoCodecStatus::Ok
    }

    fn release(&mut self) -> VideoCodecStatus {
        VideoCodecStatus::Ok
    }

    fn get_decoder_info(&mut self) -> VideoDecoderDecoderInfo {
        VideoDecoderDecoderInfo::new()
    }
}

pub trait VideoDecoderFactoryHandler: Send {
    fn get_supported_formats(&mut self) -> Vec<SdpVideoFormat> {
        Vec::new()
    }

    #[expect(unused_variables)]
    fn create(
        &mut self,
        env: EnvironmentRef<'_>,
        format: SdpVideoFormatRef<'_>,
    ) -> Option<VideoDecoder> {
        None
    }
}

struct VideoDecoderHandlerState {
    handler: Box<dyn VideoDecoderHandler>,
}

unsafe impl Send for VideoDecoderHandlerState {}

struct VideoDecoderFactoryHandlerState {
    handler: Box<dyn VideoDecoderFactoryHandler>,
}

unsafe impl Send for VideoDecoderFactoryHandlerState {}

unsafe extern "C" fn video_decoder_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_decoder_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderHandlerState) };
}

unsafe extern "C" fn video_decoder_configure(
    settings: *mut ffi::webrtc_VideoDecoder_Settings,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_configure: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderHandlerState) };
    let settings = NonNull::new(settings).expect("video_decoder_configure: settings is null");
    let settings = unsafe { VideoDecoderSettingsRef::from_raw(settings) };
    if state.handler.configure(settings) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn video_decoder_decode(
    input_image: *mut ffi::webrtc_EncodedImage,
    render_time_ms: i64,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_decode: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderHandlerState) };
    let input_image = NonNull::new(input_image).expect("video_decoder_decode: input_image is null");
    let input_image = unsafe { EncodedImageRef::from_raw(input_image) };
    state.handler.decode(input_image, render_time_ms).to_raw()
}

unsafe extern "C" fn video_decoder_register_decode_complete_callback(
    callback: *mut ffi::webrtc_VideoDecoder_DecodedImageCallback,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_register_decode_complete_callback: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderHandlerState) };
    let callback = NonNull::new(callback)
        .map(|callback| unsafe { VideoDecoderDecodedImageCallbackPtr::from_raw(callback) });
    state
        .handler
        .register_decode_complete_callback(callback)
        .to_raw()
}

unsafe extern "C" fn video_decoder_release(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_release: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderHandlerState) };
    state.handler.release().to_raw()
}

unsafe extern "C" fn video_decoder_get_decoder_info(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoDecoder_DecoderInfo_unique {
    assert!(
        !user_data.is_null(),
        "video_decoder_get_decoder_info: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderHandlerState) };
    state.handler.get_decoder_info().into_raw()
}

unsafe extern "C" fn video_decoder_factory_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_decoder_factory_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderFactoryHandlerState) };
}

unsafe extern "C" fn video_decoder_factory_get_supported_formats(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_SdpVideoFormat_vector {
    let empty = || unsafe { ffi::webrtc_SdpVideoFormat_vector_new() };
    assert!(
        !user_data.is_null(),
        "video_decoder_factory_get_supported_formats: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderFactoryHandlerState) };
    let formats = state.handler.get_supported_formats();
    let vec = empty();
    if vec.is_null() {
        return std::ptr::null_mut();
    }
    for format in &formats {
        unsafe { ffi::webrtc_SdpVideoFormat_vector_push_back(vec, format.raw().as_ptr()) };
    }
    vec
}

unsafe extern "C" fn video_decoder_factory_create(
    env: *mut ffi::webrtc_Environment,
    format: *mut ffi::webrtc_SdpVideoFormat,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoDecoder_unique {
    assert!(
        !user_data.is_null(),
        "video_decoder_factory_create: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderFactoryHandlerState) };
    let env = NonNull::new(env).expect("video_decoder_factory_create: env is null");
    let format = NonNull::new(format).expect("video_decoder_factory_create: format is null");
    let env = unsafe { EnvironmentRef::from_raw(env) };
    let format = unsafe { SdpVideoFormatRef::from_raw(format) };
    match state.handler.create(env, format) {
        Some(decoder) => decoder.into_raw(),
        None => std::ptr::null_mut(),
    }
}

/// webrtc::VideoDecoder のラッパー。
pub struct VideoDecoder {
    raw_unique: NonNull<ffi::webrtc_VideoDecoder_unique>,
}

unsafe impl Send for VideoDecoder {}

impl VideoDecoder {
    pub fn new_with_handler(handler: Box<dyn VideoDecoderHandler>) -> Self {
        let state = Box::new(VideoDecoderHandlerState { handler });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoDecoder_cbs {
            Configure: Some(video_decoder_configure),
            Decode: Some(video_decoder_decode),
            RegisterDecodeCompleteCallback: Some(video_decoder_register_decode_complete_callback),
            Release: Some(video_decoder_release),
            GetDecoderInfo: Some(video_decoder_get_decoder_info),
            OnDestroy: Some(video_decoder_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_VideoDecoder_new(&cbs, user_data) };
        let raw_unique = match NonNull::new(raw) {
            Some(raw_unique) => raw_unique,
            None => {
                let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderHandlerState) };
                panic!("BUG: webrtc_VideoDecoder_new が null を返しました");
            }
        };
        Self { raw_unique }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoder {
        unsafe { ffi::webrtc_VideoDecoder_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoDecoder_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    pub fn configure(&mut self, settings: VideoDecoderSettingsRef<'_>) -> bool {
        unsafe { ffi::webrtc_VideoDecoder_Configure(self.as_ptr(), settings.as_ptr()) != 0 }
    }

    pub fn decode(
        &mut self,
        input_image: EncodedImageRef<'_>,
        render_time_ms: i64,
    ) -> VideoCodecStatus {
        let value = unsafe {
            ffi::webrtc_VideoDecoder_Decode(self.as_ptr(), input_image.as_ptr(), render_time_ms)
        };
        VideoCodecStatus::from_raw(value)
    }

    pub fn register_decode_complete_callback(
        &mut self,
        callback: Option<VideoDecoderDecodedImageCallbackPtr>,
    ) -> VideoCodecStatus {
        let callback = callback.map_or(std::ptr::null_mut(), |callback| callback.raw.as_ptr());
        let value = unsafe {
            ffi::webrtc_VideoDecoder_RegisterDecodeCompleteCallback(self.as_ptr(), callback)
        };
        VideoCodecStatus::from_raw(value)
    }

    pub fn release(&mut self) -> VideoCodecStatus {
        let value = unsafe { ffi::webrtc_VideoDecoder_Release(self.as_ptr()) };
        VideoCodecStatus::from_raw(value)
    }

    pub fn get_decoder_info(&self) -> VideoDecoderDecoderInfo {
        let raw = unsafe { ffi::webrtc_VideoDecoder_GetDecoderInfo(self.as_ptr()) };
        let raw_unique =
            NonNull::new(raw).expect("webrtc_VideoDecoder_GetDecoderInfo が null を返しました");
        VideoDecoderDecoderInfo { raw_unique }
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoder_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::VideoDecoderFactory のラッパー。
pub struct VideoDecoderFactory {
    raw_unique: NonNull<ffi::webrtc_VideoDecoderFactory_unique>,
}

unsafe impl Send for VideoDecoderFactory {}

impl VideoDecoderFactory {
    pub fn builtin() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_CreateBuiltinVideoDecoderFactory() })
            .expect("webrtc_CreateBuiltinVideoDecoderFactory が null を返しました");
        Self { raw_unique: raw }
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn from_objc_default() -> Option<Self> {
        let objc_factory = unsafe { ffi::webrtc_objc_RTCDefaultVideoDecoderFactory_new() };
        if objc_factory.is_null() {
            return None;
        }

        let raw_unique = unsafe { ffi::webrtc_ObjCToNativeVideoDecoderFactory(objc_factory) };
        unsafe { ffi::webrtc_objc_RTCVideoDecoderFactory_release(objc_factory) };
        let raw_unique = NonNull::new(raw_unique)?;
        Some(Self { raw_unique })
    }

    #[cfg(target_os = "android")]
    pub fn from_android_default() -> Option<Self> {
        let env = unsafe { ffi::webrtc_jni_AttachCurrentThreadIfNeeded() };
        if env.is_null() {
            return None;
        }

        let class = unsafe {
            ffi::webrtc_GetClass(
                env,
                b"org/webrtc/DefaultVideoDecoderFactory\0".as_ptr().cast(),
            )
        };
        if class.is_null() {
            if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
                unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            }
            return None;
        }

        let ctor = unsafe {
            ffi::jni_JNIEnv_GetMethodID(
                env,
                class,
                b"<init>\0".as_ptr().cast(),
                b"(Lorg/webrtc/EglBase$Context;)V\0".as_ptr().cast(),
            )
        };
        if ctor.is_null() {
            unsafe { ffi::jni_JNIEnv_DeleteLocalRef(env, class) };
            if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
                unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            }
            return None;
        }

        let mut args: [ffi::jvalue; 1] = unsafe { std::mem::zeroed() };
        args[0].l = std::ptr::null_mut();
        let decoder_factory =
            unsafe { ffi::jni_JNIEnv_NewObjectA(env, class, ctor, args.as_ptr()) };
        if decoder_factory.is_null() {
            unsafe { ffi::jni_JNIEnv_DeleteLocalRef(env, class) };
            if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
                unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            }
            return None;
        }

        let raw_unique =
            unsafe { ffi::webrtc_JavaToNativeVideoDecoderFactory(env, decoder_factory) };
        unsafe {
            ffi::jni_JNIEnv_DeleteLocalRef(env, decoder_factory);
            ffi::jni_JNIEnv_DeleteLocalRef(env, class);
        }
        if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
            unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            return None;
        }

        let raw_unique = NonNull::new(raw_unique)?;
        Some(Self { raw_unique })
    }

    pub fn new_with_handler(handler: Box<dyn VideoDecoderFactoryHandler>) -> Self {
        let state = Box::new(VideoDecoderFactoryHandlerState { handler });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoDecoderFactory_cbs {
            GetSupportedFormats: Some(video_decoder_factory_get_supported_formats),
            Create: Some(video_decoder_factory_create),
            OnDestroy: Some(video_decoder_factory_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_VideoDecoderFactory_new(&cbs, user_data) };
        let raw_unique = match NonNull::new(raw) {
            Some(raw_unique) => raw_unique,
            None => {
                let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderFactoryHandlerState) };
                panic!("BUG: webrtc_VideoDecoderFactory_new が null を返しました");
            }
        };
        Self { raw_unique }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoderFactory {
        unsafe { ffi::webrtc_VideoDecoderFactory_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoDecoderFactory_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    pub fn create(
        &self,
        env: EnvironmentRef<'_>,
        format: SdpVideoFormatRef<'_>,
    ) -> Option<VideoDecoder> {
        let raw = unsafe {
            ffi::webrtc_VideoDecoderFactory_Create(self.as_ptr(), env.as_ptr(), format.as_ptr())
        };
        Some(VideoDecoder {
            raw_unique: NonNull::new(raw)?,
        })
    }

    pub fn get_supported_formats(&self) -> Vec<SdpVideoFormat> {
        let raw_vec = unsafe { ffi::webrtc_VideoDecoderFactory_GetSupportedFormats(self.as_ptr()) };
        let raw_vec = NonNull::new(raw_vec)
            .expect("BUG: webrtc_VideoDecoderFactory_GetSupportedFormats が null を返しました");
        let size = unsafe { ffi::webrtc_SdpVideoFormat_vector_size(raw_vec.as_ptr()) };
        let mut formats = Vec::with_capacity(size.max(0) as usize);
        for i in 0..size {
            let raw_format = unsafe { ffi::webrtc_SdpVideoFormat_vector_get(raw_vec.as_ptr(), i) };
            let raw_format = NonNull::new(raw_format)
                .expect("BUG: webrtc_SdpVideoFormat_vector_get が null を返しました");
            let format_ref = unsafe { SdpVideoFormatRef::from_raw(raw_format) };
            formats.push(format_ref.to_owned());
        }
        unsafe { ffi::webrtc_SdpVideoFormat_vector_delete(raw_vec.as_ptr()) };
        formats
    }
}

impl Drop for VideoDecoderFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoderFactory_unique_delete(self.raw_unique.as_ptr()) };
    }
}
