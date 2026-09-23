#!/usr/bin/env bash
set -euo pipefail

# Anchor 1.2.0 requests v1.57. cargo-build-sbf 4.1.0 can silently substitute its
# default on a cold cache when GitHub's mutable "latest" release is older.
version=v1.57
tools_dir="$HOME/.cache/solana/$version/platform-tools"
if [[ -x "$tools_dir/rust/bin/rustc" && -x "$tools_dir/llvm/bin/clang" ]]; then
  printf 'SBF tools %s: ' "$version"
  "$tools_dir/rust/bin/rustc" --version
  exit 0
fi
case "$(uname -s)" in
  Linux) platform=linux ;;
  Darwin) platform=osx ;;
  *) echo 'SBF builds require Linux or macOS' >&2; exit 1 ;;
esac
case "$(uname -m)" in
  x86_64) arch=x86_64 ;;
  arm64|aarch64) arch=aarch64 ;;
  *) echo 'Unsupported SBF build architecture' >&2; exit 1 ;;
esac
mkdir -p "$(dirname "$tools_dir")"
lock_dir="$(dirname "$tools_dir")/install.lock"
mkdir "$lock_dir" 2>/dev/null || { echo "SBF tools installation already running ($lock_dir); retry after it finishes" >&2; exit 1; }
work=""
trap 'if [[ -n "$work" ]]; then rm -rf "$work"; fi; rmdir "$lock_dir"' EXIT
# Another installer may have completed between our cache check and lock acquisition.
if [[ -x "$tools_dir/rust/bin/rustc" && -x "$tools_dir/llvm/bin/clang" ]]; then
  printf 'SBF tools %s: ' "$version"
  "$tools_dir/rust/bin/rustc" --version
  exit 0
fi
work=$(mktemp -d "$(dirname "$tools_dir")/install.XXXXXX")
curl --fail --silent --show-error --location --retry 3 \
  "https://github.com/anza-xyz/platform-tools/releases/download/$version/platform-tools-$platform-$arch.tar.bz2" \
  --output "$work/tools.tar.bz2"
mkdir "$work/platform-tools"
tar -xjf "$work/tools.tar.bz2" -C "$work/platform-tools"
[[ -x "$work/platform-tools/rust/bin/rustc" && -x "$work/platform-tools/llvm/bin/clang" ]]
# Refuse to merge a partial installation with the downloaded toolchain.
[[ ! -e "$tools_dir" ]] || { echo "Remove incomplete SBF tools at $tools_dir and retry" >&2; exit 1; }
mv "$work/platform-tools" "$tools_dir"
printf 'SBF tools %s: ' "$version"
"$tools_dir/rust/bin/rustc" --version
