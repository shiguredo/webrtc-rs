# whip.cpp の int と size_t の符号混在比較を修正する

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.cpp` のループで、ループ変数を `int` として宣言し、`size()` が返す符号なし整数（`size_t` 相当）と比較している。`int` と `size_t` の比較では暗黙の符号変換が発生し、意図しない結果やコンパイラ警告の原因になり得る。比較対象の型を揃えて符号混在比較を解消する。

## 優先度根拠

`encodings.size()` の値が `int` の表現範囲を超えることは実運用上想定しづらく、現時点で実害が出ているわけではない。ただし符号混在比較は将来の不具合や警告の温床になるため、予防的に修正する価値があると判断し Low とする。

## 現状

`webrtc/src/whip.cpp:868` 付近で、ループ変数 `i` を `int` として宣言し、`p.encodings.size()` と比較している。

```cpp
                                    for (int i = 0; i < p.encodings.size();
                                         i++) {
                                      p.encodings[i].codec =
                                          video_init.send_encodings[i].codec;
                                      p.encodings[i].scalability_mode =
                                          video_init.send_encodings[i]
                                              .scalability_mode;
                                    }
```

`p.encodings.size()` は符号なし整数を返すため、`i < p.encodings.size()` は `int` と `size_t` の符号混在比較になっている。比較時に `int` 側が符号なしへ変換されるため、符号変換に依存した挙動となっている。

## 設計方針

ループ変数の型を `size()` の戻り値の型に揃え、符号混在比較を解消する。インデックスアクセス（`p.encodings[i]` および `video_init.send_encodings[i]`）の意味は変えない。

## 完了条件

- ループ変数と `size()` の戻り値の比較が符号混在にならないようになっている。
- ループの動作（各要素への `codec` と `scalability_mode` の代入）が現状と等価である。
