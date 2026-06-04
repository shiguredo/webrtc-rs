# whep の RGB 値コメントに半角スペースを入れる

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whep-rgb-fullwidth-space

## 目的

AGENTS.md は「全角と半角の間には半角スペースを入れること」と定めている。`webrtc/src/whep.c` と `webrtc/src/whep.cpp` のコメントに `RGB値` という表記があり、半角の `RGB` と全角の `値` の間に半角スペースが入っていない。規約どおり `RGB 値` に修正する。

## 優先度根拠

機能には影響しないコメント上の表記の問題であり、規約準拠のための軽微な修正であるため Low とする。

## 現状

`webrtc/src/whep.c:128` 付近に、`RGB値` という表記のコメントがある。

```c
static int AnsiRenderer_RgbToAnsi256(uint8_t r, uint8_t g, uint8_t b) {
  // 216色キューブ（6x6x6）を使用
  // RGB値を0-5の範囲に変換
  int r6 = (r * 5) / 255;
```

`webrtc/src/whep.cpp:380` 付近にも、同じ `RGB値` という表記のコメントがある。

```cpp
  int RgbToAnsi256(uint8_t r, uint8_t g, uint8_t b) {
    // 216色キューブ（6x6x6）を使用
    // RGB値を0-5の範囲に変換
    int r6 = (r * 5) / 255;
```

いずれも半角の `RGB` と全角の `値` が半角スペースなしで隣接しており、AGENTS.md の「全角と半角の間には半角スペースを入れること」に反している。

## 設計方針

両ファイルの当該コメントの `RGB値` を `RGB 値` に修正する。コメント以外のコードは変更しない。

## 完了条件

- `webrtc/src/whep.c:128` 付近のコメントが `RGB 値` になっている。
- `webrtc/src/whep.cpp:380` 付近のコメントが `RGB 値` になっている。
- いずれも半角と全角の間に半角スペースを持ち、AGENTS.md の表記規約に準拠している。
