# libwebrtc のフィールドトライアルを指定できるようにする

- Created: 2026-09-19
- Completed: 2026-09-19
- Branch: feature/add-webrtc-field-trials
- Polished: {YYYY-MM-DD}

## 目的

webrtc-rs の利用側（sora-rust-sdk など）から libwebrtc のフィールドトライアル文字列を指定できるようにする。

sora-cpp-sdk では `SoraClientContextConfig::field_trials` を追加し、`webrtc::FieldTrials::Create` でパースしたフィールドトライアルを持つ `webrtc::Environment` を `webrtc::PeerConnectionFactoryDependencies::env` に設定することで、`WebRTC-Video-PerSsrcKeyframes` などのフィールドトライアルを有効化できるようにした。webrtc-rs にはこの経路がなく、フィールドトライアルを指定する手段自体が存在しない。

## 現状

### フィールドトライアルを指定する経路がない

- `webrtc/src/webrtc_c/api/environment.h` と `webrtc/src/webrtc_c/api/environment.cc` の C API は `webrtc_CreateEnvironment` と `webrtc_Environment_delete` のみで、`webrtc::FieldTrials` と `webrtc::FieldTrialsView` に対応する C API がない
- `src/api/environment.rs` の `Environment` は `new` と `as_ref` のみで、フィールドトライアルを設定・参照する手段がない
- `webrtc/src/webrtc_c/api/peer_connection_interface.h` と `webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `webrtc_PeerConnectionFactoryDependencies_*` に `env` を設定する関数がなく、`src/api/peer_connection.rs` の `PeerConnectionFactoryDependencies` にも `env` を設定する手段がない

### PeerConnectionFactoryDependencies の env が無視される

`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `PeerConnectionFactoryWithContext` が `webrtc::CreateEnvironment` を直接呼び直している。

- `PeerConnectionFactoryWithContext(webrtc::PeerConnectionFactoryDependencies)` は `webrtc::ConnectionContext::Create(webrtc::CreateEnvironment(), &dependencies)` を呼ぶ
- `PeerConnectionFactoryWithContext(webrtc::scoped_refptr<webrtc::ConnectionContext>, webrtc::PeerConnectionFactoryDependencies*)` は `webrtc::PeerConnectionFactory(webrtc::CreateEnvironment(), context, dependencies)` を呼ぶ

そのため `dependencies.env` に `webrtc::Environment` を設定しても `webrtc::ConnectionContext` と `webrtc::PeerConnectionFactory` には渡らず、生成のたびに別々の既定 `webrtc::Environment` が使われる。

libwebrtc 側は `pc/peer_connection_factory.cc` の `PeerConnectionFactory::Create` が `AssembleEnvironment(dependencies)` で `dependencies.env` を取り出し、`webrtc::ConnectionContext::Create(env, &dependencies)` と `webrtc::PeerConnectionFactory(env, context, &dependencies)` に渡している。webrtc-rs の C ラッパーだけがこの経路を使っていない。

### libwebrtc に api:field_trials が含まれていない

`Cargo.toml` の `package.metadata.external-dependencies.webrtc-build` は `m154.8037.1.1` を指している。webrtc-build は `m154.8037.1.2` で `patches/add_deps.patch` と `patches/windows_add_deps.patch` に `api:field_trials` を追加した。

`m154.8037.1.1` でビルドした libwebrtc には `webrtc::FieldTrials`（`webrtc::FieldTrialsView` ではなく具象クラス）の実装が含まれないことを `nm` で確認済み。

- webrtc-rs の prebuilt（GitHub Releases の `libwebrtc_c-*.tar.gz`）に含まれる `libwebrtc_c.a` には `webrtc::FieldTrials::Create` が定義されていない
- `--features source-build` で `m154.8037.1.1` からビルドした場合も同様に定義されていない
- `m154.8037.1.2` でビルドした libwebrtc（sora-cpp-sdk が使用しているもの）には `webrtc::FieldTrials::Create` が定義されている

そのため `webrtc::FieldTrials::Create` を呼ぶコードを追加するには webrtc-build の pin を `m154.8037.1.2` に上げる必要がある。また prebuilt 利用者（`source-build` feature を有効にしない利用者）が新しい API を使えるようにするには、pin を上げた状態で新しいバージョンをリリースして prebuilt を作り直す必要がある。

### フィールドトライアルが届く経路

libwebrtc のソースコードで以下を確認済み。

- `api/field_trials.cc` の `FieldTrials::Create` がフィールドトライアル文字列をパースする。不正な文字列の場合は nullptr を返し、空文字の場合は空の `FieldTrials` を返す
- `api/environment/environment_factory.cc` の `EnvironmentFactory::Set` が `webrtc::Environment` にフィールドトライアルを保持する。フィールドトライアルが設定されている場合は `DeprecatedGlobalFieldTrials` は使われない
- `pc/peer_connection_factory.cc` の `PeerConnectionFactory` はコンストラクタで受け取った `webrtc::Environment` を `env_` に保持し、`CreatePeerConnectionOrError` の `EnvironmentFactory env_factory(env_)` でフィールドトライアルを PeerConnection に継承する。`webrtc::PeerConnectionDependencies::trials` を設定していないため上書きされない
- PeerConnection の offer / answer 生成では `pc/sdp_offer_answer.cc` が `pc_->env()` を `WebRtcSessionDescriptionFactory` に渡し、`MediaSessionDescriptionFactory` が `transport_desc_factory->trials()` から `offer_rfc_8888_` などを決める
- `media/engine/webrtc_video_engine.cc` の `WebRtcVideoEngine` は `env.field_trials()` を保持し、`PeerConnectionFactory::GetRtpSenderCapabilities` などが参照する
- 映像 / 音声のエンコーダー・デコーダーファクトリには libwebrtc が `Create(const webrtc::Environment&, ...)` に渡す env がそのまま `webrtc_c` に渡される。webrtc-rs の `VideoEncoderFactoryHandler::create` と `VideoDecoderFactoryHandler::create` は既に `EnvironmentRef` を受け取るため、`PeerConnectionFactory` に env が届けばフィールドトライアルも届く
- `p2p/client/basic_port_allocator.cc` の `BasicPortAllocator` は `env().field_trials()` を参照するが、`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `webrtc_PeerConnectionDependencies_set_proxy` は `webrtc::CreateEnvironment()` で作った既定の `webrtc::Environment` を渡しており、フィールドトライアルは届かない

## 設計方針

### C API

C ラッパーは libwebrtc との薄い対応のみを実装するため、`webrtc::FieldTrials` / `webrtc::FieldTrialsView` / `webrtc::EnvironmentFactory` をそのまま公開する。フィールドトライアル付きの `Environment` を作る独自の複合関数（`webrtc_CreateEnvironmentWithFieldTrials` のような関数）は追加しない。

追加する型が増えて `webrtc/src/webrtc_c/api/environment.h` が大きくなるため、元の C++ ファイルのパスに合わせて以下に分割する。既存の `webrtc/src/webrtc_c/api/environment.h` と `webrtc/src/webrtc_c/api/environment.cc` は削除する（履歴を保つため `git mv` で `api/environment/environment.h` と `.cc` に移動してから分割する）。

| C ラッパー | 元の C++ | 内容 |
|---|---|---|
| `webrtc/src/webrtc_c/api/field_trials.h` / `.cc` | `api/field_trials.h` | `webrtc_FieldTrials` |
| `webrtc/src/webrtc_c/api/field_trials_view.h` / `.cc` | `api/field_trials_view.h` | `webrtc_FieldTrialsView` |
| `webrtc/src/webrtc_c/api/environment/environment.h` / `.cc` | `api/environment/environment.h` | `webrtc_Environment` |
| `webrtc/src/webrtc_c/api/environment/environment_factory.h` / `.cc` | `api/environment/environment_factory.h` | `webrtc_EnvironmentFactory` |

各ファイルに追加・移動する API は以下。

- `webrtc/src/webrtc_c/api/field_trials.h` と `.cc`
  - `webrtc_FieldTrials` を `WEBRTC_DECLARE_UNIQUE` / `WEBRTC_DEFINE_UNIQUE` で宣言する。`std::unique_ptr<webrtc::FieldTrials>` に対応する
  - `webrtc_FieldTrials_Create(const char* s, size_t s_len)` を追加する。`webrtc::FieldTrials::Create` に対応し、不正な文字列の場合は nullptr を返す
- `webrtc/src/webrtc_c/api/field_trials_view.h` と `.cc`
  - `webrtc_FieldTrialsView` を不完全型として宣言する（C 側から生成・破棄しない借用専用の型。`webrtc_NetworkManager` と同じ扱い）
  - `webrtc_FieldTrialsView_IsEnabled(const struct webrtc_FieldTrialsView* self, const char* key, size_t key_len)` を追加する。`webrtc::FieldTrialsView::IsEnabled` に対応する
- `webrtc/src/webrtc_c/api/environment/environment.h` と `.cc`
  - 既存の `webrtc_Environment` / `webrtc_CreateEnvironment` / `webrtc_Environment_delete` を移動する
  - `webrtc_Environment_field_trials(const struct webrtc_Environment* self)` を追加する。`webrtc::Environment::field_trials()` が返す `const webrtc::FieldTrialsView&` への借用ポインタを返す。C++ 側の `api/environment/environment.h` も `api/field_trials_view.h` を include しているため、`webrtc_c/api/field_trials_view.h` を include する
- `webrtc/src/webrtc_c/api/environment/environment_factory.h` と `.cc`
  - `webrtc_EnvironmentFactory`（C++ 側がスタック上に置くクラスなので `struct webrtc_EnvironmentFactory*` として扱う）と、`webrtc::EnvironmentFactory` のメソッドに対応する以下の関数を追加する
    - `webrtc_EnvironmentFactory_new()` と `webrtc_EnvironmentFactory_delete(struct webrtc_EnvironmentFactory* self)`
    - `webrtc_EnvironmentFactory_Set_field_trials(struct webrtc_EnvironmentFactory* self, struct webrtc_FieldTrials_unique* utility)`
      - `webrtc::EnvironmentFactory::Set` には所有権を移す `std::unique_ptr<const webrtc::FieldTrialsView>` を取る overload と、非所有の `const webrtc::FieldTrialsView*` を取る overload がある。非所有の overload は Rust 側で借用先の寿命を保証できないため公開せず、所有権を移す overload だけを公開する
      - C++ のシグネチャは `std::unique_ptr<const webrtc::FieldTrialsView>` だが、`webrtc_PeerConnectionFactoryDependencies_set_event_log_factory` が具象型の `webrtc_RtcEventLogFactory_unique*` を受け取って基底の `std::unique_ptr` に詰め替えているのと同じく、具象型の `webrtc_FieldTrials_unique*` を受け取る。そのため `webrtc_c/api/field_trials.h` を include する
      - `webrtc_FieldTrials_unique_get` で得たポインタから `std::unique_ptr<webrtc::FieldTrials>` を構築して `Set` にムーブする
    - `webrtc_EnvironmentFactory_Create(const struct webrtc_EnvironmentFactory* self)`
      - `webrtc::EnvironmentFactory::Create() const` に対応する。`webrtc::Environment` を値で返すため、既存の `webrtc_CreateEnvironment` と同じく `new webrtc::Environment(...)` した結果を `struct webrtc_Environment*` として返す
      - C++ 側が const メソッドなので `self` は `const struct webrtc_EnvironmentFactory*` にする
  - `webrtc_FieldTrialsView` と `webrtc_EnvironmentFactory` は今回の用途に必要なメソッドだけを公開する（`webrtc::FieldTrialsView::Lookup` / `IsDisabled`、`webrtc::EnvironmentFactory` の他の `Set` overload と `EnvironmentFactory(const webrtc::Environment&)` は追加しない）

分割に伴い以下も更新する。

- `webrtc/src/webrtc_c.h` の `#include "webrtc_c/api/environment.h"` を新しい 4 つのヘッダの include に置き換える
- `webrtc/src/webrtc_c/` 配下で `api/environment.h` を include している箇所を新パスに置き換える（`api/audio/audio_device.h` / `.cc`、`api/audio_codecs/audio_encoder_factory.h` / `.cc`、`api/audio_codecs/audio_decoder_factory.h` / `.cc`、`api/video_codecs/video_encoder_factory.h` / `.cc`、`api/video_codecs/video_decoder_factory.h` / `.cc`、`media/engine/simulcast_encoder_adapter.h` / `.cc`、`sdk/android/native_api/audio_device_module/audio_device_android.h`）
- `webrtc/CMakeLists.txt` の `add_library(webrtc_c ...)` に並んでいる `src/webrtc_c/api/environment.cc` を新しい 4 つの `.cc` に置き換える
- `webrtc/RULES.md` の「`webrtc_c/api/environment.h` は本来 `api/environment/environment.h` と `api/environment/environment_factory.h` に分かれているが、分ける意味があまり無いので纏めている」という例を削除し、実際に統合している別の例（`webrtc_c/api/rtc_event_log.h` は `api/rtc_event_log/rtc_event_log_factory.h` の型をフラットなファイルにまとめている）に差し替える

`webrtc/src/webrtc_c/api/peer_connection_interface.h` と `webrtc/src/webrtc_c/api/peer_connection_interface.cc` に以下を追加・修正する。

- `webrtc_PeerConnectionFactoryDependencies_set_env(struct webrtc_PeerConnectionFactoryDependencies* self, int has, const struct webrtc_Environment* env)` を追加する。`std::optional<webrtc::Environment>` の setter なので、他の optional setter と同じく `has` と値で受け取り、実装には `webrtc_c::OptionalSet` を使う
- `PeerConnectionFactoryWithContext` が `dependencies.env` を尊重するように修正する。`dependencies.env.value_or(webrtc::CreateEnvironment())` で `webrtc::Environment` を 1 つだけ作り、`webrtc::ConnectionContext::Create(env, &dependencies)` と `webrtc::PeerConnectionFactory(env, context, dependencies)` に同じ env を渡す
  - `CreateModularPeerConnectionFactoryWithContext` は `webrtc::ConnectionContext` も返すために薄いラッパーから逸脱している関数で、その旨は既存のコメントに記載されている。今回の修正はこの関数の中で `dependencies.env` を参照するようにするだけで、逸脱の範囲は広げない

### Rust API

`src/api/environment.rs` に以下を追加する。

- `FieldTrials`（`webrtc_FieldTrials_unique` を保持する所有型）
  - `FieldTrials::new(field_trials: &str) -> Result<Self>` を追加する。`webrtc_FieldTrials_Create` に対応し、nullptr が返った場合は `Error::InvalidFieldTrials` を返す
  - `Drop` で `webrtc_FieldTrials_unique_delete` を呼ぶ
  - 所有権を `EnvironmentFactory` に移すための `pub(crate)` な `into_raw` を用意する（`VideoEncoderFactory::into_raw` と同じ形）
- `FieldTrialsViewRef<'a>`（`ConstNonNull<webrtc_FieldTrialsView>` を保持する借用型）
  - `is_enabled(&self, key: &str) -> bool` を追加する。`webrtc_FieldTrialsView_IsEnabled` に対応する
  - `webrtc::FieldTrialsView` は不変なので `unsafe impl Send` を付ける（`EnvironmentRef` と同じ扱い）
- `Environment` のフィールドトライアルを参照する `Environment::field_trials(&self) -> FieldTrialsViewRef<'_>` と `EnvironmentRef::field_trials(&self) -> FieldTrialsViewRef<'a>` を追加する。`webrtc_Environment_field_trials` に対応する
- `EnvironmentFactory`（`webrtc_EnvironmentFactory` を保持する所有型）
  - `EnvironmentFactory::new()` / `EnvironmentFactory::set_field_trials(&mut self, field_trials: FieldTrials)` / `EnvironmentFactory::create(&self) -> Environment` を追加する。`FieldTrials` の所有権は `EnvironmentFactory` に移り、`create` で生成した `Environment` がフィールドトライアルを保持する
  - `Drop` で `webrtc_EnvironmentFactory_delete` を呼ぶ。`create` の後に factory を drop しても、フィールドトライアルは `Environment` 側のストレージで保持される
  - `webrtc_EnvironmentFactory_Create` は const メソッドなので Rust 側も `&self` にする
- `FieldTrials::new` は libwebrtc と同じく空文字をエラーにしない。空の `FieldTrials` を `EnvironmentFactory` に設定すると `DeprecatedGlobalFieldTrials` が使われなくなるため、「フィールドトライアルを指定しない」場合は `Environment::new()` を使う必要があることを doc コメントに明記する

`src/api/peer_connection.rs` の `PeerConnectionFactoryDependencies` に以下を追加する。

- `set_env(&mut self, env: Option<&Environment>)` を追加する。`webrtc_PeerConnectionFactoryDependencies_set_env` に対応し、`std::optional` の setter なので既存の `AudioEncoderFactoryOptions::set_codec_pair_id` と同じく `set_optional_object` を使う
- `env` の getter は必要になるまで追加しない

`src/error.rs` に `InvalidFieldTrials(String)` を追加する（`Display` は `invalid field trials: {フィールドトライアル文字列}`）。

### ドキュメント

- README と `skills/shiguredo-webrtc/SKILL.md` の型一覧に `FieldTrials` / `FieldTrialsViewRef` / `EnvironmentFactory` を追加する
- README と `skills/shiguredo-webrtc/SKILL.md` にフィールドトライアルを指定する例（`FieldTrials::new` → `EnvironmentFactory::set_field_trials` → `EnvironmentFactory::create` → `PeerConnectionFactoryDependencies::set_env`）を追加する。`AudioDeviceModule` に渡すのと同じ `Environment` を使う
- `skills/libwebrtc-c/SKILL.md` のディレクトリ構成とファイル一覧を新しい構成（`api/field_trials.h`、`api/field_trials_view.h`、`api/environment/environment.h`、`api/environment/environment_factory.h`）に更新し、`api/environment.h` を統合しているという記述を削除する
- `Cargo.toml`、`skills/shiguredo-webrtc/SKILL.md`、`skills/libwebrtc-c/SKILL.md`、`webrtc/RULES.md` の webrtc-build のバージョン表記を `m154.8037.1.2` にする
- `CHANGES.md` の `## develop` に [ADD]（`FieldTrials` / `FieldTrialsViewRef` / `EnvironmentFactory` / `Environment::field_trials` / `PeerConnectionFactoryDependencies::set_env` / `Error::InvalidFieldTrials` の追加）と [UPDATE]（libwebrtc を `m154.8037.1.2` に上げる）を追記する

### 対象外

- `webrtc::PeerConnectionDependencies::trials` による PeerConnection 単位のフィールドトライアル上書き
- `webrtc_PeerConnectionDependencies_set_proxy` が生成する `webrtc::BasicPortAllocator` への `webrtc::Environment` の伝搬（引数の追加を伴う API 変更になるため別 issue で扱う）

## 完了条件

- `FieldTrials::new` が正しいフィールドトライアル文字列をパースでき、不正な文字列では `Error::InvalidFieldTrials` を返すこと
- `EnvironmentFactory` と `FieldTrials` で生成した `Environment` の `field_trials().is_enabled()` が期待どおりの値を返し、`Environment::new()` では false になること
- `PeerConnectionFactory::create_modular` / `PeerConnectionFactory::create_modular_with_context` に渡した `PeerConnectionFactoryDependencies` の env が `webrtc::ConnectionContext` と `webrtc::PeerConnectionFactory` に渡ること
- フィールドトライアルが PeerConnection の offer SDP 生成まで届くことがテストで確認できること
- `Cargo.toml` の `package.metadata.external-dependencies.webrtc-build` が `m154.8037.1.2` になっていること
- `webrtc/src/webrtc_c/api/environment.h` と `.cc` が削除され、`api/field_trials.h` / `api/field_trials_view.h` / `api/environment/environment.h` / `api/environment/environment_factory.h` に分かれており、旧パスを include している箇所が残っていないこと
- `cargo test --workspace --features source-build` と `cargo clippy --workspace --features source-build --all-targets -- -D warnings` が通ること
- C/C++ の変更を含むため `python3 webrtc/run.py format --check` が通ること
- README / `skills/shiguredo-webrtc/SKILL.md` / `skills/libwebrtc-c/SKILL.md` / `webrtc/RULES.md` / `CHANGES.md` が更新されていること

prebuilt 利用者（`source-build` feature を有効にしない利用者）が新しい API を使えるようにするには、この変更を含むバージョンをリリースして `libwebrtc_c-*.tar.gz` を作り直す必要がある。リリース作業自体はリリース手順で行う。

## テスト

`src/tests.rs` に以下を追加する。

- `field_trials_create_and_environment_field_trials`
  - `FieldTrials::new("WebRTC-Video-PerSsrcKeyframes/Enabled/")` が成功すること
  - 末尾の `/` がない不正な文字列（`WebRTC-Video-PerSsrcKeyframes/Enabled`）で `Error::InvalidFieldTrials` が返ること
  - `EnvironmentFactory::new()` に `FieldTrials::new("WebRTC-Video-PerSsrcKeyframes/Enabled/")` を設定して `create()` で生成した `Environment` の `field_trials().is_enabled("WebRTC-Video-PerSsrcKeyframes")` が true になること
  - `Environment::new()` の `field_trials().is_enabled("WebRTC-Video-PerSsrcKeyframes")` が false になること
- `field_trials_reach_offer_sdp`
  - 既存の `always_negotiate_data_channels_adds_data_section` と同じ構成（`Thread` を 2 本、builtin の音声エンコーダー / デコーダーファクトリ、Dummy の `AudioDeviceModule`、`enable_media()`）にする。`WebRtcVoiceEngine` は生成時に音声コーデックのファクトリを参照するため必須であり、ファクトリが空だと m= セクションにコーデックが並ばず `a=rtcp-fb` も出力されない
  - `FieldTrials::new("WebRTC-RFC8888CongestionControlFeedback/Enabled,offer:true/")` を `EnvironmentFactory` に設定して `create()` で生成した `Environment` を `deps.set_env(Some(&env))` で設定し、`PeerConnection::add_transceiver(MediaType::Audio, ...)` の後に `create_offer` で作った SDP に `ack ccfb` が含まれることを確認する
  - 対照実験として `Environment::new()` を設定した場合は `ack ccfb` が含まれないことを確認する
  - `MediaSessionDescriptionFactory` が `transport_desc_factory->trials()` から `offer_rfc_8888_` を決めるため、フィールドトライアルが `PeerConnectionFactory` から PeerConnection まで届いていなければこのテストは失敗する。C ラッパーが env を無視する不具合の回帰テストにもなる

`WebRTC-Video-PerSsrcKeyframes` が `VideoSendStreamImpl` まで届くことの確認は実際の配信が必要なため、E2E テストまたは手動で別途行う。

## 解決方法

libwebrtc のフィールドトライアルを指定できるようにした。

- `webrtc_c` の `api/environment.h` / `.cc` を元の C++ のパスに合わせて `api/environment/environment.h` / `.cc` と `api/environment/environment_factory.h` / `.cc` に移動し、`api/field_trials.h` / `.cc` と `api/field_trials_view.h` / `.cc` を追加した。`webrtc_c.h`、`CMakeLists.txt`、旧パスを include していた箇所、`webrtc/RULES.md` の統合例も追随させた
- `webrtc_c` に `webrtc_FieldTrials_Create` / `webrtc_FieldTrialsView_IsEnabled` / `webrtc_Environment_field_trials` / `webrtc_Environment_copy` / `webrtc_EnvironmentFactory_new` / `webrtc_EnvironmentFactory_Set_field_trials` / `webrtc_EnvironmentFactory_Create` を追加した。`webrtc::EnvironmentFactory` をそのまま公開し、フィールドトライアル付きの `Environment` を作る独自の複合関数は追加していない
- `webrtc_c` に `webrtc_PeerConnectionFactoryDependencies_set_env` を追加し、`PeerConnectionFactoryWithContext` が `dependencies.env` を尊重して `ConnectionContext` と `PeerConnectionFactory` に同じ `webrtc::Environment` を渡すようにした
- Rust API に `FieldTrials` / `FieldTrialsViewRef` / `EnvironmentFactory` と `Environment::field_trials` / `EnvironmentRef::field_trials` を追加し、`Environment` を `Clone` に対応させた。`PeerConnectionFactoryDependencies::set_env` と `Error::InvalidFieldTrials` も追加した
- `Cargo.toml` の webrtc-build を `m154.8037.1.2` に上げた。`api:field_trials` が含まれ、`webrtc::FieldTrials` をリンクできるようになる
- README とスキルにフィールドトライアルの使い方を追記し、`CHANGES.md` の `## develop` に [ADD] と [UPDATE] を追記した

確認:

- `cargo test --workspace --features source-build` が成功することを確認した (139 件 + 35 件 + doctest 9 件)
- `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo fmt --all -- --check` / `python3 webrtc/run.py format --check` / `cargo doc --no-deps --features source-build` が成功することを確認した
- フィールドトライアルが PeerConnection の offer SDP 生成まで届くことを `WebRTC-RFC8888CongestionControlFeedback/Enabled,offer:true/` を使ったテストで確認した
- `WebRTC-Video-PerSsrcKeyframes` が `VideoSendStreamImpl` まで届くことの実配信での確認は未実施
