# webrtc_Thread_SleepMs の戻り値破棄に対処する

- Created: 2026-08-20
- Completed: 2026-08-27
- Branch: feature/fix-webrtc-c-thread-sleepms-return
- Polished: {YYYY-MM-DD}

## 目的

webrtc_c の `webrtc_Thread_SleepMs` が、スリープがシグナルで中断されたかどうかを呼び出し側へ伝えられるようにする。現状は `webrtc::Thread::SleepMs()` の `bool` 戻り値を破棄しており、シグナルによる中断を検知できない。

## 現状

`webrtc/src/webrtc_c/rtc_base/thread.cc` の `webrtc_Thread_SleepMs` は、`webrtc::Thread::SleepMs()` の戻り値を受け取らず破棄している。ヘッダ (`thread.h`) の宣言も `void` である。

libwebrtc の `webrtc::Thread::SleepMs()` (`rtc_base/thread.h`) は `bool` を返し、スリープがシグナルで中断された場合に `false` を返す（POSIX のみ。doc コメントより）。

## 設計方針

- `webrtc_Thread_SleepMs` の戻り値型を `void` から `int` に変更し、`webrtc::Thread::SleepMs()` の戻り値 `bool` を `int` (0/1) に変換して返す。webrtc_c の C API は真偽値を `bool` ではなく `int` (0/1) で表現するのが全体の慣行であり、`webrtc_Thread_Start` の戻り値修正と同様の方式とする
- C のヘッダでは `int` を使うため、`#include <stdbool.h>` の追加は不要（`int` は C の組み込み型）。実装側 `.cc` は C++ であり、`return webrtc::Thread::SleepMs(millis) ? 1 : 0;` のように `int` に変換して返す
- 実装とヘッダの両方を更新する
- 呼び出し元（`webrtc/src/whip.c` の 5 箇所、`src/tests.rs` と `examples/whip/src/main.rs` の Rust 側）は戻り値を無視する呼び出しであり、`int` への変更後もコンパイル・警告ともに問題ない（Rust の `bool` には `#[must_use]` は付与されていないため）
- Rust ラッパー `Thread::sleep_ms` (`src/rtc_base/thread.rs`) は戻り値型を `bool` に変更し、FFI の戻り値 `int` を `!= 0` で `bool` に変換する
- 戻り値型の変更は後方互換のない破壊的変更のため、CHANGES.md に `[CHANGE]` として追記する

## テスト戦略

- Rust 側: `src/tests.rs` の `thread_sleep_ms_runs` を `assert!(Thread::sleep_ms(1))` で成否検証に変更する（`bool` が `false` になるのは POSIX のシグナル割り込み時のみで、テストでは再現しないため正常系のみテストする）

## 完了条件

- `webrtc_Thread_SleepMs` の戻り値型が `int` になり、`webrtc::Thread::SleepMs()` の戻り値を呼び出し側へ伝達する
- ヘッダ (`thread.h`) と実装 (`thread.cc`) の戻り値型が一致している
- Rust ラッパー `Thread::sleep_ms` が `bool` を返すようになっている
- `src/tests.rs` の `thread_sleep_ms_runs` が成否検証を行うテストになっている
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリが追加されている

## 解決方法

- `webrtc/src/webrtc_c/rtc_base/thread.h` と `webrtc/src/webrtc_c/rtc_base/thread.cc` の `webrtc_Thread_SleepMs` の戻り値型を `void` から `int` (0/1) に変更し、`webrtc::Thread::SleepMs()` の成否を `return webrtc::Thread::SleepMs(millis) ? 1 : 0;` で呼び出し側へ伝達するようにした
- `src/rtc_base/thread.rs` の `Thread::sleep_ms` を `bool` を返すように変更し、FFI の戻り値 `int` を `!= 0` で `bool` に変換するようにした
- `src/tests.rs` の `thread_sleep_ms_runs` を `assert!(Thread::sleep_ms(1))` で成否検証するようにした
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリを追記した
- `--features source-build` で `cargo test thread_sleep_ms_runs` が通過することを確認した（prebuilt 固定の `bindings.rs` ではなくソースビルドでヘッダ変更を反映）
