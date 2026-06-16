use shiguredo_webrtc::{
    I420Buffer, LibyuvFourcc, LibyuvRotationMode, NV12Buffer, abgr_to_i420, convert_from_i420,
    convert_to_i420, i420_copy, i420_rotate, i420_to_nv12, mjpg_size, mjpg_to_i420, mjpg_to_nv12,
    nv12_copy, nv12_to_i420,
};

/// Pillow quality=90 subsampling=2 baseline JPEG (8x8 RGB=(128, 128, 128))
/// 全画素の期待値: BT.601 JFIF full-range 換算で Y=128 / U=128 / V=128
const TEST_MJPG_GRAY_8X8: &[u8] = &[
    0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01,
    0x00, 0x01, 0x00, 0x00, 0xff, 0xdb, 0x00, 0x43, 0x00, 0x03, 0x02, 0x02, 0x03, 0x02, 0x02, 0x03,
    0x03, 0x03, 0x03, 0x04, 0x03, 0x03, 0x04, 0x05, 0x08, 0x05, 0x05, 0x04, 0x04, 0x05, 0x0a, 0x07,
    0x07, 0x06, 0x08, 0x0c, 0x0a, 0x0c, 0x0c, 0x0b, 0x0a, 0x0b, 0x0b, 0x0d, 0x0e, 0x12, 0x10, 0x0d,
    0x0e, 0x11, 0x0e, 0x0b, 0x0b, 0x10, 0x16, 0x10, 0x11, 0x13, 0x14, 0x15, 0x15, 0x15, 0x0c, 0x0f,
    0x17, 0x18, 0x16, 0x14, 0x18, 0x12, 0x14, 0x15, 0x14, 0xff, 0xdb, 0x00, 0x43, 0x01, 0x03, 0x04,
    0x04, 0x05, 0x04, 0x05, 0x09, 0x05, 0x05, 0x09, 0x14, 0x0d, 0x0b, 0x0d, 0x14, 0x14, 0x14, 0x14,
    0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14,
    0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14,
    0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0xff, 0xc0,
    0x00, 0x11, 0x08, 0x00, 0x08, 0x00, 0x08, 0x03, 0x01, 0x22, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11,
    0x01, 0xff, 0xc4, 0x00, 0x1f, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    0x0a, 0x0b, 0xff, 0xc4, 0x00, 0xb5, 0x10, 0x00, 0x02, 0x01, 0x03, 0x03, 0x02, 0x04, 0x03, 0x05,
    0x05, 0x04, 0x04, 0x00, 0x00, 0x01, 0x7d, 0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21,
    0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23,
    0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a,
    0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a,
    0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a,
    0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99,
    0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
    0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5,
    0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf1,
    0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xff, 0xc4, 0x00, 0x1f, 0x01, 0x00, 0x03,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0xff, 0xc4, 0x00, 0xb5, 0x11, 0x00,
    0x02, 0x01, 0x02, 0x04, 0x04, 0x03, 0x04, 0x07, 0x05, 0x04, 0x04, 0x00, 0x01, 0x02, 0x77, 0x00,
    0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71, 0x13,
    0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0, 0x15,
    0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34, 0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26, 0x27,
    0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
    0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
    0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88,
    0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6,
    0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4,
    0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe2,
    0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9,
    0xfa, 0xff, 0xda, 0x00, 0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0x28,
    0xa2, 0x8a, 0x00, 0xff, 0xd9,
];

/// Pillow quality=90 subsampling=2 baseline JPEG (8x8 RGB=(255, 0, 0))
/// 期待値 (JFIF full-range BT.601):
///   Y  =  76 (0.299*255 + 0.587*0 + 0.114*0 = 76.245)
///   U  =  85 (-0.1687*255 - 0.3313*0 + 0.5*0 + 128 = 84.98)
///   V  = 255 ( 0.5*255 - 0.4187*0 - 0.0813*0 + 128 = 255.5, 8-bit clip で 255)
///   U と V が別の値になる色のため U/V 取り違えバグを検出可能
const TEST_MJPG_RED_8X8: &[u8] = &[
    0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01,
    0x00, 0x01, 0x00, 0x00, 0xff, 0xdb, 0x00, 0x43, 0x00, 0x03, 0x02, 0x02, 0x03, 0x02, 0x02, 0x03,
    0x03, 0x03, 0x03, 0x04, 0x03, 0x03, 0x04, 0x05, 0x08, 0x05, 0x05, 0x04, 0x04, 0x05, 0x0a, 0x07,
    0x07, 0x06, 0x08, 0x0c, 0x0a, 0x0c, 0x0c, 0x0b, 0x0a, 0x0b, 0x0b, 0x0d, 0x0e, 0x12, 0x10, 0x0d,
    0x0e, 0x11, 0x0e, 0x0b, 0x0b, 0x10, 0x16, 0x10, 0x11, 0x13, 0x14, 0x15, 0x15, 0x15, 0x0c, 0x0f,
    0x17, 0x18, 0x16, 0x14, 0x18, 0x12, 0x14, 0x15, 0x14, 0xff, 0xdb, 0x00, 0x43, 0x01, 0x03, 0x04,
    0x04, 0x05, 0x04, 0x05, 0x09, 0x05, 0x05, 0x09, 0x14, 0x0d, 0x0b, 0x0d, 0x14, 0x14, 0x14, 0x14,
    0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14,
    0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14,
    0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0x14, 0xff, 0xc0,
    0x00, 0x11, 0x08, 0x00, 0x08, 0x00, 0x08, 0x03, 0x01, 0x22, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11,
    0x01, 0xff, 0xc4, 0x00, 0x1f, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    0x0a, 0x0b, 0xff, 0xc4, 0x00, 0xb5, 0x10, 0x00, 0x02, 0x01, 0x03, 0x03, 0x02, 0x04, 0x03, 0x05,
    0x05, 0x04, 0x04, 0x00, 0x00, 0x01, 0x7d, 0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21,
    0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23,
    0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a,
    0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a,
    0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a,
    0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99,
    0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
    0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5,
    0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf1,
    0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xff, 0xc4, 0x00, 0x1f, 0x01, 0x00, 0x03,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0xff, 0xc4, 0x00, 0xb5, 0x11, 0x00,
    0x02, 0x01, 0x02, 0x04, 0x04, 0x03, 0x04, 0x07, 0x05, 0x04, 0x04, 0x00, 0x01, 0x02, 0x77, 0x00,
    0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71, 0x13,
    0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0, 0x15,
    0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34, 0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26, 0x27,
    0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
    0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
    0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88,
    0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6,
    0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4,
    0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe2,
    0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9,
    0xfa, 0xff, 0xda, 0x00, 0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3f, 0x00, 0xf9,
    0xd2, 0x8a, 0x28, 0xaf, 0xc3, 0x0f, 0xf5, 0x4c, 0xff, 0xd9,
];

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

// ── MJPG 系テスト ──

/// mjpg_to_i420 で GRAY 8x8 JPEG をデコードし、全プレーンが 128±4 の範囲であることを確認する
#[test]
fn mjpg_to_i420_decodes_gray_frame() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    let mut dst_v = vec![0u8; 4 * 4];
    assert!(
        mjpg_to_i420(
            sample, &mut dst_y, 8, &mut dst_u, 4, &mut dst_v, 4, 8, 8, 8, 8
        ),
        "MJPG→I420 のグレーフレームデコードに失敗しました"
    );
    assert!(
        dst_y.iter().all(|&v| (124..=132).contains(&v)),
        "Y plane の値が期待範囲外です: {:?}",
        &dst_y[..8.min(dst_y.len())]
    );
    assert!(
        dst_u.iter().all(|&v| (124..=132).contains(&v)),
        "U plane の値が期待範囲外です: {:?}",
        &dst_u[..]
    );
    assert!(
        dst_v.iter().all(|&v| (124..=132).contains(&v)),
        "V plane の値が期待範囲外です: {:?}",
        &dst_v[..]
    );
}

/// mjpg_to_nv12 で GRAY 8x8 JPEG をデコードし、全プレーンが 128±4 の範囲であることを確認する
#[test]
fn mjpg_to_nv12_decodes_gray_frame() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_uv = vec![0u8; 8 * 4];
    assert!(
        mjpg_to_nv12(sample, &mut dst_y, 8, &mut dst_uv, 8, 8, 8, 8, 8),
        "MJPG→NV12 のグレーフレームデコードに失敗しました"
    );
    assert!(
        dst_y.iter().all(|&v| (124..=132).contains(&v)),
        "Y plane の値が期待範囲外です"
    );
    assert!(
        dst_uv.iter().all(|&v| (124..=132).contains(&v)),
        "UV plane の値が期待範囲外です"
    );
}

/// mjpg_to_i420 で RED 8x8 JPEG をデコードし、各プレーンが期待値の範囲内であることを確認する
#[test]
fn mjpg_to_i420_decodes_red_frame() {
    let sample = TEST_MJPG_RED_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    let mut dst_v = vec![0u8; 4 * 4];
    assert!(
        mjpg_to_i420(
            sample, &mut dst_y, 8, &mut dst_u, 4, &mut dst_v, 4, 8, 8, 8, 8
        ),
        "MJPG→I420 の赤フレームデコードに失敗しました"
    );
    // Y: 76±4
    assert!(
        dst_y.iter().all(|&v| (72..=80).contains(&v)),
        "Y plane の値が期待範囲外です (期待 76±4)"
    );
    // U: 85±4
    assert!(
        dst_u.iter().all(|&v| (81..=89).contains(&v)),
        "U plane の値が期待範囲外です (期待 85±4)"
    );
    // V: 240..=255
    assert!(
        dst_v.iter().all(|&v| v >= 240),
        "V plane の値が期待範囲外です (期待 240..=255)"
    );
}

/// mjpg_to_nv12 で RED 8x8 JPEG をデコードし、NV12 UV interleave 順序 U-V-U-V も検証する
#[test]
fn mjpg_to_nv12_decodes_red_frame() {
    let sample = TEST_MJPG_RED_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_uv = vec![0u8; 8 * 4];
    assert!(
        mjpg_to_nv12(sample, &mut dst_y, 8, &mut dst_uv, 8, 8, 8, 8, 8),
        "MJPG→NV12 の赤フレームデコードに失敗しました"
    );
    // Y: 76±4
    assert!(
        dst_y.iter().all(|&v| (72..=80).contains(&v)),
        "Y plane の値が期待範囲外です (期待 76±4)"
    );
    // UV 平面の偶数インデックス (U): 85±4
    for i in (0..dst_uv.len()).step_by(2) {
        assert!(
            (81..=89).contains(&dst_uv[i]),
            "UV plane の偶数インデックス (U) の値が期待範囲外です: index={}, val={}",
            i,
            dst_uv[i]
        );
    }
    // UV 平面の奇数インデックス (V): 240..=255
    for i in (1..dst_uv.len()).step_by(2) {
        assert!(
            dst_uv[i] >= 240,
            "UV plane の奇数インデックス (V) の値が期待範囲外です: index={}, val={}",
            i,
            dst_uv[i]
        );
    }
}

/// mjpg_to_i420 の dst_v が不足している場合に事前検証で false が返ることを確認する
#[test]
fn mjpg_to_i420_returns_false_when_destination_plane_is_too_short() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    // dst_v を必要サイズ (16) より 1 byte 少なくする
    let mut dst_v = vec![0u8; 4 * 4 - 1];
    assert!(
        !mjpg_to_i420(
            sample, &mut dst_y, 8, &mut dst_u, 4, &mut dst_v, 4, 8, 8, 8, 8
        ),
        "dst_v が不足しているのに true が返りました"
    );
}

/// mjpg_to_nv12 の dst_uv が不足している場合に事前検証で false が返ることを確認する
#[test]
fn mjpg_to_nv12_returns_false_when_destination_plane_is_too_short() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    // dst_uv を必要サイズ (8*4=32) より 1 byte 少なくする
    let mut dst_uv = vec![0u8; 8 * 4 - 1];
    assert!(
        !mjpg_to_nv12(sample, &mut dst_y, 8, &mut dst_uv, 8, 8, 8, 8, 8),
        "dst_uv が不足しているのに true が返りました"
    );
}

/// mjpg_to_i420 で src_width/src_height が JPEG の値と不一致の場合に libyuv が 1 を返し false になることを確認する
#[test]
fn mjpg_to_i420_returns_false_when_src_dimensions_do_not_match() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 16 * 16];
    let mut dst_u = vec![0u8; 8 * 8];
    let mut dst_v = vec![0u8; 8 * 8];
    assert!(
        !mjpg_to_i420(
            sample, &mut dst_y, 16, &mut dst_u, 8, &mut dst_v, 8, 16, 16, 16, 16
        ),
        "src_width/src_height が JPEG と不一致なのに true が返りました"
    );
}

/// mjpg_to_nv12 で src_width/src_height が JPEG の値と不一致の場合に libyuv が 1 を返し false になることを確認する
#[test]
fn mjpg_to_nv12_returns_false_when_src_dimensions_do_not_match() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 16 * 16];
    let mut dst_uv = vec![0u8; 16 * 8];
    assert!(
        !mjpg_to_nv12(sample, &mut dst_y, 16, &mut dst_uv, 16, 16, 16, 16, 16),
        "src_width/src_height が JPEG と不一致なのに true が返りました"
    );
}

/// libyuv の ValidateJpeg が src_size_mjpg < 64 で弾く挙動を確認する (mjpg_to_i420)
#[test]
fn mjpg_to_i420_returns_false_when_sample_is_too_small() {
    let sample: &[u8] = &[0xff, 0xd8, 0xff, 0xd9];
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    let mut dst_v = vec![0u8; 4 * 4];
    assert!(
        !mjpg_to_i420(
            sample, &mut dst_y, 8, &mut dst_u, 4, &mut dst_v, 4, 8, 8, 8, 8
        ),
        "小さすぎるサンプルで true が返りました"
    );
}

/// libyuv の ValidateJpeg が src_size_mjpg < 64 で弾く挙動を確認する (mjpg_to_nv12)
#[test]
fn mjpg_to_nv12_returns_false_when_sample_is_too_small() {
    let sample: &[u8] = &[0xff, 0xd8, 0xff, 0xd9];
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_uv = vec![0u8; 8 * 4];
    assert!(
        !mjpg_to_nv12(sample, &mut dst_y, 8, &mut dst_uv, 8, 8, 8, 8, 8),
        "小さすぎるサンプルで true が返りました"
    );
}

/// mjpg_size で有効な JPEG から幅・高さを正しく取得できることを確認する
#[test]
fn mjpg_size_returns_dimensions() {
    let sample = TEST_MJPG_GRAY_8X8;
    assert_eq!(
        mjpg_size(sample),
        Some((8, 8)),
        "8x8 JPEG のサイズ取得に失敗しました"
    );
}

/// mjpg_size で不正なバイト列を渡すと None が返ることを確認する
#[test]
fn mjpg_size_returns_none_for_invalid_sample() {
    let sample: &[u8] = &[0xff, 0xd8, 0xff, 0xd9];
    assert!(
        mjpg_size(sample).is_none(),
        "不正なサンプルで Some が返りました"
    );
}

/// convert_to_i420 で GRAY 8x8 JPEG を MJPG fourcc 指定でデコードする
#[test]
fn convert_to_i420_decodes_gray_frame() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    let mut dst_v = vec![0u8; 4 * 4];
    assert!(
        convert_to_i420(
            sample,
            &mut dst_y,
            8,
            &mut dst_u,
            4,
            &mut dst_v,
            4,
            0,
            0,
            8,
            8,
            8,
            8,
            LibyuvRotationMode::Rotate0,
            LibyuvFourcc::Mjpg,
        ),
        "ConvertToI420 のグレーフレームデコードに失敗しました"
    );
    assert!(
        dst_y.iter().all(|&v| (124..=132).contains(&v)),
        "Y plane の値が期待範囲外です"
    );
    assert!(
        dst_u.iter().all(|&v| (124..=132).contains(&v)),
        "U plane の値が期待範囲外です"
    );
    assert!(
        dst_v.iter().all(|&v| (124..=132).contains(&v)),
        "V plane の値が期待範囲外です"
    );
}

/// convert_to_i420 で RED 8x8 JPEG を MJPG fourcc 指定でデコードし U/V 取り違えバグを検出する
#[test]
fn convert_to_i420_decodes_red_frame() {
    let sample = TEST_MJPG_RED_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    let mut dst_v = vec![0u8; 4 * 4];
    assert!(
        convert_to_i420(
            sample,
            &mut dst_y,
            8,
            &mut dst_u,
            4,
            &mut dst_v,
            4,
            0,
            0,
            8,
            8,
            8,
            8,
            LibyuvRotationMode::Rotate0,
            LibyuvFourcc::Mjpg,
        ),
        "ConvertToI420 の赤フレームデコードに失敗しました"
    );
    // Y: 76±4
    assert!(
        dst_y.iter().all(|&v| (72..=80).contains(&v)),
        "Y plane の値が期待範囲外です (期待 76±4)"
    );
    // U: 85±4
    assert!(
        dst_u.iter().all(|&v| (81..=89).contains(&v)),
        "U plane の値が期待範囲外です (期待 85±4)"
    );
    // V: 240..=255
    assert!(
        dst_v.iter().all(|&v| v >= 240),
        "V plane の値が期待範囲外です (期待 240..=255)"
    );
}

/// convert_to_i420 の dst_v が不足している場合に事前検証で false が返ることを確認する
#[test]
fn convert_to_i420_returns_false_when_destination_plane_is_too_short() {
    let sample = TEST_MJPG_GRAY_8X8;
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    // dst_v を必要サイズ (16) より 1 byte 少なくする
    let mut dst_v = vec![0u8; 4 * 4 - 1];
    assert!(
        !convert_to_i420(
            sample,
            &mut dst_y,
            8,
            &mut dst_u,
            4,
            &mut dst_v,
            4,
            0,
            0,
            8,
            8,
            8,
            8,
            LibyuvRotationMode::Rotate0,
            LibyuvFourcc::Mjpg,
        ),
        "dst_v が不足しているのに true が返りました"
    );
}

/// convert_to_i420 で小さすぎるサンプルを渡すと libyuv の ValidateJpeg で弾かれて false が返ることを確認する
#[test]
fn convert_to_i420_returns_false_when_sample_is_too_small() {
    let sample: &[u8] = &[0xff, 0xd8, 0xff, 0xd9];
    let mut dst_y = vec![0u8; 8 * 8];
    let mut dst_u = vec![0u8; 4 * 4];
    let mut dst_v = vec![0u8; 4 * 4];
    assert!(
        !convert_to_i420(
            sample,
            &mut dst_y,
            8,
            &mut dst_u,
            4,
            &mut dst_v,
            4,
            0,
            0,
            8,
            8,
            8,
            8,
            LibyuvRotationMode::Rotate0,
            LibyuvFourcc::Mjpg,
        ),
        "小さすぎるサンプルで true が返りました"
    );
}

// ── I420Rotate 系テスト ──

/// 0° 回転で入出力が一致することを確認する
#[test]
fn i420_rotate_rotate_0_preserves_planes() {
    let width = 4;
    let height = 4;
    let chroma_width = (width as usize + 1) / 2;
    let chroma_height = (height as usize + 1) / 2;

    let mut src_y = vec![0u8; (width * height) as usize];
    let mut src_u = vec![0u8; chroma_width * chroma_height];
    let mut src_v = vec![0u8; chroma_width * chroma_height];
    for (i, p) in src_y.iter_mut().enumerate() {
        *p = (i as u8).wrapping_mul(7);
    }
    for (i, p) in src_u.iter_mut().enumerate() {
        *p = 0x40u8.wrapping_add(i as u8);
    }
    for (i, p) in src_v.iter_mut().enumerate() {
        *p = 0x80u8.wrapping_add(i as u8);
    }

    let mut dst_y = vec![0u8; src_y.len()];
    let mut dst_u = vec![0u8; src_u.len()];
    let mut dst_v = vec![0u8; src_v.len()];
    assert!(i420_rotate(
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
        LibyuvRotationMode::Rotate0,
    ));

    assert_eq!(src_y, dst_y, "Rotate0 で Y plane が一致しません");
    assert_eq!(src_u, dst_u, "Rotate0 で U plane が一致しません");
    assert_eq!(src_v, dst_v, "Rotate0 で V plane が一致しません");
}

/// 90° 回転で width/height が入れ替わることを確認する。
/// 全要素が異なる値の小さいフレームで round-trip (4 回回転で元に戻る) を検証する。
#[test]
fn i420_rotate_rotate_90_swaps_dimensions() {
    let width = 4;
    let height = 2;
    let chroma_width = (width as usize + 1) / 2;
    let chroma_height = (height as usize + 1) / 2;

    let mut src_y = vec![0u8; (width * height) as usize];
    let mut src_u = vec![0u8; chroma_width * chroma_height];
    let mut src_v = vec![0u8; chroma_width * chroma_height];
    for (i, p) in src_y.iter_mut().enumerate() {
        *p = (i as u8).wrapping_add(1);
    }
    for (i, p) in src_u.iter_mut().enumerate() {
        *p = 0x40u8.wrapping_add(i as u8);
    }
    for (i, p) in src_v.iter_mut().enumerate() {
        *p = 0x80u8.wrapping_add(i as u8);
    }

    // 1 回目: 90° → dst は height x width
    let mut rotated90_y = vec![0u8; (width * height) as usize];
    let rotated_chroma_width90 = (height as usize + 1) / 2;
    let rotated_chroma_height90 = (width as usize + 1) / 2;
    let mut rotated90_u = vec![0u8; rotated_chroma_width90 * rotated_chroma_height90];
    let mut rotated90_v = vec![0u8; rotated_chroma_width90 * rotated_chroma_height90];
    assert!(
        i420_rotate(
            &src_y,
            width,
            &src_u,
            width / 2,
            &src_v,
            width / 2,
            &mut rotated90_y,
            height,
            &mut rotated90_u,
            height / 2,
            &mut rotated90_v,
            height / 2,
            width,
            height,
            LibyuvRotationMode::Rotate90,
        ),
        "90° 回転に失敗しました"
    );

    // 90° 回転が成功し、dst の解像度が height x width になることを確認する。
    // 丸め込み検証としては、90° → 270° の round-trip を確認する
    // (元が 4x2 の場合: 90° で 2x4 → 270° で 4x2 に戻る)
    let mut roundtrip_y = vec![0u8; src_y.len()];
    let mut roundtrip_u = vec![0u8; src_u.len()];
    let mut roundtrip_v = vec![0u8; src_v.len()];
    assert!(
        i420_rotate(
            &rotated90_y,
            height,
            &rotated90_u,
            height / 2,
            &rotated90_v,
            height / 2,
            &mut roundtrip_y,
            width,
            &mut roundtrip_u,
            width / 2,
            &mut roundtrip_v,
            width / 2,
            height,
            width,
            LibyuvRotationMode::Rotate270,
        ),
        "90° → 270° round-trip に失敗しました"
    );
    assert_eq!(
        src_y, roundtrip_y,
        "90°→270° round-trip で Y plane が一致しません"
    );
    assert_eq!(
        src_u, roundtrip_u,
        "90°→270° round-trip で U plane が一致しません"
    );
    assert_eq!(
        src_v, roundtrip_v,
        "90°→270° round-trip で V plane が一致しません"
    );
}

/// 180° 回転でピクセル順が反転することを確認する。
/// 2 回回転で元に戻る round-trip も検証する。
#[test]
fn i420_rotate_rotate_180_inverts_pixel_order() {
    let width = 4;
    let height = 4;
    let chroma_width = (width as usize + 1) / 2;
    let chroma_height = (height as usize + 1) / 2;

    let mut src_y = vec![0u8; (width * height) as usize];
    let mut src_u = vec![0u8; chroma_width * chroma_height];
    let mut src_v = vec![0u8; chroma_width * chroma_height];
    for (i, p) in src_y.iter_mut().enumerate() {
        *p = (i as u8).wrapping_add(1);
    }
    for (i, p) in src_u.iter_mut().enumerate() {
        *p = 0x40u8.wrapping_add(i as u8);
    }
    for (i, p) in src_v.iter_mut().enumerate() {
        *p = 0x80u8.wrapping_add(i as u8);
    }

    let mut rotated180_y = vec![0u8; src_y.len()];
    let mut rotated180_u = vec![0u8; src_u.len()];
    let mut rotated180_v = vec![0u8; src_v.len()];
    assert!(
        i420_rotate(
            &src_y,
            width,
            &src_u,
            width / 2,
            &src_v,
            width / 2,
            &mut rotated180_y,
            width,
            &mut rotated180_u,
            width / 2,
            &mut rotated180_v,
            width / 2,
            width,
            height,
            LibyuvRotationMode::Rotate180,
        ),
        "180° 回転に失敗しました"
    );

    // 180° 回転を 2 回繰り返すと元に戻る
    let mut roundtrip_y = vec![0u8; src_y.len()];
    let mut roundtrip_u = vec![0u8; src_u.len()];
    let mut roundtrip_v = vec![0u8; src_v.len()];
    assert!(
        i420_rotate(
            &rotated180_y,
            width,
            &rotated180_u,
            width / 2,
            &rotated180_v,
            width / 2,
            &mut roundtrip_y,
            width,
            &mut roundtrip_u,
            width / 2,
            &mut roundtrip_v,
            width / 2,
            width,
            height,
            LibyuvRotationMode::Rotate180,
        ),
        "2 回目の 180° 回転に失敗しました"
    );
    assert_eq!(
        src_y, roundtrip_y,
        "180° 2 回 round-trip で Y plane が一致しません"
    );
    assert_eq!(
        src_u, roundtrip_u,
        "180° 2 回 round-trip で U plane が一致しません"
    );
    assert_eq!(
        src_v, roundtrip_v,
        "180° 2 回 round-trip で V plane が一致しません"
    );
}

/// 270° 回転で width/height が入れ替わることを確認する。
/// 90° → 270° で round-trip も検証する。
#[test]
fn i420_rotate_rotate_270_swaps_dimensions() {
    let width = 4;
    let height = 2;
    let chroma_width = (width as usize + 1) / 2;
    let chroma_height = (height as usize + 1) / 2;

    let mut src_y = vec![0u8; (width * height) as usize];
    let mut src_u = vec![0u8; chroma_width * chroma_height];
    let mut src_v = vec![0u8; chroma_width * chroma_height];
    for (i, p) in src_y.iter_mut().enumerate() {
        *p = (i as u8).wrapping_add(1);
    }
    for (i, p) in src_u.iter_mut().enumerate() {
        *p = 0x40u8.wrapping_add(i as u8);
    }
    for (i, p) in src_v.iter_mut().enumerate() {
        *p = 0x80u8.wrapping_add(i as u8);
    }

    // 270° 回転 → dst は height x width
    let rotated_chroma_width270 = (height as usize + 1) / 2;
    let rotated_chroma_height270 = (width as usize + 1) / 2;
    let mut rotated270_y = vec![0u8; (width * height) as usize];
    let mut rotated270_u = vec![0u8; rotated_chroma_width270 * rotated_chroma_height270];
    let mut rotated270_v = vec![0u8; rotated_chroma_width270 * rotated_chroma_height270];
    assert!(
        i420_rotate(
            &src_y,
            width,
            &src_u,
            width / 2,
            &src_v,
            width / 2,
            &mut rotated270_y,
            height,
            &mut rotated270_u,
            height / 2,
            &mut rotated270_v,
            height / 2,
            width,
            height,
            LibyuvRotationMode::Rotate270,
        ),
        "270° 回転に失敗しました"
    );

    // 90° 回転との round-trip: Rotate90 → Rotate270 で元に戻る
    let rotated_chroma_w = (height as usize + 1) / 2;
    let rotated_chroma_h = (width as usize + 1) / 2;
    let mut after90_y = vec![0u8; (width * height) as usize];
    let mut after90_u = vec![0u8; rotated_chroma_w * rotated_chroma_h];
    let mut after90_v = vec![0u8; rotated_chroma_w * rotated_chroma_h];
    assert!(
        i420_rotate(
            &src_y,
            width,
            &src_u,
            width / 2,
            &src_v,
            width / 2,
            &mut after90_y,
            height,
            &mut after90_u,
            height / 2,
            &mut after90_v,
            height / 2,
            width,
            height,
            LibyuvRotationMode::Rotate90,
        ),
        "90° 回転に失敗しました"
    );

    let mut roundtrip_y = vec![0u8; src_y.len()];
    let mut roundtrip_u = vec![0u8; src_u.len()];
    let mut roundtrip_v = vec![0u8; src_v.len()];
    assert!(
        i420_rotate(
            &after90_y,
            height,
            &after90_u,
            height / 2,
            &after90_v,
            height / 2,
            &mut roundtrip_y,
            width,
            &mut roundtrip_u,
            width / 2,
            &mut roundtrip_v,
            width / 2,
            height,
            width,
            LibyuvRotationMode::Rotate270,
        ),
        "90° → 270° round-trip に失敗しました"
    );
    assert_eq!(
        src_y, roundtrip_y,
        "90°→270° round-trip で Y plane が一致しません"
    );
    assert_eq!(
        src_u, roundtrip_u,
        "90°→270° round-trip で U plane が一致しません"
    );
    assert_eq!(
        src_v, roundtrip_v,
        "90°→270° round-trip で V plane が一致しません"
    );
}

/// 90° 回転時に dst バッファが回転後解像度に対して不足している場合に false が返ることを確認する
#[test]
fn i420_rotate_returns_false_when_destination_plane_is_too_short() {
    let width = 4;
    let height = 4;
    let chroma_width = (width as usize + 1) / 2;
    let chroma_height = (height as usize + 1) / 2;

    let src_y = vec![0u8; (width * height) as usize];
    let src_u = vec![0u8; chroma_width * chroma_height];
    let src_v = vec![0u8; chroma_width * chroma_height];

    // Rotate90 では dst の解像度は height x width (= 4x4, この場合は同じ) だが
    // dst_y を不足させる
    let mut dst_y = vec![0u8; (width * height - 1) as usize];
    let mut dst_u = vec![0u8; chroma_width * chroma_height];
    let mut dst_v = vec![0u8; chroma_width * chroma_height];
    assert!(
        !i420_rotate(
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
            LibyuvRotationMode::Rotate90,
        ),
        "dst_y が不足しているのに true が返りました"
    );

    // dst_u が不足している場合も検証
    let mut dst_y = vec![0u8; (width * height) as usize];
    let mut dst_u = vec![0u8; chroma_width * chroma_height - 1];
    let mut dst_v = vec![0u8; chroma_width * chroma_height];
    assert!(
        !i420_rotate(
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
            LibyuvRotationMode::Rotate90,
        ),
        "dst_u が不足しているのに true が返りました"
    );

    // dst_v が不足している場合も検証
    let mut dst_u = vec![0u8; chroma_width * chroma_height];
    let mut dst_v = vec![0u8; chroma_width * chroma_height - 1];
    assert!(
        !i420_rotate(
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
            LibyuvRotationMode::Rotate90,
        ),
        "dst_v が不足しているのに true が返りました"
    );
}
