#!/usr/bin/env python3

import argparse
import shutil
import subprocess
import sys
from pathlib import Path
from typing import List


SUPPORTED_EXTENSIONS = {".h", ".c", ".cc", ".cpp", ".m", ".mm"}


def find_tool(base_name: str) -> str:
    for version in range(50, 9, -1):
        candidate = f"{base_name}-{version}"
        path = shutil.which(candidate)
        if path:
            return path
    path = shutil.which(base_name)
    if path:
        return path
    return ""


def collect_files(root: Path) -> List[Path]:
    files = [
        path
        for path in root.rglob("*")
        if path.is_file() and path.suffix in SUPPORTED_EXTENSIONS
    ]
    return sorted(files)


def chunk_paths(paths: List[Path], chunk_size: int) -> List[List[Path]]:
    return [paths[i : i + chunk_size] for i in range(0, len(paths), chunk_size)]


def run_command(command: list[str]) -> None:
    print(f"実行 : {' '.join(command)}")
    result = subprocess.run(command, check=False)
    if result.returncode != 0:
        sys.exit(result.returncode)


def do_format(check: bool) -> None:
    tool = find_tool("clang-format")
    if not tool:
        print("clang-format が見つかりません。", file=sys.stderr)
        sys.exit(1)

    root = Path(__file__).resolve().parent / "src" / "webrtc_c"
    files = collect_files(root)
    if not files:
        print("対象ファイルが見つかりません。", file=sys.stderr)
        return

    chunked_files = chunk_paths(files, 200)
    for chunk in chunked_files:
        if check:
            run_command([tool, "-n", "--Werror", *[str(path) for path in chunk]])
        else:
            run_command([tool, "-i", *[str(path) for path in chunk]])


def do_iwyu(target: str, profile: str, check: bool) -> None:
    tool = find_tool("clang-include-cleaner")
    if not tool:
        print("clang-include-cleaner が見つかりません。", file=sys.stderr)
        sys.exit(1)

    webrtc_dir = Path(__file__).resolve().parent
    build_dir = webrtc_dir / "_build" / target / profile / "build"
    compile_commands = build_dir / "compile_commands.json"
    if not compile_commands.exists():
        print(
            f"compile_commands.json が見つかりません： {compile_commands}",
            file=sys.stderr,
        )
        sys.exit(1)

    root = webrtc_dir / "src" / "webrtc_c"
    files = collect_files(root)
    if not files:
        print("対象ファイルが見つかりません。", file=sys.stderr)
        return

    command = [tool, "-p", str(build_dir)]
    if not check:
        command.append("--fix")
    command.extend(str(path) for path in files)
    run_command(command)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    format_parser = subparsers.add_parser("format")
    format_parser.add_argument("--check", action="store_true")

    iwyu_parser = subparsers.add_parser("iwyu")
    iwyu_parser.add_argument("target")
    iwyu_parser.add_argument("--profile", default="release")
    iwyu_parser.add_argument("--check", action="store_true")

    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.command == "format":
        do_format(args.check)
        return
    if args.command == "iwyu":
        do_iwyu(args.target, args.profile, args.check)
        return

    print(f"不明なコマンド： {args.command}", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    main()
