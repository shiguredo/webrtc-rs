# 移植ルールメモ (C++ → C ラッパー)

## 大まかなルール

- C ラッパーには libwebrtc との薄い対応のみを実装し、便利関数や機能追加を行ってはいけない
- C ラッパーは薄く保ち、基本的に **元の C++ API のシグネチャ・名前に忠実に** 移植すること。独自の便利関数やパラメータ展開は禁止
- C++ 側の `webrtc::Xxx` というクラスは `struct webrtc_Xxx` に対応させる。
- C++ 側の `webrtc::Xxx` の `Yyy` 関数は `webrtc_Xxx_Yyy` 関数に対応させる。
- `webrtc::scoped_refptr<CppType>` は `struct CType_refcounted*` として扱い、手動で `CType_AddRef` 及び `CType_Release` を呼ぶことで寿命を管理する
  - 対応する型を宣言する時は `WEBRTC_DECLARE_REFCOUNTED(CType)` を、定義する時は `WEBRTC_DEFINE_REFCOUNTED(CType, CppType)` マクロを利用する
  - `struct CType_refcounted*` から `struct CType*` を取得するには `CType_refcounted_get` 関数を利用する
- `std::unique_ptr<CppType>` は `struct CType_unique*` として扱い、手動で `CType_unique_delete` を呼ぶことで寿命を管理する
  - 対応する型を宣言する時は `WEBRTC_DECLARE_UNIQUE(T)` を、定義する時は `WEBRTC_DEFINE_UNIQUE(CType, CppType)` マクロを利用する
  - `struct CType_unique` から `struct CType*` を取得するには `CType_unique_get` 関数を利用する
- `CppType` 型のオブジェクト（スタック上に配置する C++ のクラス）は `struct CType*` として扱い、手動で `CType_delete` を呼ぶことで寿命を管理する
- `std::optional<CppType>` は `struct CType*` として扱う
  - 引数の型が `CppType` である C++ の関数を移植するときには `struct CType*` として、必要に応じて C++ 側の内部でコピーやムーブなどを行う
- C++ の構造体 `CppType` の `field` 変数へ読み書きする場合には `CppType_get_field` や `CppType_set_field` 関数を定義する
- C++ 側の `webrtc::Xxx` というクラスや関数などに対応する C の構造体や関数などを定義するファイルは、基本的には `webrtc::Xxx` があったパスと合わせる
  - 例えば `webrtc::VideoFrame` は `<api/video/video_frame.h>` で宣言されているので、この C 版を `"webrtc_c/api/video/video_frame.h"` ファイルを作って記述する
  - ただし厳密にやる必要は無く、ある程度分割をサボっても良いものとする
    - 例えば `webrtc_c/api/environment.h` は本来 `api/environment/environment.h` と `api/environment/environment_factory.h` に分かれているが、分ける意味があまり無いので纏めている。
- `*_refcounted` の型を直接 C++ の型にキャストしてはならない
  - 必ず `*_refcounted_get()` 関数を経由すること
- C++ オブジェクトを `*_refcounted` に渡すときは、必ず `webrtc::scoped_refptr<CppType>` で構築し、 `p.release()` したもののみをキャストする。
- `*_unique` の型を直接 C++ の型にキャストしてはならない
  - 必ず `*_unique_get()` 関数を経由すること
- C++ オブジェクトを `*_unique` に渡すときは、必ず `std::unique_ptr<CppType>` で構築し、 `p.release()` したもののみをキャストする。
- **新規 API 追加時は上記のルールが守られているか必ずセルフチェックすること**

## セルフチェック手順

- 作業開始前に RULES.md を読み直し、今回の作業で関係するルール（薄いラッパー、元の C++ パスと名前、命名規則、`*_unique` / `*_refcounted` の扱い）を箇条書きにする
- 対応する C++ パスと型名を必ず開いて照合し、C 側のファイル・シンボル名が元の C++ に一致しているか確認する
- `*_unique` / `*_refcounted` へのキャストが必ず `*_unique_get` / `*_refcounted_get` / `release` 経由になっているか `rg` でチェックする
- 便利関数やパラメータ展開を追加していないか、各変更ブロックごとに「薄いラッパーか」を自問する
- 変更後に再度 RULES.md を読み直し、全ルール順守をチェックリスト形式で確認してから回答する

## 細かい部分

- `webrtc::RTCErrorOr<T>` のような、複数の値を持つ型を返す関数を移植する場合、それぞれの値を受け取るための引数を追加する。
  - `webrtc::RTCErrorOr<webrtc::scoped_refptr<CppType>>` の場合は `struct webrtc_RTCError*` と `struct CppType_refcounted*` を追加してそこに結果を出力する
- `std::string` を返す関数を移植する場合は `struct std_string_unique*` にして、利用後は C アプリケーション側で `std_string_unique_delete` を呼んで解放する

## ビルドと実行

- ビルドコマンド: `python3 run.py build ubuntu-24.04_x86_64`
- 実行コマンド: `./_build/ubuntu-24.04_x86_64/release/webrtc_c/whip_c`
- デバッグビルドコマンド: `python3 run.py build ubuntu-24.04_x86_64 --local-webrtc-build-dir ../../webrtc-build/_worktree/m143.7499.1.0 --debug`
- デバッグ実行コマンド: `./_build/ubuntu-24.04_x86_64/debug/webrtc_c/whip_c`
- lldb-dap プラグインのインストールと lldb-dap バイナリのインストール、`../.vscode/launch.json` の各種パスの設定さえ適切にやれば、VSCode 上でデバッグ実行も可能です
- 参照用の libwebrtc のヘッダーファイルの場所: `./_install/ubuntu-24.04_x86_64/release/webrtc/include`

## アプリケーション

- C アプリケーション側で `webrtc::scoped_refptr` に相互変換できるクラスを作りたい場合、構造体のメンバーに `webrtc_RefCountInterface_ref*` を用意し、`webrtc_RefCountInterface_Create` を呼んで作成する
  - この時に参照カウントが 0 になった時に呼ばれるコールバック関数を設定する
