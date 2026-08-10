# whip/whep で Content-Length と chunked エンコーディングを解釈する

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-whep-content-length-chunked
- Polished: 2026-08-05

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

WHIP / WHEP のレスポンス受信処理が `Content-Length` も `Transfer-Encoding: chunked` も
解釈しておらず、ソケットが読み切れるまで受信を続ける実装になっている。これはサーバが
`Connection: close` で接続を閉じることに暗黙に依存しており、close 指示を無視するサーバや
プロキシ経由では `SSL_read` がブロックし続ける危険がある。`Content-Length` を尊重して
ボディ長を確定し、`chunked` を解釈してデチャンクすることで、HTTP メッセージ境界を正しく
扱えるようにする。

## 優先度根拠

chunked でボディを返す構成では、デチャンクしない場合にチャンクサイズ行や終端チャンクが
ボディに混入し、SDP 解析の失敗に直結する。受信処理の正しさに直結するため優先度は High と
する。

## 現状

受信処理は `whip_SendRequest`（`webrtc/src/whip.c`）/ `whep_SendRequest`
（`webrtc/src/whep.c`）/ `SignalingWhip::SendRequest`（`webrtc/src/whip.cpp`）/
`SignalingWhep::SendRequest`（`webrtc/src/whep.cpp`）内のループで `SSL_read` が 0 以下を
返すまで読み続ける。`Content-Length` の読み取り・`Transfer-Encoding: chunked` の判定・
デチャンク処理は一切ない。4 ファイルとも同一構造。

## 設計方針

- ヘッダをパースした段階で `Content-Length` を読み取り、値が存在する場合はその長さ分だけ
  ボディを受信したら受信を完了とする
- `Transfer-Encoding: chunked` が指定されている場合は、ボディをチャンク単位でデチャンク
  する。各チャンクは「16 進のチャンクサイズ行（`\r\n` 終端）」＋「チャンクデータ」＋
  「`\r\n`」で構成され、サイズ 0 のチャンクで終端となる。デチャンク後のバイト列を最終的な
  ボディとする
- `Content-Length` と `Transfer-Encoding` の両方が指定されている場合は、
  `Transfer-Encoding: chunked` を優先する（RFC 9112 §6.3。両者併存は
  request smuggling / response splitting の試みの可能性があるためエラー扱いとしてもよいが、
  少なくとも chunked を優先し、Content-Length は無視する）
- `Content-Length` と `Transfer-Encoding` のどちらも無い場合のみ、従来どおり接続クローズ
  までの読み切りにフォールバックする
- ヘッダ終端（`\r\n\r\n`）を受信し終えるまではヘッダ用に読み進め、終端以降をボディとして
  扱う。ヘッダとボディが同じ `SSL_read` 呼び出しにまたがって到着し得る点に注意する。
  また、チャンクサイズ行・チャンクデータ・CRLF が任意の境界で分割到着し得ることを前提に、
  インクリメンタルな受信バッファを持つ
- `Transfer-Encoding` は `chunked` 単独のみ受理する。case-insensitive に比較する
  （`CHUNKED` を許容する）。`gzip, chunked` のような複合エンコーディングは、
  chunked 以外の転送コーディングが含まれるため一律エラー扱いとする（gzip の伸張には
  対応しない。RFC 9112 §6.3 では transfer-coding のリストは最後のエンコーディングが
  ボディを framing するが、本 issue では複合エンコーディング自体を受理しない）
- チャンク拡張（サイズ行の `;` 以降）と終端チャンク後のトレイラセクションは読み飛ばす
- `Content-Length` の値は 1*DIGIT（RFC 9112 §6.3）のみ受理する。非数値・負値・`+` や
  `0x` プレフィックス・値の異なる複数 `Content-Length` ヘッダは、request smuggling /
  response splitting の可能性があるためエラー扱いとする（同一値の複数 `Content-Length`
  ヘッダは許容し、単一値として扱ってよい）。チャンクサイズ行が 16 進として解釈できない
  場合、`+` / `0x` プレフィックスを含む場合、または整数変換のオーバーフロー（ERANGE）が
  発生する場合は、不正レスポンスとしてエラー扱いとする（RFC 9112 §7.1）
- ヘッダ終端（`\r\n\r\n`）に到達しないまま接続がクローズした場合は、不完全なヘッダとして
  エラー扱いとする
- 巨大な `Content-Length` 宣言を受けた場合の受信上限を考慮する（宣言長どおり受信すると
  メモリを過大に消費し続けるため、上限を設けてエラー扱いとする）。デチャンク後の総量にも
  同様の上限を設ける。上限値は SDP ボディとして想定し得るサイズ（目安: 数 MB 程度）とする
- ボディが `Content-Length` の宣言値に満たないまま接続がクローズした場合は、不完全な
  メッセージとしてエラー扱いとする（RFC 9112 §6.3。欠損レスポンスを正常扱いしない）
- C 版（`whip.c` / `whep.c`）と C++ 版（`whip.cpp` / `whep.cpp`）の双方に対応する

### デチャンク後の受け渡し

デチャンク後のボディを下流の `whip_OnSendRequestResponse` / `whep_OnSendRequestResponse` /
各 `SendRequest` コールバックへ渡す際は、ヘッダは受信原文のまま、ボディだけをデチャンク後
のバイト列に置き換えた形で `on_response` に渡す。既存の処理は `\r\n\r\n` 以降をボディと
して切り出すため、この形式なら下流のパースを変更せずに済む。

### C++ 版のヘッダ検出

C 版（`whip.c` / `whep.c`）は既存の `whip_find_header_value` / `whep_find_header_value`
を再利用して `Content-Length` / `Transfer-Encoding` を読み取る。ただし値の異なる複数の
`Content-Length` ヘッダを検出するには最初の 1 件のみを返すこの関数では足りないため、
同名ヘッダの全走査（または find-all 版関数の追加）が必要になる。C++ 版（`whip.cpp` /
`whep.cpp`）は `absl::StrSplit` と正規表現でヘッダをパースしており、`find_header_value`
相当の関数を持たない。既存実装は `std::map` に同名ヘッダを上書きするため、重複
`Content-Length` の検出には C 版と同様にヘッダ行の全走査が必要になる。

### 依存関係

- `issues/0019`（SSL I/O 戻り値処理）が本修正の前提となる。`SSL_read` ループ内で
  `Content-Length` による終了判定を入れる前に、エラー判別を正しく行う必要があるため、
  0019 を先に実装すること。ただし 0019 の「`SSL_ERROR_ZERO_RETURN` は正常な TLS クローズ
  として受信完了扱い」は、`Content-Length` が宣言されている場合はそのまま適用できない。
  宣言長に満たないまま ZERO_RETURN が発生した場合はボディ欠損としてエラー扱いにする
  （上記の不完全メッセージの扱いと整合）
- `issues/0020`（`whip_find_header_value` の堅牢化）は本 issue が使うヘッダ探索関数を
  対象とする。本 issue の `Content-Length` / `Transfer-Encoding` 読み取りでは既存の
  `whip_find_header_value` / `whep_find_header_value` を再利用することを想定しており、
  実装順序によっては競合する（0020 の対象は C 版のみ。C++ 版は競合しない）
- `issues/0017`（HTTP ステータスコード検証）も同じレスポンス処理領域を対象とする。
  実装順序によっては競合する
- `issues/0023`（C++ 版 `SendRequest` の `on_response` 二重呼び出し）は本 issue が改修する
  同じ `SignalingWhip::SendRequest` / `SignalingWhep::SendRequest` を対象とする。
  実装順序によっては競合する
- `issues/0021`（Link ヘッダ解析の堅牢化）は同じレスポンス処理領域を対象とする。
  実装順序によっては競合する

## 完了条件

- `Content-Length` が指定されたレスポンスを、指定された長さちょうどで受信できる
- `Transfer-Encoding: chunked` のレスポンスを、デチャンクして正しく受信できる
- `Content-Length` と `Transfer-Encoding` の両方が指定されたレスポンスは、chunked を優先
  して受信できる
- `CHUNKED` のような大文字の Transfer-Encoding も case-insensitive に受理される
- chunk 拡張とトレイラセクションを含むレスポンスも読み飛ばして受信できる
- どちらのヘッダも無いレスポンスは従来どおり接続クローズまで読み切りで受信できる
- `Content-Length` の宣言値に満たないまま接続がクローズしたレスポンスはエラーとして扱う
- 不正なチャンク（16 進でないサイズ行・`+` / `0x` プレフィックス・CRLF 欠落・終端チャンク
  未到達・ERANGE オーバーフロー）はエラーとして扱う
- 不正な `Content-Length`（非数値・負値・`+` / `0x` プレフィックス・値の異なる複数ヘッダ）
  はエラーとして扱う
- 複合エンコーディング（`gzip, chunked` 等）はエラーとして扱う
- 巨大な `Content-Length` 宣言やデチャンク後の総量が受信上限を超える場合はエラーとして
  扱う
- ヘッダ終端に到達しないまま接続がクローズしたレスポンスはエラーとして扱う
- C 版・C++ 版の 4 ファイルで同じ受信処理が行われる（C++ 版は `absl::StrSplit` による
  ヘッダパースで対応する）

## テスト戦略

`webrtc/` 配下の C/C++ サンプルには自動テスト基盤がないため、手動確認で行う。
`Content-Length` あり・`chunked`・両方指定・どちらも無し・宣言長未満でのクローズ・
ヘッダ終端未到達でのクローズ・大文字 `CHUNKED`・chunk 拡張・トレイラ付き・
複合エンコーディング・不正な `Content-Length`（非数値・複数値不一致）・不正なチャンク・
巨大な `Content-Length`・デチャンク後の総量が受信上限を超えるケース・1 バイトずつの
分割到着の各レスポンスを返す試験用の実 HTTP
サーバを使い、正しく受信（またはエラー扱い）されることを確認する。試験用サーバは
実サーバとして位置づけ、レスポンスの差し替えを目的としたモック・スタブは使わない
（AGENTS.md の「モックやスタブは絶対に利用しないこと」に従う）。
