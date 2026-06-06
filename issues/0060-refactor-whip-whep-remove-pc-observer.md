# 未使用の pc_observer フィールドを削除する

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.cpp` の `struct SignalingWhipConfig` と `webrtc/src/whep.cpp` の `struct SignalingWhepConfig` には、宣言のみで代入も読み出しもされない `pc_observer` フィールドが存在する。デッドコードを削除して構造体を整理する。

## 優先度根拠

ビルドや動作には影響しないデッドコードであり Low とする。ただし未使用メンバは構造体の意図を曖昧にするため整理する価値はある。

## 現状

レビュー時点で実コードと `rg` による参照状況を確認済み。`whip.cpp:532-540` の `struct SignalingWhipConfig` に `pc_observer` が含まれている。

```cpp
struct SignalingWhipConfig {
  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> pc_factory;
  webrtc::PeerConnectionObserver* pc_observer;

  std::string signaling_url;
  std::string channel_id;
  std::optional<std::vector<webrtc::RtpEncodingParameters>> send_encodings;
  webrtc::scoped_refptr<webrtc::VideoTrackSourceInterface> video_source;
};
```

`whep.cpp:394-400` の `struct SignalingWhepConfig` にも同じ `webrtc::PeerConnectionObserver* pc_observer;` が含まれている。

`rg "pc_observer" whip.cpp` / `rg "pc_observer" whep.cpp` で確認したところ、いずれもこの宣言（`whip.cpp:534`、`whep.cpp:396`）のみで、代入も読み出しもない（いずれも参照ゼロを確認）。

## 設計方針

- `whip.cpp:534` および `whep.cpp:396` の `webrtc::PeerConnectionObserver* pc_observer;` を削除する。
- 他のメンバには手を加えない。

## 完了条件

- 両ファイルから `pc_observer` が除去される。
- `whip` と `whep` の挙動が変わらないこと。
