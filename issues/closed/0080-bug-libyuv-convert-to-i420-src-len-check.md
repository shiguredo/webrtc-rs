# convert_to_i420 が src_frame の必要長を検証していない

- Created: 2026-08-03
- Completed: 2026-08-27
- Branch: feature/fix-libyuv-convert-to-i420-src-len-check
- Polished: 2026-08-12

## 目的

`convert_to_i420` (`src/libyuv.rs`) が入力バッファの長さを検証せず、短い `src_frame` に対して libyuv がバッファをオーバーリードする問題を修正する。

## 現状

`convert_to_i420` (`src/libyuv.rs` の同名関数) は出力先 (`dst_y` / `dst_u` / `dst_v`) の必要長のみ `has_required_len` で検証し、入力 (`src_frame`) の長さを検証していない。

libyuv の `ConvertToI420` (`libyuv` の `convert_to_i420.cc`) は raw フォーマットで `sample_size` を一切参照せず、フォーマット別の係数で `src` ポインタを進めて幅 × 高さぶん読み込む（YUY2 / UYVY は 2 バイト/画素で `aligned_src_width` を使用、ARGB / BGRA / ABGR / RGBA は 4 バイト/画素で `src_width` を使用）。そのため短いスライスを渡すとヒープのオーバーリード (境界外読み取り) になる。

この API で到達可能なフォーマットは `LibyuvFourcc` (`src/libyuv.rs`) の `Argb` / `Bgra` / `Mjpg` の 3 つのみで、raw フォーマットの検証対象は ARGB / BGRA の 2 つである。MJPG は圧縮データのため幅 × 高さ × バイト/画素の計算が当てはまらず、libyuv 側が `sample_size` を参照して `ValidateJpeg` で検証する（`case FOURCC_MJPG: r = MJPGToI420(sample, sample_size, ...)`）。

raw フォーマット入力の変換関数群 (`i420_copy` / `nv12_copy` / `yuy2_to_i420` 等) は入力も `has_required_len` で検証しており、`convert_to_i420` だけが未検証で不整合。なお `mjpg_to_i420` / `mjpg_to_nv12` も入力未検証だが、これは圧縮サイズを事前計算できないためであり、libyuv 側の検証に委ねている。

## 設計方針

`convert_to_i420` に入力側の長さ検証を追加する。libyuv が読み込むバイト数はフォーマットと crop 設定に依存するため、crop オフセットを考慮した検証を行う:

- 検証対象は ARGB / BGRA（4 バイト/画素）の 2 フォーマットのみ。必要長は `(crop_y + abs(crop_height))` 行ぶんのオフセットと `(crop_x + crop_width)` 画素ぶんの行内オフセットを含めて計算する（`stride = src_width * 4`、`rows = crop_y + abs(crop_height)`、`row_bytes = (crop_x + crop_width) * 4`。crop_height の負値は libyuv 側で abs として処理される（垂直フリップは `src_height` の負値で発生）ため、検証も `abs(crop_height)` で計算する。乗算は `checked_mul`、加算は `checked_add` 等のオーバーフロー検知つきで行う）。負の `crop_x` / `crop_y` は libyuv 側のパラメータ検証で弾かれるため、本検証の対象外とする
- MJPG は入力長検証をスキップし、libyuv 側の検証 (`ValidateJpeg`) に委ねる（有効な JPEG を誤って拒否しないため）
- 検証に失敗した場合は他の変換関数と同じく `false` を返す（`convert_to_i420` は追加当初から bool を返す API であり、他の変換関数群との整合性のため）

## 完了条件

- 短い `src_frame`（ARGB / BGRA で必要長未満）を渡した場合に `false` が返ること（検証が unsafe FFI 呼び出しより先に走るため、libyuv が呼ばれずオーバーリードは構造的に発生しない）
- 有効な入力（ARGB / BGRA の正常系）に対して変換が `true` を返し、変換結果が変わらないこと（既存テストに ARGB / BGRA の正常系は存在しないため、正常系テストを新規追加して検証する）
- MJPG 入力の挙動が変わらないこと
- `tests/test_libyuv.rs` に短い入力（ARGB / BGRA）のテストが追加されていること（既存の `convert_to_i420_returns_false_when_sample_is_too_small` は MJPG で libyuv の `ValidateJpeg` が弾くため、本修正の検証にはならない。crop オフセット込みの必要長計算の誤りを検出するため、`crop_x` / `crop_y` が正のケースで「必要長ちょうどで `true` / 1 バイト不足で `false`」の境界テストも追加すること）
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリが追加されていること

## 解決方法

- `convert_to_i420` (`src/libyuv.rs`) に ARGB / BGRA の入力長検証を追加した（crop オフセット込みで `src_width * 4` の stride・`abs(crop_height)` 行・`(crop_x + crop_width) * 4` バイトぶんを `has_required_len` で検証し、不足時は `false` を返す）。オーバーフローは `checked_*` で検知する
- MJPG の場合は入力長検証をスキップし、libyuv 側の検証 (`ValidateJpeg`) に委ねる
- `tests/test_libyuv.rs` に ARGB / BGRA の入力長検証テストを追加した（crop オフセット込みの必要長ちょうどで `true`、必要長より 1 バイト短いと `false`、BGRA でも同じ検証分岐に入ることを確認）
