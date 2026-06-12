# src/tests.rs から libyuv テストを tests/test_libyuv.rs に分離する

- Priority: Low
- Created: 2026-06-10
- Completed: {YYYY-MM-DD}
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

`tests/` ディレクトリを新規作成し、`tests/test_libyuv.rs` を以下の構造で作成する。

```rust
// 既存 src/tests.rs から libyuv 関連テストをすべて移植する。
// public API のみを利用する。

use shiguredo_webrtc::{
    LibyuvFourcc, abgr_to_i420, convert_from_i420, i420_copy, i420_to_nv12, nv12_copy,
    nv12_to_i420, yuy2_to_i420,
    I420Buffer, NV12Buffer, // i420_buffer_planes_mut_to_nv12_round_trip で利用
};

#[test]
fn abgr_to_i420_conversion() {
    // 既存 src/tests.rs:1273-1299 の内容をそのまま移植
    // ...
}

// 他テスト関数も同様に移植
```

実装時の手順:

1. `src/tests.rs:1273-1775` を切り取って `tests/test_libyuv.rs` に貼り付ける
2. `use super::*;` を `use shiguredo_webrtc::{...}` に書き換える (public API として再エクスポートされている identifier のみインポート)
3. internal item に依存しているテストがあれば、(a) 対象 item を public 化する、(b) 該当テストだけ `src/tests.rs` に残す、のいずれかを個別判断する
4. `src/tests.rs` 側の対応行を削除する

### 2. ビルド・テスト確認

- `cargo build --features source-build --workspace`
- `cargo test --features source-build --workspace` で移行前と同じテストがすべて通ることを確認
- `cargo test --test test_libyuv --features source-build` で個別実行できることを確認

### 3. 変更履歴

`shiguredo-changelog` 規約「機能に直接影響しない変更はミスサブセクション」に従い、`CHANGES.md` の `## develop` の `### misc` 配下に以下を追加する。

```
- [UPDATE] `src/tests.rs` の libyuv 関連テストを `tests/test_libyuv.rs` に分離する
  - `shiguredo-rust` 規約のテスト配置 (`tests/test_<module>.rs`) に揃える第一歩
  - 他モジュールのテスト分離は別途 issue を起票する
  - @<implementer>
```
