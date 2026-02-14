#!/bin/bash

set -euo pipefail

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
SOLANA_PROGRAM_DIR="${REPO_ROOT}/solana/host-programs"
ENGINE_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"
VALIDATOR_LOG="${REPO_ROOT}/.solana-validator-explorer-demo.log"

PROGRAM_ID="${PROGRAM_ID:-Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq}"
RPC_URL="${RPC_URL:-http://127.0.0.1:8899}"
WALLET="${WALLET:-/Users/work/.config/solana/id.json}"
LEDGER_DIR="${LEDGER_DIR:-/tmp/solana-codex-ledger}"
KEEP_VALIDATOR="${KEEP_VALIDATOR:-false}"
CLEANUP_LEDGER="${CLEANUP_LEDGER:-true}"
SQLX_OFFLINE="${SQLX_OFFLINE:-true}"

SO_PATH="${SOLANA_PROGRAM_DIR}/target/deploy/zama_host.so"

usage() {
    cat <<EOF
Usage: $0 [options]

Options:
  --keep-validator     Keep validator running after script exits (default: false)
  --keep-ledger        Keep local ledger directory after cleanup (default: false)
  -h, --help           Show this help message

Environment overrides:
  PROGRAM_ID, RPC_URL, WALLET, LEDGER_DIR, SQLX_OFFLINE
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --keep-validator)
            KEEP_VALIDATOR=true
            shift
            ;;
        --keep-ledger)
            CLEANUP_LEDGER=false
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            log_error "Unknown argument: $1"
            usage
            exit 1
            ;;
    esac
done

for bin in cargo solana-test-validator anchor docker curl; do
    if ! command -v "$bin" >/dev/null 2>&1; then
        log_error "Missing required binary: $bin"
        exit 1
    fi
done

if [[ ! -f "$WALLET" ]]; then
    log_error "Wallet not found: $WALLET"
    exit 1
fi

rpc_healthy() {
    curl -sS -m 1 \
        -H 'Content-Type: application/json' \
        -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' \
        "$RPC_URL" | grep -q '"result":"ok"'
}

VALIDATOR_STARTED=false

cleanup() {
    if [[ "$VALIDATOR_STARTED" == "true" && "$KEEP_VALIDATOR" != "true" && -n "${VALIDATOR_PID:-}" ]]; then
        kill "$VALIDATOR_PID" >/dev/null 2>&1 || true
        wait "$VALIDATOR_PID" 2>/dev/null || true
        log_info "Stopped validator pid=$VALIDATOR_PID"
    fi
    if [[ "$VALIDATOR_STARTED" == "true" && "$KEEP_VALIDATOR" != "true" && "$CLEANUP_LEDGER" == "true" ]]; then
        rm -rf "$LEDGER_DIR"
        log_info "Removed ledger directory: $LEDGER_DIR"
    fi
}
trap cleanup EXIT

log_info "Building host program + IDL"
(
    cd "$SOLANA_PROGRAM_DIR"
    anchor build
)

if [[ ! -f "$SO_PATH" ]]; then
    log_error "Compiled program not found: $SO_PATH"
    exit 1
fi

if rpc_healthy; then
    log_warn "RPC already healthy at $RPC_URL; reusing existing validator."
else
    log_info "Starting local validator at $RPC_URL"
    mkdir -p "$LEDGER_DIR"
    solana-test-validator \
        --reset \
        --ledger "$LEDGER_DIR" \
        --rpc-port 8899 \
        --faucet-port 9900 \
        --bpf-program "$PROGRAM_ID" "$SO_PATH" >"$VALIDATOR_LOG" 2>&1 &
    VALIDATOR_PID=$!
    VALIDATOR_STARTED=true

    for _ in $(seq 1 30); do
        if rpc_healthy; then
            break
        fi
        sleep 1
    done

    if ! rpc_healthy; then
        log_error "Validator did not become healthy. Check $VALIDATOR_LOG"
        exit 1
    fi
fi

log_info "Running explorer-visible PoC runner (auto IDL publish enabled)"
(
    cd "$ENGINE_DIR"
    SQLX_OFFLINE="$SQLX_OFFLINE" cargo run -p solana-listener --features solana-e2e --bin solana_poc_runner -- \
        --rpc-url "$RPC_URL" \
        --wallet "$WALLET" \
        --program-id "$PROGRAM_ID"
)

if [[ "$VALIDATOR_STARTED" == "true" && "$KEEP_VALIDATOR" == "true" ]]; then
    log_info "Validator kept alive for Explorer inspection (pid: $VALIDATOR_PID)"
    log_info "Stop it with: kill $VALIDATOR_PID"
    if [[ "$CLEANUP_LEDGER" == "true" ]]; then
        log_info "Ledger is retained while validator is running. It will be removed only after validator stop."
    fi
fi
