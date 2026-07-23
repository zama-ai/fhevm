#!/usr/bin/env bash
# Guarantees browser-next runs against the freshly-packed SDK from a clean
# dev-server state. Run AFTER `npm run pack:prod`, which repacks the tarball but
# only reinstalls it into manual-pack/ — it never touches browser-next, so
# browser-next keeps whatever @fhevm/sdk was last extracted (the stale-tarball trap).
#
# Guarantees:
#   1. No stale Next dev server bound to :3334 (mirrors dod.sh's per-cell free_next,
#      applied once up front so a wrong-env/wedged server can't be reused).
#   2. browser-next's node_modules/@fhevm/sdk is re-extracted from the current tarball.
set -euo pipefail

BN_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" # test/browser-next
PACK_DIR="$(cd "$BN_DIR/../manual-pack" && pwd)"

TARBALL=$(echo "$PACK_DIR"/fhevm-sdk-*.tgz)
[[ -f "$TARBALL" ]] || {
  echo "refresh-sdk: no tarball in $PACK_DIR — run 'npm run pack:prod' first." >&2
  exit 1
}

# 1. Kill any Next dev server on :3334. `next dev --turbopack` spawns children that
#    keep the port bound under a graceful signal, so SIGKILL + pkill.
echo "refresh-sdk: killing any dev server on :3334..."
{ lsof -ti tcp:3334 2>/dev/null | xargs kill -9 2>/dev/null; } || true
pkill -9 -f 'next(-server)? dev' 2>/dev/null || true

# 2. Force re-extract the freshly packed SDK. npm will NOT re-extract a
#    same-named tarball overwritten in place, so remove the folder first and
#    install the tarball explicitly (--no-save keeps the file: ref in package.json).
echo "refresh-sdk: reinstalling $(basename "$TARBALL") into browser-next..."
rm -rf "$BN_DIR/node_modules/@fhevm/sdk"
(cd "$BN_DIR" && npm install --no-save "$TARBALL")

echo "refresh-sdk: done."
