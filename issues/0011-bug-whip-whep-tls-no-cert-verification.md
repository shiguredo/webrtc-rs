# whip/whep の TLS サーバ証明書検証を実装する

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-tls-no-cert-verification

## 目的

whip/whep のシグナリングは TLS 上で SDP・ICE・TURN 資格情報といった機密性の高い情報をやり取りする。
しかし現状の TLS 接続実装はサーバ証明書を一切検証していないため、任意の証明書を提示する中間者と
TLS が成立してしまい、中間者攻撃によってシグナリング内容が漏洩・改竄され得る。これを防ぐため、
正当なサーバ証明書のみを受け入れるように TLS サーバ証明書検証を実装する。

## 優先度根拠

証明書検証の欠如は中間者攻撃を許す重大なセキュリティ欠陥である。シグナリングには TURN の資格情報
など外部へ漏れてはならない情報が含まれるため、影響が大きい。さらに whip.c / whep.c / whip.cpp /
whep.cpp の 4 ファイルすべてに同じ欠陥が存在するため、優先度は High とする。

## 現状

4 ファイルとも `SSL_CTX_new(TLS_client_method())` で SSL コンテキストを作成し、SNI の設定
（`SSL_set_tlsext_host_name`）と `SSL_connect` のみを行っている。`SSL_CTX_set_verify` による
`SSL_VERIFY_PEER` の設定も、CA 証明書ストアのロードも、`SSL_get_verify_result` による検証結果の
確認も、`SSL_set1_host` 等によるホスト名照合も一切行っていない。このため任意の証明書で TLS が
成立する。

webrtc/src/whip.c:995-1034

```c
  SSL_CTX* ctx = SSL_CTX_new(TLS_client_method());
  if (ctx == NULL) {
    RTC_LOG_ERROR("SSL_CTX_new failed");
    close(sock);
    on_response(NULL, user_data);
    return;
  }
  SSL_CTX_set_min_proto_version(ctx, TLS1_2_VERSION);
  SSL_CTX_set_max_proto_version(ctx, TLS1_3_VERSION);
  SSL_CTX_set_options(ctx, SSL_OP_ALL | SSL_OP_NO_SSLv2 | SSL_OP_NO_SSLv3 |
                               SSL_OP_NO_TLSv1 | SSL_OP_NO_TLSv1_1 |
                               SSL_OP_SINGLE_DH_USE);

  SSL* ssl = SSL_new(ctx);
  if (ssl == NULL) {
    RTC_LOG_ERROR("SSL_new failed");
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }

  if (!SSL_set_tlsext_host_name(ssl, host)) {
    RTC_LOG_ERROR("Failed to set SNI");
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }

  SSL_set_fd(ssl, sock);
  if (SSL_connect(ssl) != 1) {
    RTC_LOG_ERROR("SSL_connect failed");
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }
```

webrtc/src/whep.c:969-1008 も同一の処理で、証明書検証が欠落している。

```c
  SSL_CTX* ctx = SSL_CTX_new(TLS_client_method());
  if (ctx == NULL) {
    RTC_LOG_ERROR("SSL_CTX_new failed");
    close(sock);
    on_response(NULL, user_data);
    return;
  }
  SSL_CTX_set_min_proto_version(ctx, TLS1_2_VERSION);
  SSL_CTX_set_max_proto_version(ctx, TLS1_3_VERSION);
```

webrtc/src/whip.cpp:1001-1029 も同様で、`SSL_CTX_set_verify` や検証結果の確認がない。

```cpp
    SSL_CTX* ctx = SSL_CTX_new(TLS_client_method());
    if (!ctx) {
      RTC_LOG(LS_ERROR) << "SSL_CTX_new failed";
      return;
    }
    ScopeExit ssl_ctx_free_guard{[ctx]() { SSL_CTX_free(ctx); }};
    SSL_CTX_set_min_proto_version(ctx, TLS1_2_VERSION);
    SSL_CTX_set_max_proto_version(ctx, TLS1_3_VERSION);
    SSL_CTX_set_options(ctx, SSL_OP_ALL | SSL_OP_NO_SSLv2 | SSL_OP_NO_SSLv3 |
                                 SSL_OP_NO_TLSv1 | SSL_OP_NO_TLSv1_1 |
                                 SSL_OP_SINGLE_DH_USE);

    SSL* ssl = SSL_new(ctx);
    if (!ssl) {
      RTC_LOG(LS_ERROR) << "SSL_new failed";
      return;
    }
    ScopeExit ssl_free_guard{[ssl]() { SSL_free(ssl); }};

    if (!SSL_set_tlsext_host_name(ssl, host.c_str())) {
      RTC_LOG(LS_ERROR) << "Failed to set SNI: ec=" << ERR_get_error();
      return;
    }

    SSL_set_fd(ssl, static_cast<int>(sock));
    if (SSL_connect(ssl) != 1) {
      RTC_LOG(LS_ERROR) << "SSL_connect failed: ec=" << ERR_get_error();
      return;
    }
```

webrtc/src/whep.cpp:739-767 も whip.cpp と同一の処理で、証明書検証が欠落している。

## 設計方針

4 ファイルすべてに対して、以下を TLS 接続前に実装する。

- `SSL_CTX_set_verify` で `SSL_VERIFY_PEER` を設定し、検証失敗時にハンドシェイクを失敗させる
- `SSL_CTX_set_default_verify_paths` 等で CA 証明書ストアをロードする
- `X509_VERIFY_PARAM` / `SSL_set1_host` でホスト名照合を有効にし、SNI に渡したホスト名と
  証明書のホスト名を照合する
- `SSL_connect` 成功後に `SSL_get_verify_result` の結果を確認し、`X509_V_OK` 以外は
  接続を拒否する

C ラッパー（whip.c / whep.c）と C++ アプリケーション（whip.cpp / whep.cpp）で実装の細部は
異なるが、検証ロジックの方針は共通とする。

## 完了条件

- 不正・不一致な証明書を提示するサーバへの TLS 接続が拒否される
- 正当な証明書を提示するサーバとのみ TLS が確立する
- whip.c / whep.c / whip.cpp / whep.cpp の 4 ファイルすべてに証明書検証が適用される
