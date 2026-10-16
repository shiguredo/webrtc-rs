# PeerConnectionFactoryDependencies の構築 API を new に集約する

- Created: 2026-10-16
- Completed: 2026-10-16
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

`PeerConnectionFactoryDependencies::from_params` が `PeerConnectionFactoryDependenciesParams` を受け取り、 network thread と signaling thread と ADM を構築時に確定する形にした。

`PeerConnectionFactoryDependenciesParams` は `network_thread` と `signaling_thread` を `&Thread` で、 `adm` を `Option<AudioDeviceModule>` で持つ。 `network_thread` と `signaling_thread` はフィールドに直接書くため、 省略するとコンパイルエラーになる。 `adm` は `Option` で受け取り、 `None` の場合は C API の `webrtc_PeerConnectionFactoryDependencies_set_adm` を呼ばない。

`from_params` は受け取った値を C API の setter で設定した直後に、 `adm` が設定されている場合のみ `AudioDeviceModule::runs_on` で `network_thread` との一致を検査する。 この 2 つを同じ関数で同時に受け取るため、 検査を通した後に network thread を差し替える経路が無く、 issue が挙げていた次の順序で検査を回避できない。

1. `set_network_thread(&network)` を呼ぶ
2. `set_audio_device_module(&adm)` を呼ぶ
3. `set_network_thread(&other)` を呼ぶ

`set_network_thread` / `set_signaling_thread` / `set_audio_device_module` を削除したため、 3 の差し替え自体ができず、 1 と 2 の順序も存在しない。 `new` と `Default` も削除し、 構築の入口を `from_params` に一本化した。

`signaling_thread` を必須にした理由は、 `create_modular_with_context` が C 側で `dependencies.signaling_thread->BlockingCall(...)` を無条件に呼ぶためである。 libwebrtc の `CreateModularPeerConnectionFactory` は `signaling_thread &&` で null を検査しているが、 `WithContext` 版の実装にはその検査が無く、 未指定は null 参照になっていた。 必須引数にすることでこの状態を型で塞いだ。

`network_thread` を必須にした理由は、 未指定の場合に libwebrtc が内部で network thread を生成し、 そのスレッドを Rust 側から辿れないためである。 必須にすることで内部生成経路を塞ぎ、 ADM のスレッドと食い違う依存を作れなくした。

削除した値は `set_audio_device_module` の検査で未設定の network thread を検出するためだけに使っていた `webrtc_PeerConnectionFactoryDependencies_get_network_thread` を除き、 すべて `from_params` に移動した。 `get_network_thread` は C API に残している。

`CHANGES.md` の `## develop` に `[CHANGE]` として `from_params` の追加、 setter 3 つと `new` と `Default` の削除、 検査の移動、 network thread 未設定時の panic が発生しなくなったことを記載した。 `README.md` と `skills/shiguredo-webrtc/SKILL.md` の利用例、 `examples/whip` と `examples/whep`、 `src/tests.rs` の呼び出しを `from_params` に追従させた。

## 検討したが採らなかった案

- `set_network_thread` で「ADM 設定済みなら差し替えを拒否する」検査を追加する: deps が ADM 設定済みかを Rust 側で知る手段が無く、 C 側に getter を足す必要がある。 setter の順序依存自体は残る
- `set_audio_device_module` の `# Panics` に「`set_network_thread` を後から呼ぶと検査を回避できる」と追記する: 順序依存を文書で補うだけで、 回避できる状態は残る
- `adm` を `&AudioDeviceModule` の借用で受け取る: 利用者に `clone()` を書かせる手間を省けるが、 借用のライフタイムが呼び出し側の制約になるだけで、 値渡しに対する利点が無い。 `AudioDeviceModule` の `Clone` は `ScopedRef` の参照カウントと `ThreadHandle` を複製するだけであり、 実体の再生成も ADM のスレッド契約の変更も起きない
- 引数用の型の名前に `Parts` を使う: `http` クレートの `Request::from_parts` / `Uri::from_parts` が受け取る `Parts` は分解と再構築の往復に使う中間表現であり、 外部から構造体リテラルで構築できない。 構築の入口として利用者が直接書く型に同じ名前を使うと用途が異なる
- ビルダーのメソッドチェーンにする: 必須の値が設定されているかを `build` の実行時まで検査できず、 順序依存を型で表現できない。 型状態を持つビルダーにすれば表現できるが、 3 つの値を同時に確定するだけの用途には大がかりになる
