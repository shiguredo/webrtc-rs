# ScopedRef::from_raw が safe で公開され double-free を許す

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-scoped-ref-from-raw-safety
- Polished: {YYYY-MM-DD}

## 目的

`ScopedRef::from_raw` (`src/ref_count.rs`) が safe な public API として公開されており、safe Rust だけで double-free / use-after-free を起こせる soundness ホールを塞ぐ。

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

この関数は `pub` かつ safe で、`src/lib.rs` の `pub use ref_count::{RefCountedHandle, ScopedRef};` により外部に公開されている。同一ポインタで 2 回呼べば double-free、drop 済みポインタで呼べば use-after-free が safe Rust だけで発生する。内部的には FFI コールバックからの所有権移譲に使用されており、内部利用自体は正しい。

同様の所有権移譲 API である `Thread::into_raw` 等は `into_raw` という命名で所有権の放棄を明示しているが、`from_raw` は所有権を受け取る契約がシグネチャから読み取れない。

## 設計方針

`safe` のまま所有権を受け取る API を公開しない。以下のいずれかで修正する:

1. `unsafe fn from_raw` に変更し、`# Safety` コメントで「有効な refcounted ポインタを渡し、呼び出し元が参照カウント 1 つ分の所有権を譲渡すること」を明記する
2. 外部公開をやめて `pub(crate)` に変更する

内部の全呼び出し箇所は「C 側から所有権を譲渡された参照をラップする」パターンであり、呼び出し側が契約を守っているため、修正はシグネチャとドキュメントのみで済む見込み。

## 完了条件

- `ScopedRef::from_raw` が `unsafe fn` + `# Safety` ドキュメント付き、または `pub(crate)` であること
- 内部の呼び出し箇所すべてがコンパイル・テストに通ること

## 解決方法

- `ScopedRef::from_raw` を `unsafe fn` に変更し、`# Safety` を記載する
- `# Safety` の内容: `raw_ref` は有効な refcounted ポインタであり、呼び出し元が参照カウント 1 つ分の所有権を保持していること
- 内部呼び出し箇所 (`src/api/` 配下の `from_raw` / `from_refcounted_ptr` / `from_unique_ptr` 等) に `unsafe` ブロックを追加する
- 公開 API としての意味論 (所有権を奪う操作) を Rustdoc に明記する
