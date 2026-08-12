# SSLCertChain_Get が返す借用ポインタの寿命を明確化する

- Priority: Medium
- Polished: 2026-08-12
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

webrtc_c の `webrtc_SSLCertChain_Get` が返すポインタの寿命を明確にする。現状この関数は親オブジェクト (`webrtc::SSLCertChain`) が所有する証明書への借用ポインタを返すが、その寿命が文書化されておらず、親オブジェクト破棄後に返り値を参照すると未定義動作になるため、寿命契約を明記する。

## 優先度根拠

返り値を親オブジェクトの生存期間内に使う限りは問題ないが、寿命が文書化されていないため、利用者が借用と気づかず親破棄後に参照すると未定義動作になる。常時クラッシュする問題ではないが、誤用を誘発しやすい API 設計であるため Medium とする。

## 現状

`webrtc/src/webrtc_c/rtc_base/ssl_certificate.cc` の `webrtc_SSLCertChain_Get` は、`chain->Get(index)` が返す参照のアドレスをそのまま返している。

```cpp
WEBRTC_EXPORT const struct webrtc_SSLCertificate* webrtc_SSLCertChain_Get(
    const struct webrtc_SSLCertChain* self,
    int index) {
  auto chain = reinterpret_cast<const webrtc::SSLCertChain*>(self);
  if (index < 0 || static_cast<size_t>(index) >= chain->GetSize()) {
    return nullptr;
  }
  auto& cert = chain->Get(static_cast<size_t>(index));
  return reinterpret_cast<const struct webrtc_SSLCertificate*>(&cert);
}
```

`chain->Get(index)` は `webrtc::SSLCertChain` が内部に保持する `webrtc::SSLCertificate` への参照を返しており、その所有権は `webrtc::SSLCertChain` 側にある。本関数はその参照のアドレスを `const struct webrtc_SSLCertificate*` として返す借用ポインタである。したがって返り値の有効期間は親の `webrtc::SSLCertChain` の生存期間に依存するが、`webrtc/src/webrtc_c/rtc_base/ssl_certificate.h` の `webrtc_SSLCertChain_Get` 宣言にも実装にも、この寿命に関するコメントが一切ない。親破棄後に参照すると未定義動作になる。

## 設計方針

採用方針: **寿命契約のコメント明記**。

理由:
- 元の C++ `webrtc::SSLCertChain::Get` のシグネチャは参照を返す借用であり、元ヘッダにも「Returns a temporary reference, only valid until the chain is destroyed.」という寿命コメントが既にある。これに忠実な薄いラッパーとして、C ラッパー側にも同じ契約を明記する
- `Get` に所有権返却の意味を持たせるのは、参照を返す元 C++ API の `Get` のシグネチャからの逸脱であり、`webrtc/RULES.md` の薄いラッパー原則（元の C++ API のシグネチャ・名前に忠実に移植すること）に反する。なお元 C++ API にはクローン返却の `Clone()` が存在するが、C ラッパーには未移植であり、その移植は本 issue のスコープ外とする
- 借用ポインタを返す C API は他にも多数存在する（`WEBRTC_DEFINE_VECTOR` が生成する `*_vector_get` や `webrtc_SdpVideoFormat_get_name` 等）が、いずれも寿命コメントは無い。本 issue はスコープを本関数に限定し、その寿命契約のみを明記する

以下の具体的なコメントを追記する:

`webrtc/src/webrtc_c/rtc_base/ssl_certificate.h` の `webrtc_SSLCertChain_Get` 宣言に追記:
```c
// 戻り値は親 webrtc_SSLCertChain が所有する証明書への借用ポインタである。
// 呼び出し側で解放してはならない。
// 返されたポインタは親 webrtc_SSLCertChain の生存期間中のみ有効。
WEBRTC_EXPORT const struct webrtc_SSLCertificate* webrtc_SSLCertChain_Get(
    const struct webrtc_SSLCertChain* self,
    int index);
```

`webrtc/src/webrtc_c/rtc_base/ssl_certificate.cc` の `webrtc_SSLCertChain_Get` 実装にも同じ 3 行のコメントを追記する。

### Rust 側の状況

Rust ラッパーの `SSLCertificateRef<'a>` / `SSLCertChainRef<'a>` は `PhantomData<&'a>` を保持し、
`SSLCertChainRef::get()` (`src/rtc_base/ssl_certificate.rs`) は chain のライフタイム `'a` に
紐づく `SSLCertificateRef<'a>` を返すため、safe Rust の API 経路ではこの未定義動作は
到達不可能。本 issue の修正は Rust 側に影響しない。

### テスト方針

本 issue の変更はコメント追記のみで挙動の変化が無いため、テストは不要である。

### 完了条件

- `webrtc/src/webrtc_c/rtc_base/ssl_certificate.h` の `webrtc_SSLCertChain_Get` 宣言に借用・解放禁止・寿命のコメントが追加されている
- `webrtc/src/webrtc_c/rtc_base/ssl_certificate.cc` の `webrtc_SSLCertChain_Get` 実装にも同様のコメントが追加されている
- コメントの文言が設計方針に示した文案と一致していること
- `CHANGES.md` の `## develop` に `### misc` サブセクションを新設し、[UPDATE] エントリが追記されていること
