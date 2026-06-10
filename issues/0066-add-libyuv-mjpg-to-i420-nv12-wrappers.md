# libyuv の MJPGToI420 / MJPGToNV12 ラッパーを追加する

- Priority: Medium
- Created: 2026-06-10
- Completed: {YYYY-MM-DD}
- Model: Opus 4.7
- Branch: feature/add-libyuv-mjpg-to-i420-nv12-wrappers
- Polished: 2026-06-10

## 目的

libwebrtc に同梱されている libyuv の MJPEG (Motion JPEG) 系変換関数 `MJPGToI420` / `MJPGToNV12` を、webrtc-rs の C ラッパー層と Rust safe wrapper 層を通じて利用できるようにする。これにより UVC カメラなどが MJPG フォーマットで出力するフレームを、追加の外部ライブラリを使わずに I420 / NV12 へ変換できるようにする。

## 優先度根拠

新規ユースケース (MJPG カメラ入力の取り扱い) を開く機能追加だが、現状ブロッキングではなく、既存 libyuv ラッパーへの薄い追加で副作用が小さいため Medium とする。

## 現状

webrtc-rs の libyuv ラッパー層は以下の状態:

- C ラッパー (`webrtc/src/webrtc_c/libyuv.h` / `libyuv.cc`) は libyuv の以下 8 関数を公開している
  - `libyuv_ABGRToI420` / `libyuv_ConvertFromI420` / `libyuv_NV12ToI420` / `libyuv_I420ToNV12` / `libyuv_I420Copy` / `libyuv_NV12Copy` / `libyuv_YUY2ToI420` / `libyuv_I420Rotate`
- Rust safe wrapper (`src/libyuv.rs`) は上記のうち `libyuv_I420Rotate` を除く 7 関数 (`abgr_to_i420` / `convert_from_i420` / `i420_to_nv12` / `nv12_to_i420` / `i420_copy` / `nv12_copy` / `yuy2_to_i420`) を公開している。`i420_rotate` 相当の Rust safe wrapper は未公開だが本 issue のスコープ外
- C 層・Rust 層のいずれにも MJPEG からの変換関数は公開されておらず、MJPG カメラ入力を webrtc-rs だけで I420 / NV12 化する手段が存在しない

prebuilt 側の状況:

- webrtc-rs 0.150.0 の macOS arm64 prebuilt (`libwebrtc_c-macos_arm64.tar.gz`) で `libyuv::MJPGToI420` / `libyuv::MJPGToNV12` のシンボルが含まれていることを確認済み
- shiguredo-webrtc-build の GN args は `libyuv_disable_jpeg` を明示していない (デフォルトで MJPEG サポート有効)

**iOS の制約**: libyuv の `BUILD.gn` で `if (!is_ios && !libyuv_disable_jpeg) { defines += [ "HAVE_JPEG" ] }` というガードがあり、iOS ビルドでは `HAVE_JPEG` が未定義になる。`MJPGToI420` / `MJPGToNV12` は `convert_jpeg.cc` の `#ifdef HAVE_JPEG` 内にあるため、iOS prebuilt にはこれらのシンボルが含まれない。本 issue では C ラッパー側で `WEBRTC_IOS` ガードを使ってこの制約に対応する (後述「解決方法 1」)。

## 設計方針

既存 libyuv ラッパーと同一の 2 層構造を踏襲する。新たな依存関係や設定変更は導入しない。

### 公開関数の範囲

- 公開する関数は次の 2 つに絞る
  - `MJPGToI420` (MJPG → I420)
  - `MJPGToNV12` (MJPG → NV12)
- スコープ外
  - `MJPGToNV21` / `MJPGToARGB` / `MJPGSize` / `ConvertToI420` / `ConvertToARGB` / `libyuv::MJpegDecoder`

### libyuv の引数仕様 (API として継承する重要な前提)

libyuv `MJPGToI420` / `MJPGToNV12` は **2 段階の寸法チェック** を行う:

1. `convert_jpeg.cc:130-135` (MJPGToI420) / `convert_jpeg.cc:403-408` (MJPGToNV12) で `mjpeg_decoder.GetWidth() != src_width || GetHeight() != src_height` の場合 `return 1`
2. `mjpeg_decoder.cc:336-411` の `DecodeToCallback` で `dst_width != GetWidth() || dst_height > GetHeight()` の場合 `LIBYUV_FALSE`

つまり実際の制約は:

- `src_width` / `src_height`: 入力 MJPG にエンコードされている元解像度。**JPEG ヘッダの値と完全一致が必須** (1 段目で弾かれる)
- `dst_width`: **`src_width` と完全一致が必須** (水平方向の scaling / cropping は不可、2 段目で弾かれる)
- `dst_height`: `src_height` 以下なら可 (`==` で等倍、`<` で vertical crop)
- libyuv 内部に scaling 機構 (例: libjpeg-turbo の `scale_num`) は組み込まれていない (cropping は `DecodeToCallback` 内の vertical crop のみ)

したがって本 issue のテストでは `src_width == dst_width`、`src_height == dst_height` で等倍デコードを行う。リサイズが必要な場合は別途 `I420Scale` 系の API が必要だが本 issue のスコープ外。

参考: `convert_jpeg.cc:107-108` に libyuv 上流自身が `// TODO(fbarchard): review src_width and src_height requirement. dst_width and dst_height may be enough.` というコメントを残しており、将来のバージョンで API 仕様が変わる可能性がある。

Rust safe wrapper の doc コメントにもこの制約を明記する。

### 戻り値

libyuv `MJPGToI420` / `MJPGToNV12` は成功時 `0`、失敗時 `0` 以外 (`-1` または `1`) を返す。Rust safe wrapper は既存ラッパー (例: `i420_to_nv12`) と同じく `bool` を返し、**libyuv が `0` を返した場合のみ `true`、それ以外の戻り値および事前検証で弾いた場合は `false`** とする。失敗理由を呼び出し側に伝えないトレードオフは認識した上で、既存ラッパーとの対称性を優先する。

### 命名・スタイル

- 既存ラッパーの命名 (`libyuv_XxxxToYyyy` → Rust 側 `xxxx_to_yyyy`) と引数並びを踏襲し、対称性を維持する
- `#[allow(clippy::too_many_arguments)]` は既存 libyuv ラッパー全 7 関数と同じく `#[allow]` を使う (`shiguredo-rust` 規約は `#[expect]` を推奨するが、本 issue では既存スタイル踏襲を優先)
- テストは既存 libyuv テストと同じく `src/tests.rs` へ追記する (webrtc-rs リポジトリには `tests/` も `pbt/` ディレクトリも現状存在しない)
- PBT (proptest) は適用しない (MJPG decode 専用で生成 strategy が定義しにくいため)

## 完了条件

1. `webrtc/src/webrtc_c/libyuv.h` と `webrtc/src/webrtc_c/libyuv.cc` に `libyuv_MJPGToI420` と `libyuv_MJPGToNV12` が追加され、`libyuv::MJPGToI420` / `libyuv::MJPGToNV12` を呼び出す薄いラッパーとして動作している (`.cc` 側にも既存パターンと同じく `WEBRTC_EXPORT` を付与)
2. iOS ビルドでリンクエラーが起きないよう以下のいずれかが満たされている
   - (a) iOS prebuilt にシンボルが含まれていない場合: `libyuv.cc` の関数本体内で `#if defined(WEBRTC_IOS)` ガードを使い、iOS では `libyuv::MJPGToI420` / `libyuv::MJPGToNV12` の呼び出しを行わず `1` (失敗) を返すスタブとして実装されている
   - (b) iOS prebuilt にシンボルが含まれていることが事前確認 (「解決方法 1 > iOS 対応 > 事前確認手順」) で判明した場合: iOS ガードを入れずに通常実装で良い
3. `src/libyuv.rs` に `mjpg_to_i420` と `mjpg_to_nv12` の Rust safe wrapper が追加され、`src/lib.rs` の `pub use libyuv::{...}` (アルファベット順を維持して `i420_to_nv12` と `nv12_copy` の間) に追加されている
4. Rust safe wrapper の事前検証は dst バッファ長 (`dst_width` / `dst_height` 基準) と、chroma 寸法計算 / row bytes 計算の overflow チェックを行う (既存 7 関数とパターンを揃える)
5. Rust safe wrapper の戻り値は **libyuv が `0` を返した場合のみ `true`、それ以外および事前検証で弾いた場合は `false`** とする
6. `src/tests.rs` に以下のテストが追加され、`cargo test --features source-build --workspace` がローカルで通る
   - 正常系 (4 件): `mjpg_to_i420_decodes_gray_frame` / `mjpg_to_i420_decodes_red_frame` / `mjpg_to_nv12_decodes_gray_frame` / `mjpg_to_nv12_decodes_red_frame`
   - 異常系 (6 件): `mjpg_to_i420_returns_false_when_destination_plane_is_too_short` / `mjpg_to_nv12_returns_false_when_destination_plane_is_too_short` / `mjpg_to_i420_returns_false_when_src_dimensions_do_not_match` / `mjpg_to_nv12_returns_false_when_src_dimensions_do_not_match` / `mjpg_to_i420_returns_false_when_sample_is_too_small` / `mjpg_to_nv12_returns_false_when_sample_is_too_small`
7. README.md の「## 対応 API」セクションの libyuv 変換関数の行に `mjpg_to_i420` / `mjpg_to_nv12` が追記されている
8. `CHANGES.md` の `## develop` 直下に ADD エントリが追加されている
9. `.github/workflows/ci.yml` の以下 5 ジョブが全て成功する (`slack_notify` は通知用のため対象外)
   - `ci` (Ubuntu / macOS / Windows マトリクスで `cargo test` 等を実行)
   - `cross-compile` (Ubuntu arm64 / Raspberry Pi OS arm64 のクロスコンパイル)
   - `verify-linux-arm64` (ARM64 実機での起動確認)
   - `build-ios` (iOS arm64 への `cargo build`)
   - `build-android` (Android arm64 への `cargo build`)

## 解決方法

### 1. C ラッパー層の追加

`webrtc/src/webrtc_c/libyuv.h` に既存の `libyuv_XxxxToYyyy` 群と同じスタイルで以下の宣言を追加する (iOS でも宣言は共通で OK)。

```c
WEBRTC_EXPORT int libyuv_MJPGToI420(const uint8_t* sample,
                                    size_t sample_size,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_u,
                                    int dst_stride_u,
                                    uint8_t* dst_v,
                                    int dst_stride_v,
                                    int src_width,
                                    int src_height,
                                    int dst_width,
                                    int dst_height);

WEBRTC_EXPORT int libyuv_MJPGToNV12(const uint8_t* sample,
                                    size_t sample_size,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_uv,
                                    int dst_stride_uv,
                                    int src_width,
                                    int src_height,
                                    int dst_width,
                                    int dst_height);
```

`webrtc/src/webrtc_c/libyuv.cc` 側は `libyuv::MJPGToI420` / `libyuv::MJPGToNV12` を呼び出す薄い実装にする。`.cc` 側にも既存の `libyuv_ABGRToI420` 等と同じく `WEBRTC_EXPORT` を付与する (`common.h` で Windows は `__declspec(dllexport)`、Linux / macOS は `__attribute__((visibility("default")))` に展開される)。

#### iOS 対応

libyuv の `BUILD.gn` ガード (`if (!is_ios && !libyuv_disable_jpeg)`) により、iOS ビルドでは `HAVE_JPEG` が未定義となり `libyuv::MJPGToI420` / `libyuv::MJPGToNV12` のシンボルが存在しない可能性が高い。`libyuv.cc` 内で `libyuv::MJPGToI420` を素直に呼び出すと iOS で undefined symbol のリンクエラーになるため、関数本体内で `#if defined(WEBRTC_IOS)` を使って呼び出しごとガードする。

`WEBRTC_IOS` マクロは `webrtc/CMakeLists.txt` の `ios_arm64` ターゲット時 (`WEBRTC_C_TARGET=ios_arm64`) に `target_compile_definitions` で `webrtc_c` ターゲット全体に対して定義される (libwebrtc でも同名・同用途で使われるマクロを、webrtc-rs 側の CMakeLists.txt が再定義する形)。同じ「同ファイル内で iOS のみ別実装にする」前例として `webrtc/src/webrtc_c/api/audio/audio_device.cc:779` で `#if defined(WEBRTC_IOS)` が使われている。追加の include は不要。

実装例:

```cpp
WEBRTC_EXPORT int libyuv_MJPGToI420(const uint8_t* sample,
                                    size_t sample_size,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_u,
                                    int dst_stride_u,
                                    uint8_t* dst_v,
                                    int dst_stride_v,
                                    int src_width,
                                    int src_height,
                                    int dst_width,
                                    int dst_height) {
#if defined(WEBRTC_IOS)
  // iOS は libyuv の HAVE_JPEG が無効化されているため MJPGToI420 シンボルが
  // 存在しない。リンクエラーを防ぐため呼び出しごとガードし、libyuv の失敗時
  // と同じ 1 を返す。Rust 層は 0 以外を false に変換するため、iOS 上での
  // 呼び出しは常に false を返す。
  (void)sample; (void)sample_size;
  (void)dst_y; (void)dst_stride_y;
  (void)dst_u; (void)dst_stride_u;
  (void)dst_v; (void)dst_stride_v;
  (void)src_width; (void)src_height;
  (void)dst_width; (void)dst_height;
  return 1;
#else
  return libyuv::MJPGToI420(sample, sample_size, dst_y, dst_stride_y,
                            dst_u, dst_stride_u, dst_v, dst_stride_v,
                            src_width, src_height, dst_width, dst_height);
#endif
}
```

`libyuv_MJPGToNV12` も同じパターン。`(void)` キャストは `-Wunused-parameter` 警告抑制のため。

##### 事前確認手順

実装着手時に iOS prebuilt にシンボルが含まれているかを確認し、完了条件 2 のどちら ((a) or (b)) を取るかを判定する:

1. shiguredo-webrtc-build の iOS prebuilt または webrtc-rs の prebuilt キャッシュ内の `libwebrtc.a` に対して以下のコマンドを実行する
   - `nm -g <libwebrtc.a パス> 2>/dev/null | c++filt | grep -i MJPGToI420`
   - `c++filt` を経由しないと C++ シンボルが mangle されたまま表示されるため、`libyuv::MJPGToI420(...)` の形での照合がしづらい
2. `T libyuv::MJPGToI420(...)` のように `T` (text section) で見えれば実装が含まれている (完了条件 2 (b) → iOS ガード不要)。`U libyuv::MJPGToI420` (undefined) のみ、もしくは grep 結果が空ならシンボルは含まれていない (完了条件 2 (a) → iOS ガード必須)

### 2. Rust safe wrapper の追加

`src/libyuv.rs` に既存パターンに揃えて以下を追加する。

```rust
/// `libyuv::MJPGToI420` を呼び出して、MJPEG バイト列を I420 へ変換する。
///
/// - `sample`: MJPG (baseline JPEG) のバイト列。
/// - `src_width` / `src_height`: 入力 MJPG にエンコードされている元解像度。
///   JPEG ヘッダの値と一致しない場合、libyuv 内部で失敗扱いとなり `false` を返す。
/// - `dst_width` / `dst_height`: 出力 I420 の解像度。
///   libyuv は `dst_width == src_width` 必須かつ `dst_height <= src_height` のみ
///   受け付ける (vertical crop のみ可、水平リサイズ・スケーリング不可)。
///
/// 変換に成功した場合 (libyuv が `0` を返した場合) のみ `true` を返す。
/// それ以外 (libyuv が `-1` または `1` を返した場合、事前検証に失敗した場合) は `false`。
/// iOS など MJPG サポートを含まないビルドでは常に `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn mjpg_to_i420(
    sample: &[u8],
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_u: &mut [u8],
    dst_stride_u: i32,
    dst_v: &mut [u8],
    dst_stride_v: i32,
    src_width: i32,
    src_height: i32,
    dst_width: i32,
    dst_height: i32,
) -> bool { /* ... */ }

/// `libyuv::MJPGToNV12` を呼び出して、MJPEG バイト列を NV12 へ変換する。
/// 仕様は `mjpg_to_i420` と同じ。
#[allow(clippy::too_many_arguments)]
pub fn mjpg_to_nv12(
    sample: &[u8],
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_uv: &mut [u8],
    dst_stride_uv: i32,
    src_width: i32,
    src_height: i32,
    dst_width: i32,
    dst_height: i32,
) -> bool { /* ... */ }
```

実装上の検証ルール (既存 `i420_to_nv12` の事前検証と対称):

- chroma サイズは **`i420_chroma_size(dst_width, dst_height)`** で取得する (`src_*` ではなく `dst_*` を渡すことに注意)。`None` が返ったら `false`
- NV12 の UV row bytes は `chroma_width.checked_mul(2)` でオーバーフロー検証する。`None` が返ったら `false`
- dst バッファ長は `has_required_len(dst_y.len(), dst_stride_y, dst_height, dst_width)` 等を使い、すべて `dst_width` / `dst_height` 基準で検証する
- `sample` の長さや内容に関する事前チェックは行わず libyuv に委ねる (既存 7 関数も sample 内容のチェックはしていない)
- `src_width` / `src_height` の正値性や MJPG ヘッダ寸法との一致確認もラッパー側では行わず libyuv に委ねる
- すべての事前検証を通過したら `unsafe { ffi::libyuv_MJPG*(...) } == 0` を返す

### 3. 再エクスポート

`src/lib.rs` の `pub use libyuv::{...};` に、**アルファベット順を維持しつつ** `mjpg_to_i420` と `mjpg_to_nv12` を `i420_to_nv12` の直後 (`nv12_copy` の前) に挿入する。

```rust
pub use libyuv::{
    LibyuvFourcc, abgr_to_i420, convert_from_i420, i420_copy, i420_to_nv12, mjpg_to_i420,
    mjpg_to_nv12, nv12_copy, nv12_to_i420, yuy2_to_i420,
};
```

### 4. テスト

`src/tests.rs` の既存 libyuv テストブロックの末尾 (`nv12_copy_returns_false_when_destination_plane_is_too_short` の後) に以下を追加する。既存パターンは `abgr_to_i420_conversion` (正常系) と `i420_copy_returns_false_when_*` (異常系) を参照する。

CLAUDE.md「テストはコメントを重視すること」に従い、各テスト関数の冒頭に目的を、各 const 宣言の直上に (a) 生成スクリプトの該当呼び出し / (b) 期待される Y / U / V の理論値 / (c) 許容範囲の根拠を日本語コメントで記述する。

#### テスト用 JPEG リテラルの生成方針

- テストデータは 2 つの固定 JPEG リテラルを `src/tests.rs` の libyuv テスト群直前 (`abgr_to_i420_conversion` の前) に inline 配列定数 (`const TEST_MJPG_GRAY_8X8: &[u8] = &[0xff, 0xd8, ...];`) として並べる
  - `TEST_MJPG_GRAY_8X8`: 全画素 RGB=(128, 128, 128) の 8x8 baseline JPEG。BT.601 JFIF full-range 換算で Y=128 / U=128 / V=128 になる
  - `TEST_MJPG_RED_8X8`: 全画素 RGB=(255, 0, 0) の 8x8 baseline JPEG。JFIF full-range の式 (`Y = 0.299 R + 0.587 G + 0.114 B`, `U(Cb) = -0.1687 R - 0.3313 G + 0.5 B + 128`, `V(Cr) = 0.5 R - 0.4187 G - 0.0813 B + 128`) で換算すると Y=76.245 / U=84.98 / V=255.5 (V は 8-bit clip で 255)。**U と V が別の値になる色を選ぶことで U/V 取り違えバグを検出可能にする**
- 生成手段は **Pillow (Python)** に一本化する (ImageMagick はカラースペース設定の癖で baseline JPEG にならないケースがあるため避ける)
- 生成スクリプト (1 度実行すれば `mjpg_consts.rs.txt` に貼り付け可能な const 定義が出力される。実装者は内容を `src/tests.rs` の libyuv テスト群直前にコピーする):

  ```python
  from pathlib import Path
  from PIL import Image

  JPEG_OPTIONS = {
      "format": "JPEG",
      "subsampling": 2,       # YUV 4:2:0
      "quality": 90,
      "progressive": False,   # baseline JPEG (libyuv は baseline のみ対応)
      "optimize": False,
  }

  out_lines = []
  for name, rgb in [("GRAY", (128, 128, 128)), ("RED", (255, 0, 0))]:
      img = Image.new("RGB", (8, 8), rgb)
      jpg_path = Path(f"test_mjpg_{name.lower()}_8x8.jpg")
      img.save(jpg_path, **JPEG_OPTIONS)
      data = jpg_path.read_bytes()
      hex_bytes = ", ".join(f"0x{b:02x}" for b in data)
      out_lines.append(
          f"/// Pillow quality=90 subsampling=2 baseline JPEG (8x8 RGB={rgb})\n"
          f"const TEST_MJPG_{name}_8X8: &[u8] = &[{hex_bytes}];\n"
      )
  Path("mjpg_consts.rs.txt").write_text("\n".join(out_lines))
  ```

- libyuv が受け付ける条件: baseline JPEG (`progressive=False`)、YUV 4:2:0 / 4:2:2 / 4:4:4 もしくは Grayscale YUV 4:0:0、8-bit precision、Huffman coding。Pillow デフォルト + 上記設定で満たす
- 8x8 quality=90 の JPEG は実測で 600〜700 byte 程度。`src/tests.rs` の肥大化は許容範囲内とし、外部ファイル化 (`include_bytes!`) は採用しない (既存テストファイルに `include_bytes!` の前例が無い)

#### 追加するテストケース

許容範囲の方針:

- **Y plane**: 単色 JPEG の DC 量子化誤差で `±4` を上限値とする (libjpeg quality=90 で実測 ±2〜±3 程度)。実機計測で必要なら締める
- **U / V plane (中央値付近)**: 同じく `±4` を上限値とする
- **U / V plane (飽和値 = 255 付近)**: 理論値 255.5 が 8-bit clip で 255 に張り付くため、`240 <= v <= 255` の範囲指定とする (chroma サブサンプリング 4:2:0 でブロック平均が取られるため、彩度が高い色は chroma 系で量子化誤差が広めに出る経験則がある。実機計測で必要なら締める)

正常系:

- `mjpg_to_i420_decodes_gray_frame`: `TEST_MJPG_GRAY_8X8` を `src_width=8, src_height=8, dst_width=8, dst_height=8` で `mjpg_to_i420` に渡し、戻り値 `true`、Y plane (64 byte) / U plane (16 byte) / V plane (16 byte) の全 byte が `128 ± 4` の範囲であることを assert
- `mjpg_to_nv12_decodes_gray_frame`: 同じ JPEG リテラルを `mjpg_to_nv12` に渡し、戻り値 `true`、Y plane (64 byte) と interleaved UV plane (32 byte) の全 byte が `128 ± 4` の範囲であることを assert
- `mjpg_to_i420_decodes_red_frame`: `TEST_MJPG_RED_8X8` を渡し、戻り値 `true`、Y plane (64 byte) が `76 ± 4`、U plane (16 byte) が `85 ± 4`、V plane (16 byte) が `240 <= v <= 255` であることを assert
- `mjpg_to_nv12_decodes_red_frame`: `TEST_MJPG_RED_8X8` を `mjpg_to_nv12` に渡し、戻り値 `true`、Y plane (64 byte) が `76 ± 4`、UV plane (32 byte) の偶数 index (U) が `85 ± 4`、奇数 index (V) が `240 <= v <= 255` であることを assert (NV12 の UV interleave 順を検証)

異常系 (既存 libyuv 異常系テスト命名 `*_returns_false_when_destination_plane_is_too_short` に揃える):

- `mjpg_to_i420_returns_false_when_destination_plane_is_too_short`: dst_y / dst_u / dst_v のいずれか (代表的に dst_y) を必要サイズ - 1 にして `false` が返ることを assert (事前検証で弾かれる)
- `mjpg_to_nv12_returns_false_when_destination_plane_is_too_short`: 同上 (NV12 の dst_y もしくは dst_uv のいずれか)
- `mjpg_to_i420_returns_false_when_src_dimensions_do_not_match`: `TEST_MJPG_GRAY_8X8` (8x8) を `src_width=16, src_height=16, dst_width=16, dst_height=16` で渡し、dst_y / dst_u / dst_v は 16x16 用 (256 + 64 + 64 byte) を確保する。Rust 側事前検証は通過し、libyuv 側 `convert_jpeg.cc:130-135` の `mjpeg_decoder.GetWidth() != src_width` チェックで `1` が返るため `false` が返ることを assert (libyuv 戻り値 → false 変換の回帰検出)
- `mjpg_to_nv12_returns_false_when_src_dimensions_do_not_match`: 同上 (NV12 版、libyuv 側は `convert_jpeg.cc:403-408` の同等チェックで `1` が返る)
- `mjpg_to_i420_returns_false_when_sample_is_too_small`: `&[0xff, 0xd8, 0xff, 0xd9]` のような 64 byte 未満のバイト列を渡し、libyuv 側の `ValidateJpeg` (`mjpeg_validate.cc`) が `src_size_mjpg < 64` の条件で弾くため `false` が返ることを assert
- `mjpg_to_nv12_returns_false_when_sample_is_too_small`: 同上 (NV12 版)

テスト関数とテスト const には日本語コメントを残す。`assert!` メッセージは英語でも日本語でもどちらでもよいが、`assert_eq!` / `assert!` の意図補足はコメントで日本語で書く方針とする。

### 5. ドキュメント更新

README.md の「## 対応 API」セクションの libyuv 変換関数の行 (現状 `abgr_to_i420` / `convert_from_i420` / `i420_to_nv12` / `nv12_to_i420` / `yuy2_to_i420` が並んでいる箇所) に `mjpg_to_i420` / `mjpg_to_nv12` をアルファベット順 (`i420_to_nv12` と `nv12_to_i420` の間) に追記する。

```
- `abgr_to_i420` / `convert_from_i420` / `i420_to_nv12` / `mjpg_to_i420` / `mjpg_to_nv12` / `nv12_to_i420` / `yuy2_to_i420`
  - カラーフォーマット変換 (libyuv)
```

### 6. 変更履歴

`shiguredo-changelog` スキルの規約に従い、`CHANGES.md` の `## develop` 直下 (`### misc` 配下ではなく本体) に以下の ADD エントリを追加する。既存の `i420_copy` / `nv12_copy` エントリ (0.147.1) と同じフォーマットに揃える。

```
- [ADD] libyuv の `mjpg_to_i420` / `mjpg_to_nv12` API を追加する
  - C API `libyuv_MJPGToI420` / `libyuv_MJPGToNV12` を追加し、Rust API `mjpg_to_i420` / `mjpg_to_nv12` を利用できるようにする
  - @<implementer>
```

### 7. ビルド・動作確認手順

新規 C シンボル (`libyuv_MJPGToI420` / `libyuv_MJPGToNV12`) を Rust 側から呼ぶには `bindings.rs` の再生成が必要なため、prebuilt 経路 (デフォルト) では既存配布物の `bindings.rs` に新シンボルが無くコンパイルエラーになる。開発時は source-build feature でビルド・テストする:

- `cargo build --features source-build --workspace`
- `cargo test --features source-build --workspace -- mjpg` で新規テストのみフィルタ実行 (`-- mjpg` は新規テスト関数名のサフィックス・プレフィックス全マッチ、workspace 内の他クレートに `mjpg` を含むテストは無し)
- 最終確認では `-- mjpg` を外して `cargo test --features source-build --workspace` を実行し、既存テスト群との回帰がないことを確認する
