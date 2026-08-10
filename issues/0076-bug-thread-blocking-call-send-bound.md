# Thread::blocking_call に F: Send 境界がない

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-thread-blocking-call-send-bound
- Polished: {YYYY-MM-DD}

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

実装はクロージャを `Box::into_raw` で libwebrtc の `Thread` に渡し、`webrtc_Thread_BlockingCall` / `webrtc_Thread_BlockingCall_r` 経由で実行する。libwebrtc の `BlockingCallImpl` (`rtc_base/thread.cc`) は対象スレッドが異なる場合に `PostTask` でタスクをキューし、他スレッドで実行する。つまり `F` は他スレッドで実行される可能性があるのに、`F: Send` が要求されていない。

`Rc` や非 Send 参照をキャプチャしたクロージャが safe Rust で渡せてしまい、データ競合 (UB) を起こしうる。`thread_trampoline` / `thread_trampoline_r` (同ファイル先頭の 2 つの extern "C" 関数) も同様に `F: Send` を要求していない。

## 設計方針

`F: Send + 'static` を境界に追加する。`&mut self` を要求しているため、同一 `Thread` への排他アクセスは既に保証されており、`F: Send` の追加で十分に安全になる。`'static` もクロージャの寿命要件として明示する。

## 完了条件

- `Thread::blocking_call` に `F: Send + 'static` 境界が追加されていること
- 非 Send なクロージャ (`Rc` キャプチャ等) を渡すコードがコンパイルエラーになること
- 既存の `blocking_call` 利用箇所 (`src/tests.rs` 等) がすべてコンパイル・実行可能であること

## 解決方法

- `Thread::blocking_call` の `where` 句に `F: Send + 'static` を追加する
- `thread_trampoline` / `thread_trampoline_r` の `where` 句にも同様の境界を追加する
- 既存の利用箇所が新境界を満たすことを確認する
