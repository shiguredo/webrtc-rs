use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Cargo.toml から外部依存ライブラリの情報を取得する
fn get_webrtc_build_metadata() -> (String, String) {
    let cargo_toml = shiguredo_toml::from_str(include_str!("Cargo.toml"))
        .expect("Cargo.toml のパースに失敗しました");
    let metadata = shiguredo_toml::Value::Table(cargo_toml);
    let webrtc_build = metadata
        .get("package")
        .and_then(|v| v.get("metadata"))
        .and_then(|v| v.get("external-dependencies"))
        .and_then(|v| v.get("webrtc-build"))
        .expect(
            "Cargo.toml に [package.metadata.external-dependencies.webrtc-build] が見つかりません",
        );

    let version = webrtc_build
        .get("version")
        .and_then(|v| v.as_str())
        .expect("webrtc-build.version が見つかりません")
        .to_string();
    let base_url = webrtc_build
        .get("base-url")
        .and_then(|v| v.as_str())
        .expect("webrtc-build.base-url が見つかりません")
        .to_string();

    (version, base_url)
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

    // ソースファイルの変更を監視
    println!(
        "cargo:rerun-if-changed={}",
        webrtc_dir.join("src").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        webrtc_dir.join("CMakeLists.txt").display()
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
        build_from_source(&webrtc_dir, &target_platform, &out_dir)
    };

    emit_link_directives(&lib_path);
}

struct PrebuiltPaths {
    lib_path: PathBuf,
}

/// prebuilt バイナリを使用するかどうかを判定する
fn should_use_prebuilt() -> bool {
    // source-build feature が有効 → ソースビルド
    if env::var("CARGO_FEATURE_SOURCE_BUILD").is_ok() {
        return false;
    }
    // デフォルトで prebuilt を試みる
    true
}

/// ソースからビルドする（CMake + bindgen）
fn build_from_source(webrtc_dir: &Path, target_platform: &str, out_dir: &Path) -> PathBuf {
    let header = webrtc_dir.join("src").join("webrtc_c.h");
    let include_dir = webrtc_dir.join("src");
    let lib_path = build_webrtc_c(webrtc_dir, target_platform, out_dir);
    maybe_export_local_build_dir(webrtc_dir, out_dir);
    if is_windows_target(target_platform) {
        ensure_windows_libclang(out_dir);
    }
    generate_bindings(&header, &include_dir);
    lib_path
}

/// prebuilt バイナリをダウンロードして展開する
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
    .map_err(|e| format!("bindings.rs のコピーに失敗: {}", e))?;

    Ok(PrebuiltPaths { lib_path })
}

/// curl を使ってファイルをダウンロードする
fn download(url: &str, output: &Path) -> Result<(), String> {
    let status = std::process::Command::new("curl")
        .args(["-fSL", "--retry", "3", "-o"])
        .arg(output)
        .arg(url)
        .status()
        .map_err(|e| format!("curl の実行に失敗しました: {}", e))?;
    if !status.success() {
        return Err(format!("ダウンロードに失敗しました: {}", url));
    }
    Ok(())
}

/// SHA256 チェックサムを検証する
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

/// ファイルの SHA256 ハッシュを計算する
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

/// tar を使ってアーカイブを展開する
fn extract(archive: &Path, dest: &Path) -> Result<(), String> {
    let status = std::process::Command::new("tar")
        .arg("xzf")
        .arg(archive)
        .arg("-C")
        .arg(dest)
        .status()
        .map_err(|e| format!("tar の実行に失敗しました: {}", e))?;
    if !status.success() {
        return Err(format!("展開に失敗しました: {}", archive.display()));
    }
    Ok(())
}

/// ターゲットプラットフォーム名を取得する
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

/// /etc/os-release から Linux ディストリビューションのバージョンを検出する
fn detect_linux_distro() -> String {
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

/// shiguredo_cmake crate を使って webrtc_c をビルドする
fn build_webrtc_c(webrtc_dir: &Path, target_platform: &str, out_dir: &Path) -> PathBuf {
    let mut config = shiguredo_cmake::Config::new(webrtc_dir);
    let profile = "release";
    shiguredo_cmake::set_cmake_env();

    // 配布されている libwebrtc は Release 相当のため、ラッパー側も Release で揃える
    config.profile("Release");
    config.out_dir(out_dir.join("_build").join(target_platform).join(profile));

    // Cargo.toml から WebRTC ビルド情報を取得して CMake に渡す
    let (webrtc_build_version, webrtc_base_url) = get_webrtc_build_metadata();

    // ターゲットプラットフォームを設定（CMakeLists.txt 内で自動検出もされるが明示的に指定）
    config
        .define("WEBRTC_C_TARGET", target_platform)
        .define("WEBRTC_BUILD_VERSION", &webrtc_build_version)
        .define("WEBRTC_BASE_URL", &webrtc_base_url)
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
        config.define("CMAKE_OSX_DEPLOYMENT_TARGET", "16.0");
    }

    // Android NDK ツールチェーンの設定
    if target_platform == "android_arm64" {
        let ndk = env::var("ANDROID_NDK_HOME")
            .or_else(|_| env::var("ANDROID_NDK"))
            .expect(
                "ANDROID_NDK_HOME または ANDROID_NDK 環境変数が必要です。\
                 Android NDK r29+ のパスを設定してください",
            );
        let toolchain_file = PathBuf::from(&ndk)
            .join("build")
            .join("cmake")
            .join("android.toolchain.cmake");
        if !toolchain_file.exists() {
            panic!(
                "Android NDK toolchain file が見つかりません: {}",
                toolchain_file.display()
            );
        }
        config.define("CMAKE_TOOLCHAIN_FILE", toolchain_file.to_str().unwrap());
        config.define("ANDROID_ABI", "arm64-v8a");
        config.define("ANDROID_PLATFORM", "android-24");
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
    fs::copy(&bundled_lib, &dest_lib).unwrap_or_else(|e| {
        panic!(
            "{} のコピーに失敗しました: src={}, dst={}, error={}",
            static_lib_name,
            bundled_lib.display(),
            dest_lib.display(),
            e
        )
    });

    dest_lib
}

fn is_windows_target(target_platform: &str) -> bool {
    target_platform.starts_with("windows_")
}

fn static_library_filename_for_target(target_platform: &str) -> &'static str {
    if is_windows_target(target_platform) {
        "webrtc_c.lib"
    } else {
        "libwebrtc_c.a"
    }
}

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

    let installation_path = find_visual_studio_installation(&vswhere_path).unwrap_or_else(|e| {
        panic!(
            "Visual Studio 2022 の検出に失敗しました: {}\n\
             Visual Studio 2022 に `Desktop development with C++` と \
             `C++ Clang tools for Windows (Microsoft.VisualStudio.Component.VC.Llvm.Clang)` \
             をインストールしてください。",
            e
        )
    });
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

#[cfg(unix)]
fn create_directory_link(target: &Path, link_path: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link_path)
}

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

#[cfg(unix)]
fn remove_directory_link(link_path: &Path) -> std::io::Result<()> {
    fs::remove_file(link_path)
}

#[cfg(windows)]
fn remove_directory_link(link_path: &Path) -> std::io::Result<()> {
    fs::remove_dir(link_path)
}

fn paths_point_to_same_location(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

#[cfg(unix)]
fn is_directory_link(metadata: &std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(windows)]
fn is_directory_link(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    (metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0
}

fn generate_bindings(header: &Path, include_dir: &Path) {
    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", include_dir.display());

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR の取得に失敗しました"))
        .join("bindings.rs");

    let bindings = bindgen::Builder::default()
        .header(
            header
                .to_str()
                .expect("ヘッダーパスを文字列に変換できませんでした"),
        )
        .clang_arg(format!("-I{}", include_dir.display()))
        .layout_tests(false)
        .generate()
        .expect("bindgen によるバインディング生成に失敗しました");

    fs::write(&out_path, bindings.to_string()).expect("バインディングの書き込みに失敗しました");
}

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
                "Cocoa",
                "AppKit",
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

fn get_out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR の取得に失敗しました"))
}
