# Apple 向けバンドルで -ObjC フラグ伝搬を不要にする

Created: 2026-04-01
Completed: 2026-04-01
Model: GPT-5.4

## 背景

`ObjCToNativeVideoEncoderFactory` / `ObjCToNativeVideoDecoderFactory` を利用する経路で、利用者側リンクに `-ObjC` が必要になる。
しかし `webrtc-rs` 側の `-ObjC` は static library 利用者に伝搬しないため、利用者の追加設定が必要になってしまう。

## 根拠

`libwebrtc.a` には Objective-C のカテゴリ実装が含まれるが、通常の static link では未参照メンバーが取り込まれない。
その結果、`ObjCToNative...` のリンクは通ってもカテゴリ実装が最終バイナリに入らず、実行時に必要メソッドが欠落する。

## 再現手順

1. Apple ターゲットで `VideoEncoderFactory::from_objc_default` または `VideoDecoderFactory::from_objc_default` を利用するアプリをビルドする
2. 利用者側のリンクオプションに `-ObjC` を付けない
3. カテゴリメソッド未解決の実行時エラーになる

## 対応内容

- Apple 向けバンドル処理で `libwebrtc.a` から Objective-C カテゴリ実装メンバーを抽出する
- 抽出したメンバーを `ld -r -all_load -keep_private_externs` で `webrtc_objc_blob.o` にまとめる
- `libwebrtc_core.a`（抽出メンバー除外後）と `webrtc_objc_blob.o` を `libwebrtc_c.a` に同梱する
- バンドル処理は `bundled_webrtc_c_bundling` ターゲットを明示ビルドしたときだけ実行されるようにする
- `build.rs` / `CMakeLists.txt` から `-ObjC` 指定を削除する

## 解決方法

- `webrtc/scripts/bundle_apple_static_library.sh` を追加し、Apple 向けバンドル時に Objective-C カテゴリ実装メンバーを `webrtc_objc_blob.o` に集約する方式へ変更した
- `ld -r -all_load -keep_private_externs` により `ObjCVideoEncoderFactory` / `ObjCVideoDecoderFactory` の解決に必要なシンボルを blob 側で保持し、`anchor` 呼び出しなしでリンク解決できるようにした
- `bundle_apple_static_library.sh` では `target_obj` のアーキテクチャを基準に `libtool -arch_only` を実行することで、入力 `libwebrtc.a` の形式に依存せず対象アーキテクチャのみを処理するようにした
- Objective-C メンバーの `ar -x` / `ar -d` を 1 件ずつの反復処理から一括処理へ変更し、バンドル処理時間を短縮した
- `webrtc/CMakeLists.txt` の `bundle_static_library` の `APPLE` 分岐を上記スクリプト呼び出しへ置換した
- `webrtc/CMakeLists.txt` の `bundled_webrtc_c_bundling` ターゲットから `ALL` を外し、通常の再ビルド時に不要なバンドル処理が走らないようにした
- `build.rs` の macOS / iOS から `cargo:rustc-link-arg=-ObjC` を削除した
- `webrtc/CMakeLists.txt` の macOS 向け `target_link_options(... -ObjC)` を削除した
