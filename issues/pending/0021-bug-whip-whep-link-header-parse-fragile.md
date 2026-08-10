# whip/whep の Link ヘッダ解析を堅牢化する

- Priority: Medium
- Polished: 2026-08-05
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-whep-link-header-parse-fragile

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

WHIP / WHEP では ICE サーバ情報（TURN サーバの URL・ユーザー名・クレデンシャル）を HTTP の
`Link` ヘッダで運ぶ。現在の実装は以下の問題を抱えている:

1. `Link` ヘッダが複数行存在する場合に 1 行しか取得できない（C 版は最初の一致で return、
   C++ 版は `std::map` の上書きで最後の 1 行のみ残る）
2. `rel` パラメータを確認せず、すべてのエントリを無条件に ICE サーバ情報として取り込んで
   いる（WHEP のレスポンスには `rel="ice-server"` 以外のエントリが混在し得る）
3. カンマ区切りによるエントリ分割が、URL クエリ内や quoted-string 内のカンマで誤分割される
   可能性がある

RFC 8288 に従って Link ヘッダを正しく解析し、ICE サーバ情報の取得を堅牢化する。

## 優先度根拠

Medium。`Link` ヘッダが 1 行にカンマ区切りで複数エントリをまとめている構成では現状でも
動くが、複数行の `Link` ヘッダを返すサーバ構成では C 版は 1 行目しか取得できず
（C++ 版は最後の 1 行のみ残る）、一部の TURN サーバ情報が欠落する。ICE 接続の確立性に
影響するが、即時クラッシュではないため Medium とする。

## 現状

**C 版**: `whip_OnSendRequestResponse`（`webrtc/src/whip.c`）/ `whep_OnSendRequestResponse`
（`webrtc/src/whep.c`）内で `whip_find_header_value` / `whep_find_header_value` により
最初の `Link` 行のみ取得し、`strtok_r(link_header, ",", ...)` でカンマ分割する。
`rel` パラメータの確認なし。username / credential が無いエントリは URL のみ追加する
寛容な挙動。

**C++ 版**: `SignalingWhip::Connect` / `SignalingWhep::Connect` の SendRequest コールバック
内で `headers["link"]` により取得するが、`std::map` への代入（`headers[key] = value`）で
同一キーは上書きされる。`absl::StrSplit(link, ",")` でカンマ分割する。`rel` パラメータの
確認なし。username / credential が無いエントリがあると `Failed to match ...` で
即 return（接続失敗扱い）する。

4 ファイルとも、単一の `IceServer` オブジェクトに全 URL を追加し、`username` / `password`
はループ内で上書きされるため、エントリごとに異なるクレデンシャルがある場合は最後の値のみ
有効になる。

## 設計方針

### 複数 Link ヘッダ行の取得

- `issues/0020`（find_header_value の堅牢化）で同名ヘッダの複数行取得に対応済みであること
  を前提とする。本 issue では C 版に複数行を連結した `Link` ヘッダ値が得られる前提で
  進める
- C++ 版は `std::map<std::string, std::string>` を `std::multimap` または同等の複数値保持
  構造に変更し、すべての `Link` ヘッダ行の値を `equal_range` で取得する。取得した複数行の
  値は `, ` 区切りで連結して 1 回のエントリ分割パースに渡す（RFC 8288 Appendix B.1 は
  行ごとの個別パースだが、本 issue では連結して 1 回でパースする方式を採る）

### rel パラメータの検証

- 各エントリの `rel` パラメータを確認し、`ice-server` のエントリのみを処理対象とする
- rel の一致判定は次の規則に従う（RFC 8288 §2.1.1 / §3 / §3.3 / Appendix B.2）:
  - token 形式（`rel=ice-server`）と quoted-string 形式（`rel="ice-server"`）の両方を
    受理する（RFC 8288 §3 の MUST）
  - 大文字小文字を無視して比較する（registered relation type の比較は
    case-insensitive）
  - rel 値はスペース区切りの複数 relation type リスト（`rel="ice-server alternate"`）を
    持ち得るため、SP / HTAB 分割したリストに `ice-server` が含まれるかで判定する
    （RFC 8288 Appendix B.2 step 10 の RWS 分割に従う）
  - rel パラメータが 2 回以上出現する場合は最初の値を採用し、2 回目以降は無視する
    （RFC 8288 §3.3）
- rel パラメータが無いエントリ、rel に `ice-server` を含まないエントリ（例:
  `rel="alternate"`）は無視する

### エントリ分割の堅牢化

- 単純なカンマ分割ではなく、`<` / `>` と quoted-string を考慮したエントリ境界の判定を行う
  （RFC 8288 §3 の link-value 構文に従う。quoted-string 内のカンマは区切り文字として
  扱わない）
- エントリ前後の OWS はトリムし、空エントリは無視する（RFC 8288 Appendix B.2 の
  「Consume any leading OWS」に基づく。ただし空エントリの「無視して継続」は RFC の
  アルゴリズム（step 2.2 で `"<"` 以外なら解析停止）より寛容な独自方針であり、
  その旨を注記する）
- quoted-string 内のバックスラッシュエスケープ（quoted-pair、RFC 7230 §3.2.6）を考慮する。
  エスケープ解除（unquote）は rel の一致判定前と username / credential の値の両方に
  適用する（RFC 8288 Appendix B.4）

### username / credential の任意性

- username / credential は TURN サーバにのみ存在する属性であり、STUN サーバのエントリには
  無い（RFC 9725 §4.6）。欠落時は接続失敗にせず、URL は追加して username / credential は
  空のまま処理を継続する（C 版の寛容な挙動に揃える。C++ 版の `Failed to match ...` による
  即 return は廃止する）
- username / credential の形式は現行どおり quoted-string のみを受理する（token 形式の
  対応は行わない）。RFC 8288 §3 は link-param の token / quoted-string 両形式の受理を
  MUST と定めるが、実サーバは username / credential を quoted-string で送信するため
  現行方式を維持する（MUST からの逸脱を認識した上での判断）
- username / credential が 1 エントリ内に 2 回以上出現する場合は、最初の値を採用し
  2 回目以降は無視する（rel と同じ規則）

### IceServer オブジェクトの生成

- 各 `ice-server` エントリに対して個別の `IceServer` オブジェクトを生成する。
  エントリごとに username / credential が異なる可能性があるため、
  既存の全 URL を 1 つの IceServer にまとめる方式から変更する
- エントリ単位のエラー処理は「無視（skip）」を基本とする。構文不正なエントリが混在しても
  他のエントリの処理を継続する
- `ice-server` エントリが 1 件も無い場合は、`SetConfiguration` を呼ばずにエラーとして
  扱う（RFC 9725 §4.6 はサーバ側の ICE サーバ情報提供を MAY としているが、情報なしで
  `kRelay` 構成のまま接続を続けても確立できないため、エラーで明示する）
- C 版のパーサでメモリ確保に失敗した場合は、そのエントリを skip して次のエントリへ進む
  （英語のエラーログを出力する）

### 依存関係

- `issues/0020`（find_header_value の堅牢化）は本 issue の前提であり、0020 より先に
  実装すること（0020 側でも相互に明記済み）
- `issues/0022`（C++ 版の regex 例外対策）は本 issue の前提作業として、
  堅牢なパーサへの置換を先に済ませることを想定している。実装順序によっては競合する。
  0022 の完了条件「解析対象の入力サイズに上限が設けられている」は本 issue の新パーサにも
  適用する（0021 が先に実装された場合も新パーサにサイズ上限を含めること）
- `issues/0052`（whip/whep の C 版重複排除）は `whip_OnSendRequestResponse` /
  `whep_OnSendRequestResponse` の Link ヘッダパース部分を含めて共通化対象としている。
  0052 が先に実装されると本 issue の C 版修正対象が変わる。実装順序によっては競合する
  （本 issue を先に実装することを推奨する）
- `issues/0037`（credential ログ対策）は C++ 版の Link パースループ内の
  `Server: url=... username=... password=...` ログを対象とする。本 issue の新パーサの
  ログは password を含めない（0037 の完了条件を満たした状態を維持する）

## テスト戦略

- Link ヘッダの構文パースを切り出して単体テスト可能にする（C 版は `main` 内テスト、
  C++ 版は同等の検証コードで）
- テストケース: 単一エントリ（quoted 形式の rel 含む）、カンマ区切り複数エントリ、
  複数 Link 行、rel 混在、token 形式の rel（`rel=ice-server`）、rel の大文字小文字混在、
  rel のスペース区切り複数値、rel パラメータ無しのエントリ、rel が 2 回出現するエントリ、
  username が 2 回出現するエントリ、ice-server エントリが 1 件も無い場合、
  構文不正エントリ（`<` が閉じない等）の混在、URL クエリ内カンマ、quoted-string 内カンマ、
  quoted-string 内エスケープ、先頭 OWS / 連続カンマによる空エントリ、
  username / credential 欠落エントリ（STUN のみ）、エントリごとに異なる
  username / credential の分離
- 完了条件の結合検証は既存の WHIP/WHEP ビルド実行で手動確認する。試験用サーバは
  実サーバとして位置づけ、モック・スタブは使わない（AGENTS.md の「モックやスタブは
  絶対に利用しないこと」に従う）

## 完了条件

- `Link` ヘッダが複数行に分かれているレスポンスから、すべての ICE サーバ情報を取得できる
- `rel` が `ice-server` のエントリのみが ICE サーバ情報として処理される（token 形式・
  大文字小文字混在・スペース区切り複数値も受理）
- URL クエリや quoted-string にカンマを含むエントリが正しく分割される
- username / credential を持たない STUN エントリも正しく処理される
- 構文不正なエントリが混在しても、他のエントリの処理を継続して取得できる
- エントリごとに独立した `IceServer` オブジェクトが生成される
- C 版・C++ 版の 4 ファイルで同様の解析が行われる
