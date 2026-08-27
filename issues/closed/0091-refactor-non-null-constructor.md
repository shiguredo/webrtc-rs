# C API から返る null チェック済みポインタのコンストラクタ `NonNull::new(...).expect(...)` を共通ヘルパー化する

- Created: 2026-08-26
- Completed: 2026-08-28
- Branch: feature/refactor-non-null-constructor
- Polished: {YYYY-MM-DD}

## 目的

C API が返すポインタを `NonNull` に包む際の `NonNull::new(...).expect("BUG: ... が null を返しました")` という定型コードが大量に重複している。これを共通ヘルパーに置き換えて、各呼び出しを単純化するとともに、null 違反時の panic メッセージを統一する。挙動は一切変更しない。

## 現状

`src/` 配下 (テスト以外) で `NonNull::new` を使う箇所は 26 ファイルにわたり多数存在し、その多くが次の形で「C API が null を返したら panic する」ことを書き下している。

```rust
let raw = NonNull::new(unsafe { ffi::webrtc_XXX_new() })
    .expect("BUG: webrtc_XXX_new が null を返しました");
```

- コンストラクタ (`webrtc_XXX_new` / `webrtc_CreateYYY` / `webrtc_BuiltinZZZ_Create` など)
- C API が生ポインタで返す getter (`webrtc_XXX_get_name` など) を `NonNull` にする変換
- `_unique_get` / `_refcounted` などラッパー層の変換
- `*_vector_new` / `*_vector_get` / `std_string_vector_get` など vector 要素の取得

のみならず、panic メッセージ文字列も「`BUG: ... が null を返しました`」「`BUG: ... returned null`」「`BUG: ptr が null`」など、同じ意味のものが表記ゆれでばらついている。メッセージの重複を避けるために定型的な FFI 呼び出しを 1 つずつ手書きするのは、メンテナンス性を損なう。

`NonNull::new(...).ok_or(Error::NullPointer(...))?` で `Error` を返す形 (peer_connection.rs の out 引数) と、`NonNull::new(...).map(...)` で `Option` を返す形 (ssl_identity.rs / media_stream.rs) は、panic ではなく結果を返す意図的な設計であり、本 issue の対象外とする。

## 設計方針

- null なら panic する意味の共通ヘルパーを 1 箇所に定義する (例)

```rust
pub(crate) fn expect_non_null<T>(ptr: *mut T, what: &str) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| panic!("BUG: {what} が null を返しました"))
}
```

上の例で `{what}` 部分は 関数名 (`webrtc_XXX_new` など) を渡す想定。ヘルパーの形 (ジェネリック関数 or マクロ) は実装時に決めてよい。
- 呼び出し側は 1 行になり、panic メッセージ文字列の表記ゆれがなくなる

```rust
let raw = expect_non_null(unsafe { ffi::webrtc_XXX_new() }, "webrtc_XXX_new");
```

- panic メッセージは既存の主流である日本語の「BUG: ... が null を返しました」に統一する (AGENTS のログメッセージ英語化の対象はログであって panic メッセージではないため、既存スタイルに従う)
- C API (`webrtc/`) と bindgen 生成の FFI 定義は変更しない
- 例外テキスト「BUG: ok != 0 なのに out が null」のような、null 以外も panic 理由に含むメッセージは、意味を失わないよう個別に扱う

## 完了条件

- `NonNull::new(...).expect("BUG: ...")` による「C API が返したポインタを null チェックして panic」する定型がすべて共通ヘルパー経由に置き換わっている
- panic メッセージの表記が統一されている (同じ意味のメッセージに表記ゆれが残っていない)
- 挙動がリファクタ前と同一である (panic の発生条件とメッセージの意味が変わらない)
- ビルドと全テストが通る

## 解決方法

- `src/non_null.rs` に `expect_non_null` / `expect_non_null_with_cleanup` を追加し、`src/lib.rs` で `mod non_null;` を宣言する
- 対象の 26 ファイルで、定型的な `NonNull::new(...).expect("BUG: ...")` と、observer 生成時の match + panic (null 時に state を回収してから panic) を共通ヘルパー呼び出しに置き換える
- panic メッセージは主流の日本語「BUG: {what} が null を返しました」に統一する (`what` には呼び出した関数名を渡す)
- 英語 `returned null` / 「BUG:」なし / 短縮形 (`ptr が null` など) の表記ゆれを解消する
- 例外テキスト (`ok != 0 なのに out が null` / `out_pc と out_error が両方 null` / `index が X なのにアクセサが null`) は意味を損なわないよう個別に扱い、置換しない
- 対象外 (`ok_or` / `?` / `.map()` / `if let Some` / チェック済み `unwrap()`) は変更しない
- `src/tests.rs` の既存テストとビルドで挙動が変わらないことを検証する
- `CHANGES.md` の develop に `### misc` エントリを追記する
