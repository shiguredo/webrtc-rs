# whip.cpp の RTP センダ/コンテンツへの境界チェックされていない添字アクセスを修正する

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-cpp-unchecked-sender-content-index
- Polished: 2026-08-05

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

`webrtc/src/whip.cpp` の `SignalingWhip::Connect` は、ローカルに追加したトランシーバーから
生成される RTP センダ列やメディアコンテンツ列に対し、要素数を確認せずに固定インデックス
`[1]` でアクセスしている。要素数が想定より少ない場合、範囲外アクセスとなり未定義動作・
クラッシュを引き起こす。要素数を確認してから参照するように修正し、堅牢性を確保する。

## 優先度根拠

範囲外アクセスは未定義動作でありクラッシュに直結する。下記の現状のとおり、ローカル設定
（`video_source` 未指定・`send_encodings` 未指定）で決定的に発生し得るため、優先度は
High とする。

## 現状

whip.cpp には境界チェックされていない添字アクセスが次の 4 箇所ある。

1. `offer->description()->contents()[1]` — `SignalingWhip::Connect` の CreateOffer
   成功コールバック内。`contents()` はローカルで追加したトランシーバー列であり、
   `config_.video_source` が NULL の場合は audio 1 本だけになるため `[1]` が範囲外になる。
   ```cpp
   auto& content = offer->description()->contents()[1];
   auto media_desc = content.media_description();
   ```

2. `media_desc->mutable_streams()[0]` — 上記と同じコールバック内。
   ```cpp
   auto& track = media_desc->mutable_streams()[0];
   ```

3. `pc_->GetSenders()[1]->GetParameters()` / `SetParameters(p)` —
   SetRemoteDescription 成功コールバック内。`GetSenders()` はローカルのセンダ列であり、
   `video_source` が NULL の場合は audio センダ 1 本だけになるため `[1]` が範囲外になる。
   実行順序上は先に claim 1 の `contents()[1]` でクラッシュするため、本箇所は
   claim 1 の修正後も防御として境界チェックが必要である。
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

4. 上記ループ内の `video_init.send_encodings[i]` — `p.encodings` と
   `video_init.send_encodings` は独立したベクターであり、`send_encodings` が
   `p.encodings` より小さい場合（`config_.send_encodings` 未指定時は空になる）に
   `send_encodings[i]` の範囲外アクセスが発生する。

## 設計方針

- `contents()` と `GetSenders()` の `size()` を確認し、添字 `[1]` を参照する前に
  要素数が `>= 2` であることを保証する
- `mutable_streams()` の `size()` を確認し、添字 `[0]` を参照する前に
  要素数が `>= 1` であることを保証する
- `GetSenders()` は呼び出しのたびに新しい `std::vector` を返すため、
  サイズ確認と参照を同じインスタンスで行うよう、ローカル変数に 1 回取得して使い回す
- ループでは `send_encodings.size() >= p.encodings.size()` であることを事前に確認する。
  不足時は範囲外アクセスを行わず、ループをスキップする（codec / scalability_mode の
  上書きは行わない）。`config_.send_encodings` 未指定時は `video_init.send_encodings` が
  空になり、`p.encodings` にはデフォルトの 1 エンコーディングが入るため、このケースを
  エラーにして接続を断つと video_source 指定のみの正当な設定が失敗する。ループを
  スキップすればデフォルトのエンコーディングがそのまま使われる
- `contents()` / `GetSenders()` / `mutable_streams()` の要素数が不足している場合は
  `RTC_LOG(LS_ERROR)` で英語のログを出力し、`SetState(State::kClosed)` で安全に中断する
  （既存のエラーパス `RTC_LOG(LS_ERROR)` + `SetState(State::kClosed)` と同じパターン。
  このエラー中断は固定添字アクセスの 3 箇所に適用され、`send_encodings` ループは
  スキップで対応する）

## 完了条件

- `contents()` や `GetSenders()` の要素数が 2 未満でもクラッシュしない
- `mutable_streams()` の要素数が 1 未満でもクラッシュしない
- `send_encodings` の要素数が `encodings` より小さくてもクラッシュしない
- `contents()` / `GetSenders()` / `mutable_streams()` の要素数不足時は範囲外アクセスを
  行わずエラー中断される
- `config_.send_encodings` 未指定時はループをスキップし、デフォルトのエンコーディングで
  接続が継続する
- 本 issue では audio のみの送信には対応しない（`video_source` 未指定時はエラー中断と
  なる。audio のみの WHIP 送信対応は本 issue のスコープ外とする）

## テスト戦略

`webrtc/` 配下の C++ サンプルには自動テスト基盤がないため、手動確認で行う。
サンプルの `WhipClient::Run()` は `video_source` と `send_encodings` を常に設定するため、
テスト時は該当設定を外したコードに編集して確認する。

- `video_source` を設定しない場合: `contents()[1]` / `GetSenders()[1]` の要素数不足で
  クラッシュせず、`RTC_LOG(LS_ERROR)` と `SetState(State::kClosed)` によるエラー中断が
  起きることを確認する
- `video_source` を設定し `send_encodings` を設定しない場合: ループがスキップされ、
  デフォルトのエンコーディングで接続が継続することを確認する
- 正常設定（`video_source` と `send_encodings` を設定）でビルド・実行し、
  従来どおりシグナリングが進むこと（挙動不変）を確認する

## 他 issue との関係

- `issues/0016` は同一の CreateOffer 成功コールバック内の `codec->name` 参照を対象として
  おり、実装順序によっては競合する
- `issues/0033` は `pc_->GetSenders()` の `Disconnect` による `pc_ = nullptr` との同期を
  対象としており、本 issue と同じ `GetSenders()` の行を書き換えるため競合する
- `issues/0048` は本 issue と同じ `p.encodings` ループの `int` と `size()` の符号混在
  比較を対象としており、同じ行を書き換えるため競合する
- `issues/0038` は `SignalingWhip::Connect` の巨大関数分割を対象としており、
  本 issue の変更箇所を含み得る
