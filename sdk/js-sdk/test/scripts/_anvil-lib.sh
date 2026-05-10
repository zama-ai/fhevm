#!/usr/bin/env bash
# Source this file; do not execute it directly.

_ANVIL_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

anvil_setup_dirs() {
    SCRIPT_DIR="$_ANVIL_LIB_DIR"
    TEST_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
    JS_SDK_DIR="$(cd "$TEST_DIR/.." && pwd)"
    CONTRACTS_DIR="$JS_SDK_DIR/contracts"
    DEPLOY_FHEVM_SCRIPT="$CONTRACTS_DIR/script/fhevm-deploy.sh"
    DEPLOY_FHE_TEST_SCRIPT="$CONTRACTS_DIR/script/fhetest-deploy.sh"
}

anvil_setup_vars() {
    PORT="${PORT:-8544}"
    RPC_URL="${RPC_URL:-http://127.0.0.1:${PORT}}"
    CHAIN_ID="${CHAIN_ID:-31337}"
    READY_TIMEOUT="${READY_TIMEOUT:-30}"
    export FOUNDRY_PROFILE="${FOUNDRY_PROFILE:-latest}"
}

anvil_check_deps() {
    for bin in anvil cast bash npx; do
        if ! command -v "$bin" >/dev/null 2>&1; then
            echo "Error: '$bin' not found in PATH." >&2
            exit 1
        fi
    done
}

anvil_check_scripts() {
    if [[ ! -x "$DEPLOY_FHEVM_SCRIPT" ]]; then
        echo "Error: deploy FHEVM script not found or not executable at $DEPLOY_FHEVM_SCRIPT" >&2
        exit 1
    fi
    if [[ ! -x "$DEPLOY_FHE_TEST_SCRIPT" ]]; then
        echo "Error: deploy FHETest script not found or not executable at $DEPLOY_FHE_TEST_SCRIPT" >&2
        exit 1
    fi
}

_anvil_cleanup() {
    local exit_code=$?
    trap - EXIT INT TERM
    if [[ "${REUSE_EXISTING_ANVIL:-0}" -eq 0 ]] && [[ -n "${ANVIL_PID:-}" ]] && kill -0 "$ANVIL_PID" 2>/dev/null; then
        echo "🛑 Stopping Anvil (PID $ANVIL_PID)..."
        kill "$ANVIL_PID" 2>/dev/null || true
        wait "$ANVIL_PID" 2>/dev/null || true
    fi
    exit "$exit_code"
}

anvil_setup_cleanup() {
    trap _anvil_cleanup EXIT INT TERM
}

anvil_start_and_wait() {
    ANVIL_PID=""
    echo "🚚 Starting Anvil on $RPC_URL..."
    anvil --port "$PORT" --chain-id "$CHAIN_ID" --disable-code-size-limit &
    ANVIL_PID=$!

    echo "⏳ Waiting for Anvil readiness (timeout: ${READY_TIMEOUT}s)..."
    local deadline=$(( $(date +%s) + READY_TIMEOUT ))
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
}

anvil_deploy_cleartext() {
    echo "RPC_URL=\"$RPC_URL\"" > "$TEST_DIR/.env.localhost"
    echo "🏗️  Deploying FHEVM cleartext stack..."
    (
        cd "$CONTRACTS_DIR"
        bash "$DEPLOY_FHEVM_SCRIPT"
        sleep 1
        bash "$DEPLOY_FHE_TEST_SCRIPT"
    )
}
