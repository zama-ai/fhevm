#!/usr/bin/env bash
# Copies dev/e2e/ into the e2e debug container and runs one script there with hardhat against the relayer on the
# host. Usage: e2e.sh public-decrypt.ts | user-decrypt.ts
set -euo pipefail
SCRIPT="${1:?script name under dev/e2e}"
HERE="$(cd "$(dirname "$0")/.." && pwd)"
CONTAINER=fhevm-test-suite-e2e-debug
docker inspect "$CONTAINER" >/dev/null 2>&1 || { echo "container $CONTAINER not running: start the stack first"; exit 1; }
docker cp "$HERE/e2e/." "$CONTAINER:/app/test-suite/e2e/scripts/relayer-http/"
exec docker exec -e RELAYER_HTTP_URL="${RELAYER_HTTP_URL:-http://host.docker.internal:8080}" "$CONTAINER" \
  npx hardhat run "scripts/relayer-http/$SCRIPT" --network staging
