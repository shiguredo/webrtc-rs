use std::env;
use std::fs;
use std::io::{Read, Write};
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

    let header = webrtc_dir.join("src").join("webrtc_c.h");
    let include_dir = webrtc_dir.join("src");
    let target_platform = get_target_platform();
    let out_dir = get_out_dir();

    // CMake でビルド（依存関係のダウンロードも CMakeLists.txt 内で行われる）
    let lib_path = build_webrtc_c(&webrtc_dir, &target_platform, &out_dir);
    maybe_export_local_build_dir(&webrtc_dir, &out_dir);

    generate_bindings(&header, &include_dir);
    emit_link_directives(&lib_path);
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

    download_and_extract_cmake(&url, &archive_path, &base_dir, &download.archive_name);

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

fn download_and_extract_cmake(url: &str, archive_path: &Path, dest_dir: &Path, archive_name: &str) {
    fs::create_dir_all(dest_dir).expect("CMake の保存先ディレクトリ作成に失敗しました");

    let archive_bytes = fetch_url(url);
    let tmp_path = archive_path.with_file_name(format!(
        "{}.part",
        archive_path
            .file_name()
            .expect("アーカイブ名の取得に失敗しました")
            .to_string_lossy()
    ));
    {
        let mut file = fs::File::create(&tmp_path).expect("CMake アーカイブの作成に失敗しました");
        file.write_all(&archive_bytes)
            .expect("CMake アーカイブの書き込みに失敗しました");
    }
    fs::rename(&tmp_path, archive_path).expect("CMake アーカイブの保存に失敗しました");

    verify_sha256(archive_path, archive_name);

    if archive_name.ends_with(".zip") {
        extract_zip(archive_path, dest_dir);
    } else {
        extract_tar_gz(archive_path, dest_dir);
    }
}

fn verify_sha256(archive_path: &Path, archive_name: &str) {
    let sha_url = format!(
        "https://github.com/Kitware/CMake/releases/download/v{}/cmake-{}-SHA-256.txt",
        CMAKE_VERSION, CMAKE_VERSION
    );
    let sha_bytes = fetch_url(&sha_url);
    let sha_text = String::from_utf8_lossy(&sha_bytes);
    let expected = extract_expected_sha256(&sha_text, archive_name);
    let actual = compute_sha256(archive_path);

    if expected != actual {
        panic!(
            "SHA256 が一致しません。期待値 : {}, 実際 : {}",
            expected, actual
        );
    }
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

fn fetch_url(url: &str) -> Vec<u8> {
    let mut current = url.to_string();
    for _ in 0..5 {
        let response = fetch_url_once(&current);
        if response.is_redirect() {
            let location = response
                .get_header("Location")
                .expect("Location ヘッダーがありません");
            current = resolve_redirect_url(&current, location);
            continue;
        }
        if response.status_code != 200 {
            panic!(
                "HTTP エラー : {} {} ({})",
                response.status_code, response.reason_phrase, current
            );
        }
        return response.body;
    }

    panic!("リダイレクトが多すぎます : {}", url);
}

fn fetch_url_once(url: &str) -> Response {
    let (scheme, host, port, path) = parse_url(url);
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

fn parse_url(url: &str) -> (String, String, u16, String) {
    let (scheme, rest) = if let Some(rest) = url.strip_prefix("https://") {
        ("https".to_string(), rest)
    } else if let Some(rest) = url.strip_prefix("http://") {
        ("http".to_string(), rest)
    } else {
        panic!("URL が不正です : {}", url);
    };

    let (host_port, path) = match rest.find('/') {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, "/"),
    };

    let (host, port) = match host_port.find(':') {
        Some(index) => {
            let port: u16 = host_port[index + 1..]
                .parse()
                .expect("ポートの解析に失敗しました");
            (&host_port[..index], port)
        }
        None => {
            let port = if scheme == "https" { 443 } else { 80 };
            (host_port, port)
        }
    };

    (scheme, host.to_string(), port, path.to_string())
}

fn resolve_redirect_url(current_url: &str, location: &str) -> String {
    if location.starts_with("http://") || location.starts_with("https://") {
        return location.to_string();
    }

    let (scheme, host, port, path) = parse_url(current_url);
    let host_port = if (scheme == "https" && port == 443) || (scheme == "http" && port == 80) {
        host
    } else {
        format!("{}:{}", host, port)
    };

    if location.starts_with('/') {
        return format!("{}://{}{}", scheme, host_port, location);
    }

    let base = match path.rfind('/') {
        Some(pos) => &path[..=pos],
        None => "/",
    };
    format!("{}://{}{}{}", scheme, host_port, base, location)
}

fn http_request(host: &str, port: u16, request_bytes: &[u8]) -> Response {
    let mut stream = TcpStream::connect((host, port)).expect("HTTP 接続に失敗しました");
    stream
        .write_all(request_bytes)
        .expect("HTTP リクエスト送信に失敗しました");

    let mut decoder = ResponseDecoder::with_limits(DecoderLimits::unlimited());
    let mut buf = [0u8; 8192];

    loop {
        let n = stream
            .read(&mut buf)
            .expect("HTTP レスポンス受信に失敗しました");
        if n == 0 {
            decoder.mark_eof();
            if let Some(response) = decoder.decode().expect("HTTP レスポンス解析に失敗しました")
            {
                return response;
            }
            panic!("HTTP レスポンスの受信が完了しませんでした");
        }
        decoder
            .feed(&buf[..n])
            .expect("HTTP レスポンス解析に失敗しました");
        if let Some(response) = decoder.decode().expect("HTTP レスポンス解析に失敗しました")
        {
            return response;
        }
    }
}

fn https_request(host: &str, port: u16, request_bytes: &[u8]) -> Response {
    let config = ClientConfig::with_platform_verifier().expect("TLS 設定の作成に失敗しました");
    let server_name =
        ServerName::try_from(host.to_string()).expect("サーバー名の解析に失敗しました");
    let conn =
        ClientConnection::new(Arc::new(config), server_name).expect("TLS 接続に失敗しました");
    let sock = TcpStream::connect((host, port)).expect("HTTPS 接続に失敗しました");
    let mut tls = StreamOwned::new(conn, sock);

    tls.write_all(request_bytes)
        .expect("HTTPS リクエスト送信に失敗しました");

    let mut decoder = ResponseDecoder::with_limits(DecoderLimits::unlimited());
    let mut buf = [0u8; 8192];

    loop {
        let n = match tls.read(&mut buf) {
            Ok(0) => {
                decoder.mark_eof();
                if let Some(response) = decoder
                    .decode()
                    .expect("HTTPS レスポンス解析に失敗しました")
                {
                    return response;
                }
                panic!("HTTPS レスポンスの受信が完了しませんでした");
            }
            Ok(n) => n,
            Err(err) => panic!("HTTPS レスポンス受信に失敗しました : {}", err),
        };

        decoder
            .feed(&buf[..n])
            .expect("HTTPS レスポンス解析に失敗しました");
        if let Some(response) = decoder
            .decode()
            .expect("HTTPS レスポンス解析に失敗しました")
        {
            return response;
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
