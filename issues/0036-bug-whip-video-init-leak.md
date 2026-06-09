# whip.c の video_init の早期 return によるリークを解消する

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`whip.c` の映像トランシーバ設定処理において、`SetCodecPreferences(video)` が失敗したときの早期 `return` で `video_init` を解放せずにリークしている問題を修正する。どの失敗経路でも確保済みリソースを解放してから戻るようにする。

## 優先度根拠

Low とする。リークするのは映像トランシーバの初期化オブジェクト `video_init` ひとつであり、この経路は `SetCodecPreferences(video)` が失敗した場合にのみ通る。発生頻度が低く、リーク量も小さいため、メモリ安全性に直結する他のバグより優先度は低い。ただし「壊れた窓を放置しない」という方針に従い、確実に直すべき欠陥である。

## 現状

映像トランシーバの設定は `{ ... }` ブロック内で行われ、冒頭で `video_init` を確保している (`webrtc/src/whip.c:1335-1337`)。

```c
  {
    struct webrtc_RtpTransceiverInit* video_init =
        webrtc_RtpTransceiverInit_new();
```

この `video_init` は、正常終了時にはブロック末尾で解放される (`webrtc/src/whip.c:1490`)。途中の失敗経路でも、`AddTransceiverWithTrack` 失敗時 (`webrtc/src/whip.c:1377`) や VideoTrack 生成失敗時 (`webrtc/src/whip.c:1351`) には `webrtc_RtpTransceiverInit_delete(video_init)` を呼んでから `return` している。

しかし `SetCodecPreferences(video)` の失敗を処理する早期 `return` 経路では、`video_init` を解放していない (`webrtc/src/whip.c:1483-1488`)。

```c
      if (rtc_error != NULL) {
        RTC_LOG_ERROR("Failed to SetCodecPreferences(video): error=%p",
                      rtc_error);
        webrtc_RTCError_unique_delete(rtc_error);
        return;
      }
```

この `if` ブロックの直前で `transceiver` は解放済みであり (`webrtc/src/whip.c:1481-1482`)、`rtc_error` もこの `if` 内で `webrtc_RtpTransceiverInit_delete` ではなく `webrtc_RTCError_unique_delete` により解放される (`webrtc/src/whip.c:1486`)。一方 `video_init` の解放は `if` ブロックの外、ブロック末尾の (`webrtc/src/whip.c:1490`) にしかない。そのため、この早期 `return` を通ると `video_init` が解放されないままリークする。

```c
      webrtc_RtpTransceiverInterface_Release(
          webrtc_RtpTransceiverInterface_refcounted_get(transceiver));
      if (rtc_error != NULL) {
        RTC_LOG_ERROR("Failed to SetCodecPreferences(video): error=%p",
                      rtc_error);
        webrtc_RTCError_unique_delete(rtc_error);
        return;
      }
    }
    webrtc_RtpTransceiverInit_delete(video_init);
  }
```

## 設計方針

- `SetCodecPreferences(video)` 失敗時の早期 `return` 経路でも、`return` する前に `webrtc_RtpTransceiverInit_delete(video_init)` を呼んで `video_init` を解放する。他の失敗経路 (`webrtc/src/whip.c:1351`, `webrtc/src/whip.c:1377`) と解放の仕方を揃える。
- 解放処理が複数経路に散らばっている点を踏まえ、どの経路を通っても `video_init` が確実に一度だけ解放されることを確認する。

## 完了条件

- `SetCodecPreferences(video)` が失敗する経路を含め、当該ブロックのどの失敗経路を通っても `video_init` がリークしない。
- `video_init` の二重解放が発生しない。
