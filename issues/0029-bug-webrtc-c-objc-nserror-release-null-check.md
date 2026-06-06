# objc_NSError_release に null チェックを追加する

- Priority: Medium
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

webrtc_c の `objc_NSError_release` に null チェックを追加し、`objc_NSString_release` と挙動を対称にする。現状 `objc_NSError_release` は `null` チェックをせずに `CFBridgingRelease` を呼んでおり、`null` を渡された場合の挙動が `objc_NSString_release` と非対称であるため、これを是正する。

## 優先度根拠

`null` を渡さなければ問題は起きないが、対となる `objc_NSString_release` は `null` ガードを持っており、`objc_NSError_release` だけがガードを欠いている。API の対称性が崩れていると利用者が `objc_NSError_release(nullptr)` を安全だと誤解しやすい。iOS 限定 (`WEBRTC_IOS`) のコードで影響範囲は限定的であり、修正も小さいため Medium とする。

## 現状

`webrtc/src/webrtc_c/objc.mm:79-81` の `objc_NSError_release` は `null` チェックをしていない。

```cpp
WEBRTC_EXPORT void objc_NSError_release(struct objc_NSError* self) {
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}
```

一方、対となる `webrtc/src/webrtc_c/objc.mm:50-55` の `objc_NSString_release` は冒頭で `null` チェックを行っている。

```cpp
WEBRTC_EXPORT void objc_NSString_release(struct objc_NSString* self) {
  if (self == nullptr) {
    return;
  }
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}
```

両者とも `CFBridgingRelease` を呼ぶ解放処理だが、`objc_NSString_release` は `null` を渡されると早期 return するのに対し、`objc_NSError_release` はガードがなく挙動が非対称である。これらは `#if defined(WEBRTC_IOS)` ブロック内 (`webrtc/src/webrtc_c/objc.mm:3` 以降) で定義されている。宣言はそれぞれ `webrtc/src/webrtc_c/objc.h:24` (`objc_NSString_release`)、`webrtc/src/webrtc_c/objc.h:38` (`objc_NSError_release`) にある。

## 設計方針

`objc_NSError_release` の冒頭に、`objc_NSString_release` と同じ `null` チェックを追加する。すなわち `self == nullptr` の場合は何もせず早期 return し、`null` でない場合のみ `CFBridgingRelease` を呼ぶ。これにより両 `release` 関数の `null` に対する挙動を対称にする。

## テスト戦略

- iOS ビルド環境で `objc_NSError_release(nullptr)` がクラッシュしないことを確認する
- 非 null の正常系で `objc_NSError_release` が解放後にクラッシュしないことを確認する
- `#if defined(WEBRTC_IOS)` ガード内のコードのため、非 iOS 環境では影響なし

## 完了条件

- `objc_NSError_release` に `null` を渡しても安全に何もしない (早期 return する)。
- `objc_NSError_release` と `objc_NSString_release` の `null` に対する挙動が対称になっている。
