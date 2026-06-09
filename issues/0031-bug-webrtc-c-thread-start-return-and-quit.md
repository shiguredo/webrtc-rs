# webrtc_Thread_Start の戻り値破棄と Quit 欠落に対処する

- Priority: Medium
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

webrtc_c の `webrtc_Thread_Start` がスレッド起動の成否を呼び出し側へ伝えられるようにし、必要なスレッドライフサイクル操作 (Quit 相当) が C API から利用できるようにする。現状 `webrtc_Thread_Start` は `webrtc::Thread::Start()` の `bool` 戻り値を破棄しており起動失敗を検知できない。また Quit 相当の API が欠落しているため、これらに対処する。

## 優先度根拠

スレッド起動が成功する通常時は問題が顕在化しないが、起動に失敗した場合に呼び出し側がそれを検知できず、起動していないスレッドに処理を投げて不具合に至る恐れがある。Quit 相当の操作が無いことでメッセージループの停止制御もできない。重大な機能欠落だが常時クラッシュする問題ではなく、API 追加・変更で対応できるため Medium とする。

## 現状

`webrtc/src/webrtc_c/rtc_base/thread.cc:24-27` の `webrtc_Thread_Start` は、`p->Start()` の戻り値を受け取らず破棄している。

```cpp
WEBRTC_EXPORT void webrtc_Thread_Start(struct webrtc_Thread* self) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->Start();
}
```

`webrtc::Thread::Start()` は起動の成否を `bool` で返すが、本ラッパーは戻り値型を `void` としており (ヘッダ宣言も `webrtc/src/webrtc_c/rtc_base/thread.h:14` で `void`)、起動失敗を呼び出し側へ伝える手段がない。

```cpp
WEBRTC_EXPORT void webrtc_Thread_Start(struct webrtc_Thread* self);
```

また、`webrtc/src/webrtc_c/rtc_base/thread.cc:17-52` および `webrtc/src/webrtc_c/rtc_base/thread.h:13-25` に定義されている `webrtc::Thread` 系 API は、`Start` / `Stop` / `Create` / `CreateWithSocketServer` / `BlockingCall` / `BlockingCall_r` / `SleepMs` であり、メッセージループを停止させる `Quit` 相当の API が存在しない。

## 設計方針

### Start 戻り値の修正

- `webrtc_Thread_Start` の戻り値型を `void` から `bool` に変更する。
  `webrtc::Thread::Start()` の戻り値 `bool` に忠実に従う（RULES.md のシグネチャ原則）。
  C で `bool` を使うため `#include <stdbool.h>` を実装側に追加する。
- 実装とヘッダの両方を更新する。
- Rust ラッパー `Thread::start(&mut self)` (`src/rtc_base/thread.rs:54`) は戻り値型を
  `bool` に変更し、呼び出し元で成否を確認できるようにする。
- 戻り値型の変更は後方互換のない破壊的変更のため、CHANGES.md に `[CHANGE]` として追記する。

### Quit API の扱い

`webrtc::Thread` は `MessageQueue` を継承しており `Quit()` を持つが、
本 issue では Start 戻り値の修正のみを行い、Quit の C API 追加は**別 issue** とする。
Quit 相当のライフサイクル制御は Start/Stop とは独立した要件であり、
スコープを分割して着手を容易にする。

## テスト戦略

- Rust 側: `Thread::start()` が `bool` を返すようになった後、
  `create()` で生成したスレッドの `start()` が `true` を返すことを確認する単体テストを追加する
  （起動失敗は通常環境では再現困難なため、正常系のみテストする）

## 完了条件

- `webrtc_Thread_Start` の戻り値型が `bool` になり、`webrtc::Thread::Start()` の戻り値を
  呼び出し側へ伝達する
- ヘッダ（`.h`）と実装（`.cc`）の戻り値型が一致している
- Rust ラッパー `Thread::start()` が `bool` を返すようになっている
- CHANGES.md の `## develop` セクションに `[CHANGE]` エントリが追加されている
- Quit 追加は別 issue 化されている
