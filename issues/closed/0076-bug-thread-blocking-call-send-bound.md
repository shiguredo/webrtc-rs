# Thread::blocking_call に F: Send + 'static 境界がない

- Created: 2026-08-03
- Completed: 2026-08-21
- Branch: feature/change-thread-blocking-call-send-bound
- Polished: 2026-08-12

## 目的

`Thread::blocking_call` (`src/rtc_base/thread.rs`) が safe API でありながら非 Send なクロージャを他スレッドへ渡せる soundness ホールを塞ぐ。

## 現状

`Thread::blocking_call` は以下のシグネチャで、`F` に `Send` 境界がない:

```rust
pub fn blocking_call<F, R>(&mut self, f: F) -> R
where
    F: FnOnce() -> R,
    R: Send + 'static,
```

実装はクロージャを `Box::into_raw` で libwebrtc の `Thread` に渡し、`webrtc_Thread_BlockingCall` / `webrtc_Thread_BlockingCall_r` 経由で実行する。libwebrtc 本体 (m151.7922.0.0) の `BlockingCallImpl` (`rtc_base/thread.cc`) は、対象スレッドが異なる場合に `PostTask` でタスクをキューし他スレッドで実行する（同一スレッドからの呼び出しは直接実行、`IsQuitting()` 時は実行せずに返り、その場合クロージャは実行されず `Box` がリークする）。つまり `F` は他スレッドで実行される可能性があるのに、`F: Send` が要求されていない。

なお `BlockingCall_r` 経路では `IsQuitting()` 時に未初期化ポインタが返り得る別の soundness 問題が存在するが、本 issue の対象外とする。

`Rc` や非 Send 参照をキャプチャしたクロージャが safe Rust で渡せてしまい、データ競合 (UB) を起こしうる。`thread_trampoline` / `thread_trampoline_r` (同ファイル先頭の 2 つの extern "C" 関数) も同様に `F: Send` を要求していない。

## 設計方針

`F: Send + 'static` を境界に追加する。

- `F: Send` の根拠は、実行スレッドが呼び出しスレッドと異なる可能性がある（`IsCurrent()` かどうかは実行時条件であり、型では制御できない）こと。`&mut self` は `Thread` 構造体自体への排他を保証するが、`F` がキャプチャするデータには及ばない
- `F: 'static` の根拠は、クロージャが `Box::into_raw` で C++ 側のタスクに渡り、実行・破棄のタイミングが Rust 側から制御できないこと。Rust の借用チェックは FFI 境界を越えて検証できないため、キャプチャした借用データの寿命を型で保証できない。理論上はブロッキング同期実行により `F: Send` のみでも安全だが、消費保証が FFI 越しにしかないため安全側に倒して `'static` を要求する。既存の `R: Send + 'static` 境界との対称性もあり、`R` 側は既存境界のため変更しない
- 本変更は公開 API の破壊的変更である（借用クロージャを渡していたコードがコンパイルエラーになる）。ただし既存の利用箇所は全て非借用クロージャのため影響を受けない

## 完了条件

- `Thread::blocking_call` に `F: Send + 'static` 境界が追加されていること。`thread_trampoline` / `thread_trampoline_r` にも同様の境界が追加されていること
- 非 Send なクロージャ (`Rc` キャプチャ等) を渡すコードがコンパイルエラーになること（`compile_fail,E0277` doctest を `blocking_call` の doc コメントに追加して検証する。借用クロージャを渡す `compile_fail,E0373` doctest も追加し、`F: 'static` 違反も検証する。trybuild は compile_fail doctest で十分検証できるため新規導入しない）
- `blocking_call` の doc コメントに `F: Send + 'static` が必要な理由（他スレッドで実行され得る・FFI 越しに消費されるため）が追記されていること
- 既存の `blocking_call` 利用箇所 (`src/tests.rs` の `thread_blocking_call_runs` 内の 2 箇所) がコンパイル・実行可能であること
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリが追加されていること（公開 API の境界追加による後方互換のない変更のため）

## 解決方法

- `Thread::blocking_call` の `where` 句に `F: Send + 'static` を追加した
- `thread_trampoline` / `thread_trampoline_r` の `where` 句にも同様の境界を追加した（`blocking_call` 側の境界により常に満たされるが、unsafe 関数の契約明示として防御的に追加）
- `blocking_call` の doc コメントに、境界が必要な理由の説明と positive doctest、`compile_fail,E0277` doctest（非 Send）、`compile_fail,E0373` doctest（借用クロージャ）を追記した
  - 借用クロージャのエラーコードは実測により `E0373` と判明したため、完了条件に記載していた `E0521` を `E0373` に修正した
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリを追記した
- `cargo test --workspace --features source-build` で全テストと doctest が通過することを確認した
