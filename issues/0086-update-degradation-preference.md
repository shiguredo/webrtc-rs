# DegradationPreference の DISABLED を MAINTAIN_FRAMERATE_AND_RESOLUTION に揃える

- Created: 2026-08-25
- Completed: {YYYY-MM-DD}
- Branch: feature/update-degradation-preference
- Polished: {YYYY-MM-DD}

## 目的

libwebrtc は 2025-10-13 の "Rename DISABLED to MAINTAIN_FRAMERATE_AND_RESOLUTION"（webrtc の 9559309924）で本名を `MAINTAIN_FRAMERATE_AND_RESOLUTION` に変え、`DISABLED` は後方互換エイリアスとして削除予定（TODO(webrtc:450044904)）とした。webrtc-rs は現在も旧名 `DISABLED` に依存しており、libwebrtc の削除が実行されるとビルドが壊れる。命名を本流に揃え、これから新規に依存する sora-rust-sdk 側に W3C / libwebrtc と一致した名前を提供する。

## 現状

- 本流 libwebrtc（api/rtp_parameters.h の `enum class DegradationPreference`）: `MAINTAIN_FRAMERATE_AND_RESOLUTION` が本名、`DISABLED = MAINTAIN_FRAMERATE_AND_RESOLUTION` が互換エイリアス。`DegradationPreferenceToString()` は `MAINTAIN_FRAMERATE_AND_RESOLUTION` のみを扱い `"maintain-framerate-and-resolution"` を返す
- webrtc-rs の Rust enum（src/api/rtp.rs の `DegradationPreference`）: バリアントは `Disabled` / `MaintainFramerate` / `MaintainResolution` / `Balanced` / `Unknown(i32)`。`Disabled` は `ffi::webrtc_DegradationPreference_DISABLED` を参照している
- webrtc_c の C シム（webrtc/src/webrtc_c/api/rtp_parameters.cc）: `webrtc_DegradationPreference_DISABLED = static_cast<int>(webrtc::DegradationPreference::DISABLED)` と定義している。libwebrtc 側で `DISABLED` が削除されるとここがコンパイルエラーになる
- 本流 libwebrtc の非テストコードで `DISABLED` を使う箇所は存在しない（video_stream_encoder_unittest.cc のみ）。webrtc_video_engine.cc は `MAINTAIN_FRAMERATE_AND_RESOLUTION` をデフォルト値として使用している
- 既存テスト（src/tests.rs）: `DegradationPreference::Balanced` と `None` の往復のみ検証しており、`Disabled` の往復は検証されていない

## 設計方針

- `webrtc::DegradationPreference::DISABLED` への参照を `MAINTAIN_FRAMERATE_AND_RESOLUTION` に置き換える
  - C シムの定数を `webrtc_DegradationPreference_MAINTAIN_FRAMERATE_AND_RESOLUTION` に改名し、`webrtc::DegradationPreference::MAINTAIN_FRAMERATE_AND_RESOLUTION` から定義する（webrtc/src/webrtc_c/api/rtp_parameters.h / .cc の宣言・定義の両方）
  - Rust enum の `Disabled` を `MaintainFramerateAndResolution` へ変名する（src/api/rtp.rs）
- 互換エイリアス `Disabled` は残さない。値が同一であることを API 上で隠さず、本流の将来像のまま公開する。semver 上は下位互換なしの変更（CHANGES.md では [CHANGE]）として扱う
- 値の対応は変わらないため、`to_int` / `from_int` の実装は参照先定数の変更だけになる
- src/tests.rs の往復テストに `MaintainFramerateAndResolution` のケースを追加する

## 完了条件

- `DegradationPreference::MaintainFramerateAndResolution` が公開され、`set_degradation_preference` / `degradation_preference` の往復で値が保たれること
- `webrtc::DegradationPreference::DISABLED` への参照がリポジトリ内に存在しないこと（TODO(webrtc:450044904) の実行後もビルド可能になる）
- `cargo test` と `cargo clippy --all-targets -- -D warnings` が成功すること
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリが追加されていること

## 変更対象

- `src/api/rtp.rs`（`DegradationPreference` enum の変名）
- `src/tests.rs`（往復テストの追加）
- `webrtc/src/webrtc_c/api/rtp_parameters.h` / `webrtc/src/webrtc_c/api/rtp_parameters.cc`（C シム定数の改名）
- `CHANGES.md`
