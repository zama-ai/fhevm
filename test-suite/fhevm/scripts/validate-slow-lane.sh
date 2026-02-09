#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FHEVM_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
REPO_ROOT="$(cd "${FHEVM_DIR}/../.." && pwd)"
ENGINE_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"

COMPOSE_PROJECT="fhevm"
COMPOSE_ENV="${FHEVM_DIR}/env/staging/.env.coprocessor.local"
COMPOSE_FILE="${FHEVM_DIR}/docker-compose/coprocessor-docker-compose.yml"

RUN_INTEGRATION=true
RUN_STACK=true
CAP=1
LOAD_TEST="operators"
BOOTSTRAP_TIMEOUT_SECONDS=180

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

override_file=""
listeners_overridden=false

usage() {
  cat <<EOF
Usage: $(basename "$0") [options]

Options:
  --integration-only       Run only deterministic Rust integration assertions.
  --stack-only             Run only local-stack multi-listener SQL assertions.
  --cap N                  Slow-lane cap for stack scenario (default: 1).
  --load-test NAME         fhevm-cli test to generate load (default: operators).
  --bootstrap-timeout SEC  Timeout for key-bootstrap gate (default: 180).
  -h, --help               Show this help.
EOF
}

log() {
  printf "${BLUE}[slow-lane-validate]${RESET} %s\n" "$*"
}

warn() {
  printf "${YELLOW}[slow-lane-validate]${RESET} %s\n" "$*"
}

die() {
  printf "${RED}[slow-lane-validate] %s${RESET}\n" "$*" >&2
  exit 1
}

compose() {
  ensure_compose_versions
  docker compose -p "${COMPOSE_PROJECT}" \
    --env-file "${COMPOSE_ENV}" \
    -f "${COMPOSE_FILE}" \
    "$@"
}

compose_with_override() {
  local file="$1"
  shift
  ensure_compose_versions
  docker compose -p "${COMPOSE_PROJECT}" \
    --env-file "${COMPOSE_ENV}" \
    -f "${COMPOSE_FILE}" \
    -f "${file}" \
    "$@"
}

infer_container_version() {
  local container="$1"
  local fallback="$2"
  local image
  image="$(docker inspect --format '{{.Config.Image}}' "${container}" 2>/dev/null || true)"
  if [[ -n "${image}" && "${image}" == *:* ]]; then
    printf "%s" "${image##*:}"
    return 0
  fi
  printf "%s" "${fallback}"
}

ensure_compose_versions() {
  : "${COPROCESSOR_HOST_LISTENER_VERSION:=$(infer_container_version coprocessor-host-listener v0.11.0-1)}"
  : "${COPROCESSOR_GW_LISTENER_VERSION:=$(infer_container_version coprocessor-gw-listener v0.11.0-1)}"
  : "${COPROCESSOR_TFHE_WORKER_VERSION:=$(infer_container_version coprocessor-tfhe-worker v0.11.0-1)}"
  : "${COPROCESSOR_SNS_WORKER_VERSION:=$(infer_container_version coprocessor-sns-worker v0.11.0-1)}"
  : "${COPROCESSOR_TX_SENDER_VERSION:=$(infer_container_version coprocessor-transaction-sender v0.11.0-1)}"
  : "${COPROCESSOR_ZKPROOF_WORKER_VERSION:=$(infer_container_version coprocessor-zkproof-worker v0.11.0-1)}"
  : "${COPROCESSOR_DB_MIGRATION_VERSION:=$(infer_container_version coprocessor-db-migration v0.11.0-1)}"

  export COPROCESSOR_HOST_LISTENER_VERSION
  export COPROCESSOR_GW_LISTENER_VERSION
  export COPROCESSOR_TFHE_WORKER_VERSION
  export COPROCESSOR_SNS_WORKER_VERSION
  export COPROCESSOR_TX_SENDER_VERSION
  export COPROCESSOR_ZKPROOF_WORKER_VERSION
  export COPROCESSOR_DB_MIGRATION_VERSION
}

db_query() {
  local sql="$1"
  docker exec -i coprocessor-and-kms-db \
    psql -U postgres -d coprocessor -v ON_ERROR_STOP=1 -At -c "${sql}"
}

cleanup() {
  if [[ "${listeners_overridden}" == "true" ]]; then
    warn "Restoring listener defaults (no slow-lane override)"
    compose up -d --force-recreate \
      coprocessor-host-listener \
      coprocessor-host-listener-poller >/dev/null
  fi

  if [[ -n "${override_file}" && -f "${override_file}" ]]; then
    rm -f "${override_file}"
  fi
}

trap cleanup EXIT

wait_for_bootstrap() {
  local deadline=$((SECONDS + BOOTSTRAP_TIMEOUT_SECONDS))
  while (( SECONDS < deadline )); do
    local has_activate_key
    local has_fetched_keyset
    local has_key_material

    has_activate_key="$(docker logs --since=20m coprocessor-gw-listener 2>&1 | rg -c "ActivateKey event successful" || true)"
    has_fetched_keyset="$(docker logs --since=20m coprocessor-sns-worker 2>&1 | rg -c "Fetched keyset" || true)"
    has_key_material="$(db_query "
      SELECT COALESCE(bool_and(key_bytes > 0), false)
      FROM (
        SELECT COALESCE(SUM(octet_length(lo.data)), 0) AS key_bytes
        FROM tenants t
        LEFT JOIN pg_largeobject lo
          ON lo.loid = t.sns_pk
        GROUP BY t.tenant_id
      ) s;
    ")"

    if [[ "${has_activate_key}" -gt 0 && "${has_fetched_keyset}" -gt 0 && "${has_key_material}" == "t" ]]; then
      log "Bootstrap gate passed (ActivateKey + keyset + non-empty sns_pk)"
      return 0
    fi
    sleep 3
  done

  warn "Bootstrap gate timed out, restarting gw-listener once"
  compose up -d --no-deps coprocessor-gw-listener >/dev/null

  local retry_deadline=$((SECONDS + BOOTSTRAP_TIMEOUT_SECONDS))
  while (( SECONDS < retry_deadline )); do
    local has_fetched_keyset
    has_fetched_keyset="$(docker logs --since=20m coprocessor-sns-worker 2>&1 | rg -c "Fetched keyset" || true)"
    if [[ "${has_fetched_keyset}" -gt 0 ]]; then
      log "Bootstrap recovered after gw-listener restart"
      return 0
    fi
    sleep 3
  done

  die "Bootstrap gate failed: sns-worker did not fetch keyset"
}

apply_listener_cap_override() {
  local cap="$1"
  override_file="$(mktemp)"
  cat >"${override_file}" <<EOF
services:
  coprocessor-host-listener:
    command:
      - host_listener
      - --database-url=\${DATABASE_URL}
      - --coprocessor-api-key=\${TENANT_API_KEY}
      - --acl-contract-address=\${ACL_CONTRACT_ADDRESS}
      - --tfhe-contract-address=\${FHEVM_EXECUTOR_CONTRACT_ADDRESS}
      - --url=\${RPC_WS_URL}
      - --initial-block-time=1
      - --dependent-ops-max-per-chain=${cap}

  coprocessor-host-listener-poller:
    command:
      - host_listener_poller
      - --database-url=\${DATABASE_URL}
      - --coprocessor-api-key=\${TENANT_API_KEY}
      - --acl-contract-address=\${ACL_CONTRACT_ADDRESS}
      - --tfhe-contract-address=\${FHEVM_EXECUTOR_CONTRACT_ADDRESS}
      - --url=\${RPC_HTTP_URL}
      - --dependent-ops-max-per-chain=${cap}
EOF

  compose_with_override "${override_file}" up -d --force-recreate \
    coprocessor-host-listener \
    coprocessor-host-listener-poller >/dev/null
  listeners_overridden=true
  log "Applied listener override with --dependent-ops-max-per-chain=${cap}"
}

run_integration_assertions() {
  log "Running deterministic integration assertions"
  (
    cd "${ENGINE_DIR}"
    cargo +1.91.1 test -p host-listener --test host_listener_integration_tests \
      test_slow_lane_threshold_matrix_locally -- --nocapture
    cargo +1.91.1 test -p host-listener --test host_listener_integration_tests \
      test_slow_lane_cross_block_sustained_below_cap_stays_fast_locally -- --nocapture
    cargo +1.91.1 test -p host-listener --test host_listener_integration_tests \
      test_slow_lane_off_mode_promotes_seen_chain_locally -- --nocapture
  )
  printf "${GREEN}[slow-lane-validate] Integration assertions passed${RESET}\n"
}

run_stack_assertions() {
  log "Running local-stack assertions (cap=${CAP}, load=${LOAD_TEST})"
  wait_for_bootstrap

  local before_block_height
  before_block_height="$(db_query "SELECT COALESCE(MAX(block_height), 0) FROM dependence_chain;")"
  log "Baseline block_height=${before_block_height}"

  apply_listener_cap_override "${CAP}"

  (
    cd "${FHEVM_DIR}"
    ./fhevm-cli test "${LOAD_TEST}"
  )

  local counts
  counts="$(db_query "
    SELECT
      COUNT(*) FILTER (WHERE schedule_priority = 0),
      COUNT(*) FILTER (WHERE schedule_priority = 1),
      COUNT(*)
    FROM dependence_chain
    WHERE block_height > ${before_block_height};
  ")"
  IFS='|' read -r fast_count slow_count total_count <<<"${counts}"

  log "Observed chains after baseline: total=${total_count}, fast=${fast_count}, slow=${slow_count}"

  [[ "${total_count}" -gt 0 ]] || die "No new dependence chains were ingested"
  [[ "${fast_count}" -gt 0 ]] || die "Expected at least one fast chain"
  [[ "${slow_count}" -gt 0 ]] || die "Expected at least one slow chain (raise load or lower cap)"

  local schedulable_order_head
  schedulable_order_head="$(db_query "
    SELECT schedule_priority
    FROM dependence_chain
    WHERE status = 'updated'
      AND worker_id IS NULL
      AND dependency_count = 0
      AND block_height > ${before_block_height}
    ORDER BY schedule_priority ASC, last_updated_at ASC
    LIMIT 1;
  ")"

  if [[ -n "${schedulable_order_head}" && "${schedulable_order_head}" != "0" ]]; then
    die "Expected fast-lane first in schedulable ordering, got schedule_priority=${schedulable_order_head}"
  fi

  printf "${GREEN}[slow-lane-validate] Stack assertions passed${RESET}\n"
}

while (( "$#" )); do
  case "$1" in
    --integration-only)
      RUN_INTEGRATION=true
      RUN_STACK=false
      shift
      ;;
    --stack-only)
      RUN_INTEGRATION=false
      RUN_STACK=true
      shift
      ;;
    --cap)
      [[ $# -ge 2 ]] || die "--cap requires a value"
      CAP="$2"
      shift 2
      ;;
    --load-test)
      [[ $# -ge 2 ]] || die "--load-test requires a value"
      LOAD_TEST="$2"
      shift 2
      ;;
    --bootstrap-timeout)
      [[ $# -ge 2 ]] || die "--bootstrap-timeout requires a value"
      BOOTSTRAP_TIMEOUT_SECONDS="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "Unknown argument: $1"
      ;;
  esac
done

if [[ "${RUN_INTEGRATION}" == "true" ]]; then
  run_integration_assertions
fi

if [[ "${RUN_STACK}" == "true" ]]; then
  run_stack_assertions
fi

printf "${GREEN}[slow-lane-validate] All selected checks passed${RESET}\n"
