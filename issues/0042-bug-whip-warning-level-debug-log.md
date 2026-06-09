# whip.c の通常動作ログを適切なログレベルにする

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.c` の映像コーデック選定処理で、通常動作の途中経過を示すデバッグ的なログが `WARNING` レベルで出力されている。
`WARNING` は注意を要する事象に使うべきであり、正常系の情報を `WARNING` で出すとログのノイズが増え、本当の警告が埋もれる。
適切なレベルへ修正する。

## 優先度根拠

ログの可読性・運用性の問題であり、機能そのものには影響しない。
そのため優先度は Low とする。

## 現状

`webrtc/src/whip.c:1396` では、コーデック名を列挙するだけの情報を `RTC_LOG_WARNING` で出力している。

```c
        struct std_string* codec_name = webrtc_RtpCodec_get_name(codec_base);
        RTC_LOG_WARNING("codec: %s", std_string_c_str(codec_name));
```

これは映像トランシーバー追加時にコーデック一覧を走査して出力しているだけの正常系のログであり、警告ではない。
なお同じ映像コーデック選定の処理内には、`webrtc/src/whip.c:1404` の各パラメータ出力、`webrtc/src/whip.c:1432` の `send_encoding` 出力、`webrtc/src/whip.c:1443` の `match codec` 出力、`webrtc/src/whip.c:1457` の `add codec` 出力など、同様に通常動作を `RTC_LOG_WARNING` で出している箇所が複数存在する。

## 設計方針

- 通常動作の途中経過を示すこれらのログを、`INFO` あるいは `DEBUG` 相当の適切なレベルへ変更する。
- どのレベルにするかは、出力内容が運用時に常時必要な情報か、開発時の詳細情報かを踏まえて統一的に判断する。
- 出力するメッセージ内容自体は変更しない。

## 完了条件

- 映像コーデック選定処理における通常動作のログが `WARNING` レベルで出力されなくなっている。
- 同一処理内の同種のログのレベルが一貫している。
