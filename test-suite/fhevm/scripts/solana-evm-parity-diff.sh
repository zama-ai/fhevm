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
ENGINE_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"
SQLX_OFFLINE="${SQLX_OFFLINE:-true}"

if ! command -v cargo >/dev/null 2>&1; then
    log_error "Missing required binary: cargo"
    exit 1
fi

log_info "Running Solana vs EVM ingest parity diff test"
(
    cd "$ENGINE_DIR"
    SQLX_OFFLINE="$SQLX_OFFLINE" cargo test \
        -p solana-listener \
        database::ingest::tests::parity_diff_matches_evm_semantics_for_v0_surface
)

log_info "Parity diff test completed."
