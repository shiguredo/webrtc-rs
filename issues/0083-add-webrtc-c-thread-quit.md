# webrtc::Thread::Quit 相当の C API を追加する

- Created: 2026-08-20
- Completed: {YYYY-MM-DD}
- Branch: feature/add-webrtc-c-thread-quit
- Polished: {YYYY-MM-DD}

## 目的

スレッドのメッセージループを停止させる Quit 相当の C API を提供する。現在 webrtc_c の `webrtc::Thread` ラッパーには `Start` / `Stop` しかなく、メッセージループを停止して後続の Post / Send を失敗させる制御手段がない。

## 現状

`webrtc/src/webrtc_c/rtc_base/thread.cc` と `webrtc/src/webrtc_c/rtc_base/thread.h` に定義されている `webrtc::Thread` 系 C API は `Start` / `Stop` / `Create` / `CreateWithSocketServer` / `BlockingCall` / `BlockingCall_r` / `SleepMs` であり、メッセージループを停止させる Quit 相当の API が存在しない。

libwebrtc の `webrtc::Thread::Quit()` (`rtc_base/thread.h`) は `void` を返す。スレッドが停止した後は Post / Send が失敗する（`webrtc::Thread` の doc コメントより）。

## 設計方針

- `webrtc_Thread_Quit(struct webrtc_Thread* self)` を `void` 返しで追加する。既存の `webrtc_Thread_Start` / `webrtc_Thread_Stop` と同じく `p->Quit();` を呼ぶだけのラッパーとする
- ヘッダ (`thread.h`) と実装 (`thread.cc`) の両方を更新する
- Rust ラッパー `Thread::quit(&mut self)` を `src/rtc_base/thread.rs` に追加する（C API は Rust ラッパー経由で公開するのが全体の慣行のため）
- `IsQuitting` / `Restart` の C API 追加は本 issue の対象外とする（Quit のみにスコープを絞る）

## テスト戦略

- `src/tests.rs` に新規テストを追加し、`Thread::new()` で生成したスレッドに対して `start()` → `quit()` → `stop()` が例外なく実行できることを確認する
- Quit 後の `blocking_call` がコールバックを実行しないことを `AtomicBool` で検証する（libwebrtc の仕様として Quit 後の Send は失敗するため）

## 完了条件

- `webrtc_Thread_Quit` がヘッダ (`thread.h`) と実装 (`thread.cc`) に追加されている
- Rust ラッパー `Thread::quit()` が `src/rtc_base/thread.rs` に追加されている
- `src/tests.rs` に Quit のテストが追加され、関連テストが通過すること
- `CHANGES.md` の `## develop` セクションに `[ADD]` エントリが追加されている
- `IsQuitting` / `Restart` の C API 追加は本 issue の対象外とする
