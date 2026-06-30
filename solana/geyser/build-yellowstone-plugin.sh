#!/usr/bin/env bash
# Resolve the Yellowstone gRPC geyser plugin cdylib for the native solana-test-validator host,
# multi-arch. The validator dlopen's this external plugin via `--geyser-plugin-config`.
#
# The geyser ABI is patch-coupled to the validator's Agave version, so YELLOWSTONE_REF MUST match
# the agave version setup-solana-side.sh runs (currently 2.1.21 — see solana-e2e.yml toolchain pin).
# On x86_64 Linux (CI) we download the prebuilt release artifact; on every other host (native Apple
# Silicon) we build from source, applying a small patch that gates the Linux-only `affinity` crate
# off macOS (the prebuilt Linux .so already compiles affinity fine, so it needs no patch).
#
# Prints the ABSOLUTE plugin path to stdout (and nothing else); diagnostics go to stderr. Caches
# under solana/target so repeated runs are instant. Override the tag with YELLOWSTONE_REF.
set -euo pipefail

YELLOWSTONE_REF="${YELLOWSTONE_REF:-v5.0.1+solana.2.1.21}"
# Prebuilt x86_64 Linux artifact for the pinned tag (a .tar.bz2 containing lib/<the .so>).
LINUX_TARBALL="${LINUX_TARBALL:-yellowstone-grpc-geyser-release22-x86_64-unknown-linux-gnu.tar.bz2}"
SOLANA="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PATCH="$SOLANA/geyser/yellowstone-affinity-macos.patch"
CACHE="$SOLANA/target/yellowstone/$YELLOWSTONE_REF"
LIB_LINUX="libyellowstone_grpc_geyser.so"
LIB_MAC="libyellowstone_grpc_geyser.dylib"

log() { echo "[yellowstone] $*" >&2; }

os="$(uname -s)"; arch="$(uname -m)"
if [ "$os" = "Linux" ]; then OUT="$CACHE/$LIB_LINUX"; else OUT="$CACHE/$LIB_MAC"; fi

if [ -f "$OUT" ]; then log "cached: $OUT"; echo "$OUT"; exit 0; fi
mkdir -p "$CACHE"

if [ "$os" = "Linux" ] && [ "$arch" = "x86_64" ]; then
  # Prebuilt release artifact (x86_64 Linux only — what CI runs on). Public download, no auth.
  enc="${YELLOWSTONE_REF/+/%2B}"
  url="https://github.com/rpcpool/yellowstone-grpc/releases/download/$enc/$LINUX_TARBALL"
  log "downloading prebuilt $LINUX_TARBALL for $YELLOWSTONE_REF"
  tmp="$CACHE/dl.tar.bz2"
  curl -fsSL -o "$tmp" "$url"
  tar -xjf "$tmp" -C "$CACHE"
  built="$(find "$CACHE" -name "$LIB_LINUX" -type f | head -1)"
  [ -n "$built" ] && [ -s "$built" ] || { log "prebuilt .so not found in tarball"; exit 1; }
  cp "$built" "$OUT"; rm -f "$tmp"
  log "downloaded: $OUT"; echo "$OUT"; exit 0
fi

# Build from source (non-x86-Linux, e.g. native Apple Silicon arm64). Apply the affinity patch so
# the cdylib compiles on macOS (the Linux-only `affinity` dep is target-gated off non-Linux).
log "building $YELLOWSTONE_REF from source for $os/$arch (no prebuilt artifact)"
SRC="$CACHE/src"
if [ ! -d "$SRC/.git" ]; then
  rm -rf "$SRC"
  git clone --depth 1 --branch "$YELLOWSTONE_REF" \
    https://github.com/rpcpool/yellowstone-grpc.git "$SRC" >&2
  [ -f "$PATCH" ] && git -C "$SRC" apply "$PATCH" >&2 && log "applied affinity macOS patch"
fi
( cd "$SRC" && cargo build --release -p yellowstone-grpc-geyser >&2 )
built="$SRC/target/release/$LIB_MAC"
[ -f "$built" ] || built="$SRC/target/release/$LIB_LINUX"
[ -f "$built" ] || { log "build produced no cdylib"; exit 1; }
cp "$built" "$OUT"
log "built: $OUT"; echo "$OUT"
