# convert_to_i420 が src_frame の必要長を検証していない

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-libyuv-convert-to-i420-src-len-check
- Polished: {YYYY-MM-DD}

## 目的

`convert_to_i420` (`src/libyuv.rs`) が入力バッファの長さを検証せず、短い `src_frame` に対して libyuv がバッファをオーバーリードする問題を修正する。

## 現状

`convert_to_i420` (`src/libyuv.rs` の同名関数) は出力先 (`dst_frame`) の必要長のみ `has_required_len` で検証し、入力 (`src_frame`) の長さを検証していない。

libyuv の `ConvertToI420` (libyuv の `convert_to_i420.cc`) は raw フォーマット (YUY2 / UYVY / ARGB 等) で `sample_size` を一切参照せず、`src = sample + (aligned_src_width * crop_y + crop_x) * 2` として幅 × 高さぶん読み込む。そのため短いスライスを渡すとヒープのオーバーリード (境界外読み取り) になる。

同じファイル内の他の変換関数 (`i420_copy` / `nv12_copy` / `yuy2_to_i420` 等) は入力も `has_required_len` で検証しており、`convert_to_i420` だけが未検証で不整合。

## 設計方針

`convert_to_i420` に入力側の `has_required_len` 検証を追加する。libyuv が読み込むバイト数はフォーマットと crop / scale 設定に依存するため、既存の他関数と同じ検証パターンを適用する。検証に失敗した場合は他の変換関数と同じ `Error::InvalidArgument` 系のエラーを返す。

## 完了条件

- 短い `src_frame` を渡した場合にエラーが返り、バッファのオーバーリードが発生しないこと
- 有効な入力に対して既存の変換結果が変わらないこと
- `tests/test_libyuv.rs` に短い入力のテストが追加されていること

## 解決方法

- `convert_to_i420` (`src/libyuv.rs`) に入力長の検証を追加する
- 他の変換関数 (`i420_copy` 等) の検証パターンに合わせ、フォーマット別の必要長計算を適用する
- `tests/test_libyuv.rs` に短い入力でエラーになるテストを追加する
