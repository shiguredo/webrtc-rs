# whip.cpp の std::regex 例外で terminate しないようにする

- Priority: High
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.cpp` と `webrtc/src/whep.cpp` は、サーバから受け取った HTTP レスポンスを
`std::regex` で解析している。`std::regex` のコンストラクタや `regex_match` / `regex_search` は、
入力によって `std::regex_error` を送出し得る。これらの呼び出しは `try` / `catch` で保護されておらず、
例外が送出されると `std::terminate` でプロセスが落ちる。
外部入力起因でプロセスが終了しないよう、regex 使用箇所を例外安全にする。

## 再現手順

1. `whip.cpp` または `whep.cpp` をビルドする
2. ヘッダ行が極端に長い（数万文字のヘッダ名や値）HTTP レスポンスを返すサーバに対して
   シグナリングを実行する
3. `std::regex` の内部実装によっては `std::regex_error`（complexity 超過やスタック枯渇）が
   送出され、try/catch が無いため `std::terminate` でプロセスが異常終了する

## 優先度根拠

High。WHIP のシグナリング相手（あるいは中間者）が細工したレスポンスを返すだけで `std::terminate` を
誘発し、プロセス全体をクラッシュさせられる。サービス可用性に直結する DoS の入り口であり、
外部入力起因のため優先度を高くする。

## 現状

`whip.cpp:781-786` のヘッダ行パースと `whip.cpp:804-860` の Link ヘッダ解析で
`std::regex` を使用している。全呼び出しが try/catch で保護されていない。
`whep.cpp:522-567` も同一構造。

C 版の `whip.c` / `whep.c` は `strchr` / `strstr` による文字列探索で実装されており、
例外の問題はない。

## 設計方針

方針 B（regex を使わない文字列パーサへの置換）を推奨する。理由:

- AGENTS.md「依存は最小限にすること」に合致する（`<regex>` ヘッダの依存を削除できる）
- C 版 `whip.c` / `whep.c` が既に `strchr` / `strstr` / `strncasecmp` で実装済みであり、
  同様のアプローチで C++ 版も統一できる
- `std::regex` の例外挙動は libstdc++ / libc++ / MSVC で異なり、
  try/catch でも移植性の問題が残る

regex の置換対象:
- ヘッダ行パース（`([^:]+):[ \t]*(.+)`）→ `:` の位置で名前と値を分割する文字列操作
- Link ヘッダ解析（`<([^>]+)>`, `username="..."`, `credential="..."`） →
  `strchr('<')` / `strchr('>')` と `strstr` による抽出

### 依存関係

- 本 issue は `issues/0021`（Link ヘッダ解析の堅牢化）の前提作業とする。
  #0021 で行う堅牢なパーサへの置換を #0022 で先に済ませておくことで、
  regex 例外リスクを早期に除去できる
- 解析対象の入力サイズに上限を設け、過大な入力による DoS を緩和する

## テスト戦略

- 通常の HTTP レスポンスに対するパース結果が regex 版と一致することを
  単体テストで確認する（`main` 内テスト）
- 極端に長いヘッダ行を含むレスポンスでもクラッシュしないことを検証する

## 完了条件

- `std::regex` の使用を `whip.cpp` と `whep.cpp` から完全に削除する
- 細工されたレスポンスを受け取っても `std::terminate` でプロセスが落ちない
- C 版のパーサと同等の堅牢性を持つ文字列ベースのパーサに置換される
- 解析対象の入力サイズに上限が設けられている
- `<regex>` ヘッダの include が削除される
