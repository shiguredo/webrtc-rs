# whip.cpp の on_response 二重呼び出しを修正する

- Priority: High
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.cpp` の `SendRequest` は、関数の終わりで必ず `on_response` を呼ぶための保険として `ScopeExit` ガード（`on_response_guard`）を関数冒頭に置いている。しかし一部の早期 return パスでは、その手前で `on_response(std::nullopt)` を明示的に呼んでから return しているため、明示呼び出しと `ScopeExit` の保険呼び出しの両方が走り、`on_response` コールバックが 2 回呼ばれる。コールバックが 2 回呼ばれると、受け手側で状態遷移やリソース解放が二重に行われ、未定義動作や状態不整合を招く。`on_response` が必ず 1 回だけ呼ばれるように修正する。

## 優先度根拠

High。コールバックの二重呼び出しは、受け手側の `on_response` 実装が「1 回だけ呼ばれる」前提で書かれている場合に、二重解放・二重状態遷移などの深刻な不整合を引き起こす。シグナリングの制御フローの正しさに直結し、クラッシュやメモリ破壊につながり得るため優先度を高くする。

## 現状

`webrtc/src/whip.cpp` の `SendRequest` は、冒頭で `on_response_guard` という `ScopeExit` を構築し、スコープ終了時に `on_response(response_body)` を呼ぶようにしている。`webrtc/src/whip.cpp:959` 付近:

```cpp
std::optional<std::string> response_body;
ScopeExit on_response_guard(
    [&on_response, &response_body]() { on_response(response_body); });
```

`ScopeExit` はデストラクタで関数オブジェクトを呼ぶ実装である。`webrtc/src/whip.cpp:945` 付近:

```cpp
struct ScopeExit {
  std::function<void()> f;
  ScopeExit(std::function<void()> f) : f(std::move(f)) {}
  ~ScopeExit() { f(); }
};
```

ところが、`getaddrinfo` 失敗時とソケット接続失敗時の早期 return パスでは、`on_response(std::nullopt)` を明示的に呼んでから return している。`webrtc/src/whip.cpp:970` 付近:

```cpp
int gai_err = getaddrinfo(host.c_str(), port.c_str(), &hints, &result);
if (gai_err != 0) {
  std::cerr << "getaddrinfo failed: " << gai_strerror(gai_err) << std::endl;
#ifdef _WIN32
  WSACleanup();
#endif
  on_response(std::nullopt);
  return;
}
```

および `webrtc/src/whip.cpp:994` 付近:

```cpp
if (IsInvalidSocket(sock)) {
  RTC_LOG(LS_ERROR) << "Failed to connect to " << host << ":" << port;
  on_response(std::nullopt);
  return;
}
```

これらの早期 return の時点で `on_response_guard` はまだ生存しているため、`return` によってスコープを抜けるとデストラクタが走り `on_response(response_body)`（このとき `response_body` は未設定なので `std::nullopt`）が再度呼ばれる。結果として `on_response` が 2 回呼ばれる。一方、その後の早期 return パス（`SSL_CTX_new` 失敗・`SSL_new` 失敗・`SSL_set_tlsext_host_name` 失敗・`SSL_connect` 失敗・`SSL_write` 失敗）では明示呼び出しが無く、`on_response_guard` による 1 回呼び出しのみとなっており、呼び出し回数が経路によって食い違っている。

なお `webrtc/src/whep.cpp` の `SendRequest` も同様の `ScopeExit` ＋明示呼び出しの構造になっているため、同じ二重呼び出しの可能性がある。

## 設計方針

方針 A（`ScopeExit` に一本化、明示呼び出しを削除）を採用する。理由:

- `getaddrinfo` 失敗時とソケット接続失敗時の 2 箇所の `on_response(std::nullopt)` を削除する
  だけで修正が完了する（2 行削除）
- SSL 系の早期 return パスと一貫性が取れる（これらは既に `ScopeExit` 任せ）
- `whip.cpp` を対象とする。`whep.cpp` にも同じバグがあるため合わせて修正する

## 完了条件

- `SendRequest` のすべての経路（正常終了・`getaddrinfo` 失敗・ソケット接続失敗・各 SSL 失敗）で `on_response` がちょうど 1 回だけ呼ばれる。
- 早期 return パスでの明示呼び出しと `ScopeExit` の保険呼び出しが二重に走らない。
- `whip.cpp` と `whep.cpp` の双方で同じ修正が施されている。
