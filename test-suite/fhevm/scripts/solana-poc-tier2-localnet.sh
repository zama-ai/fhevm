#!/bin/bash

set -euo pipefail

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
SOLANA_PROGRAM_DIR="${REPO_ROOT}/solana/host-programs"
LISTENER_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"
LEDGER_DIR="${REPO_ROOT}/.solana-ledger-poc"
VALIDATOR_LOG="${REPO_ROOT}/.solana-validator-poc.log"
SQLX_OFFLINE="${SQLX_OFFLINE:-true}"

if [[ $# -gt 0 ]]; then
    log_error "This script accepts no arguments"
    exit 1
fi

for bin in cargo solana-test-validator anchor; do
    if ! command -v "$bin" >/dev/null 2>&1; then
        log_error "Missing required binary: $bin"
        exit 1
    fi
done

cleanup() {
    if [[ -n "${VALIDATOR_PID:-}" ]]; then
        kill "$VALIDATOR_PID" >/dev/null 2>&1 || true
    fi
}
trap cleanup EXIT

log_info "Tier 2 localnet scaffold start"

log_info "Step 1/4: run Tier 0 listener mapping tests"
(
    cd "$LISTENER_DIR"
    SQLX_OFFLINE="$SQLX_OFFLINE" cargo test -p solana-listener database::ingest::tests
)

log_info "Step 2/4: build local Anchor host program"
(
    cd "$SOLANA_PROGRAM_DIR"
    anchor build
)

log_info "Step 3/4: start local validator"
mkdir -p "$LEDGER_DIR"
solana-test-validator --ledger "$LEDGER_DIR" --reset >"$VALIDATOR_LOG" 2>&1 &
VALIDATOR_PID=$!
sleep 3
if ! kill -0 "$VALIDATOR_PID" >/dev/null 2>&1; then
    log_error "solana-test-validator failed to start. Check $VALIDATOR_LOG"
    exit 1
fi
log_info "solana-test-validator is running (pid: $VALIDATOR_PID)"

log_info "Step 4/4: run finalized RPC source parser tests"
(
    cd "$LISTENER_DIR"
    SQLX_OFFLINE="$SQLX_OFFLINE" cargo test -p solana-listener poller::solana_rpc_source::tests
)

log_info "Scaffold completed successfully."
