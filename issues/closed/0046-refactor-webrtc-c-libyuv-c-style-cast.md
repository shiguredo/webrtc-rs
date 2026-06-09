# libyuv.cc の C スタイルキャストを C++ キャストにする

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Completed: 2026-06-08
- Model: Opus 4.8

## 目的

`webrtc/src/webrtc_c/libyuv.cc` のラッパー実装で C スタイルキャストが使われている。
C スタイルキャストは `static_cast` / `reinterpret_cast` などのいずれの意味になるかがコンパイラ任せで、型安全性とキャストの意図が読み手に伝わりにくい。
C++ のキャストへ置き換え、意図を明示する。

## 優先度根拠

挙動は変わらず、機能的な不具合もない。
ただし `webrtc_c` の他のラッパー（同ファイル内の `static_cast` 使用箇所など）が C++ キャストを用いているのに対し一貫していないため、整理する意義がある。優先度は Low とする。

## 現状

`webrtc/src/webrtc_c/libyuv.cc:157` の `libyuv_I420Rotate` で、`int` 型の `mode` を libyuv の列挙型へ C スタイルキャストで変換している。

```cpp
  return libyuv::I420Rotate(src_y, src_stride_y, src_u, src_stride_u, src_v,
                            src_stride_v, dst_y, dst_stride_y, dst_u,
                            dst_stride_u, dst_v, dst_stride_v, width, height,
                            (libyuv::RotationMode)mode);
```

最終引数 `(libyuv::RotationMode)mode` が C スタイルキャストである。
一方で同じファイルの `webrtc/src/webrtc_c/libyuv.cc:22` 付近では `static_cast<uint32_t>(...)` のように C++ キャストが使われており、ファイル内でキャストの書き方が一貫していない。

## 設計方針

- `(libyuv::RotationMode)mode` を、整数値から列挙型への変換として意図が明確になる C++ キャスト（`static_cast<libyuv::RotationMode>(mode)` など）へ置き換える。
- 同ファイル内に他に C スタイルキャストが残っていないか確認し、あれば併せて適切な C++ キャストへ統一する。
- 変換の意味・挙動は変えない。

## 完了条件

- `webrtc/src/webrtc_c/libyuv.cc` から C スタイルキャストが解消され、適切な C++ キャストに置き換えられている。
- 変換の挙動が変更前と一致している。

## 解決方法

`webrtc/src/webrtc_c/libyuv.cc:157` の `(libyuv::RotationMode)mode` を `static_cast<libyuv::RotationMode>(mode)` に置き換えた。同ファイル内に他に C スタイルキャストは存在しないことを確認済み。挙動の変更はない。
