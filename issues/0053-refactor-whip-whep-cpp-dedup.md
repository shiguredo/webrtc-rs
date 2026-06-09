# whip.cpp と whep.cpp の重複を共通化する

- Priority: Medium
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.cpp` と `webrtc/src/whep.cpp` は、複数のクラスおよび関数を 1 文字も違わずに重複して保持している。共通化することで、片方だけを修正してもう片方を直し忘れるという事故を防ぎ、保守コストを下げる。

## 優先度根拠

機能上のバグではないため High ではない。一方で、重複しているのは TLS ソケット通信・URL パースという壊れやすい箇所であり、片側だけの修正漏れが将来のバグに直結する。恒常的な保守負債であるため Medium とする。

## 現状

両ファイルには以下の重複が存在する。いずれもレビュー時点で実コードを確認済み。

### PeerConnectionFactory クラス

`whip.cpp:91-172` と `whep.cpp:78-159` で、`class PeerConnectionFactory` 全体が一致する。`Create()` 静的メソッド、デストラクタ、各アクセサ、メンバ変数まで同一である。

### 3 つの Thunk クラス

`whip.cpp:174-262` と `whep.cpp:161-249` で、`CreateSessionDescriptionThunk`、`SetLocalDescriptionThunk`、`SetRemoteDescriptionThunk` の 3 クラスが一致する。例として `SetLocalDescriptionThunk` の中核は両ファイルとも次の通り。

```cpp
  void OnSetLocalDescriptionComplete(webrtc::RTCError error) override {
    auto f = std::move(on_complete_);
    on_complete_ = nullptr;
    if (f) {
      f(error);
    }
  }
```

### URLParts

`whip.cpp:264-328` と `whep.cpp:251-315` で、`struct URLParts`（`Parse` と `GetPort` を含む）が一致する。

### SendRequest と ScopeExit とソケット補助

`whip.cpp:919-1047` と `whep.cpp:657-785` で、`SocketType` / `kInvalidSocket` の定義、`IsInvalidSocket`、`CloseSocket`、`struct ScopeExit`、`SendRequest` が一致する。`SendRequest` は `getaddrinfo` から `SSL_CTX_new` / `SSL_connect` / `SSL_read` ループまで同一である。

## 設計方針

- 重複している共通部分（`PeerConnectionFactory` クラス、3 つの Thunk クラス、`URLParts`、`ScopeExit` とソケット補助、`SendRequest`）を共通ヘッダ・共通実装へ括り出し、`whip.cpp` と `whep.cpp` の双方から利用する。
- C++ ファイルのビルド構成 (`build.rs` などの参照) を確認し、新規ファイルが正しくコンパイル対象になるようにする。

## 完了条件

- 上記の主要な重複（`PeerConnectionFactory`、3 つの Thunk クラス、`URLParts`、`SendRequest` 周辺）が共通化される。
- `whip` と `whep` の挙動が共通化前と変わらないこと。
