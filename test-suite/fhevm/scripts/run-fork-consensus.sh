#!/usr/bin/env bash
# The fork byte-consensus cases, sequenced with the service control the e2e
# container does not have.
#
# F1 to F3 need no service control and run in one phase. The other two do, and
# for the same reason in both cases: the state they reason about cannot be
# observed on a running service.
#
#   F4  A cross-block child is GATED only until its producer's chain retires,
#       which a running worker does within a polling interval. Stalling the fork
#       operator's tfhe-worker holds the gate open long enough to observe the
#       child by chain id -- which is the difference between this case and the
#       version it replaces, where a database-wide "zero stranded chains" count
#       passed on a stack that had never had a gated child and passed with the
#       repair path disabled.
#
#   F5  The poller reads its cursor from `host_listener_poller_state` at startup
#       and then keeps its position in memory. Rewinding the row under a running
#       poller replays nothing at all, so the unchanged row counts that used to
#       follow were unchanged because nothing had happened. The poller is
#       stopped, the cursor rewound, and the poller restarted so it reads the
#       new value.
#
#   run-fork-consensus.sh [--case main|f4|f5|all] [--fork-operator <index>]
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
# Failed gate removal keeps the affected worker held, but must not abandon
# unrelated services. The original restore ledger remains authoritative.
sp_case_cleanup() {
  local status=0 database="coprocessor_${FORK_OPERATOR}"
  [[ "$FORK_OPERATOR" != 0 ]] || database=coprocessor
  if [[ "${FORK_GATE_ARMED:-0}" == 1 ]]; then
    if hc_run timeout --kill-after=5s 30s docker exec "${DB_CONTAINER:-coprocessor-and-kms-db}" \
      psql -U postgres -v ON_ERROR_STOP=1 -d "$database" -c \
      'SET lock_timeout=5000; SET statement_timeout=15000; DROP TRIGGER IF EXISTS consensus_test_fork_gate ON dependence_chain; DROP FUNCTION IF EXISTS public.consensus_test_fork_gate(); DROP TABLE IF EXISTS public.consensus_test_fork_gate;' >/dev/null; then
      FORK_GATE_ARMED=0
    else status=1; fi
  fi
  if [[ "${FORK_REPLAY_ARMED:-0}" == 1 ]]; then
    if hc_run timeout --kill-after=5s 30s docker exec "${DB_CONTAINER:-coprocessor-and-kms-db}" \
      psql -U postgres -v ON_ERROR_STOP=1 -d coprocessor -c \
      'SET lock_timeout=5000; SET statement_timeout=15000; DROP TRIGGER IF EXISTS consensus_test_replay_insert ON computations; DROP FUNCTION IF EXISTS public.consensus_test_replay_insert(); DROP TABLE IF EXISTS public.consensus_test_replay_inserts;' >/dev/null; then
      FORK_REPLAY_ARMED=0
    else status=1; fi
  fi
  if [[ "$status" != 0 ]]; then
    local target action blocked_worker; blocked_worker="$(fork_prefix)-tfhe-worker"
    while IFS='|' read -r target action; do
      [[ -n "$target" ]] || continue
      [[ "${FORK_GATE_ARMED:-0}" != 1 || "$target" != "$blocked_worker" ]] || continue
      [[ "${FORK_REPLAY_ARMED:-0}" != 1 || "$target" != coprocessor-host-listener-poller ]] || continue
      case "$action" in resume) sc_resume "$target" || true ;; start) sc_start "$target" || true ;; esac
    done < <(tac "$SC_RESTORE_LOG")
    printf 'fork_operator=%s\nfork_gate_armed=%s\nfork_replay_armed=%s\nrestore_log=%s\n' \
      "$FORK_OPERATOR" "${FORK_GATE_ARMED:-0}" "${FORK_REPLAY_ARMED:-0}" "$SC_RESTORE_LOG" > "$SP_RUNTIME_DIR/fork-recovery"
    touch "$SP_RUNTIME_DIR/cleanup-failed"
    mkdir -p "$(dirname "$SP_CONTAMINATION")"
    printf 'phase_registry=%s\nrestore_log=%s\nfork_recovery=%s\n' \
      "$SP_PHASE_REGISTRY" "$SC_RESTORE_LOG" "$SP_RUNTIME_DIR/fork-recovery" > "$SP_CONTAMINATION"
  fi
  return "$status"
}
fork_restore_case() {
  sp_case_cleanup && sc_run_restores
}
cleanup_on_exit() {
  local status=$? cleanup_ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  if ! sp_cancel_all || ! sp_recover_suite_state || ! sp_case_cleanup; then
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
    *) echo "usage: run-fork-consensus.sh [--case main|f4|f5|all] [--fork-operator <index>]" >&2; exit 2 ;;
  esac
done
case "$CASE" in main|f4|f5|all) ;; *) echo "unknown case $CASE" >&2; exit 2 ;; esac

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

# Docker may discard network addresses while stopped. Call only for the live
# poller, before the stop and again after its replacement is running.
poller_database_ip() {
  docker inspect "$1" "${DB_CONTAINER:-coprocessor-and-kms-db}" | bun -e '
    const { isIP } = require("node:net");
    const [poller, database] = JSON.parse(await Bun.stdin.text());
    if (poller.State.Status !== "running" || !(poller.State.Pid > 0)) throw new Error("poller is not a live process");
    const dbNetworks = new Set(Object.values(database.NetworkSettings.Networks).map(n => n.NetworkID));
    const addresses = Object.values(poller.NetworkSettings.Networks).filter(n => dbNetworks.has(n.NetworkID)).map(n => n.IPAddress);
    if (addresses.length !== 1 || isIP(addresses[0]) === 0) throw new Error("cannot uniquely identify a valid poller DB-client IP");
    console.log(addresses[0]);
  '
}

case_main() {
  sp_case_start FORK-01-COLLIDING-HANDLE FORK-02-DISTINCT-HANDLES FORK-03-ORPHAN-ALLOW-INERT || die "cannot establish fork deadline"
  local started; started="$(cr_now)"
  local out status=0
  log "F1 to F3 on the fork topology"
  run_phase out main "[fork-consensus/F3] CASE COMPLETE" || status=$?
  local id case_id evidence reason
  local before_failures=$FAILURES
  reason="$(cr_failure_reason "$out")"
  echo "$out"
  for id in F1 F2 F3; do
    case "$id" in
      F1) case_id=FORK-01-COLLIDING-HANDLE ;;
      F2) case_id=FORK-02-DISTINCT-HANDLES ;;
      F3) case_id=FORK-03-ORPHAN-ALLOW-INERT ;;
    esac
    if grep -qF "[fork-consensus/${id}] CASE COMPLETE" <<<"$out"; then
      local -a fork_evidence=()
      if evidence="$(printf '%s\n' "$out" | bun "$SCRIPT_DIR/read-fork-evidence.ts" "$CR_RUN_ID" "$case_id")"; then
        mapfile -t fork_evidence <<<"$evidence"
        cr_record_checked_pass "$case_id" started_at="$started" cleanup=ok "${fork_evidence[@]}" assert="assertions-ran=pass"
      else
        cr_record "$case_id" FAIL started_at="$started" cleanup=ok detail="completed case lacks its own valid fault observation"
        FAILURES=$((FAILURES + 1))
      fi
    else
      cr_record "$case_id" FAIL started_at="$started" cleanup=ok detail="$reason"
      FAILURES=$((FAILURES + 1))
    fi
  done
  if [[ "$status" -ne 0 && "$FAILURES" -eq "$before_failures" ]]; then
    # All bodies completed: the failure belongs to a shared hook or phase gate.
    RS_FINAL_FAILURE=1
    FAILURES=$((FAILURES + 1))
  fi
}

case_f4() {
  sp_case_start FORK-04-STRANDED-CHILD || die "cannot establish F4 deadline"
  local started; started="$(cr_now)"
  local worker; worker="$(fork_prefix)-tfhe-worker"
  docker inspect "$worker" >/dev/null 2>&1 || {
    cr_record FORK-04-STRANDED-CHILD NOT_APPLICABLE started_at="$started" cleanup=not_required \
      detail="$worker is absent from this topology"
    return 0
  }

  log "stalling $worker so a gated child can be observed rather than raced"
  if ! sc_pause "$worker"; then
    cr_record FORK-04-STRANDED-CHILD INVALID started_at="$started" cleanup=ok \
      detail="could not stall $worker, so no gated child could be constructed"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local fault_observed; fault_observed="$(cr_now)"

  docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/fork-child.json" >/dev/null 2>&1 || true
  local out status=0
  FORK_GATE_ARMED=1
  run_phase out f4-arm "[fork-consensus/F4-arm] CASE COMPLETE" || status=$?
  if [[ "$status" -ne 0 ]]; then
    echo "$out" | tail -25
    local arm_cleanup=ok
    fork_restore_case || arm_cleanup=failed
    cr_record FORK-04-STRANDED-CHILD INVALID started_at="$started" cleanup="$arm_cleanup" \
      fault_observed_at="$fault_observed" \
      detail="the gated child could not be constructed and observed: $(cr_failure_reason "$out")"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local child_chain
  child_chain="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/fork-child.json" 2>/dev/null \
    | sed -n 's/.*"childChain": *"\([0-9a-f]*\)".*/\1/p' | head -1)"

  log "resuming $worker so the repair path can act"
  if ! sc_resume "$worker"; then
    cr_record FORK-04-STRANDED-CHILD FAIL started_at="$started" cleanup=failed \
      cleanup_detail="$worker is left stopped" fault_observed_at="$fault_observed" \
      detail="$worker could not be resumed"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local recovery_observed; recovery_observed="$(cr_now)"

  local verify_out status2=0
  run_phase verify_out f4-verify "[fork-consensus/F4-verify] CASE COMPLETE" || status2=$?
  local cleanup_state=ok
  fork_restore_case || cleanup_state=failed
  local -a record=(
    started_at="$started"
    fault_observed_at="$fault_observed"
    recovery_observed_at="$recovery_observed"
    cleanup="$cleanup_state"
  )
  [[ -n "$child_chain" ]] && record+=(workload="chain:$child_chain")
  if [[ "$status2" -eq 0 && "$cleanup_state" == ok ]]; then
    # The suite's own [consensus-assertion] receipts are the evidence; this
    # host observed the process fault and recovery, nothing about the chain.
    cr_record_checked_pass FORK-04-STRANDED-CHILD "${record[@]}"
    echo "[FORK-04-STRANDED-CHILD] PASS (child chain ${child_chain:-unknown})"
  else
    echo "$verify_out" | tail -25
    cr_record FORK-04-STRANDED-CHILD FAIL "${record[@]}" detail="$(cr_failure_reason "$verify_out")"
    FAILURES=$((FAILURES + 1))
  fi
}

case_f5() {
  sp_case_start FORK-05-REPLAY || die "cannot establish F5 deadline"
  local started; started="$(cr_now)"
  local prepared prepare_status=0
  log "minting the identified graph before stopping its poller"
  run_phase prepared f5-prepare "[fork-consensus/F5-prepare] CASE COMPLETE" || prepare_status=$?
  if [[ "$prepare_status" != 0 ]]; then
    echo "$prepared" | tail -25
    cr_record FORK-05-REPLAY INVALID started_at="$started" cleanup=not_required \
      detail="could not establish a nonempty graph to replay: $(cr_failure_reason "$prepared")"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  # The CANONICAL operator's poller, matching the suite: the fork follower's
  # chain is rewound by every reseed, so its poller's cursor outruns the chain
  # and no already-ingested range survives to be replayed.
  local poller; poller="coprocessor-host-listener-poller"
  docker inspect "$poller" >/dev/null 2>&1 || {
    cr_record FORK-05-REPLAY NOT_APPLICABLE started_at="$started" cleanup=not_required \
      detail="$poller is absent from this topology, so the poller's own replay path cannot be driven"
    return 0
  }

  log "stopping $poller so its cursor can be rewound where it will be read"
  local identity_before; identity_before="$(sc_identity "$poller")"
  local poller_ip
  poller_ip="$(poller_database_ip "$poller")" || {
    cr_record FORK-05-REPLAY INVALID started_at="$started" cleanup=not_required \
      detail="cannot identify a valid database-client IP while the poller is running"
    FAILURES=$((FAILURES + 1))
    return 1
  }
  if ! sc_stop "$poller"; then
    cr_record FORK-05-REPLAY INVALID started_at="$started" cleanup=ok \
      detail="could not stop $poller; rewinding under a running poller replays nothing"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local fault_observed; fault_observed="$(cr_now)"

  # Persist the address captured before Docker could discard it on stop.
  docker exec "$TEST_CONTAINER" node -e '
    const fs = require("fs"), path = require("path");
    const [dir, clientAddress, processBefore] = process.argv.slice(1);
    fs.mkdirSync(dir, {recursive:true});
    const file = path.join(dir,"fork-replay-poller.json");
    fs.writeFileSync(file+".partial",JSON.stringify({name:"fork-replay-poller",ready:true,payload:{clientAddress,processBefore}}));
    fs.renameSync(file+".partial",file);
  ' "$HANDSHAKE_DIR" "$poller_ip" "$identity_before" || die "cannot publish poller replay identity"

  docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/fork-replay.json" >/dev/null 2>&1 || true
  local out status=0
  FORK_REPLAY_ARMED=1
  run_phase out f5-arm "[fork-consensus/F5-arm] CASE COMPLETE" || status=$?
  if [[ "$status" -ne 0 ]]; then
    echo "$out" | tail -25
    local arm_cleanup=ok
    fork_restore_case || arm_cleanup=failed
    cr_record FORK-05-REPLAY INVALID started_at="$started" cleanup="$arm_cleanup" \
      fault_observed_at="$fault_observed" \
      detail="the cursor could not be rewound over an identified range: $(cr_failure_reason "$out")"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local range
  range="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/fork-replay.json" 2>/dev/null \
    | tr -d ' \n' | sed -n 's/.*"rewoundTo":\([0-9]*\).*"watermarkBefore":\([0-9]*\).*/\1-\2/p;s/.*"watermarkBefore":\([0-9]*\).*"rewoundTo":\([0-9]*\).*/\2-\1/p' | head -1)"

  log "restarting $poller so it reads the rewound cursor"
  if ! sc_start "$poller"; then
    cr_record FORK-05-REPLAY FAIL started_at="$started" cleanup=failed \
      cleanup_detail="$poller is left stopped" fault_observed_at="$fault_observed" \
      detail="$poller could not be restarted"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local identity_after; identity_after="$(sc_identity "$poller")"
  local restarted_ip
  if [[ "$identity_after" == "$identity_before" || ! "$identity_after" =~ [[:space:]]pid=[1-9][0-9]*[[:space:]] ]] ||
      ! restarted_ip="$(poller_database_ip "$poller")" || [[ "$restarted_ip" != "$poller_ip" ]]; then
    cr_record FORK-05-REPLAY INVALID started_at="$started" cleanup=ok \
      fault_observed_at="$fault_observed" \
      detail="poller replacement did not preserve the audited client IP or did not establish a new live process identity"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local recovery_observed; recovery_observed="$(cr_now)"

  local verify_out status2=0
  run_phase verify_out f5-verify "[fork-consensus/F5-verify] CASE COMPLETE" || status2=$?
  local cleanup_state=ok
  fork_restore_case || cleanup_state=failed
  local -a record=(
    started_at="$started"
    fault_observed_at="$fault_observed"
    recovery_observed_at="$recovery_observed"
    process_before="$poller=$identity_before"
    process_after="$poller=$identity_after"
    cleanup="$cleanup_state"
  )
  [[ -n "$range" ]] && record+=(workload="blocks:$range")
  if [[ "$status2" -eq 0 && "$cleanup_state" == ok ]]; then
    # The suite's receipts carry the replay evidence; the host only observed
    # the poller's process identity change, which is recorded above.
    cr_record_checked_pass FORK-05-REPLAY "${record[@]}"
    echo "[FORK-05-REPLAY] PASS (range ${range:-unknown})"
  else
    echo "$verify_out" | tail -25
    cr_record FORK-05-REPLAY FAIL "${record[@]}" detail="$(cr_failure_reason "$verify_out")"
    FAILURES=$((FAILURES + 1))
  fi
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR"
  docker inspect "$TEST_CONTAINER" >/dev/null 2>&1 || die "test container $TEST_CONTAINER is not running"
  local count; count="$(operator_count)"
  [[ "$count" -ge 3 ]] || die "the fork gate needs three operators, found $count"
  [[ "$FORK_OPERATOR" == 2 ]] || die "the managed fork topology routes operator 2 to the fork"
  local observed_topology
  observed_topology="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" "$count")" || die "cannot verify active topology"
  eval "$observed_topology"
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
    for declined in FORK-02-DISTINCT-HANDLES FORK-03-ORPHAN-ALLOW-INERT FORK-04-STRANDED-CHILD FORK-05-REPLAY; do
      cr_skip_wrong_scenario "$declined" >/dev/null
    done
    echo "run $CR_RUN_ID: the fork cases need the $(cr_declared_scenario FORK-01-COLLIDING-HANDLE) scenario"
    return 0
  fi

  [[ "$CASE" == all || "$CASE" == main ]] && case_main
  # F5 BEFORE F4. F5 replays a range the poller has already ingested, and F4
  # reseeds the fork from canonical, which removes that range from the chain --
  # so in the other order F5 can only decline, having been asked to replay
  # blocks that no longer exist.
  [[ "$CASE" == all || "$CASE" == f5 ]] && case_f5
  [[ "$CASE" == all || "$CASE" == f4 ]] && case_f4

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
