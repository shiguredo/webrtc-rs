# whip/whep が ICE/TURN 資格情報をログ出力しないようにする

- Priority: Medium
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

whip / whep のサンプルが、シグナリングのフルレスポンス (ICE サーバ情報や SDP を含む) や TURN のパスワードをそのままログ出力している箇所を修正し、実行時ログに ICE / TURN の資格情報が露出しないようにする。ソースコード上に資格情報の実値は含まれていないが、ログ設計として資格情報を平文で出力するのは不適切であるため、これを是正する。

## 優先度根拠

Medium とする。クラッシュやメモリ破壊を引き起こすものではないが、TURN のパスワードや ICE 資格情報が実行時ログに平文で残ると、ログの共有・保存を通じて資格情報が漏洩するリスクがある。サンプルコードであっても、利用者がそのままログを残す可能性があり、資格情報の露出は是正すべき問題であるため Medium とする。

## 現状

### whep.c のフルレスポンス出力 (`webrtc/src/whep.c`)

`whep_OnSendRequestResponse` は、受信したレスポンス全体を `RTC_LOG_INFO` でそのまま出力している (`webrtc/src/whep.c:1074`)。

```c
  RTC_LOG_INFO("Received response: %s", resp);
```

このレスポンスには ICE サーバ情報を含む `Link` ヘッダや SDP が含まれるため、ICE / TURN の資格情報がログに出力され得る。

### whip.cpp の TURN パスワード出力 (`webrtc/src/whip.cpp`)

`whip.cpp` は、レスポンスから抽出した TURN サーバの URL・ユーザー名・パスワードを `RTC_LOG(LS_INFO)` でそのまま出力している (`webrtc/src/whip.cpp:829-831`)。

```cpp
                      RTC_LOG(LS_INFO) << "Server: url=" << server.urls.back()
                                       << ", username=" << server.username
                                       << ", password=" << server.password;
```

`server.password` には抽出した TURN の資格情報 (credential) が入るため、これが平文でログに残る。

### whep.cpp のフルレスポンスと TURN パスワード出力 (`webrtc/src/whep.cpp`)

`whep.cpp` も、受信レスポンス全体を `RTC_LOG(LS_INFO)` で出力している (`webrtc/src/whep.cpp:507`)。

```cpp
                    RTC_LOG(LS_INFO) << "Received response: " << *resp;
```

加えて、抽出した TURN サーバのパスワードを平文で出力している (`webrtc/src/whep.cpp:569-571`)。

```cpp
                      RTC_LOG(LS_INFO) << "Server: url=" << server.urls.back()
                                       << ", username=" << server.username
                                       << ", password=" << server.password;
```

## 設計方針

- 資格情報を含むフィールド (TURN の `password` / `credential`、ICE サーバ情報を含むレスポンス全体や SDP) はログから除外するか、値をマスクして出力する。
- フルレスポンスをそのまま出力している箇所 (`webrtc/src/whep.c:1074`, `webrtc/src/whep.cpp:507`) は、デバッグに必要な最小限の情報 (ステータスや非機密のヘッダ等) に絞るか、資格情報を含む部分をマスクする。
- TURN パスワードを直接出力している箇所 (`webrtc/src/whip.cpp:829-831`, `webrtc/src/whep.cpp:569-571`) は、`password` をログから外すか固定のマスク文字列に置き換える。URL・ユーザー名のうち、資格情報に該当する部分があればあわせて配慮する。
- ログメッセージは引き続き英語で記述する。

## 完了条件

- 実行時ログに ICE / TURN の資格情報 (TURN パスワード、資格情報を含むフルレスポンスや SDP) が平文で出力されない。
- whip / whep の C 実装・C++ 実装の双方で、資格情報がログから除外またはマスクされている。
