# whip/whep のコメントアウトされた CleanupSSL を削除する

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.cpp` と `webrtc/src/whep.cpp` の `PeerConnectionFactory` デストラクタには、`webrtc::CleanupSSL()` の呼び出しがコメントとして残っている。意味を持たない残骸であるため削除する。

## 優先度根拠

ビルドや動作には影響しないコメントのみの問題であり Low とする。

## 現状

レビュー時点で実コードを確認済み。`whip.cpp:150-157` の `PeerConnectionFactory` デストラクタ末尾にコメントが残っている。

```cpp
  ~PeerConnectionFactory() {
    factory_ = nullptr;
    network_thread_->Stop();
    worker_thread_->Stop();
    signaling_thread_->Stop();

    // webrtc::CleanupSSL();
  }
```

`whep.cpp:137-144` のデストラクタにも同一のコメント (`// webrtc::CleanupSSL();`) が残っており、両ファイルで重複している。

## 設計方針

- `whip.cpp:156` および `whep.cpp:143` のコメントアウト済み `// webrtc::CleanupSSL();` を削除する。直前の空行も合わせて整理する。

## 完了条件

- 当該コメントが両ファイルから除去される。
