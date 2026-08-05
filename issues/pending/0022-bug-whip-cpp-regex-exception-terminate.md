# whip.cpp / whep.cpp の std::regex 例外で terminate しないようにする

- Priority: Medium
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-cpp-regex-exception-terminate

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

`webrtc/src/whip.cpp` と `webrtc/src/whep.cpp` は、サーバから受け取った HTTP レスポンスを
`std::regex` で解析している。`std::regex` の `regex_match` / `regex_search` は実装によっては
実行時に `std::regex_error`（complexity 超過等）を送出し得る。これらの呼び出しは
`try` / `catch` で保護されておらず、例外が送出されると `std::terminate` でプロセスが落ちる。
外部入力起因でプロセスが終了しないよう、regex 使用箇所を例外安全にする。

なお `std::regex` の実行時送出は実装依存であり、本リポジトリのビルド環境（libc++）では
`regex_search` / `regex_match` の実行時に `std::regex_error` を送出しない（送出するのは
不正なパターンのコンパイル時のみ）。libstdc++ / MSVC 等の実装では送出し得るため、
移植性の観点から対策する。

## 優先度根拠

Medium。`std::regex` の実行時例外の送出は libc++ / libstdc++ / MSVC で挙動が異なり、
本リポジトリのビルド環境（libc++）では送出されない。ただし外部入力（レスポンス）を
`std::regex` で解析する構造自体が実装依存のクラッシュ / ハング経路を持ち、将来の
ビルド環境変更や細工された入力でプロセスが落ちるリスクがある。即時の実害は実証されて
いないため High ではなく Medium とする。

## 現状

`SignalingWhip::Connect`（`webrtc/src/whip.cpp`）/ `SignalingWhep::Connect`
（`webrtc/src/whep.cpp`）内の SendRequest コールバックで `std::regex` を使用している。

- ヘッダ行パース: `std::regex_match(line, m, std::regex(R"(([^:]+):[ \t]*(.+))"))`
- Link ヘッダ解析: `std::regex_search(str, m, std::regex(R"(<([^>]+)>)"))` と
  `username="..."` / `credential="..."` の 2 パターン

全呼び出しが `try` / `catch` で保護されていない。C 版の `whip.c` / `whep.c` は
`strchr` / `strstr` による文字列探索で実装されており、例外の問題はない。

なお Link ヘッダ解析の堅牢化（RFC 8288 準拠のパーサへの置換）は `issues/0021` の
スコープであり、本 issue の対象はヘッダ行パースの `std::regex` 置換に限定する。

## 設計方針

### ヘッダ行パースの regex 置換

ヘッダ行パースの `std::regex_match(line, m, std::regex(R"(([^:]+):[ \t]*(.+))"))` を
`:` の位置で名前と値を分割する文字列操作に置き換える。分割の規則は regex 版と一致させる:

- `:` を含まない行は不一致（スキップ）
- 最初の `:` で名前と値を分割する
- 名前の後続空白は保持しない（regex 版は `([^:]+)` が `:` 直前までの非コロン文字列を
  捕捉するため、`Name : value` は名前が `Name ` になる。regex 版と同一挙動にするため
  `:` 直前の空白は保持する扱いで一致させる）
- 値の先頭の空白 / タブは除去する（regex 版の `[ \t]*` に相当）

Link ヘッダ解析の `std::regex_search` 3 箇所は `issues/0021` が RFC 8288 準拠のパーサに
置換するため、本 issue では置換しない（0021 の実装を待つ）。

### 入力サイズ上限

解析対象の入力サイズに上限を設け、過大な入力による DoS（クラッシュ・ハング・メモリ
消費）を緩和する。上限は `SignalingWhip::SendRequest` / `SignalingWhep::SendRequest` の
受信ループ（`resp.append(buf, n)` で無制限に蓄積する箇所）に適用する。レスポンス全体の
受信上限を設け、超過したらエラーとして扱う（解析側だけでなく受信側で制限しないと
メモリ DoS を防げない）。上限値は SDP レスポンスとして想定し得るサイズ（目安:
数 MB 程度）とする。

### 依存関係

- `issues/0021`（Link ヘッダ解析の堅牢化）は本 issue と同じコード領域を対象とする。
  本 issue はヘッダ行パースの regex 置換と受信上限の追加にスコープを絞り、
  Link ヘッダ解析の regex 置換は 0021 が担う
- `issues/0053`（whip/whep の C++ 版重複排除）は `SignalingWhip::Connect` /
  `SignalingWhep::Connect` を共通化対象としている。実装順序によっては競合する

## テスト戦略

- 通常の HTTP レスポンスに対するパース結果が regex 版と一致することを
  `main` 関数内の動作確認用テストコードで確認する
- ヘッダ行パースのテストケース: `:` が無い行、`:` が複数ある行、空のヘッダ名、
  ヘッダ名 / 値の先頭末尾空白、極端に長いヘッダ行（サイズ上限の確認）
- 受信上限超過時にエラー扱いされることを、上限を超えるレスポンスを返す試験用の
  実 HTTP サーバで確認する（モック・スタブは使わない。AGENTS.md の「モックやスタブは
  絶対に利用しないこと」に従う）

## 完了条件

- ヘッダ行パースの `std::regex` 使用を `whip.cpp` と `whep.cpp` から削除する
  （`<regex>` ヘッダの include も削除される）
- 細工されたレスポンスを受け取っても `std::terminate` でプロセスが落ちない
- ヘッダ行パースの挙動が regex 版と一致する（`:` の分割位置・値先頭の空白除去）
- レスポンス受信時にサイズ上限が適用され、超過時はエラーとして扱われる
- 完了条件の検証は上記のテスト戦略で行う
