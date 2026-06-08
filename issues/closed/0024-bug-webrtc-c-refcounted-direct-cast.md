# peer_connection_interface.cc の _refcounted 直接キャストを RULES.md 準拠にする

- Priority: Medium
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8
- Completed: 2026-06-08

## 目的

webrtc_c の C ラッパーにおける `*_refcounted` 型の取り扱いを `webrtc/RULES.md` に準拠させる。
RULES.md は `*_refcounted` 型を直接 C++ の型へキャストすることを明確に禁止しており、
現状のコードはこの規約に違反しているため是正する。

## 優先度根拠

`webrtc/RULES.md` が「必須」として定めているアクセス経路に違反している。
規約違反を放置すると、`*_refcounted` の内部表現を変更した際に当該箇所が破綻し、
原因追跡が困難になる。直ちにクラッシュする種類の問題ではないため Medium とする。

## 現状

`webrtc/RULES.md:23-24` には以下の規約がある:

```
- `*_refcounted` の型を直接 C++ の型にキャストしてはならない
  - 必ず `*_refcounted_get()` 関数を経由すること
```

`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の以下の 3 関数で、
`*_refcounted` ポインタを `*_refcounted_get()` を経由せず直接 `reinterpret_cast` している。

**違反 1: set_adm (L973-980)**

```cpp
auto audio_device_module = reinterpret_cast<webrtc::AudioDeviceModule*>(adm);
```

**違反 2: set_audio_encoder_factory (L992-1000)**

```cpp
auto factory =
    reinterpret_cast<webrtc::AudioEncoderFactory*>(audio_encoder_factory);
```

**違反 3: set_audio_decoder_factory (L1002-1011)**

```cpp
auto factory =
    reinterpret_cast<webrtc::AudioDecoderFactory*>(audio_decoder_factory);
```

`grep` による全件検証の結果、当該ファイル内の他すべての `_refcounted` 取り扱い
（`AddTransceiverWithTrack` L526-529、`AddTrack` L551-555、`CreateAudioTrack`
L1310-1312 など全 14 箇所）は `_refcounted_get()` 経由で正しく実装されており、
違反は上記 3 箇所のみである。また `peer_connection_interface.cc` 以外の全 `*.cc`
ファイルでも `_refcounted_get()` 経由が守られている。

## 修正内容

### set_adm (L978)

修正前:
```cpp
auto audio_device_module = reinterpret_cast<webrtc::AudioDeviceModule*>(adm);
```

修正後:
```cpp
auto raw_adm = webrtc_AudioDeviceModule_refcounted_get(adm);
auto audio_device_module = reinterpret_cast<webrtc::AudioDeviceModule*>(raw_adm);
```

### set_audio_encoder_factory (L998-999)

修正前:
```cpp
auto factory =
    reinterpret_cast<webrtc::AudioEncoderFactory*>(audio_encoder_factory);
```

修正後:
```cpp
auto raw_factory =
    webrtc_AudioEncoderFactory_refcounted_get(audio_encoder_factory);
auto factory =
    reinterpret_cast<webrtc::AudioEncoderFactory*>(raw_factory);
```

### set_audio_decoder_factory (L1008-1009)

修正前:
```cpp
auto factory =
    reinterpret_cast<webrtc::AudioDecoderFactory*>(audio_decoder_factory);
```

修正後:
```cpp
auto raw_factory =
    webrtc_AudioDecoderFactory_refcounted_get(audio_decoder_factory);
auto factory =
    reinterpret_cast<webrtc::AudioDecoderFactory*>(raw_factory);
```

### 設計の根拠

- `_refcounted_get()` は `common.impl.h` の `WEBRTC_DEFINE_REFCOUNTED` マクロで
  `return reinterpret_cast<struct type*>(p)` と展開される。つまり現状の直接
  `reinterpret_cast` と `_refcounted_get()` 経由は**実行時に同一の挙動**を持つ
  （ポインタ値も参照カウントも変化しない）
- `deps->adm = audio_device_module` の代入先は `scoped_refptr<webrtc::AudioDeviceModule>`
  であり、`scoped_refptr` は raw ポインタからの暗黙構築時に `AddRef()` を呼ぶ
- 修正後も `_refcounted_get()` → `reinterpret_cast<C++型>` の 2 段階を経るが、
  `_refcounted_get()` が単なる reinterpret_cast である以上、最終的なポインタ値も
  参照カウントの遷移も修正前と完全に一致する
- 同様に `audio_encoder_factory` / `audio_decoder_factory` も
  `scoped_refptr<webrtc::XxxFactory>` への代入であり、AddRef の挙動は同一

## 後方互換への影響

C API のシグネチャは一切変更されないため、後方互換は完全に維持される。

## CHANGES.md

```
### misc

- [FIX] peer_connection_interface.cc の _refcounted 直接キャストを _refcounted_get() 経由にする
  - @実装者
```

## テスト戦略

C++ 実装の内部修正であり、C API の外部仕様は変わらないため新規テストは不要。
以下の確認で完了とする:

- 既存の whip.c / whep.c がビルド可能であること
- ビルド警告が増えないこと

## 解決方法

以下の修正を行った:

1. **C++ 実装** (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`):
   - `set_adm`: `reinterpret_cast<webrtc::AudioDeviceModule*>(adm)` を `reinterpret_cast<webrtc::AudioDeviceModule*>(webrtc_AudioDeviceModule_refcounted_get(adm))` に修正
   - `set_audio_encoder_factory`: `reinterpret_cast<webrtc::AudioEncoderFactory*>(audio_encoder_factory)` を `reinterpret_cast<webrtc::AudioEncoderFactory*>(webrtc_AudioEncoderFactory_refcounted_get(audio_encoder_factory))` に修正
   - `set_audio_decoder_factory`: `reinterpret_cast<webrtc::AudioDecoderFactory*>(audio_decoder_factory)` を `reinterpret_cast<webrtc::AudioDecoderFactory*>(webrtc_AudioDecoderFactory_refcounted_get(audio_decoder_factory))` に修正
2. **C ラッパーヘッダーの include 追加**: 上記の `_refcounted_get` 関数を利用可能にするため、C ラッパーの `audio/audio_device.h`、`audio_codecs/audio_decoder_factory.h`、`audio_codecs/audio_encoder_factory.h` を include するようにした

3. **変更履歴** (`CHANGES.md`):
   `[FIX]` エントリを misc セクションに追記した

## 完了条件

- `set_adm` / `set_audio_encoder_factory` / `set_audio_decoder_factory` の 3 関数で、
  `*_refcounted` ポインタを直接 C++ 型へ `reinterpret_cast` する記述が排除されている
- すべての `*_refcounted` アクセスが `webrtc_*_refcounted_get()` 経由になっている
- 修正後のコードが `common.impl.h` の `WEBRTC_DEFINE_REFCOUNTED` 実装と一致する
  パターン（`_refcounted_get` → `reinterpret_cast<C++型>`）に従っている
- 既存の whip.c / whep.c がビルド可能で、警告が増えていない
