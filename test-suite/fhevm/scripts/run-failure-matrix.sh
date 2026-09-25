#!/usr/bin/env bash
# The failure matrix: fault one service while IDENTIFIED work depends on it, and
# require that work to survive.
#
# What this replaces, and why. Every cell used to be the same three steps --
# inject, heal, mint a fresh handle and compare bytes across operators. That is
# worth having and it is a smoke test: nothing was in flight when the service
# went away, so no cell could detect work lost, duplicated or stranded across
# the outage, which is the failure mode a failure matrix exists for. Two of the
# cells were worse than uninformative: GPU stall injection called a helper that
# did not exist, so `fault_pause` and `heal_unpause` were silent no-ops and the
# cells reported PASS against a worker that had never been interrupted.
#
# So each cell now has a shape rather than a script:
#
#   1. Freeze the selected service and verify that it is stopped.
#   2. Arm identified work and prove it depends on the frozen service.
#   3. For crash cells, kill that process and observe supervisor replacement;
#      for stalls, resume the service and verify it is running.
#   4. Require that same work to complete, with service-specific assertions and
#      byte agreement across operators.
#
# Every injection and every heal is checked, both for the command's status and
# for an independent postcondition. A cell that cannot establish its stage
# reports INVALID; it never reports PASS.
#
#   run-failure-matrix.sh [--column crash|stall|data|db|smoke|all] [--case <id>]
#                         [--list] [--keep-going] [--victim <index>]
#
# `--column all` covers crash, stall and data. The db column is excluded from it
# on purpose: taking the shared database away affects every operator at once, so
# those cells run in their own job (see `failure-matrix-db` in the inventory)
# where a wedged stack cannot contaminate results that were otherwise going to
# complete.
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
# shellcheck source=lib/case-deadline.sh
source "${SCRIPT_DIR}/lib/case-deadline.sh"
# shellcheck source=lib/suite-process.sh
source "${SCRIPT_DIR}/lib/runner-assertions.sh"
source "${SCRIPT_DIR}/lib/suite-process.sh"
# shellcheck source=lib/crash-controls.sh
source "${SCRIPT_DIR}/lib/crash-controls.sh"
sc_init || exit 1

readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
readonly DB_CONTAINER="${DB_CONTAINER:-coprocessor-and-kms-db}"
readonly TEST_NETWORK="${TEST_NETWORK:-staging}"
readonly HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"
readonly REPORT_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/failure-matrix"

COLUMN=all
ONLY=""
LIST_ONLY=0
KEEP_GOING=0
VICTIM=1
while [[ $# -gt 0 ]]; do
  case "$1" in
    --column) COLUMN="${2:?--column needs a value}"; shift 2 ;;
    --case|--only) ONLY="${2:?--case needs a cell id}"; shift 2 ;;
    --list) LIST_ONLY=1; shift ;;
    --keep-going) KEEP_GOING=1; shift ;;
    --victim) VICTIM="${2:?--victim needs an operator index}"; shift 2 ;;
    *) echo "usage: run-failure-matrix.sh [--column crash|stall|data|db|smoke|all] [--case <id>] [--list] [--keep-going] [--victim <index>]" >&2
       exit 2 ;;
  esac
done
case "$COLUMN" in crash|stall|data|db|smoke|all) ;; *) echo "unknown column $COLUMN" >&2; exit 2 ;; esac

# Every exit path restores what was faulted. The object-storage cell proved why:
# the run aborted between stopping the object store and healing it, and it stayed down
# -- so every later cell, and the next suite, would have been measuring a stack
# this runner broke. `sc_run_restores` is idempotent and silent when there is
# nothing registered.
die() { echo "failure-matrix: $*" >&2; exit 1; }
log() { printf '\n=== %s\n' "$*"; }
SC_CASE_BASELINE=""
FM_PHASE_REGISTRY=""
FM_ARM_PID=""
FM_ARM_OUTPUT=""
FM_ACTIVE_CASE=""
FM_HOST_TOKEN=""
FM_CELL_PID=""
FM_ABORT_REMAINING=0
declare -A FM_CASE_DEADLINES=()
cancel_host_case() {
  [[ -n "$FM_HOST_TOKEN" ]] || return 0
  if ! hc_run timeout --kill-after=2s 20s node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" cancel "$FM_HOST_TOKEN"; then
    mkdir -p "$REPORT_DIR"
    printf 'host_token=%s\nphase_registry=%s\nrestore_log=%s\nbaseline=%s\n' \
      "$FM_HOST_TOKEN" "$FM_PHASE_REGISTRY" "$SC_RESTORE_LOG" "$SC_CASE_BASELINE" > "$REPORT_DIR/uncancelled-phase"
    echo "failure-matrix: host case shutdown unverified; recovery ownership retained" >&2
    return 1
  fi
  if [[ -n "$FM_CELL_PID" ]]; then wait "$FM_CELL_PID" 2>/dev/null || true; FM_CELL_PID=""; fi
}
cancel_phases() {
  sp_cancel_all || return 1
  if [[ -n "$FM_ARM_PID" ]]; then wait "$FM_ARM_PID" 2>/dev/null || true; FM_ARM_PID=""; fi
}
retain_case_recovery() {
  mkdir -p "$REPORT_DIR"
  printf 'phase_registry=%s\nrestore_log=%s\nbaseline=%s\ncase=%s\n' \
    "$FM_PHASE_REGISTRY" "$SC_RESTORE_LOG" "$SC_CASE_BASELINE" "$FM_ACTIVE_CASE" > "$REPORT_DIR/uncancelled-phase"
  FM_ABORT_REMAINING=1
}
restore_case() {
  hc_begin_cleanup || return 1
  cancel_host_case || return 1
  restore_case_resources
}
restore_case_resources() {
  local status=0
  hc_begin_cleanup || return 1
  # The child can finish its container phases without signalling the host
  # supervisor which owns that child. The parent additionally stops the whole
  # host group before retrying recovery after an abort.
  cancel_phases || return 1
  sp_recover_suite_state || { retain_case_recovery; return 1; }
  if [[ "${SP_FORCED_STOP:-0}" != 0 ]]; then
    FM_ABORT_REMAINING=1
    touch "$SP_RUNTIME_DIR/cleanup-failed"
  fi
  local crash_database=coprocessor
  [[ "$VICTIM" == 0 ]] || crash_database="coprocessor_$VICTIM"
  if [[ "$FM_ACTIVE_CASE" == FM-TFHE-CRASH ]]; then
    cc_disable_failpoints "$DB_CONTAINER" "$crash_database" || { retain_case_recovery; return 1; }
  fi
  sc_run_restores || status=1
  if [[ -n "$SC_CASE_BASELINE" && -f "$SC_CASE_BASELINE" ]]; then
    sc_restore_running "$SC_CASE_BASELINE" || status=1
  fi
  # Commit control removal before resuming a paused worker. Drop audit DDL only
  # afterwards, once that worker can release any transaction holding its table.
  if [[ "$FM_ACTIVE_CASE" == FM-TFHE-CRASH && "$status" == 0 ]]; then
    cc_drop_audit "$DB_CONTAINER" "$crash_database" || status=1
  fi
  if [[ "$FM_ACTIVE_CASE" == FM-ZKPROOF-CRASH ]]; then
    local database=coprocessor
    [[ "$VICTIM" == 0 ]] || database="coprocessor_$VICTIM"
    hc_run timeout --kill-after=2s 20s docker exec -e PGCONNECT_TIMEOUT=5 "$DB_CONTAINER" psql -U postgres -d "$database" -v ON_ERROR_STOP=1 -c \
      'SET lock_timeout=5000; SET statement_timeout=10000;
       DO $$ BEGIN
         IF to_regclass('\''public.consensus_test_proof_controls'\'') IS NOT NULL THEN
           DELETE FROM verify_proofs WHERE zk_proof_id IN (SELECT zk_proof_id FROM public.consensus_test_proof_controls);
         END IF;
       END $$;
       DROP TABLE IF EXISTS public.consensus_test_proof_controls;
       DROP TRIGGER IF EXISTS consensus_test_proof_outcome ON verify_proofs;
       DROP FUNCTION IF EXISTS public.consensus_test_proof_outcome();
       DROP TABLE IF EXISTS public.consensus_test_proof_outcomes;' >/dev/null || status=1
  fi
  if [[ "${SP_FORCED_STOP:-0}" == 1 ]]; then ensure_test_container_resolves 1 || status=1; fi
  if [[ "$status" != 0 ]]; then
    [[ -z "${SP_RUNTIME_DIR:-}" ]] || touch "$SP_RUNTIME_DIR/cleanup-failed"
    retain_case_recovery
    return 1
  fi
  [[ "${SP_FORCED_STOP:-0}" == 0 ]]
}
dispose_case() {
  # Parent-visible ownership is removed only after complete verified recovery.
  sp_dispose || return 1
  rm -f "${SC_CASE_BASELINE:-}" "${FM_ARM_OUTPUT:-}"
  SC_CASE_BASELINE=""; FM_ARM_OUTPUT=""; FM_PHASE_REGISTRY=""
  FM_HOST_TOKEN=""; FM_ACTIVE_CASE=""
  unset SP_RUNTIME_DIR SP_PHASE_REGISTRY SP_FORCED_STOP
}
finish_runner() {
  local status="$1"
  hc_cleanup_signals
  if restore_case; then
    [[ -z "$FM_PHASE_REGISTRY" ]] || dispose_case || status=1
  else status=1; fi
  exit "$status"
}
trap 'finish_runner $?' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# --------------------------------------------------------------------------
# The matrix.
#
#   inventory-case | column | service template | fault | workload | notes
#
# `%d` in a service template is the victim operator's index prefix, rendered by
# `service_for`. A cell whose service is absent from the running topology is
# NOT_APPLICABLE, never PASS: several bundles ship without a consensus-detector
# or an upgrade-controller at all.
# --------------------------------------------------------------------------
MATRIX=(
  "FM-TFHE-CRASH|crash|%d-tfhe-worker|kill|compute-chain"
  "FM-TFHE-STALL|stall|%d-tfhe-worker|pause|compute-chain"
  "FM-SNS-CRASH|crash|%d-sns-worker|kill|sns-noisy"
  "FM-SNS-STALL|stall|%d-sns-worker|pause|sns-noisy"
  "FM-ZKPROOF-CRASH|crash|%d-zkproof-worker|kill|zkproof-input"
  "FM-HOST-LISTENER-CRASH|crash|%d-host-listener-poller|kill|ingestion"
  "FM-TX-SENDER-CRASH|crash|%d-transaction-sender|kill|submission"
  "FM-CONSENSUS-DETECTOR|crash|%d-transaction-sender|pause|detector-drift"
  "FM-OBJECT-STORAGE-OUTAGE|data|fhevm-object-store|stop|storage"
  "FM-HOST-LONG-OFFLINE|data|%d-host-listener-poller|stop|ingestion-backlog"
  "FM-DURABLE-BACKLOG|crash|%d-tfhe-worker|kill|compute-backlog"
  "FM-STORAGE-WORKER-RESTART|data|fhevm-object-store|stop|storage"
  "FM-BROKER-OUTAGE|data|listener-redis|stop|ingestion"
  "FM-RELAYER-CRASH|data|fhevm-relayer|kill|relayer-request"
  "FM-KMS-CONNECTOR-CRASH|data|kms-connector-kms-worker|kill|kms-decryption"
  "FMDB-OUTAGE|db|coprocessor-and-kms-db|stop|compute-chain"
  "FMDB-STALL|db|coprocessor-and-kms-db|pause|compute-chain"

  # Cells with no stronger contract than "the service came back and the fleet
  # still computes". They are labelled smoke in the inventory and they do not
  # satisfy any in-flight recovery claim; keeping them is still worth it,
  # because these service roles have no other supervisor-restart check. Pause
  # cells that merely mint fresh work after resume duplicate stronger cases.
  "FM-SMOKE-CELLS|smoke|%d-host-listener|kill|smoke"
  "FM-SMOKE-CELLS|smoke|%d-host-listener-consumer|kill|smoke"
  "FM-SMOKE-CELLS|smoke|%d-gw-listener|kill|smoke"
  "FM-SMOKE-CELLS|smoke|%d-consensus-detector|kill|smoke"
  "FM-SMOKE-CELLS|smoke|%d-upgrade-controller|kill|smoke"
)

# One operator's whole ingestion set, so an ingestion cell can isolate the path
# it is testing: everything is taken down, and only the path under test is
# brought back to deliver the pending work.
ingestion_set() {
  local prefix="$1"
  local name
  for name in host-listener host-listener-poller host-listener-consumer; do
    docker inspect "${prefix}-${name}" >/dev/null 2>&1 && printf '%s\n' "${prefix}-${name}"
  done
}

operator_prefix() {
  [[ "$1" == 0 ]] && echo "coprocessor" || echo "coprocessor$1"
}

service_for() {
  local template="$1"
  if [[ "$template" == %d-* ]]; then
    printf '%s%s' "$(operator_prefix "$VICTIM")" "${template#%d}"
  else
    printf '%s' "$template"
  fi
}

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

env_value() { sed -n "s/^$1=//p" "$2" | tail -1; }

# Can the test container resolve service names?
#
# Service churn can leave the harness resolver unable to resolve names even
# while Docker exec remains responsive. Bound the lookup itself and recreate
# the harness when resolution does not recover; restarting alone can retain
# the broken resolver state.
#
# So the repair is a recreate, and the handshake survives it: the arm phase's
# record lives in the container's filesystem, so it is copied out before and
# back in after. Without that, healing the container would silently discard the
# armed workload and the verify phase would report on nothing.
# Can the container resolve ANY running service? `db` alone is the wrong
# question: it is the alias of `coprocessor-and-kms-db`, so the database cells
# stop the very name the probe asks about, and a correct "not found" was read as
# a broken resolver and cost those cells their run.
container_resolves_something() {
  local name
  for name in db gateway-node host-node coprocessor-and-kms-db; do
    # Bound the remote lookup too: killing Docker alone leaves getent running.
    hc_run timeout --kill-after=1s 4s docker exec "$TEST_CONTAINER" timeout -k 1 2 getent hosts "$name" >/dev/null 2>&1 && return 0
  done
  return 1
}

ensure_test_container_resolves() {
  local i force="${1:-0}"
  if [[ "$force" != 1 ]]; then
    for i in $(seq 1 3); do
      container_resolves_something && return 0
      sleep 1
    done
  fi
  echo "  (test container cannot resolve service names; recreating it)" >&2

  # Its own compose files, read from the container rather than hardcoded here.
  local config_files project service
  config_files="$(docker inspect -f '{{index .Config.Labels "com.docker.compose.project.config_files"}}' "$TEST_CONTAINER" 2>/dev/null)"
  project="$(docker inspect -f '{{index .Config.Labels "com.docker.compose.project"}}' "$TEST_CONTAINER" 2>/dev/null)"
  service="$(docker inspect -f '{{index .Config.Labels "com.docker.compose.service"}}' "$TEST_CONTAINER" 2>/dev/null)"
  if [[ -z "$config_files" || -z "$project" || -z "$service" ]]; then
    echo "  (cannot identify the test container's compose project; not recreating)" >&2
    return 1
  fi

  sp_snapshot_recreate "$TEST_CONTAINER" "$HANDSHAKE_DIR" || return 1

  local -a compose_args=(compose -p "$project")
  local file
  while IFS= read -r file; do
    [[ -n "$file" ]] && compose_args+=(-f "$file")
  done < <(tr ',' '\n' <<<"$config_files")
  docker "${compose_args[@]}" up -d --force-recreate "$service" >/dev/null 2>&1 || return 1
  sp_restore_recreated "$TEST_CONTAINER" || return 1

  for i in $(seq 1 24); do
    if container_resolves_something; then
      suite_identity_assert "$TEST_CONTAINER" || return 1
      echo "  (test container recreated; name resolution restored and the handshake carried across)" >&2
      return 0
    fi
    sleep 5
  done
  return 1
}

# Run one phase of the in-container failure case. The verdict is the process
# exit status plus the phase's own completion marker; `1 passing` in mocha output
# survives an after-hook failure and is not a verdict.
# The nameref parameter is deliberately named `__phase_out`: bash refuses a
# circular name reference, so a caller whose own variable happened to be called
# `phase_out` would break this at runtime rather than at review time.
run_phase() {
  local -n __phase_out="$1"
  local case_id="$2" workload="$3" phase="$4"
  ensure_test_container_resolves || die "the test container cannot resolve service names even after a restart"
  SP_PHASE_TIMEOUT_SECONDS="${PHASE_TIMEOUT_S:-900}"
  local coprocessor_env="$ENV_DIR/coprocessor.env"
  # Bounded. Mocha's own timeout for these suites is forty minutes, which is a
  # reasonable ceiling for a suite doing real work and a terrible one for a
  # phase whose dependencies have been taken away: FMDB-STALL sat blocked on a
  # paused database for twenty minutes before anyone looked. Fifteen minutes is
  # far above what a healthy phase takes (one to three) and far below the point
  # where a run stops being worth waiting for.
  cr_run_suite __phase_out "[failure-case/${case_id}/${phase}] CASE COMPLETE" \
    sp_exec \
      -e RUN_FAILURE_CASE=1 \
      -e "FAILURE_CASE_ID=$case_id" \
      -e "FAILURE_WORKLOAD=$workload" \
      -e "FAILURE_PHASE=$phase" \
      -e "FAILURE_VICTIM_OPERATOR=$VICTIM" \
      -e "COPROCESSOR_COUNT=$(operator_count)" \
      -e "CONSENSUS_THRESHOLD=$CONSENSUS_THRESHOLD" \
      -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" \
      -e "CONSENSUS_WATCHDOG_DISABLED=$([[ "$workload" == detector-drift ]] && echo 1 || echo 0)" -e CONSENSUS_WATCHDOG_STALL_MS=2400000 \
      -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL "$coprocessor_env")" \
      -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS "$coprocessor_env")" \
      -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")" \
      -e "TFHE_WORKER_METRICS_URLS=$(gpu_worker_metrics_urls "$(operator_count)" "$TEST_CONTAINER" coprocessor-and-kms-db)" \
      -e npm_config_update_notifier=false \
      "$TEST_CONTAINER" \
      npx hardhat test test/consensus/failureCase.ts --network "$TEST_NETWORK"
}

# The suite must read host-observed fault evidence, not infer a fault from a
# transient pending row that an unfaulted service would also produce.
worker_database_ip() {
  local expected_state="${2:?expected verifier state is required}"
  hc_run timeout --kill-after=1s 10s docker inspect "$1" "$DB_CONTAINER" | bun -e '
    const {isIP}=require("node:net");
    const [worker,db]=JSON.parse(await Bun.stdin.text());
    const expected=process.argv[1];
    if(!["paused","running"].includes(expected)) throw Error("invalid expected verifier state");
    if(worker.State.Status!==expected || worker.State.Running!==true ||
       worker.State.Paused!==(expected==="paused") || worker.State.Restarting!==false ||
       !(worker.State.Pid>0)) throw Error(`verifier is not ${expected}`);
    const networks=new Set(Object.values(db.NetworkSettings.Networks).map(n=>n.NetworkID));
    const ips=Object.values(worker.NetworkSettings.Networks).filter(n=>networks.has(n.NetworkID)).map(n=>n.IPAddress);
    if(ips.length!==1 || !isIP(ips[0])) throw Error("cannot identify verifier database-client IP");
    console.log(ips[0]);
  ' "$expected_state"
}

publish_fault_ack() {
  local case_id="$1" service="$2" observed="$3" before="$4" after="${5:-}" recovered="${6:-}"
  local recovered_worker_address="" expected_worker_state=paused
  if [[ "$case_id" == FM-ZKPROOF-CRASH ]]; then
    # The first acknowledgement precedes SDK arming with the verifier held.
    # Recovery must instead prove the replacement is running and unpaused.
    [[ -z "$recovered" ]] || expected_worker_state=running
    recovered_worker_address="$(worker_database_ip "$service" "$expected_worker_state")" || return 1
  fi
  hc_run timeout --kill-after=1s 10s docker exec "$TEST_CONTAINER" node -e '
    const fs = require("fs"), path = require("path");
    const [dir, caseId, service, faultObservedAt, processBefore, processAfter, recoveryObservedAt, recoveredWorkerAddress] = process.argv.slice(1);
    const file = path.join(dir, "failure-fault.json");
    fs.mkdirSync(dir, {recursive:true});
    fs.writeFileSync(file + ".partial", JSON.stringify({name:"failure-fault", ready:true, payload:{
      applied:true, caseId, service, faultObservedAt, processBefore, processAfter, recoveryObservedAt, recoveredWorkerAddress,
      detail:"host verified the service held throughout workload arming, then verified recovery"
    }}));
    fs.renameSync(file + ".partial", file);
  ' "$HANDSHAKE_DIR" "$case_id" "$service" "$observed" "$before" "$after" "$recovered" "$recovered_worker_address"
}

# The container whose network namespace the test container shares, if any.
#
# `test-suite-e2e-debug` is composed with `network_mode: container:fhevm-object-store`
# so that `http://localhost:9000` key URLs resolve inside it. The consequence is
# structural: stopping that container removes the test container's network
# entirely -- DNS included -- and it cannot even be recreated until the owner is
# back. A cell that faults the namespace owner therefore cannot use the
# in-container suite while the fault is applied, and must arm before it.
netns_owner() {
  local mode; mode="$(docker inspect -f '{{.HostConfig.NetworkMode}}' "$TEST_CONTAINER" 2>/dev/null)"
  [[ "$mode" == container:* ]] || return 0
  docker inspect -f '{{.Name}}' "${mode#container:}" 2>/dev/null | sed 's|^/||'
}

# Does the ARM phase depend on the service this cell faults?
#
# Two services it cannot do without: the one whose network namespace the test
# container shares, and the database -- arming means minting a workload and
# reading its evidence, and both go through Postgres. A cell that faults either
# must arm first, or it is asking the suite to use the thing the fault removed.
arm_depends_on() {
  local service="$1"
  [[ "$service" == "$(netns_owner)" ]] && return 0
  [[ "$service" == "$DB_CONTAINER" ]] && return 0
  return 1
}

# Which stage to hold back across an early arm, so the work is still
# outstanding when the service goes away: the squash for an object-storage
# outage, the computation itself for a database one.
hold_back_kind() {
  [[ "$1" == "$DB_CONTAINER" ]] && echo tfhe || echo sns
}

# Did the target's own log react to the fault? Used by the data cells, where
# "the container is stopped" is not the same as "a request actually failed".
# Polled, not sampled once. The reaction a data cell looks for is a dependent
# service FAILING against the thing that was taken away, and that takes as long
# as the service's own retry cadence -- for the squash workers, longer than the
# moment between thawing them and asking. A single grep read "no service
# reacted" when the answer was "not yet".
observed_failure_reaction() {
  local target="$1" since="$2" pattern="$3" deadline_s="${4:-120}" logs
  local remaining
  remaining="$(case_seconds_left)" || return 124
  (( remaining < deadline_s )) && deadline_s="$remaining"
  local deadline=$((SECONDS + deadline_s))
  while :; do
    logs="$(sc_logs_since "$target" "$since" 2>/dev/null)" || return 2
    sc_logs_show_failure "$logs" "$pattern" && return 0
    ((SECONDS < deadline)) || return 1
    sleep 5
  done
}

verify_detector_receipt() {
  local receipt
  receipt="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/failure-verification.json")" || return 1
  printf '%s' "$receipt" | bun -e '
    const envelope=JSON.parse(await Bun.stdin.text());
    if(envelope.name!=="failure-verification" || envelope.ready!==true || !envelope.payload) process.exit(1);
    const value=envelope.payload;
    const [runId,caseId]=process.argv.slice(1);
    if(value.runId!==runId || value.caseId!==caseId || value.workload!=="detector-drift" ||
       value.driftDetected!==true || value.driftRecovered!==true ||
       !Number.isInteger(value.signalsBefore) || value.signalsBefore<0 ||
       !Number.isInteger(value.signalsAfter) || value.signalsAfter-value.signalsBefore!==1) process.exit(1);
  ' "$CR_RUN_ID" FM-CONSENSUS-DETECTOR
}

# --------------------------------------------------------------------------
# One cell.
# --------------------------------------------------------------------------
run_cell() {
  local case_id="$1" budget name
  FM_ACTIVE_CASE="$case_id"
  budget="$(bun "$CR_INVENTORY_CLI" show "$case_id" | sed -n 's/^timeout: *\([0-9]*\)s$/\1/p')"
  if [[ -z "${FM_CASE_DEADLINES[$case_id]:-}" ]]; then
    case_deadline_start "$budget" || die "missing inventory deadline for $case_id"
    FM_CASE_DEADLINES[$case_id]="$CASE_DEADLINE_EPOCH"
  fi
  unset HC_CLEANUP_DEADLINE_EPOCH
  CASE_DEADLINE_EPOCH="${FM_CASE_DEADLINES[$case_id]}"
  unset SP_RUNTIME_DIR SP_PHASE_REGISTRY
  sp_init || die "cannot establish case process ownership"
  FM_PHASE_REGISTRY="$SP_PHASE_REGISTRY"
  SC_CASE_BASELINE="$SP_RUNTIME_DIR/baseline"
  FM_ARM_OUTPUT="$SP_RUNTIME_DIR/arm-output"
  FM_STAGED_RESULTS="$SP_RUNTIME_DIR/results"
  mkdir -p "$FM_STAGED_RESULTS"
  touch "$SC_CASE_BASELINE" "$FM_ARM_OUTPUT"
  FM_HOST_TOKEN="matrix_${BASHPID}_${RANDOM}_$(date +%s%N)"
  # A separate host process group bounds hung Docker/systemd commands too.
  # Export functions rather than re-reading the script, preserving this exact
  # invocation and enabling isolated tests of the actual orchestration.
  # COLUMN is the invocation's selection; the row's column alone cannot prove
  # that a database case was selected in isolation from ordinary matrix cells.
  while read -r _ _ name; do export -f "${name?}"; done < <(declare -F)
  for name in SCRIPT_DIR REPO_ROOT ENV_DIR TEST_CONTAINER DB_CONTAINER TEST_NETWORK HANDSHAKE_DIR REPORT_DIR VICTIM COLUMN \
    CASE_DEADLINE_EPOCH CONSENSUS_RUN_ID CONSENSUS_SCENARIO CONSENSUS_OPERATORS CONSENSUS_THRESHOLD CONSENSUS_RUN_STARTED_AT \
    "${!SC_@}" "${!CR_@}" "${!GPU_@}" "${!SP_@}" "${!FM_@}"; do
    [[ "$name" == FM_CASE_DEADLINES ]] || export "${name?}"
  done
  CONSENSUS_RESULTS_DIR="$FM_STAGED_RESULTS" node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run "$FM_HOST_TOKEN" "$(( CASE_DEADLINE_EPOCH * 1000 ))" \
    bash -c 'set -uo pipefail; run_timed_cell "$@"' -- "$@" &
  FM_CELL_PID=$!
  local result=0 cleanup=ok expired=0 child_cleanup_failed=0
  wait "$FM_CELL_PID" || result=$?
  FM_CELL_PID=""
  [[ "$result" != 124 ]] && case_seconds_left >/dev/null || expired=1
  if ! restore_case; then cleanup=failed; result=1; FM_ABORT_REMAINING=1; fi
  if [[ -f "$SP_RUNTIME_DIR/cleanup-failed" ]]; then child_cleanup_failed=1; result=1; FM_ABORT_REMAINING=1; fi
  local final_state=keep final_detail="" published
  if [[ "$expired" == 1 ]]; then
    final_state=INVALID; final_detail="case exceeded its ${budget}s inventory deadline"
    result=124
  elif [[ "$cleanup" == failed ]]; then
    final_state=FAIL; final_detail="case recovery failed; parent retains its phase and writer ownership records"
  elif [[ "${SP_FORCED_STOP:-0}" == 1 ]]; then
    final_state=FAIL; final_detail="phase cancellation required stopping the test container; the case cannot establish normal recovery"
    result=1
  elif [[ "$child_cleanup_failed" == 1 ]]; then
    final_state=FAIL; final_detail="child recovery failed; parent restored the stack but the case did not complete normally"
  elif [[ "$result" != 0 ]]; then
    final_state=FAIL; final_detail="case runner exited $result before parent finalization"
  fi
  if [[ "$final_state" == keep ]] && ! bun "$SCRIPT_DIR/finalize-case-results.ts" --require-case "$FM_STAGED_RESULTS" "$case_id" "$CR_RUN_ID"; then
    final_state=FAIL; final_detail="case runner exited successfully without a successful staged verdict for $case_id"
    result=1
  fi
  published="$(bun "$SCRIPT_DIR/finalize-case-results.ts" "$FM_STAGED_RESULTS" \
    "$(dirname "$(cr_results_file)")" "$final_state" "$cleanup" "$final_detail")" || {
    cr_record "$case_id" FAIL cleanup="$cleanup" detail="parent could not finalize the staged case evidence" || true
    touch "$SP_RUNTIME_DIR/cleanup-failed"
    FM_ABORT_REMAINING=1
    return 1
  }
  if [[ "$final_state" != keep ]] && ! grep -Fxq "$CR_RUN_ID:$case_id" <<<"$published"; then
    cr_record "$case_id" "$final_state" cleanup="$cleanup" detail="$final_detail" || result=1
  fi
  if [[ "$cleanup" == ok ]]; then dispose_case || { result=1; FM_ABORT_REMAINING=1; }; fi
  return "$result"
}

run_timed_cell() {
  # These delegated suites inherit the same outer deadline and ownership files.
  # Capture their baseline before handing off: the parent owns the path and can
  # restore it even when the host deadline kills the delegated runner outright.
  case "$1" in
    FM-TFHE-CRASH|FM-RELAYER-CRASH|FM-KMS-CONNECTOR-CRASH)
      sc_snapshot_running >"$SC_CASE_BASELINE" || {
        cr_record "$1" INVALID cleanup=not_required detail="could not snapshot running service owners before delegation"
        return 1
      }
      ;;
  esac
  case "$1" in
    FM-TFHE-CRASH) CONSENSUS_ASSERTION_CASE_ID=FM-TFHE-CRASH "$SCRIPT_DIR/run-crash-retry-consensus.sh" --victim "$VICTIM" --boundary before-commit ;;
    FM-RELAYER-CRASH|FM-KMS-CONNECTOR-CRASH) "$SCRIPT_DIR/run-request-recovery.sh" "$1" ;;
    *) run_cell_body "$@" ;;
  esac
}

run_cell_body() {
  local case_id="$1" column="$2" template="$3" fault="$4" workload="$5"
  local service; service="$(service_for "$template")"

  # A cell whose case the inventory establishes under a different scenario --
  # FM-CONSENSUS-DETECTOR needs a threshold below the operator count for a
  # differing submission to reach consensus at all -- is declined here rather
  # than run and recorded, which would claim an outcome this stack cannot
  # support.
  cr_skip_wrong_scenario "$case_id" && return 0

  local started; started="$(cr_now)"
  local -a record=(started_at="$started")
  local cell_out=""
  if [[ "$case_id" == FM-CONSENSUS-DETECTOR ]]; then
    docker exec "$TEST_CONTAINER" rm -f "$HANDSHAKE_DIR/failure-verification.json" || return 1
  fi

  if ! docker inspect "$service" >/dev/null 2>&1; then
    cr_record "$case_id" NOT_APPLICABLE detail="$service is absent from this topology" cleanup=not_required started_at="$started"
    echo "[$case_id/$service] NOT_APPLICABLE: absent from this topology"
    return 0
  fi
  local state; state="$(sc_state "$service")"
  if [[ "$state" != running ]]; then
    cr_record "$case_id" INVALID detail="$service is $state before any fault was injected" cleanup=not_required started_at="$started"
    echo "[$case_id/$service] INVALID: $service is $state before any fault"
    return 1
  fi

  if [[ "$workload" == *-backlog ]]; then
    local role flag arguments
    for role in tfhe-worker host-listener-poller; do
      flag=--work-items-batch-size=4
      [[ "$role" != host-listener-poller ]] || flag=--batch-size=4
      arguments="$(docker inspect --format '{{range .Config.Cmd}}{{println .}}{{end}}' "$(operator_prefix "$VICTIM")-$role")" || return 1
      grep -Fxq -- "$flag" <<<"$arguments" || {
        cr_record "$case_id" INVALID cleanup=not_required started_at="$started" detail="backlog case requires observed $role $flag"
        return 1
      }
    done
  fi

  log "$case_id — $fault on $service (workload $workload)"
  local identity_before; identity_before="$(sc_identity "$service")"
  record+=(process_before="$service=$identity_before")

  sc_snapshot_running >"$SC_CASE_BASELINE" || {
    cr_record "$case_id" INVALID cleanup=not_required started_at="$started" detail="could not snapshot running service owners"
    return 1
  }
  local auto_restart="not_evaluated" fault_observed=""
  if [[ "$fault" == kill && "$(sc_restart_budget "$service")" == exhausted ]]; then
    sc_reset_restart_budget "$service" || return 1
    identity_before="$(sc_identity "$service")"
  fi

  # ---- 2. take the service away, verified --------------------------------
  #
  # If this cell's target owns the test container's network namespace, the
  # workload has to be armed while the network still exists. The squash workers
  # are frozen across the arm so the work is genuinely outstanding at the stage
  # the fault removes, and thawed once the service is down.
  local armed_early=0
  local -a frozen_squash=()
  if arm_depends_on "$service"; then
    local index prefix squash kind
    kind="$(hold_back_kind "$service")"
    for ((index = 0; index < $(operator_count); index++)); do
      prefix="$(operator_prefix "$index")"
      squash="${prefix}-${kind}-worker"
      if ! docker inspect "$squash" >/dev/null 2>&1 || ! sc_pause "$squash"; then
        local freeze_cleanup=ok
        restore_case_resources || freeze_cleanup=failed
        cr_record "$case_id" INVALID cleanup="$freeze_cleanup" started_at="$started" \
          detail="could not hold $squash before early workload arming; the pending stage was not established"
        return 1
      fi
      frozen_squash+=("$squash")
    done
    docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/failure-workload.json '$HANDSHAKE_DIR'/failure-proof-result.json" >/dev/null 2>&1 || true
    local early_status=0
    run_phase cell_out "$case_id" "$workload" arm || early_status=$?
    if [[ "$early_status" -ne 0 ]]; then
      echo "$cell_out" | tail -20
      for squash in "${frozen_squash[@]}"; do sc_resume "$squash" || true; done
      restore_case_resources || true
      cr_record "$case_id" INVALID cleanup=ok started_at="$started" \
        detail="the workload could not be armed before $service was taken down: $(cr_failure_reason "$cell_out")"
      echo "[$case_id/$service] INVALID: the stage could not be established"
      return 1
    fi
    armed_early=1
  fi

  local -a extra_down=()
  if [[ "$workload" == ingestion* && "$template" == %d-* ]]; then
    # Isolate the ingestion path under test: every path is taken down so the
    # blocks are genuinely pending, and only the path under test is brought
    # back to deliver them. Without this a redundant path ingests the work and
    # the cell measures nothing about the faulted one.
    local other
    while IFS= read -r other; do
      [[ "$other" == "$service" ]] && continue
      extra_down+=("$other")
    done < <(ingestion_set "$(operator_prefix "$VICTIM")")
  fi

  local -a held_senders=()
  if [[ "$case_id" == FM-CONSENSUS-DETECTOR ]]; then
    local sender_index sender
    for ((sender_index=0; sender_index<$(operator_count); sender_index++)); do
      [[ "$sender_index" == "$VICTIM" ]] && continue
      sender="$(operator_prefix "$sender_index")-transaction-sender"
      sc_pause "$sender" || return 1
      held_senders+=("$sender")
    done
  fi
  local down_ok=1
  case "$fault" in
    kill)
      sc_pause "$service" || down_ok=0
      ;;
    stop)
      sc_stop "$service" || down_ok=0
      ;;
    pause)
      sc_pause "$service" || down_ok=0
      ;;
  esac
  if [[ "$down_ok" != 1 ]]; then
    restore_case_resources || true
    cr_record "$case_id" INVALID detail="could not take $service down for the workload stage" cleanup=ok started_at="$started"
    echo "[$case_id/$service] INVALID: the service could not be taken down"
    return 1
  fi
  [[ -z "$fault_observed" ]] && fault_observed="$(cr_now)"
  # The squash can run again now that the object store is gone: its upload is
  # what the fault is aimed at, and it was held back so the work would still be
  # outstanding when the store went away.
  local squash
  for squash in "${frozen_squash[@]}"; do
    sc_resume "$squash" || echo "  WARNING could not thaw $squash after the fault landed" >&2
  done
  local target
  for target in "${extra_down[@]}"; do
    sc_stop "$target" || {
      restore_case_resources || true
      cr_record "$case_id" INVALID detail="could not isolate the ingestion path: $target stayed up" cleanup=ok started_at="$started"
      return 1
    }
  done

  publish_fault_ack "$case_id" "$service" "$fault_observed" "$identity_before" || return 1

  # ---- 3. arm the workload on the faulted service ------------------------
  #
  # Normally the workload is armed WITH the service down, so the fault lands on
  # work the service has not finished. That is impossible when the faulted
  # service owns the test container's network namespace: the arm phase would
  # have no network to run over. Those cells arm first (above, before the
  # service is taken down) and the squash is frozen meanwhile, so the work is
  # still pending at the faulted stage when the service goes away -- constructed
  # rather than raced.
  local arm_status=0
  if [[ "$armed_early" == 1 ]]; then
    echo "  (armed before the fault: the arm phase cannot run without $service)"
  else
    docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/failure-workload.json '$HANDSHAKE_DIR'/failure-proof-result.json" >/dev/null 2>&1 || true
    if [[ "$case_id" == FM-ZKPROOF-CRASH ]]; then
      (
        output=""; phase_status=0
        run_phase output "$case_id" "$workload" arm || phase_status=$?
        printf '%s\n' "$output" > "$FM_ARM_OUTPUT"
        exit "$phase_status"
      ) &
      FM_ARM_PID=$!
      # The original SDK request stays alive while the host kills and heals its
      # worker. The pending-proof handshake is the arm observation boundary.
      while ! docker exec "$TEST_CONTAINER" test -f "$HANDSHAKE_DIR/failure-workload.json"; do
        kill -0 "$FM_ARM_PID" 2>/dev/null || { arm_status=1; break; }
        case_seconds_left >/dev/null || { arm_status=124; break; }
        sleep 1
      done
      [[ "$arm_status" == 0 ]] || cell_out="$(cat "$FM_ARM_OUTPUT")"
    else
      run_phase cell_out "$case_id" "$workload" arm || arm_status=$?
    fi
  fi
  if [[ "$arm_status" -ne 0 ]]; then
    echo "$cell_out" | tail -20
    restore_case_resources || true
    cr_record "$case_id" INVALID \
      detail="the workload could not be armed at the stage this cell faults: $(cr_failure_reason "$cell_out")" \
      cleanup=ok started_at="$started" fault_observed_at="$fault_observed"
    echo "[$case_id/$service] INVALID: the stage could not be established"
    return 1
  fi
  local held_state; held_state="$(sc_state "$service")" || return 1
  if [[ ( "$fault" == stop && "$held_state" != stopped ) || ( "$fault" != stop && "$held_state" != paused ) ]]; then
    cr_record "$case_id" INVALID started_at="$started" cleanup=ok detail="$service did not remain faulted throughout arming (state=$held_state)"
    return 1
  fi

  # What the armed workload named, as handles or as ids of its own kind. A
  # case whose work has no handle -- the proof case arms by leaving specific
  # `verify_proofs` rows unverified -- names it in `identifiers` instead, and a
  # runner that only scraped 64-hex found nothing and refused the cell.
  local workload_json workload_ids
  workload_json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/failure-workload.json" 2>/dev/null)"
  # Flattened first: the handshake is pretty-printed, so `"identifiers": [` and
  # its values sit on different lines and a line-based match finds nothing --
  # which is how a case that had named its work precisely was reported as
  # having published none.
  workload_ids="$( { grep -oE '0x[0-9a-f]{64}' <<<"$workload_json"
                     tr -d '\n' <<<"$workload_json" \
                       | sed -n 's/.*"identifiers"[^]]*\[\([^]]*\)\].*/\1/p' \
                       | grep -oE '"[^"]+"' | tr -d '"'
                   } | sort -u | tr '\n' ' ')"

  # A stall cell's own claim: the work does not advance while the worker is
  # stopped. Checked from outside, on the state the arm phase left behind.
  local no_progress="not_evaluated"
  if [[ "$fault" == pause ]]; then
    if [[ "$(sc_state "$service")" == paused ]]; then
      no_progress="pass"
    else
      no_progress="fail"
    fi
  fi

  # Did the fault actually reach something? For data cells the postcondition is
  # a failure in a dependent service's own log, not just a stopped container.
  local reaction="not_evaluated"
  case "$case_id" in
    FM-OBJECT-STORAGE-OUTAGE|FM-STORAGE-WORKER-RESTART)
      # The shipped client retries up to 100 times with a 10s backoff cap;
      # it does not log an operation failure for each internal SDK retry.
      # A 120s observation raced that policy and healed storage before the
      # operation could fail. Keep the fault in place for its retry envelope,
      # sharing the case's remaining 30-minute budget with arm and verify.
      if observed_failure_reaction "$(operator_prefix "$VICTIM")-sns-worker" "$started" \
        "s3|upload|connection refused|dispatch failure|timed out" 1200; then
        reaction="pass"
      else
        reaction="fail"
      fi
      ;;
    FMDB-OUTAGE|FMDB-STALL)
      if observed_failure_reaction "$(operator_prefix "$VICTIM")-tfhe-worker" "$started" \
        "database|connection|pool|postgres"; then
        reaction="pass"
      else
        reaction="fail"
      fi
      ;;
    FM-CONSENSUS-DETECTOR)
      # What the detector itself was doing. A case that reports "the detector
      # did not react" has to show the detector's own account, or the next
      # person cannot tell a defect from a divergence that arrived after
      # consensus had already formed -- which is exactly where this stopped
      # once, with the logs gone in the teardown.
      local det_index det_prefix det
      for ((det_index = 0; det_index < $(operator_count); det_index++)); do
        det_prefix="$(operator_prefix "$det_index")"
        det="${det_prefix}-consensus-detector"
        docker inspect "$det" >/dev/null 2>&1 || continue
        echo "  --- $det since $started ---"
        sc_logs_since "$det" "$started" 2>/dev/null | grep -viE 'heartbeat' | tail -12 | sed 's/^/    /'
      done
      reaction="not_evaluated"
      ;;
    FM-BROKER-OUTAGE)
      # The broker cell's honest claim is about REDUNDANCY, not attribution: the
      # poller path can ingest the same blocks, so an outage of the broker does
      # not by itself stall the work. What must be shown is that the fault
      # reached the path that uses the broker -- the consumer's own log -- and
      # that the identified work was delivered anyway.
      # The vocabulary is the consumer's own, read from its logs during a real
      # outage rather than guessed: it says `force_reconnect`, `ClaimSweeper`,
      # `stream read failed` and `Reconnecting failed`, and never the word
      # "redis" except in the line announcing that it reconnected. Asking for
      # words a service does not use is how a case reports no reaction from a
      # service that reacted within a second.
      if observed_failure_reaction "$(operator_prefix "$VICTIM")-host-listener-consumer" "$started" \
        "force_reconnect|reconnecting failed|stream read failed|claim failed|connectionmanager|broker|connection refused"; then
        reaction="pass"
      else
        reaction="fail"
      fi
      ;;
  esac

  if [[ "$case_id" == FM-STORAGE-WORKER-RESTART ]]; then
    # A real upload failure was observed above. Restart its owner while the
    # object store is STILL unavailable, then recover the dependency normally.
    [[ "$reaction" == pass && "$(sc_state "$service")" == stopped ]] || return 1
    local uploader uploader_before
    uploader="$(operator_prefix "$VICTIM")-sns-worker"
    if [[ "$(sc_restart_budget "$uploader")" == exhausted ]]; then sc_reset_restart_budget "$uploader" || return 1; fi
    uploader_before="$(sc_kill "$uploader")" || return 1
    sc_wait_replaced "$uploader" "$uploader_before" 120 || return 1
    [[ "$(sc_state "$service")" == stopped ]] || return 1
    record+=("assert=combined-fault=pass:upload failure, replacement SNS process, and storage still unavailable" "process_before=$uploader=$uploader_before" "process_after=$uploader=$(sc_identity "$uploader")")
  fi

  # Fence possible on-chain poison before releasing its sender. Once release
  # may have happened, a local digest restore cannot prove delayed revert is
  # complete, so abort cleanup retains the journal and blocks further cases.
  if [[ "$case_id" == FM-CONSENSUS-DETECTOR ]]; then
    if ! run_phase cell_out "$case_id" "$workload" release; then
      echo "$cell_out"
      return 1
    fi
  fi

  # Kill the paused process only after the selected work is proven pending.
  local recovered=""
  if [[ "$fault" == kill ]]; then
    local killed_identity policy replaced
    killed_identity="$(sc_kill "$service" KILL 1)" || {
      restore_case_resources || true
      cr_record "$case_id" INVALID cleanup=ok started_at="$started" detail="could not kill the process holding pending work"
      return 1
    }
    fault_observed="$(cr_now)"
    sc_clear_restore "$service" resume || return 1
    if [[ "$(sc_kind "$service")" == unit ]]; then
      policy=on-failure
    else
      policy="$(docker inspect -f '{{.HostConfig.RestartPolicy.Name}}' "$service")" || return 1
    fi
    if [[ "$policy" == no ]]; then
      sc_wait_state "$service" stopped 30 && sc_start "$service" || return 1
      auto_restart=not_evaluated
    else
      replaced="$(sc_wait_replaced "$service" "$killed_identity" 120)" || {
        restore_case_resources || true
        cr_record "$case_id" FAIL cleanup=ok started_at="$started" fault_observed_at="$fault_observed" \
          detail="supervisor did not replace the interrupted process"
        return 1
      }
      auto_restart=pass
    fi
  else
    case "$fault" in
      stop) sc_start "$service" || return 1 ;;
      pause) sc_resume "$service" || return 1 ;;
    esac
  fi
  if [[ "$case_id" == FM-CONSENSUS-DETECTOR ]]; then
    if ! run_phase cell_out "$case_id" "$workload" submitted; then
      echo "$cell_out"; restore_case_resources || true
      cr_record "$case_id" INVALID cleanup=ok started_at="$started" detail="poisoned submission was not observed before honest senders resumed"
      return 1
    fi
    for sender in "${held_senders[@]}"; do sc_resume "$sender" || return 1; done
  fi
  recovered="$(cr_now)"
  local automatic_operators=0
  [[ "$case_id" == FMDB-* ]] && automatic_operators=1
  sc_restore_running "$SC_CASE_BASELINE" 1 "$automatic_operators" || {
    cr_record "$case_id" FAIL cleanup=failed started_at="$started" detail="a previously running service could not be restored"
    return 1
  }

  # The test container's network came back with its owner, but the container is
  # still attached to the namespace that went away, so it has to be recreated
  # before anything in it can run. Compose says as much where the namespace is
  # declared; `ensure_test_container_resolves` carries the armed record across.
  if [[ "$service" == "$(netns_owner)" ]] && ! ensure_test_container_resolves 1; then
    restore_case_resources || true
    cr_record "$case_id" INVALID cleanup=ok started_at="$started" \
      fault_observed_at="$fault_observed" recovery_observed_at="$recovered" \
      detail="$service came back but the test container could not be restored to a working network, so the armed work could not be verified"
    echo "[$case_id/$service] INVALID: the test container could not be restored after the outage"
    return 1
  fi

  publish_fault_ack "$case_id" "$service" "$fault_observed" "$identity_before" "$(sc_identity "$service")" "$recovered" || return 1

  # ---- 5. require the armed work to have recovered -----------------------
  local verify_status=0
  if [[ -n "$FM_ARM_PID" ]]; then
    wait "$FM_ARM_PID" || verify_status=$?
    FM_ARM_PID=""
    cell_out="$(cat "$FM_ARM_OUTPUT")"
    rm -f "$FM_ARM_OUTPUT"
  fi
  if [[ "$verify_status" == 0 ]]; then
    run_phase cell_out "$case_id" "$workload" verify || verify_status=$?
  fi
  if [[ "$verify_status" == 0 && "$case_id" == FM-CONSENSUS-DETECTOR ]] && ! verify_detector_receipt; then
    verify_status=1
    cell_out="detector verification did not publish this run's exact one-signal recovery evidence"
  fi
  local verify_reason=""
  [[ "$verify_status" -ne 0 ]] && verify_reason="$(cr_failure_reason "$cell_out")"
  [[ "$verify_status" -ne 0 ]] && echo "$cell_out" | tail -25

  # The redundant paths an ingestion cell isolated come back now, so the next
  # cell does not inherit a one-legged operator.
  local cleanup_state=ok cleanup_detail=""
  if ! restore_case_resources; then
    cleanup_state=failed
    cleanup_detail="a service could not be restored"
  fi
  local down; down="$(sc_fleet_down "")"
  if [[ -n "$down" ]]; then
    cleanup_state=failed
    cleanup_detail="${cleanup_detail:+$cleanup_detail; }still down: $down"
  fi
  if ! "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$(operator_count)" >/dev/null 2>&1; then
    cleanup_state=failed
    cleanup_detail="${cleanup_detail:+$cleanup_detail; }a queue is not served by exactly one worker"
  fi

  record+=(
    fault_observed_at="$fault_observed"
    recovery_observed_at="$recovered"
    process_after="$service=$(sc_identity "$service")"
    cleanup="$cleanup_state"
    assert="fault=pass:$fault on $service, target state verified"
  )
  [[ -n "$cleanup_detail" ]] && record+=(cleanup_detail="$cleanup_detail")
  [[ "$auto_restart" != not_evaluated ]] && record+=("assert=automatic-restart=$auto_restart")
  [[ "$no_progress" != not_evaluated ]] && record+=("assert=stalled-target-verified=$no_progress")
  [[ "$reaction" != not_evaluated ]] && record+=("assert=failure-reaction-observed=$reaction")
  local id
  for id in $workload_ids; do record+=(workload="$id"); done

  local state_out=PASS detail=""
  if ! case_seconds_left >/dev/null; then
    state_out=INVALID
    detail="the case exceeded its inventory deadline"
  elif [[ "$verify_status" -ne 0 ]]; then
    state_out=FAIL
    detail="the armed workload did not recover: $verify_reason"
  elif [[ "$auto_restart" == fail ]]; then
    state_out=FAIL
    detail="$service did not restart automatically; this run cannot report automatic recovery"
  elif [[ "$no_progress" == fail ]]; then
    state_out=FAIL
    detail="$service was not verifiably stopped while its work was pending"
  elif [[ "$reaction" == fail ]]; then
    state_out=FAIL
    detail="no dependent service recorded a failure while $service was down, so the fault may not have reached the path under test"
  elif [[ "$cleanup_state" != ok ]]; then
    state_out=FAIL
    detail="$cleanup_detail"
  elif [[ -z "$workload_ids" && "$workload" != smoke ]]; then
    state_out=INVALID
    detail="the armed workload published no identifiers, so recovery of specific work cannot be claimed"
  fi

  record+=("assert=recovered-armed-workload=$([[ "$state_out" == PASS ]] && echo pass || echo fail)")
  if [[ "$state_out" == PASS ]]; then
    if [[ "$case_id" == FM-TFHE-STALL && "$no_progress" == pass ]]; then
      record+=("assert=scope=pass:the worker was held before minting; already-acquired lease reclaim is measured separately by CR-03")
    fi
    if [[ "$case_id" == FMDB-OUTAGE && "$COLUMN" == db ]]; then
      record+=("assert=isolation=pass:the explicit database-only runner excludes ordinary cells and gates subsequent work on verified recovery")
    fi
    cr_record_checked_pass "$case_id" "${record[@]}" || return 1
  else
    cr_record "$case_id" "$state_out" "${record[@]}" detail="$detail" || return 1
  fi
  echo "[$case_id/$service] $state_out ${detail:+- $detail}"
  [[ "$state_out" == PASS ]]
}

# Establish a selected target's own precondition before fleet-wide validity.
# A deliberately stopped sensitivity victim must be recorded as that case's
# INVALID outcome, not disappear behind the expected exclusivity failure.
precheck_selected_target() {
  [[ -n "$ONLY" ]] || return 0
  local row id column template fault workload service state
  for row in "${MATRIX[@]}"; do
    IFS='|' read -r id column template fault workload <<<"$row"
    [[ "$id" == "$ONLY" ]] || continue
    service="$(service_for "$template")" || return 1
    state="$(sc_state "$service")" || state=unreadable
    if [[ "$state" != running ]]; then
      cr_record "$id" INVALID cleanup=not_required \
        detail="$service was not live before the fault (state=$state)" || return 1
      return 1
    fi
    return 0
  done
  return 0
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR; bring one up first"
  [[ ! -f "$REPORT_DIR/uncancelled-phase" ]] ||
    die "a previous matrix run could not stop its test process; recover that stack before clearing $REPORT_DIR/uncancelled-phase"

  if [[ "$LIST_ONLY" == 1 ]]; then
    printf '%-26s %-7s %-34s %-6s %s\n' "CASE" "COLUMN" "SERVICE" "FAULT" "WORKLOAD"
    local row
    for row in "${MATRIX[@]}"; do
      IFS='|' read -r id column template fault workload <<<"$row"
      printf '%-26s %-7s %-34s %-6s %s\n' "$id" "$column" "$(service_for "$template")" "$fault" "$workload"
    done
    exit 0
  fi

  docker inspect "$TEST_CONTAINER" >/dev/null 2>&1 || die "test container $TEST_CONTAINER is not running"
  mkdir -p "$REPORT_DIR"

  local count; count="$(operator_count)"
  local observed_topology
  observed_topology="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" "$count")" || die "cannot verify active topology"
  eval "$observed_topology"
  export CONSENSUS_SCENARIO CONSENSUS_OPERATORS CONSENSUS_THRESHOLD
  cr_init "${CONSENSUS_RUN_ID:-}" || die "cannot identify run metadata"
  # The image bakes the e2e suite in, so a commit since the last build does not
  # reach the container. A result labelled with a revision whose code never ran
  # is worse than no result.
  suite_identity_assert "$TEST_CONTAINER" || die "the test container is not running this working tree's e2e suite"
  export CONSENSUS_RUN_STARTED_AT="$(cr_now)"

  precheck_selected_target || return 1

  # A doubly-served queue makes every cell's verdict meaningless, and it is the
  # one fault the matrix cannot detect by probing: the fleet looks healthy until
  # two workers disagree. Ask before spending twenty minutes.
  if gpu_session_active && ! "$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1; then
    die "a queue is served twice before any fault was injected; the fleet is split across backends (gpu-consensus-workers.sh conflicts names it)"
  fi
  "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$count" ||
    die "the fleet does not have exactly one worker per queue before any fault was injected"

  local failures=0 ran=0 row id column template fault workload
  for row in "${MATRIX[@]}"; do
    IFS='|' read -r id column template fault workload <<<"$row"
    if [[ "$COLUMN" == all ]]; then
      [[ "$column" == db || "$column" == smoke ]] && continue
    else
      [[ "$COLUMN" == "$column" ]] || continue
    fi
    [[ -z "$ONLY" || "$ONLY" == "$id" ]] || continue

    ran=$((ran + 1))
    if ! run_cell "$id" "$column" "$template" "$fault" "$workload"; then
      failures=$((failures + 1))
      if [[ "$FM_ABORT_REMAINING" == 1 ]]; then
        echo "stopping: a phase required container-level cancellation; later cells cannot inherit that cleanup failure" >&2
        break
      elif [[ "$KEEP_GOING" != 1 ]]; then
        echo "stopping at the first failing cell; pass --keep-going to run the whole column" >&2
        break
      fi
    fi
  done

  [[ "$ran" -gt 0 ]] || die "the selection matched no cells; an empty selection is not a passing run"
  log "results"
  echo "run $CR_RUN_ID: $ran cell(s), $failures failing"
  echo "structured results: $(cr_results_file)"
  if [[ "${CR_RECORD_FAILURES:-0}" -gt 0 ]]; then
    echo "${CR_RECORD_FAILURES} result(s) were REFUSED and not written; this run has no record of them" >&2
    return 1
  fi
  [[ "$failures" -eq 0 ]]
}

main
