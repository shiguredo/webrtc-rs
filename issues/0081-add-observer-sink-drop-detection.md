# observer / sink を登録したまま drop した場合の実行時検出機構を追加する

- Created: 2026-08-12
- Completed: {YYYY-MM-DD}
- Branch: feature/add-observer-sink-drop-detection
- Polished: {YYYY-MM-DD}

## 目的

observer / sink を登録したまま drop すると use-after-free になる契約は `issues/0079-bug-observer-sink-lifetime-contract.md` で Rustdoc に文書化されるが、文書化は誤用を防ぐ強制力を持たない。誤用が発生しても検出されず静かに UB になるため、実行時に検出する機構を追加して誤用を開発中に気付けるようにする。

## 現状

- `DataChannelObserver` / `DtlsTransportObserver` / `VideoSink` / `AudioTrackSink` は、登録解除 (`unregister_observer` / `remove_sink`) を呼ばずに drop すると libwebrtc 内部に未解除の登録が残り、コールバック発火時に解放済みメモリへアクセスする
- 各型の `Drop` (`src/api/data_channel.rs` の `DataChannelObserver`、`src/api/dtls_transport.rs` の `DtlsTransportObserver`、`src/api/video.rs` の `VideoSink`、`src/api/audio.rs` の `AudioTrackSink`) は登録解除を呼ばないため、登録状態の追跡がないと drop 時に誤用を検出できない
- 誤用は safe Rust で発生し得る (登録 API が `&DataChannelObserver` 等の参照を受け取り、借用が登録 API の呼び出しで終了するため)

## 設計方針

検出方式は本 issue で検討して決定する。候補は以下:

- `Debug` アサーション: drop 時に登録状態を検査し、未解除の登録があれば panic する
- 登録状態のトラッキング: 各 observer / sink が登録先 (`DataChannel` / `DtlsTransport` / `VideoTrack` / `AudioTrack`) と登録済みかどうかを保持し、drop 時に検証する
- 登録先の破棄時 (`OnDestroy`) に登録済みの observer / sink の状態をクリアする

採用する方式は、既存の `# Safety` セクションや Rustdoc の記述と矛盾しないことを確認して決定する。テストは `src/tests.rs` に誤用パターン (登録したまま drop) の検出テストを追加する。

## 完了条件

- 登録したまま drop した場合に実行時検出が働くこと (検出方式とテストで確認できること)
- 正しい使い方 (登録解除後に drop) では検出が働かないこと
- 本 issue の変更で既存の公開 API のシグネチャが変わらないこと
- `src/tests.rs` の既存テストがパスすること
- `CHANGES.md` の `## develop` セクションに `### misc` サブセクションを新設し、[UPDATE] エントリが追記されていること

## 解決方法

- 採用する検出方式を決定して実装する
- 検出テストを `src/tests.rs` に追加する
- 検出方式の使い分け (`Debug` ビルドのみで有効にするか等) を `src/lib.rs` の feature 設定と整合させて実装する
