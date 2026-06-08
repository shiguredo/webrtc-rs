# objc_NSString_release の冗長な null チェックを削除する

- Priority: Medium
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/refactor-remove-objc-nsstring-release-null-check
- Polished: 2026-06-08

## 目的

webrtc_c の `objc_NSString_release` にある冗長な `null` チェックを削除し、
`objc_NSError_release` と同じ実装パターンに揃える。
修正前後で実行時の挙動は変わらず、両関数のコードを対称かつ簡潔にする。

## 優先度根拠

`CFBridgingRelease(nullptr)` は安全であり、現状のコードでもクラッシュは発生しない。
しかし、同じ役割を持つ 2 つの `release` 関数でコードの構造が異なる非対称は、
保守者が「null チェックが必要なのか否か」を判断しづらくなる。
iOS 限定 (`WEBRTC_IOS`) のコードで影響範囲は限定的であり、修正も小さいため Medium とする。

## 現状

`webrtc/src/webrtc_c/objc.mm:50-55` の `objc_NSString_release` は不要な `null` チェックを持っている。

```cpp
WEBRTC_EXPORT void objc_NSString_release(struct objc_NSString* self) {
  if (self == nullptr) {
    return;
  }
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}
```

一方、`webrtc/src/webrtc_c/objc.mm:79-81` の `objc_NSError_release` は `null` チェックを持たず、これが正しい実装である。

```cpp
WEBRTC_EXPORT void objc_NSError_release(struct objc_NSError* self) {
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}
```

`CFBridgingRelease` は Apple により `_Nullable` (`CFTypeRef _Nullable CF_CONSUMED X`) と宣言されており、`null` 入力は API 仕様として許容されている。
実体はコンパイラ組込みの `(__bridge_transfer id)X` キャストであり、`X` が `null` なら単に `nil` を返す。
ObjC / ARC では `nil` に対する retain/release は無視される。

なお、`objc_NSString_release` / `objc_NSError_release` はコードベース内 (`*.rs`, `*.c`, `*.mm`) のどこからも直接呼び出されていない（`WEBRTC_EXPORT` により外部公開されており、利用側バイナリからの FFI 呼び出しが想定されている）。

## 修正対象ファイル一覧

| ファイル | 修正内容 |
|----------|----------|
| `webrtc/src/webrtc_c/objc.mm` | `objc_NSString_release` から `null` チェックブロック (`if (self == nullptr) { return; }`) を削除する |

## 設計方針

`objc_NSString_release` から `null` チェックを削除し、`objc_NSError_release` と同じ実装パターンに揃える。
宣言側 (`objc.h`) の変更は不要。

```cpp
// 修正後
WEBRTC_EXPORT void objc_NSString_release(struct objc_NSString* self) {
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}
```

## 後方互換への影響

- C API のシグネチャは一切変更されないため、後方互換は完全に維持される
- `null` を渡した場合の実行時の挙動も修正前後で変わらない

## CHANGES.md

```
### misc

- [UPDATE] objc_NSString_release の冗長な null チェックを削除する
  - CFBridgingRelease は null 安全であり、objc_NSError_release と実装パターンを揃える
  - @実装者
```

## テスト戦略

- `#if defined(WEBRTC_IOS)` ガード内のコードのため、非 iOS 環境ではコンパイルされず影響はない
- コードベース内に直接の呼び出し元が存在しないため、コンパイルが通ることの確認で十分
- 合成テストとして `objc_NSString_release(nullptr)` がクラッシュしないことは、修正前の早期 return 削除 → `CFBridgingRelease(nullptr)` が安全であることの検証になるが、iOS ビルド環境での実施が前提

## 完了条件

- `objc_NSString_release` から冗長な `null` チェックが削除されている
- `objc_NSString_release` と `objc_NSError_release` の実装パターンが対称になっている
