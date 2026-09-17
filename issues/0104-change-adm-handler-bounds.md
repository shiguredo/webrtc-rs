# AudioDeviceModuleHandler の Send + Sync 要求を見直す

- Created: 2026-09-17
- Completed: {YYYY-MM-DD}
- Branch: feature/change-adm-handler-bounds
- Polished: {YYYY-MM-DD}

## 目的

`AudioDeviceModuleHandler` は `Send + Sync` を要求しているが、libwebrtc の ADM 実装は公開メソッドを単一スレッドで呼ぶ前提で書かれており、`Sync` は過剰な要求の可能性がある。

`Sync` を要求すると、ハンドラが排他的なハンドル（例: `AudioTransportRefMut`）を保持したまま `&mut self` のメソッドを呼ぶために `Mutex` などの内部可変性が必須になる。libwebrtc の実際のスレッド契約を確認したうえで、bound とトランポリンの方式を適切に設定し直す。

## 現状

- `src/api/audio_device_module.rs` の `AudioDeviceModuleHandler` は `Send + Sync` を要求し、すべてのメソッドが `&self` を取る
- ADM のトランポリン（`adm_state`）は `user_data` から `&'static AudioDeviceModuleHandlerState` を作り、`&self` メソッドを呼ぶ
  - 同時に呼ばれても安全にするため、ハンドラに `Sync` を要求する設計になっている
- 一方 `AudioTransportHandler` は `Send` のみを要求し、トランポリンは `user_data` から `&mut` を作って `&mut self` メソッドを呼ぶ
- libwebrtc 側の根拠
  - `modules/audio_device/linux/audio_device_pulse_linux.h` の `AudioDeviceLinuxPulse` は「Stores thread ID in constructor. We can then use RTC_DCHECK_RUN_ON(...) to ensure that other methods are called from the same thread.」とコメントし、公開メソッドの先頭で `RTC_DCHECK(thread_checker_.IsCurrent())` を取る（`Detach` はしない）
  - `sdk/android/src/jni/audio_device/audio_device_module.cc` の Android ADM はコンストラクタと `Terminate()` の後に `thread_checker_.Detach()` するため、Init / Terminate のサイクル間でスレッドが変わり得るが、同時アクセスは想定していない
  - 音声スレッドが呼ぶのは `AudioDeviceBuffer` 経由の `AudioTransport` であり、ADM の仮想メソッドではない
- `AudioDeviceModule` 自体は生成と削除を同じスレッドで行う必要がある（一部プラットフォームの制約）ため `Send` / `Sync` を実装していない

## 設計方針

- libwebrtc の契約に合わせ、`AudioDeviceModuleHandler` の要求を `Send` のみにする
- 併せて ADM のトランポリンを `&mut` ベース（`AudioTransport` と同じ方式）に変更する
  - ハンドラ側は `Mutex` のような内部可変性なしで排他的なハンドルを保持できるようになる
  - `Sync` を外すことで `&self` 経由の共有アクセスが型で禁止され、同時呼び出しが起きる使い方はコンパイルエラーになる
- ただし libwebrtc は ADM の同時呼び出しを明文化していないため、実装前に `AudioDeviceModule` の全仮想メソッドの呼び出し元スレッドを確認する
  - 同時呼び出しが確認できた場合は `Sync` を維持し、`AudioTransportRefMut` 側の扱い（`Mutex` を強制する現状の設計）を再検討する

## 完了条件

- libwebrtc の ADM のスレッド契約が issue に記録されている
- `AudioDeviceModuleHandler` の bound と ADM トランポリンの方式が、確認した契約と一致している
- `cargo clippy --workspace --features source-build -- -D warnings` と `cargo test --workspace --features source-build` が通る

## 解決方法

{実装時に記入する}
