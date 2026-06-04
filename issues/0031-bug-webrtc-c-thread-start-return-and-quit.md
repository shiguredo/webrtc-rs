# webrtc_Thread_Start の戻り値破棄と Quit 欠落に対処する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-thread-start-return-and-quit

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

- `webrtc_Thread_Start` の戻り値を、元の C++ `webrtc::Thread::Start()` の `bool` に合わせて呼び出し側へ伝える。RULES.md の「元の C++ API のシグネチャに忠実に移植する」原則 (`webrtc/RULES.md:5-6`、`webrtc/RULES.md:12`) に従い、ラッパーの戻り値型を C で扱える型 (例えば `int` あるいは `bool`) に変更する。実装 (`webrtc/src/webrtc_c/rtc_base/thread.cc`) とヘッダ (`webrtc/src/webrtc_c/rtc_base/thread.h`) の双方を更新する。戻り値型変更に伴う既存呼び出し側 (存在する場合) の追従も行う。
- Quit 相当の C API 追加を検討する。元の `webrtc::Thread` に対応するメンバ関数を確認し、RULES.md の命名規則 (`webrtc/RULES.md:12`、`webrtc_Xxx_Yyy` 形式) に従って `webrtc_Thread_Quit` などを追加するかを判断する。薄いラッパー原則に照らし、実際に必要なライフサイクル操作のみを最小限で追加すること。追加が不要と判断する場合はその理由を明記する。

## 完了条件

- `webrtc_Thread_Start` がスレッド起動の成否 (`webrtc::Thread::Start()` の `bool` 戻り値) を呼び出し側へ返せるようになり、起動失敗を検知できる。
- 実装とヘッダの戻り値型が一致し、RULES.md の命名・シグネチャ原則に沿っている。
- Quit 相当の C API について、追加するか見送るかが判断され、追加する場合は RULES.md の命名規則に沿った形で実装されている (見送る場合は理由が明記されている)。
