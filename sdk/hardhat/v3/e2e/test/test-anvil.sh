#!/usr/bin/env bash

set -euo pipefail # Exit on error, undefined vars, and pipe errors

ANVIL_PORT=8545
ANVIL_HOST=127.0.0.1
ANVIL_URL="http://${ANVIL_HOST}:${ANVIL_PORT}"
TIMEOUT_SECONDS=60 # Max time to wait for Anvil to start
CHECK_INTERVAL_SECONDS=1 # How often to poll the node

ANVIL_PID=""

# Kill Anvil however we leave: success, test failure, `set -e` abort, or Ctrl-C. Without this a
# mid-script failure leaves Anvil holding port 8545 and the next run fails confusingly.
cleanup() {
    if [ -n "${ANVIL_PID}" ] && ps -p "${ANVIL_PID}" > /dev/null 2>&1; then
        echo "--- Killing Anvil (PID: ${ANVIL_PID}) ---"
        kill "${ANVIL_PID}" 2>/dev/null || true
        wait "${ANVIL_PID}" 2>/dev/null || true
    fi

    # Anvil can outlive the wrapper; free the port whatever still holds it (one PID per line).
    local pid
    for pid in $(lsof -i ":${ANVIL_PORT}" -t 2>/dev/null || true); do
        echo "--- Killing leftover listener on port ${ANVIL_PORT} (PID: ${pid}) ---"
        kill "${pid}" 2>/dev/null || true
    done

    # Give the OS a moment to release the port before the next server instance starts.
    sleep 1
}
trap cleanup EXIT

echo "--- Starting Anvil in background ---"
# A FRESH anvil: the plugin deploys the cleartext stack onto it from the first connection.
anvil &> /dev/null &
ANVIL_PID=$! # Get the PID of the background process

echo "Anvil started with PID: $ANVIL_PID. Waiting for it to be ready..."

# --- Wait for Anvil to be ready ---
ANVIL_READY=0
ATTEMPTS=0
while [ $ATTEMPTS -lt $TIMEOUT_SECONDS ]; do
    if curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' "$ANVIL_URL" > /dev/null 2>&1; then
        echo "Anvil is ready!"
        ANVIL_READY=1
        break
    fi
    echo "Waiting for Anvil... (Attempt $((ATTEMPTS+1))/$TIMEOUT_SECONDS)"
    sleep "$CHECK_INTERVAL_SECONDS"
    ATTEMPTS=$((ATTEMPTS+1))
done

if [ "$ANVIL_READY" -ne 1 ]; then
    echo "Error: Anvil did not start within $TIMEOUT_SECONDS seconds."
    exit 1 # `cleanup` runs on EXIT
fi

# --- Run tests ---
echo "--- Running tests against external Anvil ---"
# `set +e` around the run, rather than `|| true`: with `|| true` the pipeline always succeeds and
# `$?` records *that*, so a failing suite was reported as a pass.
set +e
npm run test:anvil
TEST_EXIT_CODE=$?
set -e

# Exit with the same exit code (`cleanup` runs on EXIT)
exit "$TEST_EXIT_CODE"
