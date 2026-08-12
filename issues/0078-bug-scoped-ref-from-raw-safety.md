# ScopedRef::from_raw が safe で公開され double-free を許す

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

`RTCStatsReport::from_refcounted_ptr` (`src/api/stats.rs`) も同様に safe な public API で、内部で `ScopedRef::from_raw` を呼ぶ。`lib.rs` の `pub use api::*` で公開されており、`ScopedRef::from_raw` と同じ soundness ホールを持つ。

内部の呼び出し箇所は「C 側から所有権を譲渡された参照をラップする」パターンであり、呼び出し側が契約を守っている。同様の所有権移譲 API である `Thread::into_raw` 等は `into_raw` という命名で所有権の放棄を明示しているが、`from_raw` は所有権を受け取る契約がシグネチャから読み取れない。

## 設計方針

**`unsafe fn from_raw` に変更し、`# Safety` コメントで契約を明記する**（`pub(crate)` 化は `RefCountedHandle` が pub トレイトで公開されている以上、外部ユーザーの独自ハンドル利用を壊すため採用しない）。

- `pub unsafe fn from_raw` に変更し、既存の `# Safety` コメント（「`raw_ref` は有効な refcounted ポインタで、呼び出し元が所有権を持っていること」）を契約として機能させる（文言は既存のものを維持する）
- `RTCStatsReport::from_refcounted_ptr` も `pub unsafe fn` に変更し、`# Safety` コメント（「`raw_ref` は有効な refcounted ポインタであり、呼び出し元が参照カウント 1 つ分の所有権を譲渡すること」）を新規に付与する（現状 `# Safety` がないため。clippy の `missing_safety_doc` が CI (`cargo clippy -D warnings`) でエラー化するため必須）
- コードベースの先例 (`EnvironmentRef::from_raw` (`src/api/environment.rs`)、`VideoDecoderDecodedImageCallbackPtr::from_raw` (`src/api/video_decoder.rs`)) は既に `pub unsafe fn` + `# Safety` の形式であり、形式に合わせる（先例は借用型 (`PhantomData<&'a>`) で所有権を奪わないため、契約内容は本件の既存コメントを踏襲する）
- 本変更は公開 API の safe → unsafe fn 化であり後方互換のない変更のため、CHANGES.md に `[CHANGE]` として追記する（担当者行も含む）

## 完了条件

- `ScopedRef::from_raw` が `unsafe fn` + `# Safety` ドキュメント付きであること
- `RTCStatsReport::from_refcounted_ptr` が `unsafe fn` + `# Safety` ドキュメント付きであること
- 公開 API としての意味論 (所有権を奪う操作) が Rustdoc に明記されていること
- 内部の呼び出し箇所すべてがコンパイル・テストに通ること
- `CHANGES.md` の `## develop` セクションに `[CHANGE]` エントリ（担当者行を含む）が追加されていること

## 解決方法

- `ScopedRef::from_raw` (`src/ref_count.rs`) を `pub unsafe fn` に変更し、既存の `# Safety` コメントを契約として明記する
- `RTCStatsReport::from_refcounted_ptr` (`src/api/stats.rs`) を `pub unsafe fn` に変更し、`# Safety` コメントを付与する
- `ScopedRef::<H>::from_raw` の全呼び出し箇所 (`src/api/` 配下の `media_stream.rs` / `stats.rs` / `rtp.rs` / `audio.rs` / `video.rs` / `peer_connection.rs` / `video_codec_common.rs` / `audio_device_module.rs` の 8 ファイル) に `unsafe` ブロックを追加する（`from_unique_ptr` 系は `ScopedRef` と無関係のため対象外）
- `RTCStatsReport::from_refcounted_ptr` の呼び出し箇所 (`src/api/peer_connection.rs` の `peer_connection_on_stats`) は unsafe fn 内だが、edition 2024 の `unsafe_op_in_unsafe_fn` により unsafe 操作は明示的な `unsafe` ブロックで囲む必要がある（既存の `Box::from_raw` 呼び出しも同様の書き方）。unsafe ブロックで囲んで呼び出す
- 公開 API としての意味論 (所有権を奪う操作) を Rustdoc に明記する
