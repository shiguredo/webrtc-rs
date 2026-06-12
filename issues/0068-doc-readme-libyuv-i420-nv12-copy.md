# README に libyuv の i420_copy / nv12_copy を追記する

- Priority: Low
- Created: 2026-06-10
- Completed: {YYYY-MM-DD}
- Model: Opus 4.7
- Branch: feature/refactor-readme-libyuv-i420-nv12-copy
- Polished: {YYYY-MM-DD}

## 目的

README.md の「## 対応 API」セクションの libyuv 変換関数の行に、既存実装の `i420_copy` / `nv12_copy` が記載漏れになっているため追記し、ドキュメントと実装の整合性を回復する。

## 優先度根拠

機能的な影響は無く、API 自体は既に 0.147.1 から利用可能。ドキュメント整合性の補修のみで Low とする。ただし放置するとライブラリ利用者が API の存在を把握できず、独自に同等機能を実装してしまうリスクがあるため、近いうちに修正したい。

## 現状

`README.md:382-383` の「## 対応 API」の libyuv 変換関数の行は以下の通り `i420_copy` / `nv12_copy` が掲載されていない。

```
- `abgr_to_i420` / `convert_from_i420` / `i420_to_nv12` / `nv12_to_i420` / `yuy2_to_i420`
  - カラーフォーマット変換 (libyuv)
```

一方、実装は以下の通り 0.147.1 で追加済み:

- `CHANGES.md` 0.147.1: `[ADD] libyuv の i420_copy / nv12_copy API を追加する`
- C API: `webrtc/src/webrtc_c/libyuv.h:66-90` (`libyuv_I420Copy`, `libyuv_NV12Copy`)
- Rust safe wrapper: `src/libyuv.rs` の `i420_copy` / `nv12_copy`
- 再エクスポート: `src/lib.rs:23-26` の `pub use libyuv::{..., i420_copy, ..., nv12_copy, ...};`
- テスト: `src/tests.rs:1503-1775` に `i420_copy_with_odd_size_and_padding` 等の複数テスト

この記載漏れは issue 0066 (libyuv の MJPGToI420 / MJPGToNV12 ラッパー追加) の磨き上げレビュー過程で判明した。

## 設計方針

`README.md:382-383` の libyuv 変換関数の行に `i420_copy` / `nv12_copy` をアルファベット順を維持して追加する。掲載スタイル (スラッシュ区切りで横並び) と内訳の説明文 (`カラーフォーマット変換 (libyuv)`) は既存のままを維持する。

## 完了条件

`README.md:382-383` の libyuv 変換関数の行が以下のようにアルファベット順を維持しつつ `i420_copy` と `nv12_copy` を含む形に更新されている。

```
- `abgr_to_i420` / `convert_from_i420` / `i420_copy` / `i420_to_nv12` / `nv12_copy` / `nv12_to_i420` / `yuy2_to_i420`
  - カラーフォーマット変換 (libyuv)
```

## 解決方法

### 1. README.md の更新

`README.md:382-383` を上記の完了条件の通り書き換える。`i420_copy` は `convert_from_i420` の後、`i420_to_nv12` の前に挿入する。`nv12_copy` は `i420_to_nv12` の後、`nv12_to_i420` の前に挿入する (アルファベット順)。

### 2. 変更履歴

ドキュメント整合性の補修であり機能影響が無いため、`CHANGES.md` の `## develop` 直下ではなく `### misc` サブセクションに以下の UPDATE エントリを追加する (`shiguredo-changelog` 規約「機能に直接影響しない変更 (ドキュメント追加、リファクタリング等) は `### misc` サブセクションに記載すること」)。

```
- [UPDATE] README.md の対応 API libyuv 行に記載漏れの `i420_copy` / `nv12_copy` を追記する
  - @<implementer>
```
