# whep のコメントアウトされたログ初期化を整理する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whep-remove-commented-log-init

## 目的

`webrtc/src/whep.c` と `webrtc/src/whep.cpp` の `main` 冒頭では、ログ初期化がコメントアウトされたまま放置されている。一方で `whip` 側では同等の初期化が有効になっている。`whep` 側の方針を定めて整理し、サンプル間の不整合を解消する。

## 優先度根拠

ビルドや動作には影響しないコメントのみの問題であり Low とする。ただし `whip` と `whep` でログ初期化の有無が食い違っており、サンプルとして一貫性を欠く。

## 現状

レビュー時点で実コードを確認済み。`whep.c:1333-1336` の `main` 冒頭でログ初期化がコメントアウトされている。

```c
int main() {
  //webrtc_LogMessage_LogToDebug(webrtc_LogSeverity_LS_INFO);
  //webrtc_LogMessage_LogTimestamps();
  //webrtc_LogMessage_LogThreads();
```

`whep.cpp:918-920` でも同様にコメントアウトされている。

```cpp
  //webrtc::LogMessage::LogToDebug(webrtc::LS_INFO);
  //webrtc::LogMessage::LogTimestamps();
  //webrtc::LogMessage::LogThreads();
```

一方、`whip.c:1526-1528` および `whip.cpp:1201-1203` では同等の初期化が有効化されている（コメントアウトされていない）。例として `whip.c` 側は次の通り。

```c
  webrtc_LogMessage_LogToDebug(webrtc_LogSeverity_LS_INFO);
  webrtc_LogMessage_LogTimestamps();
  webrtc_LogMessage_LogThreads();
```

## 設計方針

- `whep` 側のログ初期化を `whip` 側に合わせて有効化するか、それともコメントアウトを削除するかの方針を決める。
- 方針を決めたうえで、`whep.c:1334-1336` と `whep.cpp:918-920` のコメントアウトされたログ初期化を整理する。

## 完了条件

- コメントアウトされたログ初期化が整理される（有効化または削除）。
- `whip` と `whep` の間でログ初期化の扱いに関する方針が一貫すること。
