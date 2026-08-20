# Thread::blocking_call が停止中のスレッドで未初期化ポインタを返す

- Created: 2026-08-20
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-thread-blocking-call-r-uninitialized-return
- Polished: {YYYY-MM-DD}

## 目的

`Thread::blocking_call` (`src/rtc_base/thread.rs`) の `R` が非 `()` の経路（`webrtc_Thread_BlockingCall_r`）で、対象スレッドが停止中（`IsQuitting()`）のときに未初期化ポインタが Rust 側へ渡り、`Box::from_raw` に流れて undefined behavior（無効アドレスの deref / 任意アドレスの解放）になる soundness 問題を修正する。

## 現状

`webrtc/src/webrtc_c/rtc_base/thread.cc` の `webrtc_Thread_BlockingCall_r` は、libwebrtc の非 void テンプレ `BlockingCall` に頼っている:

```cpp
return p->BlockingCall([func, arg]() { return func(arg); });
```

libwebrtc の非 void テンプレ (`rtc_base/thread.h`) は次の実装であり、`ReturnT` をデフォルト初期化してから void 版 `BlockingCall` で functor を実行する:

```cpp
ReturnT BlockingCall(Functor&& functor, const Location& location = ...) {
  ReturnT result;
  BlockingCall([&] { result = std::forward<Functor>(functor)(); }, location);
  return result;
}
```

対象スレッドが停止中（`IsQuitting()`）のとき、`BlockingCallImpl` は functor を実行せずに返るため、`result` はデフォルト初期化されたまま返る。`ReturnT` がスカラ型（`void*`）の場合はデフォルト初期化が**不確定値**となり、null とも限らないゴミ値が返る。

`src/rtc_base/thread.rs` の `blocking_call` はこの戻り値を `assert!(!res_ptr.is_null())` で検証するが、ゴミ値は null とは限らないため弾けない。そのまま `Box::from_raw(res_ptr as *mut R)` でゴミアドレスから `Box<R>` を構築し、`*boxed_res` で読み出し、drop 時に解放するため UB になる。また、このとき functor は実行されないためクロージャ `F` の `Box` がリークする（`()` 経路と同じ）。

なお `webrtc_Thread_BlockingCall`（`()` 経路）は void 版であり、未初期化値が返る問題はない（functor 未実行時に `()` を黙って返す）。本問題は issue 0076 で対象外としていたものである。

## 設計方針

C++ 側の仕様（「functor が実行されなかったらデフォルト初期化値を返す」）に忠実に合わせる。

- `webrtc_Thread_BlockingCall_r` は非 void テンプレを使わず void 版 `BlockingCall` を使い、`void* result = nullptr;` で確定したデフォルト値（`nullptr`）を初期値にしてから functor を実行する。未実行時は `nullptr` が返る（C++ の不確定値の穴を正規化する）
- Rust 側 `blocking_call` に `R: Default` 境界を追加し、`nullptr` なら `R::default()` を返す。実行経路は `thread_trampoline_r` が `Box::into_raw` の結果を返すため常に非 null であり、nullptr は「未実行」を一意に意味する
- 呼び出し側は「実行されて結果がデフォルト値だった」と「実行されなかった」を区別できない（C++ 仕様と同型）
- `()` 経路は元々 `()`（= デフォルト）を黙って返すため、全体の挙動が一貫する
- 未実行時の `Box<F>` のリークは本 issue の対象外とする（`F: Send + 'static` 境界の根拠と同じく、FFI 越しに消費保証がない既知の帰結）
- `R: Default` 境界追加は公開 API の破壊的変更のため、CHANGES.md に `[CHANGE]` として追記する

## テスト戦略

- `src/tests.rs` に新規テストを追加する。`Thread::new()` → `start()` → `stop()` 後の `blocking_call(|| 42)` が UB にならず `R::default()`（`0`）を返すことを `assert_eq!` で検証する（libwebrtc の仕様としてスレッド停止後の Post / Send は失敗するため、未実行経路を確定に再現できる）
- `Thread::quit()` 追加後（別 issue）は、`quit()` 後の `blocking_call` でも同経路を通る

## 完了条件

- `webrtc_Thread_BlockingCall_r` が、functor 未実行時に `nullptr`（確定したデフォルト値）を返すこと
- `Thread::blocking_call` に `R: Default` 境界が追加され、`nullptr` 時に `R::default()` を返すこと
- 停止中のスレッドに対する `blocking_call` が UB にならず `R::default()` を返すテストが `src/tests.rs` に追加され、通過すること
- 既存の `blocking_call` 利用箇所（`src/tests.rs` の `thread_blocking_call_runs`）がコンパイル・実行可能であること
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリが追加されていること
- 未実行時の `Box<F>` のリーク解消は本 issue の対象外とする
