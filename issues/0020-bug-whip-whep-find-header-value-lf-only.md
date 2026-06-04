# whip/whep の find_header_value を CRLF に対応させる

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-find-header-value-lf-only

## 目的

WHIP / WHEP の `find_header_value` は HTTP ヘッダから指定した名前のヘッダ値を抽出する関数だが、ヘッダ行の解釈に複数の堅牢性の問題がある。具体的には、(1) 折り返しヘッダ（obs-fold による継続行）を 1 つのヘッダ値として連結せず、継続行を独立した行として扱ってしまう、(2) 同名ヘッダが複数行ある場合に最初の 1 行しか返さない、(3) フィールド名と `:` の間に空白が入った行や、`:` を含まない行（ステータスライン等）の扱いが曖昧、という点である。これらを整理し、CRLF 区切りのヘッダを正しく扱えるようにして、ヘッダ値の抽出を堅牢化する。

## 優先度根拠

Medium。現状の実装でも一般的な `\r\n` 区切りの単純なヘッダは抽出できているが、折り返しヘッダや同名ヘッダ複数行といった現実のサーバが返し得るケースで値を取りこぼす。ICE サーバ情報を運ぶ `Link` ヘッダの抽出に直接影響するが、即時にクラッシュする類のバグではないため High ではなく Medium とする。

## 現状

`webrtc/src/whip.c` の `whip_find_header_value` は、行末を `\r\n` で探索し、ヘッダ名の比較に `strncasecmp` を使っている。`webrtc/src/whip.c:745` 付近:

```c
static char* whip_find_header_value(const char* headers, const char* key) {
  size_t key_len = strlen(key);
  const char* p = headers;
  while (p && *p) {
    const char* line_end = strstr(p, "\r\n");
    if (line_end == NULL) {
      line_end = p + strlen(p);
    }
    const char* colon = strchr(p, ':');
    if (colon != NULL && colon < line_end) {
      if ((size_t)(colon - p) == key_len && strncasecmp(p, key, key_len) == 0) {
        const char* value_start = colon + 1;
        while (value_start < line_end &&
               (*value_start == ' ' || *value_start == '\t')) {
          value_start++;
        }
        size_t value_len = (size_t)(line_end - value_start);
        char* value = (char*)malloc(value_len + 1);
        if (value != NULL) {
          memcpy(value, value_start, value_len);
          value[value_len] = '\0';
        }
        return value;
      }
    }
    if (*line_end == '\0') {
      break;
    }
    p = line_end + 2;
  }
  return NULL;
}
```

このコードは行末探索に `\r\n` を用い、`p = line_end + 2` で次の行頭へ進めている。したがって行区切りそのものは CRLF を前提としているが、以下の点が脆弱である。

- 折り返しヘッダ（次の行が先頭の空白 / タブで始まる継続行）を、直前のヘッダ値の続きとして連結していない。継続行は独立した行として走査され、`:` を含まなければ無視される。
- 最初に一致したヘッダ行で `return` しており、同名ヘッダが複数行ある場合に 2 行目以降を見ない。
- `colon < line_end` かつ「`:` の位置がちょうど `key_len` バイト目」という条件のため、フィールド名と `:` の間に空白が入った行は一致しない（厳密にはこれは正しい挙動だが、継続行や `:` を含む値行と組み合わさったときの取りこぼしと相まって、堅牢性を欠く）。

`webrtc/src/whep.c` の `whep_find_header_value` は完全に同一の実装で、`webrtc/src/whep.c:719` 付近に同じコードがある:

```c
static char* whep_find_header_value(const char* headers, const char* key) {
  size_t key_len = strlen(key);
  const char* p = headers;
  while (p && *p) {
    const char* line_end = strstr(p, "\r\n");
    ...
    p = line_end + 2;
  }
  return NULL;
}
```

## 設計方針

- 行区切りは CRLF（`\r\n`）として扱う。`\r` を欠く LF のみの区切りが来ても破綻しないよう、行末探索は `\r\n` を基準にしつつ、行末処理を見直す。
- ヘッダ名の比較は大文字小文字を無視する（現状どおり `strncasecmp` を使う）。フィールド名は `:` の直前までとし、フィールド名と `:` の間に空白を許容しない HTTP の規定に従う。
- 折り返しヘッダ（継続行が空白 / タブで始まる行）を、直前のヘッダ値に連結して 1 つの値として扱う。
- 用途に応じて、同名ヘッダが複数行ある場合の扱いを明確にする（少なくとも最初の 1 行で打ち切らず、必要なら連結できるようにする）。なお `Link` ヘッダの複数行対応は別 issue（whip/whep の Link ヘッダ解析を堅牢化する）で扱うため、本 issue は `find_header_value` 自体の行解釈の堅牢化に範囲を限定する。
- C 版（`whip.c` / `whep.c`）の双方に同じ修正を入れる。
- ログメッセージ・エラーメッセージは英語で記述する。

## 完了条件

- CRLF 区切りのヘッダから、指定した名前のヘッダ値を大文字小文字を無視して正しく抽出できる。
- 折り返しヘッダ（継続行）を含むヘッダ値を、継続行を連結した正しい値として抽出できる。
- `whip.c` と `whep.c` の双方で同じヘッダ解釈が行われる。
