#!/bin/sh
set -eu

if [ "$#" -lt 5 ]; then
  echo "error: usage: bundle_apple_static_library.sh <output> <target_obj> <ar_bin> <target_name> -- <static_lib...>" >&2
  exit 1
fi

output="$1"
target_obj="$2"
ar_bin="$3"
target_name="$4"
shift 4

if [ "$#" -lt 1 ]; then
  echo "error: expected separator -- before static libraries" >&2
  exit 1
fi

if [ "$1" != "--" ]; then
  echo "error: expected separator -- before static libraries" >&2
  exit 1
fi
shift

case "$target_name" in
  macos_arm64|ios_arm64)
    ;;
  *)
    echo "error: unsupported Apple target: $target_name" >&2
    exit 1
    ;;
esac

if [ "$#" -lt 1 ]; then
  echo "error: at least one static library is required" >&2
  exit 1
fi

primary_lib="$1"
shift

file_state() {
  if [ ! -f "$1" ]; then
    echo "error: input file not found: $1" >&2
    exit 1
  fi
}

non_fat_arch() {
  info="$(lipo -info "$1" 2>/dev/null || true)"
  if [ -z "$info" ]; then
    echo "error: failed to inspect library architectures: $1" >&2
    exit 1
  fi

  case "$info" in
    *"Non-fat file:"*" is architecture:"*)
      printf '%s\n' "$info" | sed -E 's/.* is architecture: //'
      ;;
    *)
      echo "error: expected single-arch static library: $1" >&2
      exit 1
      ;;
  esac
}

file_state "$target_obj"
file_state "$primary_lib"
for lib in "$@"; do
  file_state "$lib"
done

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/webrtc_bundle.XXXXXX")"
cleanup() {
  rm -rf "$tmpdir"
}
trap cleanup EXIT HUP INT TERM

thin_lib="$tmpdir/libwebrtc_thin.a"
core_lib="$tmpdir/libwebrtc_core.a"
objc_list="$tmpdir/objc_members.txt"
objc_dir="$tmpdir/objc_members"
objc_blob="$tmpdir/webrtc_objc_blob.o"

# 非 fat 前提でアーキテクチャ一致だけを検証する
target_arch="$(non_fat_arch "$target_obj")"

# libwebrtc.a 側は fat/non-fat を判定せず、必要アーキテクチャをそのまま抽出する
libtool -static -arch_only "$target_arch" -o "$thin_lib" "$primary_lib"
cp "$thin_lib" "$core_lib"

# Objective-C 実装を含むメンバーを列挙する
nm -m "$thin_lib" | awk '
/^[^[:space:]].*:$/ {
  member = $1
  sub(/:$/, "", member)
}
/__OBJC_/ && member != "" {
  print member
}
' | sort -u > "$objc_list"

if [ -s "$objc_list" ]; then
  mkdir -p "$objc_dir"

  # 対象メンバーを一括で別出しし、core 側からも一括で除外して実行時間を短縮する
  members="$(tr '\n' ' ' < "$objc_list")"
  if [ -n "$members" ]; then
    (
      cd "$objc_dir"
      # shellcheck disable=SC2086
      "$ar_bin" -x "$thin_lib" $members
    )
    # shellcheck disable=SC2086
    "$ar_bin" -d "$core_lib" $members
  fi

  # private extern を保持して、未解決シンボル解決で blob が選択される状態を作る
  ld -r -all_load -keep_private_externs -arch "$target_arch" -o "$objc_blob" "$objc_dir"/*.o

  libtool -static -o "$output" "$target_obj" "$core_lib" "$objc_blob" "$@"
else
  # 抽出対象がない場合は従来どおりに結合する
  libtool -static -o "$output" "$target_obj" "$core_lib" "$@"
fi
