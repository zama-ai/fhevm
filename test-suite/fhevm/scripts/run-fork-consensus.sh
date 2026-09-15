#!/usr/bin/env bash
# Competing-branch consensus and authorization (F1-F3).
# Repair and replay campaigns are delivered by the failure-mode layer.
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENV_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/env"
# shellcheck source=lib/service-control.sh
source "${SCRIPT_DIR}/lib/service-control.sh"
# shellcheck source=lib/case-result.sh
source "${SCRIPT_DIR}/lib/case-result.sh"
# shellcheck source=lib/suite-identity.sh
source "${SCRIPT_DIR}/lib/suite-identity.sh"
source "${SCRIPT_DIR}/lib/runner-assertions.sh"
source "${SCRIPT_DIR}/lib/suite-process.sh"
source "${SCRIPT_DIR}/lib/result-staging.sh"
sc_init || exit 1
sp_init || exit 1
cleanup_on_exit() {
  local status=$? cleanup_ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  if ! sp_cancel_all || ! sp_recover_suite_state; then
    rs_finalize_results 1 failed; exit 1
  fi
  sc_run_restores || cleanup_ok=0
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  if [[ "$cleanup_ok" == 1 ]]; then
    rs_finalize_results "$status" ok || { [[ "$status" != 0 ]] || status=1; }
    [[ -f "$SP_RUNTIME_DIR/cleanup-failed" ]] || sp_dispose || status=1
  else
    rs_finalize_results 1 failed
    status=1
  fi
  exit "$status"
}

trap cleanup_on_exit EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
readonly TEST_NETWORK="${TEST_NETWORK:-staging}"
readonly HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"

CASE=all
FORK_OPERATOR="${FORK_OPERATOR_INDEX:-2}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --case) CASE="${2:?--case needs a value}"; shift 2 ;;
    --fork-operator) FORK_OPERATOR="${2:?--fork-operator needs an index}"; shift 2 ;;
    *) echo "usage: run-fork-consensus.sh [--case main|all] [--fork-operator <index>]" >&2; exit 2 ;;
  esac
done
case "$CASE" in main|all) ;; *) echo "unknown case $CASE" >&2; exit 2 ;; esac

die() { RS_FINAL_FAILURE=1; echo "fork-consensus: $*" >&2; exit 1; }
log() { printf '\n=== %s\n' "$*"; }
env_value() { sed -n "s/^$1=//p" "$2" | tail -1; }

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

fork_prefix() {
  [[ "$FORK_OPERATOR" == 0 ]] && echo coprocessor || echo "coprocessor${FORK_OPERATOR}"
}

run_phase() {
  local -n __phase_out="$1"
  local phase="$2" marker="$3"
  local coprocessor_env="$ENV_DIR/coprocessor.env"
  cr_run_suite __phase_out "$marker" \
    sp_exec \
      -e RUN_FORK_CONSENSUS=1 \
      -e "FORK_PHASE=$phase" \
      -e "FORK_OPERATOR_INDEX=$FORK_OPERATOR" \
      -e "COPROCESSOR_COUNT=$(operator_count)" \
      -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" \
      -e CONSENSUS_WATCHDOG_DISABLED=1 \
      -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL "$coprocessor_env")" \
      -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS "$coprocessor_env")" \
      -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")" \
      -e "TFHE_WORKER_METRICS_URLS=$(gpu_worker_metrics_urls "$(operator_count)" "$TEST_CONTAINER" coprocessor-and-kms-db)" \
      -e npm_config_update_notifier=false \
      "$TEST_CONTAINER" \
      npx hardhat test test/consensus/forkConsensus.ts --network "$TEST_NETWORK"
}

FAILURES=0

case_main() {
  sp_case_start FORK-01-COLLIDING-HANDLE FORK-02-DISTINCT-HANDLES FORK-03-ORPHAN-ALLOW-INERT || die "cannot establish fork deadline"
  local started; started="$(cr_now)"
  local out status=0
  log "F1 to F3 on the fork topology"
  run_phase out main "[fork-consensus/F3] CASE COMPLETE" || status=$?
  local -a cases=(FORK-01-COLLIDING-HANDLE FORK-02-DISTINCT-HANDLES FORK-03-ORPHAN-ALLOW-INERT)
  local id
  if [[ "$status" -eq 0 ]]; then
    # The branch divergence is this family's fault, and a record without it
    # cannot be told from a run on a single chain.
    local -a fork_evidence=()
    local diverged
    diverged="$(grep -o 'branches diverged: handle 0x[0-9a-f]* canonical 0x[0-9a-f]* fork 0x[0-9a-f]* at [0-9TZ:.-]*' <<<"$out" | tail -1)"
    if [[ -n "$diverged" ]]; then
      fork_evidence+=(
        workload="$(awk '{print $4}' <<<"$diverged")"
        fault_observed_at="$(awk '{print $NF}' <<<"$diverged")"
        artifact="canonical_block=$(awk '{print $6}' <<<"$diverged")"
        artifact="fork_block=$(awk '{print $8}' <<<"$diverged")"
      )
    fi
    for id in "${cases[@]}"; do
      cr_record_checked_pass "$id" started_at="$started" cleanup=ok "${fork_evidence[@]}" \
        assert="assertions-ran=pass:the phase printed every case marker"
    done
    echo "[fork main] PASS: ${cases[*]}"
  else
    echo "$out" | tail -30
    # Attribute the failure to the case whose marker is missing, so a reader can
    # see which of the three did not complete rather than all three failing
    # together.
    local reason; reason="$(cr_failure_reason "$out")"
    for id in F1 F2 F3; do
      local case_id
      case "$id" in
        F1) case_id=FORK-01-COLLIDING-HANDLE ;;
        F2) case_id=FORK-02-DISTINCT-HANDLES ;;
        F3) case_id=FORK-03-ORPHAN-ALLOW-INERT ;;
      esac
      if grep -qF "[fork-consensus/${id}] CASE COMPLETE" <<<"$out"; then
        cr_record_checked_pass "$case_id" started_at="$started" cleanup=ok assert="assertions-ran=pass"
      else
        cr_record "$case_id" FAIL started_at="$started" cleanup=ok detail="$reason"
        FAILURES=$((FAILURES + 1))
      fi
    done
  fi
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR"
  docker inspect "$TEST_CONTAINER" >/dev/null 2>&1 || die "test container $TEST_CONTAINER is not running"
  local count; count="$(operator_count)"
  [[ "$count" -ge 3 ]] || die "the fork gate needs three operators, found $count"
  CONSENSUS_OPERATORS="$count"
  CONSENSUS_THRESHOLD="${CONSENSUS_THRESHOLD:-$count}"
  CONSENSUS_SCENARIO="${CONSENSUS_SCENARIO:-three-of-three-fork}"
  cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
  rs_stage_results || exit 1
  # The image bakes the e2e suite in, so a commit since the last build does not
  # reach the container. A result labelled with a revision whose code never ran
  # is worse than no result.
  suite_identity_assert "$TEST_CONTAINER" || die "the test container is not running this working tree's e2e suite"
  export CONSENSUS_RUN_STARTED_AT="$(cr_now)"

  # The fork cases need a stack with two host chains; on any other stack they
  # are declined rather than judged.
  if cr_skip_wrong_scenario FORK-01-COLLIDING-HANDLE; then
    local declined
    for declined in FORK-02-DISTINCT-HANDLES FORK-03-ORPHAN-ALLOW-INERT; do
      cr_skip_wrong_scenario "$declined" >/dev/null
    done
    echo "run $CR_RUN_ID: the fork cases need the $(cr_declared_scenario FORK-01-COLLIDING-HANDLE) scenario"
    return 0
  fi

  [[ "$CASE" == all || "$CASE" == main ]] && case_main
  log "results"
  echo "run $CR_RUN_ID: $FAILURES failing case(s)"
  echo "structured results: ${RS_PUBLISH_RESULTS:-$(dirname "$(cr_results_file)")}/$CR_RUN_ID.jsonl"
  if [[ "${CR_RECORD_FAILURES:-0}" -gt 0 ]]; then
    echo "${CR_RECORD_FAILURES} result(s) were REFUSED and not written; this run has no record of them" >&2
    return 1
  fi
  [[ "$FAILURES" -eq 0 ]]
}

main
