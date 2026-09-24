#!/usr/bin/env bash
# The GPU lifecycle cases, recorded as results (GPU-01, GPU-02, GPU-03, GPU-04).
#
# These four ran only as bare workflow steps: a green step said the command
# exited 0 and nothing else, and the aggregate reported them NOT_RUN because no
# result was ever written. That is the same hole the rest of this exercise
# closed -- an outcome nobody can check after the fact -- so each case now
# carries a structured record with the marker its own runner prints.
#
# The markers matter more than the exit status here. `fhevm-cli test --profile
# coprocessor-db-state-revert` exits 0 whether or not it stopped the writers
# first, which is precisely the defect GPU-02 exists to catch, so the record
# requires the quiescence line as well.
#
#   scripts/run-gpu-lifecycle-cases.sh [--case gpu-01|gpu-02|gpu-03|gpu-04|all]
#                                      [--operator N]
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly CLI_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
readonly ENV_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/env"

# shellcheck source=lib/case-result.sh
source "${SCRIPT_DIR}/lib/case-result.sh"
source "${SCRIPT_DIR}/lib/runner-assertions.sh"
# shellcheck source=lib/gpu-session.sh
source "${SCRIPT_DIR}/lib/gpu-session.sh"
# shellcheck source=lib/service-control.sh
source "${SCRIPT_DIR}/lib/service-control.sh"
sc_init || exit 1
source "$SCRIPT_DIR/lib/suite-process.sh"
source "$SCRIPT_DIR/lib/result-staging.sh"
source "$SCRIPT_DIR/lib/gpu02-transport.sh"
sp_init || exit 1
GPU_COMMAND_TOKEN=""
GPU_COMMAND_PID=""
GPU_BASELINE="$SP_RUNTIME_DIR/gpu-baseline"
gpu_cancel_command() {
  [[ -n "$GPU_COMMAND_TOKEN" ]] || return 0
  hc_run node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" cancel "$GPU_COMMAND_TOKEN" || return 1
  [[ -z "$GPU_COMMAND_PID" ]] || wait "$GPU_COMMAND_PID" 2>/dev/null || true
  GPU_COMMAND_PID=""; GPU_COMMAND_TOKEN=""
}
cleanup_gpu_cases() {
  local status=$? cleanup=ok
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  # Cancel a helper's whole host process group before retrying its restores.
  if ! gpu_cancel_command || ! gpu02_stop_remote; then rs_finalize_results 1 failed; exit 1; fi
  sc_run_restores || cleanup=failed
  if [[ -f "$GPU_BASELINE" ]]; then sc_restore_running "$GPU_BASELINE" || cleanup=failed; fi
  if [[ "$cleanup" == ok ]] && ! gpu02_dispose_remote; then cleanup=failed; fi
  if [[ "$cleanup" == failed ]]; then status=1; fi
  rs_finalize_results "$status" "$cleanup" || { [[ "$status" != 0 ]] || status=1; }
  if [[ "$cleanup" == ok && ! -f "$SP_RUNTIME_DIR/cleanup-failed" ]]; then
    sp_cancel_all && sp_dispose || status=1
    rm -f "$SC_RESTORE_LOG"
  fi
  exit "$status"
}
trap cleanup_gpu_cases EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
gpu_normalise_user_bus

CASE=all
OPERATOR=1
while [[ $# -gt 0 ]]; do
  case "$1" in
    --case) CASE="${2:?--case needs a value}"; shift 2 ;;
    --operator) OPERATOR="${2:?--operator needs an index}"; shift 2 ;;
    *) echo "usage: run-gpu-lifecycle-cases.sh [--case gpu-01|gpu-02|gpu-03|gpu-04|all] [--operator N]" >&2; exit 2 ;;
  esac
done

die() { RS_FINAL_FAILURE=1; echo "gpu-lifecycle: $*" >&2; exit 1; }
log() { printf '\n=== %s\n' "$*"; }

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

FAILURES=0

# A GPU session must be serving the queues, or none of these cases is about the
# thing it names. Recorded NOT_APPLICABLE rather than skipped, because a case
# that silently does not run is what the inventory exists to prevent.
#
# The session marker alone is not enough: it says a session was started, not
# that units are running now. At least one worker unit must be active, which is
# also what makes the difference between reading unit journals and reading a
# stopped container's logs.
session_units_active() {
  gpu_session_active || return 1
  local index kind
  for index in $(seq 0 $(($(operator_count) - 1))); do
    for kind in tfhe zkproof sns; do
      [[ "$(systemctl --user show "fhevm-gpu-consensus-${kind}-${index}" \
              --property=ActiveState --value 2>/dev/null)" == active ]] && return 0
    done
  done
  return 1
}

require_session() {
  local case_id="$1"
  session_units_active && return 0
  cr_record "$case_id" NOT_APPLICABLE started_at="$(cr_now)" cleanup=not_required \
    detail="no GPU worker session is serving the queues, so this case's subject does not exist on this stack"
  echo "[$case_id] NOT_APPLICABLE: no GPU session"
  return 1
}

# Runs one command, keeps its exit status AND requires every marker it must
# print. Sets CASE_OUT.
run_with_markers() {
  local -n __status="$1"; shift
  local -a markers=()
  while [[ "$1" != -- ]]; do markers+=("$1"); shift; done
  shift
  CASE_OUT=""
  __status=0
  MISSING_MARKER="phase did not start"
  local budget output
  budget="$(bun "$SCRIPT_DIR/consensus-inventory.ts" show "${id:?case identity required}" | sed -n 's/^timeout: *\([0-9]*\)s$/\1/p')" || { __status=1; return 1; }
  case_deadline_start "$budget" || { __status=1; return 1; }
  output="$(mktemp "$SP_RUNTIME_DIR/gpu-output.XXXXXX")" || { __status=1; return 1; }
  GPU_COMMAND_TOKEN="gpu_case_${BASHPID}_${RANDOM}_$(date +%s%N)"
  node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run "$GPU_COMMAND_TOKEN" "$((CASE_DEADLINE_EPOCH * 1000))" "$@" >"$output" 2>&1 &
  GPU_COMMAND_PID=$!
  wait "$GPU_COMMAND_PID" || __status=$?
  GPU_COMMAND_PID=""
  if ! gpu_cancel_command; then __status=1; GPU_LIFECYCLE_CLEANUP_FAILED=1; fi
  CASE_OUT="$(cat "$output")"
  rm -f "$output"
  echo "$CASE_OUT" | sed 's/^/    /'
  local marker
  for marker in "${markers[@]}"; do
    if ! grep -qF "$marker" <<<"$CASE_OUT"; then
      MISSING_MARKER="$marker"
      return 1
    fi
  done
  MISSING_MARKER=""
  return 0
}

record_from() {
  local case_id="$1" status="$2" started="$3" missing="$4"; shift 4
  if [[ "$status" -ne 0 ]]; then
    # A FAIL must always carry a reason: the recorder refuses one without, and a
    # refused record leaves no row while the log says the case failed. When the
    # output has no quotable failure line, its last words are the reason.
    local reason; reason="$(cr_failure_reason "$CASE_OUT")"
    [[ -n "$reason" ]] || reason="exited $status with no recognisable failure line; last output: $(tail -3 <<<"$CASE_OUT" | tr '\n' ' ' | cut -c1-200)"
    cr_record "$case_id" FAIL started_at="$started" cleanup=ok \
      detail="$reason" "$@"
    echo "[$case_id] FAIL"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  if [[ -n "$missing" ]]; then
    cr_record "$case_id" NOT_RUN started_at="$started" cleanup=ok \
      detail="the runner exited 0 without evidence that this case ran (no '$missing' in its output)"
    echo "[$case_id] NOT_RUN: '$missing' never printed"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  cr_record_checked_pass "$case_id" started_at="$started" cleanup=ok "$@" || { FAILURES=$((FAILURES + 1)); return 1; }
  echo "[$case_id] PASS"
  return 0
}

# ---------------------------------------------------------------- GPU-01
case_gpu01() {
  local id=GPU-01-MIXED-BACKEND-GUARD
  cr_skip_wrong_scenario "$id" && return 0
  require_session "$id" || return 1
  local started; started="$(cr_now)"
  log "$id: the mixed-backend guard, on a split created on purpose"
  local status=0
  run_with_markers status \
    "baseline clean: no queue served twice" \
    "guard fired and named the conflict" \
    "the readiness exclusivity gate refuses the split too" \
    "guard clears once the queue has one worker again" \
    -- env "SC_RESTORE_LOG=$SC_RESTORE_LOG" "$SCRIPT_DIR/run-mixed-backend-guard.sh" --operator "$OPERATOR" || true
  # The child may fail before its final marker or during EXIT restoration.
  # Inspect the queue again; a nonzero child exit never implies cleanup=ok.
  if ! hc_run "$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1 ||
     ! hc_run "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$(operator_count)" >/dev/null 2>&1; then
    GPU_LIFECYCLE_CLEANUP_FAILED=1
    cr_record "$id" FAIL started_at="$started" cleanup=failed \
      cleanup_detail="single-owner queues were not re-established after the mixed-backend guard" \
      detail="GPU guard recovery remains unverified; refusing later lifecycle cases"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  record_from "$id" "$status" "$started" "${MISSING_MARKER:-}" \
    fault_observed_at="$(cr_now)" \
    workload="queue:operator-${OPERATOR}-tfhe" \
    assert="precondition=pass:baseline queues independently checked single-owner" \
    assert="fault=pass:the guard named the deliberately duplicated queue" \
    assert="liveness=pass:queue recovered after removing the duplicate" \
    assert="safety=pass:both launcher and readiness refused the duplicate" \
    assert="cleanup=pass:single-owner readiness rechecked after child cleanup" \
    assert="guard-fires=pass:names the doubly-served queue" \
    assert="readiness-refuses=pass" \
    assert="guard-clears=pass:tracks state rather than latching"
}

# ---------------------------------------------------------------- GPU-02
case_gpu02() {
  local id=GPU-02-DB-REVERT-QUIESCENCE
  cr_skip_wrong_scenario "$id" && return 0
  require_session "$id" || return 1
  local started; started="$(cr_now)"
  log "$id: the database revert must stop the GPU writers first"
  gpu02_transport_init || die "cannot establish GPU02 remote recovery ownership"
  local status=0
  run_with_markers status \
    "[revert] quiesced" \
    "verified stopped" \
    "[revert] restored" \
    "one per queue" \
    -- env "PATH=$GPU02_SHIM_DIR:$PATH" "$CLI_DIR/fhevm-cli" test coprocessor-db-state-revert || true
  # A failed attached Docker client is not proof that the remote work ended.
  hc_begin_cleanup || die "cannot establish remote cleanup deadline"
  if ! gpu02_stop_remote; then
    GPU_LIFECYCLE_CLEANUP_FAILED=1
    die "GPU02 remote work remains unverified; retaining stopped writers"
  fi
  # Recorded once, after the postconditions below: a case that writes PASS and
  # then FAIL leaves the aggregate resolving a conflict it should never have
  # been given.
  if [[ "$status" -ne 0 || -n "${MISSING_MARKER:-}" ]]; then
    record_from "$id" "$status" "$started" "${MISSING_MARKER:-}" \
      assert="writers-quiesced-before-sql=fail"
    return 1
  fi
  # The postcondition the profile itself cannot check: the queues are single
  # owner afterwards, judged by the readiness gate rather than by the profile's
  # own report.
  if ! "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$(operator_count)"; then
    cr_record "$id" FAIL started_at="$started" cleanup=failed \
      cleanup_detail="a queue is served twice after the revert restored the writers" \
      detail="the revert left the fleet split across backends"
    echo "[$id] FAIL: the fleet is split after the revert"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  # And the half exclusivity cannot see: one worker per queue is satisfied by a
  # fleet where different operators run different implementations, which is
  # precisely how identical compute digests end up with different SNS digests.
  local kind
  for kind in tfhe sns zkproof; do
    sc_fleet_homogeneous "$kind" "$(operator_count)" && continue
    # The one real defect this inventory has found: the revert's restore brings
    # writers back on mixed implementations, which is how identical compute
    # digests end up with different SNS digests.
    cr_record "$id" FAIL started_at="$started" cleanup=failed \
      cleanup_detail="after the revert the operators no longer run the same ${kind} implementation" \
      detail="the revert restored a fleet split across implementations for ${kind}; a byte comparison after this measures the restore, not the fleet"
    echo "[$id] FAIL: the fleet is split across ${kind} implementations after the revert"
    FAILURES=$((FAILURES + 1))
    return 1
  done
  cr_record_checked_pass "$id" started_at="$started" cleanup=ok \
    assert="safety=pass:verified stopped writers before SQL and homogeneous fleet afterwards" \
    assert="cleanup=pass:remote SQL quiescence and restored single-owner fleet verified" \
    assert="writers-restored-one-per-queue=pass" \
    assert="fleet-homogeneous-after-restore=pass"
  echo "[$id] PASS"
}

# ---------------------------------------------------------------- GPU-03
case_gpu03() {
  local id=GPU-03-CONFIG-PRESERVING-RESTART
  cr_skip_wrong_scenario "$id" && return 0
  require_session "$id" || return 1
  local started; started="$(cr_now)"
  log "$id: a stopped heterogeneous operator returns with its recorded tuning"
  # Deliberately run from an environment WITHOUT the override variables: that
  # is the situation in which re-resolving the configuration would silently
  # revert operator $OPERATOR to fleet defaults.
  local status=0
  run_with_markers status \
    "restored with identical configuration" \
    "invocation " \
    -- env \
      -u "GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE_${OPERATOR}" \
      -u "GPU_CONSENSUS_COMPONENTS_PER_BATCH_${OPERATOR}" \
      -u "GPU_CONSENSUS_ADAPTIVE_BATCH_EXECUTION_${OPERATOR}" \
      -u "GPU_CONSENSUS_DEVICE_${OPERATOR}" \
      "$SCRIPT_DIR/gpu-consensus-workers.sh" verify-restore tfhe "$OPERATOR" || true
  if grep -qF 'cleanup failed for' <<<"$CASE_OUT"; then
    GPU_LIFECYCLE_CLEANUP_FAILED=1
    cr_record "$id" FAIL started_at="$started" cleanup=failed \
      cleanup_detail="the interrupted GPU owner could not be restored; session ownership retained for retry" \
      detail="GPU restart verification failed and cleanup could not restore the operator"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  record_from "$id" "$status" "$started" "${MISSING_MARKER:-}" \
    fault_observed_at="$(cr_now)" \
    workload="unit:fhevm-gpu-consensus-tfhe-${OPERATOR}" \
    assert="safety=pass:identical configuration and invocation metadata checked by verify-restore" \
    assert="cleanup=pass:verify-restore completed and verified original owner restored" \
    assert="no-revert-to-fleet-defaults=pass:restored from a shell without the overrides" \
    assert="invocation-metadata-refreshed=pass"
}

# ---------------------------------------------------------------- GPU-04
case_gpu04() {
  local id=GPU-04-LOCK-LOG-VALIDITY
  cr_skip_wrong_scenario "$id" && return 0
  require_session "$id" || return 1
  local started; started="$(cr_now)"
  log "$id: the lock gate reads the active backend's logs"
  local since="${CONSENSUS_RUN_STARTED_AT:-$started}"
  local status=0
  run_with_markers status \
    "validity: read lock-extension evidence since" \
    "(unit)" \
    -- "$SCRIPT_DIR/consensus-validity.sh" locks --since "$since" --operators "$(operator_count)" || true
  # `(unit)` is the load-bearing half: under a GPU session the containers are
  # stopped, so a gate that read container logs would report a clean fleet
  # having read nothing at all.
  record_from "$id" "$status" "$started" "${MISSING_MARKER:-}" \
    assert="safety=pass:lock gate read live unit journals and found no loss in the selected window" \
    assert="window-scoped=pass:evidence collected since $since" \
    artifact="log_sources=$(sed -n 's/.*evidence since [^ ]* from //p' <<<"$CASE_OUT" | head -1)"
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR"
  CONSENSUS_OPERATORS="$(operator_count)"
  CONSENSUS_THRESHOLD="${CONSENSUS_THRESHOLD:-$CONSENSUS_OPERATORS}"
  CONSENSUS_SCENARIO="${CONSENSUS_SCENARIO:-unknown}"
  CONSENSUS_BACKEND_CLASS="${CONSENSUS_BACKEND_CLASS:-gpu-cuda}"
  cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
  rs_stage_results || exit 1
  sc_snapshot_running > "$GPU_BASELINE" || die "cannot capture prior GPU lifecycle owners"
  export CONSENSUS_RUN_STARTED_AT="${CONSENSUS_RUN_STARTED_AT:-$(cr_now)}"

  case "$CASE" in
    gpu-01) case_gpu01 ;;
    gpu-02) case_gpu02 ;;
    gpu-03) case_gpu03 ;;
    gpu-04) case_gpu04 ;;
    all)
      # GPU-01 first, while the fleet is idle: it starts a second worker on one
      # queue on purpose, and doing that with work in flight is how B-1 made
      # disagreeing ct128 in the first place. GPU-04 last, so its window covers
      # the cases that ran before it.
      case_gpu01 || true
      [[ "${GPU_LIFECYCLE_CLEANUP_FAILED:-0}" != 1 ]] || return 1
      case_gpu03 || true
      [[ "${GPU_LIFECYCLE_CLEANUP_FAILED:-0}" != 1 ]] || return 1
      case_gpu02 || true
      [[ "${GPU_LIFECYCLE_CLEANUP_FAILED:-0}" != 1 ]] || return 1
      case_gpu04 || true
      ;;
    *) die "unknown --case $CASE" ;;
  esac

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
