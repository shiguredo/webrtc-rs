# whip/whep で HTTP ステータスコードを検証する

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-http-status-not-checked

## 目的

WHIP / WHEP のシグナリングで、サーバから返ってきた HTTP レスポンスのステータスコードを一切検証していない。WHIP は `201 Created` を期待する仕様だが、現在の実装は `4xx` / `5xx` などの失敗レスポンスであってもボディを SDP やヘッダとして処理し続けてしまう。これにより、エラーボディ（HTML のエラーページや JSON のエラーメッセージ等）を SDP や Link ヘッダとして誤って解釈し、誤動作や無言の失敗を招く。レスポンスのステータスコードを検証し、期待した成功ステータス以外は明確にエラーとして扱うことで、シグナリングの堅牢性を高める。

## 優先度根拠

High。サーバ側が認証エラー・レート制限・内部エラーなどで非成功ステータスを返した場合に、エラーを検知できず誤った SDP 解析へ進む。失敗が無言のまま進行して原因究明が困難になるうえ、攻撃者が制御するエンドポイントに対しては誤ったボディ解釈の入り口にもなる。シグナリングの正しさに直結するため優先度を高くする。

## 現状

`webrtc/src/whip.c` の `whip_OnSendRequestResponse` は、レスポンス文字列からヘッダ終端 `\r\n\r\n` を探してヘッダとボディに分割しているが、ステータスラインを一切パースしていない。`webrtc/src/whip.c:1100` 付近:

```c
char* header_end = strstr(resp, "\r\n\r\n");
if (header_end == NULL) {
  RTC_LOG_ERROR("Invalid response");
  free(resp);
  SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
  webrtc_SessionDescriptionInterface_unique_delete(desc);
  free(ctx);
  return;
}

size_t header_len = (size_t)(header_end - resp);
char* headers = (char*)malloc(header_len + 1);
```

このようにヘッダ終端の有無だけを確認し、その後すぐ Link ヘッダの抽出と SDP 処理に進んでいる。ステータスコードを参照する箇所はファイル全体に存在しない（`grep` で `HTTP/1`・`status` を検索してもステータスラインを解釈する処理は無い）。

`webrtc/src/whep.c` の `whep_OnSendRequestResponse` も同一構造で、`webrtc/src/whep.c:1076` 付近に同じく `strstr(resp, "\r\n\r\n")` でヘッダ終端を探すだけの実装がある:

```c
char* header_end = strstr(resp, "\r\n\r\n");
if (header_end == NULL) {
  RTC_LOG_ERROR("Invalid response");
  free(resp);
  SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
  webrtc_SessionDescriptionInterface_unique_delete(desc);
  free(ctx);
  return;
}
```

C++ 版の `webrtc/src/whip.cpp` でも、`SendRequest` のレスポンスコールバック（`webrtc/src/whip.cpp:772` 付近）でヘッダ終端を探したのち、ステータスを見ずにヘッダ行のパースへ進んでいる:

```cpp
auto n = resp->find("\r\n\r\n");
if (n == std::string::npos) {
  RTC_LOG(LS_ERROR) << "Invalid response";
  return;
}
auto header_str = resp->substr(0, n);
body = resp->substr(n + 4);
std::vector<std::string> lines =
    absl::StrSplit(header_str, "\r\n");
```

`webrtc/src/whep.cpp` のレスポンス処理も同様にステータスを検証していない。

## 設計方針

- レスポンスの先頭にあるステータスライン（`HTTP/1.1 <status-code> <reason-phrase>`）をパースし、ステータスコードを取り出す。
  - ステータスラインは最初の `\r\n` までを対象とし、`HTTP/` で始まることを確認したうえで、バージョントークンの次の空白区切りのトークンを 3 桁の数値として読む。
- WHIP / WHEP の成功時に期待するステータスコード以外（特に `4xx` / `5xx`）を受け取った場合は、ボディを SDP として解釈せず、明確にエラーとして扱って状態を `CLOSED` に遷移させる。
  - WHIP のリソース生成は `201 Created` を期待する。期待コードの厳密な範囲は WHIP / WHEP の各仕様に合わせる。
- ステータスラインが不正（`HTTP/` で始まらない、コード部分が数値でない等）な場合も不正レスポンスとしてエラーにする。
- C 版（`whip.c` / `whep.c`）と C++ 版（`whip.cpp` / `whep.cpp`）の双方に同等の検証を入れる。
- ログメッセージ・エラーメッセージは英語で記述する。

## 完了条件

- 非成功ステータス（例: `4xx` / `5xx`）のレスポンスを受け取った場合に、SDP 解析へ進まず明確にエラーを返し、状態が `CLOSED` に遷移する。
- ステータスラインが不正なレスポンスもエラーとして扱われる。
- 期待する成功ステータス（WHIP のリソース生成では `201 Created`）の場合のみ、従来どおりボディの処理に進む。
- C 版・C++ 版の双方で同じ検証が行われる。
