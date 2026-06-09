# whep の受信トラック操作を mutex で保護する

- Priority: High
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

whep の受信トラック (`video_track` / `video_track_`) の読み書きをロックで保護し、シグナリングスレッドからのコールバックとレンダリング処理の間で発生するデータ競合を解消する。現状はロックなしで `video_track` を読み書きしているため、未定義動作やクラッシュを招く可能性がある。これを修正する。

## 優先度根拠

High とする。`OnTrack` / `OnRemoveTrack` はシグナリングスレッドから呼ばれるコールバックであり、レンダリング側 (`DetachVideoSink` やフレーム描画) と同じ `video_track` を参照する。複数スレッドから同一ポインタを保護なしで読み書きしているため、解放済みポインタへのアクセスや二重解放によるクラッシュという、メモリ安全性に直結する不具合である。サンプルとはいえ実行時にクラッシュし得るため、優先度を高く設定する。

## 現状

### C 実装 (`webrtc/src/whep.c`)

`SignalingWhep` 構造体には `mutex` メンバが存在する (`webrtc/src/whep.c:501`)。

```c
struct SignalingWhep {
  ...
  struct webrtc_VideoTrackInterface_refcounted* video_track;
  pthread_mutex_t mutex;
  pthread_cond_t cond;
  ...
};
```

しかし `SignalingWhep_OnTrack` は `self->video_track` をロックなしで読み (`webrtc/src/whep.c:640-646`)、ロックなしで書き換えている (`webrtc/src/whep.c:659`)。

```c
  if (self->video_track != NULL &&
      webrtc_VideoTrackInterface_refcounted_get(self->video_track) ==
          webrtc_VideoTrackInterface_refcounted_get(video_track)) {
    webrtc_VideoTrackInterface_Release(
        webrtc_VideoTrackInterface_refcounted_get(video_track));
    return;
  }

  SignalingWhep_DetachVideoSink(self);
  ...
  self->video_track = video_track;
```

`SignalingWhep_OnRemoveTrack` も同様に `self->video_track` をロックなしで読み、条件次第で `SignalingWhep_DetachVideoSink` を呼ぶ (`webrtc/src/whep.c:698-705`)。

```c
  if (self->video_track != NULL &&
      webrtc_VideoTrackInterface_refcounted_get(self->video_track) ==
          webrtc_VideoTrackInterface_refcounted_get(video_track)) {
    SignalingWhep_DetachVideoSink(self);
  }
```

`SignalingWhep_DetachVideoSink` 自体もロックを取得せずに `self->video_track` を読み書きして解放している (`webrtc/src/whep.c:572-584`)。

加えて `receiver` の扱いについては、`OnTrack` では `receiver == NULL` を確認しているが (`webrtc/src/whep.c:599`)、`OnRemoveTrack` では引数 `receiver` が `NULL` でないことだけを冒頭で確認し (`webrtc/src/whep.c:666`)、その後の `webrtc_RtpReceiverInterface_track` の結果のみを検査している (`webrtc/src/whep.c:674`)。

### C++ 実装 (`webrtc/src/whep.cpp`)

C++ 側には `video_mutex_` メンバが宣言されているが (`webrtc/src/whep.cpp:860`)、`OnTrack` / `OnRemoveTrack` では使用されていない。`OnTrack` は `video_track_` をロックなしで読み書きし (`webrtc/src/whep.cpp:834-838`)、`OnRemoveTrack` は `DetachVideoSink` を呼ぶだけである (`webrtc/src/whep.cpp:851`)。`DetachVideoSink` (whep.cpp:649-655) もロックなしで `video_track_` にアクセスしている。

```cpp
    auto* video_track = static_cast<webrtc::VideoTrackInterface*>(track.get());
    if (video_track_ && video_track_.get() == video_track) {
      return;
    }
    DetachVideoSink();
    video_track_ = video_track;
    webrtc::VideoSinkWants wants;
    video_track_->AddOrUpdateSink(video_sink_.get(), wants);
```

`DetachVideoSink` は `mutex_` をロックするが (`webrtc/src/whep.cpp:644`)、`video_track_` を保護する目的で宣言された `video_mutex_` は使われていないため、`OnTrack` / `OnRemoveTrack` 経路の `video_track_` アクセスは保護されていない。

## 設計方針

- C 実装では `video_track` にアクセスする全経路 (`SignalingWhep_OnTrack` / `SignalingWhep_OnRemoveTrack` / `SignalingWhep_DetachVideoSink`) で同一の mutex を取得し、読み書きを排他制御する。ロック範囲は最小化しつつ、`video_track` の比較・差し替え・解放が原子的に行われるようにする。
- C++ 実装では `OnTrack` / `OnRemoveTrack` / `DetachVideoSink` で `video_track_` を保護する mutex (既存の `video_mutex_` を活用するか、保護対象を整理する) を一貫して取得する。`DetachVideoSink` が取るロックと、トラック差し替え時のロックが同一であることを保証し、デッドロックが起きない順序で取得する。
- あわせて `receiver` の `NULL` 判定を `OnTrack` / `OnRemoveTrack` の両方で揃え、`receiver` が `NULL` の場合に早期 return する経路を明確にする。

## 完了条件

- `video_track` / `video_track_` への読み書きが、シグナリングスレッドのコールバックとレンダリング処理の間で必ず排他制御される。
- 受信トラックの差し替え・削除が並行して発生しても、解放済みポインタへのアクセスや二重解放が起きない。
- `receiver` が `NULL` の場合でも安全に早期 return する。
