# src/tests.rs から libyuv テストを tests/test_libyuv.rs に分離する

- Priority: Low
- Created: 2026-06-10
- Completed: 2026-06-16
- Model: Opus 4.7
- Branch: feature/refactor-libyuv-tests-split
- Polished: {YYYY-MM-DD}

## 目的

`src/tests.rs` に集約されている libyuv モジュールのテスト群を `tests/test_libyuv.rs` に切り出し、`shiguredo-rust` 規約 (`SKILL.md` L59-65) のテスト配置 (`tests/test_<module>.rs` に `src/<module>.rs` を対応させる) に揃える。これにより `src/tests.rs` の肥大化を解消する第一歩とする。

## 優先度根拠

機能影響はゼロで、保守性向上のみ。`src/tests.rs` は現在 3755 行・約 100 テスト関数が集約されており、`shiguredo-rust` 規約「テストファイルが長くなった場合はファイル内で `mod` を使って分割すること。テストが長くなるのはモジュール自体が大きすぎるサインなので `src/<module>.rs` 側の分割を検討すること」も既に違反状態。短時間で機械的に分離可能なため Low とする。

## 現状

- `src/tests.rs` は単一ファイルに約 100 テスト関数 (3755 行) を集約している
- うち libyuv モジュールに対応するテストは `src/tests.rs:1273-1775` (約 500 行・15 テスト関数程度) を占める
  - 正常系: `abgr_to_i420_conversion`, `convert_from_i420_argb_conversion`, `i420_to_nv12_round_trip`, `i420_buffer_planes_mut_to_nv12_round_trip`, `i420_copy_with_odd_size_and_padding`, `nv12_copy_with_odd_size_and_padding`
  - 異常系: `i420_copy_returns_false_when_source_plane_is_too_short`, `i420_copy_returns_false_when_destination_plane_is_too_short`, `nv12_copy_returns_false_when_source_plane_is_too_short`, `nv12_copy_returns_false_when_destination_plane_is_too_short`
- `shiguredo-rust` 規約は以下を要求している
  - 単体テストのファイル名は `tests/test_<module>.rs` とし、`src/<module>.rs` に対応させること
  - PBT のファイル名は `pbt/tests/prop_<module>.rs` とし、`src/<module>.rs` に対応させること
  - テストファイルが長くなった場合はファイル内で `mod` を使って分割すること
- 一方、webrtc-rs リポジトリには `tests/` ディレクトリも `pbt/` ディレクトリも現在存在しないため、新規作成する必要がある

この肥大化は issue 0066 (libyuv の MJPGToI420 / MJPGToNV12 ラッパー追加) の磨き上げレビュー過程で表面化した。

## 設計方針

- 本 issue では **libyuv モジュールのテストの分離のみ** を対象とする (他モジュール `api/`, `rtc_base/`, `error/`, `cxxstd/`, `ref_count/` のテスト分離はそれぞれ別 issue を起票する。「1 issue 1 module」の方針)
- PBT 化 (proptest 導入) は本 issue では行わず、まず integration test (`tests/test_libyuv.rs`) への単体テストの分離のみに専念する
- `tests/test_libyuv.rs` は integration test として配置され、webrtc-rs の public API (`shiguredo_webrtc::libyuv::*` の再エクスポート経由) のみを利用する
  - 既存の libyuv テストは `abgr_to_i420` / `convert_from_i420` / `i420_to_nv12` 等の public 関数を呼ぶ形になっており、integration test への移行で API 互換性を損なわない見込み (実装時に確認)
  - 万一 internal item に依存するテストがあれば、対象を public 化するか `src/tests.rs` に残すか個別判断する
- `src/tests.rs` 側からは分離した libyuv テストを完全に削除する (重複は残さない)

## 完了条件

- `tests/test_libyuv.rs` が新規作成され、`src/tests.rs:1273-1775` の libyuv 関連テストすべて (正常系・異常系) が移行されている
- `src/tests.rs` から libyuv モジュールのテスト関数群が完全に削除されている
- `cargo test --features source-build --workspace` がローカルで通る (移行前と同一のテスト結果が得られること)
- `cargo test --test test_libyuv --features source-build` で `tests/test_libyuv.rs` のテストだけを実行できる
- `Cargo.toml` の `[[test]]` セクション追加が必要なら追加する (Cargo はデフォルトで `tests/*.rs` を自動検出するため通常は不要だが、`required-features = ["source-build"]` 等が必要かは実装時に確認)

## 解決方法

### 1. tests/test_libyuv.rs の新規作成

- `tests/` ディレクトリを新規作成
- `src/tests.rs` の全 libyuv 関連テスト (行 1272-2657, 定数 2 個・テスト 31 件) を `tests/test_libyuv.rs` に抽出
- `use shiguredo_webrtc::{...}` で必要な public API を明示的にインポート:
  - `LibyuvFourcc`, `LibyuvRotationMode`, `abgr_to_i420`, `convert_from_i420`, `convert_to_i420`, `i420_copy`, `i420_rotate`, `i420_to_nv12`, `mjpg_size`, `mjpg_to_i420`, `mjpg_to_nv12`, `nv12_copy`, `nv12_to_i420`
  - `I420Buffer`, `NV12Buffer`
- internal item への依存は無し。全テストが public API のみで動作することを確認

### 2. src/tests.rs からの削除

- 行 1272-2658 を削除 (4637 行 → 3250 行)

### 3. ビルド・テスト確認

- `cargo test --features source-build --workspace`: 全 122 テスト通過 (src/tests.rs: 91 + tests/test_libyuv.rs: 31)
- `cargo test --test test_libyuv --features source-build`: 31 テスト個別実行可能
- `Cargo.toml` の `[[test]]` セクション追加不要 (Cargo が `tests/*.rs` を自動検出)

### 4. 変更履歴

- `CHANGES.md` の `## develop` → `### misc` 配下に変更履歴を追記
