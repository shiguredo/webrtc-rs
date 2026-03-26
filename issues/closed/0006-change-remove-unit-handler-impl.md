# 公開 handler trait の impl for () を削除する

Created: 2026-03-26
Completed: 2026-03-26
Model: GPT-5 Codex

## 背景

公開 handler trait に対して `impl ... for ()` を提供しているため、型安全性と利用意図が曖昧になっている。
`new_with_handler(Box::new(()))` が通ってしまい、利用側が明示的な handler 実装を持たなくても API を呼べる状態になっている。

## 根拠

`()` への実装は callback API の責務を不明瞭にし、意図しない no-op 実装を許容する。
公開 API としては、利用側が明示的な handler 型を定義して渡す形に統一したほうが堅牢である。

## 対応内容

- `src` / `src/rtc_base` の公開 handler trait 15 箇所から `impl ... for ()` を削除する
- `src/tests.rs` の `Box::new(())` 利用箇所をテスト専用 `NoopHandler` に置換する
- `CHANGES.md` の `develop` に破壊的変更として記載する

## 解決方法

- 公開 handler trait 15 箇所から `impl ... for ()` を削除した
- `tests.rs` にテスト専用 `NoopHandler` を追加し、`Box::new(())` / `Some(Box::new(()))` をすべて置換した
- `CHANGES.md` の `develop` に `[CHANGE]` エントリを追加した
- `cargo test --lib` を実行し、全 53 テストが成功することを確認した
