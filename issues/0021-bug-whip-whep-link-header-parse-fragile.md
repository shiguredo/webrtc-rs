# whip/whep の Link ヘッダ解析を堅牢化する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-link-header-parse-fragile

## 目的

WHIP / WHEP では ICE サーバ情報（TURN サーバの URL・ユーザー名・クレデンシャル）を HTTP の `Link` ヘッダで運ぶ。現在の実装は単一の `Link` ヘッダ値しか参照しておらず、レスポンスに `Link` ヘッダが複数行存在する場合に 2 行目以降を取りこぼす。`Link` ヘッダはレスポンス内に複数回出現し得るため、すべての `Link` ヘッダを走査し、各エントリの `rel` パラメータごとに正しく ICE サーバ情報を抽出できるようにして、ICE サーバ情報の取得を堅牢化する。

## 優先度根拠

Medium。`Link` ヘッダが 1 行にカンマ区切りで複数エントリをまとめている構成では現状でも動くが、サーバがエントリごとに別々の `Link` ヘッダ行として返す構成では 1 行目しか取得できず、一部の TURN サーバ情報が欠落する。ICE 接続の確立性に影響するが、即時クラッシュではないため Medium とする。

## 現状

`webrtc/src/whip.c` の `whip_OnSendRequestResponse` は、`whip_find_header_value` で `Link` ヘッダを 1 つだけ取得し、その値をカンマで `strtok_r` 分割して各エントリを処理している。`webrtc/src/whip.c:1124` 付近:

```c
char* link_header = whip_find_header_value(headers, "link");
free(headers);
if (link_header == NULL) {
  RTC_LOG_ERROR("No Link header");
  ...
  return;
}

struct webrtc_PeerConnectionInterface_IceServer* server =
    webrtc_PeerConnectionInterface_IceServer_new();
struct std_string_vector* urls =
    webrtc_PeerConnectionInterface_IceServer_get_urls(server);

char* saveptr = NULL;
char* token = strtok_r(link_header, ",", &saveptr);
while (token != NULL) {
  char* url_start = strchr(token, '<');
  char* url_end = strchr(token, '>');
  if (url_start != NULL && url_end != NULL && url_end > url_start + 1) {
    char* url_cstr =
        strndup(url_start + 1, (size_t)(url_end - url_start - 1));
    ...
  }
  char* username_pos = strstr(token, "username=\"");
  ...
  char* credential_pos = strstr(token, "credential=\"");
  ...
  token = strtok_r(NULL, ",", &saveptr);
}
```

ここには 2 つの脆弱性がある。

- `whip_find_header_value(headers, "link")` は最初に一致した `Link` ヘッダ 1 行しか返さないため、`Link` ヘッダが複数行ある場合は 2 行目以降のエントリを取得できない（`whip_find_header_value` の単一ヘッダ取得は別 issue で扱う）。
- 各エントリの抽出を `strchr` / `strstr` と `strtok_r` のカンマ分割で行っているが、`rel` パラメータ（`rel="ice-server"` 等）を確認せずに `<...>` の中身を URL として取り込んでいる。`Link` ヘッダのコメント（`webrtc/src/whip.cpp:793` 付近）にもあるとおり、エントリは `<...>; rel="ice-server"; username="..."; credential="..."` の形式で、`rel` ごとに意味が異なる。

`webrtc/src/whep.c` も同一構造で、`webrtc/src/whep.c:1099` 付近に同じく単一の `Link` ヘッダ取得とカンマ分割がある:

```c
char* link_header = whep_find_header_value(headers, "link");
free(headers);
if (link_header == NULL) {
  RTC_LOG_ERROR("No Link header");
  ...
  return;
}
...
char* saveptr = NULL;
char* token = strtok_r(link_header, ",", &saveptr);
while (token != NULL) {
  char* url_start = strchr(token, '<');
  char* url_end = strchr(token, '>');
  ...
  token = strtok_r(NULL, ",", &saveptr);
}
```

C++ 版の `webrtc/src/whip.cpp` でも、`headers["link"]` で単一の `Link` ヘッダ値を取り出してから `absl::StrSplit(link, ",")` でカンマ分割している。`webrtc/src/whip.cpp:795` 付近:

```cpp
auto link = headers["link"];
if (link.empty()) {
  RTC_LOG(LS_ERROR) << "No Link header";
  return;
}
std::vector<std::string> strs = absl::StrSplit(link, ",");

webrtc::PeerConnectionInterface::IceServer server;
for (const auto& str : strs) {
  std::smatch m;
  if (!std::regex_search(str.begin(), str.end(), m,
                         std::regex(R"(<([^>]+)>)"))) {
    ...
  }
  server.urls.push_back(m[1].str());
  ...
}
```

`webrtc/src/whep.cpp` も同様に単一の `Link` ヘッダ値しか参照していない。

## 設計方針

- レスポンス中のすべての `Link` ヘッダ行を走査し、各行のエントリを順に処理する。`find_header_value` 側で複数ヘッダ行を取得できるようにする対応は別 issue（`find_header_value` の堅牢化）と整合させる。
- 各 `Link` エントリの `rel` パラメータを確認し、ICE サーバを表すエントリ（`rel="ice-server"`）のみを ICE サーバ情報として取り込む。
- 各エントリから URL（`<...>` の中身）、`username`、`credential` を抽出する。`Link` ヘッダ値にカンマがクォート内に含まれ得る点（URL のクエリ等）に留意し、エントリ分割が誤らないようにする。
- C 版（`whip.c` / `whep.c`）と C++ 版（`whip.cpp` / `whep.cpp`）の双方で同等の解析を行う。
- ログメッセージ・エラーメッセージは英語で記述する。

## 完了条件

- `Link` ヘッダが複数行に分かれているレスポンスから、すべての ICE サーバ情報（URL・ユーザー名・クレデンシャル）を取得できる。
- 1 行にカンマ区切りで複数エントリがまとめられているレスポンスからも、従来どおりすべてのエントリを取得できる。
- `rel="ice-server"` のエントリを ICE サーバ情報として正しく扱える。
- C 版・C++ 版の双方で同じ解析が行われる。
