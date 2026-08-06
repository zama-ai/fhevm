#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SOURCE="$ROOT/sdk/js-sdk/src"
DEST="$ROOT/test-suite/fhevm/node_modules/@fhevm/sdk"

[ -f "$SOURCE/package.json" ] || { echo "missing built SDK package: $SOURCE" >&2; exit 1; }
[ ! -e "$SOURCE/node_modules" ] || { echo "refusing to copy source-local SDK dependencies" >&2; exit 1; }
[ -d "$(dirname "$DEST")" ] || { echo "test-suite dependencies are not installed" >&2; exit 1; }

STAGED="$(mktemp -d "${DEST}.materialize.XXXXXX")"
cleanup() { rm -rf "$STAGED"; }
trap cleanup EXIT

cp -RL "$SOURCE/." "$STAGED/"
[ -f "$STAGED/_esm/solana/index.js" ] || { echo "SDK ESM output is not built" >&2; exit 1; }
[ -f "$STAGED/_types/solana/index.d.ts" ] || { echo "SDK type output is not built" >&2; exit 1; }

rm -rf "$DEST"
mv "$STAGED" "$DEST"
trap - EXIT
