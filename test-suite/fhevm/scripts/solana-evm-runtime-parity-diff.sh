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
ENGINE_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"
E2E_DIR="${REPO_ROOT}/test-suite/e2e"
SQLX_OFFLINE="${SQLX_OFFLINE:-true}"
EVM_NETWORK="${EVM_NETWORK:-zwsDev}"

if ! command -v cargo >/dev/null 2>&1; then
    log_error "Missing required binary: cargo"
    exit 1
fi
if ! command -v npx >/dev/null 2>&1; then
    log_error "Missing required binary: npx"
    exit 1
fi
if ! (cd "$E2E_DIR" && npx --no-install hardhat --version >/dev/null 2>&1); then
    log_error "Hardhat is not available in test-suite/e2e. Run npm install in test-suite/e2e first."
    exit 1
fi

log_info "Running Solana runtime parity slice (add + allow + decrypt)"
SOLANA_OUTPUT="$(
    cd "$ENGINE_DIR"
    SQLX_OFFLINE="$SQLX_OFFLINE" cargo test \
        -p solana-listener \
        --features solana-e2e \
        --test localnet_harness_integration \
        localnet_solana_request_add_runtime_parity_value \
        -- \
        --ignored \
        --nocapture \
        --test-threads=1
)"
SOLANA_VALUE="$(printf '%s\n' "$SOLANA_OUTPUT" | rg -o 'SOLANA_RUNTIME_PARITY_VALUE=[0-9]+' | tail -n1 | cut -d= -f2 || true)"
if [[ -z "$SOLANA_VALUE" ]]; then
    log_error "Unable to extract SOLANA_RUNTIME_PARITY_VALUE"
    exit 1
fi
log_info "Solana decrypted value: $SOLANA_VALUE"

log_info "Running EVM smoke runtime slice (add42 + decrypt)"
EVM_OUTPUT="$(
    cd "$E2E_DIR"
    npx hardhat run --network "$EVM_NETWORK" scripts/smoke-inputflow.ts
)"
EVM_VALUE="$(printf '%s\n' "$EVM_OUTPUT" | rg -o 'SMOKE_DECRYPT_VALUE=[0-9]+' | tail -n1 | cut -d= -f2 || true)"
if [[ -z "$EVM_VALUE" ]]; then
    log_error "Unable to extract SMOKE_DECRYPT_VALUE"
    exit 1
fi
log_info "EVM decrypted value: $EVM_VALUE"

if [[ "$SOLANA_VALUE" != "$EVM_VALUE" ]]; then
    log_error "Runtime parity mismatch: solana=$SOLANA_VALUE evm=$EVM_VALUE"
    exit 1
fi

log_info "Runtime parity OK: solana=$SOLANA_VALUE evm=$EVM_VALUE"
