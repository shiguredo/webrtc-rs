# whip/whep の C/C++ サンプルに TLS サーバ証明書検証を実装する

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-tls-no-cert-verification
- Polished: 2026-06-05

## pending の理由

C 版（whip.c / whep.c）の CA 証明書供給方法の確定に、外部依存の追加・設計判断が必要なため pending
とする。CA バンドル（PEM）同梱は外部リソース（`https://pki.goog/roots.pem`）の取り込みと
`webrtc/CMakeLists.txt` での配置・実行時パス解決を伴い、OS トラストストア利用はプラットフォーム別
実装とリンク追加を要する。いずれを採るかが実装着手の前提となるため、方針が確定するまで保留する。

## 目的

whip/whep の C/C++ サンプル実装（`webrtc/src/whip.c` / `whep.c` / `whip.cpp` / `whep.cpp` の 4 つの
スタンドアロン実行ファイル）は、TLS 上で SDP・ICE・TURN 資格情報といった機密性の高い情報をやり取り
するにもかかわらず、サーバ証明書を一切検証していない。任意の証明書を提示する中間者と TLS が成立して
しまい、中間者攻撃によってシグナリング内容が漏洩・改竄され得る。正当なサーバ証明書のみを受け入れる
よう、この 4 つのサンプルに TLS サーバ証明書検証を実装する。

Rust 側のサンプル（`examples/whip` / `examples/whep`）は `rustls` + `rustls-platform-verifier`
（`examples/whip/src/main.rs` の `ClientConfig::with_platform_verifier()`）で既に証明書を検証して
おり、本 issue の対象外である。本 issue は C/C++ サンプルを Rust サンプルと同等の安全性に揃える。

## 優先度根拠

High とする。証明書検証の欠如は「TLS で接続しているのに中間者攻撃を許す」典型的なセキュリティ欠陥で
あり、安全に見えて安全でない点が危険である。これらは WHIP/WHEP クライアントのリファレンスとなる
サンプルであり、利用者が自身の実装の雛形としてコピー・流用する起点になりやすい。検証を欠いた TLS
実装が雛形として広まると、各利用者の実装にも同じ欠陥が持ち込まれる。一方で修正は検証処理の追加のみで
完結しコストは低い。放置する利点がなく優先的に修正すべきである。

ただし影響範囲は次の通り限定される。

- 対象は C/C++ のサンプル実行ファイルであり、webrtc-rs が提供するライブラリ本体や Rust サンプルは
  影響しない。
- デフォルト接続先が到達不能なドキュメント用アドレス（`http://192.0.2.1/whip` 等、RFC 5737）の
  ため実害は顕在化していないが、サンプルは scheme に関わらず常に TLS ハンドシェイクを行う（現状を
  参照）ため、利用者が到達可能な実エンドポイントへ向けた瞬間に未検証の TLS が成立する。

## 現状

4 ファイルとも libwebrtc 同梱の BoringSSL（`webrtc/CMakeLists.txt` の
`third_party/boringssl/src/include`、`OPENSSL_IS_BORINGSSL` を確認済み）を `<openssl/ssl.h>` 経由で
利用する。OpenSSL 互換の API だが、デフォルト挙動は OpenSSL と異なる（設計方針を参照）。

`SSL_CTX_new(TLS_client_method())` で SSL コンテキストを作成し、SNI の設定
（`SSL_set_tlsext_host_name`、どのホスト宛かをサーバへ伝えるのみで証明書検証には寄与しない）と
`SSL_connect` のみを行っている。証明書検証に必要な処理が一切ない。具体的には、`SSL_CTX_set_verify`
による `SSL_VERIFY_PEER` の設定、CA 証明書ストアのロード、`SSL_get_verify_result` による検証結果の
確認、`SSL_set1_host` 等によるホスト名照合のいずれも行っていない（4 ファイルを grep し、これらの
シンボルが 0 件であることを確認済み）。このため任意の証明書で TLS が成立する。

TLS は scheme に関わらず常に実行される。`whip_SendRequest`（`whip.c:953`）は `host` と `port` のみ
受け取り、平文と TLS を切り替える分岐を持たない。`getaddrinfo` → `connect` の後、無条件に
`SSL_CTX_new` / `SSL_connect` を実行する（scheme はデフォルトポートを 80 / 443 のどちらにするかの
決定にのみ使われ、デフォルトの `http://192.0.2.1` ではポート 80 へ TLS ハンドシェイクを投げる）。
各ファイルの TLS 接続箇所は 1 関数内の 1 箇所のみで、リダイレクトや DELETE 時の TLS 再接続は存在
しない（`SSL_CTX_new` の呼び出しは各ファイル 1 箇所）。修正対象は明確である。

- `webrtc/src/whip.c:995-1034`（`whip_SendRequest` 内）
- `webrtc/src/whep.c:969-1008`（`whep_SendRequest` 内）
- `webrtc/src/whip.cpp:1001-1029`（`SendRequest` 内）
- `webrtc/src/whep.cpp:739-767`（`SendRequest` 内）

検証欠如を示す骨格は次の通り（whip.c。エラーハンドリングは省略）。

```c
  SSL_CTX* ctx = SSL_CTX_new(TLS_client_method());
  SSL_CTX_set_min_proto_version(ctx, TLS1_2_VERSION);
  SSL_CTX_set_max_proto_version(ctx, TLS1_3_VERSION);
  SSL_CTX_set_options(ctx, SSL_OP_ALL | SSL_OP_NO_SSLv2 | ...);
  SSL* ssl = SSL_new(ctx);
  SSL_set_tlsext_host_name(ssl, host);  // SNI のみ。検証には寄与しない
  SSL_set_fd(ssl, sock);
  SSL_connect(ssl);  // 証明書を検証せず接続を確立している
```

残り 3 ファイルも同型で、証明書検証が欠落している。

## 設計方針

実装は (1) CA 供給方法の確定、(2) 検証の主軸（証明書検証とホスト名照合）と補助確認の実装、
(3) エッジケース・ログ対応、の順で進める。実体は BoringSSL であるため OpenSSL とは挙動が異なる。

### (1) CA 証明書ストアのロード（着手前に方針を確定する）

BoringSSL では `SSL_CTX_set_default_verify_paths` が OpenSSL と異なり信頼できない（ヘッダにも
"There is no corresponding concept for BoringSSL"・"not recommended" と明記）。これが参照する環境変数
`SSL_CERT_FILE` / `SSL_CERT_DIR` も、パスを明示的に渡す `SSL_CTX_load_verify_locations` では参照
されない。そのため CA ルートを明示的に供給する必要がある。C 版と C++ 版で利用できる手段が異なる。

- C++ 版（whip.cpp / whep.cpp）: libwebrtc 同梱の
  `webrtc::openssl::LoadBuiltinSSLRootCertificates(ctx)`（`rtc_base/openssl_utility.h`、include の
  追加が必要）を呼ぶ。これは組み込みルート CA（`rtc_base/ssl_roots.h`、36 個）を `ctx` の
  `X509_STORE` に登録するワンコールのローダで、libwebrtc 本体をリンク済みの C++ 版からはこの 1 行で
  済む。ただし (a) 組み込みルートは 2023-05-09 のスナップショット固定で更新されず一部ルートは既に
  期限切れ、(b) この関数は `WEBRTC_EXCLUDE_BUILT_IN_SSL_ROOT_CERTS` が定義されたビルドでは存在しない
  （本リポジトリの libwebrtc ビルドでは未定義で利用可能）というトレードオフがある。
- C 版（whip.c / whep.c）: 上記ローダも `ssl_roots.h`（`<cstddef>` を含む C++ ヘッダ）も C++ シンボル
  のため C コンパイル単位からは利用できない。CA バンドル（PEM）をリポジトリに同梱して
  `SSL_CTX_load_verify_locations` でパス指定して読む（PEM は `ssl_roots.h` と同じ
  `https://pki.goog/roots.pem` を取得して用意できる。この場合は `webrtc/CMakeLists.txt` で PEM の
  配置・インストールと、実行時のパス解決方針も決める）か、OS のトラストストアを使う（後者は macOS /
  iOS は Security framework、Windows は CryptoAPI とプラットフォーム別実装が必要で、
  `webrtc/CMakeLists.txt` の各ターゲットへ `-framework Security` / `crypt32.lib` 等のリンク追加も要る）。

C 版の最新性・保守性と C++ 版との一貫性（4 ファイルで方針を揃えるか否か）を踏まえ、着手前に確定する。

### (2) 検証の主軸（必須）と補助確認

- `SSL_CTX_set_verify(ctx, SSL_VERIFY_PEER, NULL)` を設定し、検証失敗時に `SSL_connect` 自体を
  失敗させる。これが主たる防御線である。`SSL_VERIFY_PEER` を設定しないと検証がスキップされ、
  `SSL_get_verify_result` は常に `X509_V_OK` を返すため、検証結果の事後確認だけに頼ってはならない。
- ホスト名照合を有効にする。SNI（`SSL_set_tlsext_host_name`、既存）とは別に、`SSL_set1_host(ssl, host)`
  で照合対象のホスト名を設定する。両者は別 API であり、両方の設定が必要である。さらに BoringSSL の
  `SSL_set1_host` はデフォルトで非推奨の CN も照合するため、
  `SSL_set_hostflags(ssl, X509_CHECK_FLAG_NEVER_CHECK_SUBJECT)` を併用して SAN のみを照合する。
- 補助確認として `SSL_connect` 成功後に `SSL_get_verify_result` の結果を確認し、`X509_V_OK` 以外は
  接続を拒否する。

挿入位置は、`SSL_CTX_set_options(...)` の直後に `SSL_CTX_set_verify` と CA ストアのロード、
`SSL_set_tlsext_host_name(...)` の直後に `SSL_set1_host` と `SSL_set_hostflags`、`SSL_connect` 成功
直後に `SSL_get_verify_result` の確認を追加する。既存の
`SSL_CTX_set_min/max_proto_version`・`SSL_CTX_set_options` は変更しない。

### (3) エッジケースとログ

- IP アドレスへの直接接続は本 issue では非対応とする。`SSL_set1_host` は DNS 名照合のため IP では
  成立せず、IP 照合には別 API（`X509_VERIFY_PARAM_set1_ip` 系）が必要でスコープを超える。今回は
  接続先が IPv4 リテラルの場合は TLS を試みず明示的にエラーとする（IP 対応が必要になれば別 issue）。
  判定は各 `SendRequest` / `whip_SendRequest` の冒頭（`getaddrinfo` 呼び出し前）で
  `inet_pton(AF_INET, host, ...)` を用いる。`inet_pton` の宣言には `<arpa/inet.h>` の追加 include が
  必要である（4 ファイルとも現状は未 include。Windows の C++ 版は `<ws2tcpip.h>` で提供済みのため
  追加不要）。IPv6 リテラルについては、4 ファイルの URL パーサ（C 版 `URLParts_Parse`・C++ 版
  `URLParts::Parse` とも最初の `:` で host/port を分割する）がブラケット表記 `[::1]` を正しく扱えず
  host を破壊するため、`getaddrinfo` が失敗して `SSL_set1_host` に到達せず未検証 TLS は成立しない。
  ただしこの安全性はパーサの副作用に依存するため、将来 URL パーサが IPv6 に対応する場合は
  `inet_pton(AF_INET6, ...)` による判定追加が必要になる。
- デフォルト接続先は `http://192.0.2.1/whip` 等（IP リテラル・平文 scheme）であり、IP リテラルを
  明示エラーとする本変更後はそのままでは起動時にエラーで終了する。本 issue でデフォルト接続先を
  ドキュメント用ホスト名の `https` URL（例: `https://example.com/whip`、RFC 2606）に変更する。
  利用者はこれを実エンドポイントへ書き換えて使う。
- 検証失敗を通常の接続失敗と区別できるよう、`SSL_get_verify_result` の値を
  `X509_verify_cert_error_string` で文字列化して英語ログに出力する。検証分岐で early return する際、
  C++ 版は既存の `ScopeExit` が SSL リソースを解放するが、C 版は各 return で手動解放（`SSL_free` /
  `SSL_CTX_free` / `close`）の追記が必要である。

### 関連 issue との関係

- 既存の `SSLCertificateVerifier` / `set_tls_cert_verifier`（CHANGES.md に記載）は DTLS /
  PeerConnection（メディア）経路の証明書検証であり、本 issue が対象とするシグナリングの生 BoringSSL
  経路とは別物で流用できない。
- 0019（`SSL_read` / `SSL_write` の戻り値処理）と 0034（接続タイムアウト）は同じ `SendRequest` 内の
  同じ SSL ブロックを改変するため、実装順序によっては競合する。
- 0052 / 0053（whip/whep の C / C++ 重複排除）が `SendRequest` の共通化を扱うが、本 issue は番号順・
  High 優先の原則上それらを待たずに先行してよい。先に共通化されていれば、共通化箇所に検証を追加する。

## テスト戦略

CLAUDE.md はモック・スタブを禁止する。また `WHIP_ENDPOINT` / `whip_client` の結合テスト枠組みは
Rust 側 `examples/whip` 専用であり、本 issue の C/C++ サンプルには適用されない。これらのサンプルは
`int main()` の手動実行バイナリで接続先がハードコードされており、自動テストの枠組みを持たない。
したがって検証は手動で行う。デフォルト接続先（上記で `https` のホスト名に変更したもの）を確認対象に
書き換えてビルド・実行し、以下を確認する。

- 正当な証明書を提示する実 WHIP/WHEP エンドポイント（`https`）へ接続し、TLS が確立してシグナリング
  が成功する。
- 不正な証明書を提示するローカル TLS サーバへ接続し、`SSL_connect` が失敗して接続が拒否され、検証
  失敗である旨が英語ログに出力される。確認すべき不正ケースは、自己署名・CA 不明・ホスト名不一致・
  期限切れ・SAN を持たず CN のみ一致（CN フォールバックが無効であることの確認）とする。不正サーバは
  `openssl` 等で各ケースの証明書を生成し、それを提示する簡易 TLS サーバを立てて用意する。

確認手順とログ抜粋を PR 説明に記録する際は、接続先の実 URL・ホスト名・資格情報を残さず、プレースホルダ
（`<WHIP_ENDPOINT>` 等）に置き換える。

## 完了条件

- テスト戦略に挙げた正常系（正当な証明書で TLS 確立）および各不正ケース（自己署名・CA 不明・ホスト名
  不一致・期限切れ・SAN を持たず CN のみ一致）の確認項目をすべて満たす。
- 接続先が IPv4 リテラルの場合は TLS を試みず明示的にエラーとなる。
- IP リテラル非対応化に伴い、サンプルのデフォルト接続先が、IPv4 リテラルの平文 URL からドキュメント用
  ホスト名の `https` URL に変更されている。
- whip.c / whep.c / whip.cpp / whep.cpp の 4 ファイルすべてに証明書検証が適用される。
- 検証失敗時に、通常の接続失敗と区別できる英語ログが出力される。
- 手動確認手順とログ抜粋が、機密情報をプレースホルダ化したうえで PR 説明に記録されている。
