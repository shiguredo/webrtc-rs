# URLParts_Parse の strndup 戻り値を検査して NULL 参照を防ぐ

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-urlparts-strndup-null-deref

## 目的

`URLParts_Parse`（whip.c / whep.c）は URL を分解する際に `strndup` を複数回呼ぶが、その戻り値が
NULL になる失敗を一切検査していない。`strndup` が NULL を返した直後にその結果を参照・`strlen` する
ため、メモリ確保失敗時に NULL 参照でクラッシュする。これを防ぐため、各 `strndup` の戻り値を検査して
失敗時にはパース失敗として安全に返すようにする。

## 優先度根拠

NULL 参照はプロセスのクラッシュに直結する。メモリ確保失敗時や境界条件で発生し得る堅牢性の欠陥で
あり、シグナリング接続の入口で起きるため、優先度は High とする。

## 現状

`URLParts_Parse` 内の各 `strndup` / `strdup` は戻り値を検査せず、後続処理でそのポインタを参照する。
例えば `strndup` で確保した `user_pass_host_port` を `strchr` に渡し、`host_port` を `strchr` に
渡している。確保が失敗して NULL が返ると、その時点で NULL 参照となる。

webrtc/src/whip.c:227-274

```c
static int URLParts_Parse(const char* url, struct URLParts* parts) {
  URLParts_clear(parts);
  if (url == NULL) {
    return 0;
  }
  const char* p = strstr(url, "://");
  if (p == NULL) {
    return 0;
  }
  size_t scheme_len = (size_t)(p - url);
  parts->scheme = strndup(url, scheme_len);

  p += 3;  // skip ://
  const char* slash = strchr(p, '/');
  size_t uphp_len = 0;
  if (slash == NULL) {
    uphp_len = strlen(p);
    parts->path_query_fragment = strdup("");
  } else {
    uphp_len = (size_t)(slash - p);
    parts->path_query_fragment = strdup(slash);
  }
  char* user_pass_host_port = strndup(p, uphp_len);

  char* at = strchr(user_pass_host_port, '@');
  char* host_port = NULL;
  if (at == NULL) {
    parts->user_pass = strdup("");
    host_port = user_pass_host_port;
  } else {
    parts->user_pass =
        strndup(user_pass_host_port, (size_t)(at - user_pass_host_port));
    host_port = strdup(at + 1);
    free(user_pass_host_port);
  }

  char* colon = strchr(host_port, ':');
  if (colon == NULL) {
    parts->host = host_port;
    parts->port = strdup("");
  } else {
    parts->host = strndup(host_port, (size_t)(colon - host_port));
    parts->port = strdup(colon + 1);
    free(host_port);
  }

  return 1;
}
```

`user_pass_host_port` の確保失敗時は直後の `strchr(user_pass_host_port, '@')`（whip.c:251）で
NULL 参照となる。`host_port` も同様に `strchr(host_port, ':')`（whip.c:263）で NULL 参照となる。

webrtc/src/whep.c:279-326 も同一の実装で、同じ箇所に同じ欠陥がある（`strndup` は whep.c:289,
301, 310, 320、`strchr` への引き渡しは whep.c:303, 315）。

## 設計方針

- 各 `strndup` / `strdup` の直後で戻り値が NULL でないか検査する
- いずれかが NULL の場合は、それまでに確保した領域を `URLParts_clear` で解放し、パース失敗
  （`return 0`）を返す
- 既存の所有権の流れ（`user_pass_host_port` / `host_port` を `parts` のメンバへ移す、あるいは
  途中で `free` する経路）を崩さないように解放漏れ・二重解放を避ける
- whip.c / whep.c の両方に適用する

## 完了条件

- `strndup` / `strdup` が失敗（NULL 返却）してもクラッシュせず、`URLParts_Parse` がパース失敗を
  返す
- 失敗経路でメモリリークや二重解放が発生しない
- whip.c / whep.c の両方で対応される
