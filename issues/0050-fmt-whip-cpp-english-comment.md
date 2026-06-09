# whip.cpp の英語コメントを日本語にする

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

AGENTS.md は「コメントは全て日本語にすること」と定めている。`webrtc/src/whip.cpp` には英語の散文コメントが残っており、この規約に反している。当該コメントを日本語に書き換え、規約に準拠させる。

## 優先度根拠

機能には影響しないコメント上の表記の問題であり、規約準拠のための軽微な修正であるため Low とする。

## 現状

`webrtc/src/whip.cpp:504` 付近に、英語の散文コメントが残っている。

```cpp
    if (adapted_width != frame.width() || adapted_height != frame.height()) {
      // Video adapter has requested a down-scale. Allocate a new buffer and
      // return scaled version.
      webrtc::scoped_refptr<webrtc::I420Buffer> i420_buffer =
          webrtc::I420Buffer::Create(adapted_width, adapted_height);
```

`// Video adapter has requested a down-scale. Allocate a new buffer and return scaled version.` という 2 行の英語コメントが、AGENTS.md の「コメントは全て日本語にすること」に反している。

## 設計方針

当該英語コメントを、内容を保ったまま日本語コメントに書き換える。AGENTS.md の「全角と半角の間には半角スペースを入れること」にも従う。コメント以外のコードは変更しない。

## 完了条件

- `webrtc/src/whip.cpp:504` 付近の英語コメントが日本語コメントに書き換えられている。
- 書き換えたコメントが AGENTS.md の表記規約（日本語、全角と半角の間の半角スペース）に準拠している。
