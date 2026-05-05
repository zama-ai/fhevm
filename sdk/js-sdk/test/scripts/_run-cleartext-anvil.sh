#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
#
# Flow:
#   1. If Anvil is already running on RPC_URL, reuse it and run tests directly.
#   2. Otherwise start a fresh Anvil instance.
#   3. Deploy the cleartext FHEVM stack.
#   4. Run only test/fheTest/cleartext-{ethers|viem}
#   5. Tear down Anvil only if this script started it.
#
# ------------------------------------------------------------------------------

ETH_LIBRARY="${ETH_LIBRARY:-}"
if [[ "$ETH_LIBRARY" != "ethers" && "$ETH_LIBRARY" != "viem" ]]; then
    echo "Error: ETH_LIBRARY must be set to 'ethers' or 'viem' (got: '${ETH_LIBRARY}')." >&2
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
JS_SDK_DIR="$(cd "$TEST_DIR/.." && pwd)"
CONTRACTS_DIR="$JS_SDK_DIR/contracts"

DEPLOY_FHEVM_SCRIPT="$CONTRACTS_DIR/script/fhevm-deploy.sh"
DEPLOY_FHE_TEST_SCRIPT="$CONTRACTS_DIR/script/fhetest-deploy.sh"

PORT="${PORT:-8544}"
RPC_URL="${RPC_URL:-http://127.0.0.1:${PORT}}"
CHAIN_ID="${CHAIN_ID:-31337}"
READY_TIMEOUT="${READY_TIMEOUT:-30}"

ANVIL_PID=""
REUSE_EXISTING_ANVIL=0

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

if [[ ! -x "$DEPLOY_FHEVM_SCRIPT" ]]; then
    echo "Error: deploy FHEVM script not found or not executable at $DEPLOY_FHEVM_SCRIPT" >&2
    exit 1
fi

# ------------------------------------------------------------------------------
# Check if 'fhetest-deploy.sh' exists
# ------------------------------------------------------------------------------

if [[ ! -x "$DEPLOY_FHE_TEST_SCRIPT" ]]; then
    echo "Error: deploy FHETest script not found or not executable at $DEPLOY_FHE_TEST_SCRIPT" >&2
    exit 1
fi

# ------------------------------------------------------------------------------
# Cleanup when scripts exits
# ------------------------------------------------------------------------------

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

# ------------------------------------------------------------------------------
# Test if anvil is already running
# ------------------------------------------------------------------------------

if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
    REUSE_EXISTING_ANVIL=1
    echo "♻️  Reusing existing Anvil on $RPC_URL."
    echo "⏭️  Skipping Anvil startup and deployment."
else
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

    echo "RPC_URL=\"$RPC_URL\"" > "$TEST_DIR/.env.localhost"

    echo "🏗️  Deploying FHEVM cleartext stack..."
    (
        cd "$CONTRACTS_DIR"
        bash "$DEPLOY_FHEVM_SCRIPT"
        sleep 1
        bash "$DEPLOY_FHE_TEST_SCRIPT"
    )
fi

# ------------------------------------------------------------------------------
# Run cleartext tests
# ------------------------------------------------------------------------------

echo "🧪 Running cleartext ${ETH_LIBRARY} tests..."
(
    TEST_TARGET="test/fheTest/cleartext-${ETH_LIBRARY}"

    cd "$JS_SDK_DIR"
    CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts "$TEST_TARGET"
    #CHAIN=localhost npx vitest run --config test/fheTest/vitest-manual-packing.config.ts "$TEST_TARGET"
)

echo "✅ cleartext-${ETH_LIBRARY} tests passed."
