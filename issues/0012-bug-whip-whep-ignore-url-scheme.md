# whip/whep で URL scheme を尊重して接続方式を切り替える

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Polished: 2026-06-05

## 目的

whip/whep のシグナリング URL には `http://` または `https://` の scheme が指定される。
しかし現状の実装は `SendRequest` が scheme を参照せず、無条件で TLS ハンドシェイクを行う。
このため `http://` を指定すると平文ポートへ TLS ClientHello を送ってしまい接続できない。
scheme を尊重して TLS／平文を切り替えられるようにする。

## 再現手順

1. whip.c / whep.c または whip.cpp / whep.cpp をビルドする
2. signaling_url に `http://` で始まる HTTP サーバの URL を指定する
3. TCP 80 番ポートに TLS ClientHello が送信され、`SSL_connect failed` のエラーで
   接続不能となる

## 優先度根拠

scheme を無視する実装は `http://` 指定時に接続不能となる機能上の欠陥であり、
設定例も全 4 ファイルで `http://` をデフォルト値としているため、
利用者が指定したプロトコルが反映されないことは影響が大きい。優先度は High とする。

## 現状

`URLParts_Parse` / `URLParts::Parse` で URL から scheme を取得しているが、
`SendRequest` 内ではその scheme を参照せず、常に TLS で接続する。
whip.c / whip.cpp / whep.c / whep.cpp の 4 ファイルすべてが同一構造。

```c
// whip.c:953-957 — SendRequest は host と port のみを受け取る
static void whip_SendRequest(const char* host,
                             const char* port,
                             const char* req,
                             void (*on_response)(char* resp, void* user_data),
                             void* user_data) {
```

ポート決定ロジック `URLParts_GetPort` / `GetPort` は既に scheme を参照しているが、
case-sensitive な比較で `wss` / `https` のみを 443 ポートに振り分けている:

```c
// whip.c:276-285
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

## 設計方針

### 接続方式の分岐

`SendRequest` に scheme を引数で渡す。TCP 接続確立後に scheme を検証し分岐する:

| scheme | 接続方式 | デフォルトポート |
|--------|---------|----------------|
| `https` | TLS | 443 |
| `http` | 平文 TCP | 80 |
| その他 | エラー | — |

WHIP/WHEP シグナリングは HTTP/HTTPS ベースのため、`ws`/`wss` は対象外。
未知 scheme の場合はエラーコールバック（C 版: `on_response(NULL, user_data)`、
C++ 版: `on_response(std::nullopt)`）を呼び、接続しない。

### スキームの比較

- RFC 3986 Section 3.1 に基づき、case-insensitive で比較する
- `URLParts_GetPort` / `GetPort` の scheme 比較も case-insensitive に修正し、
  `https` のみを 443 に振り分ける（`wss` 判定は不要のため削除する）

### 変更対象

- `webrtc/src/whip.c` / `whip.cpp` / `whep.c` / `whep.cpp` の 4 ファイル
- `SendRequest` のシグネチャに scheme パラメータを追加する
  - C 版: `const char* scheme` を追加。前方宣言と実装定義の両方
  - C++ 版: `const std::string& scheme` を追加
- 呼び出し元（whip.c:944, whep.c:918, whip.cpp:754, whep.cpp:494）から
  `parts.scheme` を渡すように修正する
- `URLParts_GetPort` / `GetPort` の scheme 比較を修正する

### 平文 TCP 接続時の注意点

- `SSL_CTX_new` から `SSL_connect` までの TLS セットアップブロック全体をスキップする
- `SSL_write` / `SSL_read` を `send()` / `recv()` に置き換える
- C++ 版では `ssl_ctx_free_guard` / `ssl_free_guard` を平文パスでは生成しない
- エラー時はソケットを close してからコールバックすること（C 版では FD リーク防止のため明示的に close が必要）

### 後方互換

- 設定例のデフォルト URL は `http://` のまま変更しない
- `http://` 指定時の動作が「TLS 試行（失敗）」から「平文 TCP 接続」に変わる

### 他 issue との関係

- `issues/pending/0011` も `SendRequest` を改修対象としており、
  本 issue の解決を先行させることが望ましい

## テスト戦略

- **URLParts の scheme パース・ポート選択**: 各ビルドの動作確認用テストコードを
  `main` 関数内に追加し、`http`/`https` の各 scheme・大文字 scheme・明示ポート指定・
  未知 scheme の各ケースでデフォルトポートとエラーの有無を検証する
- **SendRequest の scheme 分岐**: 実際の HTTP/HTTPS サーバとの結合テストで、
  `http://` の平文接続成功、`https://` の TLS 接続成功、未知 scheme のエラー検出を
  確認する

## 完了条件

- `http://` 指定時は平文 TCP で接続し、データ送受信ができる
- `https://` 指定時は TLS で接続し、データ送受信ができる
- scheme に応じてデフォルトポート（http は 80、https は 443）が選択される
- 大文字 scheme（`HTTP`、`Https`、`HTTPS` など）が case-insensitive に処理される
- 不明な scheme（`ftp://`、`ws://`、`wss://` など）はエラーとして扱われる
- `send()` の部分書き込みに対して全データ送信が保証される
- `send()` が SIGPIPE でプロセス終了しない
