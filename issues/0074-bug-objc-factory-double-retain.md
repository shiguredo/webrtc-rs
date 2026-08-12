# ObjC ビデオコーデックファクトリの二重リテインによるメモリリーク

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-objc-factory-double-retain
- Polished: 2026-08-12

## 目的

macOS / iOS でビデオコーデックファクトリを生成するたびに ObjC オブジェクトが恒久リークする問題を修正する。

## 現状

`webrtc_objc_RTCDefaultVideoDecoderFactory_new` (`webrtc/src/webrtc_c/sdk/objc/components/video_codec/RTCDefaultVideoDecoderFactory.mm`) と `webrtc_objc_RTCDefaultVideoEncoderFactory_new` (`webrtc/src/webrtc_c/sdk/objc/components/video_codec/RTCDefaultVideoEncoderFactory.mm`) は、`[[RTCDefaultVideoDecoderFactory alloc] init]` で生成したオブジェクト (+1) を `RetainRTCVideoDecoderFactory` → `CFBridgingRetain` でさらにリテイン (+1) している。解放側 (`webrtc_objc_RTCVideoDecoderFactory_release`) は `CFBridgingRelease` (-1) のみのため、1 リテイン分が恒久リークする。

`webrtc/CMakeLists.txt` の macOS / iOS ブランチに `-fobjc-arc` は存在せず、`.mm` は MRC でコンパイルされる。MRC では `CFBridgingRetain` は `CFRetain` (+1)、`CFBridgingRelease` は autorelease と等価のため、+1 が確定的に残る。

Rust 側 (`src/api/video_encoder.rs` の `VideoEncoderFactory::from_objc_default` / `src/api/video_decoder.rs` の `VideoDecoderFactory::from_objc_default`) は、`new` → `webrtc_ObjCToNativeVideoEncoderFactory` / `webrtc_ObjCToNativeVideoDecoderFactory`（native 側の Adapter が retain する）→ `release` の順で呼ぶ。二重リテインがあると、new の +2 に対し release が -1 のため +1 が恒久リークする。

なお `audio_session.mm` / `objc.mm` の CFBridging 使用箇所は、`sharedInstance` / `webRTCConfiguration` / `stringWithUTF8String` のような +0 起点の戻り値を `CFBridgingRetain` (+1) する構造であり、`alloc/init` の +1 と `CFBridgingRetain` の +1 が重なる二重リテイン構造は存在しない。ただしこれらは iOS 専用のビルド対象 (`webrtc/CMakeLists.txt` の ios_arm64 ブランチ) であるため、修正案 1 を採用する場合は ARC 化の影響を iOS ビルドで確認する。

## 設計方針

二重リテインを解消する。2 つの修正案がある:

1. **`-fobjc-arc` を明示的に付与する**: `webrtc/CMakeLists.txt` の macOS / iOS ブランチの `target_compile_options` に `-fobjc-arc` を追加する。ARC では `alloc/init` の +1 は式評価後のテンポラリ解放と `RetainRTCVideoDecoderFactory` の strong パラメータの解放で相殺され、`CFBridgingRetain` の +1 のみが残り、release の `CFBridgingRelease` と釣り合う
2. **リテイン構造を修正する**: `alloc/init` の +1 を残したまま `CFBridgingRetain` を呼ばず、`alloc/init` の結果をそのまま CF 型へ渡す。ただしこれは MRC 維持が前提であり、将来 ARC 化した場合は `__bridge` 相当のキャストで返すと関数終了時に ARC が解放してダングリングポインタになるため、ARC 化と併用できない。また release 側の `CFBridgingRelease` は MRC では autorelease 相当のため、解放が autorelease pool の drain まで遅延する欠点がある

案 1 はビルド設定でリテイン構造を一元管理でき、将来追加される `.mm` にも同一のリテイン規約が適用されるため、案 1 が推奨される。ただし Linux / Android / Windows / Raspberry Pi などの非 Apple ターゲットでは `.mm` が CXX としてコンパイルされるため、`OBJCXX` 言語にのみフラグを適用する必要がある (`$<$<COMPILE_LANGUAGE:OBJCXX>:-fobjc-arc>`)。

## 完了条件

- `RTCDefaultVideoDecoderFactory.mm` / `RTCDefaultVideoEncoderFactory.mm` の `new` / `release` のリテイン収支が +1 / -1 で釣り合うこと（コードレビューで確認）。iOS 専用の `audio_session.mm` / `objc.mm` も ARC 化後の収支が変わらないことをコードレビューで確認すること（実行テストができないため）
- factory の生成・解放を繰り返すテストを `src/tests.rs` に追加し、リテインリークが発生しないことを確認すること（MRC では `CFBridgingRelease` が autorelease 相当のため、テスト内で autorelease pool を明示的に drain してから確認する。macOS では `leaks` コマンド / Instruments でリーク 0 を確認）
- 既存の macOS テスト (`src/tests.rs` の ObjC ファクトリテスト) がパスすること
- iOS ビルドが CI で通ること（`audio_session.mm` / `objc.mm` を含む全 `.mm` が ARC 化されるため）

## 解決方法

- `webrtc/CMakeLists.txt`: macOS / iOS ブランチで `-fobjc-arc` を追加する
- 修正後、`RTCDefaultVideoDecoderFactory.mm` / `RTCDefaultVideoEncoderFactory.mm` の `new` が +1 のみで返ることを確認する
- `webrtc/src/webrtc_c/sdk/objc/native/api/video_decoder_factory.mm` / `video_encoder_factory.mm` も ARC 化の影響を受けるため、ビルド・テストで正しく動作することを確認する
