use super::*;
use std::ptr::NonNull;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

struct NoopHandler;

impl AudioDeviceModuleHandler for NoopHandler {}
impl PeerConnectionObserverHandler for NoopHandler {}
impl DtlsTransportObserverHandler for NoopHandler {}
impl CreateSessionDescriptionObserverHandler for NoopHandler {}
impl SetLocalDescriptionObserverHandler for NoopHandler {}
impl SetRemoteDescriptionObserverHandler for NoopHandler {}
impl VideoEncoderHandler for NoopHandler {}
impl VideoDecoderHandler for NoopHandler {}

#[test]
fn create_and_drop_environment() {
    let _env = Environment::new();
}

#[test]
fn cxx_string_round_trip() {
    let mut s = CxxString::from_str("hello");
    assert_eq!(s.len(), 5);
    assert_eq!(
        s.to_string().expect("CxxString の変換に失敗しました"),
        "hello"
    );

    s.append(" world");
    assert_eq!(
        s.to_string().expect("CxxString の変換に失敗しました"),
        "hello world"
    );

    let r = CxxStringRef::from_ptr(NonNull::new(s.as_ptr()).unwrap());
    assert_eq!(r.len(), 11);
    assert_eq!(
        r.to_string().expect("CxxStringRef の変換に失敗しました"),
        "hello world"
    );
}

#[test]
fn time_millis_moves_forward() {
    let start = time_millis();
    std::thread::sleep(Duration::from_millis(10));
    let end = time_millis();
    assert!(
        end >= start,
        "time_millis が単調増加していません: start={start}, end={end}"
    );
}

#[test]
fn random_string_has_requested_length() {
    let s = random_string(8);
    assert_eq!(s.len(), 8);
}

#[test]
fn random_bytes_length_matches() {
    let b = random_bytes(16);
    assert_eq!(b.len(), 16);
}

#[test]
fn timestamp_aligner_translates() {
    let mut aligner = TimestampAligner::new();
    let base = aligner.translate(1_000_000, 2_000_000);
    let later = aligner.translate(2_000_000, 3_000_000);
    assert!(
        later >= base,
        "TimestampAligner の結果が期待と異なります: base={base}, later={later}"
    );
}

#[test]
fn string_vector_push_and_get() {
    let mut vec = StringVector::new(0);
    let hello = CxxString::from_str("hello");
    let world = CxxString::from_str("world");
    vec.push(&hello);
    vec.push(&world);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec.get(0).expect("0 番目の取得に失敗しました"), "hello");
    assert_eq!(vec.get(1).expect("1 番目の取得に失敗しました"), "world");
}

#[test]
fn sdp_type_round_trip() {
    let offer = SdpType::Offer;
    let val = offer.to_int();
    let back = SdpType::from_int(val);
    assert_eq!(back, SdpType::Offer);
}

#[test]
fn media_type_constants() {
    assert_eq!(
        MediaType::from_int(MediaType::Audio.to_int()),
        MediaType::Audio
    );
    assert_eq!(
        MediaType::from_int(MediaType::Video.to_int()),
        MediaType::Video
    );
}

#[test]
fn common_constants_values() {
    assert_eq!(no_picture_id(), -1);
    assert_eq!(no_tl0_pic_idx(), -1);
    assert_eq!(no_temporal_idx(), 0xFF);
    assert_eq!(no_key_idx(), -1);
}

#[test]
fn session_description_to_string() {
    // datachannel 用の最小構成 SDP を使う。
    let sdp = "v=0\r\n\
                   o=- 0 0 IN IP4 127.0.0.1\r\n\
                   s=-\r\n\
                   t=0 0\r\n\
                   a=group:BUNDLE 0\r\n\
                   m=application 9 UDP/DTLS/SCTP webrtc-datachannel\r\n\
                   c=IN IP4 0.0.0.0\r\n\
                   a=mid:0\r\n\
                   a=sctp-port:5000\r\n\
                   a=max-message-size:262144\r\n";
    let desc = SessionDescription::new(SdpType::Offer, sdp)
        .expect("SessionDescription の生成に失敗しました");
    assert_eq!(desc.sdp_type(), SdpType::Offer);
    let out = desc.to_string().expect("SDP の文字列化に失敗しました");
    assert!(
        out.contains("m=application"),
        "SDP に datachannel 用 m=application が含まれていません: {out}"
    );
}

#[test]
fn sdp_video_format_with_parameters() {
    let mut fmt = SdpVideoFormat::new_with_parameters(
        "VP8",
        &std::collections::HashMap::from([
            (String::from("profile-id"), String::from("0")),
            (String::from("level"), String::from("3.1")),
        ]),
        &[ScalabilityMode::L1T1, ScalabilityMode::L1T2],
    );
    let params = fmt.parameters_mut();
    assert_eq!(params.len(), 2);

    let mut found = std::collections::HashMap::new();
    for (k, v) in params.iter() {
        found.insert(k, v);
    }
    assert_eq!(found.get("profile-id").map(String::as_str), Some("0"));
    assert_eq!(found.get("level").map(String::as_str), Some("3.1"));
    assert_eq!(
        fmt.scalability_modes(),
        vec![ScalabilityMode::L1T1, ScalabilityMode::L1T2]
    );

    let other = SdpVideoFormat::new_with_parameters(
        "VP8",
        &std::collections::HashMap::from([
            (String::from("profile-id"), String::from("0")),
            (String::from("level"), String::from("3.1")),
        ]),
        &[ScalabilityMode::L1T1, ScalabilityMode::L1T2],
    );

    assert!(fmt.is_equal(other.as_ref()));

    let mut cloned = fmt.clone();
    assert!(fmt.is_equal(cloned.as_ref()));
    {
        let mut params = cloned.parameters_mut();
        params.set("packetization-mode", "1");
    }
    let mut has_packetization_mode = false;
    for (k, _) in fmt.parameters_mut().iter() {
        if k == "packetization-mode" {
            has_packetization_mode = true;
            break;
        }
    }
    assert!(
        !has_packetization_mode,
        "clone への変更が元の SdpVideoFormat に影響しています"
    );
}

#[test]
fn sdp_video_format_new_has_empty_scalability_modes() {
    let fmt = SdpVideoFormat::new("VP8");
    assert!(fmt.scalability_modes().is_empty());
}

#[test]
fn fuzzy_match_sdp_video_format_prefers_more_parameter_matches() {
    let supported_formats = vec![
        SdpVideoFormat::new_with_parameters(
            "H264",
            &std::collections::HashMap::from([
                (String::from("profile-level-id"), String::from("42e01f")),
                (String::from("packetization-mode"), String::from("1")),
            ]),
            &[],
        ),
        SdpVideoFormat::new_with_parameters(
            "H264",
            &std::collections::HashMap::from([
                (String::from("profile-level-id"), String::from("42e01f")),
                (String::from("packetization-mode"), String::from("0")),
            ]),
            &[],
        ),
    ];
    let requested = SdpVideoFormat::new_with_parameters(
        "H264",
        &std::collections::HashMap::from([
            (String::from("profile-level-id"), String::from("42e01f")),
            (String::from("packetization-mode"), String::from("1")),
            (String::from("x-google-start-bitrate"), String::from("500")),
        ]),
        &[],
    );

    let mut matched = fuzzy_match_sdp_video_format(&supported_formats, requested.as_ref())
        .expect("fuzzy_match_sdp_video_format should return a matched format");
    let params = matched
        .parameters_mut()
        .iter()
        .collect::<std::collections::HashMap<String, String>>();

    assert_eq!(
        params.get("packetization-mode").map(String::as_str),
        Some("1")
    );
}

#[test]
fn fuzzy_match_sdp_video_format_keeps_first_candidate_on_tie() {
    let supported_formats = vec![
        SdpVideoFormat::new_with_parameters(
            "H264",
            &std::collections::HashMap::from([(
                String::from("x-google-start-bitrate"),
                String::from("300"),
            )]),
            &[],
        ),
        SdpVideoFormat::new_with_parameters(
            "H264",
            &std::collections::HashMap::from([(
                String::from("x-google-start-bitrate"),
                String::from("500"),
            )]),
            &[],
        ),
    ];
    let requested = SdpVideoFormat::new("H264");

    let mut matched = fuzzy_match_sdp_video_format(&supported_formats, requested.as_ref())
        .expect("fuzzy_match_sdp_video_format should return a matched format");
    let params = matched
        .parameters_mut()
        .iter()
        .collect::<std::collections::HashMap<String, String>>();

    assert_eq!(
        params.get("x-google-start-bitrate").map(String::as_str),
        Some("300")
    );
}

#[test]
fn fuzzy_match_sdp_video_format_returns_none_for_different_codec_name() {
    let supported_formats = vec![SdpVideoFormat::new("VP8")];
    let requested = SdpVideoFormat::new("H264");

    assert!(fuzzy_match_sdp_video_format(&supported_formats, requested.as_ref()).is_none());
}

#[test]
fn sdp_video_format_is_same_codec_follows_codec_specific_rules() {
    let h264_upper = SdpVideoFormat::new("H264");
    let h264_lower = SdpVideoFormat::new("h264");
    assert!(h264_upper.is_same_codec(h264_lower.as_ref()));

    let h264_packetization_mode_1 = SdpVideoFormat::new_with_parameters(
        "H264",
        &std::collections::HashMap::from([(String::from("packetization-mode"), String::from("1"))]),
        &[],
    );
    assert!(!h264_upper.is_same_codec(h264_packetization_mode_1.as_ref()));

    let h264_profile_a = SdpVideoFormat::new_with_parameters(
        "H264",
        &std::collections::HashMap::from([(
            String::from("profile-level-id"),
            String::from("42e01f"),
        )]),
        &[],
    );
    let h264_profile_b = SdpVideoFormat::new_with_parameters(
        "H264",
        &std::collections::HashMap::from([(
            String::from("profile-level-id"),
            String::from("640c34"),
        )]),
        &[],
    );
    assert!(!h264_profile_a.is_same_codec(h264_profile_b.as_ref()));

    let vp9_profile_0 = SdpVideoFormat::new_with_parameters(
        "VP9",
        &std::collections::HashMap::from([(String::from("profile-id"), String::from("0"))]),
        &[],
    );
    let vp9_profile_2 = SdpVideoFormat::new_with_parameters(
        "VP9",
        &std::collections::HashMap::from([(String::from("profile-id"), String::from("2"))]),
        &[],
    );
    assert!(!vp9_profile_0.is_same_codec(vp9_profile_2.as_ref()));
    assert!(vp9_profile_0.is_same_codec(vp9_profile_0.clone().as_ref()));
}

#[test]
fn scalability_mode_round_trip() {
    let mode = ScalabilityMode::L2T2;
    assert_eq!(
        mode.as_str()
            .expect("ScalabilityMode の文字列化に失敗しました"),
        "L2T2"
    );
}

#[test]
fn i420_buffer_and_video_frame() {
    let mut buf = I420Buffer::new(4, 4);
    buf.y_data_mut().fill(0x10);
    buf.u_data_mut().fill(0x80);
    buf.v_data_mut().fill(0x90);

    let frame_buffer = buf.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(12345)
        .set_timestamp_rtp(0)
        .build();
    assert_eq!(frame.width(), 4);
    assert_eq!(frame.height(), 4);
    assert_eq!(frame.timestamp_us(), 12345);

    let mut copied = frame.buffer();
    let copied = copied
        .to_i420()
        .expect("VideoFrameBuffer から I420Buffer への変換に失敗しました");
    assert_eq!(copied.y_data()[0], 0x10);
}

#[test]
fn video_frame_set_video_frame_buffer_replaces_buffer() {
    let src = I420Buffer::new(2, 2);
    let src_buffer = src.cast_to_video_frame_buffer();
    let dst = I420Buffer::new(4, 2);
    let dst_buffer = dst.cast_to_video_frame_buffer();

    let mut frame = VideoFrame::builder(&src_buffer)
        .set_timestamp_us(123)
        .build();
    frame.set_video_frame_buffer(&dst_buffer);

    assert_eq!(frame.width(), 4);
    assert_eq!(frame.height(), 2);
    assert_eq!(frame.timestamp_us(), 123);
}

#[test]
fn video_codec_ref_getter_setter_and_simulcast_stream_ref_roundtrip() {
    let mut codec = VideoCodec::new();
    codec.set_codec_type(VideoCodecType::Av1);
    codec.set_width(1280);
    codec.set_height(720);
    codec.set_start_bitrate_kbps(1200);
    codec.set_min_bitrate_kbps(300);
    codec.set_max_bitrate_kbps(2500);
    codec.set_max_framerate(60);
    codec.set_number_of_simulcast_streams(2);

    assert_eq!(codec.codec_type(), VideoCodecType::Av1);
    assert_eq!(codec.width(), 1280);
    assert_eq!(codec.height(), 720);
    assert_eq!(codec.start_bitrate_kbps(), 1200);
    assert_eq!(codec.min_bitrate_kbps(), 300);
    assert_eq!(codec.max_bitrate_kbps(), 2500);
    assert_eq!(codec.max_framerate(), 60);
    assert_eq!(codec.number_of_simulcast_streams(), 2);

    {
        let mut stream0 = codec
            .simulcast_stream(0)
            .expect("simulcast stream 0 の取得に失敗");
        stream0.set_width(640);
        stream0.set_height(360);
        stream0.set_min_bitrate_kbps(150);
        stream0.set_target_bitrate_kbps(500);
        stream0.set_max_bitrate_kbps(900);
        assert_eq!(stream0.width(), 640);
        assert_eq!(stream0.height(), 360);
        assert_eq!(stream0.min_bitrate_kbps(), 150);
        assert_eq!(stream0.target_bitrate_kbps(), 500);
        assert_eq!(stream0.max_bitrate_kbps(), 900);
    }
    {
        let mut stream1 = codec
            .simulcast_stream(1)
            .expect("simulcast stream 1 の取得に失敗");
        stream1.set_width(320);
        stream1.set_height(180);
        stream1.set_min_bitrate_kbps(80);
        stream1.set_target_bitrate_kbps(240);
        stream1.set_max_bitrate_kbps(400);
        assert_eq!(stream1.width(), 320);
        assert_eq!(stream1.height(), 180);
        assert_eq!(stream1.min_bitrate_kbps(), 80);
        assert_eq!(stream1.target_bitrate_kbps(), 240);
        assert_eq!(stream1.max_bitrate_kbps(), 400);
    }

    assert!(codec.simulcast_stream(2).is_none());
    let cloned = codec.as_ref().to_owned();
    assert_eq!(cloned.codec_type(), VideoCodecType::Av1);
    assert_eq!(cloned.width(), 1280);
    assert_eq!(cloned.height(), 720);
    assert_eq!(cloned.number_of_simulcast_streams(), 2);
}

#[test]
fn i420_buffer_mutable_planes_and_video_frame_rtp_timestamp() {
    let mut buf = I420Buffer::new(4, 4);
    buf.y_data_mut().fill(0x11);
    buf.u_data_mut().fill(0x22);
    buf.v_data_mut().fill(0x33);
    assert!(buf.y_data().iter().all(|&v| v == 0x11));
    assert!(buf.u_data().iter().all(|&v| v == 0x22));
    assert!(buf.v_data().iter().all(|&v| v == 0x33));

    let frame_buffer = buf.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(12345)
        .set_timestamp_rtp(67890)
        .build();
    assert_eq!(frame.timestamp_us(), 12345);
    assert_eq!(frame.rtp_timestamp(), 67890);
    assert_eq!(frame.as_ref().rtp_timestamp(), 67890);
}

#[test]
fn video_frame_clone() {
    let mut buf = I420Buffer::new(4, 4);
    buf.y_data_mut().fill(0x44);
    buf.u_data_mut().fill(0x55);
    buf.v_data_mut().fill(0x66);

    let frame_buffer = buf.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(11111)
        .set_timestamp_rtp(22222)
        .build();
    let cloned = frame.clone();

    assert_eq!(cloned.width(), frame.width());
    assert_eq!(cloned.height(), frame.height());
    assert_eq!(cloned.timestamp_us(), frame.timestamp_us());
    assert_eq!(cloned.rtp_timestamp(), frame.rtp_timestamp());
    assert_ne!(cloned.as_ref().as_ptr(), frame.as_ref().as_ptr());

    let mut copied = cloned.buffer();
    let copied = copied
        .to_i420()
        .expect("clone した VideoFrame の buffer 変換に失敗しました");
    assert_eq!(copied.y_data()[0], 0x44);
}

#[test]
fn video_frame_ref_to_owned() {
    let mut buf = I420Buffer::new(4, 4);
    buf.y_data_mut().fill(0x77);
    buf.u_data_mut().fill(0x88);
    buf.v_data_mut().fill(0x99);

    let frame_buffer = buf.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(33333)
        .set_timestamp_rtp(44444)
        .build();
    let copied = frame.as_ref().to_owned();

    assert_eq!(copied.width(), frame.width());
    assert_eq!(copied.height(), frame.height());
    assert_eq!(copied.timestamp_us(), frame.timestamp_us());
    assert_eq!(copied.rtp_timestamp(), frame.rtp_timestamp());
    assert_ne!(copied.as_ref().as_ptr(), frame.as_ref().as_ptr());

    let mut copied_buffer = copied.buffer();
    let copied_i420 = copied_buffer
        .to_i420()
        .expect("to_owned した VideoFrame の buffer 変換に失敗しました");
    assert_eq!(copied_i420.y_data()[0], 0x77);
}

#[test]
fn video_frame_update_rect_roundtrip() {
    let mut rect = VideoFrameUpdateRect::new();
    rect.set_offset_x(11);
    rect.set_offset_y(22);
    rect.set_width(33);
    rect.set_height(44);

    assert_eq!(rect.offset_x(), 11);
    assert_eq!(rect.offset_y(), 22);
    assert_eq!(rect.width(), 33);
    assert_eq!(rect.height(), 44);
}

#[test]
fn video_frame_builder_roundtrip_all_fields() {
    let i420 = I420Buffer::new(4, 4);
    let frame_buffer = i420.cast_to_video_frame_buffer();
    let mut update_rect = VideoFrameUpdateRect::new();
    update_rect.set_offset_x(1);
    update_rect.set_offset_y(2);
    update_rect.set_width(3);
    update_rect.set_height(4);
    let color_space = ColorSpace::new();
    let color_space_string = color_space
        .as_string()
        .expect("ColorSpace::as_string に失敗しました");

    let presentation_timestamp = Duration::from_micros(1_234_567);
    let reference_time = Duration::from_micros(2_345_678);
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(765_432)
        .set_timestamp_rtp(1122)
        .set_id(5566)
        .set_ntp_time_ms(7788)
        .set_rotation(VideoRotation::R270)
        .set_presentation_timestamp(Some(presentation_timestamp))
        .set_reference_time(Some(reference_time))
        .set_color_space(Some(&color_space))
        .set_update_rect(Some(&update_rect))
        .set_is_repeat_frame(true)
        .build();

    assert_eq!(frame.timestamp_us(), 765_432);
    assert_eq!(frame.rtp_timestamp(), 1122);
    assert_eq!(frame.id(), 5566);
    assert_eq!(frame.ntp_time_ms(), 7788);
    assert_eq!(frame.rotation(), VideoRotation::R270);
    assert_eq!(frame.presentation_timestamp(), Some(presentation_timestamp));
    assert_eq!(frame.reference_time(), Some(reference_time));
    assert!(frame.has_update_rect());
    assert!(frame.is_repeat_frame());
    let frame_update_rect = frame.update_rect();
    assert_eq!(frame_update_rect.offset_x(), 1);
    assert_eq!(frame_update_rect.offset_y(), 2);
    assert_eq!(frame_update_rect.width(), 3);
    assert_eq!(frame_update_rect.height(), 4);
    let frame_color_space = frame
        .color_space()
        .expect("ColorSpace が設定されていません");
    assert_eq!(
        frame_color_space
            .as_string()
            .expect("VideoFrame::color_space の as_string に失敗しました"),
        color_space_string
    );

    let frame_ref = frame.as_ref();
    assert_eq!(frame_ref.id(), 5566);
    assert_eq!(frame_ref.ntp_time_ms(), 7788);
    assert_eq!(frame_ref.rotation(), VideoRotation::R270);
    assert_eq!(
        frame_ref.presentation_timestamp(),
        Some(presentation_timestamp)
    );
    assert_eq!(frame_ref.reference_time(), Some(reference_time));
    assert!(frame_ref.has_update_rect());
    assert!(frame_ref.is_repeat_frame());
}

#[test]
fn video_frame_builder_none_update_rect() {
    let i420 = I420Buffer::new(2, 2);
    let frame_buffer = i420.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(10)
        .set_update_rect(None)
        .build();

    assert!(!frame.has_update_rect());
    let update_rect = frame.update_rect();
    assert_eq!(update_rect.offset_x(), 0);
    assert_eq!(update_rect.offset_y(), 0);
    assert_eq!(update_rect.width(), frame.width());
    assert_eq!(update_rect.height(), frame.height());
}

#[test]
#[should_panic(expected = "Duration microseconds overflowed i64")]
fn video_frame_builder_overflow_duration_panics() {
    let i420 = I420Buffer::new(2, 2);
    let frame_buffer = i420.cast_to_video_frame_buffer();
    let overflow = Duration::from_micros(i64::MAX as u64 + 1);
    let _ = VideoFrame::builder(&frame_buffer).set_presentation_timestamp(Some(overflow));
}

#[test]
fn i420_buffer_chroma_dimensions_for_odd_size() {
    let width = 5;
    let height = 3;
    let buf = I420Buffer::new(width, height);

    assert_eq!(buf.chroma_width(), 3);
    assert_eq!(buf.chroma_height(), 2);
    assert_eq!(
        buf.u_data().len(),
        (buf.stride_u() as usize) * (buf.chroma_height() as usize)
    );
    assert_eq!(
        buf.v_data().len(),
        (buf.stride_v() as usize) * (buf.chroma_height() as usize)
    );
}

#[test]
fn i420_buffer_new_with_strides_preserves_stride_and_plane_lengths() {
    let width = 5;
    let height = 3;
    let stride_y = 8;
    let stride_u = 4;
    let stride_v = 6;
    let buf = I420Buffer::new_with_strides(width, height, stride_y, stride_u, stride_v);

    assert_eq!(buf.width(), width);
    assert_eq!(buf.height(), height);
    assert_eq!(buf.stride_y(), stride_y);
    assert_eq!(buf.stride_u(), stride_u);
    assert_eq!(buf.stride_v(), stride_v);
    assert_eq!(buf.y_data().len(), (stride_y * height) as usize);
    assert_eq!(
        buf.u_data().len(),
        (stride_u * buf.chroma_height()) as usize
    );
    assert_eq!(
        buf.v_data().len(),
        (stride_v * buf.chroma_height()) as usize
    );
}

#[test]
fn i420_buffer_data_and_data_mut_use_contiguous_memory_with_padding() {
    let width = 5;
    let height = 3;
    let stride_y = 8;
    let stride_u = 4;
    let stride_v = 6;
    let chroma_height = (height as usize).div_ceil(2);
    let len_y = (stride_y as usize) * (height as usize);
    let len_u = (stride_u as usize) * chroma_height;
    let len_v = (stride_v as usize) * chroma_height;
    let total_len = len_y + len_u + len_v;
    let mut buf = I420Buffer::new_with_strides(width, height, stride_y, stride_u, stride_v);

    let base = buf.data().as_ptr() as usize;
    assert_eq!(buf.data().len(), total_len);
    assert_eq!(buf.y_data().as_ptr() as usize, base);
    assert_eq!(buf.u_data().as_ptr() as usize - base, len_y);
    assert_eq!(buf.v_data().as_ptr() as usize - base, len_y + len_u);

    {
        let data = buf.data_mut();
        data[0] = 0x11;
        data[len_y] = 0x22;
        data[len_y + len_u] = 0x33;
        data[total_len - 1] = 0x44;
    }

    assert_eq!(buf.y_data()[0], 0x11);
    assert_eq!(buf.u_data()[0], 0x22);
    assert_eq!(buf.v_data()[0], 0x33);
    assert_eq!(buf.v_data()[len_v - 1], 0x44);
}

#[test]
fn nv12_buffer_planes_kind_and_to_i420() {
    let width = 4;
    let height = 3;
    let mut buf = NV12Buffer::new(width, height);

    assert_eq!(buf.width(), width);
    assert_eq!(buf.height(), height);
    assert_eq!(buf.y_data().len(), (buf.stride_y() * height) as usize);
    assert_eq!(
        buf.uv_data().len(),
        (buf.stride_uv() as usize) * (height as usize).div_ceil(2)
    );

    for (i, v) in buf.y_data_mut().iter_mut().enumerate() {
        *v = (i as u8).wrapping_add(0x10);
    }
    for uv in buf.uv_data_mut().chunks_exact_mut(2) {
        uv[0] = 0x44;
        uv[1] = 0x88;
    }

    let mut frame_buffer = buf.cast_to_video_frame_buffer();
    assert_eq!(frame_buffer.kind(), VideoFrameBufferKind::Nv12);

    let i420 = frame_buffer
        .to_i420()
        .expect("VideoFrameBuffer から I420Buffer への変換に失敗しました");
    assert_eq!(i420.y_data(), buf.y_data());
    assert!(i420.u_data().iter().all(|&v| v == 0x44));
    assert!(i420.v_data().iter().all(|&v| v == 0x88));
}

#[test]
fn nv12_buffer_chroma_dimensions_for_odd_size() {
    let width = 5;
    let height = 3;
    let buf = NV12Buffer::new(width, height);

    assert_eq!(buf.chroma_width(), 3);
    assert_eq!(buf.chroma_height(), 2);
    assert_eq!(
        buf.uv_data().len(),
        (buf.stride_uv() as usize) * (buf.chroma_height() as usize)
    );
}

#[test]
fn nv12_buffer_new_with_strides_preserves_stride_and_plane_lengths() {
    let width = 5;
    let height = 3;
    let stride_y = 8;
    let stride_uv = 8;
    let buf = NV12Buffer::new_with_strides(width, height, stride_y, stride_uv);

    assert_eq!(buf.width(), width);
    assert_eq!(buf.height(), height);
    assert_eq!(buf.stride_y(), stride_y);
    assert_eq!(buf.stride_uv(), stride_uv);
    assert_eq!(buf.y_data().len(), (stride_y * height) as usize);
    assert_eq!(
        buf.uv_data().len(),
        (stride_uv * buf.chroma_height()) as usize
    );
}

#[test]
fn nv12_buffer_data_and_data_mut_use_contiguous_memory_with_padding() {
    let width = 5;
    let height = 3;
    let stride_y = 8;
    let stride_uv = 8;
    let chroma_height = (height as usize).div_ceil(2);
    let len_y = (stride_y as usize) * (height as usize);
    let len_uv = (stride_uv as usize) * chroma_height;
    let total_len = len_y + len_uv;
    let mut buf = NV12Buffer::new_with_strides(width, height, stride_y, stride_uv);

    let base = buf.data().as_ptr() as usize;
    assert_eq!(buf.data().len(), total_len);
    assert_eq!(buf.y_data().as_ptr() as usize, base);
    assert_eq!(buf.uv_data().as_ptr() as usize - base, len_y);

    {
        let data = buf.data_mut();
        data[0] = 0x11;
        data[len_y] = 0x22;
        data[total_len - 1] = 0x33;
    }

    assert_eq!(buf.y_data()[0], 0x11);
    assert_eq!(buf.uv_data()[0], 0x22);
    assert_eq!(buf.uv_data()[len_uv - 1], 0x33);
}

#[test]
fn nv12_buffer_crop_and_scale_from() {
    let mut src = NV12Buffer::new(4, 4);
    src.y_data_mut().fill(0x11);
    for uv in src.uv_data_mut().chunks_exact_mut(2) {
        uv[0] = 0x22;
        uv[1] = 0x66;
    }

    let mut dst = NV12Buffer::new(2, 2);
    dst.crop_and_scale_from(&src, 0, 0, 4, 4);

    assert!(dst.y_data().iter().all(|&v| v == 0x11));
    for uv in dst.uv_data().chunks_exact(2) {
        assert_eq!(uv[0], 0x22);
        assert_eq!(uv[1], 0x66);
    }

    let mut frame_buffer = dst.cast_to_video_frame_buffer();
    assert_eq!(frame_buffer.kind(), VideoFrameBufferKind::Nv12);
    let i420 = frame_buffer
        .to_i420()
        .expect("VideoFrameBuffer から I420Buffer への変換に失敗しました");
    assert!(i420.y_data().iter().all(|&v| v == 0x11));
    assert!(i420.u_data().iter().all(|&v| v == 0x22));
    assert!(i420.v_data().iter().all(|&v| v == 0x66));
}

#[test]
fn video_frame_buffer_handler_native_roundtrip() {
    struct NativeBufferHandler;

    impl VideoFrameBufferHandler for NativeBufferHandler {
        fn width(&self) -> i32 {
            2
        }

        fn height(&self) -> i32 {
            2
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            let mut buffer = I420Buffer::new(2, 2);
            buffer.y_data_mut().fill(0x12);
            buffer.u_data_mut().fill(0x34);
            buffer.v_data_mut().fill(0x56);
            Some(buffer)
        }
    }

    let mut buffer = VideoFrameBuffer::new_with_handler(Box::new(NativeBufferHandler));
    assert_eq!(buffer.kind(), VideoFrameBufferKind::Native);
    assert_eq!(buffer.width(), 2);
    assert_eq!(buffer.height(), 2);

    let converted = buffer
        .to_i420()
        .expect("VideoFrameBufferHandler の ToI420 が None になりました");
    assert_eq!(converted.y_data()[0], 0x12);

    let frame = VideoFrame::builder(&buffer)
        .set_timestamp_us(12345)
        .set_timestamp_rtp(67890)
        .build();
    assert_eq!(frame.width(), 2);
    assert_eq!(frame.height(), 2);
    assert_eq!(frame.timestamp_us(), 12345);
    assert_eq!(frame.rtp_timestamp(), 67890);

    let mut frame_buffer = frame.buffer();
    assert_eq!(frame_buffer.kind(), VideoFrameBufferKind::Native);
    let frame_i420 = frame_buffer
        .to_i420()
        .expect("VideoFrame の VideoFrameBuffer から I420 変換に失敗しました");
    assert_eq!(frame_i420.y_data()[0], 0x12);
}

#[test]
fn video_frame_buffer_handler_custom_type_roundtrip() {
    struct I420TypeBufferHandler;

    impl VideoFrameBufferHandler for I420TypeBufferHandler {
        fn kind(&self) -> VideoFrameBufferKind {
            VideoFrameBufferKind::I420
        }

        fn width(&self) -> i32 {
            2
        }

        fn height(&self) -> i32 {
            2
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            let mut buffer = I420Buffer::new(2, 2);
            buffer.y_data_mut().fill(0x77);
            buffer.u_data_mut().fill(0x88);
            buffer.v_data_mut().fill(0x99);
            Some(buffer)
        }
    }

    let buffer = VideoFrameBuffer::new_with_handler(Box::new(I420TypeBufferHandler));
    assert_eq!(buffer.kind(), VideoFrameBufferKind::I420);

    let frame = VideoFrame::builder(&buffer)
        .set_timestamp_us(222)
        .set_timestamp_rtp(333)
        .build();
    let mut frame_buffer = frame.buffer();
    assert_eq!(frame_buffer.kind(), VideoFrameBufferKind::I420);

    let converted = frame_buffer
        .to_i420()
        .expect("VideoFrameBuffer の I420 変換に失敗しました");
    assert_eq!(converted.y_data()[0], 0x77);
}

#[test]
fn video_frame_buffer_handler_to_i420_none() {
    struct NoI420BufferHandler;

    impl VideoFrameBufferHandler for NoI420BufferHandler {
        fn width(&self) -> i32 {
            2
        }

        fn height(&self) -> i32 {
            2
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            None
        }
    }

    let mut buffer = VideoFrameBuffer::new_with_handler(Box::new(NoI420BufferHandler));
    assert!(buffer.to_i420().is_none());

    let frame = VideoFrame::builder(&buffer)
        .set_timestamp_us(100)
        .set_timestamp_rtp(0)
        .build();
    let mut frame_buffer = frame.buffer();
    assert!(frame_buffer.to_i420().is_none());
}

#[test]
fn video_frame_buffer_crop_and_scale_from_i420_buffer() {
    let mut src = I420Buffer::new(4, 4);
    src.y_data_mut().fill(0x10);
    src.u_data_mut().fill(0x20);
    src.v_data_mut().fill(0x30);

    let mut frame_buffer = src.cast_to_video_frame_buffer();
    let scaled = frame_buffer
        .scale(2, 2)
        .expect("VideoFrameBuffer::scale の変換に失敗しました");
    assert_eq!(scaled.width(), 2);
    assert_eq!(scaled.height(), 2);

    let mut frame_buffer = src.cast_to_video_frame_buffer();
    let cropped_scaled = frame_buffer
        .crop_and_scale(1, 1, 2, 2, 3, 3)
        .expect("VideoFrameBuffer::crop_and_scale の変換に失敗しました");
    assert_eq!(cropped_scaled.width(), 3);
    assert_eq!(cropped_scaled.height(), 3);
}

#[test]
fn video_frame_buffer_handler_crop_and_scale_callback() {
    struct CropAndScaleBufferHandler {
        called: Arc<AtomicBool>,
        args: Arc<Mutex<Option<(i32, i32, i32, i32, i32, i32)>>>,
    }

    impl VideoFrameBufferHandler for CropAndScaleBufferHandler {
        fn width(&self) -> i32 {
            8
        }

        fn height(&self) -> i32 {
            8
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            Some(I420Buffer::new(8, 8))
        }

        fn crop_and_scale(
            &mut self,
            offset_x: i32,
            offset_y: i32,
            crop_width: i32,
            crop_height: i32,
            scaled_width: i32,
            scaled_height: i32,
        ) -> Option<VideoFrameBuffer> {
            self.called.store(true, Ordering::SeqCst);
            *self.args.lock().expect("args のロックに失敗しました") = Some((
                offset_x,
                offset_y,
                crop_width,
                crop_height,
                scaled_width,
                scaled_height,
            ));
            Some(I420Buffer::new(scaled_width, scaled_height).cast_to_video_frame_buffer())
        }
    }

    let called = Arc::new(AtomicBool::new(false));
    let args = Arc::new(Mutex::new(None));
    let handler = CropAndScaleBufferHandler {
        called: Arc::clone(&called),
        args: Arc::clone(&args),
    };
    let mut buffer = VideoFrameBuffer::new_with_handler(Box::new(handler));

    let scaled = buffer
        .crop_and_scale(1, 2, 3, 4, 5, 6)
        .expect("VideoFrameBufferHandler::crop_and_scale の実行に失敗しました");

    assert!(called.load(Ordering::SeqCst));
    assert_eq!(
        *args.lock().expect("args のロックに失敗しました"),
        Some((1, 2, 3, 4, 5, 6))
    );
    assert_eq!(scaled.width(), 5);
    assert_eq!(scaled.height(), 6);
}

#[test]
fn video_frame_buffer_handler_crop_and_scale_fallback() {
    struct NoCropAndScaleBufferHandler;

    impl VideoFrameBufferHandler for NoCropAndScaleBufferHandler {
        fn width(&self) -> i32 {
            4
        }

        fn height(&self) -> i32 {
            4
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            let mut buffer = I420Buffer::new(4, 4);
            buffer.y_data_mut().fill(0x55);
            buffer.u_data_mut().fill(0x66);
            buffer.v_data_mut().fill(0x77);
            Some(buffer)
        }
    }

    let mut buffer = VideoFrameBuffer::new_with_handler(Box::new(NoCropAndScaleBufferHandler));
    let scaled = buffer
        .scale(2, 2)
        .expect("VideoFrameBuffer::scale のフォールバックに失敗しました");
    assert_eq!(scaled.width(), 2);
    assert_eq!(scaled.height(), 2);
}

#[test]
fn video_frame_buffer_as_native_roundtrip() {
    struct DowncastBufferHandler {
        value: u8,
    }

    impl VideoFrameBufferHandler for DowncastBufferHandler {
        fn width(&self) -> i32 {
            2
        }

        fn height(&self) -> i32 {
            2
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            let mut buffer = I420Buffer::new(2, 2);
            buffer.y_data_mut().fill(self.value);
            buffer.u_data_mut().fill(0x01);
            buffer.v_data_mut().fill(0x02);
            Some(buffer)
        }
    }

    let mut buffer =
        VideoFrameBuffer::new_with_handler(Box::new(DowncastBufferHandler { value: 7 }));
    // Safety: このテストでは同一実体への同時アクセスを行いません。
    let handler = unsafe { buffer.as_native_ref::<DowncastBufferHandler>() }
        .expect("as_native_ref が失敗しました");
    assert_eq!(handler.value, 7);

    // Safety: このテストでは同一実体への同時アクセスを行いません。
    let handler = unsafe { buffer.as_native_mut::<DowncastBufferHandler>() }
        .expect("as_native_mut が失敗しました");
    handler.value = 9;

    // Safety: このテストでは同一実体への同時アクセスを行いません。
    let handler = unsafe { buffer.as_native_ref::<DowncastBufferHandler>() }
        .expect("as_native_ref が失敗しました");
    assert_eq!(handler.value, 9);

    let i420 = buffer
        .to_i420()
        .expect("VideoFrameBuffer の I420 変換に失敗しました");
    assert_eq!(i420.y_data()[0], 9);
}

#[test]
fn video_frame_buffer_as_native_clone_and_frame_buffer() {
    struct DowncastBufferHandler {
        value: u8,
    }

    impl VideoFrameBufferHandler for DowncastBufferHandler {
        fn width(&self) -> i32 {
            2
        }

        fn height(&self) -> i32 {
            2
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            let mut buffer = I420Buffer::new(2, 2);
            buffer.y_data_mut().fill(self.value);
            buffer.u_data_mut().fill(0x11);
            buffer.v_data_mut().fill(0x22);
            Some(buffer)
        }
    }

    let mut buffer =
        VideoFrameBuffer::new_with_handler(Box::new(DowncastBufferHandler { value: 3 }));
    // Safety: このテストでは同一実体への同時アクセスを行いません。
    unsafe { buffer.as_native_mut::<DowncastBufferHandler>() }
        .expect("as_native_mut が失敗しました")
        .value = 5;

    let cloned = buffer.clone();
    // Safety: このテストでは同一実体への同時アクセスを行いません。
    let cloned_handler = unsafe { cloned.as_native_ref::<DowncastBufferHandler>() }
        .expect("clone からの as_native_ref が失敗しました");
    assert_eq!(cloned_handler.value, 5);

    let frame = VideoFrame::builder(&buffer)
        .set_timestamp_us(10)
        .set_timestamp_rtp(20)
        .build();
    let frame_buffer = frame.buffer();
    // Safety: このテストでは同一実体への同時アクセスを行いません。
    let frame_handler = unsafe { frame_buffer.as_native_ref::<DowncastBufferHandler>() }
        .expect("VideoFrame::buffer からの as_native_ref が失敗しました");
    assert_eq!(frame_handler.value, 5);
}

#[test]
fn video_frame_buffer_as_native_returns_none_for_builtin_buffers() {
    struct NativeBufferHandler;

    impl VideoFrameBufferHandler for NativeBufferHandler {
        fn width(&self) -> i32 {
            1
        }

        fn height(&self) -> i32 {
            1
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            Some(I420Buffer::new(1, 1))
        }
    }

    let i420 = I420Buffer::new(2, 2);
    let mut i420_frame_buffer = i420.cast_to_video_frame_buffer();
    // Safety: 参照を取り出すだけで、同時アクセスは行いません。
    assert!(unsafe {
        i420_frame_buffer
            .as_native_ref::<NativeBufferHandler>()
            .is_none()
    });
    // Safety: 参照を取り出すだけで、同時アクセスは行いません。
    assert!(unsafe {
        i420_frame_buffer
            .as_native_mut::<NativeBufferHandler>()
            .is_none()
    });

    let nv12 = NV12Buffer::new(2, 2);
    let mut nv12_frame_buffer = nv12.cast_to_video_frame_buffer();
    // Safety: 参照を取り出すだけで、同時アクセスは行いません。
    assert!(unsafe {
        nv12_frame_buffer
            .as_native_ref::<NativeBufferHandler>()
            .is_none()
    });
    // Safety: 参照を取り出すだけで、同時アクセスは行いません。
    assert!(unsafe {
        nv12_frame_buffer
            .as_native_mut::<NativeBufferHandler>()
            .is_none()
    });
}

#[test]
fn video_frame_buffer_as_i420_and_as_nv12() {
    let i420 = I420Buffer::new(2, 2);
    let i420_frame_buffer = i420.cast_to_video_frame_buffer();
    let i420_view = i420_frame_buffer
        .as_i420()
        .expect("as_i420 failed on I420 buffer");
    assert_eq!(i420_view.width(), 2);
    assert_eq!(i420_view.height(), 2);
    assert!(i420_frame_buffer.as_nv12().is_none());

    let nv12 = NV12Buffer::new(2, 2);
    let nv12_frame_buffer = nv12.cast_to_video_frame_buffer();
    let nv12_view = nv12_frame_buffer
        .as_nv12()
        .expect("as_nv12 failed on NV12 buffer");
    assert_eq!(nv12_view.width(), 2);
    assert_eq!(nv12_view.height(), 2);
    assert!(nv12_frame_buffer.as_i420().is_none());
}

#[test]
fn video_frame_buffer_as_i420_and_as_nv12_return_none_for_native() {
    struct NativeBufferHandler;

    impl VideoFrameBufferHandler for NativeBufferHandler {
        fn width(&self) -> i32 {
            2
        }

        fn height(&self) -> i32 {
            2
        }

        fn to_i420(&mut self) -> Option<I420Buffer> {
            Some(I420Buffer::new(2, 2))
        }
    }

    let frame_buffer = VideoFrameBuffer::new_with_handler(Box::new(NativeBufferHandler));
    assert!(frame_buffer.as_i420().is_none());
    assert!(frame_buffer.as_nv12().is_none());
}

#[test]
fn abgr_to_i420_conversion() {
    // 2x2 ピクセル、ABGR = 0xff804020 (B=0x20, G=0x40, R=0x80, A=0xff)
    let pixel = [0x20u8, 0x40, 0x80, 0xff];
    let mut src = Vec::new();
    for _ in 0..4 {
        src.extend_from_slice(&pixel);
    }
    let mut y_plane = vec![0u8; 2 * 2];
    let mut u_plane = vec![0u8; 1];
    let mut v_plane = vec![0u8; 1];
    assert!(abgr_to_i420(
        &src,
        2 * 4,
        &mut y_plane,
        2,
        &mut u_plane,
        1,
        &mut v_plane,
        1,
        2,
        2,
    ));
    // 単色なので Y/U/V は全て同一値になるはず。
    assert!(y_plane.iter().all(|&v| v == y_plane[0]));
    assert!(u_plane.iter().all(|&v| v == u_plane[0]));
    assert!(v_plane.iter().all(|&v| v == v_plane[0]));
}

#[test]
fn convert_from_i420_argb_conversion() {
    let y_plane = vec![0x30; 4];
    let u_plane = vec![0x80; 1];
    let v_plane = vec![0x80; 1];
    let mut dst = vec![0u8; 2 * 2 * 4];
    assert!(convert_from_i420(
        &y_plane,
        2,
        &u_plane,
        1,
        &v_plane,
        1,
        &mut dst,
        2 * 4,
        2,
        2,
        LibyuvFourcc::Argb,
    ));
    assert_eq!(dst.len(), 2 * 2 * 4);
}

#[test]
fn i420_to_nv12_round_trip() {
    let width = 4;
    let height = 4;
    let mut src_y = vec![0u8; (width * height) as usize];
    let mut src_u = vec![0u8; ((width / 2) * (height / 2)) as usize];
    let mut src_v = vec![0u8; ((width / 2) * (height / 2)) as usize];
    for (i, p) in src_y.iter_mut().enumerate() {
        *p = (i as u8).wrapping_mul(3);
    }
    for (i, p) in src_u.iter_mut().enumerate() {
        *p = 0x40u8.wrapping_add(i as u8);
    }
    for (i, p) in src_v.iter_mut().enumerate() {
        *p = 0x80u8.wrapping_add(i as u8);
    }
    let mut nv12_y = vec![0u8; (width * height) as usize];
    let mut nv12_uv = vec![0u8; (width * (height / 2)) as usize];
    assert!(i420_to_nv12(
        &src_y,
        width,
        &src_u,
        width / 2,
        &src_v,
        width / 2,
        &mut nv12_y,
        width,
        &mut nv12_uv,
        width,
        width,
        height,
    ));
    let mut restored_y = vec![0u8; src_y.len()];
    let mut restored_u = vec![0u8; src_u.len()];
    let mut restored_v = vec![0u8; src_v.len()];
    assert!(nv12_to_i420(
        &nv12_y,
        width,
        &nv12_uv,
        width,
        &mut restored_y,
        width,
        &mut restored_u,
        width / 2,
        &mut restored_v,
        width / 2,
        width,
        height,
    ));

    assert_eq!(src_y, restored_y);
    assert_eq!(src_u, restored_u);
    assert_eq!(src_v, restored_v);
}

#[test]
fn i420_buffer_planes_mut_to_nv12_round_trip() {
    let width = 5;
    let height = 3;
    let mut src = I420Buffer::new(width, height);
    let src_stride_y = src.stride_y();
    let src_stride_u = src.stride_u();
    let src_stride_v = src.stride_v();
    let chroma_width = src.chroma_width();
    let chroma_height = src.chroma_height();

    {
        let (src_y, src_u, src_v) = src.planes_mut();
        for row in 0..height as usize {
            let begin = row * src_stride_y as usize;
            let end = begin + width as usize;
            for (col, v) in src_y[begin..end].iter_mut().enumerate() {
                *v = (row as u8).wrapping_mul(17).wrapping_add(col as u8);
            }
        }
        for row in 0..chroma_height as usize {
            let begin = row * src_stride_u as usize;
            let end = begin + chroma_width as usize;
            for (col, v) in src_u[begin..end].iter_mut().enumerate() {
                *v = 0x40u8
                    .wrapping_add((row as u8).wrapping_mul(7))
                    .wrapping_add(col as u8);
            }
        }
        for row in 0..chroma_height as usize {
            let begin = row * src_stride_v as usize;
            let end = begin + chroma_width as usize;
            for (col, v) in src_v[begin..end].iter_mut().enumerate() {
                *v = 0x80u8
                    .wrapping_add((row as u8).wrapping_mul(11))
                    .wrapping_add(col as u8);
            }
        }
    }

    let mut nv12 = NV12Buffer::new(width, height);
    let dst_stride_y = nv12.stride_y();
    let dst_stride_uv = nv12.stride_uv();
    {
        let (dst_y, dst_uv) = nv12.planes_mut();
        assert!(i420_to_nv12(
            src.y_data(),
            src_stride_y,
            src.u_data(),
            src_stride_u,
            src.v_data(),
            src_stride_v,
            dst_y,
            dst_stride_y,
            dst_uv,
            dst_stride_uv,
            width,
            height,
        ));
    }

    let mut restored = I420Buffer::new(width, height);
    let restored_stride_y = restored.stride_y();
    let restored_stride_u = restored.stride_u();
    let restored_stride_v = restored.stride_v();
    {
        let (restored_y, restored_u, restored_v) = restored.planes_mut();
        assert!(nv12_to_i420(
            nv12.y_data(),
            nv12.stride_y(),
            nv12.uv_data(),
            nv12.stride_uv(),
            restored_y,
            restored_stride_y,
            restored_u,
            restored_stride_u,
            restored_v,
            restored_stride_v,
            width,
            height,
        ));
    }

    let assert_plane_eq =
        |lhs: &[u8], lhs_stride: i32, rhs: &[u8], rhs_stride: i32, row_bytes: i32, rows: i32| {
            let lhs_stride = lhs_stride as usize;
            let rhs_stride = rhs_stride as usize;
            let row_bytes = row_bytes as usize;
            let rows = rows as usize;
            for row in 0..rows {
                let lhs_begin = row * lhs_stride;
                let lhs_end = lhs_begin + row_bytes;
                let rhs_begin = row * rhs_stride;
                let rhs_end = rhs_begin + row_bytes;
                assert_eq!(lhs[lhs_begin..lhs_end], rhs[rhs_begin..rhs_end]);
            }
        };

    assert_plane_eq(
        src.y_data(),
        src_stride_y,
        restored.y_data(),
        restored_stride_y,
        width,
        height,
    );
    assert_plane_eq(
        src.u_data(),
        src_stride_u,
        restored.u_data(),
        restored_stride_u,
        chroma_width,
        chroma_height,
    );
    assert_plane_eq(
        src.v_data(),
        src_stride_v,
        restored.v_data(),
        restored_stride_v,
        chroma_width,
        chroma_height,
    );
}

#[test]
fn i420_copy_with_odd_size_and_padding() {
    let width = 5;
    let height = 3;
    let chroma_width = (width + 1) / 2;
    let chroma_height = (height + 1) / 2;

    let src_stride_y = 8;
    let src_stride_u = 4;
    let src_stride_v = 6;
    let mut src_y = vec![0u8; (src_stride_y * height) as usize];
    let mut src_u = vec![0u8; (src_stride_u * chroma_height) as usize];
    let mut src_v = vec![0u8; (src_stride_v * chroma_height) as usize];

    for row in 0..height as usize {
        let row_begin = row * src_stride_y as usize;
        let row_end = row_begin + width as usize;
        for (col, px) in src_y[row_begin..row_end].iter_mut().enumerate() {
            *px = (row as u8).wrapping_mul(13).wrapping_add(col as u8);
        }
    }
    for row in 0..chroma_height as usize {
        let row_begin = row * src_stride_u as usize;
        let row_end = row_begin + chroma_width as usize;
        for (col, px) in src_u[row_begin..row_end].iter_mut().enumerate() {
            *px = 0x40u8
                .wrapping_add((row as u8).wrapping_mul(7))
                .wrapping_add(col as u8);
        }
    }
    for row in 0..chroma_height as usize {
        let row_begin = row * src_stride_v as usize;
        let row_end = row_begin + chroma_width as usize;
        for (col, px) in src_v[row_begin..row_end].iter_mut().enumerate() {
            *px = 0x80u8
                .wrapping_add((row as u8).wrapping_mul(11))
                .wrapping_add(col as u8);
        }
    }

    let dst_stride_y = 9;
    let dst_stride_u = 5;
    let dst_stride_v = 7;
    let mut dst_y = vec![0u8; (dst_stride_y * height) as usize];
    let mut dst_u = vec![0u8; (dst_stride_u * chroma_height) as usize];
    let mut dst_v = vec![0u8; (dst_stride_v * chroma_height) as usize];
    assert!(i420_copy(
        &src_y,
        src_stride_y,
        &src_u,
        src_stride_u,
        &src_v,
        src_stride_v,
        &mut dst_y,
        dst_stride_y,
        &mut dst_u,
        dst_stride_u,
        &mut dst_v,
        dst_stride_v,
        width,
        height,
    ));

    let assert_plane_eq =
        |lhs: &[u8], lhs_stride: i32, rhs: &[u8], rhs_stride: i32, row_bytes: i32, rows: i32| {
            let lhs_stride = lhs_stride as usize;
            let rhs_stride = rhs_stride as usize;
            let row_bytes = row_bytes as usize;
            let rows = rows as usize;
            for row in 0..rows {
                let lhs_begin = row * lhs_stride;
                let lhs_end = lhs_begin + row_bytes;
                let rhs_begin = row * rhs_stride;
                let rhs_end = rhs_begin + row_bytes;
                assert_eq!(lhs[lhs_begin..lhs_end], rhs[rhs_begin..rhs_end]);
            }
        };

    assert_plane_eq(&src_y, src_stride_y, &dst_y, dst_stride_y, width, height);
    assert_plane_eq(
        &src_u,
        src_stride_u,
        &dst_u,
        dst_stride_u,
        chroma_width,
        chroma_height,
    );
    assert_plane_eq(
        &src_v,
        src_stride_v,
        &dst_v,
        dst_stride_v,
        chroma_width,
        chroma_height,
    );
}

#[test]
fn i420_copy_returns_false_when_source_plane_is_too_short() {
    let width = 4;
    let height = 4;
    let src_y = vec![0u8; (width * height) as usize];
    let src_u = vec![0u8; ((width / 2) * (height / 2) - 1) as usize];
    let src_v = vec![0u8; ((width / 2) * (height / 2)) as usize];
    let mut dst_y = vec![0u8; (width * height) as usize];
    let mut dst_u = vec![0u8; ((width / 2) * (height / 2)) as usize];
    let mut dst_v = vec![0u8; ((width / 2) * (height / 2)) as usize];

    assert!(!i420_copy(
        &src_y,
        width,
        &src_u,
        width / 2,
        &src_v,
        width / 2,
        &mut dst_y,
        width,
        &mut dst_u,
        width / 2,
        &mut dst_v,
        width / 2,
        width,
        height,
    ));
}

#[test]
fn i420_copy_returns_false_when_destination_plane_is_too_short() {
    let width = 4;
    let height = 4;
    let src_y = vec![0u8; (width * height) as usize];
    let src_u = vec![0u8; ((width / 2) * (height / 2)) as usize];
    let src_v = vec![0u8; ((width / 2) * (height / 2)) as usize];
    let mut dst_y = vec![0u8; (width * height) as usize];
    let mut dst_u = vec![0u8; ((width / 2) * (height / 2)) as usize];
    let mut dst_v = vec![0u8; ((width / 2) * (height / 2) - 1) as usize];

    assert!(!i420_copy(
        &src_y,
        width,
        &src_u,
        width / 2,
        &src_v,
        width / 2,
        &mut dst_y,
        width,
        &mut dst_u,
        width / 2,
        &mut dst_v,
        width / 2,
        width,
        height,
    ));
}

#[test]
fn nv12_copy_with_odd_size_and_padding() {
    let width = 5;
    let height = 3;
    let chroma_width = (width + 1) / 2;
    let chroma_height = (height + 1) / 2;
    let uv_row_bytes = chroma_width * 2;

    let src_stride_y = 8;
    let src_stride_uv = 10;
    let mut src_y = vec![0u8; (src_stride_y * height) as usize];
    let mut src_uv = vec![0u8; (src_stride_uv * chroma_height) as usize];
    for row in 0..height as usize {
        let row_begin = row * src_stride_y as usize;
        let row_end = row_begin + width as usize;
        for (col, px) in src_y[row_begin..row_end].iter_mut().enumerate() {
            *px = 0x20u8
                .wrapping_add((row as u8).wrapping_mul(9))
                .wrapping_add(col as u8);
        }
    }
    for row in 0..chroma_height as usize {
        let row_begin = row * src_stride_uv as usize;
        let row_end = row_begin + uv_row_bytes as usize;
        for (col, px) in src_uv[row_begin..row_end].iter_mut().enumerate() {
            *px = 0x60u8
                .wrapping_add((row as u8).wrapping_mul(5))
                .wrapping_add(col as u8);
        }
    }

    let dst_stride_y = 9;
    let dst_stride_uv = 11;
    let mut dst_y = vec![0u8; (dst_stride_y * height) as usize];
    let mut dst_uv = vec![0u8; (dst_stride_uv * chroma_height) as usize];
    assert!(nv12_copy(
        &src_y,
        src_stride_y,
        &src_uv,
        src_stride_uv,
        &mut dst_y,
        dst_stride_y,
        &mut dst_uv,
        dst_stride_uv,
        width,
        height,
    ));

    let assert_plane_eq =
        |lhs: &[u8], lhs_stride: i32, rhs: &[u8], rhs_stride: i32, row_bytes: i32, rows: i32| {
            let lhs_stride = lhs_stride as usize;
            let rhs_stride = rhs_stride as usize;
            let row_bytes = row_bytes as usize;
            let rows = rows as usize;
            for row in 0..rows {
                let lhs_begin = row * lhs_stride;
                let lhs_end = lhs_begin + row_bytes;
                let rhs_begin = row * rhs_stride;
                let rhs_end = rhs_begin + row_bytes;
                assert_eq!(lhs[lhs_begin..lhs_end], rhs[rhs_begin..rhs_end]);
            }
        };

    assert_plane_eq(&src_y, src_stride_y, &dst_y, dst_stride_y, width, height);
    assert_plane_eq(
        &src_uv,
        src_stride_uv,
        &dst_uv,
        dst_stride_uv,
        uv_row_bytes,
        chroma_height,
    );
}

#[test]
fn nv12_copy_returns_false_when_source_plane_is_too_short() {
    let width = 4;
    let height = 4;
    let src_y = vec![0u8; (width * height) as usize];
    let src_uv = vec![0u8; (width * (height / 2) - 1) as usize];
    let mut dst_y = vec![0u8; (width * height) as usize];
    let mut dst_uv = vec![0u8; (width * (height / 2)) as usize];

    assert!(!nv12_copy(
        &src_y,
        width,
        &src_uv,
        width,
        &mut dst_y,
        width,
        &mut dst_uv,
        width,
        width,
        height,
    ));
}

#[test]
fn nv12_copy_returns_false_when_destination_plane_is_too_short() {
    let width = 4;
    let height = 4;
    let src_y = vec![0u8; (width * height) as usize];
    let src_uv = vec![0u8; (width * (height / 2)) as usize];
    let mut dst_y = vec![0u8; (width * height) as usize];
    let mut dst_uv = vec![0u8; (width * (height / 2) - 1) as usize];

    assert!(!nv12_copy(
        &src_y,
        width,
        &src_uv,
        width,
        &mut dst_y,
        width,
        &mut dst_uv,
        width,
        width,
        height,
    ));
}

#[test]
fn logging_functions_are_callable() {
    // severity は 0 にしておく。実際のログ内容は検証しない。
    log::log_to_debug(log::Severity::Info);
    log::enable_timestamps();
    log::enable_threads();
    log::print(log::Severity::Info, "webrtc-c", 0, "log test");
}

#[test]
fn thread_blocking_call_runs() {
    let mut thread = Thread::new();
    thread.start();
    let result = thread.blocking_call(|| 42);
    assert_eq!(result, 42);

    // () 戻り値のパスも通す
    thread.blocking_call(|| {});
    thread.stop();
}

#[test]
fn thread_sleep_ms_runs() {
    thread_sleep_ms(1);
}

#[test]
fn builtin_audio_factories_create() {
    let dec = AudioDecoderFactory::builtin();
    assert!(!dec.as_ptr().is_null());
    let enc = AudioEncoderFactory::builtin();
    assert!(!enc.as_ptr().is_null());

    // AudioProcessingBuilder も生成確認する。
    let apb = AudioProcessingBuilder::new_builtin();
    assert!(!apb.as_ptr().is_null());

    // PeerConnectionFactoryDependencies を組み立てて EnableMedia まで呼ぶ。
    let mut deps = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps.set_network_thread(&network);
    deps.set_worker_thread(&worker);
    deps.set_signaling_thread(&signaling);
    deps.set_audio_encoder_factory(&enc);
    deps.set_audio_decoder_factory(&dec);
    deps.set_audio_processing_builder(apb);
    // Dummy ADM を設定してメディア初期化を通す。
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps.set_audio_device_module(&adm);
    deps.enable_media();
    assert!(!deps.as_ptr().is_null());
    drop(deps);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn audio_device_module_recording_device_name_roundtrip() {
    struct TestAudioDeviceModuleHandler {
        name: String,
        guid: String,
    }

    impl AudioDeviceModuleHandler for TestAudioDeviceModuleHandler {
        fn init(&self) -> i32 {
            0
        }

        fn recording_devices(&self) -> i16 {
            1
        }

        fn recording_device_name(&self, index: u16) -> Option<(String, String)> {
            if index == 0 {
                Some((self.name.clone(), self.guid.clone()))
            } else {
                None
            }
        }
    }

    fn make_ascii_string(len: usize) -> String {
        (0..len).map(|i| (b'a' + (i % 26) as u8) as char).collect()
    }

    let lengths = [0usize, 1, 2, 3, 7, 31, 63, 64];
    for &len in &lengths {
        let name = make_ascii_string(len);
        let guid = make_ascii_string(64usize.saturating_sub(len));
        let expected_name = name.clone();
        let expected_guid = guid.clone();
        let mut adm = AudioDeviceModule::new_with_handler(Box::new(TestAudioDeviceModuleHandler {
            name,
            guid,
        }));
        adm.init().expect("AudioDeviceModule::init が失敗しました");
        assert_eq!(adm.recording_devices(), 1);
        let (got_name, got_guid) = adm
            .recording_device_name(0)
            .expect("recording_device_name が失敗しました");
        assert_eq!(got_name, expected_name);
        assert_eq!(got_guid, expected_guid);
    }
}

#[test]
fn audio_parameters_unique_roundtrip() {
    let raw = unsafe { ffi::webrtc_AudioParameters_new(48_000, 2, 480) };
    assert!(!raw.is_null());
    let params = unsafe { ffi::webrtc_AudioParameters_unique_get(raw) };
    assert!(!params.is_null());
    assert_eq!(
        unsafe { ffi::webrtc_AudioParameters_get_sample_rate(params) },
        48_000
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioParameters_get_channels(params) },
        2
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioParameters_get_frames_per_buffer(params) },
        480
    );
    unsafe { ffi::webrtc_AudioParameters_unique_delete(raw) };
}

#[test]
fn audio_device_module_stats_unique_roundtrip() {
    let raw = unsafe { ffi::webrtc_AudioDeviceModule_Stats_new(1.25, 12, 3.5, 0.75, 999) };
    assert!(!raw.is_null());
    let stats = unsafe { ffi::webrtc_AudioDeviceModule_Stats_unique_get(raw) };
    assert!(!stats.is_null());
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_synthesized_samples_duration_s(stats) },
        1.25
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_synthesized_samples_events(stats) },
        12
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_samples_duration_s(stats) },
        3.5
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_playout_delay_s(stats) },
        0.75
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_samples_count(stats) },
        999
    );
    unsafe { ffi::webrtc_AudioDeviceModule_Stats_unique_delete(raw) };
}

#[test]
fn audio_device_module_get_stats_returns_unique() {
    struct TestAudioDeviceModuleGetStatsHandler;

    impl AudioDeviceModuleHandler for TestAudioDeviceModuleGetStatsHandler {
        fn get_stats(&self) -> Option<AudioDeviceModuleStats> {
            Some(AudioDeviceModuleStats::new(1.0, 2, 3.0, 4.0, 5))
        }
    }

    let adm = AudioDeviceModule::new_with_handler(Box::new(TestAudioDeviceModuleGetStatsHandler));
    let mut out_stats: *mut ffi::webrtc_AudioDeviceModule_Stats_unique = std::ptr::null_mut();
    let ret = unsafe { ffi::webrtc_AudioDeviceModule_GetStats(adm.as_ptr(), &mut out_stats) };
    assert_eq!(ret, 1);
    assert!(!out_stats.is_null());
    let stats = unsafe { ffi::webrtc_AudioDeviceModule_Stats_unique_get(out_stats) };
    assert!(!stats.is_null());
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_synthesized_samples_duration_s(stats) },
        1.0
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_synthesized_samples_events(stats) },
        2
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_samples_duration_s(stats) },
        3.0
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_playout_delay_s(stats) },
        4.0
    );
    assert_eq!(
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_samples_count(stats) },
        5
    );
    unsafe { ffi::webrtc_AudioDeviceModule_Stats_unique_delete(out_stats) };
}

#[test]
fn audio_device_module_get_stats_none_returns_zero() {
    let adm = AudioDeviceModule::new_with_handler(Box::new(NoopHandler));
    let mut out_stats: *mut ffi::webrtc_AudioDeviceModule_Stats_unique = std::ptr::null_mut();
    let ret = unsafe { ffi::webrtc_AudioDeviceModule_GetStats(adm.as_ptr(), &mut out_stats) };
    assert_eq!(ret, 0);
    assert!(out_stats.is_null());
}

#[test]
fn adapted_video_track_source() {
    let mut src = AdaptedVideoTrackSource::new();
    let adapted = src.adapt_frame(640, 480, 1_000_000);
    // applied が false の場合でもサイズ情報が得られることを確認する。
    assert!(adapted.size.adapted_width >= 0);
    assert!(adapted.size.adapted_height >= 0);

    let buf = I420Buffer::new(2, 2);
    let frame_buffer = buf.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(2_000_000)
        .set_timestamp_rtp(0)
        .build();
    src.on_frame(&frame);
}

#[test]
fn peer_connection_factory_and_capabilities() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();

    // PeerConnectionFactoryDependencies を組み立てる。スレッドのライフサイクルはここで管理する。
    let mut deps = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps.set_network_thread(&network);
    deps.set_worker_thread(&worker);
    deps.set_signaling_thread(&signaling);
    deps.set_audio_encoder_factory(&enc);
    deps.set_audio_decoder_factory(&dec);
    deps.set_audio_processing_builder(apb);
    let event_log = RtcEventLogFactory::new();
    deps.set_event_log_factory(event_log);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps.set_audio_device_module(&adm);
    deps.enable_media();

    // Factory を生成し、オプションと RTP 能力を取得する。
    let (mut factory, context) = PeerConnectionFactory::create_modular_with_context(&mut deps)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");
    let mut opts = PeerConnectionFactoryOptions::new();
    opts.set_disable_encryption(false);
    let dtls12 = unsafe { ffi::webrtc_SSL_PROTOCOL_DTLS_12 };
    opts.set_ssl_max_version(dtls12);
    factory.set_options(&opts);

    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());

    let caps = factory.get_rtp_sender_capabilities(MediaType::Audio);
    assert!(caps.codec_len() >= 0);
    let codecs = caps.codecs();
    assert_eq!(codecs.len() as i32, caps.codec_len());
    if !codecs.is_empty() {
        let first = codecs.get(0).expect("先頭 codec の取得に失敗しました");
        assert!(first.name().is_ok());
    }

    drop(caps);
    drop(context);
    drop(factory);
    drop(deps);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn rtc_configuration_and_ice_server() {
    let mut config = PeerConnectionRtcConfiguration::new();
    config.set_type(IceTransportsType::Relay);
    let mut server = IceServer::new();
    assert_eq!(server.urls_len(), 0);
    server.set_username("user");
    server.set_password("pass");
    server.set_tls_cert_policy(TlsCertPolicy::InsecureNoCheck);
    server.add_url("stun:192.0.2.1:3478");
    assert_eq!(server.urls_len(), 1);
    server.add_url("turn:192.0.2.2:3478?transport=udp");
    assert_eq!(server.urls_len(), 2);

    {
        let mut servers = config.servers();
        let len_before = servers.len();
        servers.push(&server);
        assert_eq!(servers.len(), len_before + 1);
    }

    // 所有ベクタでも同じ挙動になることを確認しておく。
    let mut owned = IceServerVector::new(0);
    let len_before = owned.len();
    owned.push(&server);
    assert_eq!(owned.len(), len_before + 1);
}

#[test]
fn tls_cert_policy_round_trip() {
    assert_eq!(
        TlsCertPolicy::from_int(TlsCertPolicy::Secure.to_int()),
        TlsCertPolicy::Secure
    );
    assert_eq!(
        TlsCertPolicy::from_int(TlsCertPolicy::InsecureNoCheck.to_int()),
        TlsCertPolicy::InsecureNoCheck
    );
    assert_eq!(
        TlsCertPolicy::from_int(123456),
        TlsCertPolicy::Unknown(123456)
    );
}

#[test]
fn create_modular_with_context_returns_default_network_objects() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();

    let mut deps = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps.set_network_thread(&network);
    deps.set_worker_thread(&worker);
    deps.set_signaling_thread(&signaling);
    deps.set_audio_encoder_factory(&enc);
    deps.set_audio_decoder_factory(&dec);
    deps.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps.set_audio_device_module(&adm);
    deps.enable_media();

    let (factory, context) = PeerConnectionFactory::create_modular_with_context(&mut deps)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");
    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());
    assert!(!factory.as_ptr().is_null());

    drop(context);
    drop(factory);
    drop(deps);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn rtp_codec_capability_vector() {
    let mut cap = RtpCodecCapability::new();
    cap.set_kind(MediaType::Audio);
    cap.set_name("opus");
    cap.set_clock_rate(Some(48_000));
    {
        let mut params = cap.parameters();
        params.set("stereo", "1");
        assert!(params.iter().any(|(k, v)| k == "stereo" && v == "1"));
    }

    let mut vec = RtpCodecCapabilityVector::new(0);
    let len_before = vec.len();
    vec.push(&cap.as_ref());
    assert_eq!(vec.len(), len_before + 1);
    vec.resize(2);
    let mut cap2 = RtpCodecCapability::new();
    cap2.set_kind(MediaType::Audio);
    cap2.set_name("PCMU");
    cap2.set_clock_rate(Some(8_000));
    assert!(vec.set(1, &cap2.as_ref()));
    assert_eq!(vec.len(), 2);
    let first = vec.get(0).expect("先頭 codec の取得に失敗しました");
    let second = vec.get(1).expect("2 番目 codec の取得に失敗しました");
    assert_eq!(
        first.name().expect("1 番目 codec 名の取得に失敗しました"),
        "opus"
    );
    assert_eq!(
        second.name().expect("2 番目 codec 名の取得に失敗しました"),
        "PCMU"
    );
}

#[test]
fn rtp_encoding_parameters_and_transceiver_init() {
    let mut codec = RtpCodec::new();
    codec.set_kind(MediaType::Audio);
    codec.set_name("opus");
    codec.set_clock_rate(Some(48_000));
    codec.set_num_channels(Some(2));

    let mut enc = RtpEncodingParameters::new();
    enc.set_rid("f");
    enc.set_ssrc(Some(1234));
    enc.set_max_bitrate_bps(Some(1_500_000));
    enc.set_min_bitrate_bps(Some(100_000));
    enc.set_max_framerate(Some(30.0));
    enc.set_scale_resolution_down_by(Some(2.0));
    let mut resolution = Resolution::new();
    resolution.set_width(960);
    resolution.set_height(540);
    enc.set_scale_resolution_down_to(Some(&resolution));
    enc.set_active(false);
    enc.set_adaptive_ptime(true);
    enc.set_scalability_mode(Some("L1T3"));
    enc.set_codec(Some(&codec));
    assert_eq!(enc.bitrate_priority(), default_bitrate_priority());
    assert_eq!(enc.network_priority(), Priority::Low);
    enc.set_bitrate_priority(4.0);
    enc.set_network_priority(Priority::VeryLow);
    enc.set_request_key_frame(true);
    enc.set_num_temporal_layers(Some(2));
    assert_eq!(enc.bitrate_priority(), 4.0);
    assert_eq!(enc.network_priority(), Priority::VeryLow);
    assert!(enc.request_key_frame());
    assert_eq!(enc.num_temporal_layers(), Some(2));
    enc.set_request_key_frame(false);
    enc.set_num_temporal_layers(None);
    assert!(!enc.request_key_frame());
    assert!(enc.num_temporal_layers().is_none());
    let mid = Priority::Medium;
    assert_eq!(Priority::from_int(mid.to_int()), mid);
    let unknown = 123456;
    assert_eq!(Priority::from_int(unknown), Priority::Unknown(unknown));
    enc.set_network_priority(Priority::Unknown(unknown));
    assert_eq!(enc.network_priority(), Priority::Unknown(unknown));
    assert_eq!(enc.rid().expect("rid の取得に失敗しました"), "f");
    assert_eq!(enc.ssrc(), Some(1234));
    assert_eq!(enc.max_bitrate_bps(), Some(1_500_000));
    assert_eq!(enc.min_bitrate_bps(), Some(100_000));
    assert_eq!(enc.max_framerate(), Some(30.0));
    assert_eq!(enc.scale_resolution_down_by(), Some(2.0));
    let got_resolution = enc
        .scale_resolution_down_to()
        .expect("scale_resolution_down_to の取得に失敗しました");
    assert_eq!(got_resolution.width(), 960);
    assert_eq!(got_resolution.height(), 540);
    assert!(!enc.active());
    assert!(enc.adaptive_ptime());
    assert_eq!(
        enc.scalability_mode()
            .expect("scalability_mode が未設定でした")
            .expect("scalability_mode の取得に失敗しました"),
        "L1T3".to_string()
    );
    let enc_codec = enc.codec().expect("codec の取得に失敗しました");
    assert_eq!(
        enc_codec.name().expect("codec 名の取得に失敗しました"),
        "opus"
    );
    assert_eq!(enc_codec.clock_rate(), Some(48_000));
    assert_eq!(enc_codec.num_channels(), Some(2));
    enc.set_scalability_mode(None);
    assert!(enc.scalability_mode().is_none());
    enc.set_codec(None);
    assert!(enc.codec().is_none());

    let mut vec = RtpEncodingParametersVector::new(0);
    vec.push(&enc);
    assert_eq!(vec.len(), 1);
    vec.resize(2);
    let mut enc2 = RtpEncodingParameters::new();
    enc2.set_rid("h");
    assert!(vec.set(1, &enc2));
    assert_eq!(vec.len(), 2);
    let cloned = vec.clone_self();
    assert_eq!(cloned.len(), vec.len());

    let mut init = RtpTransceiverInit::new();
    init.set_direction(RtpTransceiverDirection::SendOnly);
    init.set_send_encodings(&vec);
    let mut stream_ids = init.stream_ids();
    stream_ids.push(&CxxString::from_str("stream-1"));
    assert_eq!(stream_ids.len(), 1);

    let mut offer = PeerConnectionOfferAnswerOptions::new();
    offer.set_offer_to_receive_audio(1);
    offer.set_offer_to_receive_video(1);
    offer.set_voice_activity_detection(true);
    offer.set_ice_restart(false);
    offer.set_use_rtp_mux(true);
    offer.set_raw_packetization_for_video(false);
    offer.set_num_simulcast_layers(0);
    offer.set_use_obsolete_sctp_sdp(false);
    assert_eq!(offer.offer_to_receive_audio(), 1);
    assert_eq!(offer.offer_to_receive_video(), 1);
    assert!(offer.voice_activity_detection());
    assert!(offer.use_rtp_mux());
}

#[test]
fn rtp_parameters_round_trip() {
    let mut params = RtpParameters::new();
    params.set_transaction_id("tx-1");
    params.set_mid("video-0");
    assert_eq!(
        params
            .transaction_id()
            .expect("transaction_id の取得に失敗しました"),
        "tx-1"
    );
    assert_eq!(params.mid().expect("mid の取得に失敗しました"), "video-0");

    let mut enc = RtpEncodingParameters::new();
    enc.set_rid("r0");
    enc.set_max_bitrate_bps(Some(500_000));
    let mut encodings = RtpEncodingParametersVector::new(0);
    encodings.push(&enc);
    params.set_encodings(&encodings);

    let got = params.encodings();
    assert_eq!(got.len(), 1);
    let first = got.get(0).expect("encodings の取得に失敗しました");
    assert_eq!(first.rid().expect("rid の取得に失敗しました"), "r0");

    params.set_degradation_preference(Some(DegradationPreference::Balanced));
    assert_eq!(
        params.degradation_preference(),
        Some(DegradationPreference::Balanced)
    );
    params.set_degradation_preference(None);
    assert_eq!(params.degradation_preference(), None);
}

#[test]
fn rtp_sender_get_set_parameters() {
    let dec_audio = AudioDecoderFactory::builtin();
    let enc_audio = AudioEncoderFactory::builtin();
    let enc_video = VideoEncoderFactory::builtin();
    let dec_video = VideoDecoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();

    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc_audio);
    deps_factory.set_audio_decoder_factory(&dec_audio);
    deps_factory.set_video_encoder_factory(enc_video);
    deps_factory.set_video_decoder_factory(dec_video);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let source = AdaptedVideoTrackSource::new();
    let vts = source.cast_to_video_track_source();
    let track = factory
        .create_video_track(&vts, "video-track-1")
        .expect("VideoTrack の生成に失敗しました");

    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let stream_track = track.cast_to_media_stream_track();
    let mut stream_ids = StringVector::new(0);
    stream_ids.push(&CxxString::from_str("stream-0"));
    let mut sender = pc
        .add_track(&stream_track, &stream_ids)
        .expect("AddTrack が失敗しました");

    let params = sender.get_parameters();
    sender
        .set_parameters(&params)
        .expect("set_parameters が失敗しました");

    drop(sender);
    drop(stream_track);
    drop(pc);
    drop(track);
    drop(vts);
    drop(source);
    drop(pc_deps);
    drop(factory);
    drop(deps_factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn peer_connection_create_and_transceiver() {
    // Factory を組み立てる。
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();
    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc);
    deps_factory.set_audio_decoder_factory(&dec);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    // PC 用の構成と observer/dependencies を準備する。
    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);

    // PeerConnection を生成し、取得できることを確認する。
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");
    assert!(!pc.as_ptr().is_null());

    drop(pc);
    drop(pc_deps);
    drop(factory);
    drop(deps_factory);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn peer_connection_lookup_dtls_transport() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();
    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc);
    deps_factory.set_audio_decoder_factory(&dec);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let mut transceiver_init = RtpTransceiverInit::new();
    transceiver_init.set_direction(RtpTransceiverDirection::SendRecv);
    let _ = pc
        .add_transceiver(MediaType::Audio, &mut transceiver_init)
        .expect("transceiver の追加に失敗しました");

    if let Some(dtls_transport) = pc.lookup_dtls_transport_by_mid("0") {
        let observer = DtlsTransportObserver::new_with_handler(Box::new(NoopHandler));
        let _ = dtls_transport.state();
        dtls_transport.register_observer(&observer);
        dtls_transport.unregister_observer();
    }

    drop(pc);
    drop(pc_deps);
    drop(factory);
    drop(deps_factory);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn peer_connection_create_with_proxy_allocator() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();
    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc);
    deps_factory.set_audio_decoder_factory(&dec);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let (factory, context) = PeerConnectionFactory::create_modular_with_context(&mut deps_factory)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");

    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());

    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    pc_deps.set_proxy(
        network_manager,
        socket_factory,
        "127.0.0.1",
        8080,
        "user",
        "pass",
        "shiguredo_webrtc test",
    );
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("Proxy 設定付き PeerConnection の生成に失敗しました");
    assert!(!pc.as_ptr().is_null());

    drop(pc);
    drop(pc_deps);
    drop(context);
    drop(factory);
    drop(deps_factory);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn video_track_and_transceiver_with_track() {
    let dec_audio = AudioDecoderFactory::builtin();
    let enc_audio = AudioEncoderFactory::builtin();
    let enc_video = VideoEncoderFactory::builtin();
    let dec_video = VideoDecoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();

    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc_audio);
    deps_factory.set_audio_decoder_factory(&dec_audio);
    deps_factory.set_video_encoder_factory(enc_video);
    deps_factory.set_video_decoder_factory(dec_video);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    // VideoTrack を生成する。
    let mut source = AdaptedVideoTrackSource::new();
    let vts = source.cast_to_video_track_source();
    let track = factory
        .create_video_track(&vts, "video-track-0")
        .expect("VideoTrack の生成に失敗しました");
    // ついでにフレーム投入 API も呼んでおく。
    let buf = I420Buffer::new(2, 2);
    let frame_buffer = buf.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(1_000_000)
        .set_timestamp_rtp(0)
        .build();
    source.on_frame(&frame);

    // PeerConnection を作成し、トラック付きで transceiver を追加する。
    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let mut init = RtpTransceiverInit::new();
    init.set_direction(RtpTransceiverDirection::SendOnly);
    pc.add_transceiver_with_track(&track, &mut init)
        .expect("AddTransceiverWithTrack が失敗しました");

    // webrtc オブジェクトを先に解放してからスレッドを停止する。
    drop(pc);
    drop(track);
    drop(vts);
    drop(source);
    drop(pc_deps);
    drop(factory);
    drop(deps_factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn peer_connection_observer_and_dependencies() {
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let deps = PeerConnectionDependencies::new(&observer);
    assert!(!deps.as_ptr().is_null());
    drop(deps);
}

#[test]
fn peer_connection_dependencies_set_tls_cert_verifier() {
    struct TestVerifier {
        dropped: Arc<AtomicBool>,
    }

    impl SSLCertificateVerifierHandler for TestVerifier {
        fn verify_chain(&mut self, _chain: SSLCertChainRef<'_>) -> bool {
            true
        }
    }

    impl Drop for TestVerifier {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::SeqCst);
        }
    }

    let dropped = Arc::new(AtomicBool::new(false));
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let mut deps = PeerConnectionDependencies::new(&observer);
    let verifier = SSLCertificateVerifier::new_with_handler(Box::new(TestVerifier {
        dropped: dropped.clone(),
    }));
    deps.set_tls_cert_verifier(verifier);

    drop(deps);
    assert!(
        dropped.load(Ordering::SeqCst),
        "SSLCertificateVerifierHandler が解放されていません"
    );
}

#[test]
fn create_and_set_local_description_observers() {
    let _create_obs = CreateSessionDescriptionObserver::new_with_handler(Box::new(NoopHandler));
    let _set_local = SetLocalDescriptionObserver::new_with_handler(Box::new(NoopHandler));
    let _set_remote = SetRemoteDescriptionObserver::new_with_handler(Box::new(NoopHandler));
}

// VideoEncoderFactory でカスタムエンコーダーを登録して encode を呼び、
// encode callback が呼ばれることを確認する。
#[test]
fn custom_video_encoder_factory_create_and_encode_calls_callbacks() {
    struct TestVideoEncoderHandler {
        encode_count: i32,
    }
    impl VideoEncoderHandler for TestVideoEncoderHandler {
        fn encode(
            &mut self,
            _frame: VideoFrameRef<'_>,
            frame_types: Option<VideoFrameTypeVectorRef<'_>>,
        ) -> VideoCodecStatus {
            let frame_types = frame_types.expect("frame_types が None です");
            assert_eq!(frame_types.len(), 2);
            assert_eq!(frame_types.get(0), Some(VideoFrameType::Key));
            assert_eq!(frame_types.get(1), Some(VideoFrameType::Delta));
            self.encode_count += 1;
            VideoCodecStatus::Unknown(self.encode_count)
        }
    }

    struct TestVideoEncoderFactoryHandler {
        created: bool,
    }
    impl VideoEncoderFactoryHandler for TestVideoEncoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            format: SdpVideoFormatRef<'_>,
        ) -> Option<VideoEncoder> {
            assert!(!env.as_ptr().is_null());
            assert_eq!(
                format
                    .name()
                    .expect("SdpVideoFormatRef::name に失敗しました"),
                "VP8"
            );
            if self.created {
                return None;
            }
            self.created = true;
            Some(VideoEncoder::new_with_handler(Box::new(
                TestVideoEncoderHandler { encode_count: 0 },
            )))
        }
    }

    let factory = VideoEncoderFactory::new_with_handler(Box::new(TestVideoEncoderFactoryHandler {
        created: false,
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("VP8");
    let mut encoder = factory
        .create(env.as_ref(), format.as_ref())
        .expect("custom encoder の作成に失敗しました");

    let buffer = I420Buffer::new(2, 2);
    let frame_buffer = buffer.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(123)
        .set_timestamp_rtp(0)
        .build();
    let mut frame_types = VideoFrameTypeVector::new(0);
    frame_types.push(VideoFrameType::Key);
    frame_types.push(VideoFrameType::Delta);

    assert_eq!(
        encoder.encode(frame.as_ref(), Some(frame_types.as_ref())),
        VideoCodecStatus::NoOutput
    );
    assert_eq!(
        encoder.encode(frame.as_ref(), Some(frame_types.as_ref())),
        VideoCodecStatus::Unknown(2)
    );
    assert!(
        factory.create(env.as_ref(), format.as_ref()).is_none(),
        "2 回目の create は None を返す想定です"
    );
}

#[test]
fn custom_video_encoder_get_encoder_info_roundtrip_all_fields() {
    struct TestVideoEncoderHandler;
    impl VideoEncoderHandler for TestVideoEncoderHandler {
        fn get_encoder_info(&mut self) -> VideoEncoderEncoderInfo {
            let mut info = VideoEncoderEncoderInfo::new();
            info.set_implementation_name("encoder-info-full");

            let mut scaling = VideoEncoderScalingSettings::new();
            let mut thresholds = VideoEncoderQpThresholds::new();
            thresholds.set_low(11);
            thresholds.set_high(33);
            scaling.set_thresholds(Some(&thresholds));
            scaling.set_min_pixels_per_frame(12345);
            info.set_scaling_settings(&scaling);

            info.set_requested_resolution_alignment(4);
            info.set_apply_alignment_to_all_simulcast_layers(true);
            info.set_supports_native_handle(true);
            info.set_has_trusted_rate_controller(true);
            info.set_is_hardware_accelerated(true);

            if let Some(mut fps0) = info.fps_allocation(0) {
                fps0.clear();
                fps0.push(128);
                fps0.push(255);
            } else {
                panic!("fps_allocation(0) が取得できません");
            }
            if let Some(mut fps1) = info.fps_allocation(1) {
                fps1.clear();
                fps1.push(64);
            } else {
                panic!("fps_allocation(1) が取得できません");
            }

            let limits0 =
                VideoEncoderResolutionBitrateLimits::new(640 * 360, 100000, 80000, 500000);
            let limits1 =
                VideoEncoderResolutionBitrateLimits::new(1280 * 720, 300000, 200000, 1500000);
            {
                let mut limits = info.resolution_bitrate_limits();
                limits.clear();
                limits.push(&limits0);
                limits.push(&limits1);
            }

            info.set_supports_simulcast(true);
            {
                let mut preferred = info.preferred_pixel_formats();
                preferred.clear();
                preferred.push(VideoFrameBufferKind::I420);
                preferred.push(VideoFrameBufferKind::Nv12);
            }

            info.set_is_qp_trusted(Some(true));
            info.set_min_qp(Some(9));
            let mapped = VideoEncoderResolution::new(1280, 720);
            info.set_mapped_resolution(Some(&mapped));
            info
        }
    }

    let encoder = VideoEncoder::new_with_handler(Box::new(TestVideoEncoderHandler));
    let mut info = encoder.get_encoder_info();

    assert_eq!(
        info.implementation_name()
            .expect("implementation_name の取得に失敗しました"),
        "encoder-info-full"
    );
    assert_eq!(info.requested_resolution_alignment(), 4);
    assert!(info.apply_alignment_to_all_simulcast_layers());
    assert!(info.supports_native_handle());
    assert!(info.has_trusted_rate_controller());
    assert!(info.is_hardware_accelerated());
    assert!(info.supports_simulcast());

    let scaling = info.scaling_settings();
    let thresholds = scaling.thresholds().expect("thresholds が None です");
    assert_eq!(thresholds.low(), 11);
    assert_eq!(thresholds.high(), 33);
    assert_eq!(scaling.min_pixels_per_frame(), 12345);

    let mut fps0 = info
        .fps_allocation(0)
        .expect("fps_allocation(0) が None です");
    assert_eq!(fps0.len(), 2);
    assert_eq!(fps0.get(0), Some(128));
    assert_eq!(fps0.get(1), Some(255));
    assert!(fps0.set(1, 200));
    assert_eq!(fps0.get(1), Some(200));

    let fps1 = info
        .fps_allocation(1)
        .expect("fps_allocation(1) が None です");
    assert_eq!(fps1.len(), 1);
    assert_eq!(fps1.get(0), Some(64));

    {
        let mut limits = info.resolution_bitrate_limits();
        assert_eq!(limits.len(), 2);
        let limits0 = limits
            .get(0)
            .expect("resolution_bitrate_limits[0] が None です");
        assert_eq!(limits0.frame_size_pixels(), 640 * 360);
        assert_eq!(limits0.min_start_bitrate_bps(), 100000);
        assert_eq!(limits0.min_bitrate_bps(), 80000);
        assert_eq!(limits0.max_bitrate_bps(), 500000);

        let replacement =
            VideoEncoderResolutionBitrateLimits::new(1920 * 1080, 500000, 400000, 2500000);
        assert!(
            limits.set(1, &replacement),
            "resolution_bitrate_limits.set(1) が失敗しました"
        );
        let limits1 = limits
            .get(1)
            .expect("resolution_bitrate_limits[1] が None です");
        assert_eq!(limits1.frame_size_pixels(), 1920 * 1080);
        assert_eq!(limits1.min_start_bitrate_bps(), 500000);
        assert_eq!(limits1.min_bitrate_bps(), 400000);
        assert_eq!(limits1.max_bitrate_bps(), 2500000);
    }

    let mut preferred = info.preferred_pixel_formats();
    assert_eq!(preferred.len(), 2);
    assert_eq!(preferred.get(0), Some(VideoFrameBufferKind::I420));
    assert_eq!(preferred.get(1), Some(VideoFrameBufferKind::Nv12));
    assert!(preferred.set(1, VideoFrameBufferKind::I420A));
    assert_eq!(preferred.get(1), Some(VideoFrameBufferKind::I420A));

    assert_eq!(info.is_qp_trusted(), Some(true));
    assert_eq!(info.min_qp(), Some(9));
    let mapped = info
        .mapped_resolution()
        .expect("mapped_resolution が None です");
    assert_eq!(mapped.width(), 1280);
    assert_eq!(mapped.height(), 720);

    let info_text = info.to_string().expect("ToString に失敗しました");
    assert!(!info_text.is_empty(), "ToString の結果が空です");
    assert!(
        info_text.contains("encoder-info-full"),
        "ToString に implementation_name が含まれていません: {}",
        info_text
    );

    let limits = info
        .get_encoder_bitrate_limits_for_resolution(640 * 360)
        .expect("GetEncoderBitrateLimitsForResolution(640x360) が None です");
    assert_eq!(limits.frame_size_pixels(), 640 * 360);
    assert_eq!(limits.min_start_bitrate_bps(), 100000);
    assert_eq!(limits.min_bitrate_bps(), 80000);
    assert_eq!(limits.max_bitrate_bps(), 500000);

    info.set_is_qp_trusted(None);
    assert_eq!(info.is_qp_trusted(), None);
    info.set_min_qp(None);
    assert_eq!(info.min_qp(), None);
    info.set_mapped_resolution(None);
    assert!(info.mapped_resolution().is_none());

    let mut scaling_none = VideoEncoderScalingSettings::new();
    scaling_none.set_thresholds(None);
    info.set_scaling_settings(&scaling_none);
    assert!(info.scaling_settings().thresholds().is_none());
}

#[test]
fn video_encoder_factory_get_supported_formats_returns_owned_formats() {
    struct TestVideoEncoderFactoryHandler;
    impl VideoEncoderFactoryHandler for TestVideoEncoderFactoryHandler {
        fn get_supported_formats(&mut self) -> Vec<SdpVideoFormat> {
            let mut h264 = SdpVideoFormat::new("H264");
            h264.parameters_mut().set("profile-level-id", "42e01f");
            let mut vp8 = SdpVideoFormat::new("VP8");
            vp8.parameters_mut().set("x-google-start-bitrate", "300");
            vec![h264, vp8]
        }
    }

    let factory = VideoEncoderFactory::new_with_handler(Box::new(TestVideoEncoderFactoryHandler));
    let mut formats = factory.get_supported_formats();
    assert_eq!(formats.len(), 2);
    assert_eq!(
        formats[0].name().expect("name の取得に失敗しました"),
        "H264"
    );
    assert_eq!(formats[1].name().expect("name の取得に失敗しました"), "VP8");

    let params: std::collections::HashMap<String, String> = formats
        .get_mut(0)
        .expect("先頭フォーマットが存在しません")
        .parameters_mut()
        .iter()
        .collect();
    assert_eq!(
        params.get("profile-level-id").map(String::as_str),
        Some("42e01f")
    );
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn objc_video_encoder_factory_bridge_works() {
    let objc_factory = unsafe { ffi::webrtc_objc_RTCDefaultVideoEncoderFactory_new() };
    assert!(
        !objc_factory.is_null(),
        "webrtc_objc_RTCDefaultVideoEncoderFactory_new returned null"
    );

    let native_unique = unsafe { ffi::webrtc_ObjCToNativeVideoEncoderFactory(objc_factory) };
    assert!(
        !native_unique.is_null(),
        "webrtc_ObjCToNativeVideoEncoderFactory returned null"
    );

    let native = unsafe { ffi::webrtc_VideoEncoderFactory_unique_get(native_unique) };
    assert!(
        !native.is_null(),
        "webrtc_VideoEncoderFactory_unique_get returned null"
    );

    let formats = unsafe { ffi::webrtc_VideoEncoderFactory_GetSupportedFormats(native) };
    assert!(
        !formats.is_null(),
        "webrtc_VideoEncoderFactory_GetSupportedFormats returned null"
    );
    let size = unsafe { ffi::webrtc_SdpVideoFormat_vector_size(formats) };
    assert!(size >= 0, "invalid format size: {size}");

    unsafe {
        ffi::webrtc_SdpVideoFormat_vector_delete(formats);
        ffi::webrtc_VideoEncoderFactory_unique_delete(native_unique);
        ffi::webrtc_objc_RTCVideoEncoderFactory_release(objc_factory);
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn objc_video_decoder_factory_bridge_works() {
    let objc_factory = unsafe { ffi::webrtc_objc_RTCDefaultVideoDecoderFactory_new() };
    assert!(
        !objc_factory.is_null(),
        "webrtc_objc_RTCDefaultVideoDecoderFactory_new returned null"
    );

    let native_unique = unsafe { ffi::webrtc_ObjCToNativeVideoDecoderFactory(objc_factory) };
    assert!(
        !native_unique.is_null(),
        "webrtc_ObjCToNativeVideoDecoderFactory returned null"
    );

    let native = unsafe { ffi::webrtc_VideoDecoderFactory_unique_get(native_unique) };
    assert!(
        !native.is_null(),
        "webrtc_VideoDecoderFactory_unique_get returned null"
    );

    let formats = unsafe { ffi::webrtc_VideoDecoderFactory_GetSupportedFormats(native) };
    assert!(
        !formats.is_null(),
        "webrtc_VideoDecoderFactory_GetSupportedFormats returned null"
    );
    let size = unsafe { ffi::webrtc_SdpVideoFormat_vector_size(formats) };
    assert!(size >= 0, "invalid format size: {size}");

    unsafe {
        ffi::webrtc_SdpVideoFormat_vector_delete(formats);
        ffi::webrtc_VideoDecoderFactory_unique_delete(native_unique);
        ffi::webrtc_objc_RTCVideoDecoderFactory_release(objc_factory);
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn video_encoder_factory_from_objc_default_works() {
    let factory = VideoEncoderFactory::from_objc_default()
        .expect("VideoEncoderFactory::from_objc_default が None を返しました");
    let _formats = factory.get_supported_formats();
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn video_decoder_factory_from_objc_default_works() {
    let factory = VideoDecoderFactory::from_objc_default()
        .expect("VideoDecoderFactory::from_objc_default が None を返しました");
    let _formats = factory.get_supported_formats();
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[test]
fn objc_video_factory_functions_return_null_on_non_apple() {
    let enc_objc = unsafe { ffi::webrtc_objc_RTCDefaultVideoEncoderFactory_new() };
    assert!(
        enc_objc.is_null(),
        "encoder objc factory should be null on non-Apple platforms"
    );
    let enc_native = unsafe {
        ffi::webrtc_ObjCToNativeVideoEncoderFactory(std::ptr::null_mut::<
            ffi::webrtc_objc_RTCVideoEncoderFactory,
        >())
    };
    assert!(
        enc_native.is_null(),
        "encoder native factory should be null on non-Apple platforms"
    );
    unsafe {
        ffi::webrtc_objc_RTCVideoEncoderFactory_release(std::ptr::null_mut::<
            ffi::webrtc_objc_RTCVideoEncoderFactory,
        >())
    };

    let dec_objc = unsafe { ffi::webrtc_objc_RTCDefaultVideoDecoderFactory_new() };
    assert!(
        dec_objc.is_null(),
        "decoder objc factory should be null on non-Apple platforms"
    );
    let dec_native = unsafe {
        ffi::webrtc_ObjCToNativeVideoDecoderFactory(std::ptr::null_mut::<
            ffi::webrtc_objc_RTCVideoDecoderFactory,
        >())
    };
    assert!(
        dec_native.is_null(),
        "decoder native factory should be null on non-Apple platforms"
    );
    unsafe {
        ffi::webrtc_objc_RTCVideoDecoderFactory_release(std::ptr::null_mut::<
            ffi::webrtc_objc_RTCVideoDecoderFactory,
        >())
    };
}

#[test]
fn video_decoder_factory_get_supported_formats_returns_owned_formats() {
    struct TestVideoDecoderFactoryHandler;
    impl VideoDecoderFactoryHandler for TestVideoDecoderFactoryHandler {
        fn get_supported_formats(&mut self) -> Vec<SdpVideoFormat> {
            let mut h264 = SdpVideoFormat::new("H264");
            h264.parameters_mut().set("packetization-mode", "1");
            vec![h264]
        }
    }

    let factory = VideoDecoderFactory::new_with_handler(Box::new(TestVideoDecoderFactoryHandler));
    let mut formats = factory.get_supported_formats();
    assert_eq!(formats.len(), 1);
    assert_eq!(
        formats[0].name().expect("name の取得に失敗しました"),
        "H264"
    );
    let params: std::collections::HashMap<String, String> = formats
        .get_mut(0)
        .expect("先頭フォーマットが存在しません")
        .parameters_mut()
        .iter()
        .collect();
    assert_eq!(
        params.get("packetization-mode").map(String::as_str),
        Some("1")
    );
}

#[test]
fn video_encoder_factory_create_calls_create_callback() {
    struct TestVideoEncoderFactoryHandler {
        called: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }
    impl VideoEncoderFactoryHandler for TestVideoEncoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            format: SdpVideoFormatRef<'_>,
        ) -> Option<VideoEncoder> {
            self.called.store(true, std::sync::atomic::Ordering::SeqCst);
            assert!(!env.as_ptr().is_null());
            assert_eq!(
                format
                    .name()
                    .expect("SdpVideoFormatRef::name に失敗しました"),
                "H264"
            );
            Some(VideoEncoder::new_with_handler(Box::new(NoopHandler)))
        }
    }

    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let factory = VideoEncoderFactory::new_with_handler(Box::new(TestVideoEncoderFactoryHandler {
        called: called.clone(),
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("H264");
    let encoder = factory.create(env.as_ref(), format.as_ref());
    assert!(encoder.is_some(), "create が None を返しました");
    assert!(
        called.load(std::sync::atomic::Ordering::SeqCst),
        "create callback が呼ばれていません"
    );
}

#[test]
fn video_decoder_factory_create_calls_create_callback() {
    struct TestVideoDecoderFactoryHandler {
        called: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }
    impl VideoDecoderFactoryHandler for TestVideoDecoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            format: SdpVideoFormatRef<'_>,
        ) -> Option<VideoDecoder> {
            self.called.store(true, std::sync::atomic::Ordering::SeqCst);
            assert!(!env.as_ptr().is_null());
            assert_eq!(
                format
                    .name()
                    .expect("SdpVideoFormatRef::name に失敗しました"),
                "H264"
            );
            Some(VideoDecoder::new_with_handler(Box::new(NoopHandler)))
        }
    }

    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let factory = VideoDecoderFactory::new_with_handler(Box::new(TestVideoDecoderFactoryHandler {
        called: called.clone(),
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("H264");
    let decoder = factory.create(env.as_ref(), format.as_ref());
    assert!(decoder.is_some(), "create が None を返しました");
    assert!(
        called.load(std::sync::atomic::Ordering::SeqCst),
        "create callback が呼ばれていません"
    );
}

// VideoEncoder で encode を呼び、encoded_image と codec_specific_info を受け取れることを確認する。
#[test]
fn custom_video_encoder_register_and_encode_calls_encoded_image_and_codec_specific_info() {
    #[derive(Default)]
    struct State {
        callback_ptr: Option<VideoEncoderEncodedImageCallbackPtr>,
        register_called: bool,
        encode_called: bool,
        on_encoded_image_called: bool,
        order: Vec<&'static str>,
    }

    #[derive(Clone, Copy)]
    struct StatePtr(*mut State);
    unsafe impl Send for StatePtr {}
    impl StatePtr {
        unsafe fn get_mut<'a>(&self) -> &'a mut State {
            unsafe { &mut *self.0 }
        }
    }

    struct TestVideoEncoderHandler {
        state_ptr: StatePtr,
    }
    impl VideoEncoderHandler for TestVideoEncoderHandler {
        fn register_encode_complete_callback(
            &mut self,
            callback: Option<VideoEncoderEncodedImageCallbackRef<'_>>,
        ) -> VideoCodecStatus {
            let callback = callback.expect("register 側 callback が None です");
            let state = unsafe { self.state_ptr.get_mut() };
            state.register_called = true;
            state.order.push("register");
            state.callback_ptr =
                Some(unsafe { VideoEncoderEncodedImageCallbackPtr::from_ref(callback) });
            VideoCodecStatus::Ok
        }

        fn encode(
            &mut self,
            _frame: VideoFrameRef<'_>,
            _frame_types: Option<VideoFrameTypeVectorRef<'_>>,
        ) -> VideoCodecStatus {
            {
                let state = unsafe { self.state_ptr.get_mut() };
                state.encode_called = true;
                state.order.push("encode");
            }

            let callback_ptr = {
                let state = unsafe { self.state_ptr.get_mut() };
                state
                    .callback_ptr
                    .expect("encode 側 callback_ptr が未設定です")
            };

            let buffer = EncodedImageBuffer::from_bytes(&[1, 2, 3, 4]);
            let mut image = EncodedImage::new();
            image.set_encoded_data(&buffer);
            image.set_rtp_timestamp(12345);
            image.set_encoded_width(640);
            image.set_encoded_height(360);
            image.set_frame_type(VideoFrameType::Key);
            image.set_qp(31);

            let mut codec_specific_info = CodecSpecificInfo::new();
            codec_specific_info.set_codec_type(VideoCodecType::H264);
            codec_specific_info.set_end_of_picture(true);
            codec_specific_info.set_h264_packetization_mode(H264PacketizationMode::SingleNalUnit);
            codec_specific_info.set_h264_temporal_idx(2);
            codec_specific_info.set_h264_base_layer_sync(true);
            codec_specific_info.set_h264_idr_frame(true);

            let result = unsafe {
                callback_ptr.on_encoded_image(image.as_ref(), Some(codec_specific_info.as_ref()))
            };
            assert_eq!(
                result.error(),
                VideoEncoderEncodedImageCallbackResultError::Ok
            );
            assert_eq!(result.frame_id(), 9999);
            assert!(!result.drop_next_frame());
            VideoCodecStatus::Unknown(88)
        }
    }

    struct TestEncodedImageCallbackHandler {
        state_ptr: StatePtr,
    }
    impl VideoEncoderEncodedImageCallbackHandler for TestEncodedImageCallbackHandler {
        fn on_encoded_image(
            &mut self,
            image: EncodedImageRef<'_>,
            codec_specific_info: Option<CodecSpecificInfoRef<'_>>,
        ) -> VideoEncoderEncodedImageCallbackResult {
            let state = unsafe { self.state_ptr.get_mut() };
            state.on_encoded_image_called = true;
            state.order.push("on_encoded_image");

            let encoded_data = image.encoded_data().expect("encoded_data が None です");
            assert_eq!(encoded_data.data(), [1, 2, 3, 4]);
            assert_eq!(encoded_data.data().len(), 4);
            assert_eq!(image.rtp_timestamp(), 12345);
            assert_eq!(image.encoded_width(), 640);
            assert_eq!(image.encoded_height(), 360);
            assert_eq!(image.frame_type(), VideoFrameType::Key);
            assert_eq!(image.qp(), 31);

            let codec_specific_info =
                codec_specific_info.expect("codec_specific_info が None です");
            assert_eq!(codec_specific_info.codec_type(), VideoCodecType::H264);
            assert!(codec_specific_info.end_of_picture());
            assert_eq!(
                codec_specific_info.h264_packetization_mode(),
                H264PacketizationMode::SingleNalUnit
            );
            assert_eq!(codec_specific_info.h264_temporal_idx(), 2);
            assert!(codec_specific_info.h264_base_layer_sync());
            assert!(codec_specific_info.h264_idr_frame());
            VideoEncoderEncodedImageCallbackResult::new_with_frame_id(
                VideoEncoderEncodedImageCallbackResultError::Ok,
                9999,
            )
        }
    }

    let mut state = Box::new(State::default());
    let state_ptr = StatePtr((&mut *state) as *mut State);
    let mut encoder =
        VideoEncoder::new_with_handler(Box::new(TestVideoEncoderHandler { state_ptr }));
    let encoded_image_callback = VideoEncoderEncodedImageCallback::new_with_handler(Box::new(
        TestEncodedImageCallbackHandler { state_ptr },
    ));

    assert_eq!(
        encoder.register_encode_complete_callback(Some(encoded_image_callback.as_ref())),
        VideoCodecStatus::Ok
    );

    let buffer = I420Buffer::new(2, 2);
    let frame_buffer = buffer.cast_to_video_frame_buffer();
    let frame = VideoFrame::builder(&frame_buffer)
        .set_timestamp_us(123)
        .set_timestamp_rtp(0)
        .build();
    assert_eq!(
        encoder.encode(frame.as_ref(), None),
        VideoCodecStatus::Unknown(88)
    );

    assert!(state.register_called, "register が呼ばれていません");
    assert!(state.encode_called, "encode が呼ばれていません");
    assert!(
        state.on_encoded_image_called,
        "on_encoded_image が呼ばれていません"
    );
    assert_eq!(
        state.order,
        vec!["register", "encode", "on_encoded_image"],
        "呼び出し順が不正です"
    );
}

#[test]
fn simulcast_encoder_adapter_new_works() {
    let env = Environment::new();
    let primary_factory = VideoEncoderFactory::builtin();
    let format = SdpVideoFormat::new("VP8");

    let _adapter =
        SimulcastEncoderAdapter::new(env.as_ref(), &primary_factory, None, format.as_ref());
}

#[test]
fn simulcast_encoder_adapter_cast_to_video_encoder_works() {
    let env = Environment::new();
    let primary_factory = VideoEncoderFactory::builtin();
    let format = SdpVideoFormat::new("VP8");

    let adapter =
        SimulcastEncoderAdapter::new(env.as_ref(), &primary_factory, None, format.as_ref());
    let encoder = adapter.cast_to_video_encoder();
    let info = encoder.get_encoder_info();
    let _ = info
        .implementation_name()
        .expect("implementation_name の取得に失敗しました");
}

// VideoDecoderFactory の create callback と、VideoDecoder の decode callback が呼ばれることを確認する。
#[test]
fn custom_video_decoder_factory_create_and_decode_calls_callbacks() {
    struct TestVideoDecoderHandler {
        decode_count: i32,
    }
    impl VideoDecoderHandler for TestVideoDecoderHandler {
        fn decode(
            &mut self,
            input_image: EncodedImageRef<'_>,
            render_time_ms: i64,
        ) -> VideoCodecStatus {
            assert!(input_image.encoded_data().is_none());
            assert_eq!(render_time_ms, 456);
            self.decode_count += 1;
            VideoCodecStatus::Unknown(self.decode_count)
        }
    }

    struct TestVideoDecoderFactoryHandler {
        created: bool,
    }
    impl VideoDecoderFactoryHandler for TestVideoDecoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            _format: SdpVideoFormatRef<'_>,
        ) -> Option<VideoDecoder> {
            assert!(!env.as_ptr().is_null());
            if self.created {
                return None;
            }
            self.created = true;
            Some(VideoDecoder::new_with_handler(Box::new(
                TestVideoDecoderHandler { decode_count: 0 },
            )))
        }
    }

    let factory = VideoDecoderFactory::new_with_handler(Box::new(TestVideoDecoderFactoryHandler {
        created: false,
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("VP8");
    let mut decoder = factory
        .create(env.as_ref(), format.as_ref())
        .expect("custom decoder の作成に失敗しました");
    let image = EncodedImage::new();

    assert_eq!(
        decoder.decode(image.as_ref(), 456),
        VideoCodecStatus::NoOutput
    );
    assert_eq!(
        decoder.decode(image.as_ref(), 456),
        VideoCodecStatus::Unknown(2)
    );
    assert!(
        factory.create(env.as_ref(), format.as_ref()).is_none(),
        "2 回目の create は None を返す想定です"
    );
}

#[test]
fn video_decoder_handler_register_decode_complete_callback_accepts_none_and_some() {
    struct TestVideoDecoderHandler {
        called_with_none: bool,
        called_with_some: bool,
    }
    impl VideoDecoderHandler for TestVideoDecoderHandler {
        fn register_decode_complete_callback(
            &mut self,
            callback: Option<VideoDecoderDecodedImageCallbackPtr>,
        ) -> VideoCodecStatus {
            if callback.is_some() {
                self.called_with_some = true;
            } else {
                self.called_with_none = true;
            }
            VideoCodecStatus::Ok
        }
    }

    let mut handler = TestVideoDecoderHandler {
        called_with_none: false,
        called_with_some: false,
    };
    assert_eq!(
        handler.register_decode_complete_callback(None),
        VideoCodecStatus::Ok
    );
    let dummy_callback = unsafe {
        // このテストでは callback を呼び出さず Option::Some 経路だけを確認する。
        VideoDecoderDecodedImageCallbackPtr::from_raw(NonNull::dangling())
    };
    assert_eq!(
        handler.register_decode_complete_callback(Some(dummy_callback)),
        VideoCodecStatus::Ok
    );
    assert!(handler.called_with_none);
    assert!(handler.called_with_some);
}

// implementation_name() が解放済みの値を返していることがあったので、その回帰テストを行う
#[test]
fn custom_video_decoder_get_decoder_info_name_experiment() {
    struct TestVideoDecoderHandler {
        expected: String,
    }
    impl VideoDecoderHandler for TestVideoDecoderHandler {
        fn get_decoder_info(&mut self) -> VideoDecoderDecoderInfo {
            let mut info = VideoDecoderDecoderInfo::new();
            info.set_implementation_name(&self.expected);
            info.set_is_hardware_accelerated(false);
            info
        }
    }

    let expected = "decoder-info-name-".repeat(128);
    let decoder = VideoDecoder::new_with_handler(Box::new(TestVideoDecoderHandler {
        expected: expected.clone(),
    }));

    for _ in 0..100 {
        let info = decoder.get_decoder_info();
        assert_eq!(
            info.implementation_name()
                .expect("implementation_name の取得に失敗しました"),
            expected,
            "GetDecoderInfo の implementation_name が不一致になりました"
        );
        assert!(!info.is_hardware_accelerated());
    }
}

#[test]
fn create_local_media_stream_returns_requested_id() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();
    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc);
    deps_factory.set_audio_decoder_factory(&dec);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let stream = factory
        .create_local_media_stream("stream-0")
        .expect("CreateLocalMediaStream が失敗しました");
    assert_eq!(
        stream.id().expect("MediaStream id の取得に失敗しました"),
        "stream-0"
    );

    drop(stream);
    drop(factory);
    drop(deps_factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn media_stream_track_round_trip() {
    let dec_audio = AudioDecoderFactory::builtin();
    let enc_audio = AudioEncoderFactory::builtin();
    let enc_video = VideoEncoderFactory::builtin();
    let dec_video = VideoDecoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();
    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc_audio);
    deps_factory.set_audio_decoder_factory(&dec_audio);
    deps_factory.set_video_encoder_factory(enc_video);
    deps_factory.set_video_decoder_factory(dec_video);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let stream = factory
        .create_local_media_stream("stream-1")
        .expect("CreateLocalMediaStream が失敗しました");
    let audio_source = factory
        .create_audio_source()
        .expect("AudioSource の生成に失敗しました");
    let audio_track = factory
        .create_audio_track(&audio_source, "audio-track-0")
        .expect("AudioTrack の生成に失敗しました");
    let video_source = AdaptedVideoTrackSource::new();
    let vts = video_source.cast_to_video_track_source();
    let video_track = factory
        .create_video_track(&vts, "video-track-0")
        .expect("VideoTrack の生成に失敗しました");

    assert!(stream.audio_tracks().is_empty());
    assert!(stream.video_tracks().is_empty());
    assert!(stream.add_audio_track(&audio_track));
    assert!(stream.add_video_track(&video_track));

    let audio_tracks = stream.audio_tracks();
    let video_tracks = stream.video_tracks();
    assert_eq!(audio_tracks.len(), 1);
    assert_eq!(video_tracks.len(), 1);

    let found_audio = stream
        .find_audio_track("audio-track-0")
        .expect("FindAudioTrack が None を返しました");
    let found_video = stream
        .find_video_track("video-track-0")
        .expect("FindVideoTrack が None を返しました");
    assert_eq!(
        found_audio
            .cast_to_media_stream_track()
            .id()
            .expect("audio track id の取得に失敗しました"),
        "audio-track-0"
    );
    assert_eq!(
        found_video
            .cast_to_media_stream_track()
            .id()
            .expect("video track id の取得に失敗しました"),
        "video-track-0"
    );
    assert!(stream.find_audio_track("audio-track-unknown").is_none());
    assert!(stream.find_video_track("video-track-unknown").is_none());

    assert!(stream.remove_audio_track(&audio_track));
    assert!(stream.remove_video_track(&video_track));
    assert!(stream.find_audio_track("audio-track-0").is_none());
    assert!(stream.find_video_track("video-track-0").is_none());

    drop(found_video);
    drop(found_audio);
    drop(video_tracks);
    drop(audio_tracks);
    drop(video_track);
    drop(vts);
    drop(video_source);
    drop(audio_track);
    drop(audio_source);
    drop(stream);
    drop(factory);
    drop(deps_factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}
