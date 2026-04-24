#!/usr/bin/env bash
set -euo pipefail

################################################################################
#
# run-viem-cleartext-anvil.sh
#
# Flow:
#   1. If Anvil is already running on RPC_URL, reuse it and run tests directly.
#   2. Otherwise start a fresh Anvil instance.
#   3. Deploy the cleartext FHEVM stack.
#   4. Run only test/fheTest/viem-cleartext.
#   5. Tear down Anvil only if this script started it.
#
################################################################################

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
REUSE_EXISTING_ANVIL=0

################################################################################
#
# Checks
#
################################################################################

for bin in anvil cast bash npx; do
    if ! command -v "$bin" >/dev/null 2>&1; then
        echo "Error: '$bin' not found in PATH." >&2
        exit 1
    fi
done

if [[ ! -x "$DEPLOY_SCRIPT" ]]; then
    echo "Error: deploy script not found or not executable at $DEPLOY_SCRIPT" >&2
    exit 1
fi

################################################################################
#
# Cleanup
#
################################################################################

cleanup() {
    local exit_code=$?
    trap - EXIT INT TERM

    if [[ "$REUSE_EXISTING_ANVIL" -eq 0 ]] && [[ -n "${ANVIL_PID:-}" ]] && kill -0 "$ANVIL_PID" 2>/dev/null; then
        echo "🛑 Stopping Anvil (PID $ANVIL_PID)..."
        kill "$ANVIL_PID" 2>/dev/null || true
        wait "$ANVIL_PID" 2>/dev/null || true
    fi

    exit "$exit_code"
}
trap cleanup EXIT INT TERM

################################################################################
#
# Anvil Setup
#
################################################################################

if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
    REUSE_EXISTING_ANVIL=1
    echo "♻️  Reusing existing Anvil on $RPC_URL."
    echo "⏭️  Skipping Anvil startup and deployment."
else
    echo "🚚 Starting Anvil on $RPC_URL..."
    anvil --port "$PORT" --chain-id "$CHAIN_ID" &
    ANVIL_PID=$!

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

    ################################################################################
    #
    # Deploy Cleartext FHEVM Stack
    #
    ################################################################################

    echo "🏗️  Deploying FHEVM cleartext stack..."
    (
        cd "$CONTRACTS_DIR"
        bash "$DEPLOY_SCRIPT"
    )
fi

################################################################################
#
# Run viem-cleartext tests
#
################################################################################

echo "🧪 Running viem-cleartext tests..."
(
    cd "$JS_SDK_DIR"
    CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts "$TEST_TARGET"
)

echo "✅ viem-cleartext tests passed."
