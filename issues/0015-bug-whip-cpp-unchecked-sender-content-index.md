# whip.cpp の RTP センダ/コンテンツへの固定添字アクセスを境界チェックする

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Polished: 2026-06-06

## 目的

whip.cpp はネゴシエーション結果の RTP センダ列やメディアコンテンツ列に対し、要素数を確認せずに
固定インデックス `[1]` でアクセスしている。センダやコンテンツが想定どおり 2 本以上存在しない
ネゴシエーション結果になった場合、範囲外アクセスとなり未定義動作・クラッシュを引き起こす。要素数を
確認してから参照するように修正し、堅牢性を確保する。

## 優先度根拠

範囲外アクセスは未定義動作でありクラッシュに直結する。相手の応答内容に依存して発生し得るため、
攻撃や異常応答に対する堅牢性の欠陥として優先度は High とする。

## 現状

whip.cpp は `contents()` と `GetSenders()` の 2 番目の要素（添字 `[1]`）を、要素数を確認せずに
参照している。

`webrtc/src/whip.cpp:694` — `contents()[1]`:
```cpp
auto& content = offer->description()->contents()[1];
auto media_desc = content.media_description();
```

`webrtc/src/whip.cpp:866-876` — `GetSenders()[1]`:
```cpp
auto p = pc_->GetSenders()[1]->GetParameters();
for (int i = 0; i < p.encodings.size(); i++) {
  p.encodings[i].codec =
      video_init.send_encodings[i].codec;
  p.encodings[i].scalability_mode =
      video_init.send_encodings[i]
          .scalability_mode;
}
pc_->GetSenders()[1]->SetParameters(p);
```

`webrtc/src/whip.cpp:710` — `mutable_streams()[0]`（issue 記載漏れを追記）:
```cpp
auto& track = media_desc->mutable_streams()[0];
```

L870-874 では、`p.encodings` と `video_init.send_encodings` が独立したベクターであり、
`send_encodings` が `encodings` より小さい場合に `send_encodings[i]` の範囲外アクセスが
発生する。この問題も併せて修正する。

## 設計方針

- `contents()` と `GetSenders()` の `size()` を確認し、添字 `[1]` を参照する前に
  要素数が `>= 2` であることを保証する
- `mutable_streams()` の `size()` を確認し、添字 `[0]` を参照する前に
  要素数が `>= 1` であることを保証する
- L870-874 のループでは `send_encodings.size() >= p.encodings.size()` であることを
  事前に確認する。不足時は範囲外アクセスを行わずエラーとする
- 要素数が不足している場合は `RTC_LOG(LS_ERROR)` でログ出力し、
  `SetState(State::kClosed)` で安全に中断する

## 完了条件

- `contents()` や `GetSenders()` の要素数が 2 未満でもクラッシュしない
- `mutable_streams()` の要素数が 1 未満でもクラッシュしない
- `send_encodings` の要素数が `encodings` より小さくてもクラッシュしない
- 要素数不足時は範囲外アクセスを行わずエラー処理される
