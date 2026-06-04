# whip/whep のマジックナンバーを定数化する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-whep-magic-numbers

## 目的

`webrtc/src/whip.c`・`webrtc/src/whep.c`・`webrtc/src/whip.cpp`・`webrtc/src/whep.cpp` の WHIP / WHEP サンプル実装では、バッファサイズ・待機時間などのマジックナンバーが各所に直書きされている。値の意味がコード上から読み取りにくく、複数箇所に同じ値が散在しているため変更時に追従漏れが起きやすい。意味のある名前付き定数に整理し、可読性と変更容易性を高める。

## 優先度根拠

いずれもサンプルコードであり、現状で動作上の不具合は確認されていない。可読性と保守性の改善が主目的で利用者への影響もないため Low とする。

## 現状

各ファイルにバッファサイズや待機時間などのマジックナンバーが直書きされている。代表的な箇所を以下に示す。

`webrtc/src/whep.c:1019` 付近では、受信バッファの初期容量と読み出しバッファのサイズに `4096` を直書きしている。

```c
  size_t resp_cap = 4096;
  size_t resp_len = 0;
  char* resp = (char*)malloc(resp_cap);
```

```c
  char buf[4096];
```

`webrtc/src/whep.c:1352` 付近では、待機時間としてマイクロ秒のリテラルを直書きしている。

```c
  usleep(30000000);
```

`webrtc/src/whip.cpp:1037` 付近でも同様に `4096` が重複している。

```cpp
    resp.reserve(4096);
    char buf[4096];
```

`webrtc/src/whep.cpp:775` 付近も同じく `4096` の直書きである。

```cpp
    resp.reserve(4096);
    char buf[4096];
```

このほか、`webrtc/src/whip.c:901` の `char content_length[32]`、`webrtc/src/whep.c:104` の `char tmp[32]`、各ファイルのクエリ文字列 `?video_bit_rate=6000`（`webrtc/src/whip.c:898`・`webrtc/src/whep.cpp:482`・`webrtc/src/whip.cpp:742`）など、意味を持つ数値・文字列リテラルが複数箇所に散在している。

## 設計方針

バッファサイズ・待機時間などの主要なマジックナンバーを、意味の分かる名前付き定数として定義し、各使用箇所から参照する形に整理する。各ファイルのスタイル（C 側は `#define` ないし `static const`、C++ 側は `constexpr` 等）に合わせて自然な形で定義する。サンプルの動作は変えず、値そのものは現状を維持する。

## 完了条件

- バッファサイズや待機時間などの主要なマジックナンバーが名前付き定数として定義され、使用箇所から参照されている。
- 定数化により値の意味がコードから読み取れるようになっている。
- サンプルの動作と各値が現状と等価である。
