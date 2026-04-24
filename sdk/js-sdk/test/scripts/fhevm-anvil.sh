#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
JS_SDK_DIR="$(cd "$TEST_DIR/.." && pwd)"
CONTRACTS_DIR="$JS_SDK_DIR/contracts"
DEPLOY_SCRIPT="$CONTRACTS_DIR/script/fhevm-deploy.sh"

PORT="${PORT:-8545}"
RPC_URL="${RPC_URL:-http://127.0.0.1:${PORT}}"
CHAIN_ID="${CHAIN_ID:-31337}"
READY_TIMEOUT="${READY_TIMEOUT:-30}"
TEST_TARGET="test/fheTest/viem-cleartext"

ANVIL_PID=""

# ------------------------------------------------------------------------------
# Check if anvil is installed
# ------------------------------------------------------------------------------

for bin in anvil cast bash npx; do
    if ! command -v "$bin" >/dev/null 2>&1; then
        echo "Error: '$bin' not found in PATH." >&2
        exit 1
    fi
done

# ------------------------------------------------------------------------------
# Check if 'fhevm-deploy.sh' exists
# ------------------------------------------------------------------------------

if [[ ! -x "$DEPLOY_SCRIPT" ]]; then
    echo "Error: deploy script not found or not executable at $DEPLOY_SCRIPT" >&2
    exit 1
fi

# ------------------------------------------------------------------------------
# Cleanup when scripts exits
# ------------------------------------------------------------------------------

cleanup() {
    local exit_code=$?
    trap - EXIT INT TERM

    if [[ -n "${ANVIL_PID:-}" ]] && kill -0 "$ANVIL_PID" 2>/dev/null; then
        echo "🛑 Stopping Anvil (PID $ANVIL_PID)..."
        kill "$ANVIL_PID" 2>/dev/null || true
        wait "$ANVIL_PID" 2>/dev/null || true
    fi

    exit "$exit_code"
}
trap cleanup EXIT INT TERM

# ------------------------------------------------------------------------------
# Test if anvil is already running
# ------------------------------------------------------------------------------

if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
    echo "Error: something is already listening on $RPC_URL. Stop it or use another PORT/RPC_URL." >&2
    exit 1
fi

# ------------------------------------------------------------------------------
# Start anvil
# ------------------------------------------------------------------------------

echo "🚚 Starting Anvil on $RPC_URL..."
anvil --port "$PORT" --chain-id "$CHAIN_ID" &
ANVIL_PID=$!

# ------------------------------------------------------------------------------
# Wait for anvil pid
# ------------------------------------------------------------------------------

echo "⏳ Waiting for Anvil readiness (timeout: ${READY_TIMEOUT}s)..."
deadline=$(( $(date +%s) + READY_TIMEOUT ))
until cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; do
    if ! kill -0 "$ANVIL_PID" 2>/dev/null; then
        echo "Error: Anvil exited before becoming ready." >&2
        exit 1
    fi
    if (( $(date +%s) > deadline )); then
        echo "Error: Anvil did not become ready within ${READY_TIMEOUT}s." >&2
        exit 1
    fi
    sleep 0.2
done
echo "✅ Anvil is ready."

# ------------------------------------------------------------------------------
# Deploy FHEVM cleartext
# ------------------------------------------------------------------------------

echo "🏗️  Deploying FHEVM cleartext stack..."
(
    cd "$CONTRACTS_DIR"
    bash "$DEPLOY_SCRIPT"
)

echo
echo "✅ FHEVM Cleartext stack deployed and initialized on ${RPC_URL} (chain-id: $(cast chain-id --rpc-url "$RPC_URL"))."
echo "anvil is running as PID ${ANVIL_PID}. Press Ctrl-C to stop."
wait "$ANVIL_PID"
