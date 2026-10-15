# whip/whep のコメントアウトされた CleanupSSL を削除する

- Priority: Low
- Completed: 2026-10-15
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## pending の理由

whip/whep 関連の対応を保留するため

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

## 解決方法

C/C++ の WHIP / WHEP サンプル (`webrtc/src/whip.c` / `whep.c` / `whip.cpp` / `whep.cpp`) と、対応する CMake ターゲット (`whip_c` / `whep_c` / `whip_cpp` / `whep_cpp`) を削除した。4 ファイルはどのサポートプラットフォームでもビルドできず、C API / C++ API の利用例として機能していなかった。本 issue が対象としていたコードはこの削除でリポジトリから無くなり、修正の要否を検討する対象自体が存在しなくなるため closed にする。C API 本体 (`webrtc/src/webrtc_c/`) と `webrtc_c.h` はそのまま残る。利用例は Rust の `examples/whip` / `examples/whep` を参照する。

コメントアウトされた `CleanupSSL` は削除されたファイル内のコメントである。
