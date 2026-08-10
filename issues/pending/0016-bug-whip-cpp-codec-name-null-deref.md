# whip.cpp で codec の未設定時参照を修正する

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-cpp-codec-name-null-deref
- Polished: 2026-08-05

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

`webrtc/src/whip.cpp` の `SignalingWhip::Connect` は、CreateOffer 成功コールバック内で
各 `send_encoding` のコーデックを処理する際、`send_encoding.codec` が optional 型である
にもかかわらず、値の有無を確認せずに `send_encoding.codec->name` でログ出力している。
`codec` が値を持たない場合、`operator->` は未定義動作となりクラッシュし得る。参照前に
`codec` の有無を確認するように修正する。

## 優先度根拠

未定義動作はプロセスのクラッシュに直結する堅牢性の欠陥である。`config_.video_source` と
`config_.send_encodings` が設定され、codec 未設定のエンコーディングが含まれる場合に
発生し得る。ライブラリ利用者は `send_encodings` を codec 無しで設定し得るため、
サンプル外の利用では発火し得る。優先度は High とする。
なお現行サンプルの `WhipClient::Run()` は全エンコーディングに codec を設定しているため、
現状では発火しない潜在バグである。

## 現状

`SignalingWhip::Connect` の CreateOffer 成功コールバック内で、rid と codec の対応付けを
行うループが `send_encoding.codec->name` を参照しているが、`send_encoding.codec` が
値を持つかどうかの確認はその直後の `if (send_encoding.codec && ...)` で初めて行われる。
値の有無の確認の前に `->name` を参照している:

```cpp
for (auto& send_encoding : video_init.send_encodings) {
  RTC_LOG(LS_WARNING)
      << "send_encoding: " << send_encoding.codec->name;  // 未定義動作
  for (auto& codec : media_desc->codecs()) {
    RTC_LOG(LS_WARNING) << "codec: " << codec.name;
    if (send_encoding.codec &&                            // 確認が遅すぎる
        webrtc::IsSameRtpCodec(codec, *send_encoding.codec)) {
      ...
    }
  }
}
```

同一関数内の別ループ（ビデオセットアップ側）では既に
`(send_encoding.codec ? send_encoding.codec->name : "none")` の形で codec 未設定を
許容しており、codec 未設定が想定された入力であることが分かる。

## 設計方針

- `send_encoding.codec->name` を参照する前に `send_encoding.codec` の有無を確認する
- `codec` が未設定の場合は、既存のビデオセットアップ側ループと同じ `"none"` を
  ログに出力する（三項演算子 `(send_encoding.codec ? send_encoding.codec->name : "none")`
  に揃える）
- 直後の `if (send_encoding.codec && ...)` 内の処理に影響は与えない

## 完了条件

- `send_encoding.codec` が値を持たない場合でもクラッシュしない
- `codec` 未設定時は安全な代替文字列（`"none"`）がログ出力される

## テスト戦略

`webrtc/` 配下の C++ サンプルには自動テスト基盤がないため、手動確認で行う。

- codec 未設定時: サンプルの `WhipClient::Run()` の `send_encodings` の codec 代入
  （例: `send_encodings[2].codec = av1_codec;`）を外したコードに編集してビルド・実行し、
  クラッシュせず `"none"` がログ出力されることを確認する
- 正常系: 全エンコーディングに codec を設定した状態でビルド・実行し、従来どおり
  `codec->name` がログ出力され、`rid_codec_map` が構築されること（挙動不変）を確認する

## 他 issue との関係

- `issues/0015` は同一の CreateOffer 成功コールバック内の `contents()[1]` /
  `GetSenders()[1]` の境界チェックを対象としており、実装順序によっては競合する
- `issues/0038` は `SignalingWhip::Connect` の巨大関数分割を対象としており、
  本 issue の変更箇所を含み得る
