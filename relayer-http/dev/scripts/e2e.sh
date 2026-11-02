#!/usr/bin/env bash
# Copies dev/e2e/ into the e2e debug container and runs one flow script there with hardhat.
# Relayer: the stack's (relayer-http:RELAYER_HTTP_PORT) when container fhevm-relayer-http runs, else the one `make run`
# starts on the host (RELAYER_HTTP_RUN_PORT); RELAYER_HTTP_URL overrides both. Both ports come from the Makefile.
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

if [ -z "${RELAYER_HTTP_URL:-}" ]; then
  if [ "$(docker inspect -f '{{.State.Running}}' fhevm-relayer-http 2>/dev/null)" = true ]; then
    RELAYER_HTTP_URL="http://relayer-http:$RELAYER_HTTP_PORT"
  else
    # Relayer on the host: Docker Desktop (macOS) names the host host.docker.internal, Linux Docker Engine reaches it at
    # the gateway of the stack's network.
    case "$(uname -s)" in
      Darwin) HOST=host.docker.internal ;;
      *) HOST="$(docker network inspect fhevm_default --format '{{(index .IPAM.Config 0).Gateway}}')" ;;
    esac
    RELAYER_HTTP_URL="http://$HOST:$RELAYER_HTTP_RUN_PORT"
  fi
fi
echo "relayer: $RELAYER_HTTP_URL"

ENV_ARGS=(-e "RELAYER_HTTP_URL=$RELAYER_HTTP_URL")
for var in MIN_SHARES ATTEMPTS RETRY_DELAY_MS; do
  if [ -n "${!var:-}" ]; then ENV_ARGS+=(-e "$var=${!var}"); fi
done
exec docker exec "${ENV_ARGS[@]}" "$CONTAINER" npx hardhat run "scripts/relayer-http/$SCRIPT" --network staging
