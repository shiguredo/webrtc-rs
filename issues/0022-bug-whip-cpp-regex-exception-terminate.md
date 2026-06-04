# whip.cpp の std::regex 例外で terminate しないようにする

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-cpp-regex-exception-terminate

## 目的

`webrtc/src/whip.cpp` の `SendRequest` のレスポンスコールバックは、サーバから受け取った HTTP レスポンス（攻撃者が制御し得る入力）を `std::regex` に直接渡してヘッダ行や `Link` ヘッダの各エントリを解析している。`std::regex` のコンストラクタや `std::regex_match` / `std::regex_search` は、入力やパターンの複雑さによっては `std::regex_error`（あるいは内部の再帰によるスタック枯渇）を送出し得る。これらの呼び出しは `try` / `catch` で保護されておらず、例外が送出されると `std::terminate` でプロセスが落ちる。攻撃者由来のレスポンスでプロセスを終了させられないよう、regex 使用箇所を例外安全にする（または regex を使わない堅実なパーサに置換する）。

## 優先度根拠

High。WHIP のシグナリング相手（あるいは中間者）が細工したレスポンスを返すだけで `std::terminate` を誘発し、プロセス全体を巻き込んでクラッシュさせられる可能性がある。サービス可用性に直結する DoS の入り口であり、外部入力起因のため優先度を高くする。

## 現状

`webrtc/src/whip.cpp` は冒頭で `#include <regex>` しており（`webrtc/src/whip.cpp:16`）、`SendRequest` のレスポンスコールバック内でヘッダ行のパースに `std::regex` を使っている。`webrtc/src/whip.cpp:781` 付近:

```cpp
for (const auto& line : lines) {
  std::smatch m;
  auto r =
      std::regex_match(line.begin(), line.end(), m,
                       std::regex(R"(([^:]+):[ \t]*(.+))"));
  if (r) {
    headers[absl::AsciiStrToLower(m[1].str())] = m[2].str();
  }
}
```

ここで `line` は HTTP レスポンスのヘッダ部を `\r\n` で分割した各行であり、外部入力そのものである。さらに `Link` ヘッダの各エントリ解析でも、外部入力 `str` に対して `std::regex_search` を 3 回呼んでいる。`webrtc/src/whip.cpp:804` 付近:

```cpp
for (const auto& str : strs) {
  std::smatch m;
  if (!std::regex_search(str.begin(), str.end(), m,
                         std::regex(R"(<([^>]+)>)"))) {
    RTC_LOG(LS_ERROR)
        << "Failed to match <...>: str=" << str;
    return;
  }
  server.urls.push_back(m[1].str());
  if (!std::regex_search(
          str.begin(), str.end(), m,
          std::regex(R"|(username="([^"]+)")|"))) {
    RTC_LOG(LS_ERROR)
        << "Failed to match username=\"...\": str=" << str;
    return;
  }
  server.username = m[1].str();
  if (!std::regex_search(
          str.begin(), str.end(), m,
          std::regex(R"|(credential="([^"]+)")|"))) {
    RTC_LOG(LS_ERROR)
        << "Failed to match credential=\"...\": str="
        << str;
    return;
  }
  server.password = m[1].str();
  ...
}
```

これら `std::regex` の構築および `regex_match` / `regex_search` の呼び出しは、いずれも `try` / `catch` で囲まれていない。ファイル全体を検索しても、これらを保護する `try` / `catch` は存在しない（`webrtc/src/whip.cpp` 内に `try` / `catch` / `regex_error` の語は出現しない。1139 行目付近に現れる `catch` は別の文字列の一部であり例外処理ではない）。なお `webrtc/src/whep.cpp` も `#include <regex>`（`webrtc/src/whep.cpp:10`）し、`webrtc/src/whep.cpp:522` 付近以降に同じ構造の `std::regex` 使用箇所があり、同様に保護されていない。

## 設計方針

- `std::regex` を使う箇所を例外安全にする。いずれかの方針を採る。
  - 方針 A: regex の構築と `regex_match` / `regex_search` 呼び出しを `try` / `catch (const std::regex_error&)`（必要に応じて `std::exception`）で囲み、例外時はそのレスポンスをエラーとして扱って状態を `kClosed` に遷移させる。例外でプロセスを落とさない。
  - 方針 B: regex を使わず、文字列探索ベースの手堅いパーサに置換する（C 版 `whip.c` がすでに `strchr` / `strstr` 等で行っている方式に揃える）。これにより `std::regex` 由来の例外そのものを排除する。
- いずれの方針でも、解析対象とする入力サイズに上限を設け、過大なレスポンスやヘッダ行を解析に渡さないようにする（DoS 緩和）。
- `whip.cpp` を対象とする。`whep.cpp` にも同じ構造の問題があるため、合わせて修正する。
- ログメッセージ・エラーメッセージは英語で記述する。

## 完了条件

- 細工された（regex 例外を誘発し得る）レスポンスを受け取っても `std::terminate` でプロセスが落ちず、当該レスポンスがエラーとして扱われる。
- 解析対象の入力サイズに上限が設けられている。
- `whip.cpp` と `whep.cpp` の双方で同じ対策が施されている。
