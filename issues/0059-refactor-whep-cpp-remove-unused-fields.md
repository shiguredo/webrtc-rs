# whep.cpp の未使用フィールドを削除する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whep-cpp-remove-unused-fields

## 目的

`webrtc/src/whep.cpp` の `class SignalingWhep` には、宣言のみで読み書きされていないメンバ変数が 3 つ存在する。デッドコードを削除して構造体を整理する。

## 優先度根拠

ビルドや動作には影響しないデッドコードであり Low とする。ただし未使用メンバはクラスの意図を曖昧にするため整理する価値はある。

## 現状

レビュー時点で実コードと `rg` による参照状況を確認済み。`whep.cpp:854-862` の `class SignalingWhep` のメンバ宣言に、未使用の 3 フィールドが含まれている。

```cpp
 private:
  SignalingWhepConfig config_;

  webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc_;
  std::unique_ptr<AnsiRenderer> video_sink_;
  webrtc::scoped_refptr<webrtc::VideoTrackInterface> video_track_;
  std::mutex video_mutex_;
  std::optional<webrtc::VideoFrame> last_video_frame_;
  bool logged_first_frame_ = false;
```

`rg "video_mutex_" whep.cpp` / `rg "last_video_frame_" whep.cpp` / `rg "logged_first_frame_" whep.cpp` で確認したところ、3 フィールドはいずれも宣言（`whep.cpp:860-862`）のみで、読み書きの参照がない（いずれも参照ゼロを確認）。

## 設計方針

- `whep.cpp:860-862` の `video_mutex_` / `last_video_frame_` / `logged_first_frame_` の宣言を削除する。
- 他のメンバ（`config_` / `pc_` / `video_sink_` / `video_track_` / `mutex_` / `cv_` / `state_`）には手を加えない。

## 完了条件

- 未使用フィールド `video_mutex_` / `last_video_frame_` / `logged_first_frame_` が除去される。
- `whep` の挙動が変わらないこと。
