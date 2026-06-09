# SSLCertChain_Get が返す借用ポインタの寿命を明確化する

- Priority: Medium
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

webrtc_c の `webrtc_SSLCertChain_Get` が返すポインタの寿命を明確にする。現状この関数は親オブジェクト (`webrtc::SSLCertChain`) が所有する証明書への借用ポインタを返すが、その寿命が文書化されておらず、親オブジェクト破棄後に返り値を参照すると未定義動作になり得るため、寿命契約の明記または所有権を返す API への変更を行う。

## 優先度根拠

返り値を親オブジェクトの生存期間内に使う限りは問題ないが、寿命が文書化されていないため、利用者が借用と気づかず親破棄後に参照すると未定義動作になる。常時クラッシュする問題ではないが、誤用を誘発しやすい API 設計であり、コメント追記または API 変更で誤用を防げるため Medium とする。

## 現状

`webrtc/src/webrtc_c/rtc_base/ssl_certificate.cc:83-92` の `webrtc_SSLCertChain_Get` は、`chain->Get(index)` が返す参照のアドレスをそのまま返している。

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

`chain->Get(index)` は `webrtc::SSLCertChain` が内部に保持する `webrtc::SSLCertificate` への参照を返しており、その所有権は `webrtc::SSLCertChain` 側にある。本関数はその参照のアドレスを `const struct webrtc_SSLCertificate*` として返す借用ポインタである。したがって返り値の有効期間は親の `webrtc::SSLCertChain` の生存期間に依存するが、ヘッダ宣言 (`webrtc/src/webrtc_c/rtc_base/ssl_certificate.h:31-33`) にも実装にも、この寿命に関するコメントが一切ない。利用者が返り値を解放しようとしたり、親破棄後に参照したりすると未定義動作になる。

```cpp
WEBRTC_EXPORT const struct webrtc_SSLCertificate* webrtc_SSLCertChain_Get(
    const struct webrtc_SSLCertChain* self,
    int index);
```

## 設計方針

採用方針: **寿命契約のコメント明記**。

理由:
- 元の C++ `webrtc::SSLCertChain::Get` のシグネチャは参照を返す借用であり、これに忠実な
  薄いラッパーが RULES.md の方針に沿う
- 所有権を返す API（クローン返却等）は元 C++ API から逸脱し、RULES.md の
  薄いラッパー原則 (`webrtc/RULES.md:5-6`) に反する
- コードベース全体で借用ポインタを返す C API は本関数が唯一であり、この特異性を
  明示的に文書化することが重要

以下の具体的なコメントを追記する:

`ssl_certificate.h` の `webrtc_SSLCertChain_Get` 宣言に追記:
```c
// 戻り値は親 SSLCertChain が所有する証明書への借用ポインタである。
// 呼び出し側で解放してはならない。
// 返されたポインタは親 SSLCertChain の生存期間中のみ有効。
WEBRTC_EXPORT const struct webrtc_SSLCertificate* webrtc_SSLCertChain_Get(
    const struct webrtc_SSLCertChain* self,
    int index);
```

`ssl_certificate.cc` の実装にも同様のコメントを追記する。

### Rust 側の状況

Rust ラッパー `SSLCertChainRef::get()` (`src/rtc_base/ssl_certificate.rs:75-85`) は
`PhantomData<&'a>` によりライフタイムを型で強制しており、Rust 利用者にはこの
未定義動作は到達不可能。本 issue の修正は Rust 側に影響しない。

### テスト方針

寿命違反は静的解析・ASan 等でも検出困難なため、本 issue ではテスト不要。
コメントによる契約明示のみで対処する。

### 完了条件

- `ssl_certificate.h` の `webrtc_SSLCertChain_Get` 宣言に借用・解放禁止・寿命の
  コメントが追加されている
- `ssl_certificate.cc` の実装にも同様のコメントが追加されている
- コメント文案が上記の通りであること（レビューで確認）
