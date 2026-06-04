# webrtc_CreateRandomString の長さ引数を size_t にする

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-create-random-string-int-length

## 目的

C ラッパー `webrtc_CreateRandomString` は長さ引数を `int` で受け取るが、ラップ対象の
`webrtc::CreateRandomString` は `size_t` を取る。`int` で受けると負値を渡せてしまい、`size_t` へ
変換される際に符号拡張で極端に大きな値へ化け、過大なメモリ確保や未定義動作を招く。RULES.md が
求める「元の C++ API のシグネチャに忠実な移植」に合わせ、引数型を `size_t` に揃える。

## 優先度根拠

負値を `size_t` に渡すことによる過大確保・未定義動作は堅牢性の重大な欠陥である。さらに RULES.md の
移植ルール（元の C++ シグネチャに忠実であること）にも反しているため、優先度は High とする。

## 現状

C ラッパーの宣言・定義とも長さ引数の型が `int` になっている。

webrtc/src/webrtc_c/rtc_base/crypto_random.h:14

```c
WEBRTC_EXPORT struct std_string_unique* webrtc_CreateRandomString(int length);
```

webrtc/src/webrtc_c/rtc_base/crypto_random.cc:16-21

```c
extern "C" {
WEBRTC_EXPORT struct std_string_unique* webrtc_CreateRandomString(int length) {
  auto str = std::make_unique<std::string>(webrtc::CreateRandomString(length));
  return reinterpret_cast<struct std_string_unique*>(str.release());
}
}
```

ラップ対象の `webrtc::CreateRandomString`（`<rtc_base/crypto_random.h>`、crypto_random.cc:7 で
include）は `size_t` を取る。C 側が `int` で受けて `webrtc::CreateRandomString(length)` に渡すため、
負値を渡すと `int` から `size_t` への暗黙変換で巨大な値となる。

RULES.md（webrtc/RULES.md:6）は「C ラッパーは薄く保ち、基本的に元の C++ API のシグネチャ・名前に
忠実に移植すること」と定めており、`int` 受けはこのルールに反する。

## 設計方針

- crypto_random.h の宣言と crypto_random.cc の定義の双方で、長さ引数の型を `int` から `size_t` に
  変更する
- RULES.md の移植ルール（元の C++ シグネチャに忠実）に従い、`webrtc::CreateRandomString` の
  `size_t` 引数と一致させる

## 完了条件

- `webrtc_CreateRandomString` の長さ引数が `size_t` になり、負値を渡せなくなる
- 受け取った `size_t` がそのまま `webrtc::CreateRandomString` へ渡る
- 宣言（.h）と定義（.cc）の型が一致する
