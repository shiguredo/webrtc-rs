# libyuv の I420Rotate に対応する Rust safe wrapper を追加する

- Priority: Low
- Created: 2026-06-10
- Completed: {YYYY-MM-DD}
- Model: Opus 4.7
- Branch: feature/add-libyuv-i420-rotate-wrapper
- Polished: {YYYY-MM-DD}

## 目的

libyuv の `I420Rotate` (I420 フレームの 0°/90°/180°/270° 回転) を webrtc-rs から Rust safe wrapper として呼び出せるようにする。C 層には既に `libyuv_I420Rotate` が公開されているが、Rust safe wrapper が未公開のため Rust 側から呼び出せない状態を解消する。

## 優先度根拠

C 層には既に `libyuv_I420Rotate` が公開されており、unsafe で呼び出せば現状でも動作する (回避策あり)。Rust safe wrapper が無いことで開発が止まる状況ではないため Low とする。一方で C 層 8 関数に対して Rust 層が 7 関数という非対称性は libyuv ラッパー全体の整合性を損なうため、放置せず追加する。

## 現状

- C 層 (`webrtc/src/webrtc_c/libyuv.h:103-117`, `webrtc/src/webrtc_c/libyuv.cc:139-158`) に `libyuv_I420Rotate` が公開されている
- Rust 層 (`src/libyuv.rs`) には `i420_rotate` 相当の safe wrapper が存在しない。`src/lib.rs:23-26` の `pub use libyuv::{...}` にも含まれていない
- C 層が公開している libyuv 8 関数 (`libyuv_ABGRToI420` / `libyuv_ConvertFromI420` / `libyuv_NV12ToI420` / `libyuv_I420ToNV12` / `libyuv_I420Copy` / `libyuv_NV12Copy` / `libyuv_YUY2ToI420` / `libyuv_I420Rotate`) のうち、Rust 層が公開しているのは 7 関数 (`abgr_to_i420` / `convert_from_i420` / `i420_to_nv12` / `nv12_to_i420` / `i420_copy` / `nv12_copy` / `yuy2_to_i420`) のみ

この非対称性は issue 0066 (libyuv の MJPGToI420 / MJPGToNV12 ラッパー追加) の磨き上げレビュー過程で判明した。

## 設計方針

既存 libyuv ラッパーと同一の 2 層構造を踏襲する。新たな依存関係や設定変更は導入しない。

- 既存ラッパーの命名 (`libyuv_XxxxToYyyy` → Rust 側 `xxxx_to_yyyy`) と引数並びを踏襲し、Rust 側関数名は `i420_rotate` とする
- libyuv の `RotationMode` (`kRotate0=0` / `kRotate90=90` / `kRotate180=180` / `kRotate270=270`) は既存 `LibyuvFourcc` enum と同じパターンで `LibyuvRotationMode` enum として Rust 側に公開する (C 層は既に `int mode` で受けて `static_cast<libyuv::RotationMode>(mode)` でキャストしている)
- 90°/270° 回転時に dst の width と height が src と逆になる仕様 (`dst_width = src_height`, `dst_height = src_width`) を doc コメントに明記し、dst バッファ長検証も回転後の解像度を基準に行う
- 戻り値は既存ラッパーと同じく `bool` (libyuv が `0` を返したら `true`、それ以外は `false`)
- `#[allow(clippy::too_many_arguments)]` は既存 libyuv ラッパー全 7 関数と同じく `#[allow]` を使う
- テストは既存 libyuv テストと同じく `src/tests.rs` へ追記する

## 完了条件

1. `src/libyuv.rs` に `i420_rotate` の Rust safe wrapper と `LibyuvRotationMode` enum (`Rotate0` / `Rotate90` / `Rotate180` / `Rotate270`) が追加されている
2. `src/lib.rs` の `pub use libyuv::{...}` に `i420_rotate` と `LibyuvRotationMode` が追加されている (アルファベット順を維持)
3. Rust safe wrapper の事前検証は src/dst バッファ長 (90°/270° の場合は dst を回転後の解像度基準) と overflow チェックを行う (既存 7 関数とパターンを揃える)
4. 戻り値は libyuv が `0` を返した場合のみ `true`、それ以外および事前検証で弾いた場合は `false`
5. `src/tests.rs` に正常系 (0°/90°/180°/270° の 4 種の round-trip テストもしくは既知パターンでの回転結果検証) と異常系 (dst バッファ長不足) のテストが追加され、`cargo test` が通る
6. README.md の「## 対応 API」セクションの libyuv 変換関数の行に `i420_rotate` が追記されている
7. `CHANGES.md` の `## develop` 直下に ADD エントリが追加されている

## 解決方法

### 1. Rust safe wrapper の追加

`src/libyuv.rs` に既存 `LibyuvFourcc` パターンに倣って `LibyuvRotationMode` enum を追加する。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibyuvRotationMode {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl LibyuvRotationMode {
    fn as_raw(self) -> i32 {
        match self {
            LibyuvRotationMode::Rotate0 => 0,
            LibyuvRotationMode::Rotate90 => 90,
            LibyuvRotationMode::Rotate180 => 180,
            LibyuvRotationMode::Rotate270 => 270,
        }
    }
}
```

`i420_rotate` 本体は既存 `i420_copy` の検証パターンを踏襲する。90°/270° 回転時は dst の chroma サイズも回転後の解像度から導出する必要がある (例: `i420_chroma_size(height, width)` で取得)。

### 2. 再エクスポート

`src/lib.rs` の `pub use libyuv::{...};` に `i420_rotate` と `LibyuvRotationMode` をアルファベット順で挿入する。

### 3. テスト

`src/tests.rs` に既存 libyuv テストパターンに沿って追加:

- `i420_rotate_rotate_0_preserves_planes`: 0° 回転で入力と一致
- `i420_rotate_rotate_180_inverts_pixel_order`: 180° 回転で水平・垂直反転を検証
- `i420_rotate_rotate_90_swaps_dimensions`: 90° 回転で width/height が入れ替わる
- `i420_rotate_rotate_270_swaps_dimensions`: 270° 回転で width/height が入れ替わる
- `i420_rotate_returns_false_when_destination_plane_is_too_short`: dst バッファ長不足で `false`

### 4. ドキュメント更新

README.md の「## 対応 API」セクションの libyuv 変換関数の行に `i420_rotate` を追記する。

### 5. 変更履歴

`CHANGES.md` の `## develop` 直下に以下を追加:

```
- [ADD] libyuv の `i420_rotate` API を追加する
  - C API `libyuv_I420Rotate` は既に公開済み、Rust API `i420_rotate` と `LibyuvRotationMode` enum を追加する
  - @<implementer>
```
