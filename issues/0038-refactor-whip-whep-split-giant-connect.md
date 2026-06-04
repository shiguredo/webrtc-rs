# whip/whep の巨大な接続関数を分割する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-whep-split-giant-connect

## 目的

WHIP の接続処理を担う関数が 1 関数で 200〜300 行を超えており、さらに深いネストのラムダを含んでいる。
このため、処理の流れが追いづらく、可読性と保守性が著しく低下している。
処理段階ごとに関数へ分割し、ネストを浅くすることで、見通しの良いコードへ改善する。

## 優先度根拠

接続処理は WHIP/WHEP の中核であり、バグ修正や機能追加のたびに読み解く必要がある。
巨大関数のままでは変更時の認知負荷が高く、修正漏れや回帰を招きやすい。
ただし現時点で動作上の不具合が出ているわけではないため、優先度は Medium とする。

## 現状

### `webrtc/src/whip.c` の `SignalingWhip_Connect`

`webrtc/src/whip.c:1260` から始まる `SignalingWhip_Connect` は、`webrtc/src/whip.c:1502` まで続く約 242 行の単一関数である。
PeerConnection の生成、音声トランシーバーの追加とコーデック設定、映像トランシーバーの追加とコーデック設定、オファー生成までを 1 関数で行っている。

```c
void SignalingWhip_Connect(struct SignalingWhip* self) {
  struct webrtc_PeerConnectionInterface_RTCConfiguration* rtc_config =
      webrtc_PeerConnectionInterface_RTCConfiguration_new();
  self->observer = webrtc_PeerConnectionObserver_new(&self->observer_cbs, self);
```

映像コーデックの選定では、`webrtc/src/whip.c:1419` 付近からの送信エンコーディングを走査するループの内側に、コーデック一致判定のループ（`webrtc/src/whip.c:1436`）と重複判定のループ（`webrtc/src/whip.c:1448`）が三重に入れ子になっている。

### `webrtc/src/whip.cpp` の `Connect`

`webrtc/src/whip.cpp` の `Connect` は約 336 行に及び、内部に多重のネストラムダを抱えている。
`webrtc/src/whip.cpp:678` の `CreateOffer` のコールバックラムダの中に、`webrtc/src/whip.cpp:754` の `SendRequest` のコールバックラムダがあり、さらにその中に `webrtc/src/whip.cpp:841` の `SetLocalDescription` のコールバックラムダ、その中に `webrtc/src/whip.cpp:854` の `SetRemoteDescription` のコールバックラムダがある。

```cpp
    pc->CreateOffer(
        CreateSessionDescriptionThunk::Create(
            [this,
             video_init](webrtc::SessionDescriptionInterface* description) {
```

このように非同期コールバックが 5〜6 重に入れ子になっており、対応する閉じ括弧（`webrtc/src/whip.cpp:887` 付近）まで処理の対応関係を追うのが困難である。

## 設計方針

- 処理段階（PeerConnection 生成、音声トランシーバー設定、映像トランシーバー設定、オファー生成、シグナリング応答処理など）ごとに、責務の明確な補助関数へ分割する。
- `webrtc/src/whip.c` では、現在 1 関数に詰め込まれている各ブロックを静的な補助関数として切り出し、`SignalingWhip_Connect` 本体は段階の呼び出しの並びとして読めるようにする。
- `webrtc/src/whip.cpp` では、多重に入れ子になったコールバックラムダをメンバ関数として切り出し、各段階のラムダはそのメンバ関数を呼ぶだけにとどめてネストを浅くする。
- 既存の挙動（エラー時のログ出力・状態遷移・リソース解放の順序）は変えない。

## 完了条件

- `SignalingWhip_Connect`（`webrtc/src/whip.c`）と `Connect`（`webrtc/src/whip.cpp`）が、それぞれ処理段階ごとの補助関数に分割されている。
- 各関数が画面内で見通せる程度の長さに収まっている。
- コールバックの入れ子の深さが浅くなっている。
- 分割前後で WHIP の接続挙動が変わっていない。
