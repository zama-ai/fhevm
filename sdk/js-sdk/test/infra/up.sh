#!/usr/bin/env bash
set -euo pipefail

# Launch the test infra (2 cleartext anvils + the same-origin gateway).
#
#   ./test/infra/up.sh              foreground; logs stream here; Ctrl-C to stop.
#   ./test/infra/up.sh -d|--detach  background it and return the terminal once
#                                   ready. Startup logs still stream to THIS
#                                   terminal (no log file); stop with down.sh.
#
# While it is up, run any tests against it — they reuse this infra:
#   cd test/browser-next && \
#     npx playwright test specs/gw-skeleton.spec.ts --config playwright.config.ts
#
# Requires foundry (anvil/cast/forge) on PATH.

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JS_SDK_DIR="$(cd "$DIR/../.." && pwd)"
TSX="$JS_SDK_DIR/node_modules/.bin/tsx"

# Readiness probe: the gateway only comes up after both anvils have deployed.
# Keep in sync with GATEWAY_PORT / GATEWAY_MOUNT_PREFIX in test/infra/topology.ts.
READY_URL="http://127.0.0.1:8590/gw/v12/relayer/v2/keyurl"

detach=0
case "${1:-}" in
  -d | --detach) detach=1 ;;
  "") ;;
  *)
    echo "usage: $(basename "$0") [-d|--detach]" >&2
    exit 1
    ;;
esac

if [[ "$detach" -eq 0 ]]; then
  exec "$TSX" "$DIR/up.ts"
fi

# Detached: background a blocking instance that INHERITS this terminal's
# stdout/stderr (so logs stream here, no file), then poll the gateway port for
# readiness and hand the terminal back. up.ts goes quiet after "Infra ready".
"$TSX" "$DIR/up.ts" &
pid=$!
disown "$pid" 2>/dev/null || true

for _ in $(seq 1 150); do
  if curl -fsS -m 2 "$READY_URL" >/dev/null 2>&1; then
    echo ""
    echo "✅ infra ready (pid ${pid}). Stop with: ${DIR}/down.sh"
    exit 0
  fi
  if ! kill -0 "$pid" 2>/dev/null; then
    echo "❌ infra launch failed (see logs above)." >&2
    exit 1
  fi
  sleep 2
done

echo "❌ timed out waiting for infra ready (see logs above)." >&2
exit 1
