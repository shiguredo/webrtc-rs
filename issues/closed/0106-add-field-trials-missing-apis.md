# FieldTrialsViewRef と EnvironmentRef の不足 API を追加する

- Created: 2026-09-19
- Completed: 2026-09-19
- Branch: feature/add-field-trials-is-disabled-and-environment-to-owned
- Polished: {YYYY-MM-DD}

## 目的

0105 で追加したフィールドトライアル API に以下の 3 つが無く、利用側で実用にならない。

- libwebrtc の `webrtc::FieldTrialsView::IsDisabled` に対応する API が無い。`IsEnabled` の否定では代用できない
- libwebrtc の `webrtc::FieldTrialsView::Lookup` に対応する API が無く、`Enabled,offer:true` のようなパラメータを含む値を読み取れない
- `EnvironmentRef` から所有権を持つ `Environment` を作る方法が無く、`EnvironmentRef` の借用が切れると `Environment` を保持し続けられない

## 現状

### is_disabled が無い

- `webrtc/src/webrtc_c/api/field_trials_view.h` と `.cc` は `webrtc_FieldTrialsView_IsEnabled` だけを公開しており、`webrtc::FieldTrialsView::IsDisabled` に対応する C API が無い
- `src/api/environment.rs` の `FieldTrialsViewRef` は `is_enabled` だけを持ち、無効判定を行う手段が無い

libwebrtc の `api/field_trials_view.h` の `FieldTrialsView` は `Lookup` の結果が `"Enabled"` で始まる場合だけ `IsEnabled` が true、`"Disabled"` で始まる場合だけ `IsDisabled` が true になる。値が `"Enabled"` でも `"Disabled"` でもないキーと、そもそも設定されていないキーは両方 false になる。

そのため `is_enabled` の否定を `is_disabled` の代用にすると、設定されていないキーを「無効」と誤判定する。フィールドトライアルを指定していない `Environment::new()` では全てのキーがこれに当たる。

### lookup が無い

- `webrtc/src/webrtc_c/api/field_trials_view.h` と `.cc` は `webrtc::FieldTrialsView::Lookup` に対応する C API を持たない
- `src/api/environment.rs` の `FieldTrialsViewRef` は `is_enabled` だけを持ち、`Enabled,offer:true` の `offer:true` のようなパラメータを読み取れない
- libwebrtc の `api/field_trials_registry.h` の `FieldTrialsRegistry::Lookup` は `GetValue` の結果をそのまま返し、設定されていないキーでは空文字列を返す
- libwebrtc の `api/field_trials.cc` の `Parse` は値が空のフィールドトライアルを不正として弾くため、空文字列は「設定されていない」ことだけを意味する

### EnvironmentRef::to_owned が無い

- `src/api/environment.rs` の `EnvironmentRef` は `ConstNonNull<ffi::webrtc_Environment>` を保持する借用型で、`Environment` を作る手段が `Environment::clone` に限られる
- `EnvironmentRef` を引数で受け取るハンドラ (例: `VideoEncoderFactoryHandler::create`) は、受け取った `Environment` を借用の外へ持ち出せない
- `webrtc/src/webrtc_c/api/environment/environment.h` には `webrtc::Environment` のコピーを作る `webrtc_Environment_copy` があるが、`EnvironmentRef` からは呼べない

## 設計方針

### C API

- `webrtc/src/webrtc_c/api/field_trials_view.h` と `.cc` に `webrtc_FieldTrialsView_IsDisabled` を追加する。`webrtc::FieldTrialsView::IsDisabled` に対応し、`webrtc_FieldTrialsView_IsEnabled` と同じ引数・戻り値にする
- `webrtc/src/webrtc_c/api/field_trials_view.h` と `.cc` に `webrtc_FieldTrialsView_Lookup` を追加する
  - `webrtc::FieldTrialsView::Lookup` は `std::string` を値で返すため、`webrtc_TransformableFrameInterface_GetMimeType` と同じくヒープ確保したコピーを `struct std_string_unique*` として返す。破棄は呼び出し側が `std_string_unique_delete` で行う
  - `struct std_string_unique` を使うため `webrtc/src/webrtc_c/std.h` を include する

### Rust API

- `src/api/environment.rs` の `FieldTrialsViewRef` に `is_disabled(&self, key: &str) -> bool` を追加する
  - doc コメントに、`is_enabled` の否定ではなく両方 false になるキーがあることを明記する
- `src/api/environment.rs` の `FieldTrialsViewRef` に `lookup(&self, key: &str) -> Result<String>` を追加する
  - libwebrtc と同じく、設定されていないキーは空文字列を返す。空の値はフィールドトライアル文字列として不正なため、空文字列が未設定を意味することを doc コメントに明記する
  - UTF-8 に変換できない値は `Result` のエラーになる (`CxxString::to_string` と同じ扱い)
- `src/api/environment.rs` の `EnvironmentRef` に `to_owned(&self) -> Environment` を追加する
  - 既存の `webrtc_Environment_copy` を使うため C API の追加は不要
  - `Environment::clone` の実装もこの経路に寄せて、コピー処理を 1 箇所にする

### ドキュメント

- README と `skills/shiguredo-webrtc/SKILL.md` のフィールドトライアルの説明に `is_disabled` / `lookup` / `EnvironmentRef::to_owned` を追加する
- `CHANGES.md` の `## develop` のフィールドトライアルのエントリに追記する

## 完了条件

- `FieldTrialsViewRef::is_disabled` が `"Disabled"` のフィールドトライアルで true、それ以外で false になること
- `is_enabled` と `is_disabled` が両方 false になるキーがあること (否定の関係にないこと) をテストで確認できること
- `FieldTrialsViewRef::lookup` が `Enabled,offer:true` のようなパラメータ付きの値をそのまま返し、設定されていないキーでは空文字列を返すこと
- `EnvironmentRef::to_owned` で作った `Environment` が、元の `Environment` を drop した後もフィールドトライアルを保持していること
- `cargo test --workspace --features source-build` と `cargo clippy --workspace --features source-build --all-targets -- -D warnings` が通ること
- C/C++ の変更を含むため `python3 webrtc/run.py format --check` が通ること
- README / `skills/shiguredo-webrtc/SKILL.md` / `CHANGES.md` が更新されていること

## テスト

`src/tests.rs` に以下を追加する。

- `field_trials_is_enabled_and_is_disabled_are_independent`
  - 値が `"Enabled"` のキーは `is_enabled` が true で `is_disabled` が false になること
  - 値が `"Disabled"` のキーは `is_enabled` が false で `is_disabled` が true になること
  - 値が `"Enabled,offer:true"` のようにパラメータ付きで `"Enabled"` で始まるキーは `is_enabled` が true になること
  - 値が `"Enabled"` でも `"Disabled"` でもないキーと、設定されていないキーは両方 false になること (否定の関係にないこと)
- `field_trials_lookup_returns_value`
  - 値が `"Enabled,offer:true"` のキーは `lookup` が `"Enabled,offer:true"` を返すこと
  - 値が `"Disabled"` のキーは `lookup` が `"Disabled"` を返すこと
  - 設定されていないキーは空文字列を返すこと
  - フィールドトライアルを指定していない `Environment::new()` でも空文字列を返すこと
- `environment_ref_to_owned_extends_lifetime`
  - フィールドトライアルを設定した `Environment` の `EnvironmentRef` から `to_owned` で `Environment` を作る
  - 元の `Environment` と `EnvironmentFactory` を drop した後も、`to_owned` で作った `Environment` でフィールドトライアルが有効なままであること

## 解決方法

フィールドトライアル API の不足分を追加した。

- `webrtc_c` の `api/field_trials_view.h` / `.cc` に `webrtc_FieldTrialsView_IsDisabled` と `webrtc_FieldTrialsView_Lookup` を追加した。`Lookup` は `std::string` を値で返すため、`webrtc_TransformableFrameInterface_GetMimeType` と同じくヒープ確保したコピーを `std_string_unique` として返す
- `FieldTrialsViewRef::is_disabled` を追加し、`is_enabled` の否定ではないこと (値が `Enabled` でも `Disabled` でもないキーと、設定されていないキーは両方 false になること) を doc コメントに明記した
- `FieldTrialsViewRef::lookup` を追加し、`Enabled,offer:true` のようなパラメータも含めた設定値をそのまま取得できるようにした。設定されていないキーは空文字列になる
- `EnvironmentRef::to_owned` を追加し、`webrtc_Environment_copy` で所有権を持つ `Environment` を作れるようにした。`Environment::clone` の実装もこの経路に寄せた
- `src/tests.rs` に `field_trials_is_enabled_and_is_disabled_are_independent` / `field_trials_lookup_returns_value` / `environment_ref_to_owned_extends_lifetime` を追加した
- README / `skills/shiguredo-webrtc/SKILL.md` / `CHANGES.md` を更新した

確認:

- `cargo test --workspace --features source-build` が成功することを確認した (142 件 + 35 件 + doctest 9 件)
- `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo fmt --all -- --check` / `python3 webrtc/run.py format --check` が成功することを確認した

prebuilt 利用者 (source-build feature を有効にしない利用者) が新しい API を使えるようにするには、この変更を含むバージョンをリリースして libwebrtc_c-*.tar.gz を作り直す必要がある。リリース作業自体はリリース手順で行う。
