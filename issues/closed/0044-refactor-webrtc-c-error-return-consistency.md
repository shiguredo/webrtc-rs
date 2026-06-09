# webrtc_c のエラー伝達方法を統一する

- Priority: Low
- Polished: 2026-06-09
- Completed: 2026-06-09
- Created: 2026-06-05
- Model: DeepSeek V4 Pro

## 目的

`webrtc_c` の C ラッパー API では、エラーの伝達方法が `webrtc_RTCError_unique*` を戻り値として返す API と、`out` パラメータ（`webrtc_RTCError_unique**`）で返す API とで混在している。
このため、呼び出し側はどちらの方式かを API ごとに把握する必要があり、扱いがばらつく。
エラー伝達の方針を out パラメータ方式に統一し、呼び出し側の扱いをそろえる。

また、戻り値方式の API と `peer_connection_interface.cc` の out パラメータ方式 API の両方で、C++ オブジェクトの `*_unique` への渡し方が RULES.md に違反している（`new T(...)` を直接 `reinterpret_cast` している）。
この RULES.md 違反を同時に修正し、`std::unique_ptr` + `release()` を用いた正しい確保方法に統一する。

## 優先度根拠

機能的には双方とも動作するため、即座の不具合にはつながらない。
ただし C-API としての一貫性・利用しやすさに関わり、利用側コードの単純化や誤用防止に資する。優先度は Low とする。

## 設計判断根拠

RULES.md は「元の C++ API のシグネチャ・名前に忠実に移植する」ことを原則としている。
C++ 側の `RtpTransceiverInterface::SetCodecPreferences` と `RtpSenderInterface::SetParameters` はいずれも `webrtc::RTCError` を戻り値で返すため、戻り値方式のほうが C++ のシグネチャに忠実である。

一方、`webrtc_c` の他のエラー返却 API（`peer_connection_interface.cc` の 7 関数）はすべて out パラメータ方式を採用しており、C-API 全体としては out パラメータ方式が支配的である。
C++ API に忠実であることより、C-API 単体としての一貫性を優先し、少数側の 2 API を多数派に合わせる判断とした。

本判断は RULES.md の「忠実な移植」原則に対する明確な例外となるため、RULES.md の「細かい部分」セクションに「`*_unique` でない C 関数の移植で C++ 側が戻り値を返す場合、C-API 全体の一貫性を優先して out パラメータ方式で統一することがある」旨を追記する。

## 現状

`webrtc_c` の C-API におけるエラー伝達は以下の 2 方式が混在している:

1. **戻り値方式**: 関数の戻り値として `webrtc_RTCError_unique*` を返す。成功時は `nullptr`、失敗時はエラーオブジェクトのポインタを返す。
2. **out パラメータ方式**: 最終引数 `out_rtc_error`（`webrtc_RTCError_unique**`）でエラーを出力する。成功時には `*out_rtc_error` に `nullptr` が設定され、失敗時のみ非 null となり、呼び出し側が `webrtc_RTCError_unique_delete` で解放責任を負う。

### 戻り値方式の API（全 2 件）

| # | 関数 | ファイル | 行 |
|---|------|----------|----|
| 1 | `webrtc_RtpTransceiverInterface_SetCodecPreferences` | `webrtc_c/api/rtp_transceiver_interface.cc` | 63 |
| 2 | `webrtc_RtpSenderInterface_SetParameters` | `webrtc_c/api/rtp_sender_interface.cc` | 26 |

### out パラメータ方式の API（全 7 件）

すべて `webrtc_c/api/peer_connection_interface.cc` に存在する。これらの関数はすでに out パラメータ方式を採用しているため、本 issue でシグネチャ変更の対象にはならない。

### RULES.md 違反（`new T` → `std::make_unique` + `release` が必要な箇所）

戻り値方式・out パラメータ方式の両方で、以下の RULES.md ルールに違反している:

> C++ オブジェクトを `*_unique` に渡すときは、必ず `std::unique_ptr<CppType>` で構築し、`p.release()` したもののみをキャストする。

## 設計方針

1. **エラー伝達は `out` パラメータ（`webrtc_RTCError_unique** out_rtc_error`）に統一する。**
   - 戻り値方式の 2 つの API を out パラメータ方式に変更する。
   - 戻り値型を `void` に変更する。これにより C API のシグネチャが変更され、ABI 互換性が破壊されるため、CHANGES.md に `[CHANGE]` エントリを追加する。
   - RULES.md の「デリファレンス前に `assert(ptr != nullptr)` を入れる」ルールに従い、両関数とも `*out_rtc_error` のデリファレンス前に `assert(out_rtc_error != nullptr)` を追加する。
2. **C++ オブジェクトの確保時は `std::unique_ptr` + `release()` を使う。**
   - `new webrtc::RTCError(...)` を `std::make_unique<webrtc::RTCError>(...).release()` に変更する。
   - 対象: 2 つの変更対象 API + `peer_connection_interface.cc` の既存の全 `new webrtc::RTCError` 箇所。
   - CHANGES.md に `[FIX]` エントリを追加し、RULES.md 違反を修正したことを記録する。
3. `rtp_sender_interface.cc:32-35` のランタイム null チェック（`if (parameters == nullptr) { ... INVALID_PARAMETER ... }`）は RULES.md の「C ラッパーではポインタ引数の null チェックを原則として行わない」に反するため削除する。31 行目の `assert(parameters != nullptr)` は維持する。
4. 両 `.cc` ファイルに新たに `std::make_unique` を使用するため明示的に `#include <memory>` を追加し、`assert(out_rtc_error != nullptr)` を使用するため `rtp_transceiver_interface.cc` には `#include <assert.h>` も追加する（`rtp_sender_interface.cc` は既に `#include <assert.h>` 済み）。
5. 変更する場合は呼び出し側（`webrtc/src/whip.c`、`src/api/rtp.rs`）の扱いも合わせて更新し、成功・失敗時の挙動を変えない。
6. `src/api/rtp.rs` にある既存の `.unwrap()` を `.expect("BUG: error is null")` に修正する（AGENTS.md の `.expect()` 使用ルールに従う）。

## ブランチ

- prefix: `feature/change-`（ABI 互換性を破壊するため、AGENTS.md の「後方互換のない変更」ルールに従う）
- ブランチ名例: `feature/change-webrtc-c-error-return-consistency`

## 変更対象ファイル

### 関数シグネチャ変更（戻り値 → out パラメータ）

いずれの行番号も変更**前**のものを基準とする。

| ファイル | 変更内容 |
|----------|----------|
| `webrtc_c/api/rtp_transceiver_interface.h:36-39` | 戻り値 `webrtc_RTCError_unique*` を `void` に変更し、引数 `webrtc_RTCError_unique** out_rtc_error` を追加。`#include "rtc_error.h"` を追加する |
| `webrtc_c/api/rtp_transceiver_interface.cc:63-76` | 実装を out パラメータ方式に変更。`#include <memory>` と `#include <assert.h>` を追加する |
| `webrtc_c/api/rtp_sender_interface.h:19-22` | 戻り値 `webrtc_RTCError_unique*` を `void` に変更し、引数 `webrtc_RTCError_unique** out_rtc_error` を追加（`#include "rtc_error.h"` は既存） |
| `webrtc_c/api/rtp_sender_interface.cc:26-44` | 実装を out パラメータ方式に変更。`#include <memory>` を追加する。32-35 行目のランタイム null チェックブロックも併せて削除する |

### 具体的な実装変更例

#### rtp_transceiver_interface.cc（SetCodecPreferences）変更後

```cpp
WEBRTC_EXPORT void
webrtc_RtpTransceiverInterface_SetCodecPreferences(
    struct webrtc_RtpTransceiverInterface* self,
    struct webrtc_RtpCodecCapability_vector* codecs,
    struct webrtc_RTCError_unique** out_rtc_error) {
  assert(out_rtc_error != nullptr);
  auto transceiver = reinterpret_cast<webrtc::RtpTransceiverInterface*>(self);
  auto vec = reinterpret_cast<std::vector<webrtc::RtpCodecCapability>*>(codecs);
  auto result = transceiver->SetCodecPreferences(*vec);
  if (result.ok()) {
    *out_rtc_error = nullptr;
  } else {
    auto error = std::make_unique<webrtc::RTCError>(result);
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(error.release());
  }
}
```

#### rtp_sender_interface.cc（SetParameters）変更後

```cpp
WEBRTC_EXPORT void
webrtc_RtpSenderInterface_SetParameters(
    struct webrtc_RtpSenderInterface* self,
    const struct webrtc_RtpParameters* parameters,
    struct webrtc_RTCError_unique** out_rtc_error) {
  assert(out_rtc_error != nullptr);
  assert(parameters != nullptr);
  auto sender = reinterpret_cast<webrtc::RtpSenderInterface*>(self);
  auto p = reinterpret_cast<const webrtc::RtpParameters*>(parameters);
  auto result = sender->SetParameters(*p);
  if (result.ok()) {
    *out_rtc_error = nullptr;
  } else {
    auto error = std::make_unique<webrtc::RTCError>(result);
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(error.release());
  }
}
```

### RULES.md 違反修正（`new webrtc::RTCError` → `std::make_unique` + `release`）

| ファイル | 行 | 変更内容 |
|----------|-----|----------|
| `webrtc_c/api/rtp_transceiver_interface.cc` | 73-74 | `new webrtc::RTCError(result)` → `std::make_unique<webrtc::RTCError>(result).release()` |
| `webrtc_c/api/rtp_sender_interface.cc` | 33-35 | ブロックごと削除（RULES.md の「null チェック原則禁止」に従う） |
| `webrtc_c/api/rtp_sender_interface.cc` | 42-43 | `new webrtc::RTCError(result)` → `std::make_unique<webrtc::RTCError>(result).release()` |
| `webrtc_c/api/peer_connection_interface.cc` | 482-484, 505-507, 530-532, 556-558, 573-575, 669-671, 1166-1168 | 同様の違反を修正。全 7 箇所 |

### RULES.md の追記

`webrtc/RULES.md` の「細かい部分」セクションに以下を追記する:

```markdown
- `*_unique` でない C 関数の移植で C++ 側が戻り値を返す場合、C-API 全体の一貫性を優先して out パラメータ方式で統一することがある
```

### 呼び出し側の変更

#### C 側（whip.c）

`webrtc_RtpSenderInterface_SetParameters` は whip.c 内では呼ばれていないため、C 側での呼び出し変更は `SetCodecPreferences` のみ。

| ファイル | 行 | 変更内容 |
|----------|-----|----------|
| `webrtc/src/whip.c` | 1323, 1477 | `SetCodecPreferences` の呼び出し: 戻り値受け取り → `&rtc_error` を引数に追加 |

```c
// 変更前（whip.c:1323）
rtc_error = webrtc_RtpTransceiverInterface_SetCodecPreferences(
    webrtc_RtpTransceiverInterface_refcounted_get(transceiver), codecs);

// 変更後
webrtc_RtpTransceiverInterface_SetCodecPreferences(
    webrtc_RtpTransceiverInterface_refcounted_get(transceiver), codecs, &rtc_error);
```

変更後の関数実装は成功・失敗の両分岐で `*out_rtc_error` を上書するため、呼び出し前の NULL 再代入は不要である。
video 側（whip.c:1477）も同様に変更する。

#### Rust 側（src/api/rtp.rs）

C ヘッダのシグネチャ変更に伴い、bindgen が自動生成する `bindings.rs` の FFI 宣言も変わる。`--features source-build` でビルドする際に再生成される。

| ファイル | 行 | 変更内容 |
|----------|-----|----------|
| `src/api/rtp.rs` | 1417-1429 | `set_codec_preferences`: 戻り値チェック → out パラメータ `&mut err` 渡しに変更。既存の `.unwrap()` を `.expect("BUG: error is null")` に修正 |
| `src/api/rtp.rs` | 1490-1499 | `set_parameters`: 戻り値チェック → out パラメータ `&mut err` 渡しに変更。既存の `.unwrap()` を `.expect("BUG: error is null")` に修正 |

Rust 側の変更例（`set_codec_preferences`）:

```rust
// 変更後
pub fn set_codec_preferences(&mut self, codecs: &RtpCodecCapabilityVector) -> Result<()> {
    let mut err: *mut ffi::webrtc_RTCError_unique = std::ptr::null_mut();
    unsafe {
        ffi::webrtc_RtpTransceiverInterface_SetCodecPreferences(
            self.raw_ref.as_ptr(),
            codecs.as_ptr(),
            &mut err,
        )
    };
    if !err.is_null() {
        let rtc = RtcError::from_unique_ptr(
            NonNull::new(err).expect("BUG: error is null"),
        );
        return Err(Error::RtcError(rtc));
    }
    Ok(())
}
```

`set_parameters`（`src/api/rtp.rs:1490-1499`）も同様の変更を行う。

## テスト

本変更は C API シグネチャ変更 + メモリ確保方法修正 + 呼び出し側修正 + `.unwrap()` → `.expect()` 修正の組み合わせである。

### コンパイル確認

- C 側: `python3 run.py build ubuntu-24.04_x86_64` が成功すること
- Rust 側 (`source-build`): `cargo build --features source-build` が成功し、bindings.rs が再生成されること
- Rust 側 (`prebuilt` / 通常): `cargo build` が成功すること
- examples: `cargo build --example whip --features whip_client` が成功すること

### 単体テスト（Rust 側）

- `cargo test rtp_sender_get_set_parameters`（`src/tests.rs`）が通過すること。`set_parameters` の呼び出しは `src/tests.rs:2379` のみであり、他に Rust 側からの `set_parameters` 呼び出しはない
- `cargo test --lib` でその他のテストが通過すること

### whip_c 正常系確認

- whip_c バイナリをビルドし、既存の正常系動作が変更前と変わらないこと
- `SetCodecPreferences` の失敗時テストは WHIP シグナリング完了（`WHIP_ENDPOINT` 環境変数による結合テスト）が必要なため、本 issue では確認をスコープ外とする

### 静的確認

- 完了後に以下を grep で確認する:
  - `rg 'new webrtc::RTCError' webrtc/src/webrtc_c/` が 0 件であること
  - `rg 'reinterpret_cast<webrtc_RTCError_unique\*>' webrtc/src/webrtc_c/api/rtp_transceiver_interface.cc webrtc/src/webrtc_c/api/rtp_sender_interface.cc` で `new` が残っていないこと（`release()` 経由のみであること）
  - `rtp_sender_interface.cc` の `if (parameters == nullptr)` が削除されていることを目視確認

## 完了条件

- `webrtc_c` のエラー伝達方法が out パラメータ方式に統一されている。
- `reinterpret_cast<webrtc_RTCError_unique*>(new webrtc::RTCError(...))` というパターンが `webrtc_c` 以下から一掃されている（grep で 0 件）。
- `rtp_sender_interface.cc` のランタイム null チェック (`if (parameters == nullptr)`) が削除されている。
- `std::make_unique<webrtc::RTCError>(...).release()` を用いた正しい確保方法に修正されている。
- `src/api/rtp.rs` の `.unwrap()` が `.expect("BUG: ...")` に修正されている。
- 両 `.cc` ファイルに `#include <memory>` が追加され、`rtp_transceiver_interface.cc` には `#include <assert.h>` も追加されている。
- `rtp_transceiver_interface.h` に `#include "rtc_error.h"` が追加されている。
- 両関数の `*out_rtc_error` デリファレンス前に `assert(out_rtc_error != nullptr)` が追加されている。
- `webrtc/RULES.md` の「細かい部分」に out パラメータ統一の例外が追記されている。
- C 側のビルドが通ること（`python3 run.py build ubuntu-24.04_x86_64`）。
- Rust 側のビルドが通ること（`cargo build` または `cargo build --features source-build`）。
- Rust 側の関連テストが通過すること（`cargo test rtp_sender_get_set_parameters`、`cargo test --lib`）。
- 既存の呼び出し側（`whip.c`、`src/api/rtp.rs`）の成功・失敗時の挙動が変わっていない。
- CHANGES.md の `## develop` セクションに `[CHANGE]` エントリ（エラー伝達方式の統一）と `[FIX]` エントリ（RULES.md 違反修正）を追加している。
  - `[CHANGE]` には「`webrtc_c` のエラー伝達方法を out パラメータ方式に統一する」を記載する
  - `[FIX]` には「`webrtc_c` の `new webrtc::RTCError` を `std::make_unique` + `release()` に修正する」を記載する（`### misc` サブセクションに配置）

## 解決方法

- `webrtc_RtpTransceiverInterface_SetCodecPreferences` と `webrtc_RtpSenderInterface_SetParameters` の戻り値型を `webrtc_RTCError_unique*` から `void` に変更し、`webrtc_RTCError_unique** out_rtc_error` 引数を追加した
- 両関数の実装で、成功時は `*out_rtc_error = nullptr`、失敗時は `std::make_unique<webrtc::RTCError>(result).release()` を `reinterpret_cast` する方式に変更した
- `rtp_sender_interface.cc` のランタイム null チェック (`if (parameters == nullptr)`) ブロックを削除し、`assert(parameters != nullptr)` は維持した
- `webrtc_c/api/peer_connection_interface.cc` の全 7 箇所の `new webrtc::RTCError(...)` を `std::make_unique<webrtc::RTCError>(...).release()` に修正した
- `webrtc/src/whip.c` の `SetCodecPreferences` 呼び出し (2 箇所) を out パラメータ方式に変更した
- `src/api/rtp.rs` の `set_codec_preferences` と `set_parameters` を out パラメータ方式に変更し、`.unwrap()` を `.expect("BUG: error is null")` に修正した
- `webrtc/RULES.md` の「細かい部分」に out パラメータ統一の例外を追記した
- `CHANGES.md` の `## develop` に `[CHANGE]` エントリ、`### misc` に `[FIX]` エントリを追加した
- `rtp_transceiver_interface.cc` に `#include <memory>` と `#include <assert.h>` を追加し、`rtp_sender_interface.cc` に `#include <memory>` を追加した
- `rtp_transceiver_interface.h` に `#include "rtc_error.h"` を追加した
- `source-build` ビルドと `cargo test --lib` 全 101 テストが通過することを確認した
