# whip.cpp で codec の null チェック前参照を修正する

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-cpp-codec-name-null-deref

## 目的

whip.cpp は各 `send_encoding` のコーデックを処理する際、`send_encoding.codec` が未設定である
ケースを後続の条件分岐で潰しているにもかかわらず、その手前のログ出力で `send_encoding.codec->name`
を参照している。`codec` が未設定（値を持たない）の場合、null 参照となりクラッシュする。参照前に
`codec` の有無を確認するように修正する。

## 優先度根拠

null 参照はプロセスのクラッシュに直結する堅牢性の欠陥である。`send_encodings` の内容次第で発生し
得るため、優先度は High とする。

## 現状

ログ出力で `send_encoding.codec->name` を参照しているが、`send_encoding.codec` が値を持つかどうかの
確認はその直後のループ内の `if (send_encoding.codec && ...)` で初めて行われる。つまり null チェックの
前に `->name` を参照している。

webrtc/src/whip.cpp:696-708

```cpp
              for (auto& send_encoding : video_init.send_encodings) {
                RTC_LOG(LS_WARNING)
                    << "send_encoding: " << send_encoding.codec->name;
                for (auto& codec : media_desc->codecs()) {
                  RTC_LOG(LS_WARNING) << "codec: " << codec.name;
                  if (send_encoding.codec &&
                      webrtc::IsSameRtpCodec(codec, *send_encoding.codec)) {
                    RTC_LOG(LS_WARNING) << "rid=" << send_encoding.rid
                                        << " codec=" << codec.name
                                        << " payload_type=" << codec.id;
                    rid_codec_map[send_encoding.rid] = codec;
                  }
                }
              }
```

whip.cpp:698 の `send_encoding.codec->name` は、whip.cpp:701 の `if (send_encoding.codec && ...)`
よりも前に実行されるため、`codec` が未設定の場合 null 参照となる。

## 設計方針

- `send_encoding.codec->name` を参照する前に `send_encoding.codec` の有無
  （`has_value()` / null チェック）を確認する
- `codec` が未設定の場合は `->name` を参照しないようにし、ログ出力もそのケースを安全に扱う

## 完了条件

- `send_encoding.codec` が未設定でもクラッシュしない
- `codec` が未設定のケースで `->name` を参照しない
