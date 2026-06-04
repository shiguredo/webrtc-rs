# whip.c と whep.c の大規模重複を共通化する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-whep-c-dedup

## 目的

`webrtc/src/whip.c` と `webrtc/src/whep.c` は、関数の接頭辞 (`whip_` / `whep_`) と一部の状態 enum 名以外、数百行にわたってほぼ同一のコードを重複して保持している。共通化することで、片方だけを修正してもう片方を直し忘れるという事故を防ぎ、保守コストを大きく下げる。

## 優先度根拠

機能上のバグではないため High ではない。一方で、重複しているのは TLS ソケット通信・URL パース・HTTP ヘッダ解析という比較的壊れやすい箇所であり、片側だけの修正漏れが将来のバグに直結する。広範囲かつ恒常的な保守負債であるため Medium とする。

## 現状

両ファイルには以下の重複が存在する。いずれもレビュー時点で実コードを確認済み。

### PeerConnectionFactory 関連

`whip.c:96-128` と `whep.c:29-61` で、`struct PeerConnectionFactory`、`PeerConnectionFactory_delete`、`_BlockingCall_create_adm` が一致する。例として `struct PeerConnectionFactory` の定義は両ファイルとも次の通り。

```c
struct PeerConnectionFactory {
  struct webrtc_RefCountInterface_ref* ref;
  struct webrtc_Thread_unique* network_thread;
  struct webrtc_Thread_unique* worker_thread;
  struct webrtc_Thread_unique* signaling_thread;
  struct webrtc_PeerConnectionFactoryInterface_refcounted* factory;
};
```

また `PeerConnectionFactory_Create` 本体 (`whip.c:470-553` と `whep.c:339-422`) も一致する。

### URLParts

`whip.c:205-285` と `whep.c:257-337` で、`struct URLParts`、`URLParts_clear`、`URLParts_Parse`、`URLParts_GetPort` が一致する。例として `URLParts_GetPort` は両ファイルとも次の通り。

```c
static const char* URLParts_GetPort(struct URLParts* parts) {
  if (parts->port != NULL && parts->port[0] != '\0') {
    return parts->port;
  }
  if (parts->scheme != NULL && (strcmp(parts->scheme, "wss") == 0 ||
                                strcmp(parts->scheme, "https") == 0)) {
    return "443";
  }
  return "80";
}
```

### find_header_value

`whip_find_header_value` (`whip.c:745-776`) と `whep_find_header_value` (`whep.c:719-750`) は接頭辞以外一致する。

### SendRequest

`whip_SendRequest` (`whip.c:953-1086`) と `whep_SendRequest` (`whep.c:927-1060`) は接頭辞以外ほぼ同一であり、`getaddrinfo` から `SSL_CTX_new` / `SSL_connect` / `SSL_read` ループまでの実装が一致する。

### OnSendRequestResponse

`whip_OnSendRequestResponse` (`whip.c:1088-1240`) と `whep_OnSendRequestResponse` (`whep.c:1062-1215`) は、Link ヘッダのパースと `SetConfiguration` 呼び出しを含めて一致する。差分は `whep.c:1074` の `RTC_LOG_INFO("Received response: %s", resp);` の 1 行程度である。

## 設計方針

- 重複している共通部分（`PeerConnectionFactory` 一式、`URLParts` 一式、`find_header_value`、`SendRequest`、`OnSendRequestResponse` の共通ロジック）を共通ヘッダ・共通実装ファイルへ括り出し、`whip.c` と `whep.c` の双方から利用する。
- 接頭辞の差異のみであれば共通名へ統一する。`whep.c` 固有のログ行などの差分は、共通化後も挙動が変わらないように扱う。
- C ファイルのビルド構成 (`build.rs` などの参照) を確認し、新規ファイルが正しくコンパイル対象になるようにする。

## 完了条件

- 上記の主要な重複（`PeerConnectionFactory`、`URLParts`、`find_header_value`、`SendRequest`、`OnSendRequestResponse`）が共通化され、二重メンテが不要になる。
- `whip` と `whep` の挙動が共通化前と変わらないこと。
