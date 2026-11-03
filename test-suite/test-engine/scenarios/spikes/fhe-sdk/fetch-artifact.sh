#!/bin/sh
# Copies the compiled TestInput contract out of the e2e container of the local stack. The artifact
# has the stack's protocol addresses compiled in (E2ECoprocessorConfig), so it is not committed.
set -eu

CONTAINER="${E2E_CONTAINER:-fhevm-test-suite-e2e-debug}"
SOURCE=/app/test-suite/e2e/artifacts/contracts/smoke/TestInput.sol/TestInput.json
TARGET="$(dirname "$0")/artifacts/TestInput.json"

mkdir -p "$(dirname "$TARGET")"
docker cp "$CONTAINER:$SOURCE" "$TARGET"
echo "copied $SOURCE from $CONTAINER to $TARGET"
