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

DATABASE_URL="postgresql://postgres:postgres@localhost:5432/coprocessor"
TENANT_ID=1
HOST_CHAIN_ID=4242
EXPECTED_COMPUTATIONS=1
EXPECTED_ALLOWED=1
EXPECTED_PBS=1
MIN_CURSOR=1

while [[ $# -gt 0 ]]; do
    case "$1" in
        --database-url)
            DATABASE_URL="$2"
            shift 2
            ;;
        --tenant-id)
            TENANT_ID="$2"
            shift 2
            ;;
        --host-chain-id)
            HOST_CHAIN_ID="$2"
            shift 2
            ;;
        --expected-computations)
            EXPECTED_COMPUTATIONS="$2"
            shift 2
            ;;
        --expected-allowed)
            EXPECTED_ALLOWED="$2"
            shift 2
            ;;
        --expected-pbs)
            EXPECTED_PBS="$2"
            shift 2
            ;;
        --min-cursor)
            MIN_CURSOR="$2"
            shift 2
            ;;
        *)
            log_error "Unknown argument: $1"
            exit 1
            ;;
    esac
done

if ! command -v psql >/dev/null 2>&1; then
    log_error "psql not found. Install PostgreSQL client first."
    exit 1
fi

run_scalar_query() {
    local sql="$1"
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -tA -c "$sql"
}

assert_eq() {
    local actual="$1"
    local expected="$2"
    local label="$3"
    if [[ "$actual" != "$expected" ]]; then
        log_error "$label failed: expected=$expected actual=$actual"
        exit 1
    fi
    log_info "$label OK: $actual"
}

assert_ge() {
    local actual="$1"
    local expected="$2"
    local label="$3"
    if (( actual < expected )); then
        log_error "$label failed: expected>=$expected actual=$actual"
        exit 1
    fi
    log_info "$label OK: $actual"
}

log_info "Running Tier 1 DB assertions"
log_info "DATABASE_URL=$DATABASE_URL"
log_info "TENANT_ID=$TENANT_ID HOST_CHAIN_ID=$HOST_CHAIN_ID"

computations_count="$(run_scalar_query "SELECT COUNT(*) FROM computations WHERE tenant_id = ${TENANT_ID};")"
allowed_count="$(run_scalar_query "SELECT COUNT(*) FROM allowed_handles WHERE tenant_id = ${TENANT_ID};")"
pbs_count="$(run_scalar_query "SELECT COUNT(*) FROM pbs_computations WHERE tenant_id = ${TENANT_ID};")"
cursor_value="$(run_scalar_query "SELECT COALESCE(MAX(last_caught_up_block), -1) FROM host_listener_poller_state WHERE chain_id = ${HOST_CHAIN_ID};")"

assert_eq "$computations_count" "$EXPECTED_COMPUTATIONS" "computations count"
assert_eq "$allowed_count" "$EXPECTED_ALLOWED" "allowed_handles count"
assert_eq "$pbs_count" "$EXPECTED_PBS" "pbs_computations count"
assert_ge "$cursor_value" "$MIN_CURSOR" "poller cursor"

log_info "Tier 1 assertions passed."
