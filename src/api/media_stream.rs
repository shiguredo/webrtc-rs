use crate::ref_count::{AudioTrackHandle, MediaStreamHandle, VideoTrackHandle};
use crate::{AudioTrack, CxxString, Result, ScopedRef, VideoTrack, ffi};
use std::ptr::NonNull;

/// webrtc::MediaStreamInterface のラッパー。
pub struct MediaStream {
    raw_ref: ScopedRef<MediaStreamHandle>,
}

impl MediaStream {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<MediaStreamHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_MediaStreamInterface {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_MediaStreamInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    pub fn id(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_MediaStreamInterface_id(self.raw_ref.as_ptr()) };
        let id = CxxString::from_unique(
            NonNull::new(ptr).expect("BUG: webrtc_MediaStreamInterface_id returned null"),
        );
        id.to_string()
    }

    pub fn audio_tracks(&self) -> Vec<AudioTrack> {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_MediaStreamInterface_GetAudioTracks(self.raw_ref.as_ptr())
        })
        .expect("BUG: webrtc_MediaStreamInterface_GetAudioTracks returned null");
        let len = unsafe { ffi::webrtc_AudioTrackInterface_refcounted_vector_size(raw.as_ptr()) }
            .max(0) as usize;
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            let track = unsafe {
                ffi::webrtc_AudioTrackInterface_refcounted_vector_get(raw.as_ptr(), i as i32)
            };
            if let Some(track) = NonNull::new(track) {
                let raw_ref = ScopedRef::<AudioTrackHandle>::from_raw(track);
                out.push(AudioTrack::from_scoped_ref(raw_ref));
            }
        }
        unsafe { ffi::webrtc_AudioTrackInterface_refcounted_vector_delete(raw.as_ptr()) };
        out
    }

    pub fn video_tracks(&self) -> Vec<VideoTrack> {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_MediaStreamInterface_GetVideoTracks(self.raw_ref.as_ptr())
        })
        .expect("BUG: webrtc_MediaStreamInterface_GetVideoTracks returned null");
        let len = unsafe { ffi::webrtc_VideoTrackInterface_refcounted_vector_size(raw.as_ptr()) }
            .max(0) as usize;
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            let track = unsafe {
                ffi::webrtc_VideoTrackInterface_refcounted_vector_get(raw.as_ptr(), i as i32)
            };
            if let Some(track) = NonNull::new(track) {
                let raw_ref = ScopedRef::<VideoTrackHandle>::from_raw(track);
                out.push(VideoTrack::from_scoped_ref(raw_ref));
            }
        }
        unsafe { ffi::webrtc_VideoTrackInterface_refcounted_vector_delete(raw.as_ptr()) };
        out
    }

    pub fn find_audio_track(&self, track_id: &str) -> Option<AudioTrack> {
        let raw = unsafe {
            ffi::webrtc_MediaStreamInterface_FindAudioTrack(
                self.raw_ref.as_ptr(),
                track_id.as_ptr() as *const _,
                track_id.len(),
            )
        };
        let raw = NonNull::new(raw)?;
        let raw_ref = ScopedRef::<AudioTrackHandle>::from_raw(raw);
        Some(AudioTrack::from_scoped_ref(raw_ref))
    }

    pub fn find_video_track(&self, track_id: &str) -> Option<VideoTrack> {
        let raw = unsafe {
            ffi::webrtc_MediaStreamInterface_FindVideoTrack(
                self.raw_ref.as_ptr(),
                track_id.as_ptr() as *const _,
                track_id.len(),
            )
        };
        let raw = NonNull::new(raw)?;
        let raw_ref = ScopedRef::<VideoTrackHandle>::from_raw(raw);
        Some(VideoTrack::from_scoped_ref(raw_ref))
    }

    pub fn add_audio_track(&self, track: &AudioTrack) -> bool {
        unsafe {
            ffi::webrtc_MediaStreamInterface_AddTrackWithAudioTrack(
                self.raw_ref.as_ptr(),
                track.as_refcounted_ptr(),
            ) != 0
        }
    }

    pub fn add_video_track(&self, track: &VideoTrack) -> bool {
        unsafe {
            ffi::webrtc_MediaStreamInterface_AddTrackWithVideoTrack(
                self.raw_ref.as_ptr(),
                track.as_refcounted_ptr(),
            ) != 0
        }
    }

    pub fn remove_audio_track(&self, track: &AudioTrack) -> bool {
        unsafe {
            ffi::webrtc_MediaStreamInterface_RemoveTrackWithAudioTrack(
                self.raw_ref.as_ptr(),
                track.as_refcounted_ptr(),
            ) != 0
        }
    }

    pub fn remove_video_track(&self, track: &VideoTrack) -> bool {
        unsafe {
            ffi::webrtc_MediaStreamInterface_RemoveTrackWithVideoTrack(
                self.raw_ref.as_ptr(),
                track.as_refcounted_ptr(),
            ) != 0
        }
    }
}

impl Clone for MediaStream {
    fn clone(&self) -> Self {
        Self {
            raw_ref: self.raw_ref.clone(),
        }
    }
}

unsafe impl Send for MediaStream {}
// ここで生成する MediaStreamInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/media_stream_proxy.h;l=21-40;drc=702cda56c1ee1a82d800698dd6e863e45fe5da49
unsafe impl Sync for MediaStream {}
