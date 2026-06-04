# peer_connection_interface.cc の _refcounted 直接キャストを RULES.md 準拠にする

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-refcounted-direct-cast

## 目的

webrtc_c の C ラッパーにおける `*_refcounted` 型の取り扱いを `webrtc/RULES.md` に準拠させ、メモリモデルの一貫性を保つ。RULES.md は `*_refcounted` 型を直接 C++ の型へキャストすることを明確に禁止しており、現状のコードはこの規約に違反しているため是正する。

## 優先度根拠

現状でも動作上は問題が顕在化していない可能性が高いが、`webrtc/RULES.md` が「必須」として定めているアクセス経路に違反している。規約違反を放置すると、`*_refcounted` の内部表現を変更した際に当該箇所が破綻し、原因追跡が困難になる。直ちにクラッシュする種類の問題ではないため High ではないが、規約準拠とメモリモデルの一貫性の観点から Medium とする。

## 現状

`webrtc/RULES.md` には以下の規約がある (`webrtc/RULES.md:23-24`)。

```
- `*_refcounted` の型を直接 C++ の型にキャストしてはならない
  - 必ず `*_refcounted_get()` 関数を経由すること
```

しかし `webrtc/src/webrtc_c/api/peer_connection_interface.cc` の以下の箇所では、`*_refcounted` ポインタを `*_refcounted_get()` を経由せず直接 `reinterpret_cast` している。

`webrtc/src/webrtc_c/api/peer_connection_interface.cc:973-980` の `set_adm`:

```cpp
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_set_adm(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioDeviceModule_refcounted* adm) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto audio_device_module = reinterpret_cast<webrtc::AudioDeviceModule*>(adm);
  deps->adm = audio_device_module;
}
```

`webrtc/src/webrtc_c/api/peer_connection_interface.cc:992-1001` の `set_audio_encoder_factory`:

```cpp
  auto factory =
      reinterpret_cast<webrtc::AudioEncoderFactory*>(audio_encoder_factory);
  deps->audio_encoder_factory = factory;
```

`webrtc/src/webrtc_c/api/peer_connection_interface.cc:1002-1011` の `set_audio_decoder_factory`:

```cpp
  auto factory =
      reinterpret_cast<webrtc::AudioDecoderFactory*>(audio_decoder_factory);
  deps->audio_decoder_factory = factory;
```

いずれも引数は `struct webrtc_*_refcounted*` 型でありながら、`webrtc_*_refcounted_get()` を経由せずに直接 C++ 型へキャストしている。比較として、同じファイル内の `_unique` 系の処理 (`webrtc/src/webrtc_c/api/peer_connection_interface.cc:984-990` の `set_event_log_factory` など) では `webrtc_RtcEventLogFactory_unique_get()` を経由しており、`_refcounted` 側だけが規約から外れている。

## 設計方針

`webrtc/RULES.md:23-24` に従い、`*_refcounted` ポインタから C++ 型を取得する際は必ず対応する `webrtc_*_refcounted_get()` 関数を経由するよう書き換える。具体的には `set_adm` / `set_audio_encoder_factory` / `set_audio_decoder_factory` の各関数で、引数の `*_refcounted*` を直接 `reinterpret_cast` するのではなく、`webrtc_AudioDeviceModule_refcounted_get()` などを呼び出して得た `struct webrtc_*` を経由してから C++ 型へ変換する。

代入先 (`deps->adm` 等) が参照カウントの所有権をどのように扱うか (`scoped_refptr` への代入による参照カウント増加の有無) についても RULES.md の `*_refcounted` の寿命管理ルール (`webrtc/RULES.md:13-15`、`webrtc/RULES.md:25`) を確認し、所有権の移譲方法を現状の挙動と一致させること。挙動を変えずに経路だけを規約準拠にすることを基本とする。

## 完了条件

- `webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `set_adm` / `set_audio_encoder_factory` / `set_audio_decoder_factory` から、`*_refcounted` ポインタを直接 C++ 型へ `reinterpret_cast` する記述が排除されている。
- `*_refcounted` 型へのアクセスがすべて `webrtc_*_refcounted_get()` 経由となり、`webrtc/RULES.md:23-24` に準拠している。
- 参照カウントの扱いが従来の挙動と一致している。
