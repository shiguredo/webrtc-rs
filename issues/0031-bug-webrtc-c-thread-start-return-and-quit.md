# webrtc_Thread_Start の戻り値破棄に対処する

- Priority: Medium
- Polished: 2026-08-12
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/change-webrtc-c-thread-start-return

## 目的

webrtc_c の `webrtc_Thread_Start` がスレッド起動の成否を呼び出し側へ伝えられるようにする。現状は `webrtc::Thread::Start()` の `bool` 戻り値を破棄しており、起動失敗を検知できない。なお Quit 相当 API の欠落は既知の課題だが、本 issue では対応しない（対象外とする）。

## 優先度根拠

スレッド起動が成功する通常時は問題が顕在化しないが、起動に失敗した場合に呼び出し側がそれを検知できず、起動していないスレッドに処理を投げて不具合に至る恐れがある。常時クラッシュする問題ではないが、起動成否の伝達はスレッド API の基本契約であり、戻り値型の変更で対応できるため Medium とする。

## 現状

`webrtc/src/webrtc_c/rtc_base/thread.cc` の `webrtc_Thread_Start` は、`p->Start()` の戻り値を受け取らず破棄している。

```cpp
WEBRTC_EXPORT void webrtc_Thread_Start(struct webrtc_Thread* self) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->Start();
}
```

`webrtc::Thread::Start()` は起動の成否を `bool` で返すが、本ラッパーは戻り値型を `void` としており (`webrtc/src/webrtc_c/rtc_base/thread.h` の `webrtc_Thread_Start` 宣言)、起動失敗を呼び出し側へ伝える手段がない。

```cpp
WEBRTC_EXPORT void webrtc_Thread_Start(struct webrtc_Thread* self);
```

また、`webrtc/src/webrtc_c/rtc_base/thread.cc` と `webrtc/src/webrtc_c/rtc_base/thread.h` に定義されている `webrtc::Thread` 系 API は、`Start` / `Stop` / `Create` / `CreateWithSocketServer` / `BlockingCall` / `BlockingCall_r` / `SleepMs` であり、メッセージループを停止させる `Quit` 相当の API が存在しない。`Quit` の C API 追加は本 issue の対象外とする。なお `webrtc_Thread_SleepMs` も同様に `webrtc::Thread::SleepMs()` の `bool` 戻り値を破棄しているが、こちらも本 issue の対象外とする。

## 設計方針

### Start 戻り値の修正

- `webrtc_Thread_Start` の戻り値型を `void` から `bool` に変更する。
  `webrtc::Thread::Start()` の戻り値 `bool` に忠実に従う（RULES.md の「元の C++ API のシグネチャ・名前に忠実に移植すること」原則）。
- 戻り値の移植には RULES.md の例外規定（「C-API 全体の一貫性を優先して out パラメータ方式で統一することがある」）が存在する。issue 0044 は `webrtc_RTCError_unique` を戻り値で返す 2 API を対象に、C-API 単体の一貫性を優先して out パラメータ方式へ統一した判断である。本関数はエラー詳細ではなく起動成否の単純な真偽値を返すものであり、例外規定は「することがある」の裁量規定であるため、RULES.md の忠実移植原則を優先して `bool` 戻り値とする（`bool* out_success` の out パラメータ方式は採用しない）。C-API 全体でも `int` を戻り値で返す API（`libyuv.h` の `libyuv_MJPGSize` 等）が多数存在し、プリミティブ値を戻り値で返す慣行がある。
- C のヘッダで `bool` を使うため、`webrtc/src/webrtc_c/rtc_base/thread.h` に `#include <stdbool.h>` を追加する（実装側 `.cc` は C++ であり `bool` は組み込み型のため不要）。
- 実装とヘッダの両方を更新する。
- 呼び出し元（`webrtc/src/whip.c` / `webrtc/src/whep.c` の C 側 6 箇所、`src/tests.rs` と
  `examples/whip/src/main.rs` / `examples/whep/src/main.rs` の Rust 側）は戻り値を無視する
  呼び出しであり、`bool` への変更後もコンパイル・警告ともに問題ない（Rust の `bool` には
  `#[must_use]` は付与されていないため）。
- Rust ラッパー `Thread::start(&mut self)` (`src/rtc_base/thread.rs`) は戻り値型を
  `bool` に変更し、呼び出し元で成否を確認できるようにする。テストでは `assert!` により
  成否を検証する（既存の戻り値無視の呼び出し元は変更不要）。
- 戻り値型の変更は後方互換のない破壊的変更のため、CHANGES.md に `[CHANGE]` として追記する。

### Quit API の扱い

`webrtc::Thread` は `Quit()` を持つが、本 issue では Start 戻り値の修正のみを行い、Quit の C API 追加は対象外とする。Quit 相当のライフサイクル制御は Start/Stop とは独立した要件であり、スコープを分割して着手を容易にする。

## テスト戦略

- Rust 側: `Thread::start()` が `bool` を返すようになった後、
  `src/tests.rs` に新規テスト関数を追加し、`Thread::new()` で生成したスレッドの
  `start()` が `true` を返すことを `assert!` で確認する
  （`bool` が `false` になる再現経路は libwebrtc 実装依存のため、正常系のみテストする）

## 完了条件

- `webrtc_Thread_Start` の戻り値型が `bool` になり、`webrtc::Thread::Start()` の戻り値を
  呼び出し側へ伝達する
- ヘッダ（`.h`）と実装（`.cc`）の戻り値型が一致している
- `webrtc/src/webrtc_c/rtc_base/thread.h` に `#include <stdbool.h>` が追加されている
- Rust ラッパー `Thread::start()` が `bool` を返すようになっている
- C 側のビルドが通ること（`python3 run.py build ubuntu-24.04_x86_64`）
- Rust 側の関連テストが通過すること（`cargo test thread_blocking_call_runs` と新規テスト）
- CHANGES.md の `## develop` セクションに `[CHANGE]` エントリが追加されている
- Quit の C API 追加と `webrtc_Thread_SleepMs` の戻り値修正は本 issue の対象外とする
