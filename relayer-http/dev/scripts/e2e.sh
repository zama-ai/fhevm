#!/usr/bin/env bash
# Copies dev/e2e/ into the e2e debug container and runs one flow script there with hardhat.
# Relayer: the stack's (relayer-http:RELAYER_HTTP_PORT) when container fhevm-relayer-http runs, else the one `make run`
# starts on the host (RELAYER_HTTP_RUN_PORT); RELAYER_HTTP_URL overrides both. Both ports come from the Makefile.
# RELAYER_HTTP_URL may name a port published on the host (http://localhost:3003, the stack's relayer proxy): the flow
# runs in the container, so localhost is rewritten to the host as seen from there.
# Health: the flow checks /healthz on the stack's relayer-http when it runs (the proxy routes only /v2-/v4), else on
# RELAYER_HTTP_URL.
# MIN_SHARES, ATTEMPTS and RETRY_DELAY_MS pass through when set.
# Usage: e2e.sh public-decrypt.ts | user-decrypt.ts
set -euo pipefail
SCRIPT="${1:?script name under dev/e2e}"
: "${RELAYER_HTTP_PORT:?run through make (dev/Makefile sets it)}"
: "${RELAYER_HTTP_RUN_PORT:?run through make (dev/Makefile sets it)}"
HERE="$(cd "$(dirname "$0")/.." && pwd)"
CONTAINER=fhevm-test-suite-e2e-debug
docker inspect "$CONTAINER" >/dev/null 2>&1 || { echo "container $CONTAINER not running: start the stack first"; exit 1; }
docker cp "$HERE/e2e/." "$CONTAINER:/app/test-suite/e2e/scripts/relayer-http/"

# The host as seen from the container: Docker Desktop (macOS) names it host.docker.internal, Linux Docker Engine reaches
# it at the gateway of the stack's network.
case "$(uname -s)" in
  Darwin) HOST=host.docker.internal ;;
  *) HOST="$(docker network inspect fhevm_default --format '{{(index .IPAM.Config 0).Gateway}}')" ;;
esac

STACK_RELAYER_HTTP_URL=""
if [ "$(docker inspect -f '{{.State.Running}}' fhevm-relayer-http 2>/dev/null)" = true ]; then
  STACK_RELAYER_HTTP_URL="http://relayer-http:$RELAYER_HTTP_PORT"
fi
if [ -z "${RELAYER_HTTP_URL:-}" ]; then
  RELAYER_HTTP_URL="${STACK_RELAYER_HTTP_URL:-http://$HOST:$RELAYER_HTTP_RUN_PORT}"
fi
case "$RELAYER_HTTP_URL" in
  http://*) ;;
  *) RELAYER_HTTP_URL="http://$RELAYER_HTTP_URL" ;;
esac
RELAYER_HTTP_URL="$(echo "$RELAYER_HTTP_URL" | sed -E "s#^http://(localhost|127\.0\.0\.1)([:/]|\$)#http://$HOST\2#")"
RELAYER_HTTP_HEALTH_URL="${STACK_RELAYER_HTTP_URL:-$RELAYER_HTTP_URL}"
echo "relayer: $RELAYER_HTTP_URL (health: $RELAYER_HTTP_HEALTH_URL)"

ENV_ARGS=(-e "RELAYER_HTTP_URL=$RELAYER_HTTP_URL" -e "RELAYER_HTTP_HEALTH_URL=$RELAYER_HTTP_HEALTH_URL")
for var in MIN_SHARES ATTEMPTS RETRY_DELAY_MS; do
  if [ -n "${!var:-}" ]; then ENV_ARGS+=(-e "$var=${!var}"); fi
done
exec docker exec "${ENV_ARGS[@]}" "$CONTAINER" npx hardhat run "scripts/relayer-http/$SCRIPT" --network staging
