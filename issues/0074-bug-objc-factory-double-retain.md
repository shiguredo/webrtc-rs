# ObjC ビデオコーデックファクトリの二重リテインによるメモリリーク

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-objc-factory-double-retain
- Polished: {YYYY-MM-DD}

## 目的

macOS / iOS でビデオコーデックファクトリを生成するたびに ObjC オブジェクトが恒久リークする問題を修正する。

## 現状

`webrtc_objc_RTCDefaultVideoDecoderFactory_new` (`webrtc/src/webrtc_c/sdk/objc/components/video_codec/RTCDefaultVideoDecoderFactory.mm`) と `webrtc_objc_RTCDefaultVideoEncoderFactory_new` (`RTCDefaultVideoEncoderFactory.mm`) は、`[[RTCDefaultVideoDecoderFactory alloc] init]` で生成したオブジェクト (+1) を `RetainRTCVideoDecoderFactory` → `CFBridgingRetain` でさらにリテイン (+1) している。解放側 (`webrtc_objc_RTCVideoDecoderFactory_release`) は `CFBridgingRelease` (-1) のみのため、1 リテイン分が恒久リークする。

実際のビルドコマンド (compile_commands.json) を確認したところ、`.mm` ファイルは `-x objective-c++` + `-fobjc-arc` なし (MRC) でコンパイルされている。`webrtc/CMakeLists.txt` の macOS / iOS ブランチにも `-fobjc-arc` は存在しない。MRC では `CFBridgingRetain` は `CFRetain` (+1)、`CFBridgingRelease` は autorelease と等価のため、+1 が確定的に残る。

Rust 側 (`src/api/video_encoder.rs` の `new_with_objc_default_factory` / `src/api/video_decoder.rs` の同関数) が factory を生成するたびに発現する。

なお同一の二重リテイン構造は `audio_session.mm` 系の release 関数にも波及する可能性があるため、修正時に合わせて確認すること。

## 設計方針

二重リテインを解消する。2 つの修正案がある:

1. **`-fobjc-arc` を明示的に付与する**: `webrtc/CMakeLists.txt` の macOS / iOS ブランチの `.mm` ターゲットに `-fobjc-arc` を追加する。ARC では `alloc/init` の +1 が ARC 管理となり、`CFBridgingRetain` とのバランスが取れる
2. **リテイン構造を修正する**: `alloc/init` の +1 を残したまま `CFBridgingRetain` を呼ばず、`alloc/init` の結果をそのまま CF 型へブリッジする (例: `(struct webrtc_objc_RTCVideoDecoderFactory*)CFBridgingRetain` を `__bridge` 相当に変更)

CMakeLists の構造を保つ観点から案 1 が推奨される。ただし Linux / Android では `.mm` が CXX としてコンパイルされるため、`OBJCXX` 言語にのみフラグを適用する必要がある (`$<$<COMPILE_LANGUAGE:OBJCXX>:-fobjc-arc>`)。

## 完了条件

- `RTCDefaultVideoDecoderFactory.mm` / `RTCDefaultVideoEncoderFactory.mm` の `new` / `release` のリテイン収支が +1 / -1 で釣り合うこと
- macOS で factory を生成・解放を繰り返してもメモリ使用量が増加しないこと (Leaks 検出で確認)
- 既存の macOS テスト (`src/tests.rs` の ObjC ファクトリテスト) がパスすること

## 解決方法

- `webrtc/CMakeLists.txt`: macOS / iOS ブランチで `-fobjc-arc` を追加する
- 修正後、`RTCDefaultVideoDecoderFactory.mm` / `RTCDefaultVideoEncoderFactory.mm` の `new` が +1 のみで返ることを確認する
- `audio_session.mm` / `objc.mm` の release 系関数 (`CFBridgingRelease` 使用箇所) も ARC 化で正しく動作することを確認する
