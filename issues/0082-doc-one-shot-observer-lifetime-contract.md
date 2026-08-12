# 1 回限り observer のライフタイム契約を文書化する

- Created: 2026-08-12
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-one-shot-observer-lifetime-contract
- Polished: {YYYY-MM-DD}

## 目的

1 回限り observer (`CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver`) のライフタイム契約が API に文書化されておらず、ユーザーが observer を drop してよいタイミングを判断できない状態を解消する。参照カウント管理により UAF にはならないが、コールバックが発火するまで C++ 側が参照を保持する契約を明記する。

## 現状

- `src/api/peer_connection.rs` の `CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver` は `make_ref_counted` で生成され、Rust ラッパーを先に drop しても実体は C++ 側の参照で生存する (コールバックは必ず発火する)
- コールバックの発火タイミングと、observer を drop してよいタイミングが doc に記載されていない
- 登録解除 API が存在しないため、`issues/0079-bug-observer-sink-lifetime-contract.md` の対象外となっている

## 設計方針

- `CreateSessionDescriptionObserver` は `create_offer` / `create_answer` に渡され、コールバック (`on_success` / `on_failure`) が発火した時点で C++ 側の参照が解放される
- `SetLocalDescriptionObserver` は `set_local_description` に渡され、`on_set_local_description_complete` が発火した時点で C++ 側の参照が解放される
- `SetRemoteDescriptionObserver` は `set_remote_description` に渡され、`on_set_remote_description_complete` が発火した時点で C++ 側の参照が解放される
- これらの契約を各 API (`create_offer` / `create_answer` / `set_local_description` / `set_remote_description`) の Rustdoc に日本語で記載する

## 完了条件

- 以下の API すべてに observer のライフタイム契約が Rustdoc で明記されていること:
  - `PeerConnection::create_offer` (`src/api/peer_connection.rs`): `CreateSessionDescriptionObserver` はコールバック発火後に drop してよいこと
  - `PeerConnection::create_answer` (`src/api/peer_connection.rs`): 同上
  - `PeerConnection::set_local_description` (`src/api/peer_connection.rs`): `SetLocalDescriptionObserver` はコールバック発火後に drop してよいこと
  - `PeerConnection::set_remote_description` (`src/api/peer_connection.rs`): `SetRemoteDescriptionObserver` はコールバック発火後に drop してよいこと
- 各 observer 型の doc にも同旨が記載されていること
- 本 issue の変更は doc 追記のみで挙動の変化が無いため、テストは不要である
- `CHANGES.md` の `## develop` セクションに `### misc` サブセクションを新設し、[UPDATE] エントリが追記されていること

## 解決方法

- 各 API の Rustdoc に「コールバック発火後に drop してよい」旨を日本語で記載する (既存の `# Safety` セクション形式には合わせず、`# ライフタイム` セクションを新設する)
- 参照カウントにより drop しても安全である旨 (コールバックは必ず発火する) も併記する
