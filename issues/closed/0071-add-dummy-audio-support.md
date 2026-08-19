# ダミー PCM 音声データを注入できる DummyAudioSource を C ラッパーに追加する

- Priority: Medium
- Created: 2026-06-23
- Completed: 2026-08-19
- Model: DeepSeek V4 Pro
- Branch: feature/add-dummy-audio-support
- Polished: 2026-07-01

## 目的

映像のダミー送信には `AdaptedVideoTrackSource` が利用可能だが、音声については `PeerConnectionFactory::create_audio_source()` で生成される実デバイス由来の `AudioTrackSource` しか存在しない。これにより、マイク等の実入力が存在しない環境では音声送信を伴うテストやサンプルが実行できない。

映像の `AdaptedVideoTrackSource` が行っている「解像度適応（adaptation）」は音声には該当しないため、本 issue では「適応」を意味しない命名として `DummyAudioSource` を採用する。映像と同様の C ラッパーパターンで、ユーザーが PCM 音声データを注入できる `DummyAudioSource` を C ラッパー側に追加する。

なお、Rust ラッパー (`src/api/`) への追加は本 issue のスコープ外とする。

## 優先度根拠

Medium: 既存のサンプルやテストでは音声送信を回避して対応しているが、今後の音声送受信の結合テスト追加や audio stats 検証等の前提として必要な機能である。

## 現状

### 映像のダミー送信 (実装済み、参考パターン)

C ラッパーに `AdaptedVideoTrackSourceWrapper` (`webrtc/src/webrtc_c/media/base/adapted_video_track_source.{h,cc}`) が存在する。`AdaptedVideoTrackSource` クラスを継承し、`AdaptFrame` / `OnFrame` を公開メソッドとして外部からフレーム注入できる。

### 音声の現状

- C ラッパーにダミー音声データを注入できるソースは存在しない
  - `PeerConnectionFactoryInterface::CreateAudioSource` は実デバイス由来の `AudioSourceInterface` を返す
  - `AudioSourceInterface` は `WEBRTC_DECLARE_REFCOUNTED` 宣言のみで、データ注入のための独自拡張は持たない
  - 既存のヘッダー (`api/audio/audio_device.h` 等) はデバイス入出力向けであり、ダミー注入には利用できない

### 映像と音声の実装上の違い

`AdaptedVideoTrackSourceWrapper` は `webrtc::AdaptedVideoTrackSource`（`VideoTrackSourceInterface` 継承）を継承している。この基底クラスが `is_screencast()`、`needs_denoising()`、`state()`、`remote()` のデフォルト実装を提供する。

一方、音声側にはこれに相当する基底クラスが libwebrtc に存在しない。そのため `DummyAudioSourceWrapper` は `webrtc::AudioSourceInterface`（`webrtc::MediaSourceInterface` 継承）を直接継承し、以下の純粋仮想メソッドを自前で実装する必要がある:

- `webrtc::MediaSourceInterface::state()` → `kLive` を返す
- `webrtc::MediaSourceInterface::remote()` → `false` を返す

## 設計方針

`adapted_video_track_source.h` / `adapted_video_track_source.cc` の C ラッパーパターンに倣い、`DummyAudioSource` を新規実装する。

### クラス名とファイル名

| 項目 | 名称 |
|------|------|
| C 型名 | `webrtc_DummyAudioSource` |
| C refcounted 型 | `webrtc_DummyAudioSource_refcounted` |
| C++ 実装クラス | `DummyAudioSourceWrapper` |
| 継承元 | `webrtc::AudioSourceInterface` |
| 新規ヘッダー | `webrtc/src/webrtc_c/media/base/dummy_audio_source.h` |
| 新規実装 | `webrtc/src/webrtc_c/media/base/dummy_audio_source.cc` |

### C API シグネチャ

```c
// ダミー音声ソースを生成する
// sample_rate: サンプリングレート（Hz）。0 や負の値は許容しない
// channels: チャンネル数。0 は許容しない
webrtc_DummyAudioSource_Create(int sample_rate, size_t channels);

// PCM データを注入する
// audio_data: インターリーブされた PCM データ
// samples_per_channel: 1チャンネルあたりのサンプル数
//   全データ長は samples_per_channel * channels * sizeof(int16_t) となる
//   audio_data に null が渡された場合の動作は未定義
webrtc_DummyAudioSource_OnData(source, audio_data, samples_per_channel);

// webrtc_AudioSourceInterface へのキャスト
// WEBRTC_DECLARE_CAST_REFCOUNTED / WEBRTC_DEFINE_CAST_REFCOUNTED マクロを使用する
// refcount 管理
webrtc_DummyAudioSource_refcounted_get / Release
```

### C++ 実装の設計

`DummyAudioSourceWrapper` クラスを新設し、`webrtc::AudioSourceInterface` を継承する。

```cpp
class DummyAudioSourceWrapper : public webrtc::AudioSourceInterface {
 public:
  DummyAudioSourceWrapper(int sample_rate, size_t channels);

  // AudioSourceInterface の実装
  void OnData(const int16_t* audio_data, size_t samples_per_channel);

  // MediaSourceInterface の実装（AdaptedVideoTrackSourceWrapper と同様）
  SourceState state() const override { return kLive; }
  bool remote() const override { return false; }

 private:
  int sample_rate_;
  size_t channels_;
};
```

`OnData` は受け取った PCM データを保持する。`DummyAudioSourceWrapper` は `AudioSourceInterface` を継承しているため、`PeerConnectionFactoryInterface::CreateAudioTrack` の source 引数として直接渡すことができる。

データの sink への配信経路は実装時に決定する。想定する方式を以下に示す:

1. `DummyAudioSourceWrapper` が内部バッファに PCM データを保持する
2. WebRTC の内部パイプライン（`AudioTransport` 経由等）を通じて `AudioTrack` がデータを取得する
3. `AudioTrack` に登録された `AudioTrackSinkInterface` へ配信される

`OnData` は任意のスレッドから呼ばれる可能性があるため、内部状態の保護には `std::mutex` 等の排他制御を用いること。

### 変更対象

- `webrtc/src/webrtc_c/media/base/dummy_audio_source.h` (新規)
- `webrtc/src/webrtc_c/media/base/dummy_audio_source.cc` (新規)
- `webrtc/CMakeLists.txt` (`add_library(webrtc_c ...)` のソースリストに上記 2 ファイルを追加)

### テスト

C ラッパーに対する結合テスト等は後続 issue とする。本 issue ではビルドがパスすることを完了条件とする。

## 完了条件

- `webrtc/src/webrtc_c/media/base/dummy_audio_source.{h,cc}` が追加されている
- `webrtc/CMakeLists.txt` の `add_library(webrtc_c ...)` にビルド対象として追加されている
- C ラッパーのビルドがパスすること
- `CHANGES.md` の `## develop` にエントリが追加されている

## 解決方法

本 issue は実装しない方針とし、closed にする。

webrtc-rs は libwebrtc の薄いラッパーであり、ロジックを含む処理は一切実装しない方針である。`DummyAudioSource` は、受け取った PCM データを内部バッファに保持して sink へ配信するといったロジックを含むため、この方針に反する。

ダミー音声データの注入は webrtc-rs の利用者が各プロジェクトで実装することとし、本 issue はクローズする。
