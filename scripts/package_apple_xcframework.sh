#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/package_apple_xcframework.sh \
    --ios-tar <libwebrtc_c-ios_arm64.tar.gz> \
    --macos-tar <libwebrtc_c-macos_arm64.tar.gz> \
    [--tag <release-tag>] \
    [--out-dir <output-dir>]
USAGE
}

ios_tar=""
macos_tar=""
out_dir="dist/apple-xcframework"
tag=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --ios-tar) ios_tar="$2"; shift 2 ;;
    --macos-tar) macos_tar="$2"; shift 2 ;;
    --tag) tag="$2"; shift 2 ;;
    --out-dir) out_dir="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "ERROR: unknown argument: $1" >&2; usage; exit 1 ;;
  esac
done

[ -n "$ios_tar" ] && [ -n "$macos_tar" ] || { echo "ERROR: --ios-tar and --macos-tar are required" >&2; exit 1; }

for cmd in tar xcodebuild zip shasum swift; do
  command -v "$cmd" >/dev/null 2>&1 || { echo "ERROR: required command not found: $cmd" >&2; exit 1; }
done
[ -z "$tag" ] || command -v gh >/dev/null 2>&1 || { echo "ERROR: required command not found: gh" >&2; exit 1; }

workdir="$(mktemp -d "${TMPDIR:-/tmp}/webrtc_apple_xcframework.XXXXXX")"
trap 'rm -rf "$workdir"' EXIT

mkdir -p "$workdir/ios" "$workdir/macos" "$workdir/headers" "$out_dir"

tar -xzf "$ios_tar" -C "$workdir/ios"
tar -xzf "$macos_tar" -C "$workdir/macos"

ios_lib="$workdir/ios/lib/libwebrtc_c.a"
macos_lib="$workdir/macos/lib/libwebrtc_c.a"

[ -f "$ios_lib" ] || { echo "ERROR: missing file: $ios_lib" >&2; exit 1; }
[ -f "$macos_lib" ] || { echo "ERROR: missing file: $macos_lib" >&2; exit 1; }
[ -f "$workdir/ios/include/webrtc_c.h" ] || { echo "ERROR: missing header: webrtc_c.h" >&2; exit 1; }

cp -R "$workdir/ios/include"/. "$workdir/headers"/
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

# 現在は iOS と macOS で同一のヘッダー (webrtc_c.h) のみを扱うため、両スライスに同じ
# --headers を指定している。将来的にプラットフォーム間で異なるヘッダーが必要に
# なった場合は、各スライスに個別のヘッダーディレクトリを用意すること。
xcodebuild -create-xcframework \
  -library "$ios_lib" -headers "$workdir/headers" \
  -library "$macos_lib" -headers "$workdir/headers" \
  -output "$xcframework_path"

(
  cd "$out_dir"
  zip -r -q "$(basename "$zip_path")" "$(basename "$xcframework_path")"
)

shasum -a 256 "$zip_path" > "$zip_path.sha256"
swift package compute-checksum "$zip_path" > "$zip_path.swift-checksum.txt"

if [ -n "$tag" ]; then
  gh release upload "$tag" "$zip_path" "$zip_path.sha256" --clobber
fi

echo "INFO: created $xcframework_path"
echo "INFO: created $zip_path"
