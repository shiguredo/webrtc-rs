# whip.cpp の未使用 frame_counter_ を削除する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-cpp-remove-frame-counter

## 目的

`webrtc/src/whip.cpp` の `class FakeVideoCapturer` には、代入・インクリメントはされるが一度も読み出されない `frame_counter_` メンバが存在する。出力にもログにも使われていないデッドコードであるため、関連する代入とともに削除する。

## 優先度根拠

ビルドや動作には影響しないデッドコードであり Low とする。ただし未使用メンバはクラスの意図を曖昧にするため整理する価値はある。

## 現状

レビュー時点で実コードと `rg` による参照状況を確認済み。`whip.cpp:529` で `frame_counter_` が宣言されている。

```cpp
  std::unique_ptr<uint32_t[]> image_;
  uint32_t frame_counter_ = 0;
```

`frame_counter_` は次の箇所で代入・インクリメントされる。`whip.cpp:365`（`StartCapture` 内）と `whip.cpp:387`（`CaptureThread` 内）で `frame_counter_ = 0;`、`whip.cpp:423`（`CaptureThread` のループ内）で `frame_counter_ += 1;`。

```cpp
        std::this_thread::sleep_for(
            std::chrono::milliseconds(1000 / config_.fps - 2));
        frame_counter_ += 1;
```

`rg "frame_counter_" whip.cpp` で確認したところ、参照は宣言（529）と代入・インクリメント（365 / 387 / 423）のみで、値を読み出す箇所がない。出力にもログにも使われていない。

## 設計方針

- `whip.cpp:529` の `frame_counter_` の宣言を削除する。
- 関連する代入・インクリメント（`whip.cpp:365` / `387` / `423`）を削除する。
- 周辺のスリープ処理などの挙動が変わらないことを確認する。

## 完了条件

- 未使用メンバ `frame_counter_` と関連する代入・インクリメントが除去される。
- `whip` の挙動が変わらないこと。
