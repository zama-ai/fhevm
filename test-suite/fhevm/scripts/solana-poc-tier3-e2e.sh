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
LISTENER_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"
SQLX_OFFLINE="${SQLX_OFFLINE:-true}"
CASE="${CASE:-all}"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --case)
            CASE="$2"
            shift 2
            ;;
        *)
            log_error "Unknown argument: $1"
            exit 1
            ;;
    esac
done

if [[ "$CASE" != "emit" && "$CASE" != "emit-cpi" && "$CASE" != "acl" && "$CASE" != "all" ]]; then
    log_error "Invalid --case value: $CASE (allowed: emit | emit-cpi | acl | all)"
    exit 1
fi

for bin in cargo anchor solana-test-validator docker; do
    if ! command -v "$bin" >/dev/null 2>&1; then
        log_error "Missing required binary: $bin"
        exit 1
    fi
done

run_test_case() {
    local test_name="$1"
    log_info "Running ${test_name}"
    (
        cd "$LISTENER_DIR"
        SQLX_OFFLINE="$SQLX_OFFLINE" cargo test \
            -p solana-listener \
            --features solana-e2e \
            --test localnet_harness_integration \
            "${test_name}" \
            -- \
            --ignored \
            --nocapture \
            --test-threads=1
    )
}

log_info "Tier 3 e2e start (case: $CASE, SQLX_OFFLINE=$SQLX_OFFLINE)"

if [[ "$CASE" == "emit" || "$CASE" == "all" ]]; then
    run_test_case "localnet_solana_request_add_computes_and_decrypts"
fi

if [[ "$CASE" == "emit-cpi" || "$CASE" == "all" ]]; then
    run_test_case "localnet_solana_request_add_cpi_computes_and_decrypts"
fi

if [[ "$CASE" == "acl" || "$CASE" == "all" ]]; then
    run_test_case "localnet_acl_gate_blocks_then_allows_compute"
fi

log_info "Tier 3 e2e completed."
