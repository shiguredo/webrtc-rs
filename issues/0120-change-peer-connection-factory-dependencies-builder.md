# PeerConnectionFactoryDependencies の構築 API を new に集約する

- Created: 2026-10-16
- Completed: {YYYY-MM-DD}
- Branch: feature/change-peer-connection-factory-dependencies-builder
- Polished: {YYYY-MM-DD}

## 目的

`PeerConnectionFactoryDependencies` の setter が設定順に依存する状態を無くす。

現在 `set_audio_device_module` は、設定済みの network thread と ADM がメソッドを実行するスレッドの一致を検査して panic する。 ところが `set_network_thread` は検査を行わずに無条件で上書きするため、 `set_audio_device_module` の後に `set_network_thread` を呼ぶと一致検査を回避できる。 値の設定順で検査の有効性が変わる状態を解消する。

## 現状

`PeerConnectionFactoryDependencies` の値は `new` で空の依存を作った後に setter で設定する。 `set_network_thread` は C API の `webrtc_PeerConnectionFactoryDependencies_set_network_thread` を呼ぶだけで、 `deps->network_thread` を無検査で差し替える (`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `webrtc_PeerConnectionFactoryDependencies_set_network_thread`)。

`set_audio_device_module` は `webrtc_PeerConnectionFactoryDependencies_get_network_thread` で設定済みの network thread を読み、 `AudioDeviceModule::runs_on` で一致を検査してから `webrtc_PeerConnectionFactoryDependencies_set_adm` を呼ぶ。 検査はこの関数の呼び出し時点で一度だけ行われるため、 次の順序で検査を通過した後に食い違った依存を作れる。

1. `set_network_thread(&network)` を呼ぶ
2. `set_audio_device_module(&adm)` を呼ぶ (ここで一致を検査して通過する)
3. `set_network_thread(&other)` を呼ぶ (検査されずに差し替わる)

この順序では ADM のスレッドと実際の network thread が食い違ったまま `PeerConnectionFactory::create_modular` を呼べる。 libwebrtc は worker thread (= network thread) から ADM を利用するため (`refs/webrtc/src/media/engine/webrtc_voice_engine.cc` の `WebRtcVoiceEngine::adm` が `RTC_DCHECK_RUN_ON(&worker_thread_checker_)` を取る)、 デバッグビルドでは `RTC_DCHECK` に掛かり、 リリースビルドではデータ競合になる。

また `set_network_thread` の Rustdoc には、 `set_audio_device_module` を呼ぶ場合に先に設定する必要があることが書かれていない。 順序の契約が setter の片側にしか無い。

## 設計方針

必須の値を `PeerConnectionFactoryDependencies::new` の引数にし、 setter で後から差し替えられないようにする。 C++ の `webrtc::PeerConnectionFactoryDependencies` は `network_thread` と `adm` が同じ struct の兄弟フィールドであり (`refs/webrtc/src/api/peer_connection_interface.h`)、 C++ 側には設定順の制約が無い。 Rust 側の setter チェーンが順序依存を作っているため、 構築時に値を渡す形が C++ の構造に忠実になる。

`PeerConnectionFactoryDependencies::new` のシグネチャが変わるため、 `new` の呼び出しと `set_network_thread` の呼び出しは利用者側の修正が必要になる。 `CHANGES.md` には `[CHANGE]` として記載する。

検討が必要な点:

- `network_thread` を `new` の必須引数に昇格させ、 `set_network_thread` を削除する。 削除しない場合、 差し替えを拒否するか、 `set_audio_device_module` の検査を `set_network_thread` でも再実行する必要がある
- `worker_thread` は削除済みのため、 factory が使う thread は network thread に一本化されている (`refs/webrtc/src/pc/connection_context.cc` が `dependencies->worker_thread != nullptr` のとき worker thread、 そうでなければ network thread を worker thread として使う)
- `adm` は `enable_media` を呼ぶ場合にのみ必要なため、 `new` の必須引数にすると ADM を使わない利用者にも値を求めることになる。 `Option` で受けるか、 ADM だけ setter に残すかを決める
- `network_thread` を指定しない場合、 libwebrtc は `refs/webrtc/src/pc/connection_context.cc` の `MaybeStartNetworkThread` で内部の network thread を生成する。 この経路を許すか、 `new` の必須引数にして塞ぐかを決める
- `set_*` が 20 個以上あるため、 どの値を構築時に受けるかの線引きを決める

## 完了条件

- `set_network_thread` を `set_audio_device_module` の後に呼んでも、 ADM のスレッドと network thread が食い違った依存を作れないこと
- 順序の契約が型または単一の関数で表現され、 setter の呼び出し順に依存しないこと
- 公開 API を変更する場合は `CHANGES.md` の `## develop` に反映されていること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること

## 解決方法

未確定。 closed にするときに、 実際に何をどう修正したかを書く。

## 検討したが採らなかった案

- `set_network_thread` で「ADM 設定済みなら差し替えを拒否する」検査を追加する: deps が ADM 設定済みかを Rust 側で知る手段が無く、 C 側に getter を足す必要がある。 setter の順序依存自体は残る
- `set_audio_device_module` の `# Panics` に「`set_network_thread` を後から呼ぶと検査を回避できる」と追記する: 順序依存を文書で補うだけで、 回避できる状態は残る
