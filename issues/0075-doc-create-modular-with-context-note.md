# CreateModularPeerConnectionFactoryWithContext が薄いラッパーでない理由をコメントに明記する

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Polished: 2026-08-12

## 目的

`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の内部ヘルパー `CreateModularPeerConnectionFactoryWithContext`（と `PeerConnectionFactoryWithContext`）は、C API で `ConnectionContext` を返す必要があるため、libwebrtc の `webrtc::CreateModularPeerConnectionFactory` に委譲せずに factory 生成のディスパッチを再実装している（薄いラッパーではない）。しかしその理由がコードコメントに一切書かれておらず、将来のメンテナが「なぜ再実装しているのか」「なぜ signaling_thread の null を許容しないのか」を判断できない。設計判断を日本語コメントとして明記する。

## 現状

libwebrtc 本体の `webrtc::CreateModularPeerConnectionFactory`（`api/create_modular_peer_connection_factory.cc`）は factory のみを生成し、`ConnectionContext` は返さない。C API の `webrtc_CreateModularPeerConnectionFactoryWithContext` は `out_context` として `ConnectionContext` を返すため、ラッパー側で `PeerConnectionFactoryWithContext` と内部ヘルパー `CreateModularPeerConnectionFactoryWithContext` を実装して factory 生成を再現している。

この再実装には libwebrtc 本体と異なる点が 2 つある:

- signaling_thread の null ガードが無い。`webrtc_PeerConnectionFactoryDependencies_new` のデフォルト構築では `signaling_thread` は `nullptr` であり、未設定のまま呼ぶと `dependencies.signaling_thread->BlockingCall(...)` が null deref でクラッシュする。libwebrtc 本体は null を有効入力として現在のスレッド上で続行する
- `IsCurrent()` による現在スレッドとの一致チェックが無い

これらは設計上仕方がない部分であり、本 issue では挙動を修正しない。signaling_thread 未設定によるクラッシュは呼び出し側の責任（RULES.md の「それによって生じるクラッシュや未定義動作は呼び出し側の責任である」）として扱い、その契約をコメントで明記する。

なお Rust ラッパー `PeerConnectionFactory::create_modular_with_context`（`src/api/peer_connection.rs`）は signaling_thread の設定を強制しないため、設定せずに呼ぶと上記のクラッシュに到達しうる。既存の全呼び出し元（`src/tests.rs`）は `set_signaling_thread` を設定している。Rust API 側での設定強制（必須引数化など）は本 issue のスコープ外とする。

## 設計方針

本 issue の変更はコードコメントの追加のみであり、挙動は変更しない（薄いラッパー原則への回帰も行わない）。

`CreateModularPeerConnectionFactoryWithContext`（および必要なら `PeerConnectionFactoryWithContext`）に日本語コメントを追加し、以下を明記する:

- 本関数は薄いラッパーではなく、`ConnectionContext` を C API で返すために factory 生成のディスパッチを再実装している（libwebrtc 本体の `webrtc::CreateModularPeerConnectionFactory` は `ConnectionContext` を返さないため委譲できない）
- signaling_thread は必ず設定すること。未設定（null）で呼ぶと `BlockingCall` が null deref でクラッシュする
- libwebrtc 本体のガード（null 時は現在スレッド上で続行、`IsCurrent()` 一致時は直接実行）は再現しない。これはディスパッチを再実装する本関数の契約であり、呼び出し側の責任で signaling_thread を設定する

## テスト方針

本 issue の変更はコメント追加のみで挙動の変化が無いため、テストは不要である。

## 完了条件

- `CreateModularPeerConnectionFactoryWithContext`（および必要なら `PeerConnectionFactoryWithContext`）に、薄いラッパーではない理由と signaling_thread の設定契約が日本語コメントで明記されていること
- コードの挙動が変更されていないこと
- `CHANGES.md` の `## develop` に `### misc` サブセクションを新設し、[UPDATE] エントリが追記されていること
