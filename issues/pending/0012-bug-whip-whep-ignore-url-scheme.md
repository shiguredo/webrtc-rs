# whip/whep で URL scheme を尊重して接続方式を切り替える

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-whep-ignore-url-scheme
- Polished: 2026-08-05

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

whip/whep のシグナリング URL には `http://` または `https://` の scheme が指定され得る。
しかし現状の実装は `SendRequest` が scheme を参照せず、無条件で TLS ハンドシェイクを行う。
このため `http://` を指定すると平文ポートへ TLS ClientHello を送ってしまい接続できない。
scheme を尊重して TLS／平文を切り替えられるようにする。

## 再現手順

1. `webrtc/src/whip.c` / `whep.c` または `whip.cpp` / `whep.cpp` をビルドする
2. `main()` 内の signaling_url を `http://` で始まる実 HTTP サーバの URL に
   書き換えて再ビルドする
3. TCP 80 番ポートに TLS ClientHello が送信され、`SSL_connect failed` のエラーで
   接続不能となる

## 優先度根拠

現状のデフォルト URL は全 4 ファイルで `http://` のため、利用者が指定したプロトコルが
反映されないことは影響が大きい。優先度は High とする。

## 現状

`URLParts_Parse` / `URLParts::Parse` で URL から scheme を取得しているが、
`SendRequest` 内ではその scheme を参照せず、常に TLS で接続する。
`webrtc/src/whip.c` / `whip.cpp` / `whep.c` / `whep.cpp` の 4 ファイルすべてが
同一構造。

```c
// whip_SendRequest は host と port のみを受け取り、scheme を受け取らない
static void whip_SendRequest(const char* host,
                             const char* port,
                             const char* req,
                             void (*on_response)(char* resp, void* user_data),
                             void* user_data) {
```

ポート決定ロジック `URLParts_GetPort` / `GetPort` は既に scheme を参照しているが、
case-sensitive な比較で `wss` / `https` のみを 443 ポートに振り分けている:

```c
// URLParts_GetPort
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

`SendRequest` に scheme を引数で渡す。接続先ポートは従来どおり呼び出し元の
`URLParts_GetPort` / `GetPort` が決定して `port` 引数で渡す（ポート決定の責務は
変更しない）。`SendRequest` 内では getaddrinfo / connect の前に scheme を検証して
接続方式を切り替える:

| scheme | 接続方式 | デフォルトポート |
|--------|---------|----------------|
| `https` | TLS | 443 |
| `http` | 平文 TCP | 80 |
| その他 | エラー | — |

WHIP/WHEP シグナリングは HTTP/HTTPS ベースのため、`ws`/`wss` は対象外。
未知 scheme の場合は接続前にエラーとして扱い、コールバックが発火して接続しない
（C 版: `on_response(NULL, user_data)` を呼ぶ。C++ 版は `on_response_guard` 経由で
`on_response(std::nullopt)` が呼ばれる）。未知 scheme でも呼び出し元の
`URLParts_GetPort` / `GetPort` は呼ばれるが、その戻り値はエラー分岐で破棄される。
エラー時は `Unsupported scheme` 等の英語ログを出力してからコールバックする。

C++ 版は `on_response_guard`（ScopeExit）が関数終了時に自動で `on_response` を呼ぶ
構造のため、未知 scheme パスでは明示的に `on_response` を呼ばず return するだけでよい
（明示呼び出しとガードの重複は issues/0023 の二重呼び出しバグと同じパターンになるため
避ける）。未知 scheme の判定は `on_response_guard` 生成後に置くこと（ガード生成前に
return するとガードが未生成のため `on_response` が呼ばれない）。

### スキームの比較

- RFC 3986 Section 3.1 に基づき、case-insensitive で比較する
- `URLParts_GetPort` / `GetPort` の scheme 比較も case-insensitive に修正し、
  `https` のみを 443 に振り分ける（`wss` 判定は不要のため削除する）
- C++ 版 `GetPort` 直上のコメント（`// scheme が https/wss の場合は 443、それ以外の
  場合は 80 を返す`）も `https` のみを 443 に振り分ける旨へ更新する

### 変更対象

- `webrtc/src/whip.c` / `whip.cpp` / `whep.c` / `whep.cpp` の 4 ファイル
- `SendRequest` のシグネチャに scheme パラメータを追加する
  - C 版: `const char* scheme` を追加。前方宣言と実装定義の両方
  - C++ 版: `const std::string& scheme` を追加
- 呼び出し元（各ファイルの `URLParts_Parse` / `URLParts::Parse` 呼び出し以降）から
  `parts.scheme` を渡すように修正する
- `URLParts_GetPort` / `GetPort` の scheme 比較を修正する

### 平文 TCP 接続時の注意点

- `SSL_CTX_new` から `SSL_connect` までの TLS セットアップブロック全体をスキップする
- `SSL_write` / `SSL_read` を `send()` / `recv()` に置き換える
- `send()` は部分書き込みが発生し得るため、未送信バイトが無くなるまでループで送信する
- `send()` は SIGPIPE でプロセスを終了させ得るため、通信先が閉じたソケットへ送っても
  プロセスが終了しないようにする（Linux / Android は `send()` の flags に
  `MSG_NOSIGNAL` を指定し、macOS / iOS は `setsockopt()` で `SO_NOSIGPIPE` を設定する。
  Windows は SIGPIPE が無いため不要）
- C++ 版では `ssl_ctx_free_guard` / `ssl_free_guard` を平文パスでは生成しない
- エラー時はソケットを close してからコールバックすること（C 版では FD リーク防止のため
  明示的に close が必要）

### 後方互換

- 設定例のデフォルト URL は本 issue では変更しない（`http://` のまま）。
  デフォルト URL の `https` 化は pending の issues/0011 が担う
- `http://` 指定時の動作が「TLS 試行（失敗）」から「平文 TCP 接続」に変わる
- `wss://` 指定時の動作が「443 への TLS 接続（偶発的に機能）」から「未知 scheme として
  エラー」に変わる

### 他 issue との関係

`SendRequest` を同じように改修する issue が複数ある。

- `issues/pending/0011`（TLS 証明書検証）: 本 issue を先に解決してから 0011 へ取り組む
- `issues/0013`（URLParts_Parse の strndup null deref）: URLParts を先に安全化してから
  本 issue の拡張を行う
- `issues/0019`（SSL_read / SSL_write の戻り値処理）: 同一の SSL ブロックを改変するため
  実装順序によっては競合する
- `issues/0023`（on_response 二重呼び出し）: C++ 版の `on_response_guard` と関連

## テスト戦略

- **URLParts の scheme パース・ポート選択**: 各ビルドの動作確認用テストコードを
  `main` 関数内に追加し、`http`/`https` の各 scheme・大文字 scheme・明示ポート指定の
  各ケースでポート選択を検証する（明示ポート指定時はデフォルトポートが使われず明示値が
  優先されることを確認する。未知 scheme のエラーは `SendRequest` 側の検証であり、
  URLParts レベルでは発生しないため、結合テストで検証する）
- **SendRequest の scheme 分岐**: 実際の HTTP/HTTPS サーバとの結合テストで、
  `http://` の平文接続成功、`https://` の TLS 接続成功、大文字 scheme の分岐を
  確認する。未知 scheme のエラー検出は接続を伴わないため、実サーバを立てずに
  `SendRequest` の実フローを実行して確認する
- **平文パスの送信**: 次の 2 シナリオに分けて確認する
  - 部分書き込み: サーバが読み取らず送信バッファを詰まらせる構成で、
    `send()` の部分書き込みに対して全データ送信が保証されることを確認する
  - SIGPIPE: クライアント送信後にサーバが close して RST を発生させる構成で、
    `send()` でプロセスが終了しないことを確認する
- `main` 関数は実接続フローを実行するため、テストコードはフラグ分岐などで
  実フローと共存させる

## 完了条件

- `http://` 指定時は平文 TCP で接続し、データ送受信ができる
- `https://` 指定時は TLS で接続し、データ送受信ができる
- scheme に応じてデフォルトポート（http は 80、https は 443）が選択される
- 大文字 scheme（`HTTP`、`Https`、`HTTPS` など）が case-insensitive に処理される
- 不明な scheme（`ftp://`、`ws://`、`wss://` など）はエラーとして扱われる
- `send()` の部分書き込みに対して全データ送信が保証される
- `send()` が SIGPIPE でプロセス終了しない
