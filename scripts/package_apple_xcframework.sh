#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<USAGE
Usage:
  $0
    --library <path-to-static-library>
    [--library <path-to-static-library> ...]
    --headers <path-to-headers-dir>
    --out-dir <output-dir>
USAGE
}

# --library は複数指定を受け付ける。
libraries=()
headers_dir=""
out_dir=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --library) libraries+=("$2"); shift 2 ;;
    --headers) headers_dir="$2"; shift 2 ;;
    --out-dir) out_dir="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "ERROR: unknown argument: $1" >&2; usage; exit 1 ;;
  esac
done

# 少なくとも 1 つの static library を受け取る。
[ "${#libraries[@]}" -gt 0 ] || { echo "ERROR: --library is required (at least one)" >&2; exit 1; }
[ -n "$headers_dir" ] || { echo "ERROR: --headers is required" >&2; exit 1; }
[ -n "$out_dir" ] || { echo "ERROR: --out-dir is required" >&2; exit 1; }

# 利用ツールの存在を先に確認して、後続処理の失敗を早めに検知する。
for cmd in xcodebuild zip shasum swift ar cp; do
  command -v "$cmd" >/dev/null 2>&1 || { echo "ERROR: required command not found: $cmd" >&2; exit 1; }
done

# 一時作業ディレクトリは終了時に必ず削除する。
workdir="$(mktemp -d "${TMPDIR:-/tmp}/webrtc_apple_xcframework.XXXXXX")"
trap 'rm -rf "$workdir"' EXIT

# 入力ファイルはテンポラリへコピーしてから扱う。
mkdir -p "$workdir/libs" "$workdir/headers" "$out_dir"
[ -d "$headers_dir" ] || { echo "ERROR: missing headers directory: $headers_dir" >&2; exit 1; }
cp -R "$headers_dir"/. "$workdir/headers"/
[ -f "$workdir/headers/webrtc_c.h" ] || { echo "ERROR: missing header: webrtc_c.h" >&2; exit 1; }

strip_unwanted_members() {
  local archive="$1"
  # XCFramework では libwebrtc_c.a を -all_load でリンクするため、
  # 実行エントリを含む不要オブジェクトが残っていると duplicate symbol(_main) を引き起こす。
  # そのため、Apple 向け配布物では該当メンバーを事前に除去する。
  for member in main.o cppgen_plugin.o protozero_plugin.o; do
    ar -d "$archive" "$member" 2>/dev/null || true
  done
}

# xcodebuild -create-xcframework に渡す -library/-headers 引数を組み立てる。
library_args=()
library_index=0
for library_path in "${libraries[@]}"; do
  [ -f "$library_path" ] || { echo "ERROR: missing file: $library_path" >&2; exit 1; }
  # 元ファイルを直接変更しないよう、テンポラリに複製してから加工する。
  copied_library="$workdir/libs/lib$(printf "%02d" "$library_index").a"
  cp "$library_path" "$copied_library"
  # duplicate symbol 回避のため、不要オブジェクトを除去する。
  strip_unwanted_members "$copied_library"
  # 各ライブラリに同一の公開ヘッダーセットを対応付ける。
  library_args+=(-library "$copied_library" -headers "$workdir/headers")
  library_index=$((library_index + 1))
done

cat > "$workdir/headers/module.modulemap" <<'MODULEMAP'
module webrtc_c {
  header "webrtc_c.h"
  export *
}
MODULEMAP

xcframework_path="$out_dir/libwebrtc_c.xcframework"
zip_path="$out_dir/libwebrtc_c.xcframework.zip"

rm -rf "$xcframework_path"
rm -f "$zip_path" "$zip_path.sha256" "$zip_path.swift-checksum.txt"

xcodebuild -create-xcframework \
  "${library_args[@]}" \
  -output "$xcframework_path"

# SwiftPM 配布向けに zip と checksum を生成する。
(
  cd "$out_dir"
  zip -r -q "$(basename "$zip_path")" "$(basename "$xcframework_path")"
)

shasum -a 256 "$zip_path" > "$zip_path.sha256"
swift package compute-checksum "$zip_path" > "$zip_path.swift-checksum.txt"

echo "INFO: created $xcframework_path"
echo "INFO: created $zip_path"
