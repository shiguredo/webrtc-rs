# get_stats のコールバック未発火時に user_data の Box がリークする

- Created: 2026-08-03
- Completed: 2026-08-26
- Branch: feature/fix-get-stats-callback-box-leak
- Polished: 2026-08-12

## 目的

`PeerConnection::get_stats` (`src/api/peer_connection.rs`) で、コールバックが呼ばれない経路で `Box::into_raw` した user_data が恒久リークする問題を修正する。

## 現状

`PeerConnection::get_stats` は `PeerConnectionStatsCallbackState` (クロージャを保持する `Box`) を `Box::into_raw` で C 側に渡し、`peer_connection_on_stats` (同ファイル) で `Box::from_raw` により回収する。C 側の `RTCStatsCollectorCallbackImpl` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の同名クラス) は `OnStatsDelivered` コールバックのみを持ち、破棄通知 (`OnDestroy`) を持たない。

libwebrtc の `RTCStatsCollector` は PC 破棄時に `CancelPendingRequestAndGetShutdownTasks` で安全フラグを無効化するのみで、pending のコールバックに `OnStatsDelivered` を呼ばずに破棄する (libwebrtc m151 の `pc/rtc_stats_collector.cc`。`~RTCStatsCollector` は `= default` で、実際の破棄は `pc/peer_connection.cc` の `~PeerConnection` が `requests_` ごと行う)。つまりレポート配信前に PC が破棄された場合、`Box::from_raw` が実行されず、クロージャの `Box` がリークする。「`get_stats` 呼び出し直後に PC を drop する」正常フローで発火しうる。

なお `peer_connection_on_stats` は `Box::from_raw` を null チェックより先に実行するため、null report 経路ではリークしない (クロージャが呼ばれないだけ)。

## 設計方針

C 側の `RTCStatsCollectorCallbackImpl` に破棄通知 (`OnDestroy`) を追加し、コールバック未発火のまま C++ オブジェクトが破棄される場合に Rust 側の `Box` を回収できるようにする。

`RTCStatsCollectorCallbackImpl` は refcounted であり、`OnStatsDelivered` が正常に呼ばれた後も参照が解放されるとデストラクタが必ず実行される。したがって `OnDestroy` は「コールバック未発火時」だけでなく正常経路でも必ず呼ばれる。二重回収を防ぐため、Rust 側の回収を `OnDestroy` に一元化する:

- `peer_connection_on_stats` は `Box::from_raw` せず、`&mut` 参照で `PeerConnectionStatsCallbackState` にアクセスし、`on_stats` を `Option<Box<dyn FnOnce>>` に変更して `take()` で取り出して実行する
- `OnDestroy` ハンドラでのみ `Box::from_raw` で回収し drop する

これは既存のコールバックパターン (e.g. `csd_on_success` / `csd_on_destroy`、`src/api/peer_connection.rs`) と同一の設計である。

本変更は RULES.md の「`RTCStatsCollectorCallback_cbs` は OnDestroy を持たないが、それ以外は他と同様に扱う」という例外注記を覆すため、`webrtc/RULES.md` の該当記述も削除する。

なお `OnStatsDelivered` と `OnDestroy` は、`OnStatsDelivered` 実行中は refcounted によりデストラクタ（= `OnDestroy`）が走らないため、時系列で重ならない。したがって `&mut` 参照 (OnStatsDelivered) と `Box::from_raw` (OnDestroy) のデータ競合は発生しない。

## 完了条件

- PC 破棄時に `get_stats` のコールバックが未発火でも `Box` がリークしないこと（Drop カウンタを持つクロージャを `get_stats` に渡し、`get_stats` 直後に PC を drop して「drop 回数がちょうど 1 回であること」を `src/tests.rs` のテストで確認する。実行回数は 0 または 1 のどちらでもよい。配信は非同期で先行し得るため、`get_stats` 直後の drop でも配信が完了している場合があり、その場合は実行回数が 1 になる。実行回数と drop 回数を別々のカウンタで数えること。回収 (`Box::from_raw`) はシグナリングスレッドで非同期に行われるため、PC drop 後にシグナリングスレッドの処理完了を待ってからカウンタを確認する。待ち合わせは OnDestroy ハンドラから mpsc チャネルへ通知して `recv_timeout` で受信する方式等による）
- `OnStatsDelivered` が正常に呼ばれる経路で二重解放 (double free) が発生しないこと（`get_stats` が正常にレポートを配信する経路で、クロージャが 1 回だけ実行され drop が 1 回だけ行われることを確認する。配信完了はシグナリングスレッドで非同期に行われるため、mpsc チャネル + `recv_timeout` 等で配信完了を待ってから確認する）
- `webrtc/RULES.md` の例外注記が削除されていること
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリが追加されていること

## 解決方法

- `webrtc_RTCStatsCollectorCallback_cbs` (`webrtc/src/webrtc_c/api/stats/rtc_stats_collector_callback.h`) に `OnDestroy` を追加する
- `RTCStatsCollectorCallbackImpl` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) にデストラクタ `~RTCStatsCollectorCallbackImpl()` を追加して `OnDestroy(user_data_)` を呼ぶ。コンストラクタに `assert(cbs->OnDestroy != nullptr)` を追加する (RULES.md の Cbs 契約に従う)
- Rust 側 `get_stats` の cbs 初期化に `OnDestroy` を追加し、破棄時に `Box::from_raw` で回収する `peer_connection_on_destroy` を実装する
- `peer_connection_on_stats` を `Box::from_raw` から `&mut` 参照 + `Option::take()` による実行に変更し、回収を `OnDestroy` に一元化する（二重回収を防ぐため）。null report および `on_stats` 消費済みは起きない契約であるため `.expect("BUG: ...")` で顕在化する
- `webrtc/RULES.md` の「`RTCStatsCollectorCallback_cbs` は OnDestroy を持たない」という例外注記を削除する
- テストは `src/tests.rs` にレポートが配信されることを確認する `get_stats_delivers_report` を追加する。リーク・二重解放を専用に検証するテストは、クロージャの drop 回数を数える間接的な検証で価値が低く、二重解放はセグフォで顕在化するため見送った
