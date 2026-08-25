# ScopedRef 系の内部機構の公開を pub(crate) 化する

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-scoped-ref-from-raw-safety
- Polished: 2026-08-12

## 目的

`ScopedRef::from_raw` (`src/ref_count.rs`) が safe な public API として公開されており、safe Rust だけで double-free / use-after-free を起こせる soundness ホールを塞ぐ。同型の `RTCStatsReport::from_refcounted_ptr` (`src/api/stats.rs`) も safe のままのため、あわせて塞ぐ。なお `from_unique_ptr` 系 (`RtcError` / `SdpParseError` / `SessionDescription`) にも同型のホールが存在するが、本 issue の対象外とする。

## 現状

`ScopedRef::from_raw` は所有権 (参照カウント 1 つ分) を移譲する意味を持つ関数であり、`Drop` で必ず `Release` を呼ぶ:

```rust
pub fn from_raw(raw_ref: NonNull<H::Refcounted>) -> Self {
    Self {
        raw_ref,
        _marker: PhantomData,
    }
}
```

この関数は `pub` かつ safe で、`src/lib.rs` の `pub use ref_count::{RefCountedHandle, ScopedRef};` により外部に公開されている。`RefCountedHandle` は pub トレイトのため、外部ユーザーが独自ハンドルを実装して `from_raw` を safe で呼ぶことも可能である。同一ポインタで 2 回呼べば double-free、drop 済みポインタで呼べば use-after-free が safe Rust だけで発生する。なお `from_raw` には `# Safety` コメントが既に付いているが、safe 関数に付いているため契約として機能していない。

`RTCStatsReport::from_refcounted_ptr` (`src/api/stats.rs`) も同様に safe な public API で、内部で `ScopedRef::from_raw` を呼ぶ。`RTCStatsReport` は統計コールバック (`peer_connection_on_stats`) 経由でしか外部に渡らないため、外部での構築ニーズは無い。

一方で `ScopedRef` / `RefCountedHandle` は公開 API のシグネチャに一切登場しない:

- 全ての `RefCountedHandle` の実装は `pub(crate)` (`src/ref_count.rs` の `AudioDecoderFactoryHandle` 等)
- `ScopedRef` の使用箇所は private フィールドと `pub(crate) fn from_scoped_ref` のみ

つまり `lib.rs` の `pub use` が内部機構 (参照カウント管理の足回り) を無理やり外部公開しているだけで、公開の必然性が無い。

内部の呼び出し箇所は「C 側から所有権を譲渡された参照をラップする」パターンであり、呼び出し側が契約を守っている。

## 設計方針

採用方針: **`ScopedRef` / `RefCountedHandle` / `from_raw` / `from_refcounted_ptr` を pub(crate) 化し、`lib.rs` の `pub use` を撤去する**。

理由:
- `from_raw` は所有権を奪う危険な操作であり、公開 API として残す限り `unsafe fn` 化しても外部利用者が誤用できる footgun が残る
- `ScopedRef` / `RefCountedHandle` は公開シグネチャに登場せず、外部利用者に意味のある使い方が無い。`from_raw` だけ pub(crate) 化すると「public なのに外部から構築不能な型」が残るため、まとめて internal 化する
- `RTCStatsReport` はコールバック経由でしか取得されないため、`from_refcounted_ptr` の外部公開も不要

変更内容:

- `src/ref_count.rs`: `RefCountedHandle` トレイトと `ScopedRef` 構造体を `pub(crate)` にする（`from_raw` 含む全メンバが内部利用のみになる）
- `src/lib.rs`: `pub use ref_count::{RefCountedHandle, ScopedRef};` を削除する（`mod ref_count;` は元々 private のため、これで完全に internal になる）
- `src/api/stats.rs`: `RTCStatsReport::from_refcounted_ptr` を `pub(crate)` にする
- 内部の呼び出し箇所 (クレート内の `from_raw` / `from_refcounted_ptr` / `from_scoped_ref` の全使用箇所) はクレート内なので変更不要

### 代替案（参考）: `unsafe fn` 化

`ScopedRef::from_raw` と `RTCStatsReport::from_refcounted_ptr` を `pub unsafe fn` + `# Safety` にする案（コードベースの先例 `EnvironmentRef::from_raw` / `VideoDecoderDecodedImageCallbackPtr::from_raw` と同形式）。safe Rust からの発火は防げるが、所有権を奪う危険な API が公開面に残り、外部利用者が `unsafe` ブロック内で誤用できる。本 issue では採用しない。

## テスト方針

本 issue の変更は可視性 (pub → pub(crate)) の変更のみで挙動の変化が無いため、ビルドと既存テストの通過をもって検証する。

## 完了条件

- `src/ref_count.rs` の `RefCountedHandle` と `ScopedRef` が `pub(crate)` になっている
- `src/lib.rs` の `pub use ref_count::{RefCountedHandle, ScopedRef};` が削除されている
- `src/api/stats.rs` の `RTCStatsReport::from_refcounted_ptr` が `pub(crate)` になっている
- クレート内の全使用箇所がコンパイル・テストに通ること（`cargo build` / `cargo test`）
- 外部から `ScopedRef` / `RefCountedHandle` / `from_raw` にアクセスできないこと（`cargo doc` の公開 API に現れないこと）
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリ（担当者行を含む）が追加されていること
