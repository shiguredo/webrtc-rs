use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use nojson::RawJson;

struct BuildMetadata {
    webrtc_build_version: String,
    webrtc_base_url: String,
    cmake_osx_deployment_target: String,
    android_platform: String,
    android_commandlinetools_version: String,
    android_ndk_version: String,
}

/// Cargo.toml からビルド設定を取得する。
///
/// 引数:
/// - `manifest_dir`: この crate の `Cargo.toml` があるディレクトリ。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `BuildMetadata`: ビルドに必要な固定設定値。
fn get_build_metadata(manifest_dir: &Path) -> BuildMetadata {
    let cargo_toml_path = manifest_dir.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).unwrap_or_else(|e| {
        panic!(
            "Failed to read Cargo.toml: path={}, error={}",
            cargo_toml_path.display(),
            e
        )
    });
    let cargo_toml =
        shiguredo_toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");
    let metadata = shiguredo_toml::Value::Table(cargo_toml);
    let package_metadata = metadata
        .get("package")
        .and_then(|v| v.get("metadata"))
        .expect("package.metadata is missing in Cargo.toml");
    let webrtc_build = package_metadata
        .get("external-dependencies")
        .and_then(|v| v.get("webrtc-build"))
        .expect("package.metadata.external-dependencies.webrtc-build is missing in Cargo.toml");
    let build_config = package_metadata
        .get("build-config")
        .expect("package.metadata.build-config is missing in Cargo.toml");

    let webrtc_build_version = webrtc_build
        .get("version")
        .and_then(|v| v.as_str())
        .expect("webrtc-build.version is missing")
        .to_string();
    let webrtc_base_url = webrtc_build
        .get("base-url")
        .and_then(|v| v.as_str())
        .expect("webrtc-build.base-url is missing")
        .to_string();
    let cmake_osx_deployment_target = build_config
        .get("cmake-osx-deployment-target")
        .and_then(|v| v.as_str())
        .expect("build-config.cmake-osx-deployment-target is missing")
        .to_string();
    let android_platform = build_config
        .get("android-platform")
        .and_then(|v| v.as_str())
        .expect("build-config.android-platform is missing")
        .to_string();
    let android_commandlinetools_version = build_config
        .get("android-commandlinetools-version")
        .and_then(|v| v.as_str())
        .expect("build-config.android-commandlinetools-version is missing")
        .to_string();
    let android_ndk_version = build_config
        .get("android-ndk-version")
        .and_then(|v| v.as_str())
        .expect("build-config.android-ndk-version is missing")
        .to_string();

    BuildMetadata {
        webrtc_build_version,
        webrtc_base_url,
        cmake_osx_deployment_target,
        android_platform,
        android_commandlinetools_version,
        android_ndk_version,
    }
}

fn main() {
    // Cargo.toml か build.rs が更新されたら、依存ライブラリを再ビルドする
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_LOCAL_EXPORT");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_SOURCE_BUILD");
    println!("cargo::rerun-if-env-changed=WEBRTC_C_TARGET");
    println!("cargo::rerun-if-env-changed=WEBRTC_C_SYSROOT");
    println!("cargo::rerun-if-env-changed=LIBCLANG_PATH");
    println!("cargo::rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo::rerun-if-env-changed=ANDROID_NDK");

    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR の取得に失敗しました"),
    );
    let webrtc_dir = manifest_dir.join("webrtc");
    let build_metadata = get_build_metadata(&manifest_dir);

    // ソースファイルの変更を監視
    println!(
        "cargo:rerun-if-changed={}",
        webrtc_dir.join("src").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        webrtc_dir.join("CMakeLists.txt").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        webrtc_dir.join("scripts").display()
    );

    let target_platform = get_target_platform();
    let out_dir = get_out_dir();

    let lib_path = if should_use_prebuilt() {
        let paths = download_prebuilt(&target_platform, &out_dir).unwrap_or_else(|e| {
            panic!(
                "prebuilt ライブラリのダウンロードに失敗しました: {}\n\
                 ソースからビルドする場合は --features source-build を指定してください",
                e
            )
        });
        paths.lib_path
    } else {
        build_from_source(
            &manifest_dir,
            &webrtc_dir,
            &target_platform,
            &out_dir,
            &build_metadata,
        )
    };

    emit_link_directives(&lib_path);
}

struct PrebuiltPaths {
    lib_path: PathBuf,
}

/// prebuilt バイナリを使用するかどうかを判定する。
///
/// 引数: なし。
///
/// 環境変数依存:
/// - `CARGO_FEATURE_SOURCE_BUILD`
///
/// 戻り値:
/// - `true`: prebuilt を利用する。
/// - `false`: ソースビルドする。
///
/// 副作用: なし。
fn should_use_prebuilt() -> bool {
    // source-build feature が有効 → ソースビルド
    if env::var("CARGO_FEATURE_SOURCE_BUILD").is_ok() {
        return false;
    }
    // デフォルトで prebuilt を試みる
    true
}

/// ソースからビルドする（CMake + bindgen）。
///
/// 引数:
/// - `manifest_dir`: crate ルートディレクトリ。
/// - `webrtc_dir`: C/C++ 側ソースディレクトリ。
/// - `target_platform`: `android_arm64` などの内部ターゲット名。
/// - `out_dir`: Cargo が割り当てた出力先。
/// - `build_metadata`: `Cargo.toml` 由来のビルド設定。
///
/// 環境変数依存:
/// - `ANDROID_NDK_HOME`, `ANDROID_NDK`（Android 時）
/// - `LIBCLANG_PATH`（Windows 時）
///
/// 戻り値:
/// - 生成した静的ライブラリのパス。
///
/// 副作用:
/// - Android 時に NDK 自動セットアップを実行する。
/// - CMake ビルド結果を `out_dir` 配下へ出力する。
/// - `bindings.rs` を生成する。
fn build_from_source(
    manifest_dir: &Path,
    webrtc_dir: &Path,
    target_platform: &str,
    out_dir: &Path,
    build_metadata: &BuildMetadata,
) -> PathBuf {
    if target_platform == "android_arm64" {
        ensure_android_ndk(manifest_dir, build_metadata);
    }

    let header = webrtc_dir.join("src").join("webrtc_c.h");
    let include_dir = webrtc_dir.join("src");
    let lib_path = build_webrtc_c(webrtc_dir, target_platform, out_dir, build_metadata);
    maybe_export_local_build_dir(webrtc_dir, out_dir);
    if is_windows_target(target_platform) {
        ensure_windows_libclang(out_dir);
    }
    generate_bindings(&header, &include_dir);
    lib_path
}

/// Android NDK の存在を保証する。
///
/// 引数:
/// - `manifest_dir`: crate ルートディレクトリ。
/// - `build_metadata`: Android NDK / command-line tools のバージョン情報を含む設定。
///
/// 環境変数依存:
/// - `ANDROID_NDK_HOME`, `ANDROID_NDK`
///
/// 戻り値: なし。
///
/// 副作用:
/// - 必要時に `target/android-sdk` へ command-line tools / NDK をインストールする。
/// - 環境変数 `ANDROID_NDK_HOME` と `ANDROID_NDK` を設定する。
fn ensure_android_ndk(manifest_dir: &Path, build_metadata: &BuildMetadata) {
    if let Some(ndk) = resolve_android_ndk_from_env() {
        export_android_ndk_env(&ndk);
        return;
    }

    let target_directory = load_target_directory_from_metadata(manifest_dir);
    let sdk_root = PathBuf::from(target_directory).join("android-sdk");
    install_android_commandline_tools(&sdk_root, &build_metadata.android_commandlinetools_version);

    let ndk_dir = android_ndk_dir(&sdk_root, &build_metadata.android_ndk_version);
    if !android_ndk_toolchain_file(&ndk_dir).exists() {
        let sdkmanager = android_sdkmanager_path(&sdk_root);
        if !sdkmanager.exists() {
            panic!("sdkmanager が見つかりません: {}", sdkmanager.display());
        }
        run_sdkmanager_with_auto_yes(&sdkmanager, &sdk_root, &["--licenses"]);
        let ndk_package = format!("ndk;{}", build_metadata.android_ndk_version);
        run_sdkmanager_with_auto_yes(&sdkmanager, &sdk_root, &["--install", &ndk_package]);
    }

    export_android_ndk_env(&ndk_dir);
}

/// 環境変数から既存の Android NDK パスを解決する。
///
/// 引数: なし。
///
/// 環境変数依存:
/// - `ANDROID_NDK_HOME`, `ANDROID_NDK`
///
/// 戻り値:
/// - `Some(path)`: 有効な NDK が見つかった。
/// - `None`: 見つからなかった。
///
/// 副作用:
/// - 無効な値を検出した場合は標準エラーへ警告を出力する。
fn resolve_android_ndk_from_env() -> Option<PathBuf> {
    for key in ["ANDROID_NDK_HOME", "ANDROID_NDK"] {
        let Ok(value) = env::var(key) else {
            continue;
        };
        let ndk = PathBuf::from(&value);
        if android_ndk_toolchain_file(&ndk).exists() {
            return Some(ndk);
        }
        eprintln!(
            "{} を無視します (Android NDK toolchain file が見つかりません): {}",
            key, value
        );
    }
    None
}

/// Android NDK 関連の環境変数を現在プロセスに設定する。
///
/// 引数:
/// - `ndk`: 設定する NDK ルートパス。
///
/// 環境変数依存: なし。
///
/// 戻り値: なし。
///
/// 副作用:
/// - `ANDROID_NDK_HOME`, `ANDROID_NDK` を設定する。
fn export_android_ndk_env(ndk: &Path) {
    unsafe {
        env::set_var("ANDROID_NDK_HOME", ndk);
        env::set_var("ANDROID_NDK", ndk);
    }
}

/// `cargo metadata` から `target_directory` を取得する。
///
/// 引数:
/// - `manifest_dir`: 対象 `Cargo.toml` の親ディレクトリ。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `cargo metadata` が返した `target_directory` 文字列。
///
/// 副作用:
/// - 外部コマンド `cargo metadata` を実行する。
fn load_target_directory_from_metadata(manifest_dir: &Path) -> String {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps")
        .arg("--manifest-path")
        .arg(manifest_dir.join("Cargo.toml"))
        .output()
        .expect("cargo metadata の実行に失敗しました");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("cargo metadata の実行に失敗しました: {stderr}");
    }

    let stdout = String::from_utf8(output.stdout)
        .expect("cargo metadata の標準出力のデコードに失敗しました");
    let json = RawJson::parse(&stdout).expect("cargo metadata JSON のパースに失敗しました");
    json.value()
        .to_member("target_directory")
        .expect("cargo metadata に target_directory がありません")
        .required()
        .expect("cargo metadata の target_directory が null です")
        .try_into()
        .expect("target_directory の文字列変換に失敗しました")
}

/// Android command-line tools を必要に応じて配置する。
///
/// 引数:
/// - `sdk_root`: Android SDK ルート (`target/android-sdk`)。
/// - `version`: command-line tools のバージョン番号。
///
/// 環境変数依存:
/// - ホスト OS 判定に `env::consts::OS` を利用。
///
/// 戻り値: なし。
///
/// 副作用:
/// - `curl` でダウンロードし、`tar` / `unzip` で展開する。
/// - `sdk_root` 配下へディレクトリ作成、移動、削除を行う。
fn install_android_commandline_tools(sdk_root: &Path, version: &str) {
    if android_sdkmanager_path(sdk_root).exists() {
        return;
    }

    if sdk_root.exists() {
        fs::remove_dir_all(sdk_root).expect("既存 Android SDK ディレクトリの削除に失敗しました");
    }
    fs::create_dir_all(sdk_root).expect("Android SDK ディレクトリの作成に失敗しました");

    let temp_extract_dir = sdk_root.join("cmdline-tools-tmp");
    if temp_extract_dir.exists() {
        fs::remove_dir_all(&temp_extract_dir)
            .expect("既存の command-line tools 一時ディレクトリの削除に失敗しました");
    }
    fs::create_dir_all(&temp_extract_dir)
        .expect("command-line tools 一時ディレクトリの作成に失敗しました");

    let os_segment = commandline_tools_os_segment();
    let url = format!(
        "https://dl.google.com/android/repository/commandlinetools-{os_segment}-{version}_latest.zip"
    );
    let archive_path = sdk_root.join(format!("commandlinetools-{os_segment}-{version}.zip"));

    eprintln!("Android command-line tools をダウンロード中: {url}");
    download(&url, &archive_path).expect("command-line tools のダウンロードに失敗しました");

    extract(&archive_path, &temp_extract_dir)
        .expect("command-line tools アーカイブ展開に失敗しました");

    fs::rename(
        temp_extract_dir.join("cmdline-tools"),
        sdk_root.join("cmdline-tools"),
    )
    .expect("展開済み command-line tools の移動に失敗しました");

    let _ = fs::remove_file(&archive_path);
    let _ = fs::remove_dir_all(&temp_extract_dir);

    let sdkmanager = android_sdkmanager_path(sdk_root);
    if !sdkmanager.exists() {
        panic!(
            "command-line tools 展開後に sdkmanager が見つかりません: {}",
            sdkmanager.display()
        );
    }
}

/// `sdkmanager` を `yes` パイプ付きで実行する。
///
/// 引数:
/// - `sdkmanager`: 実行する `sdkmanager` のパス。
/// - `sdk_root`: `--sdk_root` に渡す Android SDK ルート。
/// - `args`: `sdkmanager` に追加で渡す引数列。
///
/// 環境変数依存: なし。
///
/// 戻り値: なし。
///
/// 副作用:
/// - 外部プロセスを起動する。
/// - `ANDROID_SDK_ROOT`, `ANDROID_HOME` を子プロセス環境へ設定する。
fn run_sdkmanager_with_auto_yes(sdkmanager: &Path, sdk_root: &Path, args: &[&str]) {
    let mut command = Command::new(sdkmanager);
    command.arg(format!("--sdk_root={}", sdk_root.display()));
    command.args(args);
    command.env("ANDROID_SDK_ROOT", sdk_root);
    command.env("ANDROID_HOME", sdk_root);
    command.stdin(Stdio::piped());

    let mut child = command.spawn().expect("sdkmanager の起動に失敗しました");

    {
        let mut stdin = child
            .stdin
            .take()
            .expect("sdkmanager の標準入力オープンに失敗しました");
        for _ in 0..200 {
            stdin
                .write_all(b"y\n")
                .expect("sdkmanager の標準入力への書き込みに失敗しました");
        }
    }

    let status = child
        .wait()
        .expect("sdkmanager プロセスの終了待機に失敗しました");
    if !status.success() {
        panic!(
            "sdkmanager の実行に失敗しました: {} {}",
            sdkmanager.display(),
            args.join(" ")
        );
    }
}

/// Android NDK のバージョン付きディレクトリを組み立てる。
///
/// 引数:
/// - `sdk_root`: Android SDK ルート。
/// - `version`: NDK バージョン文字列。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `sdk_root/ndk/<version>`。
///
/// 副作用: なし。
fn android_ndk_dir(sdk_root: &Path, version: &str) -> PathBuf {
    sdk_root.join("ndk").join(version)
}

/// Android NDK の標準 toolchain ファイルパスを組み立てる。
///
/// 引数:
/// - `ndk`: Android NDK ルート。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `<ndk>/build/cmake/android.toolchain.cmake`。
///
/// 副作用: なし。
fn android_ndk_toolchain_file(ndk: &Path) -> PathBuf {
    ndk.join("build")
        .join("cmake")
        .join("android.toolchain.cmake")
}

/// command-line tools ダウンロード URL 用の OS セグメントを返す。
///
/// 引数: なし。
///
/// 環境変数依存: なし（`env::consts::OS` のみ参照）。
///
/// 戻り値:
/// - `linux` / `mac` / `win`。
///
/// 副作用:
/// - 未対応 OS の場合に panic する。
fn commandline_tools_os_segment() -> &'static str {
    match env::consts::OS {
        "linux" => "linux",
        "macos" => "mac",
        "windows" => "win",
        other => panic!("Android command-line tools に未対応のホスト OS です: {other}"),
    }
}

/// `sdk_root` から `sdkmanager` 実行ファイルのフルパスを組み立てる。
///
/// 引数:
/// - `sdk_root`: Android SDK ルート。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `<sdk_root>/cmdline-tools/bin/<sdkmanager(.bat)>`。
///
/// 副作用: なし。
fn android_sdkmanager_path(sdk_root: &Path) -> PathBuf {
    let executable_name = if cfg!(target_os = "windows") {
        "sdkmanager.bat"
    } else {
        "sdkmanager"
    };
    sdk_root
        .join("cmdline-tools")
        .join("bin")
        .join(executable_name)
}

/// prebuilt バイナリをダウンロードして展開し、リンク用成果物を配置する。
///
/// 引数:
/// - `target`: ダウンロード対象のターゲット名（`android_arm64` など）。
/// - `out_dir`: Cargo の出力先ディレクトリ。
///
/// 環境変数依存:
/// - `CARGO_PKG_VERSION`（GitHub Releases のタグ決定に使用）。
///
/// 戻り値:
/// - `Ok(PrebuiltPaths)`: 配置済み静的ライブラリのパス情報。
/// - `Err(String)`: ダウンロード、検証、展開、コピーいずれかに失敗。
///
/// 副作用:
/// - 外部ネットワークからファイルをダウンロードする。
/// - `out_dir` 配下へファイル作成、展開、コピー、削除を行う。
fn download_prebuilt(target: &str, out_dir: &Path) -> Result<PrebuiltPaths, String> {
    let version = env::var("CARGO_PKG_VERSION").map_err(|e| e.to_string())?;
    let base_url = format!(
        "https://github.com/shiguredo/webrtc-rs/releases/download/{}",
        version
    );
    let archive_name = format!("libwebrtc_c-{}.tar.gz", target);
    let archive_url = format!("{}/{}", base_url, archive_name);
    let sha256_url = format!("{}/{}.sha256", base_url, archive_name);

    eprintln!("prebuilt ライブラリをダウンロード中: {}", archive_url);

    // OUT_DIR/prebuilt/ に展開
    let prebuilt_dir = out_dir.join("prebuilt");
    let archive_path = out_dir.join("prebuilt.tar.gz");
    let sha256_path = out_dir.join("prebuilt.sha256");
    download(&archive_url, &archive_path)?;
    download(&sha256_url, &sha256_path)?;
    verify_sha256(&archive_path, &sha256_path)?;
    fs::create_dir_all(&prebuilt_dir)
        .map_err(|e| format!("展開先ディレクトリ作成に失敗: {}", e))?;
    extract(&archive_path, &prebuilt_dir)?;
    let _ = fs::remove_file(&archive_path);
    let _ = fs::remove_file(&sha256_path);

    // 静的ライブラリを OUT_DIR/lib/ にコピー
    let static_lib_name = static_library_filename_for_target(target);
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).map_err(|e| format!("lib ディレクトリ作成に失敗: {}", e))?;
    let lib_path = lib_dir.join(static_lib_name);
    fs::copy(prebuilt_dir.join("lib").join(static_lib_name), &lib_path)
        .map_err(|e| format!("{static_lib_name} のコピーに失敗: {}", e))?;

    // bindgen 生成済みの bindings.rs を OUT_DIR/ にコピー
    // （利用者が libclang-dev をインストールしなくて済むようにするため）
    fs::copy(
        prebuilt_dir.join("bindings.rs"),
        out_dir.join("bindings.rs"),
    )
    .map_err(|e| format!("bindings.rs のコピーに失敗: target={target}, error={e}"))?;

    Ok(PrebuiltPaths { lib_path })
}

/// `curl` を使って単一ファイルをダウンロードする。
///
/// 引数:
/// - `url`: ダウンロード元 URL。
/// - `output`: 保存先ファイルパス。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(())`: ダウンロード成功。
/// - `Err(String)`: `curl` 実行失敗または非 0 終了。
///
/// 副作用:
/// - 外部コマンド `curl` を実行する。
/// - `output` の作成または上書きを行う。
fn download(url: &str, output: &Path) -> Result<(), String> {
    let status = Command::new("curl")
        .args(["-fSL", "--retry", "3", "-o"])
        .arg(output)
        .arg(url)
        .status()
        .map_err(|e| format!("コマンド実行に失敗しました (curl): {e}"))?;
    if !status.success() {
        return Err(format!("ダウンロードに失敗しました: {url}"));
    }
    Ok(())
}

/// ファイルの SHA256 チェックサムを検証する。
///
/// 引数:
/// - `file_path`: 検証対象ファイル。
/// - `sha256_path`: 期待値を含む `.sha256` ファイル。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(())`: チェックサム一致。
/// - `Err(String)`: 期待値読込失敗、パース失敗、または不一致。
///
/// 副作用:
/// - ファイルを読み込む。
/// - 検証成功時に標準エラーへログを出力する。
fn verify_sha256(file_path: &Path, sha256_path: &Path) -> Result<(), String> {
    let expected = fs::read_to_string(sha256_path)
        .map_err(|e| format!("SHA256 チェックサムファイルの読み込みに失敗: {}", e))?
        .split_whitespace()
        .next()
        .ok_or("SHA256 チェックサムファイルが空です")?
        .to_lowercase();

    let actual = compute_sha256(file_path)?;
    if actual != expected {
        return Err(format!(
            "SHA256 チェックサムが一致しません:\n  expected: {}\n  actual:   {}",
            expected, actual
        ));
    }
    eprintln!("SHA256 チェックサム検証成功: {}", actual);
    Ok(())
}

/// ホスト OS に応じたコマンドで SHA256 ハッシュを計算する。
///
/// 引数:
/// - `path`: ハッシュ計算対象ファイル。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(String)`: 小文字 16 進の SHA256 値。
/// - `Err(String)`: 実行失敗、非 0 終了、または出力パース失敗。
///
/// 副作用:
/// - 外部コマンド（`sha256sum` / `shasum` / `certutil`）を実行する。
fn compute_sha256(path: &Path) -> Result<String, String> {
    let output = if cfg!(target_os = "macos") {
        std::process::Command::new("shasum")
            .args(["-a", "256"])
            .arg(path)
            .output()
    } else if cfg!(target_os = "windows") {
        std::process::Command::new("certutil")
            .args(["-hashfile"])
            .arg(path)
            .arg("SHA256")
            .output()
    } else {
        std::process::Command::new("sha256sum").arg(path).output()
    }
    .map_err(|e| format!("SHA256 計算コマンドの実行に失敗: {}", e))?;

    if !output.status.success() {
        return Err("SHA256 チェックサムの計算に失敗しました".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if cfg!(target_os = "windows") {
        // certutil -hashfile の出力は3行:
        //   SHA256 hash of <file>:
        //   <hex hash>
        //   CertUtil: -hashfile command completed successfully.
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() < 2 {
            return Err("certutil の出力のパースに失敗しました".to_string());
        }
        Ok(lines[1].trim().replace(' ', "").to_ascii_lowercase())
    } else {
        stdout
            .split_whitespace()
            .next()
            .map(|s| s.to_lowercase())
            .ok_or_else(|| "SHA256 出力のパースに失敗しました".to_string())
    }
}

/// アーカイブを展開する。
///
/// 引数:
/// - `archive`: 展開対象アーカイブ。
/// - `dest`: 展開先ディレクトリ。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(())`: 展開成功。
/// - `Err(String)`: 展開コマンド実行失敗または非 0 終了。
///
/// 副作用:
/// - 外部コマンド `tar` または `unzip` を実行する。
/// - `dest` 配下へファイルを作成する。
fn extract(archive: &Path, dest: &Path) -> Result<(), String> {
    let is_zip = archive
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("zip"));

    // .zip ファイルで Windows 以外なら unzip を使う
    // それ以外（.zip でないまたは Windows） の場合は tar を使う
    //
    // ※Windows は tar コマンドで zip が展開できる
    // ※macOS も tar コマンドで zip が展開できるはずだが、念のため unzip を利用する
    let (command_name, status) = if is_zip && !cfg!(target_os = "windows") {
        (
            "unzip",
            Command::new("unzip")
                .arg("-q")
                .arg(archive)
                .arg("-d")
                .arg(dest)
                .status(),
        )
    } else {
        (
            "tar",
            Command::new("tar")
                .arg("xf")
                .arg(archive)
                .arg("-C")
                .arg(dest)
                .status(),
        )
    };
    let status = status.map_err(|e| format!("コマンド実行に失敗しました ({command_name}): {e}"))?;
    if !status.success() {
        return Err(format!(
            "アーカイブ展開に失敗しました ({command_name}): {}",
            archive.display()
        ));
    }
    Ok(())
}

/// Cargo のターゲット情報から内部ターゲット名を決定する。
///
/// 引数: なし。
///
/// 環境変数依存:
/// - `WEBRTC_C_TARGET`（設定されていれば最優先）。
/// - `CARGO_CFG_TARGET_OS`
/// - `CARGO_CFG_TARGET_ARCH`
///
/// 戻り値:
/// - `linux` / `macos` / `windows` / `ios` / `android` 向け内部ターゲット名。
///
/// 副作用:
/// - サポート外の組み合わせでは panic する。
fn get_target_platform() -> String {
    // 環境変数で明示的に指定されている場合はそちらを優先する
    if let Ok(target) = env::var("WEBRTC_C_TARGET") {
        return target;
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    match (target_os.as_str(), target_arch.as_str()) {
        ("linux", "x86_64") => format!("{}_x86_64", detect_linux_distro()),
        ("linux", "aarch64") => format!("{}_armv8", detect_linux_distro()),
        ("macos", "aarch64") => "macos_arm64".to_string(),
        ("windows", "x86_64") => "windows_x86_64".to_string(),
        ("ios", "aarch64") => "ios_arm64".to_string(),
        ("android", "aarch64") => "android_arm64".to_string(),
        _ => panic!(
            "サポートされていないターゲットです: os={}, arch={}",
            target_os, target_arch
        ),
    }
}

/// `/etc/rpi-issue` や `/etc/os-release` から Linux ディストリビューション識別子を解決する。
///
/// 引数: なし。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `ubuntu-22.04`
/// - `ubuntu-24.04`
/// - `raspberry-pi-os`
///
/// 副作用:
/// - `/etc/os-release` を読み込む。
/// - サポート外の場合は panic する。
fn detect_linux_distro() -> String {
    if Path::new("/etc/rpi-issue").exists() {
        return "raspberry-pi-os".to_string();
    }

    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(version) = line.strip_prefix("VERSION_ID=") {
                let version = version.trim_matches('"');
                match version {
                    "22.04" | "24.04" => return format!("ubuntu-{}", version),
                    _ => {}
                }
            }
        }
    }
    panic!(
        "サポートされていない Linux ディストリビューションです。\
         WEBRTC_C_TARGET 環境変数で明示的にターゲットを指定してください"
    );
}

/// `shiguredo_cmake` を使って `webrtc_c` をソースビルドし、静的ライブラリを配置する。
///
/// 引数:
/// - `webrtc_dir`: C/C++ 側ソースディレクトリ。
/// - `target_platform`: 内部ターゲット名。
/// - `out_dir`: Cargo の出力先ディレクトリ。
/// - `build_metadata`: `Cargo.toml` 由来のビルド設定。
///
/// 環境変数依存:
/// - `WEBRTC_C_SYSROOT`（任意）。
/// - Android 時に `ANDROID_NDK_HOME` / `ANDROID_NDK`。
///
/// 戻り値:
/// - `out_dir/lib` にコピーした静的ライブラリのパス。
///
/// 副作用:
/// - CMake を実行してビルドを行う。
/// - `out_dir` 配下へビルド成果物を作成し、ファイルコピーを行う。
/// - 設定不整合時に panic する。
fn build_webrtc_c(
    webrtc_dir: &Path,
    target_platform: &str,
    out_dir: &Path,
    build_metadata: &BuildMetadata,
) -> PathBuf {
    let mut config = shiguredo_cmake::Config::new(webrtc_dir);
    let profile = "release";
    shiguredo_cmake::set_cmake_env();

    // 配布されている libwebrtc は Release 相当のため、ラッパー側も Release で揃える
    config.profile("Release");
    config.out_dir(out_dir.join("_build").join(target_platform).join(profile));

    // ターゲットプラットフォームを設定
    config
        .define("WEBRTC_C_TARGET", target_platform)
        .define("WEBRTC_BUILD_VERSION", &build_metadata.webrtc_build_version)
        .define("WEBRTC_BASE_URL", &build_metadata.webrtc_base_url)
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON");

    // WEBRTC_C_SYSROOT が設定されていれば CMake に渡す
    if let Ok(sysroot) = env::var("WEBRTC_C_SYSROOT") {
        config.define("WEBRTC_C_SYSROOT", &sysroot);
    }

    // iOS クロスコンパイル設定
    if target_platform == "ios_arm64" {
        config.define("CMAKE_SYSTEM_NAME", "iOS");
        config.define("CMAKE_OSX_ARCHITECTURES", "arm64");
        config.define(
            "CMAKE_OSX_DEPLOYMENT_TARGET",
            &build_metadata.cmake_osx_deployment_target,
        );
    }

    // Android NDK ツールチェーンの設定
    if target_platform == "android_arm64" {
        let ndk = resolve_android_ndk_from_env().expect(
            "ANDROID_NDK_HOME または ANDROID_NDK には有効な Android NDK パスを指定してください",
        );
        let ndk_toolchain_file = android_ndk_toolchain_file(&ndk);
        if !ndk_toolchain_file.exists() {
            panic!(
                "Android NDK toolchain file が見つかりません: {}",
                ndk_toolchain_file.display()
            );
        }
        let android_toolchain_file = webrtc_dir.join("android.toolchain.cmake");
        if !android_toolchain_file.exists() {
            panic!(
                "Android ツールチェーン上書きファイルが見つかりません: {}",
                android_toolchain_file.display()
            );
        }
        let postfix = if cfg!(target_os = "windows") {
            ".exe"
        } else {
            ""
        };
        let llvm_clang_dir = out_dir
            .join("_build")
            .join(target_platform)
            .join(profile)
            .join("build")
            .join("_deps")
            .join("llvm")
            .join("clang")
            .join("bin");
        let override_c_compiler = llvm_clang_dir.join(format!("clang{postfix}"));
        let override_cxx_compiler = llvm_clang_dir.join(format!("clang++{postfix}"));

        config.define(
            "CMAKE_TOOLCHAIN_FILE",
            android_toolchain_file.to_str().unwrap(),
        );
        config.define(
            "ANDROID_OVERRIDE_TOOLCHAIN_FILE",
            ndk_toolchain_file.to_str().unwrap(),
        );
        config.define(
            "ANDROID_OVERRIDE_C_COMPILER",
            override_c_compiler.to_str().unwrap(),
        );
        config.define(
            "ANDROID_OVERRIDE_CXX_COMPILER",
            override_cxx_compiler.to_str().unwrap(),
        );
        config.define("CMAKE_TRY_COMPILE_TARGET_TYPE", "STATIC_LIBRARY");
        config.define("ANDROID_ABI", "arm64-v8a");
        config.define("ANDROID_PLATFORM", &build_metadata.android_platform);
    }

    // bundled_webrtc_c_bundling ターゲットのみをビルド（all ターゲットを避ける）
    let dst = config.build_target("bundled_webrtc_c_bundling").build();

    // ビルド結果から静的ライブラリを OUT_DIR にコピー
    let static_lib_name = static_library_filename_for_target(target_platform);
    let build_dir = dst.join("build");
    let bundled_lib = build_dir.join("bundled").join(static_lib_name);
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).expect("lib ディレクトリの作成に失敗しました");
    let dest_lib = lib_dir.join(static_lib_name);
    fs::copy(&bundled_lib, &dest_lib).expect("静的ライブラリのコピーに失敗しました");

    dest_lib
}

/// ターゲットが Windows 系かどうかを判定する。
///
/// 引数:
/// - `target_platform`: 内部ターゲット名。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `true`: Windows 系ターゲット。
/// - `false`: それ以外。
///
/// 副作用: なし。
fn is_windows_target(target_platform: &str) -> bool {
    target_platform.starts_with("windows_")
}

/// ターゲットに対応する静的ライブラリ名を返す。
///
/// 引数:
/// - `target_platform`: 内部ターゲット名。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - Windows では `webrtc_c.lib`、それ以外は `libwebrtc_c.a`。
///
/// 副作用: なし。
fn static_library_filename_for_target(target_platform: &str) -> &'static str {
    if is_windows_target(target_platform) {
        "webrtc_c.lib"
    } else {
        "libwebrtc_c.a"
    }
}

/// Windows で `bindgen` 用 `libclang` を利用可能にする。
///
/// 引数:
/// - `out_dir`: Cargo の出力先ディレクトリ（`vswhere.exe` の配置先に使用）。
///
/// 環境変数依存:
/// - `LIBCLANG_PATH`（設定済みなら何もしない）。
///
/// 戻り値: なし。
///
/// 副作用:
/// - 必要に応じて `vswhere.exe` をダウンロードする。
/// - Visual Studio を検出し、`LIBCLANG_PATH` を設定する。
/// - 前提が満たせない場合は panic する。
fn ensure_windows_libclang(out_dir: &Path) {
    if env::var_os("LIBCLANG_PATH").is_some() {
        return;
    }

    let tools_dir = out_dir.join("tools");
    fs::create_dir_all(&tools_dir).expect("tools ディレクトリの作成に失敗しました");
    let vswhere_path = tools_dir.join("vswhere.exe");
    if !vswhere_path.exists() {
        download(
            "https://github.com/microsoft/vswhere/releases/latest/download/vswhere.exe",
            &vswhere_path,
        )
        .expect("vswhere.exe のダウンロードに失敗しました");
    }

    let installation_path = find_visual_studio_installation(&vswhere_path).expect(
        "Visual Studio 2022 の検出に失敗しました。\
         Visual Studio 2022 に `Desktop development with C++` と \
         `C++ Clang tools for Windows (Microsoft.VisualStudio.Component.VC.Llvm.Clang)` \
         をインストールしてください。",
    );
    let libclang_dir = PathBuf::from(installation_path)
        .join("VC")
        .join("Tools")
        .join("Llvm")
        .join("x64")
        .join("bin");
    let libclang_dll = libclang_dir.join("libclang.dll");
    if !libclang_dll.exists() {
        panic!(
            "Visual Studio の LLVM コンポーネントが見つかりません: {}\n\
             Visual Studio 2022 に `C++ Clang tools for Windows \
             (Microsoft.VisualStudio.Component.VC.Llvm.Clang)` をインストールしてください。",
            libclang_dll.display()
        );
    }

    // bindgen が libclang.dll を見つけられるように環境変数を設定する。
    unsafe { env::set_var("LIBCLANG_PATH", &libclang_dir) };
}

/// `vswhere.exe` を実行して Visual Studio のインストール先を検出する。
///
/// 引数:
/// - `vswhere_path`: 実行する `vswhere.exe` のパス。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(String)`: 検出したインストールディレクトリ。
/// - `Err(String)`: `vswhere` 実行失敗、非 0 終了、または検出結果なし。
///
/// 副作用:
/// - 外部コマンド `vswhere.exe` を実行する。
fn find_visual_studio_installation(vswhere_path: &Path) -> Result<String, String> {
    let output = std::process::Command::new(vswhere_path)
        .args([
            "-latest",
            "-products",
            "*",
            "-version",
            "[17.0,18.0)",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Llvm.Clang",
            "-property",
            "installationPath",
        ])
        .output()
        .map_err(|e| format!("vswhere.exe の実行に失敗しました: {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "vswhere.exe が失敗しました: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| "Visual Studio 2022 のインストールが見つかりません".to_string())
}

/// `local-export` 有効時に `webrtc/_build` への参照リンクを整備する。
///
/// 引数:
/// - `webrtc_dir`: `webrtc` ディレクトリ。
/// - `out_dir`: Cargo の出力先ディレクトリ。
///
/// 環境変数依存:
/// - `CARGO_FEATURE_LOCAL_EXPORT`（未設定なら何もしない）。
///
/// 戻り値: なし。
///
/// 副作用:
/// - ディレクトリ作成、既存リンクやファイルの削除、リンク作成を行う。
/// - 失敗時に panic する。
fn maybe_export_local_build_dir(webrtc_dir: &Path, out_dir: &Path) {
    if env::var_os("CARGO_FEATURE_LOCAL_EXPORT").is_none() {
        return;
    }

    let build_dir = out_dir.join("_build");
    let link_path = webrtc_dir.join("_build");
    fs::create_dir_all(&build_dir).expect("_build ディレクトリの作成に失敗しました");

    if let Ok(metadata) = fs::symlink_metadata(&link_path) {
        if is_directory_link(&metadata) {
            if paths_point_to_same_location(&link_path, &build_dir) {
                return;
            }
            remove_directory_link(&link_path)
                .expect("既存 webrtc/_build リンクの削除に失敗しました");
        } else if metadata.is_dir() {
            fs::remove_dir_all(&link_path)
                .expect("既存 webrtc/_build ディレクトリの削除に失敗しました");
        } else {
            fs::remove_file(&link_path).expect("既存 webrtc/_build ファイルの削除に失敗しました");
        }
    }

    if let Err(err) = create_directory_link(&build_dir, &link_path) {
        panic!(
            "webrtc/_build リンクの作成に失敗しました: {} -> {} ({})",
            link_path.display(),
            build_dir.display(),
            err
        );
    }
}

/// Unix 系環境でディレクトリシンボリックリンクを作成する。
///
/// 引数:
/// - `target`: リンク先ディレクトリ。
/// - `link_path`: 作成するリンクパス。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(())`: 作成成功。
/// - `Err(std::io::Error)`: 作成失敗。
///
/// 副作用:
/// - ファイルシステム上にシンボリックリンクを作成する。
#[cfg(unix)]
fn create_directory_link(target: &Path, link_path: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link_path)
}

/// Windows 環境でジャンクション（`mklink /J`）を作成する。
///
/// 引数:
/// - `target`: リンク先ディレクトリ。
/// - `link_path`: 作成するリンクパス。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(())`: 作成成功。
/// - `Err(std::io::Error)`: `mklink` 実行失敗または非 0 終了。
///
/// 副作用:
/// - 外部コマンド `cmd /C mklink /J` を実行する。
/// - ファイルシステム上にジャンクションを作成する。
#[cfg(windows)]
fn create_directory_link(target: &Path, link_path: &Path) -> std::io::Result<()> {
    // Windows では権限要求の少ないジャンクションを使う。
    let status = std::process::Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(link_path)
        .arg(target)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "mklink /J の実行に失敗しました: {}",
            status
        )))
    }
}

/// Unix 系環境でディレクトリリンクを削除する。
///
/// 引数:
/// - `link_path`: 削除対象リンク。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(())`: 削除成功。
/// - `Err(std::io::Error)`: 削除失敗。
///
/// 副作用:
/// - ファイルシステム上のリンクを削除する。
#[cfg(unix)]
fn remove_directory_link(link_path: &Path) -> std::io::Result<()> {
    fs::remove_file(link_path)
}

/// Windows 環境でディレクトリリンク（ジャンクション）を削除する。
///
/// 引数:
/// - `link_path`: 削除対象リンク。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `Ok(())`: 削除成功。
/// - `Err(std::io::Error)`: 削除失敗。
///
/// 副作用:
/// - ファイルシステム上のリンクを削除する。
#[cfg(windows)]
fn remove_directory_link(link_path: &Path) -> std::io::Result<()> {
    fs::remove_dir(link_path)
}

/// 2 つのパスが同じ実体を指しているかを判定する。
///
/// 引数:
/// - `a`: 比較対象パス 1。
/// - `b`: 比較対象パス 2。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `true`: 同一実体を指す。
/// - `false`: 異なる、または正規化失敗。
///
/// 副作用:
/// - `fs::canonicalize` によりファイルシステム参照を行う。
fn paths_point_to_same_location(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

/// Unix 系環境でメタデータがディレクトリリンクかどうかを判定する。
///
/// 引数:
/// - `metadata`: 判定対象メタデータ。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `true`: シンボリックリンク。
/// - `false`: それ以外。
///
/// 副作用: なし。
#[cfg(unix)]
fn is_directory_link(metadata: &std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

/// Windows 環境でメタデータが再解析ポイントかどうかを判定する。
///
/// 引数:
/// - `metadata`: 判定対象メタデータ。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - `true`: 再解析ポイント（リンク相当）。
/// - `false`: それ以外。
///
/// 副作用: なし。
#[cfg(windows)]
fn is_directory_link(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    (metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0
}

/// C ヘッダーから Rust FFI バインディングを生成して `OUT_DIR` に書き込む。
///
/// 引数:
/// - `header`: `bindgen` 入力ヘッダー。
/// - `include_dir`: 追加インクルードディレクトリ。
///
/// 環境変数依存:
/// - `OUT_DIR`
/// - `CARGO_CFG_TARGET_OS`
/// - `CARGO_CFG_TARGET_ARCH`
/// - Android 時に `ANDROID_NDK_HOME` / `ANDROID_NDK`
///
/// 戻り値: なし。
///
/// 副作用:
/// - `cargo:rerun-if-changed` を標準出力へ出力する。
/// - `bindgen` を実行する。
/// - `OUT_DIR/bindings.rs` を作成または上書きする。
/// - 前提不備時に panic する。
fn generate_bindings(header: &Path, include_dir: &Path) {
    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", include_dir.display());

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR の取得に失敗しました"))
        .join("bindings.rs");

    let mut builder = bindgen::Builder::default()
        .header(
            header
                .to_str()
                .expect("ヘッダーパスを文字列に変換できませんでした"),
        )
        .clang_arg(format!("-I{}", include_dir.display()))
        .layout_tests(false);

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_os == "android" {
        let ndk = resolve_android_ndk_from_env().expect(
            "ANDROID_NDK_HOME または ANDROID_NDK には有効な Android NDK パスを指定してください",
        );
        let sysroot = resolve_android_ndk_sysroot(&ndk);
        let target = android_clang_target(&target_arch);
        let include = sysroot.join("usr").join("include");
        let target_include = include.join(target);
        builder = builder
            .header(
                header
                    .parent()
                    .unwrap()
                    .join("webrtc_c")
                    .join("android.h")
                    .to_str()
                    .unwrap(),
            )
            .clang_arg(format!("--target={target}"))
            .clang_arg(format!("--sysroot={}", sysroot.display()))
            .clang_arg(format!("-isystem{}", include.display()));
        if target_include.exists() {
            builder = builder.clang_arg(format!("-isystem{}", target_include.display()));
        }
    }

    let bindings = builder
        .generate()
        .expect("bindgen によるバインディング生成に失敗しました");

    fs::write(&out_path, bindings.to_string()).expect("バインディングの書き込みに失敗しました");
}

/// Android 用 `clang --target` 値をターゲットアーキテクチャから解決する。
///
/// 引数:
/// - `target_arch`: Cargo のアーキテクチャ文字列。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - 対応する Android ターゲットトリプル文字列。
///
/// 副作用:
/// - 未対応アーキテクチャでは panic する。
fn android_clang_target(target_arch: &str) -> &'static str {
    match target_arch {
        "aarch64" => "aarch64-linux-android",
        "arm" => "armv7a-linux-androideabi",
        "x86_64" => "x86_64-linux-android",
        "x86" => "i686-linux-android",
        _ => panic!("unsupported android target arch: {}", target_arch),
    }
}

/// Android NDK から `sysroot` ディレクトリを解決する。
///
/// 引数:
/// - `ndk`: Android NDK ルート。
///
/// 環境変数依存: なし。
///
/// 戻り値:
/// - 利用可能な prebuilt に対応する `sysroot` パス。
///
/// 副作用:
/// - ファイルシステムを探索する。
/// - 必須ディレクトリが無い場合に panic する。
fn resolve_android_ndk_sysroot(ndk: &Path) -> PathBuf {
    let prebuilt = ndk.join("toolchains").join("llvm").join("prebuilt");
    let selected = [
        "linux-x86_64",
        "darwin-x86_64",
        "darwin-arm64",
        "windows-x86_64",
    ]
    .iter()
    .map(|name| prebuilt.join(name))
    .find(|path| path.exists())
    .expect("android ndk prebuilt dir not found");
    let sysroot = selected.join("sysroot");
    if !sysroot.exists() {
        panic!("android ndk sysroot not found: {}", sysroot.display());
    }
    sysroot
}

/// Cargo へリンク設定を通知する `cargo:` ディレクティブを出力する。
///
/// 引数:
/// - `lib_path`: 生成済み静的ライブラリのパス。
///
/// 環境変数依存:
/// - `CARGO_CFG_TARGET_OS`
///
/// 戻り値: なし。
///
/// 副作用:
/// - 標準出力へ `cargo:rustc-link-*` 行を出力する。
/// - ターゲット OS 不正時に panic する。
fn emit_link_directives(lib_path: &Path) {
    let lib_dir = lib_path
        .parent()
        .expect("静的ライブラリの親ディレクトリ取得に失敗しました");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=webrtc_c");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "linux" => {
            for lib in ["m", "dl", "rt", "X11", "pthread"] {
                println!("cargo:rustc-link-lib={lib}");
            }
        }
        "macos" => {
            println!("cargo:rustc-link-lib=c++");
            for framework in [
                "AVFoundation",
                "AppKit",
                "AudioToolbox",
                "CoreAudio",
                "CoreMedia",
                "IOSurface",
                "Metal",
                "MetalKit",
                "OpenGL",
                "QuartzCore",
                "ScreenCaptureKit",
                "VideoToolbox",
            ] {
                println!("cargo:rustc-link-lib=framework={framework}");
            }
        }
        "windows" => {
            for lib in [
                "winmm",
                "ws2_32",
                "strmiids",
                "dmoguids",
                "iphlpapi",
                "msdmo",
                "secur32",
                "wmcodecdspuuid",
            ] {
                println!("cargo:rustc-link-lib={lib}");
            }
        }
        "ios" => {
            println!("cargo:rustc-link-lib=c++");
            for framework in [
                "Foundation",
                "AVFoundation",
                "CoreAudio",
                "AudioToolbox",
                "CoreMedia",
                "CoreVideo",
                "CoreGraphics",
                "CoreFoundation",
                "VideoToolbox",
                "Security",
                "Metal",
                "IOSurface",
                "QuartzCore",
                "UIKit",
            ] {
                println!("cargo:rustc-link-lib=framework={framework}");
            }
        }
        "android" => {
            for lib in ["log", "OpenSLES", "m", "dl"] {
                println!("cargo:rustc-link-lib={lib}");
            }
        }
        _ => panic!("サポートされていないターゲット OS です: {}", target_os),
    }
}

/// Cargo が提供する `OUT_DIR` を `PathBuf` として取得する。
///
/// 引数: なし。
///
/// 環境変数依存:
/// - `OUT_DIR`
///
/// 戻り値:
/// - `OUT_DIR` のパス。
///
/// 副作用:
/// - `OUT_DIR` 未設定時に panic する。
fn get_out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR の取得に失敗しました"))
}
