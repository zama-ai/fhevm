#!/bin/sh
# Runs the official end-to-end scenarios (scenarios/, tag @e2e) against the local fhEVM stack
# started with `fhevm-cli up`, the way a user would: `test-engine run --profile local`. The engine
# runs inside the e2e container, only for its network (the relayer hands out addresses that only
# resolve there); it uses nothing of the e2e project. Needs `npm run build`. Set E2E_CONTAINER for
# another container.
set -u
cd "$(dirname "$0")/../../.."

CONTAINER="${E2E_CONTAINER:-fhevm-test-suite-e2e-debug}"
DEST=/tmp/test-engine-e2e

if [ "$(docker inspect -f '{{.State.Running}}' "$CONTAINER" 2>/dev/null)" != "true" ]; then
  echo "container '$CONTAINER' is not running: start the local stack with fhevm-cli up" >&2
  exit 1
fi

# A fresh copy of the package (built output and node_modules included) on every run.
docker exec -u root "$CONTAINER" rm -rf "$DEST" || exit 1
docker exec "$CONTAINER" mkdir -p "$DEST" || exit 1
COPYFILE_DISABLE=1 tar --exclude=./reports -cf - . | docker exec -i "$CONTAINER" tar -xf - -C "$DEST" || exit 1

TE_E2E_CONTAINER="$CONTAINER" TE_E2E_DIR="$DEST" \
  node --test --test-reporter=spec "dist/test/integration/e2e/**/*.test.js"
