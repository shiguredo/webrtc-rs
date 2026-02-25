use std::env;
use std::fs;
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use aws_lc_rs::digest::{Context, SHA256};
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use rustls_platform_verifier::ConfigVerifierExt;
use shiguredo_http11::{DecoderLimits, Request, Response, ResponseDecoder};

const CMAKE_VERSION: &str = "4.2.3";

fn main() {
    // Cargo.toml か build.rs が更新されたら、依存ライブラリを再ビルドする
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_LOCAL_EXPORT");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_SOURCE_BUILD");
    println!("cargo::rerun-if-env-changed=WEBRTC_C_TARGET");
    println!("cargo::rerun-if-env-changed=WEBRTC_C_SYSROOT");

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
    println!(
        "cargo:rerun-if-changed={}",
        webrtc_dir.join("deps.json").display()
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
    generate_bindings(&header, &include_dir);
    lib_path
}

/// prebuilt バイナリをダウンロードして展開する
fn download_prebuilt(target: &str, out_dir: &Path) -> Result<PrebuiltPaths, String> {
    let version = env::var("CARGO_PKG_VERSION").map_err(|e| e.to_string())?;
    let url = format!(
        "https://github.com/shiguredo/webrtc-rs/releases/download/{}/libwebrtc_c-{}.tar.gz",
        version, target
    );

    eprintln!("prebuilt ライブラリをダウンロード中: {}", url);

    let archive_bytes = fetch_url(&url)?;

    // OUT_DIR/prebuilt/ に展開
    let prebuilt_dir = out_dir.join("prebuilt");
    extract_tar_gz_from_bytes(&archive_bytes, &prebuilt_dir)?;

    // libwebrtc_c.a を OUT_DIR/lib/ にコピー
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).map_err(|e| format!("lib ディレクトリ作成に失敗: {}", e))?;
    let lib_path = lib_dir.join("libwebrtc_c.a");
    fs::copy(prebuilt_dir.join("lib").join("libwebrtc_c.a"), &lib_path)
        .map_err(|e| format!("libwebrtc_c.a のコピーに失敗: {}", e))?;

    // bindgen 生成済みの bindings.rs を OUT_DIR/ にコピー
    // （利用者が libclang-dev をインストールしなくて済むようにするため）
    fs::copy(
        prebuilt_dir.join("bindings.rs"),
        out_dir.join("bindings.rs"),
    )
    .map_err(|e| format!("bindings.rs のコピーに失敗: {}", e))?;

    Ok(PrebuiltPaths { lib_path })
}

/// バイト列から tar.gz を展開する
fn extract_tar_gz_from_bytes(data: &[u8], dest_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(dest_dir).map_err(|e| format!("展開先ディレクトリ作成に失敗: {}", e))?;
    let decoder = flate2::read::GzDecoder::new(Cursor::new(data));
    let mut archive = tar::Archive::new(decoder);
    let entries = archive
        .entries()
        .map_err(|e| format!("tar エントリ取得に失敗: {}", e))?;

    for entry in entries {
        let mut entry = entry.map_err(|e| format!("tar エントリ読み込みに失敗: {}", e))?;
        let path = entry
            .path()
            .map_err(|e| format!("tar パス取得に失敗: {}", e))?;
        if !is_safe_path(&path) {
            return Err(format!("不正なパスが含まれています: {}", path.display()));
        }
        let out_path = dest_dir.join(&*path);
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("展開先ディレクトリ作成に失敗: {}", e))?;
        }
        entry
            .unpack(&out_path)
            .map_err(|e| format!("tar 展開に失敗: {}", e))?;
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

/// cmake crate を使って webrtc_c をビルドする
fn build_webrtc_c(webrtc_dir: &Path, target_platform: &str, out_dir: &Path) -> PathBuf {
    let mut config = cmake::Config::new(webrtc_dir);
    let profile = "release";
    let cmake_path = ensure_cmake(out_dir);
    set_cmake_env(&cmake_path);

    // 配布されている libwebrtc は Release 相当のため、ラッパー側も Release で揃える
    config.profile("Release");
    config.out_dir(out_dir.join("_build").join(target_platform).join(profile));

    // ターゲットプラットフォームを設定（CMakeLists.txt 内で自動検出もされるが明示的に指定）
    config
        .define("WEBRTC_C_TARGET", target_platform)
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON");

    // WEBRTC_C_SYSROOT が設定されていれば CMake に渡す
    if let Ok(sysroot) = env::var("WEBRTC_C_SYSROOT") {
        config.define("WEBRTC_C_SYSROOT", &sysroot);
    }

    // bundled_webrtc_c_bundling ターゲットのみをビルド（all ターゲットを避ける）
    let dst = config.build_target("bundled_webrtc_c_bundling").build();

    // ビルド結果から libwebrtc_c.a を OUT_DIR にコピー
    let build_dir = dst.join("build");
    let bundled_lib = build_dir.join("bundled").join("libwebrtc_c.a");
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).expect("lib ディレクトリの作成に失敗しました");
    let dest_lib = lib_dir.join("libwebrtc_c.a");
    fs::copy(&bundled_lib, &dest_lib).expect("libwebrtc_c.a のコピーに失敗しました");

    dest_lib
}

fn maybe_export_local_build_dir(webrtc_dir: &Path, out_dir: &Path) {
    if env::var_os("CARGO_FEATURE_LOCAL_EXPORT").is_none() {
        return;
    }

    let build_dir = out_dir.join("_build");
    let link_path = webrtc_dir.join("_build");
    fs::create_dir_all(&build_dir).expect("_build ディレクトリの作成に失敗しました");

    if let Ok(metadata) = fs::symlink_metadata(&link_path) {
        if metadata.file_type().is_symlink() {
            let current = fs::read_link(&link_path)
                .expect("既存 webrtc/_build シンボリックリンク先の取得に失敗しました");
            if current == build_dir {
                return;
            }
            remove_symlink(&link_path)
                .expect("既存 webrtc/_build シンボリックリンクの削除に失敗しました");
        } else if metadata.is_dir() {
            fs::remove_dir_all(&link_path)
                .expect("既存 webrtc/_build ディレクトリの削除に失敗しました");
        } else {
            fs::remove_file(&link_path).expect("既存 webrtc/_build ファイルの削除に失敗しました");
        }
    }

    create_dir_symlink(&build_dir, &link_path).unwrap_or_else(|err| {
        panic!(
            "webrtc/_build シンボリックリンクの作成に失敗しました: {} -> {} ({})",
            link_path.display(),
            build_dir.display(),
            err
        )
    });
}

#[cfg(unix)]
fn create_dir_symlink(target: &Path, link_path: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link_path)
}

#[cfg(windows)]
fn create_dir_symlink(target: &Path, link_path: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link_path)
}

#[cfg(unix)]
fn remove_symlink(link_path: &Path) -> std::io::Result<()> {
    fs::remove_file(link_path)
}

#[cfg(windows)]
fn remove_symlink(link_path: &Path) -> std::io::Result<()> {
    fs::remove_dir(link_path)
}

fn set_cmake_env(cmake_path: &Path) {
    // build.rs は単一スレッドで実行される前提のため、安全に環境変数を設定する
    unsafe {
        env::set_var("CMAKE", cmake_path);
    }
}

struct CmakeDownload {
    archive_name: String,
    platform: String,
    /// アーカイブのルートディレクトリからの相対パス
    executable_path: String,
}

fn ensure_cmake(out_dir: &Path) -> PathBuf {
    let host = env::var("HOST").unwrap_or_default();
    let download = cmake_download_info(&host);
    let base_dir = out_dir.join("cmake").join(CMAKE_VERSION);
    let cmake_root = base_dir.join(format!("cmake-{}-{}", CMAKE_VERSION, download.platform));
    let cmake_bin = cmake_root.join(&download.executable_path);
    if cmake_bin.exists() {
        return cmake_bin;
    }

    fs::create_dir_all(&base_dir).expect("CMake の保存先ディレクトリ作成に失敗しました");
    let archive_path = base_dir.join(&download.archive_name);
    let url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/{}",
        CMAKE_VERSION, download.archive_name
    );

    download_and_extract_cmake(&url, &archive_path, &base_dir, &download.archive_name)
        .unwrap_or_else(|err| panic!("CMake のダウンロードまたは展開に失敗しました : {}", err));

    if archive_path.exists() {
        let _ = fs::remove_file(&archive_path);
    }

    if !cmake_bin.exists() {
        panic!("CMake の展開に失敗しました: {}", cmake_bin.display());
    }

    cmake_bin
}

fn cmake_download_info(host: &str) -> CmakeDownload {
    let is_windows = host.contains("windows");
    let is_macos = host.contains("apple-darwin") || host.contains("darwin");
    let is_linux = host.contains("linux");
    let is_x86_64 = host.contains("x86_64") || host.contains("amd64");
    let is_arm64 = host.contains("aarch64") || host.contains("arm64");

    match (is_windows, is_macos, is_linux, is_x86_64, is_arm64) {
        (true, _, _, true, _) => CmakeDownload {
            archive_name: format!("cmake-{}-windows-x86_64.zip", CMAKE_VERSION),
            platform: "windows-x86_64".to_string(),
            executable_path: "bin/cmake.exe".to_string(),
        },
        (true, _, _, _, true) => CmakeDownload {
            archive_name: format!("cmake-{}-windows-arm64.zip", CMAKE_VERSION),
            platform: "windows-arm64".to_string(),
            executable_path: "bin/cmake.exe".to_string(),
        },
        (false, true, _, _, _) => CmakeDownload {
            archive_name: format!("cmake-{}-macos-universal.tar.gz", CMAKE_VERSION),
            platform: "macos-universal".to_string(),
            executable_path: "CMake.app/Contents/bin/cmake".to_string(),
        },
        (false, false, true, true, _) => CmakeDownload {
            archive_name: format!("cmake-{}-linux-x86_64.tar.gz", CMAKE_VERSION),
            platform: "linux-x86_64".to_string(),
            executable_path: "bin/cmake".to_string(),
        },
        (false, false, true, _, true) => CmakeDownload {
            archive_name: format!("cmake-{}-linux-aarch64.tar.gz", CMAKE_VERSION),
            platform: "linux-aarch64".to_string(),
            executable_path: "bin/cmake".to_string(),
        },
        _ => panic!("サポートされていない実行環境です: {}", host),
    }
}

fn download_and_extract_cmake(
    url: &str,
    archive_path: &Path,
    dest_dir: &Path,
    archive_name: &str,
) -> Result<(), String> {
    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("CMake の保存先ディレクトリ作成に失敗しました : {}", e))?;

    let archive_bytes = fetch_url(url)?;
    let tmp_path = archive_path.with_file_name(format!(
        "{}.part",
        archive_path
            .file_name()
            .expect("アーカイブ名の取得に失敗しました")
            .to_string_lossy()
    ));
    {
        let mut file = fs::File::create(&tmp_path)
            .map_err(|e| format!("CMake アーカイブの作成に失敗しました : {}", e))?;
        file.write_all(&archive_bytes)
            .map_err(|e| format!("CMake アーカイブの書き込みに失敗しました : {}", e))?;
    }
    fs::rename(&tmp_path, archive_path)
        .map_err(|e| format!("CMake アーカイブの保存に失敗しました : {}", e))?;

    verify_sha256(archive_path, archive_name)?;

    if archive_name.ends_with(".zip") {
        extract_zip(archive_path, dest_dir);
    } else {
        extract_tar_gz(archive_path, dest_dir);
    }

    Ok(())
}

fn verify_sha256(archive_path: &Path, archive_name: &str) -> Result<(), String> {
    let sha_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/cmake-{}-SHA-256.txt",
        CMAKE_VERSION, CMAKE_VERSION
    );
    let sha_bytes = fetch_url(&sha_url)?;
    let sha_text = String::from_utf8_lossy(&sha_bytes);
    let expected = extract_expected_sha256(&sha_text, archive_name);
    let actual = compute_sha256(archive_path);

    if expected != actual {
        return Err(format!(
            "SHA256 が一致しません。期待値 : {}, 実際 : {}",
            expected, actual
        ));
    }

    Ok(())
}

fn extract_expected_sha256(text: &str, archive_name: &str) -> String {
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let hash = parts.next();
        let name = parts.next();
        if let (Some(hash), Some(name)) = (hash, name)
            && name == archive_name
        {
            return hash.to_ascii_lowercase();
        }
    }

    panic!("SHA256 の取得に失敗しました : {}", archive_name);
}

fn compute_sha256(path: &Path) -> String {
    let mut file = fs::File::open(path).expect("SHA256 計算用ファイルの取得に失敗しました");
    let mut context = Context::new(&SHA256);
    let mut buf = [0u8; 8192];
    loop {
        let n = file
            .read(&mut buf)
            .expect("SHA256 計算用ファイルの読み込みに失敗しました");
        if n == 0 {
            break;
        }
        context.update(&buf[..n]);
    }
    let digest = context.finish();
    bytes_to_hex_lower(digest.as_ref())
}

fn bytes_to_hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) {
    let file = fs::File::open(archive_path).expect("CMake アーカイブの読み込みに失敗しました");
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    let entries = archive
        .entries()
        .expect("CMake アーカイブのエントリ取得に失敗しました");

    for entry in entries {
        let mut entry = entry.expect("CMake アーカイブのエントリ取得に失敗しました");
        let path = entry
            .path()
            .expect("CMake アーカイブのパス取得に失敗しました");
        if !is_safe_path(&path) {
            panic!("不正なパスが含まれています : {}", path.display());
        }
        let out_path = dest_dir.join(&path);
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).expect("展開先ディレクトリ作成に失敗しました");
        }
        entry
            .unpack(&out_path)
            .expect("CMake アーカイブの展開に失敗しました");
    }
}

fn extract_zip(archive_path: &Path, dest_dir: &Path) {
    let file = fs::File::open(archive_path).expect("CMake アーカイブの読み込みに失敗しました");
    let mut archive = zip::ZipArchive::new(file).expect("CMake アーカイブの解析に失敗しました");

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .expect("CMake アーカイブのエントリ取得に失敗しました");
        let Some(path) = file.enclosed_name() else {
            panic!("不正なパスが含まれています");
        };
        let out_path = dest_dir.join(path);
        if file.name().ends_with('/') {
            fs::create_dir_all(&out_path).expect("展開先ディレクトリ作成に失敗しました");
            continue;
        }
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).expect("展開先ディレクトリ作成に失敗しました");
        }
        let mut out = fs::File::create(&out_path).expect("展開先ファイルの作成に失敗しました");
        std::io::copy(&mut file, &mut out).expect("CMake アーカイブの展開に失敗しました");
    }
}

fn is_safe_path(path: &Path) -> bool {
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::ParentDir => return false,
            _ => {}
        }
    }
    true
}

fn fetch_url(url: &str) -> Result<Vec<u8>, String> {
    let mut current = url.to_string();
    for _ in 0..5 {
        let response = fetch_url_once(&current)?;
        if response.is_redirect() {
            let location = response
                .get_header("Location")
                .ok_or_else(|| "Location ヘッダーがありません".to_string())?;
            current = resolve_redirect_url(&current, location)?;
            continue;
        }
        if response.status_code != 200 {
            return Err(format!(
                "HTTP エラー : {} {} ({})",
                response.status_code, response.reason_phrase, current
            ));
        }
        return Ok(response.body);
    }

    Err(format!("リダイレクトが多すぎます : {}", url))
}

fn fetch_url_once(url: &str) -> Result<Response, String> {
    let (scheme, host, port, path) = parse_url(url)?;
    let request = Request::new("GET", &path)
        .header("Host", &host)
        .header("User-Agent", "shiguredo_webrtc-build")
        .header("Accept", "*/*")
        .header("Connection", "close");
    let request_bytes = request.encode();
    if scheme == "https" {
        https_request(&host, port, &request_bytes)
    } else {
        http_request(&host, port, &request_bytes)
    }
}

fn parse_url(url: &str) -> Result<(String, String, u16, String), String> {
    let (scheme, rest) = if let Some(rest) = url.strip_prefix("https://") {
        ("https".to_string(), rest)
    } else if let Some(rest) = url.strip_prefix("http://") {
        ("http".to_string(), rest)
    } else {
        return Err(format!("URL が不正です : {}", url));
    };

    let (host_port, path) = match rest.find('/') {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, "/"),
    };

    let (host, port) = match host_port.find(':') {
        Some(index) => {
            let port: u16 = host_port[index + 1..]
                .parse()
                .map_err(|e| format!("ポートの解析に失敗しました : {}", e))?;
            (&host_port[..index], port)
        }
        None => {
            let port = if scheme == "https" { 443 } else { 80 };
            (host_port, port)
        }
    };

    Ok((scheme, host.to_string(), port, path.to_string()))
}

fn resolve_redirect_url(current_url: &str, location: &str) -> Result<String, String> {
    if location.starts_with("http://") || location.starts_with("https://") {
        return Ok(location.to_string());
    }

    let (scheme, host, port, path) = parse_url(current_url)?;
    let host_port = if (scheme == "https" && port == 443) || (scheme == "http" && port == 80) {
        host
    } else {
        format!("{}:{}", host, port)
    };

    if location.starts_with('/') {
        return Ok(format!("{}://{}{}", scheme, host_port, location));
    }

    let base = match path.rfind('/') {
        Some(pos) => &path[..=pos],
        None => "/",
    };
    Ok(format!("{}://{}{}{}", scheme, host_port, base, location))
}

fn http_request(host: &str, port: u16, request_bytes: &[u8]) -> Result<Response, String> {
    let mut stream =
        TcpStream::connect((host, port)).map_err(|e| format!("HTTP 接続に失敗しました : {}", e))?;
    stream
        .write_all(request_bytes)
        .map_err(|e| format!("HTTP リクエスト送信に失敗しました : {}", e))?;

    let mut decoder = ResponseDecoder::with_limits(DecoderLimits::unlimited());
    let mut buf = [0u8; 8192];

    loop {
        let n = stream
            .read(&mut buf)
            .map_err(|e| format!("HTTP レスポンス受信に失敗しました : {}", e))?;
        if n == 0 {
            decoder.mark_eof();
            if let Some(response) = decoder
                .decode()
                .map_err(|e| format!("HTTP レスポンス解析に失敗しました : {}", e))?
            {
                return Ok(response);
            }
            return Err("HTTP レスポンスの受信が完了しませんでした".to_string());
        }
        decoder
            .feed(&buf[..n])
            .map_err(|e| format!("HTTP レスポンス解析に失敗しました : {}", e))?;
        if let Some(response) = decoder
            .decode()
            .map_err(|e| format!("HTTP レスポンス解析に失敗しました : {}", e))?
        {
            return Ok(response);
        }
    }
}

fn https_request(host: &str, port: u16, request_bytes: &[u8]) -> Result<Response, String> {
    let config = ClientConfig::with_platform_verifier()
        .map_err(|e| format!("TLS 設定の作成に失敗しました : {}", e))?;
    let server_name = ServerName::try_from(host.to_string())
        .map_err(|e| format!("サーバー名の解析に失敗しました : {}", e))?;
    let conn = ClientConnection::new(Arc::new(config), server_name)
        .map_err(|e| format!("TLS 接続に失敗しました : {}", e))?;
    let sock = TcpStream::connect((host, port))
        .map_err(|e| format!("HTTPS 接続に失敗しました : {}", e))?;
    let mut tls = StreamOwned::new(conn, sock);

    tls.write_all(request_bytes)
        .map_err(|e| format!("HTTPS リクエスト送信に失敗しました : {}", e))?;

    let mut decoder = ResponseDecoder::with_limits(DecoderLimits::unlimited());
    let mut buf = [0u8; 8192];

    loop {
        let n = match tls.read(&mut buf) {
            Ok(0) => {
                decoder.mark_eof();
                if let Some(response) = decoder
                    .decode()
                    .map_err(|e| format!("HTTPS レスポンス解析に失敗しました : {}", e))?
                {
                    return Ok(response);
                }
                return Err("HTTPS レスポンスの受信が完了しませんでした".to_string());
            }
            Ok(n) => n,
            Err(err) => {
                return Err(format!("HTTPS レスポンス受信に失敗しました : {}", err));
            }
        };

        decoder
            .feed(&buf[..n])
            .map_err(|e| format!("HTTPS レスポンス解析に失敗しました : {}", e))?;
        if let Some(response) = decoder
            .decode()
            .map_err(|e| format!("HTTPS レスポンス解析に失敗しました : {}", e))?
        {
            return Ok(response);
        }
    }
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
        .expect("libwebrtc_c.a の親ディレクトリ取得に失敗しました");
    println!("cargo:rerun-if-changed={}", lib_path.display());
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
        _ => panic!("サポートされていないターゲット OS です: {}", target_os),
    }
}

fn get_out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR の取得に失敗しました"))
}
