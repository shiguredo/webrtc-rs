# whip.cpp の RTP センダ/コンテンツへの固定添字アクセスを境界チェックする

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-cpp-unchecked-sender-content-index

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

webrtc/src/whip.cpp:694（`contents()[1]`）

```cpp
              auto& content = offer->description()->contents()[1];
              auto media_desc = content.media_description();
```

webrtc/src/whip.cpp:866-876（`GetSenders()[1]`）

```cpp
                                    auto p =
                                        pc_->GetSenders()[1]->GetParameters();
                                    for (int i = 0; i < p.encodings.size();
                                         i++) {
                                      p.encodings[i].codec =
                                          video_init.send_encodings[i].codec;
                                      p.encodings[i].scalability_mode =
                                          video_init.send_encodings[i]
                                              .scalability_mode;
                                    }
                                    pc_->GetSenders()[1]->SetParameters(p);
```

`contents()` の要素数が 2 未満、または `GetSenders()` の要素数が 2 未満の場合、添字 `[1]` は
範囲外アクセスとなる。

## 設計方針

- `contents()` と `GetSenders()` の `size()` を確認し、添字 `[1]` を参照する前に要素数が十分で
  あることを保証する
- 要素数が不足している場合は範囲外アクセスを行わず、エラーとして扱う（ログ出力のうえ
  `SetState(State::kClosed)` 等で安全に中断する）

## 完了条件

- `contents()` や `GetSenders()` の要素数が不足していてもクラッシュしない
- 要素数不足時は範囲外アクセスを行わずエラー処理される
