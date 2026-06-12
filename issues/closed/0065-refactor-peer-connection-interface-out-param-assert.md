# peer_connection_interface.cc の out パラメータデリファレンス前に assert を追加する

- Priority: Low
- Polished: 2026-06-09
- Created: 2026-06-09
- Completed: 2026-06-09
- Model: DeepSeek V4 Pro

## 目的

`webrtc/src/webrtc_c/api/peer_connection_interface.cc` には、`out_*` パラメータをデリファレンス（`*out_rtc_error = nullptr`、`*out_rtc_error = reinterpret_cast<...>(...)` 等）する関数が 11 件存在する。
しかし、`webrtc/RULES.md` が規定する「デリファレンスが必要な箇所では `assert(ptr != nullptr)` を入れる」がどの関数でも行われておらず、out パラメータに `NULL` が渡された際の契約違反をデバッグビルドで検出できない。

## 優先度根拠

いずれの呼び出し側も現状では非 null の out パラメータを渡しており、即座の不具合にはつながらない。
ただし、issue 0043（`issues/closed/0043-refactor-webrtc-c-null-check-consistency.md`）で他の箇所の assert 化が行われたあとも、これら 11 関数は未対応のまま残っている。issue 0043 の解決方法で定められた「出力引数ガード: `if (out_xxx != nullptr)` を `assert(out_xxx != nullptr);` に置換」は既存の null チェックを assert に置き換えるものであり、本 issue が対象とする「もともと null チェックが一切ないデリファレンス箇所」とは性質が異なるが、RULES.md の一貫性の問題として対処する価値がある。

## 現状

以下の 11 関数で out パラメータのデリファレンス前に `assert` が存在しない:

| # | 関数 | 行 | デリファレンスされる out パラメータ |
|---|------|----|--------------------------------------|
| 1 | `webrtc_PeerConnectionInterface_CreateDataChannelOrError` | 463 | `out_data_channel`, `out_rtc_error` |
| 2 | `webrtc_PeerConnectionInterface_AddTransceiver` | 488 | `out_transceiver`, `out_rtc_error` |
| 3 | `webrtc_PeerConnectionInterface_AddTransceiverWithTrack` | 510 | `out_transceiver`, `out_rtc_error` |
| 4 | `webrtc_PeerConnectionInterface_AddTrack` | 535 | `out_sender`, `out_rtc_error` |
| 5 | `webrtc_PeerConnectionInterface_RemoveTrackOrError` | 561 | `out_rtc_error` |
| 6 | `webrtc_PeerConnectionInterface_SetConfiguration` | 657 | `out_rtc_error` |
| 7 | `webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError` | 1144 | `out_pc`, `out_rtc_error` |
| 8 | `webrtc_PeerConnectionFactoryInterface_CreateVideoTrack` | 1171 | `out_track` |
| 9 | `webrtc_PeerConnectionFactoryInterface_CreateLocalMediaStream` | 1193 | `out_stream` |
| 10 | `webrtc_PeerConnectionFactoryInterface_CreateAudioSource` | 1274 | `out_source` |
| 11 | `webrtc_PeerConnectionFactoryInterface_CreateAudioTrack` | 1290 | `out_track` |

参考: 同ファイルの `webrtc_CreateModularPeerConnectionFactoryWithContext`（1127 行目）は `assert(out_context != nullptr)` を適切に行っており、これが正しい手本となる。

各関数のパターン（例: `CreateDataChannelOrError`）:

```cpp
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateDataChannelOrError(
    struct webrtc_PeerConnectionInterface* self,
    const char* label,
    size_t label_len,
    struct webrtc_DataChannelInit* init,
    struct webrtc_DataChannelInterface_refcounted** out_data_channel,
    struct webrtc_RTCError_unique** out_rtc_error) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  // ...
  if (r.ok()) {
    // ...
    *out_rtc_error = nullptr;  // assert なしでデリファレンス
  } else {
    // ...
    *out_rtc_error = ...;      // assert なしでデリファレンス
  }
}
```

## 設計方針

1. 各関数の先頭（最初の `reinterpret_cast` の直後）に、デリファレンスされる全 out パラメータの `assert` を追加する。
2. 対象となる out パラメータは以下の 8 種類:
   - `out_rtc_error`（11 関数中 7 関数で使用）
   - `out_data_channel`（`CreateDataChannelOrError` のみ）
   - `out_transceiver`（`AddTransceiver`、`AddTransceiverWithTrack` の 2 関数）
   - `out_sender`（`AddTrack` のみ）
   - `out_pc`（`CreatePeerConnectionOrError` のみ）
   - `out_track`（`CreateVideoTrack`、`CreateAudioTrack` の 2 関数）
   - `out_stream`（`CreateLocalMediaStream` のみ）
   - `out_source`（`CreateAudioSource` のみ）
3. 本 issue では out パラメータのデリファレンスに限定して assert を追加する。入力パラメータのデリファレンス（`*init`、`*cfg`、`*config`、`*ids`、`*opts` 等）は `webrtc/RULES.md` の「C ラッパーではポインタ引数の null チェックを原則として行わない」に従い assert 不要であり、対象外とする。
   - 例: `CreateDataChannelOrError` は `out_data_channel` と `out_rtc_error` の両方をデリファレンスするため、両方に assert を追加する。
   - 例: `RemoveTrackOrError` は `out_rtc_error` のみをデリファレンスするため、`assert(out_rtc_error != nullptr)` のみを追加する。
   - 例: `SetConfiguration` は `out_rtc_error` のみをデリファレンスするため、`assert(out_rtc_error != nullptr)` のみを追加する。
4. `#include <cassert>` は既にファイル先頭（5 行目）に存在するため、新規 include の追加は不要。
5. 既存の正常系の挙動は変えない（リリースビルドでは `NDEBUG` により `assert` が無効化される）。

## 変更対象ファイル

| ファイル | 行 | 変更内容 |
|----------|-----|----------|
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 470 前後 | `CreateDataChannelOrError`: `assert(out_data_channel != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 494 前後 | `AddTransceiver`: `assert(out_transceiver != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 516 前後 | `AddTransceiverWithTrack`: `assert(out_transceiver != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 541 前後 | `AddTrack`: `assert(out_sender != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 565 前後 | `RemoveTrackOrError`: `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 661 前後 | `SetConfiguration`: `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 1152 前後 | `CreatePeerConnectionOrError`: `assert(out_pc != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 1177 前後 | `CreateVideoTrack`: `assert(out_track != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 1199 前後 | `CreateLocalMediaStream`: `assert(out_stream != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 1279 前後 | `CreateAudioSource`: `assert(out_source != nullptr)` を追加 |
| `webrtc/src/webrtc_c/api/peer_connection_interface.cc` | 1296 前後 | `CreateAudioTrack`: `assert(out_track != nullptr)` を追加 |
| `CHANGES.md` | `### misc` 内 | `[UPDATE]` エントリを追加 |

## 完了条件

- 上記 11 関数すべてで、デリファレンスされる全 out パラメータに対して `assert(ptr != nullptr)` が追加されている。
- リリースビルドが通ることを確認する（`python3 run.py build ubuntu-24.04_x86_64`）。
- デバッグビルドが通ることを確認する（`python3 run.py build ubuntu-24.04_x86_64 --debug`）。
  - デバッグビルドでは `assert` が有効であり、正しくコンパイルできることを検証する。
- 既存のテストがすべて通ることを確認する。
- 本変更は `assert` 追加のみで機能面の変更はないため、新規テストは不要。
- CHANGES.md の `## develop` セクションの `### misc` に以下のエントリを追加する:
  - `- [UPDATE] peer_connection_interface.cc の out パラメータデリファレンス前に assert を追加する`
  - `- @melpon`

## 解決方法

11 関数すべての先頭（最初の `reinterpret_cast` の直後）に、以下の方針で `assert(ptr != nullptr)` を追加した:

- デリファレンスされる全 out パラメータに対して assert を追加する
- 入力パラメータの assert は不要（`webrtc/RULES.md` の「ポインタ引数の null チェックを原則として行わない」に従う）
- 関数ごとの追加内容は変更対象ファイルの表の通り

本変更は assert 追加のみで機能面の変更はない。既存のテスト 101 件が全てパスすることを確認した。
