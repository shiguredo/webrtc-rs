# SSLCertChain_Get が返す借用ポインタの寿命を明確化する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-ssl-cert-chain-get-lifetime

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

以下のいずれかを検討する。

- 寿命契約のコメント明記: `webrtc_SSLCertChain_Get` の返り値が親 `webrtc_SSLCertChain` の所有する証明書への借用であり、(1) 呼び出し側が解放してはならないこと、(2) 親 `webrtc_SSLCertChain` が生存している間のみ有効であること、をヘッダ宣言 (`webrtc/src/webrtc_c/rtc_base/ssl_certificate.h`) および実装 (`webrtc/src/webrtc_c/rtc_base/ssl_certificate.cc`) のコメントに明記する。RULES.md の薄いラッパー原則 (`webrtc/RULES.md:5-6`) に沿うため、まずはこの方針を基本とする。
- 所有権を返す API への変更: 元の C++ `webrtc::SSLCertChain::Get` のシグネチャは参照を返す借用であるため、所有権を返す形に変えることは元の C++ API から逸脱する。クローンを返すなどの設計は薄いラッパー原則に反する可能性があるため、採用する場合は元 C++ API との対応関係と RULES.md との整合を確認し、必要なら許可を得ること。

## 完了条件

- `webrtc_SSLCertChain_Get` の返り値の寿命 (借用であること、解放禁止、親生存期間内のみ有効) がヘッダおよび実装のコメントで明確になっている、または所有権を返す API として再設計され寿命が明確になっている。
- 採用した方針が RULES.md の薄いラッパー原則と整合していることを確認している。
