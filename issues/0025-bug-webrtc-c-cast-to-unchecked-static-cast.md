# cast_to_* の未チェック static_cast ダウンキャストを安全にする

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-cast-to-unchecked-static-cast

## 目的

webrtc_c の `cast_to_*` 系 API が行うダウンキャストを安全にするか、少なくとも誤用を避けられるよう前提条件を明確にする。現状は実型を確認せずに `static_cast` でダウンキャストしており、渡されたオブジェクトの実型が想定と一致しない場合に未定義動作を引き起こすため、これを是正する。

## 優先度根拠

C API の利用者が実型を取り違えて `cast_to_*` を呼ぶと未定義動作になるが、正しく使えば問題は起きない。クラッシュが常時発生する種類の問題ではなく、利用者の誤用が前提条件となるため High ではない。一方で、未定義動作はデバッグが極めて困難であり、安全側に倒すか前提を明示する価値が高いため Medium とする。

## 現状

ダウンキャスト用のマクロは `webrtc/src/webrtc_c/common.impl.h:10-26` で定義されている。

`webrtc/src/webrtc_c/common.impl.h:10-15` の `WEBRTC_DEFINE_CAST`:

```cpp
#define WEBRTC_DEFINE_CAST(type, cast_to, cpptype, cpp_cast_to)             \
  WEBRTC_EXPORT struct cast_to* WEBRTC_CONCAT(                              \
      type, WEBRTC_CONCAT(_cast_to_, cast_to))(struct type * self) {        \
    auto s = reinterpret_cast<cpptype*>(self);                              \
    return reinterpret_cast<struct cast_to*>(static_cast<cpp_cast_to*>(s)); \
  }
```

`webrtc/src/webrtc_c/common.impl.h:17-26` の `WEBRTC_DEFINE_CAST_REFCOUNTED`:

```cpp
#define WEBRTC_DEFINE_CAST_REFCOUNTED(type, cast_to, cpptype, cpp_cast_to) \
  WEBRTC_EXPORT struct WEBRTC_CONCAT(cast_to, _refcounted) *               \
      WEBRTC_CONCAT(type, WEBRTC_CONCAT(_refcounted_cast_to_, cast_to))(   \
          struct WEBRTC_CONCAT(type, _refcounted) * self) {                \
    auto s = reinterpret_cast<cpptype*>(                                   \
        WEBRTC_CONCAT(type, _refcounted_get)(self));                       \
    webrtc::scoped_refptr<cpp_cast_to> ptr(static_cast<cpp_cast_to*>(s));  \
    return reinterpret_cast<struct WEBRTC_CONCAT(cast_to, _refcounted)*>(  \
        ptr.release());                                                    \
  }
```

いずれも `static_cast<cpp_cast_to*>(s)` で基底型から派生型へダウンキャストしているが、`s` が指すオブジェクトの実型が `cpp_cast_to` であるかどうかを一切確認していない。実型が不一致の場合、`static_cast` によるダウンキャストの結果を利用すると未定義動作となる。

実際の利用例として `webrtc/src/webrtc_c/api/media_stream_interface.cc:38-45` で、`webrtc::MediaStreamTrackInterface` から `webrtc::VideoTrackInterface` および `webrtc::AudioTrackInterface` へのダウンキャストが定義されている。

```cpp
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                              webrtc_VideoTrackInterface,
                              webrtc::MediaStreamTrackInterface,
                              webrtc::VideoTrackInterface);
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                              webrtc_AudioTrackInterface,
                              webrtc::MediaStreamTrackInterface,
                              webrtc::AudioTrackInterface);
```

例えば音声トラックに対して `webrtc_MediaStreamTrackInterface_refcounted_cast_to_webrtc_VideoTrackInterface` を呼ぶと、実型 (`AudioTrackInterface`) と要求型 (`VideoTrackInterface`) が不一致のまま `static_cast` され、未定義動作になる。

## 設計方針

以下のいずれか、または組み合わせを検討する。

- 型タグの確認による安全なダウンキャスト: `webrtc::MediaStreamTrackInterface` には `kind()` で `"video"` / `"audio"` を判別できるため、ダウンキャスト前に実型を確認し、不一致なら `nullptr` を返す方式を検討する。ただし RULES.md の「薄いラッパー」原則 (`webrtc/RULES.md:5-6`) に照らし、ラッパー側で判定ロジックを足すことが許容されるかを確認する。
- 前提条件の明記: 安全なダウンキャスト手段の導入が難しい、あるいは薄いラッパー原則に反する場合は、最低限「`cast_to_*` は呼び出し側が実型を保証する責務を負う」「実型と不一致な場合は未定義動作」であることをマクロ定義箇所および各 `cast_to_*` の宣言箇所のコメントとして明記する。

どちらを採るかは設計判断が必要なため、判断の根拠を issue 対応時に整理し、必要なら許可を得ること。

## 完了条件

- 不正な型を渡した `cast_to_*` 呼び出しが未定義動作にならないようにする、または `cast_to_*` の前提条件 (呼び出し側が実型を保証する責務、不一致時の挙動) がコードコメントで明確になっている。
- 採用した方針が RULES.md の薄いラッパー原則と整合していることを確認している。
