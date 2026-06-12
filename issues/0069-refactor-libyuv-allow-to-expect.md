# src/libyuv.rs の #[allow(clippy::too_many_arguments)] を #[expect(...)] に揃える

- Priority: Low
- Created: 2026-06-10
- Completed: {YYYY-MM-DD}
- Model: Opus 4.7
- Branch: feature/refactor-libyuv-allow-to-expect
- Polished: {YYYY-MM-DD}

## 目的

`src/libyuv.rs` の全 7 関数で使われている `#[allow(clippy::too_many_arguments)]` を `shiguredo-rust` 規約に従って `#[expect(clippy::too_many_arguments)]` に置換し、規約準拠の状態にする。

## 優先度根拠

機能には影響せず、規約準拠のみのリファクタリング。`#[allow]` のままでも実害は無いが、`shiguredo-rust` 規約の趣旨 (将来 lint 項目が消えたり、関数引数が減って警告対象でなくなったときに気づけるようにする) からすると放置すべきではない。短時間で機械的に修正可能なため Low とする。

## 現状

`src/libyuv.rs` の以下 7 関数で `#[allow(clippy::too_many_arguments)]` が使われている:

- `:44` `convert_from_i420`
- `:92` `i420_to_nv12`
- `:143` `i420_copy`
- `:197` `nv12_copy`
- `:243` `abgr_to_i420`
- `:289` `nv12_to_i420`
- `:340` `yuy2_to_i420`

`shiguredo-rust` 規約 (`SKILL.md` L30-31) は明確に以下を規定している:

> - lint 警告を抑制する必要があるときは `#[allow(...)]` ではなく `#[expect(...)]` を使うこと
>   - `#[allow(...)]` では、その lint 項目がなくなったり、コードの修正によって不要になったときに気づけないため

`src/libyuv.rs` の 7 関数すべてが規約違反の状態。

なお `src/api/audio_device_module.rs` でも同様の規約違反が複数存在するが、本 issue のスコープは libyuv モジュールに限定し、audio_device_module 側は別途扱う。

この規約違反は issue 0066 (libyuv の MJPGToI420 / MJPGToNV12 ラッパー追加) の磨き上げレビュー過程で判明し、新規追加分と既存スタイルの整合性をどうするかの議論で表面化した。

## 設計方針

`src/libyuv.rs` の `#[allow(clippy::too_many_arguments)]` を `#[expect(clippy::too_many_arguments)]` に機械的に置換する。挙動・公開 API・関数シグネチャは一切変更しない。

`#[expect(...)]` は警告抑制という効果は `#[allow(...)]` と同じだが、抑制対象の警告が実際に発生しない場合にコンパイラが `unfulfilled_lint_expectations` 警告を出す点が違い、規約が求める「不要になったときに気づける」性質を持つ。

## 完了条件

- `src/libyuv.rs` 内の `#[allow(clippy::too_many_arguments)]` がすべて `#[expect(clippy::too_many_arguments)]` に置換されている (7 箇所)
- `grep -n "#\[allow(clippy::too_many_arguments)\]" src/libyuv.rs` が 0 件を返す
- `cargo build`、`cargo test`、`cargo clippy -- -D warnings` がすべて通る (`unfulfilled_lint_expectations` 警告が出ないこと、すなわち全関数で実際に too_many_arguments lint が発生していることを確認)

## 解決方法

### 1. 置換

`src/libyuv.rs` 内の `#[allow(clippy::too_many_arguments)]` を `#[expect(clippy::too_many_arguments)]` に置換する (Edit ツールの `replace_all` でも一括可能、対象 7 箇所すべて同一文字列)。

### 2. 検証

- `cargo clippy -- -D warnings` を実行し、警告抑制が機能していること (= too_many_arguments lint が実際に発生し、`#[expect]` がそれを補足していること) を確認する
- `cargo test` で既存テストが通ることを確認する

### 3. 変更履歴

`shiguredo-changelog` 規約「機能に直接影響しない変更 (ドキュメント追加、リファクタリング等) は `### misc` サブセクションに記載すること」に従い、`CHANGES.md` の `## develop` の `### misc` 配下に以下を追加する。

```
- [UPDATE] `src/libyuv.rs` の `#[allow(clippy::too_many_arguments)]` を `#[expect(clippy::too_many_arguments)]` に揃える
  - `shiguredo-rust` 規約 (`#[allow]` ではなく `#[expect]` を使う) に準拠
  - @<implementer>
```
