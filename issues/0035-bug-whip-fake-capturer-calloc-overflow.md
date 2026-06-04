# FakeVideoCapturer の calloc の整数オーバーフローと戻り値未チェックを修正する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-fake-capturer-calloc-overflow

## 目的

`FakeVideoCapturer_Create` の画像バッファ確保における整数オーバーフローと、`calloc` の戻り値未チェックを修正する。現状は `width * height` を `int` の乗算で計算しており、大きな値でオーバーフローし得るうえ、`calloc` が OOM で `NULL` を返した場合も検査せずに使用するため、クラッシュの原因になる。これを修正する。

## 優先度根拠

Medium とする。`FakeVideoCapturer` はサンプル用のフェイク映像ソースであり、通常はデフォルト解像度 (640x480) で使われるため、現実的なクラッシュ頻度は高くない。しかし `width` / `height` を外部から指定できる構造であり、巨大な値を渡されると `int` 乗算のオーバーフローで過小なサイズが確保され、その後のバッファ書き込みでヒープ破壊に至る。また OOM 時の `NULL` 参照でもクラッシュする。メモリ安全性に関わる欠陥であり、防御的に直すべきだが、デフォルト経路では発現しにくいため Medium とする。

## 現状

`FakeVideoCapturerConfig` の `width` / `height` は `int` である (`webrtc/src/whip.c:130-132`)。

```c
struct FakeVideoCapturerConfig {
  int width;
  int height;
  ...
};
```

`FakeVideoCapturer_Create` では、まず構造体本体を `calloc` で確保しているが、戻り値を検査せずに直後で `p->ref = ...` と参照している (`webrtc/src/whip.c:173-175`)。

```c
  struct FakeVideoCapturer* p =
      (struct FakeVideoCapturer*)calloc(1, sizeof(struct FakeVideoCapturer));
  p->ref = webrtc_RefCountInterface_Create(FakeVideoCapturer_delete, p);
```

画像バッファの確保では、`p->config.width * p->config.height` という `int` 同士の乗算を要素数として `calloc` に渡している (`webrtc/src/whip.c:192-193`)。

```c
  p->image =
      (uint32_t*)calloc(p->config.width * p->config.height, sizeof(uint32_t));
```

要素数と要素サイズは分けて渡されているものの、`width * height` 自体が `int` の範囲で計算されるため、両者が大きいとこの乗算が `int` でオーバーフローし、`calloc` には意図より小さい (あるいは負から変換された) 要素数が渡され得る。さらに `p->image` の `NULL` 検査がないため、OOM 時にもそのまま使用される。確保した `p->image` は `FakeVideoCapturer_delete` で `free` される (`webrtc/src/whip.c:165-167`)。

なお、同ファイルの他の `calloc` も戻り値を検査せずに直後で参照している箇所がある。たとえば `PeerConnectionFactory_Create` の `calloc` も、戻り値を検査せずに `p->ref = ...` と使用している (`webrtc/src/whip.c:473-475`)。

```c
  struct PeerConnectionFactory* p = (struct PeerConnectionFactory*)calloc(
      1, sizeof(struct PeerConnectionFactory));
  p->ref = webrtc_RefCountInterface_Create(PeerConnectionFactory_delete, p);
```

## 設計方針

- 画像バッファのサイズ計算で、`width * height` の乗算を行う前にオーバーフローを検査する。`width` / `height` が非負であることを確認し、`size_t` での乗算がオーバーフローしないことをチェックしてから `calloc` を呼ぶ。`calloc` は引き続き要素数と要素サイズを分けて渡す。
- `FakeVideoCapturer_Create` の構造体本体の `calloc`、画像バッファの `calloc` のいずれも戻り値を検査し、`NULL` の場合は確保済みリソースを解放したうえで `NULL` を返すなど、安全に失敗を伝播させる。
- あわせて `PeerConnectionFactory_Create` など、同様に `calloc` の戻り値を未検査で参照している箇所も `NULL` チェックを追加する。

## 完了条件

- `width` / `height` に大きな値を指定しても、`int` 乗算のオーバーフローによる過小確保が起きず、オーバーフローを検出して安全に失敗する。
- `calloc` が `NULL` を返した場合に `NULL` 参照でクラッシュしない。
- 失敗時に確保済みリソースを解放したうえで安全に呼び出し元へ失敗が伝わる。
