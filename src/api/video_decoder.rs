use super::video_codec_common::{
    EncodedImageRef, SdpVideoFormat, SdpVideoFormatRef, VideoCodecStatus, VideoCodecType,
};
use crate::{CxxString, EnvironmentRef, Result, ffi};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub struct VideoDecoderDecoderInfo {
    raw_unique: NonNull<ffi::webrtc_VideoDecoder_DecoderInfo_unique>,
}

impl Default for VideoDecoderDecoderInfo {
    fn default() -> Self {
        Self::new()
    }
}

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

impl Drop for VideoDecoderDecoderInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for VideoDecoderDecoderInfo {}

pub struct VideoDecoderSettingsRef<'a> {
    raw: NonNull<ffi::webrtc_VideoDecoder_Settings>,
    _marker: PhantomData<&'a ffi::webrtc_VideoDecoder_Settings>,
}

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
}

unsafe impl<'a> Send for VideoDecoderSettingsRef<'a> {}

#[allow(dead_code)]
pub struct VideoDecoderDecodedImageCallbackRef<'a> {
    raw: NonNull<ffi::webrtc_VideoDecoder_DecodedImageCallback>,
    _marker: PhantomData<&'a ffi::webrtc_VideoDecoder_DecodedImageCallback>,
}

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
}

unsafe impl<'a> Send for VideoDecoderDecodedImageCallbackRef<'a> {}

type VideoDecoderConfigureCallback =
    Box<dyn for<'a> FnMut(VideoDecoderSettingsRef<'a>) -> bool + Send + 'static>;
type VideoDecoderDecodeCallback =
    Box<dyn for<'a> FnMut(EncodedImageRef<'a>, i64) -> VideoCodecStatus + Send + 'static>;
type VideoDecoderRegisterDecodeCompleteCallback = Box<
    dyn for<'a> FnMut(Option<VideoDecoderDecodedImageCallbackRef<'a>>) -> VideoCodecStatus
        + Send
        + 'static,
>;
type VideoDecoderReleaseCallback = Box<dyn FnMut() -> VideoCodecStatus + Send + 'static>;
type VideoDecoderGetDecoderInfoCallback =
    Box<dyn FnMut() -> VideoDecoderDecoderInfo + Send + 'static>;

#[derive(Default)]
pub struct VideoDecoderCallbacks {
    pub configure: Option<VideoDecoderConfigureCallback>,
    pub decode: Option<VideoDecoderDecodeCallback>,
    pub register_decode_complete_callback: Option<VideoDecoderRegisterDecodeCompleteCallback>,
    pub release: Option<VideoDecoderReleaseCallback>,
    pub get_decoder_info: Option<VideoDecoderGetDecoderInfoCallback>,
}

impl VideoDecoderCallbacks {
    fn with_defaults(mut self) -> Self {
        if self.configure.is_none() {
            self.configure = Some(Box::new(|_| true));
        }
        if self.decode.is_none() {
            self.decode = Some(Box::new(|_, _| VideoCodecStatus::Ok));
        }
        if self.register_decode_complete_callback.is_none() {
            self.register_decode_complete_callback = Some(Box::new(|_| VideoCodecStatus::Ok));
        }
        if self.release.is_none() {
            self.release = Some(Box::new(|| VideoCodecStatus::Ok));
        }
        if self.get_decoder_info.is_none() {
            self.get_decoder_info = Some(Box::new(VideoDecoderDecoderInfo::new));
        }
        self
    }
}

type VideoDecoderFactoryGetSupportedFormatsCallback =
    Box<dyn FnMut() -> Vec<SdpVideoFormat> + Send + 'static>;
type VideoDecoderFactoryCreateCallback = Box<
    dyn for<'a> FnMut(EnvironmentRef<'a>, SdpVideoFormatRef<'a>) -> Option<VideoDecoder>
        + Send
        + 'static,
>;

#[derive(Default)]
pub struct VideoDecoderFactoryCallbacks {
    pub get_supported_formats: Option<VideoDecoderFactoryGetSupportedFormatsCallback>,
    pub create: Option<VideoDecoderFactoryCreateCallback>,
}

impl VideoDecoderFactoryCallbacks {
    fn with_defaults(mut self) -> Self {
        if self.get_supported_formats.is_none() {
            self.get_supported_formats = Some(Box::new(Vec::new));
        }
        if self.create.is_none() {
            self.create = Some(Box::new(|_, _| None));
        }
        self
    }
}

struct VideoDecoderCallbackState {
    callbacks: VideoDecoderCallbacks,
}

struct VideoDecoderFactoryCallbackState {
    callbacks: VideoDecoderFactoryCallbacks,
}

unsafe extern "C" fn video_decoder_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderCallbackState) };
}

unsafe extern "C" fn video_decoder_configure(
    settings: *mut ffi::webrtc_VideoDecoder_Settings,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_configure: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .configure
        .as_mut()
        .expect("video_decoder_configure: callback is None");
    let settings = NonNull::new(settings).expect("video_decoder_configure: settings is null");
    let settings = unsafe { VideoDecoderSettingsRef::from_raw(settings) };
    if cb(settings) { 1 } else { 0 }
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
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .decode
        .as_mut()
        .expect("video_decoder_decode: callback is None");
    let input_image = NonNull::new(input_image).expect("video_decoder_decode: input_image is null");
    let input_image = unsafe { EncodedImageRef::from_raw(input_image) };
    cb(input_image, render_time_ms).to_raw()
}

unsafe extern "C" fn video_decoder_register_decode_complete_callback(
    callback: *mut ffi::webrtc_VideoDecoder_DecodedImageCallback,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_register_decode_complete_callback: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .register_decode_complete_callback
        .as_mut()
        .expect("video_decoder_register_decode_complete_callback: callback is None");
    let callback = NonNull::new(callback)
        .map(|callback| unsafe { VideoDecoderDecodedImageCallbackRef::from_raw(callback) });
    cb(callback).to_raw()
}

unsafe extern "C" fn video_decoder_release(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_release: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .release
        .as_mut()
        .expect("video_decoder_release: callback is None");
    cb().to_raw()
}

unsafe extern "C" fn video_decoder_get_decoder_info(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoDecoder_DecoderInfo_unique {
    assert!(
        !user_data.is_null(),
        "video_decoder_get_decoder_info: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .get_decoder_info
        .as_mut()
        .expect("video_decoder_get_decoder_info: callback is None");
    cb().into_raw()
}

unsafe extern "C" fn video_decoder_factory_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderFactoryCallbackState) };
}

unsafe extern "C" fn video_decoder_factory_get_supported_formats(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_SdpVideoFormat_vector {
    let empty = || unsafe { ffi::webrtc_SdpVideoFormat_vector_new() };
    assert!(
        !user_data.is_null(),
        "video_decoder_factory_get_supported_formats: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderFactoryCallbackState) };
    let cb = state
        .callbacks
        .get_supported_formats
        .as_mut()
        .expect("video_decoder_factory_get_supported_formats: callback is None");
    let formats = cb();
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
    let state = unsafe { &mut *(user_data as *mut VideoDecoderFactoryCallbackState) };
    let cb = state
        .callbacks
        .create
        .as_mut()
        .expect("video_decoder_factory_create: callback is None");
    let env = NonNull::new(env).expect("video_decoder_factory_create: env is null");
    let format = NonNull::new(format).expect("video_decoder_factory_create: format is null");
    let env = unsafe { EnvironmentRef::from_raw(env) };
    let format = unsafe { SdpVideoFormatRef::from_raw(format) };
    match cb(env, format) {
        Some(decoder) => decoder.into_raw(),
        None => std::ptr::null_mut(),
    }
}

/// webrtc::VideoDecoder のラッパー。
pub struct VideoDecoder {
    raw_unique: NonNull<ffi::webrtc_VideoDecoder_unique>,
}

impl VideoDecoder {
    pub fn new_with_callbacks(callbacks: VideoDecoderCallbacks) -> Self {
        let state = Box::new(VideoDecoderCallbackState {
            callbacks: callbacks.with_defaults(),
        });
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
                let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderCallbackState) };
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

    pub fn configure(&self) -> bool {
        unsafe { ffi::webrtc_VideoDecoder_Configure(self.as_ptr(), std::ptr::null_mut()) != 0 }
    }

    pub fn decode(
        &self,
        input_image: Option<EncodedImageRef<'_>>,
        render_time_ms: i64,
    ) -> VideoCodecStatus {
        let input_image =
            input_image.map_or(std::ptr::null_mut(), |input_image| input_image.as_ptr());
        let value =
            unsafe { ffi::webrtc_VideoDecoder_Decode(self.as_ptr(), input_image, render_time_ms) };
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

impl VideoDecoderFactory {
    pub fn builtin() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_CreateBuiltinVideoDecoderFactory() })
            .expect("webrtc_CreateBuiltinVideoDecoderFactory が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn new_with_callbacks(callbacks: VideoDecoderFactoryCallbacks) -> Self {
        let state = Box::new(VideoDecoderFactoryCallbackState {
            callbacks: callbacks.with_defaults(),
        });
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
                let _ =
                    unsafe { Box::from_raw(user_data as *mut VideoDecoderFactoryCallbackState) };
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
        env: &crate::Environment,
        format: &SdpVideoFormat,
    ) -> Option<VideoDecoder> {
        let raw = unsafe {
            ffi::webrtc_VideoDecoderFactory_Create(
                self.as_ptr(),
                env.as_ptr(),
                format.raw().as_ptr(),
            )
        };
        Some(VideoDecoder {
            raw_unique: NonNull::new(raw)?,
        })
    }
}

impl Drop for VideoDecoderFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoderFactory_unique_delete(self.raw_unique.as_ptr()) };
    }
}
