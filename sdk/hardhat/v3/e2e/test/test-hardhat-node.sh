#!/usr/bin/env bash

set -euo pipefail # Exit on error, undefined vars, and pipe errors

HARDHAT_NODE_PORT=8545
HARDHAT_NODE_HOST=127.0.0.1
HARDHAT_NODE_URL="http://${HARDHAT_NODE_HOST}:${HARDHAT_NODE_PORT}"
TIMEOUT_SECONDS=60 # Max time to wait for Hardhat Node to start
CHECK_INTERVAL_SECONDS=1 # How often to poll the node

HARDHAT_PID_ROOT=""

# Kill the node however we leave: success, test failure, `set -e` abort, or Ctrl-C. Without this the
# node survives a mid-script failure and the next run finds port 8545 already taken.
cleanup() {
    if [ -n "${HARDHAT_PID_ROOT}" ] && ps -p "${HARDHAT_PID_ROOT}" > /dev/null 2>&1; then
        echo "--- Killing Hardhat Node (PID: ${HARDHAT_PID_ROOT}) ---"
        kill "${HARDHAT_PID_ROOT}" 2>/dev/null || true
        wait "${HARDHAT_PID_ROOT}" 2>/dev/null || true
    fi

    # `npx hardhat node` spawns the listener as a child, so killing the wrapper does not always free
    # the port. Kill whatever still holds it — one PID per line, hence the loop.
    local pid
    for pid in $(lsof -i ":${HARDHAT_NODE_PORT}" -t 2>/dev/null || true); do
        echo "--- Killing leftover listener on port ${HARDHAT_NODE_PORT} (PID: ${pid}) ---"
        kill "${pid}" 2>/dev/null || true
    done

    # Give the OS a moment to release the port before the next server instance starts.
    sleep 1
}
trap cleanup EXIT

echo "--- Starting Hardhat Node in background ---"
# The node process carries the plugin: it deploys the cleartext stack BEFORE it starts listening.
npx hardhat node &> /dev/null &
HARDHAT_PID_ROOT=$! # Get the PID of the background process

echo "Hardhat Node started with PID: $HARDHAT_PID_ROOT. Waiting for it to be ready..."

# --- Wait for Hardhat Node to be ready ---
NODE_READY=0
ATTEMPTS=0
while [ $ATTEMPTS -lt $TIMEOUT_SECONDS ]; do
    if curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' "$HARDHAT_NODE_URL" > /dev/null 2>&1; then
        echo "Hardhat Node is ready!"
        NODE_READY=1
        break
    fi
    echo "Waiting for Hardhat Node... (Attempt $((ATTEMPTS+1))/$TIMEOUT_SECONDS)"
    sleep "$CHECK_INTERVAL_SECONDS"
    ATTEMPTS=$((ATTEMPTS+1))
done

if [ "$NODE_READY" -ne 1 ]; then
    echo "Error: Hardhat Node did not start within $TIMEOUT_SECONDS seconds."
    exit 1 # `cleanup` runs on EXIT
fi

# --- Run tests ---
echo "--- Running tests against external Hardhat Node ---"
# `set +e` around the run, rather than `|| true`: with `|| true` the pipeline always succeeds and
# `$?` records *that*, so a failing suite was reported as a pass.
set +e
npm run test:node
TEST_EXIT_CODE=$?
set -e

# Exit with the same exit code (`cleanup` runs on EXIT)
exit "$TEST_EXIT_CODE"
