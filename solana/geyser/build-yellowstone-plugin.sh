#!/usr/bin/env bash
# Resolve the Yellowstone gRPC geyser plugin cdylib for the native solana-test-validator host,
# multi-arch. The validator dlopen's this external plugin via `--geyser-plugin-config`.
#
# The geyser ABI is patch-coupled to the validator's Agave version, so the `+solana.<version>`
# suffix on YELLOWSTONE_REF MUST match the agave version setup-solana-side.sh runs (currently
# 4.1.2 — see the toolchain pins in solana-e2e.yml and solana-tests.yml).
# On x86_64 Linux (CI) we download the prebuilt release artifact; on every other host (native Apple
# Silicon) we build from source. Upstream gates the Linux-only `affinity` crate off macOS itself as
# of v14, so the local patch that used to do it is gone.
#
# Prints the ABSOLUTE plugin path to stdout (and nothing else); diagnostics go to stderr. Caches
# under solana/target so repeated runs are instant. Override the tag with YELLOWSTONE_REF.
set -euo pipefail

YELLOWSTONE_REF="${YELLOWSTONE_REF:-v14.2.2+solana.4.1.0}"
SOLANA="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
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
  # Since v14 the release ships the cdylib directly rather than inside a tarball.
  enc="${YELLOWSTONE_REF/+/%2B}"
  url="https://github.com/rpcpool/yellowstone-grpc/releases/download/$enc/$LIB_LINUX"
  log "downloading prebuilt $LIB_LINUX for $YELLOWSTONE_REF"
  curl -fsSL -o "$OUT" "$url"
  [ -s "$OUT" ] || { log "prebuilt .so download was empty"; rm -f "$OUT"; exit 1; }
  log "downloaded: $OUT"; echo "$OUT"; exit 0
fi

# Build from source (non-x86-Linux, e.g. native Apple Silicon arm64). No patch needed: upstream
# target-gates the Linux-only `affinity` dep off macOS from v14 onwards.
log "building $YELLOWSTONE_REF from source for $os/$arch (no prebuilt artifact)"
SRC="$CACHE/src"
if [ ! -d "$SRC/.git" ]; then
  rm -rf "$SRC"
  git clone --depth 1 --branch "$YELLOWSTONE_REF" \
    https://github.com/rpcpool/yellowstone-grpc.git "$SRC" >&2
fi
( cd "$SRC" && cargo build --release -p yellowstone-grpc-geyser >&2 )
built="$SRC/target/release/$LIB_MAC"
[ -f "$built" ] || built="$SRC/target/release/$LIB_LINUX"
[ -f "$built" ] || { log "build produced no cdylib"; exit 1; }
cp "$built" "$OUT"
log "built: $OUT"; echo "$OUT"
