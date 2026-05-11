#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
#
# Run cleartext FHEVM tests against an Anvil node.
#
# Usage: run-cleartext-anvil.sh [--ethlib ethers|viem|ethers,viem|none] [--profile <name>] [--verbose] [--help]
#
# Flow:
#   1. Parse --ethlib (default: ethers,viem) and --profile (default: latest).
#   2. If Anvil is already running on RPC_URL, reuse it; otherwise start one.
#   3. Deploy the cleartext FHEVM stack (skipped when reusing).
#   4. Run the requested test suites sequentially against the same node.
#   5. Tear down Anvil only if this script started it.
#
# ------------------------------------------------------------------------------

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Run cleartext FHEVM tests against an Anvil node.

Options:
  --ethlib ethers          Run only the ethers test suite.
  --ethlib viem            Run only the viem test suite.
  --ethlib ethers,viem     Run both suites (default).
  --ethlib none            Start Anvil + deploy only; wait for Anvil to exit (no tests run).
  --profile <name>         Foundry profile to use (default: latest, possible values: v12, v13).
  --verbose                Print Anvil logs to the console instead of redirecting to a file.
  --help                   Print this help message and exit.

Environment variables (all optional, CLI flags take precedence):
  PORT             Anvil port          (default: 8544)
  RPC_URL          Anvil RPC URL       (default: http://127.0.0.1:\$PORT)
  CHAIN_ID         Anvil chain ID      (default: 31337)
  READY_TIMEOUT    Ready timeout       (default: 30s)
  FOUNDRY_PROFILE  Foundry profile     (default: latest; overridden by --profile)
EOF
}

# ------------------------------------------------------------------------------
# Parse arguments
# ------------------------------------------------------------------------------

LIB="ethers,viem"
PROFILE=""
VERBOSE=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --ethlib)
            shift
            if [[ $# -eq 0 ]]; then
                echo "Error: --ethlib requires an argument." >&2
                exit 1
            fi
            LIB="$1"
            shift
            ;;
        --ethlib=*)
            LIB="${1#--ethlib=}"
            shift
            ;;
        --profile)
            shift
            if [[ $# -eq 0 ]]; then
                echo "Error: --profile requires an argument." >&2
                exit 1
            fi
            PROFILE="$1"
            shift
            ;;
        --profile=*)
            PROFILE="${1#--profile=}"
            shift
            ;;
        --verbose)
            VERBOSE=1
            shift
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            echo "Error: unknown option '$1'. Use --help for usage." >&2
            exit 1
            ;;
    esac
done

RUN_ETHERS=0
RUN_VIEM=0
RUN_NONE=0
case "$LIB" in
    ethers)       RUN_ETHERS=1 ;;
    viem)         RUN_VIEM=1 ;;
    ethers,viem)  RUN_ETHERS=1; RUN_VIEM=1 ;;
    viem,ethers)  RUN_ETHERS=1; RUN_VIEM=1 ;;
    none)         RUN_NONE=1 ;;
    *)
        echo "Error: --ethlib must be 'ethers', 'viem', 'ethers,viem', or 'none' (got: '${LIB}')." >&2
        exit 1
        ;;
esac

# ------------------------------------------------------------------------------
# Anvil helpers
# ------------------------------------------------------------------------------

anvil_setup_dirs() {
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
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
    if [[ "${VERBOSE:-0}" -eq 1 ]]; then
        echo "🚚 Starting Anvil on $RPC_URL..."
        anvil --port "$PORT" --chain-id "$CHAIN_ID" --disable-code-size-limit &
    else
        ANVIL_LOG="${TMPDIR:-/tmp}/anvil-${PORT}.log"
        echo "🚚 Starting Anvil on $RPC_URL (log: $ANVIL_LOG)..."
        anvil --port "$PORT" --chain-id "$CHAIN_ID" --disable-code-size-limit \
            >"$ANVIL_LOG" 2>&1 &
    fi
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

# ------------------------------------------------------------------------------
# Bootstrap
# ------------------------------------------------------------------------------

anvil_setup_dirs
# Let --profile override FOUNDRY_PROFILE before anvil_setup_vars reads it.
[[ -n "$PROFILE" ]] && FOUNDRY_PROFILE="$PROFILE"
anvil_setup_vars
anvil_check_deps
anvil_check_scripts

REUSE_EXISTING_ANVIL=0
anvil_setup_cleanup

# ------------------------------------------------------------------------------
# Start Anvil (or reuse for test mode)
# ------------------------------------------------------------------------------

if [[ "$RUN_NONE" -eq 1 ]]; then
    # Server mode: refuse to start if something is already listening.
    if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
        echo "Error: something is already listening on $RPC_URL. Stop it or use another PORT/RPC_URL." >&2
        exit 1
    fi
    anvil_start_and_wait
    anvil_deploy_cleartext
else
    # Test mode: reuse an existing Anvil if available.
    if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
        REUSE_EXISTING_ANVIL=1
        echo "♻️  Reusing existing Anvil on $RPC_URL."
        echo "⏭️  Skipping Anvil startup and deployment."
    else
        anvil_start_and_wait
        anvil_deploy_cleartext
    fi
fi

echo
echo "================================================================================"
echo "🎯  Foundry profile: ${FOUNDRY_PROFILE}"
echo "================================================================================"

# ------------------------------------------------------------------------------
# Server mode: just wait for Anvil to exit
# ------------------------------------------------------------------------------

if [[ "$RUN_NONE" -eq 1 ]]; then
    echo
    echo "✅ FHEVM cleartext stack deployed on ${RPC_URL} (chain-id: $(cast chain-id --rpc-url "$RPC_URL"))."
    echo "Anvil is running as PID ${ANVIL_PID}. Press Ctrl-C to stop."
    wait "$ANVIL_PID"
    exit 0
fi

# ------------------------------------------------------------------------------
# Run requested test suites
# ------------------------------------------------------------------------------

run_suite() {
    local lib="$1"
    echo
    echo "🧪 Running cleartext ${lib} tests..."
    (
        cd "$JS_SDK_DIR"
        CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts "test/fheTest/cleartext-${lib}"
    )
    echo "✅ cleartext-${lib} tests passed."
}

[[ "$RUN_ETHERS" -eq 1 ]] && run_suite ethers
[[ "$RUN_VIEM"   -eq 1 ]] && run_suite viem

echo
echo "✅ All requested cleartext tests passed (--ethlib ${LIB})."
