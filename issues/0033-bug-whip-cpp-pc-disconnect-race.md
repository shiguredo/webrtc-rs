# whip.cpp の pc_ を Disconnect 時に保護し状態遷移の競合を解消する

- Priority: High
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`whip.cpp` の `Disconnect` における `pc_` への書き込みを mutex で保護し、状態機械 (`state_`) の遷移と `pc_` のライフサイクルを一貫させる。現状は `Disconnect` が `pc_` をロックなしで `nullptr` 代入しており、シグナリングスレッド上で進行中のコールバックが同じ `pc_` を参照する経路と競合する。これを修正する。

## 優先度根拠

High とする。`pc_` はシグナリングスレッドのコールバック (CreateOffer / SetLocalDescription /
SetRemoteDescription / レスポンス処理) から参照される一方、`Disconnect` は別スレッド (呼び出し側)
から `pc_ = nullptr` を実行し得る。`PeerConnectionInterface` の参照カウント付きポインタを保護なしで
一方が破棄し他方が参照するため、解放済みオブジェクトへのアクセスに直結する。
メモリ安全性と接続状態の整合性の双方に関わるため優先度を高くする。

## 現状

`Disconnect` は `pc_` をロックなしで `nullptr` に代入し、その後 `SetState` を呼んでいる (`webrtc/src/whip.cpp:899-903`)。

```cpp
  void Disconnect() {
    RTC_LOG(LS_INFO) << "SignalingWhip::Disconnect";
    pc_ = nullptr;
    SetState(State::kClosed);
  }
```

一方、シグナリングスレッド上で実行されるコールバック群は `pc_` を直接参照している。たとえば `pc_->SetConfiguration` (`webrtc/src/whip.cpp:837`)、`pc_->SetLocalDescription` (`webrtc/src/whip.cpp:841`)、`pc_->SetRemoteDescription` (`webrtc/src/whip.cpp:854`)、`pc_->GetSenders()` (`webrtc/src/whip.cpp:867`, `webrtc/src/whip.cpp:876`) などである。これらは `Disconnect` による `pc_ = nullptr` と同期されていない。

状態機械の側では、`SetState` は `mutex_` を取得して `state_` を更新し条件変数を通知する (`webrtc/src/whip.cpp:913-917`)。

```cpp
  void SetState(State state) {
    std::unique_lock<std::mutex> lock(mutex_);
    state_ = state;
    cv_.notify_all();
  }
```

`WaitForConnect` は `mutex_` を取得し `state_ != State::kConnecting` を待ってから `state_ == State::kConnected` を返す (`webrtc/src/whip.cpp:892-897`)。

```cpp
  bool WaitForConnect() {
    RTC_LOG(LS_INFO) << "SignalingWhip::WaitForConnected";
    std::unique_lock<std::mutex> lock(mutex_);
    cv_.wait(lock, [this]() { return state_ != State::kConnecting; });
    return state_ == State::kConnected;
  }
```

`state_` と `pc_` のメンバ宣言は (`webrtc/src/whip.cpp:1090`, `webrtc/src/whip.cpp:1092`, `webrtc/src/whip.cpp:1094`) にある。`state_` の更新と待機は `mutex_` で保護されているが、`pc_` への代入・参照はこの mutex の保護対象になっていない。そのため `Disconnect` による `pc_ = nullptr` と、コールバックでの `pc_->...` 参照が競合する。

`Disconnect` の呼び出し側は (`webrtc/src/whip.cpp:1173`) にある。

## 設計方針

- `pc_` へのアクセス (代入・参照) を `mutex_` (または `pc_` 専用の mutex) で保護する。`Disconnect` では mutex を取得したうえで、いったんローカルへ移し替えてからロック外で解放するなど、ロック中に長時間の解放処理を持ち込まない形にする。
- コールバックで `pc_->...` を呼ぶ前に保護下でローカルへコピーし、`nullptr` であればその時点で安全に処理を打ち切る。`Disconnect` 後に到達したコールバックが解放済みの `pc_` を触らないようにする。
- 状態遷移を一貫させ、`kConnecting` から抜けた後の判定が ABA を取りこぼさないようにする。`Disconnect` による `kClosed` 遷移と接続成功の `kConnected` 遷移が競合しても、待機側が正しい最終状態を観測できるようにする。

## 完了条件

- `Disconnect` と、シグナリングスレッド上のコールバックによる `pc_` 参照が並行しても、解放済み `pc_` へのアクセスが起きない。
- `state_` の遷移と `pc_` のライフサイクルが同じロックの下で一貫し、切断と状態遷移が並行しても競合しない。
- `WaitForConnect` が、間に `kClosed` を経由したケースでも誤って `kConnected` を返さない。
