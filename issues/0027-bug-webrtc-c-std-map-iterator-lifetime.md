# std_map イテレータの無効化リスクを解消する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-std-map-iterator-lifetime

## 目的

webrtc_c の `std_map_string_string` 用イテレータが、元のマップへの変更によって無効化され得る設計になっている点を解消する。イテレータが内部に保持する `std::map::iterator` は、元のマップに対する変更で無効化される可能性があり、無効化されたイテレータを進めると未定義動作になるため、安全なライフタイム設計または契約の明確化を行う。

## 優先度根拠

イテレータ使用中に元のマップを変更しなければ問題は起きないが、変更した場合は未定義動作になる。C API の利用者がこの制約を知らずにマップを変更すると、原因の特定が困難なクラッシュやメモリ破壊につながる。常時発生する問題ではなく利用パターンに依存するため High ではないが、未定義動作の回避と契約の明確化のため Medium とする。

## 現状

`webrtc/src/webrtc_c/std.cc:62-67` でイテレータ構造体が定義されている。

```cpp
struct std_map_string_string_iter {
  std::map<std::string, std::string>* map;
  std::map<std::string, std::string>::iterator it;
  bool started;
};
```

`webrtc/src/webrtc_c/std.cc:82-90` の `std_map_string_string_iter_new` で、元のマップへのポインタを保持したイテレータを生成する。

```cpp
WEBRTC_EXPORT struct std_map_string_string_iter* std_map_string_string_iter_new(
    struct std_map_string_string* map) {
  if (map == nullptr) {
    return nullptr;
  }
  auto m = reinterpret_cast<std::map<std::string, std::string>*>(map);
  auto iter = new std_map_string_string_iter{m, {}, false};
  return reinterpret_cast<struct std_map_string_string_iter*>(iter);
}
```

`webrtc/src/webrtc_c/std.cc:96-123` の `std_map_string_string_iter_next` では、保持した `impl->map` に対して `begin()` / `end()` を呼び、`impl->it` を進めながらキーと値を返す。

```cpp
  if (!impl->started) {
    impl->it = impl->map->begin();
    impl->started = true;
  } else if (impl->it != impl->map->end()) {
    ++impl->it;
  }
  if (impl->it == impl->map->end()) {
    return 0;
  }
```

イテレータは元のマップへのポインタ (`impl->map`) と、そのマップに紐づく `std::map::iterator` (`impl->it`) を保持している。`std_map_string_string_iter_new` から `std_map_string_string_iter_delete` までの間に、元のマップに対して `std_map_string_string_set` (`webrtc/src/webrtc_c/std.cc:69-76`) などで要素の挿入・削除が行われると、保持中の `impl->it` が指す要素が無効化され得る。無効化されたイテレータに対する `++impl->it` や比較は未定義動作となる。また、イテレータ生成後に元のマップ自体が破棄された場合も `impl->map` が宙吊りになり未定義動作となる。現状、これらの制約を述べたコメントやガードは存在しない。

## 設計方針

以下のいずれかを検討する。

- 契約の明確化: 「イテレータの有効期間中は元のマップを変更してはならない」「元のマップはイテレータより長く生存していなければならない」という契約を、`std_map_string_string_iter` 定義箇所および `std_map_string_string_iter_new` / `_iter_next` の宣言・実装コメントとして明記する。
- スナップショット方式: イテレータ生成時にキーと値の一覧をコピーして保持し、以降は元のマップの変更に影響されないようにする。ただし CLAUDE.md の「入力サイズを信用しない / 事前確保を避ける」方針や RULES.md の薄いラッパー原則 (`webrtc/RULES.md:5-6`) との整合を確認すること。スナップショット方式を採る場合、コピーコスト増加の妥当性も併せて判断する。

どちらを採るかは設計判断が必要なため、判断の根拠を整理し、必要なら許可を得ること。

## 完了条件

- イテレータ使用中に元のマップを操作しても未定義動作にならない、または「イテレータ有効期間中はマップを変更しない」「マップはイテレータより長く生存する」という契約がコードコメントで明確になっている。
- 採用した方針が RULES.md の薄いラッパー原則と整合していることを確認している。
