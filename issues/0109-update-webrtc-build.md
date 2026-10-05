# webrtc-build を m154.8037.4.1 に更新する

- Created: 2026-10-05
- Completed: {YYYY-MM-DD}
- Branch: feature/update-webrtc-build
- Polished: {YYYY-MM-DD}

## 目的

Android 向け prebuilt (`libwebrtc_c-android_arm64.tar.gz`) に同梱される `jar/webrtc.jar` に、`JavaAudioDeviceModule` の pauseRecording / resumeRecording と `AudioTrackSink` を含める。

Rust ベースの Sora Kotlin SDK は Android の音声入力に Java の `JavaAudioDeviceModule` を使うため、この 2 API が無いと音声ハードミュート (録音停止によるマイクインジケータ消灯) と Java の `AudioTrackSink` 経由の受信音声 PCM 取得ができない。

## 前提条件

webrtc-build の `m154.8037.4.1` のビルドが完了し、GitHub Release が公開されていること (2026-10-05 にタグを push 済み。着手時点ではビルド進行中)。

## 現状

- `Cargo.toml` の `package.metadata.external-dependencies.webrtc-build` は `m154.8037.3.0` を指している
- `m154.8037.3.0` の `webrtc.android.tar.gz` 同梱の `jar/webrtc.jar` には `pauseRecording` / `resumeRecording` / `AudioTrackSink` が無い (実物の jar で確認済み)
- webrtc-build では `android_sdk` にだけ `android_audio_pause_resume.patch` と `android_audio_track_sink.patch` を適用していた。`m154.8037.4.1` で `android` にも適用されるようになり、Rust 側が使う成果物にも 2 API が入る
- webrtc-rs の Android 向け prebuilt は `.github/workflows/release.yml` の `build-prebuilt-android` が `cargo build --features source-build --target aarch64-linux-android` の `OUT_DIR` から `webrtc.jar` を拾って `jar/webrtc.jar` として同梱している。この jar の出どころが webrtc-build の `android` 成果物である

## 設計方針

- `Cargo.toml` の `package.metadata.external-dependencies.webrtc-build` を `m154.8037.4.1` に更新する
- `CHANGES.md` の `## develop` に [UPDATE] を記載する
- canary をリリースして prebuilt を作り直す
- 事前ビルドの `jar/webrtc.jar` を展開して 2 API が含まれることを確認する

## 完了条件

- `Cargo.toml` の pin が `m154.8037.4.1` になっている
- canary がリリースされ `libwebrtc_c-android_arm64.tar.gz` が公開されている
- その `jar/webrtc.jar` に `JavaAudioDeviceModule.pauseRecording` / `resumeRecording` と `AudioTrackSink` が含まれている
- `CHANGES.md` の `## develop` に [UPDATE] が入る

## 解決方法

（詳細は polish / 実装時に確定する）
