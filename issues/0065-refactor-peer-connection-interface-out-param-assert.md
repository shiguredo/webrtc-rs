# peer_connection_interface.cc の out パラメータデリファレンス前に assert を追加する

- Priority: Low
- Polished: 2026-06-09
- Created: 2026-06-09
- Model: DeepSeek V4 Pro

## 目的

`webrtc_c/api/peer_connection_interface.cc` には、`webrtc_RTCError_unique** out_rtc_error` をデリファレンス（`*out_rtc_error = nullptr`）、`*out_rtc_error = reinterpret_cast<...>(...)`）する関数が 7 件存在する。
しかし、RULES.md が規定する「デリファレンスが必要な箇所では `assert(ptr != nullptr)` を入れる」がどの関数でも行われておらず、`out_rtc_error` に `NULL` が渡された際の契約違反をデバッグビルドで検出できない。

## 優先度根拠

いずれの呼び出し側も現状では非 null の `out_rtc_error` を渡しており、即座の不具合にはつながらない。
ただし、issue 0043 で他の箇所の assert 化が行われたあともこの 7 関数は未対応のまま残っており、RULES.md の一貫性の問題として対処する価値がある。優先度は Low とする。

## 現状

以下の 7 関数で `out_rtc_error` のデリファレンス前に `assert(out_rtc_error != nullptr)` が存在しない:

| # | 関数 | 行 |
|---|------|----|
| 1 | `webrtc_PeerConnectionInterface_CreateDataChannelOrError` | 463 |
| 2 | `webrtc_PeerConnectionInterface_AddTransceiver` | 488 |
| 3 | `webrtc_PeerConnectionInterface_AddTransceiverWithTrack` | 510 |
| 4 | `webrtc_PeerConnectionInterface_AddTrack` | 535 |
| 5 | `webrtc_PeerConnectionInterface_RemoveTrackOrError` | 561 |
| 6 | `webrtc_PeerConnectionInterface_SetConfiguration` | 657 |
| 7 | `webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError` | 1144 |

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

1. 各関数の先頭（`auto pc = reinterpret_cast<...>(self)` の直前または直後）に `assert(out_rtc_error != nullptr)` を追加する。
2. `out_rtc_error` 以外の out パラメータ（`out_data_channel`、`out_transceiver`、`out_sender`、`out_pc`）についても、同様にデリファレンス前に `assert` を追加する。
   - これらも RULES.md のデリファレンス前 assert ルールの対象である。
   - 例: `SetConfiguration` は `out_rtc_error` のみであり、`out_rtc_error` の assert のみでよい。
   - 例: `CreateDataChannelOrError` は `out_data_channel` と `out_rtc_error` の両方をデリファレンスするため、両方に assert を追加する。
3. 既存の正常系の挙動は変えない（リリースビルドでの動作に影響しない）。

## 変更対象ファイル

| ファイル | 行 | 変更内容 |
|----------|-----|----------|
| `webrtc_c/api/peer_connection_interface.cc` | 470 前後 | `CreateDataChannelOrError`: `assert(out_data_channel != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc_c/api/peer_connection_interface.cc` | 494 前後 | `AddTransceiver`: `assert(out_transceiver != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc_c/api/peer_connection_interface.cc` | 516 前後 | `AddTransceiverWithTrack`: `assert(out_transceiver != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc_c/api/peer_connection_interface.cc` | 541 前後 | `AddTrack`: `assert(out_sender != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc_c/api/peer_connection_interface.cc` | 565 前後 | `RemoveTrackOrError`: `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc_c/api/peer_connection_interface.cc` | 661 前後 | `SetConfiguration`: `assert(out_rtc_error != nullptr)` を追加 |
| `webrtc_c/api/peer_connection_interface.cc` | 1150 前後 | `CreatePeerConnectionOrError`: `assert(out_pc != nullptr)` と `assert(out_rtc_error != nullptr)` を追加 |

## 完了条件

- 上記 7 関数の `out_rtc_error` デリファレンス前に `assert(out_rtc_error != nullptr)` が追加されている。
- その他の out パラメータ（`out_data_channel`、`out_transceiver`、`out_sender`、`out_pc`）にもデリファレンス前に `assert` が追加されている。
- 既存の正常系の挙動が変わっていない（リリースビルドで影響がない）。
- ビルドが通ることを確認する（`python3 run.py build ubuntu-24.04_x86_64`）。
- CHANGES.md の `## develop` セクションの `### misc` に `[FIX]` エントリを追加する。
