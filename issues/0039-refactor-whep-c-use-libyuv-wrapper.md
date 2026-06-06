# whep.c で libyuv の C ラッパーを使う

- Priority: Medium
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whep.c` が `webrtc_c` の薄い C ラッパーを迂回し、libyuv の関数とシンボルを直接呼び出している。
すでに `libyuv_ConvertFromI420` ラッパーと `libyuv_FOURCC_ARGB` シンボルが用意されているため、それらを経由するように改める。
これにより libyuv への依存を `webrtc_c` のラッパー層に閉じ込め、利用側のコードを一貫させる。

## 優先度根拠

現状でも動作はするが、libyuv を薄いラッパーで隠蔽するという `webrtc_c` の設計方針に反している。
ラッパーを介さない直接呼び出しが残っていると、依存の境界が曖昧になり、将来 libyuv の差し替えや API 変更時に修正漏れの温床となる。
動作不具合ではないため優先度は Medium とする。

## 現状

`webrtc/src/whep.c:168` で、I420 から ARGB への変換に libyuv の `ConvertFromI420` を直接呼び、変換先フォーマットとして `FOURCC_ARGB` を直接指定している。

```c
  ConvertFromI420(
      webrtc_I420Buffer_MutableDataY(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_StrideY(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_MutableDataU(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_StrideU(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_MutableDataV(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_StrideV(webrtc_I420Buffer_refcounted_get(buf)), image,
      renderer->width * 4, renderer->width, renderer->height, FOURCC_ARGB);
```

一方、`webrtc/src/webrtc_c/libyuv.h:28` には同等の機能を持つ C ラッパーが宣言されている。

```c
WEBRTC_EXPORT int libyuv_ConvertFromI420(const uint8_t* src_y,
                                         int src_stride_y,
                                         const uint8_t* src_u,
                                         int src_stride_u,
                                         const uint8_t* src_v,
                                         int src_stride_v,
                                         uint8_t* dst_argb,
                                         int dst_stride_argb,
                                         int width,
                                         int height,
                                         uint32_t fourcc);
```

その実装（`webrtc/src/webrtc_c/libyuv.cc:41`）は内部で `libyuv::ConvertFromI420` を呼び出すだけの薄いラッパーである。
また `FOURCC_ARGB` に対応する定数として `webrtc/src/webrtc_c/libyuv.h:15` に `libyuv_FOURCC_ARGB` が公開されており、その実体は `webrtc/src/webrtc_c/libyuv.cc:22` で `libyuv::FOURCC_ARGB` から定義されている。

つまり `webrtc/src/whep.c` は、既存のラッパーを使わずに libyuv のシンボルへ直接依存している。

## 設計方針

- `webrtc/src/whep.c` の `ConvertFromI420` 呼び出しを `libyuv_ConvertFromI420` 経由に置き換える。
- 変換先フォーマット指定の `FOURCC_ARGB` を `libyuv_FOURCC_ARGB` に置き換える。
- 引数の並びは既存ラッパーの宣言（`webrtc/src/webrtc_c/libyuv.h:28`）に合わせ、変換結果が現状と一致するようにする。
- libyuv のヘッダを直接 include している箇所があれば不要になるため整理する。

## 完了条件

- `webrtc/src/whep.c` が libyuv の関数・シンボルを直接呼ばず、`libyuv_ConvertFromI420` と `libyuv_FOURCC_ARGB` を経由して変換を行っている。
- 変換結果（描画内容）が変更前と一致している。
