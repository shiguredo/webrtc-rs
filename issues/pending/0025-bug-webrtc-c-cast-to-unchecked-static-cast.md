# cast_to_* の未チェック static_cast ダウンキャストを安全にする

- Priority: Medium
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

## pending 理由

ダウンキャストの安全性確保（`dynamic_cast` + nullptr チェック）が
`webrtc/RULES.md:5-6` の「薄いラッパー」原則（独自の判定ロジック追加禁止）に
反するかどうかの判断が未解決。RULES.md 自体の修正要否を含めて設計判断が必要なため pending とする。

## 目的

`WEBRTC_DEFINE_CAST` / `WEBRTC_DEFINE_CAST_REFCOUNTED` マクロが生成する `cast_to_*`
API のうち、ダウンキャスト（基底型→派生型）を行う関数では `static_cast` が実型を
確認せずに実行されるため、渡されたオブジェクトの実型が想定と一致しない場合に
未定義動作となる。これを是正する。

## 優先度根拠

不正な型で `cast_to_*` を呼ぶと未定義動作になるが、正しく使えば問題は起きない。
クラッシュが常時発生する種類の問題ではなく、利用者の誤用が前提条件となるため
Medium とする。一方で未定義動作はデバッグが極めて困難であり、安全側に倒すか
前提を明示する価値が高い。

## 現状

### マクロ定義

`webrtc/src/webrtc_c/common.impl.h:10-26`:

```cpp
#define WEBRTC_DEFINE_CAST(type, cast_to, cpptype, cpp_cast_to)             \
  WEBRTC_EXPORT struct cast_to* WEBRTC_CONCAT(                              \
      type, WEBRTC_CONCAT(_cast_to_, cast_to))(struct type * self) {        \
    auto s = reinterpret_cast<cpptype*>(self);                              \
    return reinterpret_cast<struct cast_to*>(static_cast<cpp_cast_to*>(s)); \
  }

#define WEBRTC_DEFINE_CAST_REFCOUNTED(type, cast_to, cpptype, cpp_cast_to) \
  WEBRTC_EXPORT struct WEBRTC_CONCAT(cast_to, _refcounted) *               \
      WEBRTC_CONCAT(type, WEBRTC_CONCAT(_refcounted_cast_to_, cast_to))(   \
          struct WEBRTC_CONCAT(type, _refcounted) * self) {                \
    auto s = reinterpret_cast<cpptype*>(                                   \
        WEBRTC_CONCAT(type, _refcounted_get)(self));                       \
    webrtc::scoped_refptr<cpp_cast_to> ptr(static_cast<cpp_cast_to*>(s));  \
    return reinterpret_cast<struct WEBRTC_CONCAT(cast_to, _refcounted)*>(  \
        ptr.release());                                                    \
  }
```

いずれも `static_cast<cpp_cast_to*>(s)` でキャストを行うが、ポインタが指す
オブジェクトの実型を一切確認しない。

### 全使用箇所の分類（grep による全件調査）

| ファイル | 変換 | 方向 | 安全性 |
|----------|------|------|--------|
| `i420_buffer.cc` | `I420Buffer` → `VideoFrameBuffer` | upcast | 安全 |
| `nv12_buffer.cc` | `NV12Buffer` → `VideoFrameBuffer` | upcast | 安全 |
| `adapted_video_track_source.cc` | `AdaptedVideoTrackSourceWrapper` → `VideoTrackSourceInterface` | upcast | 安全 |
| `simulcast_encoder_adapter.cc` | `SimulcastEncoderAdapter` → `VideoEncoder` | upcast | 安全 |
| `media_stream_interface.cc` (2 箇所) | `VideoTrackInterface`/`AudioTrackInterface` → `MediaStreamTrackInterface` | upcast | 安全 |
| `media_stream_interface.cc:38-41` | `MediaStreamTrackInterface` → `VideoTrackInterface` | downcast | **危険** |
| `media_stream_interface.cc:42-45` | `MediaStreamTrackInterface` → `AudioTrackInterface` | downcast | **危険** |
| `rtp_parameters.cc` | `RtpCodecCapability` → `RtpCodec` | downcast | **危険** |

9 箇所中 6 箇所がアップキャスト（派生型→基底型）、3 箇所がダウンキャスト（基底型→派生型）。
アップキャストの `static_cast` は C++ 標準で well-defined。問題があるのは 3 箇所のダウンキャスト。

### 問題の具体例

音声トラックに対して `webrtc_MediaStreamTrackInterface_refcounted_cast_to_webrtc_VideoTrackInterface`
を呼ぶと、実型 (`AudioTrackInterface`) と要求型 (`VideoTrackInterface`) が不一致のまま
`static_cast` が実行され、未定義動作となる。

### 先行事例: video_frame_buffer.cc の dynamic_cast パターン

`webrtc/src/webrtc_c/api/video/video_frame_buffer.cc:144-166` には、ダウンキャストを
`dynamic_cast` + nullptr チェックで安全に実装した先行事例が既に存在する:

```cpp
WEBRTC_EXPORT struct webrtc_I420Buffer_refcounted*
webrtc_VideoFrameBuffer_cast_to_webrtc_I420Buffer(
    struct webrtc_VideoFrameBuffer* self) {
  auto buffer = reinterpret_cast<webrtc::VideoFrameBuffer*>(self);
  auto i420 = dynamic_cast<webrtc::I420Buffer*>(buffer);
  if (i420 == nullptr) {
    return nullptr;
  }
  // ...
}
```

この関数はマクロを使わず手書きされており、ダウンキャスト失敗時に `nullptr` を
返すパターンが既に確立している。本 issue で対象となる `MediaStreamTrackInterface` の
ダウンキャストにも同様のアプローチを適用するのが最も一貫性のある対応となる。

## 設計上のジレンマ

### RULES.md との整合性

- `webrtc/RULES.md:5-6` は「C ラッパーは薄く保ち、独自の便利関数や機能追加を行ってはいけない」
  と規定している
- `dynamic_cast` + nullptr チェックの追加は「元の C++ API に存在しない判定ロジックを
  ラッパー側で追加する」ことに該当し、「薄いラッパー」原則に反する可能性がある
- 一方、先行事例の `video_frame_buffer.cc` では既に `dynamic_cast` + nullptr チェックが
  実装されており、プロジェクトとして部分的に許容されているとも解釈できる
- もう一つの選択肢「前提条件の明記（コメントのみ）」は RULES.md に反しないが、
  実行時の安全性は改善されない

### 戻り値のセマンティクス変更

ダウンキャスト失敗を `nullptr` で表現する場合、呼び出し側（Rust 等）で nullptr
チェックが新たに必要になる。アップキャスト用のマクロも共通している場合、
アップキャストでは事実上 nullptr が発生し得ないにもかかわらず、API 上は
nullable に見えるという不一致が生じる。このため、ダウンキャスト用の独立した
マクロまたは手書き関数に分離することが望ましい。

## 推奨する設計方針

先行事例との一貫性を最優先し、以下の方針を推奨する:

1. **ダウンキャスト 3 箇所**をマクロ呼び出しから `video_frame_buffer.cc` と同様の
   手書き `dynamic_cast` + nullptr チェック関数に置き換える:
   - `media_stream_interface.cc`: `MediaStreamTrackInterface` → `VideoTrackInterface`
   - `media_stream_interface.cc`: `MediaStreamTrackInterface` → `AudioTrackInterface`
   - `rtp_parameters.cc`: `RtpCodecCapability` → `RtpCodec`
2. `dynamic_cast` を使用する理由:
   - 既存の先行事例と一貫する
   - `kind()` 等の文字列比較より確実
   - マクロ呼び出しより明示的で意図が伝わりやすい
3. アップキャスト 6 箇所は現状の `static_cast` のままで問題ないため変更しない

### 判断が必要な点（ユーザー確認事項）

- `dynamic_cast` + nullptr チェックが RULES.md の「薄いラッパー」原則に反するか否か
- 反する場合、RULES.md 自体を修正するか、コメントによる前提条件明記で妥協するか

## テスト戦略

- `media_stream_interface.cc` に `#[cfg(test)]` 相当の C++ テストで
  `MediaStreamTrackInterface`（音声）に対して `cast_to_VideoTrackInterface` を呼び、
  `nullptr` が返ることを検証する
- 正しい型のキャストが成功することの確認（既存の whip/whep ビルドで担保）

## 完了条件

- ダウンキャスト 3 箇所が `dynamic_cast` + nullptr チェックに置き換えられている
- 不正な型を渡した場合に未定義動作ではなく `nullptr` が返る
- アップキャスト 6 箇所は変更不要であることを確認している
- 採用した方針について RULES.md との整合性が確認・承認されている
