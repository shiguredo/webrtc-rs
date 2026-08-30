use super::*;
use std::ptr::NonNull;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
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
fn random_string_zero_length() {
    let s = random_string(0);
    assert_eq!(s.len(), 0, "長さ 0 を指定したら空文字列が返ること");
}

#[test]
fn random_bytes_zero_length() {
    let b = random_bytes(0);
    assert_eq!(b.len(), 0, "長さ 0 を指定したら空 Vec が返ること");
}

#[test]
fn random_string_usize_values() {
    let s = random_string(65536);
    assert_eq!(s.len(), 65536, "65536 バイトの文字列が返ること");
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
    assert_eq!(constants::no_picture_id(), -1);
    assert_eq!(constants::no_tl0_pic_idx(), -1);
    assert_eq!(constants::no_temporal_idx(), 0xFF);
    assert_eq!(constants::no_key_idx(), -1);
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
        .expect("fuzzy_match_sdp_video_format が一致するフォーマットを見つけられませんでした");
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
        .expect("fuzzy_match_sdp_video_format が一致するフォーマットを見つけられませんでした");
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

    // 配列サイズの公開メソッドが libwebrtc の定数と一致することを確認する。
    assert_eq!(constants::max_simulcast_streams(), 3);
    assert_eq!(constants::max_spatial_layers(), 5);

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
    // (offset_x, offset_y, crop_width, crop_height, scaled_width, scaled_height)
    type CropAndScaleArgs = (i32, i32, i32, i32, i32, i32);

    struct CropAndScaleBufferHandler {
        called: Arc<AtomicBool>,
        args: Arc<Mutex<Option<CropAndScaleArgs>>>,
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
        .expect("I420 buffer の as_i420 に失敗しました");
    assert_eq!(i420_view.width(), 2);
    assert_eq!(i420_view.height(), 2);
    assert!(i420_frame_buffer.as_nv12().is_none());

    let nv12 = NV12Buffer::new(2, 2);
    let nv12_frame_buffer = nv12.cast_to_video_frame_buffer();
    let nv12_view = nv12_frame_buffer
        .as_nv12()
        .expect("NV12 buffer の as_nv12 に失敗しました");
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
fn logging_functions_are_callable() {
    // severity は Info にしておく。実際のログ内容は検証しない。
    // initialize_logging は最初のログ出力前に呼ぶ必要があるが、テストの実行順序は
    // 保証されないため戻り値の検証は行わない。
    let mut config = log::LoggingConfig::new();
    config.set_min_severity(log::Severity::Info);
    config.set_debug_severity(log::Severity::Info);
    config.set_log_timestamp(true);
    config.set_log_thread(true);
    config.set_log_queue_name(true);
    config.set_log_to_stderr(true);
    config.set_log_prefix("prefix");
    assert_eq!(config.min_severity(), log::Severity::Info);
    assert_eq!(config.debug_severity(), log::Severity::Info);
    assert!(config.log_timestamp());
    assert!(config.log_thread());
    assert!(config.log_queue_name());
    assert!(config.log_to_stderr());
    assert_eq!(config.log_prefix().unwrap(), "prefix");
    log::initialize_logging(config);
    log::print(log::Severity::Info, "webrtc-c", 0, "log test");
}

#[test]
fn logging_long_message_is_not_truncated() {
    // webrtc_c のログは C++ 側 (webrtc::LogMessage) が stderr へ直接書き込むため、
    // Rust テストハーネスの標準的な出力キャプチャでは捕捉できない。
    // テストバイナリをサブプロセスとして起動して stderr を捕捉し、
    // 出力内容を照合する方式で検証する。
    let exe = std::env::current_exe().expect("テストバイナリのパスを取得できませんでした");
    for len in [16, 4096, 70000] {
        // 4096 は旧実装の固定バッファサイズ、70000 は 65536 バイトを超える
        // 長文メッセージの代表。16 は既存の短いメッセージ相当。
        let output = std::process::Command::new(&exe)
            .arg("logging_message_helper")
            .env("WEBRTC_LOG_MESSAGE_LEN", len.to_string())
            .stderr(std::process::Stdio::piped())
            .output()
            .expect("サブプロセスの実行に失敗しました");
        let stderr = String::from_utf8(output.stderr)
            .expect("サブプロセスの stderr が UTF-8 ではありません");
        // stderr にはヘッダー（時刻・スレッド ID など）が付く可能性があるため、
        // 'A' の総数ではなく「連続する 'A' の最大長」で照合する。
        // 切り詰められていれば連続長は len 未満になり、ヘッダーに 'A' が
        // 含まれていてもメッセージ長の検証に影響しない。
        let max_a_run = stderr
            .chars()
            .fold((0, 0), |(best, current), c| {
                if c == 'A' {
                    (best.max(current + 1), current + 1)
                } else {
                    (best, 0)
                }
            })
            .0;
        assert_eq!(
            max_a_run, len,
            "len={len} のメッセージが切り詰められずに出力されていません\nstderr:\n{stderr}"
        );
    }
}

#[test]
fn logging_message_helper() {
    // 検証用ヘルパー。ログ (webrtc::LogMessage) は stderr へ直接書き込まれるため、
    // logging_long_message_is_not_truncated からサブプロセスとして実行される。
    let config = log::LoggingConfig::new();
    log::initialize_logging(config);
    // 環境変数でメッセージ長を指定する（指定なしの場合は短いメッセージ）。
    let len = std::env::var("WEBRTC_LOG_MESSAGE_LEN")
        .map(|v| {
            v.parse::<usize>()
                .expect("WEBRTC_LOG_MESSAGE_LEN は数値で指定してください")
        })
        .unwrap_or(16);
    let message = "A".repeat(len);
    log::print(log::Severity::Info, "webrtc-c", 0, &message);
}

#[test]
fn logging_sink_drop_releases_handler() {
    struct TestSinkHandler {
        destroyed: Arc<AtomicBool>,
    }

    impl log::LogSinkHandler for TestSinkHandler {}

    impl Drop for TestSinkHandler {
        fn drop(&mut self) {
            self.destroyed.store(true, Ordering::SeqCst);
        }
    }

    // LogSink 単体の drop で OnDestroy が呼ばれ、handler が解放されることと、
    // 二重解放でないことを検証する。
    let destroyed = Arc::new(AtomicBool::new(false));
    let sink = log::LogSink::new_with_handler(Box::new(TestSinkHandler {
        destroyed: destroyed.clone(),
    }));
    drop(sink);
    assert!(
        destroyed.load(Ordering::SeqCst),
        "log::LogSinkHandler が解放されていません"
    );
}

#[test]
fn logging_sink_receives_log_line_ref() {
    let exe = std::env::current_exe().expect("テストバイナリのパスを取得できませんでした");
    let expected = "log sink test message";
    // log::print はグローバルの LoggingConfig へ sink を登録するため、
    // initialize_logging の競合を避けるべくサブプロセスとして実行する。
    let output = std::process::Command::new(&exe)
        .arg("logging_sink_helper")
        .env("WEBRTC_LOG_SINK_EXPECT", expected)
        .output()
        .expect("サブプロセスの実行に失敗しました");
    assert!(
        output.status.success(),
        "sink がメッセージと重大度を受け取れていません\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn logging_sink_helper() {
    // 検証用ヘルパー。logging_sink_receives_log_line_ref からサブプロセスとして
    // 実行され、sink がメッセージと重大度を受け取れることを検証する。
    struct TestSinkHandler {
        expected: String,
        seen: Arc<Mutex<bool>>,
    }

    impl log::LogSinkHandler for TestSinkHandler {
        fn on_log_message(&mut self, line: log::LogLineRef<'_>) {
            if line.message().contains(&self.expected) && line.severity() == log::Severity::Info {
                *self.seen.lock().unwrap() = true;
            }
        }
    }

    let expected = std::env::var("WEBRTC_LOG_SINK_EXPECT").unwrap_or_default();
    // このヘルパーは logging_sink_receives_log_line_ref からのみサブプロセスと
    // して実行される。テスト本体として直接実行された場合（グローバルの
    // LoggingConfig が別のテストで既に初期化済みで、検証が意味を持たない場合）
    // は何もせず成功とする。
    if expected.is_empty() {
        return;
    }
    let seen = Arc::new(Mutex::new(false));
    let mut config = log::LoggingConfig::new();
    config.set_min_severity(log::Severity::Info);
    config.set_debug_severity(log::Severity::Info);
    config.add_sink(log::LogSink::new_with_handler(Box::new(TestSinkHandler {
        expected: expected.clone(),
        seen: seen.clone(),
    })));
    if !log::initialize_logging(config) {
        panic!("logging_sink_helper: initialize_logging が失敗しました");
    }
    log::print(log::Severity::Info, "webrtc-c", 0, &expected);
    assert!(
        *seen.lock().unwrap(),
        "sink がメッセージを受け取っていません"
    );
}

#[test]
fn thread_blocking_call_runs() {
    let mut thread = Thread::new();
    assert!(thread.start());
    let result = thread.blocking_call(|| 42);
    assert_eq!(result, 42);

    // () 戻り値のパスも通す
    thread.blocking_call(|| {});
    thread.stop();
}

#[test]
fn thread_start_returns_true() {
    // スレッド起動が成功すれば true を返すことを検証する。
    // false になる再現経路は libwebrtc 実装依存のため、正常系のみ確認する。
    let mut thread = Thread::new();
    assert!(thread.start());
    thread.stop();
}

#[test]
fn thread_quit_runs() {
    // quit 後も stop が例外なく実行できることを確認する。
    // libwebrtc の Stop は Quit + Join のため、quit 済みでも安全に実行できる。
    let mut thread = Thread::new();
    assert!(thread.start());
    thread.quit();
    thread.stop();
}

#[test]
fn thread_blocking_call_after_quit_does_not_run() {
    // quit 後はメッセージループが停止し、() を返す blocking_call は
    // クロージャを実行せずに即座に戻ることを確認する。
    let mut thread = Thread::new();
    assert!(thread.start());
    thread.quit();

    let called = Arc::new(AtomicBool::new(false));
    let called_clone = Arc::clone(&called);
    // 必ず quit の後に blocking_call を呼び、quit との競合を避ける。
    thread.blocking_call(move || {
        called_clone.store(true, Ordering::SeqCst);
    });
    assert!(
        !called.load(Ordering::SeqCst),
        "quit 後にブロックしたコールバックが実行されました"
    );
    thread.stop();
}

#[test]
fn thread_blocking_call_after_stop_returns_default() {
    // stop 後はメッセージループが停止し、非 void の blocking_call は
    // クロージャを実行せずに R::default() (0) を返すことを確認する。
    // 未実行時に未初期化ポインタが渡り Box::from_raw で UB になる問題の回帰テスト。
    let mut thread = Thread::new();
    assert!(thread.start());
    thread.stop();
    let result = thread.blocking_call(|| 42);
    assert_eq!(result, 0);
}

#[test]
fn thread_sleep_ms_runs() {
    assert!(Thread::sleep_ms(1));
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
    let src = AdaptedVideoTrackSource::new();
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
    let (factory, context) = PeerConnectionFactory::create_modular_with_context(deps)
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

    let (factory, context) = PeerConnectionFactory::create_modular_with_context(deps)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");
    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());
    assert!(!factory.as_ptr().is_null());

    drop(context);
    drop(factory);
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
    // clock_rate / num_channels を None に戻せば getter が None に戻ることを検証する
    codec.set_clock_rate(None);
    codec.set_num_channels(None);
    assert_eq!(codec.clock_rate(), None);
    assert_eq!(codec.num_channels(), None);
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
    params.set_degradation_preference(Some(DegradationPreference::MaintainFramerateAndResolution));
    assert_eq!(
        params.degradation_preference(),
        Some(DegradationPreference::MaintainFramerateAndResolution)
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let source = AdaptedVideoTrackSource::new();
    let vts = source.cast_to_video_track_source();
    let track = factory
        .create_video_track(&vts, "video-track-1")
        .expect("VideoTrack の生成に失敗しました");

    let pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &pc_config, pc_deps)
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
    drop(factory);
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    // PC 用の構成と observer/dependencies を準備する。
    let pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let pc_deps = PeerConnectionDependencies::new(&observer);

    // PeerConnection を生成し、取得できることを確認する。
    let pc = PeerConnection::create(&factory, &pc_config, pc_deps)
        .expect("PeerConnection の生成に失敗しました");
    assert!(!pc.as_ptr().is_null());

    drop(pc);
    drop(factory);
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &pc_config, pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let mut transceiver_init = RtpTransceiverInit::new();
    transceiver_init.set_direction(RtpTransceiverDirection::SendRecv);
    let _ = pc
        .add_transceiver(MediaType::Audio, &transceiver_init)
        .expect("transceiver の追加に失敗しました");

    if let Some(dtls_transport) = pc.lookup_dtls_transport_by_mid("0") {
        let observer = DtlsTransportObserver::new_with_handler(Box::new(NoopHandler));
        let _ = dtls_transport.state();
        dtls_transport.register_observer(&observer);
        dtls_transport.unregister_observer();
    }

    drop(pc);
    drop(factory);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn get_stats_delivers_report() {
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &pc_config, pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let (tx, rx) = mpsc::channel::<()>();
    pc.get_stats(move |_report| {
        let _ = tx.send(());
    });

    // 配信はシグナリングスレッドで非同期に行われるため、コールバック発火を待つ。
    rx.recv_timeout(Duration::from_secs(10))
        .expect("get_stats のコールバックが呼ばれませんでした");

    drop(pc);
    drop(factory);
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
    let (factory, context) = PeerConnectionFactory::create_modular_with_context(deps_factory)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");

    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());

    let pc_config = PeerConnectionRtcConfiguration::new();
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
    let pc = PeerConnection::create(&factory, &pc_config, pc_deps)
        .expect("Proxy 設定付き PeerConnection の生成に失敗しました");
    assert!(!pc.as_ptr().is_null());

    drop(pc);
    drop(context);
    drop(factory);
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    // VideoTrack を生成する。
    let source = AdaptedVideoTrackSource::new();
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
    let pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
    let pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &pc_config, pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let mut init = RtpTransceiverInit::new();
    init.set_direction(RtpTransceiverDirection::SendOnly);
    pc.add_transceiver_with_track(&track, &init)
        .expect("AddTransceiverWithTrack が失敗しました");

    // webrtc オブジェクトを先に解放してからスレッドを停止する。
    drop(pc);
    drop(track);
    drop(vts);
    drop(source);
    drop(factory);
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

#[test]
fn always_negotiate_data_channels_adds_data_section() {
    struct OfferHandler {
        tx: mpsc::Sender<Result<String>>,
    }

    impl CreateSessionDescriptionObserverHandler for OfferHandler {
        fn on_success(&mut self, desc: SessionDescription) {
            let sdp = desc.to_string();
            let _ = self.tx.send(sdp);
        }

        fn on_failure(&mut self, err: RtcError) {
            let _ = self.tx.send(Err(err.into()));
        }
    }

    fn create_offer_sdp(
        factory: &PeerConnectionFactory,
        config: &PeerConnectionRtcConfiguration,
    ) -> String {
        let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopHandler));
        let pc_deps = PeerConnectionDependencies::new(&observer);
        let pc = PeerConnection::create(factory, config, pc_deps)
            .expect("PeerConnection の生成に失敗しました");

        let opts = PeerConnectionOfferAnswerOptions::new();
        let (tx, rx) = mpsc::channel::<Result<String>>();
        let mut obs =
            CreateSessionDescriptionObserver::new_with_handler(Box::new(OfferHandler { tx }));
        pc.create_offer(&mut obs, &opts);
        let sdp = rx
            .recv_timeout(Duration::from_secs(5))
            .expect("createOffer がタイムアウトしました")
            .expect("createOffer が失敗しました");

        drop(obs);
        drop(pc);
        sdp
    }

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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    // always_negotiate_data_channels=true かつ DataChannel 未生成でも m=application が含まれる。
    let mut pc_config_on = PeerConnectionRtcConfiguration::new();
    pc_config_on.set_always_negotiate_data_channels(true);
    let sdp_on = create_offer_sdp(&factory, &pc_config_on);
    assert!(
        sdp_on.contains("m=application"),
        "always_negotiate_data_channels=true で SDP に m=application が含まれません: {sdp_on}"
    );

    // 対照実験: デフォルト (false) で DataChannel 未生成なら m=application は含まれない。
    let pc_config_off = PeerConnectionRtcConfiguration::new();
    let sdp_off = create_offer_sdp(&factory, &pc_config_off);
    assert!(
        !sdp_off.contains("m=application"),
        "always_negotiate_data_channels=false で SDP に m=application が含まれました: {sdp_off}"
    );

    drop(factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
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
        "webrtc_objc_RTCDefaultVideoEncoderFactory_new が null を返しました"
    );

    let native_unique = unsafe { ffi::webrtc_ObjCToNativeVideoEncoderFactory(objc_factory) };
    assert!(
        !native_unique.is_null(),
        "webrtc_ObjCToNativeVideoEncoderFactory が null を返しました"
    );

    let native = unsafe { ffi::webrtc_VideoEncoderFactory_unique_get(native_unique) };
    assert!(
        !native.is_null(),
        "webrtc_VideoEncoderFactory_unique_get が null を返しました"
    );

    let formats = unsafe { ffi::webrtc_VideoEncoderFactory_GetSupportedFormats(native) };
    assert!(
        !formats.is_null(),
        "webrtc_VideoEncoderFactory_GetSupportedFormats が null を返しました"
    );
    let size = unsafe { ffi::webrtc_SdpVideoFormat_vector_size(formats) };
    assert!(size >= 0, "フォーマット数が不正です: {size}");

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
        "webrtc_objc_RTCDefaultVideoDecoderFactory_new が null を返しました"
    );

    let native_unique = unsafe { ffi::webrtc_ObjCToNativeVideoDecoderFactory(objc_factory) };
    assert!(
        !native_unique.is_null(),
        "webrtc_ObjCToNativeVideoDecoderFactory が null を返しました"
    );

    let native = unsafe { ffi::webrtc_VideoDecoderFactory_unique_get(native_unique) };
    assert!(
        !native.is_null(),
        "webrtc_VideoDecoderFactory_unique_get が null を返しました"
    );

    let formats = unsafe { ffi::webrtc_VideoDecoderFactory_GetSupportedFormats(native) };
    assert!(
        !formats.is_null(),
        "webrtc_VideoDecoderFactory_GetSupportedFormats が null を返しました"
    );
    let size = unsafe { ffi::webrtc_SdpVideoFormat_vector_size(formats) };
    assert!(size >= 0, "フォーマット数が不正です: {size}");

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

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn objc_video_encoder_factory_release_no_retain_leak() {
    let objc_factory = unsafe { ffi::webrtc_objc_RTCDefaultVideoEncoderFactory_new() };
    assert!(
        !objc_factory.is_null(),
        "webrtc_objc_RTCDefaultVideoEncoderFactory_new が null を返しました"
    );

    // new が +1 のみを返していれば参照カウントは 1、二重リテインがあれば 2 になる
    let retain_count = unsafe { ffi::objc_NSObject_retainCount(objc_factory.cast()) };
    assert_eq!(
        retain_count, 1,
        "エンコーダーファクトリのリテインリークを検出しました: retain_count={}",
        retain_count
    );

    unsafe { ffi::webrtc_objc_RTCVideoEncoderFactory_release(objc_factory) };
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn objc_video_decoder_factory_release_no_retain_leak() {
    let objc_factory = unsafe { ffi::webrtc_objc_RTCDefaultVideoDecoderFactory_new() };
    assert!(
        !objc_factory.is_null(),
        "webrtc_objc_RTCDefaultVideoDecoderFactory_new が null を返しました"
    );

    // new が +1 のみを返していれば参照カウントは 1、二重リテインがあれば 2 になる
    let retain_count = unsafe { ffi::objc_NSObject_retainCount(objc_factory.cast()) };
    assert_eq!(
        retain_count, 1,
        "デコーダーファクトリのリテインリークを検出しました: retain_count={}",
        retain_count
    );

    unsafe { ffi::webrtc_objc_RTCVideoDecoderFactory_release(objc_factory) };
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[test]
fn objc_video_factory_functions_return_null_on_non_apple() {
    let enc_objc = unsafe { ffi::webrtc_objc_RTCDefaultVideoEncoderFactory_new() };
    assert!(
        enc_objc.is_null(),
        "非 Apple プラットフォームでは encoder objc factory が null になること"
    );
    let enc_native = unsafe {
        ffi::webrtc_ObjCToNativeVideoEncoderFactory(std::ptr::null_mut::<
            ffi::webrtc_objc_RTCVideoEncoderFactory,
        >())
    };
    assert!(
        enc_native.is_null(),
        "非 Apple プラットフォームでは encoder native factory が null になること"
    );
    unsafe {
        ffi::webrtc_objc_RTCVideoEncoderFactory_release(std::ptr::null_mut::<
            ffi::webrtc_objc_RTCVideoEncoderFactory,
        >())
    };

    let dec_objc = unsafe { ffi::webrtc_objc_RTCDefaultVideoDecoderFactory_new() };
    assert!(
        dec_objc.is_null(),
        "非 Apple プラットフォームでは decoder objc factory が null になること"
    );
    let dec_native = unsafe {
        ffi::webrtc_ObjCToNativeVideoDecoderFactory(std::ptr::null_mut::<
            ffi::webrtc_objc_RTCVideoDecoderFactory,
        >())
    };
    assert!(
        dec_native.is_null(),
        "非 Apple プラットフォームでは decoder native factory が null になること"
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let stream = factory
        .create_local_media_stream("stream-1")
        .expect("CreateLocalMediaStream が失敗しました");
    let audio_options = AudioOptions::new();
    let audio_source = factory
        .create_audio_source(&audio_options)
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
    drop(audio_options);
    drop(stream);
    drop(factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn audio_options_set_and_get_options() {
    // 未設定の AudioOptions はすべての getter が None を返すことを検証する
    let options = AudioOptions::new();
    assert_eq!(options.echo_cancellation(), None);
    assert_eq!(options.auto_gain_control(), None);
    assert_eq!(options.noise_suppression(), None);
    assert_eq!(options.highpass_filter(), None);
    assert_eq!(options.stereo_swapping(), None);
    assert_eq!(options.audio_jitter_buffer_max_packets(), None);
    assert_eq!(options.audio_jitter_buffer_fast_accelerate(), None);
    assert_eq!(options.audio_jitter_buffer_min_delay_ms(), None);
    drop(options);

    // 全フィールドに設定した値が getter で取得できることを検証する
    let mut options = AudioOptions::new();
    options.set_echo_cancellation(Some(false));
    options.set_auto_gain_control(Some(true));
    options.set_noise_suppression(Some(false));
    options.set_highpass_filter(Some(true));
    options.set_stereo_swapping(Some(false));
    options.set_audio_jitter_buffer_max_packets(Some(50));
    options.set_audio_jitter_buffer_fast_accelerate(Some(true));
    options.set_audio_jitter_buffer_min_delay_ms(Some(100));
    assert_eq!(options.echo_cancellation(), Some(false));
    assert_eq!(options.auto_gain_control(), Some(true));
    assert_eq!(options.noise_suppression(), Some(false));
    assert_eq!(options.highpass_filter(), Some(true));
    assert_eq!(options.stereo_swapping(), Some(false));
    assert_eq!(options.audio_jitter_buffer_max_packets(), Some(50));
    assert_eq!(options.audio_jitter_buffer_fast_accelerate(), Some(true));
    assert_eq!(options.audio_jitter_buffer_min_delay_ms(), Some(100));

    // 未設定 (None) に戻せば getter が None に戻ることを検証する
    options.set_echo_cancellation(None);
    options.set_audio_jitter_buffer_max_packets(None);
    assert_eq!(options.echo_cancellation(), None);
    assert_eq!(options.audio_jitter_buffer_max_packets(), None);
    drop(options);
}

#[test]
fn create_audio_source_with_audio_options() {
    // 設定付きの AudioOptions を渡して AudioSource を生成できることを検証する
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let mut options = AudioOptions::new();
    options.set_echo_cancellation(Some(false));
    options.set_auto_gain_control(Some(false));
    options.set_noise_suppression(Some(false));
    options.set_highpass_filter(Some(false));
    let audio_source = factory
        .create_audio_source(&options)
        .expect("AudioSource の生成に失敗しました");

    drop(audio_source);
    drop(options);
    drop(factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn create_audio_source_with_default_audio_options() {
    // 何も設定しない AudioOptions を渡しても、従来と同じように AudioSource を生成できることを検証する
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
    let factory = PeerConnectionFactory::create_modular(deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let options = AudioOptions::new();
    let audio_source = factory
        .create_audio_source(&options)
        .expect("AudioSource の生成に失敗しました");

    drop(audio_source);
    drop(options);
    drop(factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn frame_transformer_create_and_drop() {
    struct TransformHandler;

    impl FrameTransformerHandler for TransformHandler {
        fn transform(&self, frame: TransformableFrame) -> Option<TransformableFrame> {
            Some(frame)
        }
    }

    let transformer = FrameTransformer::new_with_handler(Box::new(TransformHandler));
    drop(transformer);
}

#[test]
fn video_frame_metadata_full_roundtrip() {
    let mut metadata = VideoFrameMetadata::new();

    metadata.set_rotation(VideoRotation::R90);
    assert_eq!(metadata.rotation(), VideoRotation::R90);
    metadata.set_content_type(VideoContentType::Unspecified);
    assert_eq!(metadata.content_type(), VideoContentType::Unspecified);
    metadata.set_content_type(VideoContentType::Screenshare);
    assert_eq!(metadata.content_type(), VideoContentType::Screenshare);
    metadata.set_csrcs(&[1, 2, 3]);
    assert_eq!(metadata.csrcs(), vec![1, 2, 3]);
    metadata.set_decode_target_indications(&[
        DecodeTargetIndication::NotPresent,
        DecodeTargetIndication::Discardable,
        DecodeTargetIndication::Switch,
        DecodeTargetIndication::Required,
    ]);
    assert_eq!(
        metadata.decode_target_indications(),
        vec![
            DecodeTargetIndication::NotPresent,
            DecodeTargetIndication::Discardable,
            DecodeTargetIndication::Switch,
            DecodeTargetIndication::Required,
        ]
    );

    let mut vp8 = RTPVideoHeaderVP8::new();
    vp8.set_picture_id(42);
    vp8.set_temporal_idx(1);
    metadata.set_rtp_video_header_codec_specifics(RTPVideoHeaderCodecSpecifics::VP8(vp8));
    match metadata.rtp_video_header_codec_specifics() {
        RTPVideoHeaderCodecSpecifics::VP8(v) => {
            assert_eq!(v.picture_id(), 42);
            assert_eq!(v.temporal_idx(), 1);
        }
        _ => panic!("RTPVideoHeaderCodecSpecifics::VP8 が返る想定でした"),
    }

    let mut vp9 = RTPVideoHeaderVP9::new();
    vp9.set_spatial_idx(2);
    vp9.set_num_spatial_layers(3);
    metadata.set_rtp_video_header_codec_specifics(RTPVideoHeaderCodecSpecifics::VP9(vp9));
    match metadata.rtp_video_header_codec_specifics() {
        RTPVideoHeaderCodecSpecifics::VP9(v) => {
            assert_eq!(v.spatial_idx(), 2);
            assert_eq!(v.num_spatial_layers(), 3);
        }
        _ => panic!("RTPVideoHeaderCodecSpecifics::VP9 が返る想定でした"),
    }

    let mut h264 = RTPVideoHeaderH264::new();
    h264.set_nalu_type(5);
    h264.set_packetization_type(H264PacketizationType::StapA);
    h264.set_packetization_mode(H264PacketizationMode::NonInterleaved);
    metadata.set_rtp_video_header_codec_specifics(RTPVideoHeaderCodecSpecifics::H264(h264));
    match metadata.rtp_video_header_codec_specifics() {
        RTPVideoHeaderCodecSpecifics::H264(v) => {
            assert_eq!(v.nalu_type(), 5);
            assert_eq!(v.packetization_type(), H264PacketizationType::StapA);
            assert_eq!(
                v.packetization_mode(),
                H264PacketizationMode::NonInterleaved
            );
        }
        _ => panic!("RTPVideoHeaderCodecSpecifics::H264 が返る想定でした"),
    }

    metadata.set_rtp_video_header_codec_specifics(RTPVideoHeaderCodecSpecifics::None);
    assert!(matches!(
        metadata.rtp_video_header_codec_specifics(),
        RTPVideoHeaderCodecSpecifics::None
    ));

    // Clone がディープコピーされ、元の値と一致することを確認する。
    metadata.set_width(640);
    let cloned = metadata.clone();
    assert_eq!(cloned.frame_type(), metadata.frame_type());
    assert_eq!(cloned.width(), metadata.width());
    assert_eq!(cloned.height(), metadata.height());
    assert_eq!(cloned.frame_id(), metadata.frame_id());
    assert_eq!(cloned.spatial_index(), metadata.spatial_index());
    assert_eq!(cloned.temporal_index(), metadata.temporal_index());
    assert_eq!(cloned.dependencies(), metadata.dependencies());
    assert_eq!(
        cloned.is_last_frame_in_picture(),
        metadata.is_last_frame_in_picture()
    );
    assert_eq!(cloned.simulcast_idx(), metadata.simulcast_idx());
    assert_eq!(cloned.codec(), metadata.codec());
    assert_eq!(cloned.ssrc(), metadata.ssrc());
    assert_eq!(cloned.rotation(), metadata.rotation());
    assert_eq!(cloned.content_type(), metadata.content_type());
    assert_eq!(
        cloned.decode_target_indications(),
        metadata.decode_target_indications()
    );
    assert_eq!(cloned.csrcs(), metadata.csrcs());
    // クローン後の書き換えが元に影響しないことを確認する。
    let mut cloned = cloned;
    cloned.set_width(999);
    assert_eq!(metadata.width(), 640);
    assert_eq!(cloned.width(), 999);
}

#[test]
fn video_frame_metadata_basic_fields_roundtrip() {
    let mut metadata = VideoFrameMetadata::new();

    metadata.set_frame_type(VideoFrameType::Key);
    assert_eq!(metadata.frame_type(), VideoFrameType::Key);
    metadata.set_width(640);
    metadata.set_height(480);
    assert_eq!(metadata.width(), 640);
    assert_eq!(metadata.height(), 480);
    metadata.set_frame_id(Some(42));
    assert_eq!(metadata.frame_id(), Some(42));
    metadata.set_frame_id(None);
    assert_eq!(metadata.frame_id(), None);
    metadata.set_spatial_index(1);
    metadata.set_temporal_index(2);
    assert_eq!(metadata.spatial_index(), 1);
    assert_eq!(metadata.temporal_index(), 2);
    metadata.set_dependencies(Some(&[10, 20, 30]));
    assert_eq!(metadata.dependencies(), Some(&[10, 20, 30][..]));
    metadata.set_dependencies(None);
    assert_eq!(metadata.dependencies(), None);
    metadata.set_is_last_frame_in_picture(true);
    assert!(metadata.is_last_frame_in_picture());
    metadata.set_simulcast_idx(3);
    assert_eq!(metadata.simulcast_idx(), 3);
    metadata.set_codec(VideoCodecType::Vp8);
    assert_eq!(metadata.codec(), VideoCodecType::Vp8);
    metadata.set_ssrc(12345);
    assert_eq!(metadata.ssrc(), 12345);
}

#[test]
fn rtp_video_header_vp8_full_roundtrip() {
    let mut header = RTPVideoHeaderVP8::new();
    header.set_non_reference(true);
    header.set_picture_id(1234);
    header.set_tl0_pic_idx(5);
    header.set_temporal_idx(2);
    header.set_layer_sync(true);
    header.set_key_idx(3);
    header.set_partition_id(7);
    header.set_beginning_of_partition(true);
    assert!(header.non_reference());
    assert_eq!(header.picture_id(), 1234);
    assert_eq!(header.tl0_pic_idx(), 5);
    assert_eq!(header.temporal_idx(), 2);
    assert!(header.layer_sync());
    assert_eq!(header.key_idx(), 3);
    assert_eq!(header.partition_id(), 7);
    assert!(header.beginning_of_partition());

    // Clone がディープコピーされ、元の値と一致することを確認する。
    let cloned = header.clone();
    assert_eq!(cloned.picture_id(), header.picture_id());
    assert_eq!(cloned.temporal_idx(), header.temporal_idx());
    assert_eq!(cloned.key_idx(), header.key_idx());
    assert_eq!(
        cloned.beginning_of_partition(),
        header.beginning_of_partition()
    );
}

#[test]
fn rtp_video_header_vp9_full_roundtrip() {
    let mut header = RTPVideoHeaderVP9::new();
    header.set_inter_pic_predicted(true);
    header.set_flexible_mode(true);
    header.set_beginning_of_frame(true);
    header.set_end_of_frame(true);
    header.set_ss_data_available(true);
    header.set_non_ref_for_inter_layer_pred(true);
    header.set_picture_id(4321);
    header.set_max_picture_id(8191);
    header.set_tl0_pic_idx(9);
    header.set_temporal_idx(3);
    header.set_spatial_idx(1);
    header.set_temporal_up_switch(true);
    header.set_inter_layer_predicted(true);
    header.set_gof_idx(2);
    header.set_num_ref_pics(2);
    header.set_pid_diff(0, 1);
    header.set_pid_diff(1, 2);
    header.set_ref_picture_id(0, 100);
    header.set_ref_picture_id(1, 200);
    header.set_num_spatial_layers(3);
    header.set_first_active_layer(1);
    header.set_spatial_layer_resolution_present(true);
    header.set_width(0, 640);
    header.set_width(1, 1280);
    header.set_height(0, 480);
    header.set_height(1, 720);
    header.set_end_of_picture(true);

    let mut gof = GofInfoVP9::new();
    gof.set_num_frames_in_gof(2);
    gof.set_temporal_idx(0, 0);
    gof.set_temporal_idx(1, 1);
    gof.set_temporal_up_switch(0, true);
    gof.set_num_ref_pics(0, 1);
    gof.set_pid_diff(0, 0, 2);
    gof.set_pid_start(10);
    header.set_gof(&gof);

    assert!(header.inter_pic_predicted());
    assert!(header.flexible_mode());
    assert!(header.beginning_of_frame());
    assert!(header.end_of_frame());
    assert!(header.ss_data_available());
    assert!(header.non_ref_for_inter_layer_pred());
    assert_eq!(header.picture_id(), 4321);
    assert_eq!(header.max_picture_id(), 8191);
    assert_eq!(header.tl0_pic_idx(), 9);
    assert_eq!(header.temporal_idx(), 3);
    assert_eq!(header.spatial_idx(), 1);
    assert!(header.temporal_up_switch());
    assert!(header.inter_layer_predicted());
    assert_eq!(header.gof_idx(), 2);
    assert_eq!(header.num_ref_pics(), 2);
    assert_eq!(header.pid_diff(0), Some(1));
    assert_eq!(header.pid_diff(1), Some(2));
    assert_eq!(header.ref_picture_id(0), Some(100));
    assert_eq!(header.ref_picture_id(1), Some(200));
    assert_eq!(header.num_spatial_layers(), 3);
    assert_eq!(header.first_active_layer(), 1);
    assert!(header.spatial_layer_resolution_present());
    assert_eq!(header.width(0), Some(640));
    assert_eq!(header.width(1), Some(1280));
    assert_eq!(header.height(0), Some(480));
    assert_eq!(header.height(1), Some(720));
    assert!(header.end_of_picture());

    let got_gof = header.gof();
    assert_eq!(got_gof.num_frames_in_gof(), 2);
    assert_eq!(got_gof.temporal_idx(0), Some(0));
    assert_eq!(got_gof.temporal_idx(1), Some(1));
    assert_eq!(got_gof.temporal_up_switch(0), Some(true));
    assert_eq!(got_gof.num_ref_pics(0), Some(1));
    assert_eq!(got_gof.pid_diff(0, 0), Some(2));
    assert_eq!(got_gof.pid_start(), 10);

    // 境界を超える index へのアクセスは None を返すことを確認する。
    assert_eq!(header.pid_diff(usize::MAX), None);
    assert_eq!(header.ref_picture_id(usize::MAX), None);
    assert_eq!(header.width(usize::MAX), None);
    assert_eq!(header.height(usize::MAX), None);

    // 配列サイズの公開メソッドが libwebrtc の定数と一致することを確認する。
    assert_eq!(constants::max_vp9_ref_pics(), 3);
    assert_eq!(constants::max_vp9_num_spatial_layers(), 8);
}

#[test]
fn gof_info_vp9_full_roundtrip() {
    let mut gof = GofInfoVP9::new();
    gof.set_num_frames_in_gof(3);
    gof.set_temporal_idx(0, 0);
    gof.set_temporal_idx(1, 2);
    gof.set_temporal_idx(2, 1);
    gof.set_temporal_up_switch(0, true);
    gof.set_temporal_up_switch(1, false);
    gof.set_num_ref_pics(0, 2);
    gof.set_pid_diff(0, 0, 4);
    gof.set_pid_diff(0, 1, 1);
    gof.set_pid_start(100);
    assert_eq!(gof.num_frames_in_gof(), 3);
    assert_eq!(gof.temporal_idx(0), Some(0));
    assert_eq!(gof.temporal_idx(1), Some(2));
    assert_eq!(gof.temporal_idx(2), Some(1));
    assert_eq!(gof.temporal_up_switch(0), Some(true));
    assert_eq!(gof.temporal_up_switch(1), Some(false));
    assert_eq!(gof.num_ref_pics(0), Some(2));
    assert_eq!(gof.pid_diff(0, 0), Some(4));
    assert_eq!(gof.pid_diff(0, 1), Some(1));
    assert_eq!(gof.pid_start(), 100);

    // 境界を超える index へのアクセスは None を返すことを確認する。
    assert_eq!(gof.temporal_idx(usize::MAX), None);
    assert_eq!(gof.temporal_up_switch(usize::MAX), None);
    assert_eq!(gof.num_ref_pics(usize::MAX), None);
    assert_eq!(gof.pid_diff(usize::MAX, 0), None);
    assert_eq!(gof.pid_diff(0, usize::MAX), None);

    // 配列サイズの公開メソッドが libwebrtc の定数と一致することを確認する。
    assert_eq!(constants::max_vp9_frames_in_gof(), 0xFF);
    assert_eq!(constants::max_vp9_ref_pics(), 3);

    // Clone がディープコピーされ、元の値と一致することを確認する。
    let cloned = gof.clone();
    assert_eq!(cloned.num_frames_in_gof(), gof.num_frames_in_gof());
    assert_eq!(cloned.temporal_idx(0), gof.temporal_idx(0));
    assert_eq!(cloned.temporal_up_switch(0), gof.temporal_up_switch(0));
    assert_eq!(cloned.pid_diff(0, 0), gof.pid_diff(0, 0));
    assert_eq!(cloned.pid_start(), gof.pid_start());
}

#[test]
#[should_panic(expected = "MAX_FRAMES_IN_GOF")]
fn gof_info_vp9_set_num_frames_in_gof_overflow() {
    let mut gof = GofInfoVP9::new();
    gof.set_num_frames_in_gof(300);
}

#[test]
#[should_panic(expected = "MAX_FRAMES_IN_GOF")]
fn gof_info_vp9_set_temporal_idx_out_of_bounds() {
    let mut gof = GofInfoVP9::new();
    gof.set_temporal_idx(255, 1);
}

#[test]
fn nalu_info_vector_roundtrip() {
    let mut nalu = NaluInfo::new();
    nalu.set_type(7);
    nalu.set_sps_id(3);
    nalu.set_pps_id(4);
    assert_eq!(nalu.type_(), 7);
    assert_eq!(nalu.sps_id(), 3);
    assert_eq!(nalu.pps_id(), 4);

    let mut vec = NaluInfoVector::new(0);
    assert!(vec.is_empty());
    vec.push(&nalu);
    assert_eq!(vec.len(), 1);
    let elem = vec.get(0).expect("要素が存在する想定");
    assert_eq!(elem.type_(), 7);
    assert_eq!(elem.sps_id(), 3);
    assert_eq!(elem.pps_id(), 4);
    assert!(vec.get(1).is_none());
}

#[test]
fn rtp_video_header_h264_full_roundtrip() {
    let mut header = RTPVideoHeaderH264::new();
    header.set_nalu_type(5);
    header.set_packetization_type(H264PacketizationType::FuA);
    header.set_packetization_mode(H264PacketizationMode::SingleNalUnit);

    let mut nalu1 = NaluInfo::new();
    nalu1.set_type(5);
    nalu1.set_sps_id(1);
    nalu1.set_pps_id(2);
    let mut nalu2 = NaluInfo::new();
    nalu2.set_type(1);
    nalu2.set_sps_id(-1);
    nalu2.set_pps_id(-1);
    let mut nalus = NaluInfoVector::new(0);
    nalus.push(&nalu1);
    nalus.push(&nalu2);
    header.set_nalus(&nalus);

    assert_eq!(header.nalu_type(), 5);
    assert_eq!(header.packetization_type(), H264PacketizationType::FuA);
    assert_eq!(
        header.packetization_mode(),
        H264PacketizationMode::SingleNalUnit
    );
    let got = header.nalus();
    assert_eq!(got.len(), 2);
    let e1 = got.get(0).expect("要素が存在する想定");
    assert_eq!(e1.type_(), 5);
    assert_eq!(e1.sps_id(), 1);
    assert_eq!(e1.pps_id(), 2);
    let e2 = got.get(1).expect("要素が存在する想定");
    assert_eq!(e2.type_(), 1);
    assert_eq!(e2.sps_id(), -1);
    assert_eq!(e2.pps_id(), -1);

    // Clone がディープコピーされ、元の値と一致することを確認する。
    let cloned = header.clone();
    assert_eq!(cloned.nalu_type(), header.nalu_type());
    assert_eq!(cloned.packetization_type(), header.packetization_type());
    assert_eq!(cloned.packetization_mode(), header.packetization_mode());
    let cloned_nalus = cloned.nalus();
    assert_eq!(cloned_nalus.len(), 2);
    let e1 = cloned_nalus.get(0).expect("要素が存在する想定");
    assert_eq!(e1.sps_id(), 1);
    let e2 = cloned_nalus.get(1).expect("要素が存在する想定");
    assert_eq!(e2.sps_id(), -1);
}

#[test]
fn audio_codec_type_raw_round_trip() {
    let cases = [
        (AudioCodecType::Other, None),
        (AudioCodecType::Opus, Some("opus")),
        (AudioCodecType::Isac, Some("ISAC")),
        (AudioCodecType::G722, Some("G722")),
        (AudioCodecType::PcmA, Some("PCMA")),
        (AudioCodecType::PcmU, Some("PCMU")),
    ];
    for (codec_type, name) in cases {
        assert_eq!(
            codec_type.to_raw(),
            AudioCodecType::from_raw(codec_type.to_raw()).to_raw()
        );
        assert_eq!(AudioCodecType::from_raw(codec_type.to_raw()), codec_type);
        assert_eq!(codec_type.as_str(), name);
    }
    let unknown = AudioCodecType::from_raw(999);
    assert_eq!(unknown, AudioCodecType::Unknown(999));
    assert_eq!(unknown.to_raw(), 999);
    assert_eq!(unknown.as_str(), None);
    // kOther は 0、kG722 は 5 (kMaxLoggedAudioCodecTypes=6 未満) であることを確認する。
    assert_eq!(AudioCodecType::Other.to_raw(), 0);
    assert_eq!(AudioCodecType::G722.to_raw(), 5);
}

#[test]
fn audio_codec_type_try_from_invalid() {
    let error =
        AudioCodecType::try_from("bogus").expect_err("未知のコーデック名はエラーになる想定です");
    assert!(matches!(error, Error::InvalidAudioCodecType(_)));
}

#[test]
fn audio_encoder_encoded_info_encoder_type_validation() {
    let mut info = AudioEncoderEncodedInfo::new();
    for codec_type in [
        AudioCodecType::Other,
        AudioCodecType::Opus,
        AudioCodecType::Isac,
        AudioCodecType::G722,
        AudioCodecType::PcmA,
        AudioCodecType::PcmU,
    ] {
        info.set_encoder_type(codec_type);
        assert_eq!(info.encoder_type(), codec_type);
    }
    // 範囲外 (6以上) は libwebrtc 内部 OOB になるため panic する。
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        info.set_encoder_type(AudioCodecType::Unknown(6));
    }));
    assert!(
        result.is_err(),
        "encoder_type の範囲外指定は panic する想定です"
    );
}

#[test]
fn audio_encoder_encoded_info_redundant_round_trip() {
    let mut info = AudioEncoderEncodedInfo::new();
    assert!(info.redundant().is_empty());
    let mut leaf = AudioEncoderEncodedInfoLeaf::new();
    leaf.set_encoded_bytes(10);
    leaf.set_payload_type(96);
    leaf.set_speech(true);
    info.set_redundant(vec![leaf]);
    let redundant = info.redundant();
    assert_eq!(redundant.len(), 1);
    assert_eq!(redundant[0].encoded_bytes(), 10);
    assert_eq!(redundant[0].payload_type(), 96);
    assert!(redundant[0].speech());
}

struct TestAudioEncoderHandler {
    pub(crate) encoded: bool,
}

impl AudioEncoderHandler for TestAudioEncoderHandler {
    fn sample_rate_hz(&mut self) -> i32 {
        48000
    }
    fn num_channels(&mut self) -> usize {
        2
    }
    fn num_10ms_frames_in_next_packet(&mut self) -> usize {
        1
    }
    fn max_10ms_frames_in_a_packet(&mut self) -> usize {
        4
    }
    fn get_target_bitrate(&mut self) -> i32 {
        32000
    }
    fn encode(
        &mut self,
        _rtp_timestamp: u32,
        _audio: &[i16],
        encoded: &mut BufferRef<'_>,
    ) -> AudioEncoderEncodedInfo {
        self.encoded = true;
        encoded.append_data(&[0x01, 0x02, 0x03]);
        let mut info = AudioEncoderEncodedInfo::new();
        info.set_encoded_bytes(encoded.size());
        info.set_payload_type(111);
        info
    }
    fn reset(&mut self) {}
    fn get_frame_length_range(&mut self) -> Option<(i64, i64)> {
        None
    }
}

struct TestAudioEncoderFactoryHandler {
    created: bool,
}

impl AudioEncoderFactoryHandler for TestAudioEncoderFactoryHandler {
    fn get_supported_encoders(&mut self) -> Vec<AudioCodecSpec> {
        vec![AudioCodecSpec::new(
            SdpAudioFormat::new("opus", 48000, 2),
            AudioCodecInfo::new(48000, 2, 32000, 6000, 510000),
        )]
    }
    fn create(
        &mut self,
        env: EnvironmentRef<'_>,
        format: SdpAudioFormatRef<'_>,
        _options: &AudioEncoderFactoryOptions,
    ) -> Option<AudioEncoder> {
        assert!(!env.as_ptr().is_null());
        assert_eq!(format.name().expect("名前の取得に失敗しました"), "opus");
        if self.created {
            return None;
        }
        self.created = true;
        Some(AudioEncoder::new_with_handler(Box::new(
            TestAudioEncoderHandler { encoded: false },
        )))
    }
}

#[test]
fn custom_audio_encoder_factory_roundtrip() {
    let factory = AudioEncoderFactory::new_with_handler(Box::new(TestAudioEncoderFactoryHandler {
        created: false,
    }));
    assert_eq!(factory.get_supported_encoders().len(), 1);
    let env = Environment::new();
    let format = SdpAudioFormat::new("opus", 48000, 2);
    let mut options = AudioEncoderFactoryOptions::new();
    options.set_payload_type(111);
    let mut encoder = factory
        .create(env.as_ref(), format.as_ref(), &options)
        .expect("カスタムエンコーダーの作成に失敗しました");
    assert_eq!(encoder.sample_rate_hz(), 48000);
    assert_eq!(encoder.num_channels(), 2);
    assert_eq!(encoder.num_10ms_frames_in_next_packet(), 1);
    assert_eq!(encoder.max_10ms_frames_in_a_packet(), 4);
    assert_eq!(encoder.get_target_bitrate(), 32000);
    // 既定実装の仕様を確認する (set_dtx は !enable、get_dtx は false)。
    assert!(!encoder.get_dtx());
    assert!(encoder.set_dtx(false));
    assert!(!encoder.set_application(0));
    let mut buffer = Buffer::new();
    let info = encoder.encode(0, &[0i16; 960], &mut buffer);
    assert_eq!(buffer.size(), 3);
    assert_eq!(info.payload_type(), 111);
    assert!(
        factory
            .create(env.as_ref(), format.as_ref(), &options)
            .is_none(),
        "2 回目の create は None を返す想定です"
    );
}

#[test]
fn custom_audio_encoder_no_output() {
    let mut encoder = AudioEncoder::new_with_handler(Box::new(TestAudioEncoderNoOutputHandler));
    let mut buffer = Buffer::new();
    let info = encoder.encode(0, &[0i16; 960], &mut buffer);
    assert_eq!(buffer.size(), 0);
    assert_eq!(info.encoded_bytes(), 0);
}

struct TestAudioEncoderNoOutputHandler;

impl AudioEncoderHandler for TestAudioEncoderNoOutputHandler {
    fn sample_rate_hz(&mut self) -> i32 {
        48000
    }
    fn num_channels(&mut self) -> usize {
        2
    }
    fn num_10ms_frames_in_next_packet(&mut self) -> usize {
        1
    }
    fn max_10ms_frames_in_a_packet(&mut self) -> usize {
        4
    }
    fn get_target_bitrate(&mut self) -> i32 {
        32000
    }
    fn encode(
        &mut self,
        _rtp_timestamp: u32,
        _audio: &[i16],
        _encoded: &mut BufferRef<'_>,
    ) -> AudioEncoderEncodedInfo {
        AudioEncoderEncodedInfo::new()
    }
    fn reset(&mut self) {}
    fn get_frame_length_range(&mut self) -> Option<(i64, i64)> {
        None
    }
}

struct TestAudioDecoderHandler;

impl AudioDecoderHandler for TestAudioDecoderHandler {
    fn sample_rate_hz(&mut self) -> i32 {
        48000
    }
    fn channels(&mut self) -> usize {
        2
    }
    fn decode(
        &mut self,
        _encoded: &[u8],
        _sample_rate_hz: i32,
        decoded: &mut RawBufferWriter<'_, i16>,
    ) -> (i32, AudioSpeechType) {
        decoded.write(&[0x1111i16; 160]);
        (160, AudioSpeechType::Speech)
    }
    fn reset(&mut self) {}
}

struct TestAudioDecoderFactoryHandler {
    created: bool,
}

impl AudioDecoderFactoryHandler for TestAudioDecoderFactoryHandler {
    fn get_supported_decoders(&mut self) -> Vec<AudioCodecSpec> {
        vec![AudioCodecSpec::new(
            SdpAudioFormat::new("opus", 48000, 2),
            AudioCodecInfo::new(48000, 2, 32000, 6000, 510000),
        )]
    }
    fn is_supported_decoder(&mut self, format: SdpAudioFormatRef<'_>) -> bool {
        format.name().map(|name| name == "opus").unwrap_or(false)
    }
    fn create(
        &mut self,
        env: EnvironmentRef<'_>,
        format: SdpAudioFormatRef<'_>,
    ) -> Option<AudioDecoder> {
        assert!(!env.as_ptr().is_null());
        assert_eq!(format.name().expect("名前の取得に失敗しました"), "opus");
        if self.created {
            return None;
        }
        self.created = true;
        Some(AudioDecoder::new_with_handler(Box::new(
            TestAudioDecoderHandler,
        )))
    }
}

#[test]
fn custom_audio_decoder_factory_roundtrip() {
    let factory = AudioDecoderFactory::new_with_handler(Box::new(TestAudioDecoderFactoryHandler {
        created: false,
    }));
    assert_eq!(factory.get_supported_decoders().len(), 1);
    let env = Environment::new();
    let format = SdpAudioFormat::new("opus", 48000, 2);
    assert!(factory.is_supported_decoder(format.as_ref()));
    let mut decoder = factory
        .create(env.as_ref(), format.as_ref())
        .expect("カスタムデコーダーの作成に失敗しました");
    assert_eq!(decoder.sample_rate_hz(), 48000);
    assert_eq!(decoder.channels(), 2);
    let mut decoded = [0x7FFFi16; 320];
    let (samples, speech) = decoder.decode(&[0x01, 0x02, 0x03], 48000, &mut decoded);
    assert_eq!(samples, 160);
    assert_eq!(speech, AudioSpeechType::Speech);
    assert!(
        decoded[..160].iter().all(|&v| v == 0x1111),
        "FFI 経由でデコード結果が書き込まれていない想定です"
    );
    assert!(
        decoded[160..].iter().all(|&v| v == 0x7FFF),
        "未書き込み領域の番兵が破壊された想定です"
    );
    assert!(
        factory.create(env.as_ref(), format.as_ref()).is_none(),
        "2 回目の create は None を返す想定です"
    );
}

struct TestComfortNoiseDecoderHandler;

impl AudioDecoderHandler for TestComfortNoiseDecoderHandler {
    fn sample_rate_hz(&mut self) -> i32 {
        48000
    }
    fn channels(&mut self) -> usize {
        2
    }
    fn decode(
        &mut self,
        _encoded: &[u8],
        _sample_rate_hz: i32,
        decoded: &mut RawBufferWriter<'_, i16>,
    ) -> (i32, AudioSpeechType) {
        decoded.write(&[0x2222i16; 80]);
        (80, AudioSpeechType::ComfortNoise)
    }
    fn reset(&mut self) {}
}

#[test]
fn audio_decoder_comfort_noise_round_trip() {
    let mut decoder = AudioDecoder::new_with_handler(Box::new(TestComfortNoiseDecoderHandler));
    let mut decoded = [0x7FFFi16; 160];
    let (samples, speech) = decoder.decode(&[0x01], 48000, &mut decoded);
    assert_eq!(samples, 80);
    assert_eq!(speech, AudioSpeechType::ComfortNoise);
    assert!(
        decoded[..80].iter().all(|&v| v == 0x2222),
        "快音のデコード結果が書き込まれていない想定です"
    );
    assert!(
        decoded[80..].iter().all(|&v| v == 0x7FFF),
        "未書き込み領域の番兵が破壊された想定です"
    );
}

#[test]
fn audio_decoder_default_queries() {
    // 既定実装は packet_duration が -2 (kNotImplemented)、packet_has_fec / has_decode_plc が false。
    let decoder = AudioDecoder::new_with_handler(Box::new(TestAudioDecoderHandler));
    assert_eq!(decoder.packet_duration(&[0x01, 0x02]), -2);
    assert!(!decoder.packet_has_fec(&[0x01]));
    assert!(!decoder.has_decode_plc());
}

#[test]
fn audio_codec_pair_id_eq_ord() {
    let a = AudioCodecPairId::create();
    let b = a.clone();
    assert!(a == b);
    assert!(a.cmp(&b) == std::cmp::Ordering::Equal);
    assert_eq!(a.numeric_representation(), b.numeric_representation());
}

#[test]
fn audio_encoder_owned_wrapper_methods() {
    let mut encoder =
        AudioEncoder::new_with_handler(Box::new(TestAudioEncoderHandler { encoded: false }));
    // 既定実装の仕様。
    encoder.set_max_playback_rate(48000);
    encoder.disable_audio_network_adaptor();
    assert!(!encoder.enable_audio_network_adaptor(&[0x01]));
    encoder.on_received_rtt(10);
    encoder.on_received_target_audio_bitrate(64000);
    encoder.on_received_overhead(12);
}
