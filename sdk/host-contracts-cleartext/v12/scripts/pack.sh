#!/usr/bin/env bash
#
# Build the publishable tarball with `npm pack`, into a directory you name.
#
# Usage: ./scripts/pack.sh <out-dir> [--clean]
#        ./scripts/pack.sh --out-dir <dir> [--clean]
#
#   <out-dir>   where the .tgz is written; created if missing
#   --clean     delete existing *.tgz in that directory first
#
# Prints the absolute path of the tarball on stdout, and a listing summary on stderr — so
# `TARBALL="$(./scripts/pack.sh /tmp/out)"` works in a pipeline.
#
# ## Two details that are not incidental
#
# `npm pack` runs in ./pkg, not the package root. pkg/package.json is the PUBLISHED manifest — the root
# one is the private harness, and packing it would ship the tests, the generators and the whole toolchain
# instead of the payload (RULES.md rule 9).
#
# It also runs with its own npm cache under $TMPDIR. Sharing the user's cache makes concurrent runs race
# on the same lock, which surfaces as an unrelated-looking npm error rather than a pack failure.
#
# internal/createPackageTarball.ts does the same thing from the Node side, defaulting to ./tarball.
# This exists for callers outside the Node toolchain, and for writing the tarball somewhere else.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

OUT_DIR=""
CLEAN=0

while [ $# -gt 0 ]; do
    case "$1" in
        --out-dir)
            [ $# -ge 2 ] || { echo "Error: --out-dir requires a value." >&2; exit 1; }
            OUT_DIR="$2"; shift 2 ;;
        --out-dir=*) OUT_DIR="${1#--out-dir=}"; shift ;;
        --clean) CLEAN=1; shift ;;
        -h | --help) sed -n '2,10p' "${BASH_SOURCE[0]}"; exit 0 ;;
        -*) echo "Error: unknown option '$1'. Try --help." >&2; exit 1 ;;
        *)
            [ -z "$OUT_DIR" ] || { echo "Error: out dir given twice ('$OUT_DIR' and '$1')." >&2; exit 1; }
            OUT_DIR="$1"; shift ;;
    esac
done

[ -n "$OUT_DIR" ] || { echo "Error: an out dir is required. Try --help." >&2; exit 1; }

command -v npm >/dev/null 2>&1 || { echo "Error: npm not on PATH." >&2; exit 1; }
command -v jq >/dev/null 2>&1 || { echo "Error: jq not on PATH." >&2; exit 1; }

PKG_DIR="$PACKAGE_ROOT/pkg"
[ -f "$PKG_DIR/package.json" ] || {
    echo "Error: $PKG_DIR/package.json not found — is this the package root?" >&2
    exit 1
}

mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

# Only ever removes tarballs, and only when asked: this is a directory the caller named, so deleting
# anything else in it would be well outside what a pack command should do.
if [ "$CLEAN" -eq 1 ]; then
    find "$OUT_DIR" -maxdepth 1 -name '*.tgz' -type f -delete
    echo "🧹 removed existing *.tgz in $OUT_DIR" >&2
fi

NPM_CACHE="${TMPDIR:-/tmp}/fhevm-host-contracts-cleartext-npm-cache"
mkdir -p "$NPM_CACHE"

echo "📦 npm pack in $PKG_DIR -> $OUT_DIR" >&2
PACK_JSON="$(cd "$PKG_DIR" && npm_config_cache="$NPM_CACHE" npm pack --json --pack-destination "$OUT_DIR")" || {
    echo "Error: npm pack failed." >&2
    exit 1
}

# `npm pack --json` emits an array of one entry per tarball. Parsed properly rather than with a regex
# because the same JSON carries the full file list, which contains .tgz-looking paths of its own.
#
# `-e` makes jq exit non-zero when the result is null or false, so a shape change is caught here instead
# of producing an empty filename and a confusing "does not exist" a few lines down.
FILENAME="$(printf '%s' "$PACK_JSON" | jq -re '.[0].filename')" || {
    echo "Error: could not read the filename from npm pack output:" >&2
    printf '%s\n' "$PACK_JSON" >&2
    exit 1
}

TARBALL="$OUT_DIR/$FILENAME"

# npm reports success even when the destination write is not what you expect (a stale --pack-destination,
# a name with a scope directory), so confirm the file rather than trusting the exit code.
[ -f "$TARBALL" ] || { echo "Error: npm pack reported $FILENAME but $TARBALL does not exist." >&2; exit 1; }

# A tarball missing src/ still installs and still fails later, at `forge install` time in a consumer —
# far from here. Cheap to rule out now (RULES.md rule 16).
for required in package/package.json package/src; do
    tar -tzf "$TARBALL" | grep -q "^${required}" || {
        echo "Error: $FILENAME does not contain $required." >&2
        exit 1
    }
done

{
    echo "✅ $FILENAME ($(wc -c < "$TARBALL" | tr -d ' ') bytes)"
    echo "   top-level entries:"
    tar -tzf "$TARBALL" | sed 's|^package/||' | awk -F/ 'NF>1 {print $1"/"} NF==1 && $1!="" {print $1}' \
        | sort -u | sed 's/^/     /'
} >&2

printf '%s\n' "$TARBALL"
