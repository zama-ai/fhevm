#!/usr/bin/env bash
# The degraded-cluster cases that need an operator or a service taken away.
#
# Stopping services needs host process control, which the e2e container does not
# have and should not have, so the faults are applied here and the assertions
# are made by `test/consensus/degradedConsensus.ts` between the steps.
#
# Three things this runner had to stop doing.
#
# It passed `PROBE_EXPECT_QUORUM=0` for C4a and described the case as asserting
# that a unanimous topology must NOT reach quorum. `0` meant "do not check", so
# the case asserted nothing about quorum in either direction -- and the same `0`
# ran on 2-of-3 in CI, where quorum should form, so the cell could not tell the
# two topologies apart. The quorum expectation is now derived by the suite from
# the RUNNING gateway's threshold, and this runner records which of DEG-03 and
# DEG-04 the topology actually exercised.
#
# It restored the victim and then minted a NEW handle to show it worked. That is
# a liveness check. The handles minted during the outage are published by the
# outage phase and re-read by the recovery phase, so the case is about backlog
# convergence rather than about fresh work.
#
# Read the full service set from the running topology so an unlisted ingestion
# path cannot remain active and invalidate the operator-outage precondition.
#
#   run-degraded-consensus.sh --case agreement|canary|same-block|outage|gw|core
#                             [--victim <index>]
#
# There is deliberately no `all`: the core cases require drift auto-revert
# enabled on every gateway listener and `gw` requires it disabled, so no
# single stack can establish both groups.
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
GW_POISON_ARMED=0
sp_case_cleanup() {
  [[ "$GW_POISON_ARMED" != 1 ]] || gw_restore_originals
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

CASE=""
VICTIM="${DEGRADED_VICTIM_OPERATOR:-2}"
usage() {
  echo "usage: run-degraded-consensus.sh --case agreement|canary|same-block|outage|gw|core [--victim <index>]" >&2
  echo "       (core and gw need opposite drift auto-revert settings, so there is no 'all')" >&2
  exit 2
}
while [[ $# -gt 0 ]]; do
  case "$1" in
    --case) CASE="${2:?--case needs a value}"; shift 2 ;;
    --victim) VICTIM="${2:?--victim needs an operator index}"; shift 2 ;;
    *) usage ;;
  esac
done
[[ -n "$CASE" ]] || usage
case "$CASE" in agreement|canary|same-block|outage|gw|core) ;; *) echo "unknown case $CASE" >&2; usage ;; esac

die() { RS_FINAL_FAILURE=1; echo "degraded-consensus: $*" >&2; exit 1; }
log() { printf '\n=== %s\n' "$*"; }

env_value() { sed -n "s/^$1=//p" "$2" | tail -1; }

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

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
    [[ -z "${CASE_DEADLINE_EPOCH:-}" ]] || case_seconds_left >/dev/null || return 124
    # Alpine uses BusyBox timeout; -k is supported by both BusyBox and GNU.
    hc_run timeout --kill-after=1s 4s docker exec "$TEST_CONTAINER" timeout -k 1 2 getent hosts "$name" >/dev/null 2>&1 && return 0
  done
  return 1
}

ensure_test_container_resolves() {
  local i
  for i in $(seq 1 6); do
    container_resolves_something && return 0
    sleep 5
  done
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

# Runs one phase of the degraded suite. The verdict is the exit status plus the
# phase's own completion marker.
run_phase() {
  local -n __phase_out="$1"
  local phase="$2"
  local watchdog_disabled=0
  [[ "$phase" == drift ]] && watchdog_disabled=1
  ensure_test_container_resolves || die "the test container cannot resolve service names even after a restart"
  local coprocessor_env="$ENV_DIR/coprocessor.env"
  cr_run_suite __phase_out "[degraded/${phase}] CASE COMPLETE" \
    sp_exec \
      -e RUN_DEGRADED_CONSENSUS=1 \
      -e "DEGRADED_PHASE=$phase" \
      -e "DEGRADED_VICTIM_OPERATOR=$VICTIM" \
      -e "COPROCESSOR_COUNT=$(operator_count)" \
      -e "CONSENSUS_THRESHOLD=$CONSENSUS_THRESHOLD" \
      -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" \
      -e "CONSENSUS_WATCHDOG_DISABLED=$watchdog_disabled" -e CONSENSUS_WATCHDOG_STALL_MS=2400000 \
      -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL "$coprocessor_env")" \
      -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS "$coprocessor_env")" \
      -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")" \
      -e "TFHE_WORKER_METRICS_URLS=$(gpu_worker_metrics_urls "$(operator_count)" "$TEST_CONTAINER" coprocessor-and-kms-db)" \
      -e npm_config_update_notifier=false \
      "$TEST_CONTAINER" \
      npx hardhat test test/consensus/degradedConsensus.ts --network "$TEST_NETWORK"
}

FAILURES=0

# Core degraded coverage deliberately retains production drift recovery.
core_require_auto_revert() {
  local index prefix container valid=1 case_id
  for ((index=0; index<$(operator_count); index++)); do
    prefix="$([[ "$index" == 0 ]] && echo coprocessor || echo "coprocessor$index")"
    container="$prefix-gw-listener"
    if ! hc_run timeout --kill-after=1s 10s docker inspect "$container" | bun -e '
      const c=(await Bun.stdin.json())[0];
      const env=Object.fromEntries((c.Config.Env??[]).map(v=>{const i=v.indexOf("=");return [v.slice(0,i),v.slice(i+1)];}));
      const args=[...(c.Config.Entrypoint??[]),...(c.Config.Cmd??[])];
      if(c.State.Status!=="running" || env.DRIFT_AUTO_REVERT_ENABLED!=="true" || args.some(v=>String(v).startsWith("--drift-auto-revert-enabled"))) process.exitCode=1;
    '; then valid=0; fi
  done
  [[ "$valid" != 1 ]] || return 0
  for case_id in "$@"; do
    cr_skip_wrong_scenario "$case_id" && continue
    cr_record "$case_id" INVALID cleanup=not_required detail="core degraded coverage requires live gateway listeners with drift auto-revert enabled"
  done
  FAILURES=$((FAILURES + 1))
  return 1
}

degraded_record_pass() {
  local case_id="$1"; shift
  if [[ "$case_id" == DEG-06-GW-LISTENER-INFLIGHT ]]; then
    cr_record_checked_pass "$case_id" "$@" artifact="drift_auto_revert=false"
  else
    cr_record_checked_pass "$case_id" "$@" artifact="drift_auto_revert=true"
  fi
}

# Records a simple single-phase case.
simple_case() {
  local case_id="$1" phase="$2" started out status=0
  cr_skip_wrong_scenario "$case_id" && return 0
  started="$(cr_now)"
  sp_case_start "$case_id" || die "cannot establish case deadline"
  core_require_auto_revert "$case_id" || return 1
  log "$case_id ($phase)"
  run_phase out "$phase" || status=$?
  if [[ "$status" -eq 0 ]]; then
    # A case that declares a fault must carry the evidence that it landed. The
    # canary phase poisons a digest and says so; without those fields a PASS
    # here is indistinguishable from a run where nothing was tampered with, and
    # the aggregate rejects it -- correctly.
    local -a evidence=()
    local poisoned_line poisoned_handle poisoned_at
    poisoned_line="$(grep -o 'canary poisoned 0x[0-9a-f]* at [0-9TZ:.-]*' <<<"$out" | tail -1)"
    if [[ -n "$poisoned_line" ]]; then
      poisoned_handle="$(grep -oE '0x[0-9a-f]+' <<<"$poisoned_line" | tail -1)"
      poisoned_at="$(awk '{print $NF}' <<<"$poisoned_line")"
      evidence+=(workload="$poisoned_handle" fault_observed_at="$poisoned_at")
    fi
    degraded_record_pass "$case_id" started_at="$started" cleanup=not_required "${evidence[@]}" \
      assert="assertions-ran=pass:completion marker printed"
    echo "[$case_id] PASS"
  else
    echo "$out" | tail -25
    cr_record "$case_id" FAIL started_at="$started" cleanup=not_required \
      detail="$(cr_failure_reason "$out")" assert="assertions-ran=fail"
    echo "[$case_id] FAIL"
    FAILURES=$((FAILURES + 1))
  fi
}

# The gateway threshold, as the suite reported it. Used to decide which of
# DEG-03 / DEG-04 this topology exercised, so the other is recorded
# NOT_APPLICABLE rather than silently omitted.
threshold_from_output() {
  sed -n 's/.*quorum \(required\|forbidden\).*/\1/p' <<<"$1" | tail -1
}

case_outage() {
  sp_case_start DEG-03-MAJORITY-AVAILABLE || die "cannot establish outage deadline"
  core_require_auto_revert DEG-03-MAJORITY-AVAILABLE DEG-04-UNANIMOUS-NO-QUORUM DEG-05-BACKLOG-CONVERGENCE || return 1
  local started; started="$(cr_now)"
  local -a services=()
  mapfile -t services < <(sc_operator_services "$VICTIM")
  [[ "${#services[@]}" -gt 0 ]] || die "no services found for operator $VICTIM; the topology cannot be read"
  log "taking operator $VICTIM fully offline (${#services[@]} service(s): ${services[*]})"

  local service
  # One of the two outage cases is always declared under the other scenario;
  # recording it here on this stack's scenario would make the aggregate report
  # a bogus topology mismatch for the sibling.
  record_outage_pair() {
    local id
    for id in DEG-03-MAJORITY-AVAILABLE DEG-04-UNANIMOUS-NO-QUORUM; do
      cr_skip_wrong_scenario "$id" || cr_record "$id" "$@"
    done
  }
  for service in "${services[@]}"; do
    if ! sc_stop "$service"; then
      local stop_cleanup=ok
      sc_run_restores || stop_cleanup=failed
      record_outage_pair INVALID started_at="$started" cleanup="$stop_cleanup" \
        detail="could not stop $service, so operator $VICTIM was never fully offline"
      FAILURES=$((FAILURES + 1))
      return 1
    fi
  done
  local fault_observed; fault_observed="$(cr_now)"
  echo "  operator $VICTIM verified offline across every configured service"

  docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/degraded-outage.json" >/dev/null 2>&1 || true
  local out status=0
  run_phase out outage || status=$?
  local expectation; expectation="$(threshold_from_output "$out")"
  local workloads
  workloads="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/degraded-outage.json" 2>/dev/null \
    | grep -oE '0x[0-9a-f]{64}' | sort -u | tr '\n' ' ')"

  local -a record=(
    started_at="$started"
    fault_observed_at="$fault_observed"
    cleanup=not_required
  )
  local id
  for id in $workloads; do record+=(workload="$id"); done

  local exercised="" other=""
  case "$expectation" in
    required) exercised=DEG-03-MAJORITY-AVAILABLE; other=DEG-04-UNANIMOUS-NO-QUORUM ;;
    forbidden) exercised=DEG-04-UNANIMOUS-NO-QUORUM; other=DEG-03-MAJORITY-AVAILABLE ;;
    *) exercised=""; other="" ;;
  esac

  if [[ "$status" -ne 0 || -z "$exercised" ]]; then
    echo "$out" | tail -25
    local reason="${status:+the outage assertions failed: $(cr_failure_reason "$out")}"
    [[ -z "$exercised" ]] && reason="the suite did not report which quorum expectation the gateway's threshold implies"
    record_outage_pair FAIL "${record[@]}" detail="$reason"
    FAILURES=$((FAILURES + 1))
  else
    degraded_record_pass "$exercised" "${record[@]}" \
      assert="victim-offline=pass:every configured service of operator $VICTIM verified stopped" \
      assert="survivors-agree=pass" \
      assert="quorum-$expectation=pass"
    # The other direction needs a different threshold, which this topology does
    # not have. Recorded so the aggregate shows it rather than omitting it.
    cr_record "$other" NOT_APPLICABLE "${record[@]}" \
      detail="this topology's gateway threshold exercises $expectation quorum; the other direction needs a different threshold"
    echo "[$exercised] PASS ($expectation quorum); [$other] NOT_APPLICABLE on this threshold"
  fi

  # DEG-05 is a three-of-three case; the outage phase itself runs on either
  # threshold. Without this a two-of-three session records a DEG-05 row the
  # inventory cannot accept, and the whole union aggregate fails on it.
  if cr_skip_wrong_scenario DEG-05-BACKLOG-CONVERGENCE; then
    log "restoring operator $VICTIM"
    local restore_only=0
    for service in "${services[@]}"; do sc_start "$service" || restore_only=1; done
    [[ "$restore_only" == 0 ]] || FAILURES=$((FAILURES + 1))
    return 0
  fi

  log "restoring operator $VICTIM"
  local restore_failed=0
  for service in "${services[@]}"; do
    sc_start "$service" || restore_failed=1
  done
  local recovery_observed; recovery_observed="$(cr_now)"
  if [[ "$restore_failed" == 1 ]]; then
    cr_record DEG-05-BACKLOG-CONVERGENCE FAIL started_at="$started" cleanup=failed \
      cleanup_detail="operator $VICTIM could not be restored" \
      detail="operator $VICTIM could not be restored, so backlog convergence cannot be measured"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  if ! "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$(operator_count)" >/dev/null 2>&1; then
    cr_record DEG-05-BACKLOG-CONVERGENCE FAIL started_at="$started" cleanup=failed \
      cleanup_detail="a queue is not served by exactly one worker after the restore" \
      detail="the restored operator's queue is not served by exactly one worker"
    FAILURES=$((FAILURES + 1))
    return 1
  fi

  local recovery_out status2=0
  run_phase recovery_out recovery || status2=$?
  local -a record2=(
    started_at="$started"
    fault_observed_at="$fault_observed"
    recovery_observed_at="$recovery_observed"
    cleanup=ok
  )
  for id in $workloads; do record2+=(workload="$id"); done
  if [[ "$status2" -eq 0 ]]; then
    degraded_record_pass DEG-05-BACKLOG-CONVERGENCE "${record2[@]}" \
      assert="same-handles=pass:the recovery phase read the outage phase's handles" \
      assert="victim-converged=pass" \
      assert="quorum-required=pass"
    echo "[DEG-05-BACKLOG-CONVERGENCE] PASS"
  else
    echo "$recovery_out" | tail -25
    cr_record DEG-05-BACKLOG-CONVERGENCE FAIL "${record2[@]}" detail="$(cr_failure_reason "$recovery_out")"
    echo "[DEG-05-BACKLOG-CONVERGENCE] FAIL"
    FAILURES=$((FAILURES + 1))
  fi
}

# This entrypoint does not run a Hardhat before-hook or mint fresh work. It uses
# originals persisted before arming, including after a killed/failed arm phase.
gw_restore_originals() {
  [[ "$GW_POISON_ARMED" == 1 ]] || return 0
  if ! sp_recovery_exec "$TEST_CONTAINER" env "COPROCESSOR_COUNT=$(operator_count)" "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" \
      node -r ts-node/register/transpile-only -e \
      'require("./test/consensus/gatewayRecovery").restorePendingGatewayDigests().catch(e => {console.error(e.message); process.exitCode=1;})'; then
    echo "gateway digest cleanup failed; originals remain in $HANDSHAKE_DIR/degraded-gw-event.json" >&2
    return 1
  fi
  GW_POISON_ARMED=0
}

gw_require_no_auto_revert() {
  docker inspect "$1" | bun -e '
    const c=(await Bun.stdin.json())[0];
    const env=Object.fromEntries((c.Config.Env??[]).map(v=>{const i=v.indexOf("=");return [v.slice(0,i),v.slice(i+1)];}));
    const args=[...(c.Config.Entrypoint??[]),...(c.Config.Cmd??[])];
    if ((env.DRIFT_AUTO_REVERT_ENABLED??"false")!=="false" || args.some(v=>String(v).startsWith("--drift-auto-revert-enabled"))) {
      console.error("DEG06 requires auto-revert disabled before local digest poisoning"); process.exitCode=1;
    }
  '
}

gw_wait_exact_event_warnings() {
  local event_file="$1" since="$2" evidence_dir="$3"; shift 3
  local listener result all_seen all_seen_at=0 deadline=$((SECONDS + 480))
  while ((SECONDS < deadline)); do
    all_seen=1
    for listener in "$@"; do
      sc_logs_since "$listener" "$since" >"$evidence_dir/$listener.log" || return 1
      result=0
      bun "$SCRIPT_DIR/gateway-event-evidence.ts" "$event_file" "$evidence_dir/$listener.log" \
        >"$evidence_dir/$listener-event.log" || result=$?
      case "$result" in 0) ;; 2) all_seen=0 ;; *) return 1 ;; esac
    done
    if [[ "$all_seen" == 1 ]]; then
      [[ "$all_seen_at" != 0 ]] || all_seen_at=$SECONDS
      # Keep observing through another poll interval before restoring poison,
      # so repeated processing in the same catch-up batch is rejected too.
      ((SECONDS - all_seen_at >= 4)) && return 0
    fi
    sleep 2
  done
  echo "replacement gateway listeners never acknowledged the precise pending event" >&2
  return 1
}

case_gw() {
  local status=0 prior_failures="$FAILURES"
  # Every early failure belongs to DEG-06, including filesystem/transport and
  # partial recovery errors. Otherwise finalization can blame a passed sibling.
  local CR_TERMINAL_FILE="$SP_RUNTIME_DIR/degraded-gw-terminal"
  : > "$CR_TERMINAL_FILE" || die "cannot track gateway case verdict"
  case_gw_body || status=$?
  if [[ "$status" != 0 ]]; then
    if ! grep -Fxq DEG-06-GW-LISTENER-INFLIGHT "$CR_TERMINAL_FILE"; then
      cr_record DEG-06-GW-LISTENER-INFLIGHT INVALID cleanup=ok \
        detail="gateway case stopped before its event/recovery evidence could be recorded; final recovery gates publication" || RS_FINAL_FAILURE=1
    fi
    (( FAILURES > prior_failures )) || FAILURES=$((FAILURES + 1))
  fi
  return "$status"
}

case_gw_body() {
  sp_case_start DEG-06-GW-LISTENER-INFLIGHT || die "cannot establish gateway case deadline"
  cr_skip_wrong_scenario DEG-06-GW-LISTENER-INFLIGHT && return 0
  local started; started="$(cr_now)"
  local count; count="$(operator_count)"
  local -a listeners=() index prefix
  for ((index = 0; index < count; index++)); do
    prefix="$([[ "$index" == 0 ]] && echo coprocessor || echo "coprocessor$index")"
    docker inspect "${prefix}-gw-listener" >/dev/null 2>&1 && listeners+=("${prefix}-gw-listener")
  done
  if [[ "${#listeners[@]}" -ne "$count" ]]; then
    cr_record DEG-06-GW-LISTENER-INFLIGHT INVALID started_at="$started" cleanup=not_required \
      detail="expected $count gateway listeners, could inspect only ${#listeners[@]}"
    FAILURES=$((FAILURES + 1)); return 1
  fi
  for listener in "${listeners[@]}"; do
    if ! gw_require_no_auto_revert "$listener"; then
      cr_record DEG-06-GW-LISTENER-INFLIGHT INVALID started_at="$started" cleanup=not_required \
        detail="cannot prove drift auto-revert disabled on $listener"
      FAILURES=$((FAILURES + 1)); return 1
    fi
  done

  log "stalling ${#listeners[@]} gateway listener(s) so an event can be left pending"
  local listener
  # Reset an exhausted supervisor before creating pending work: recreation can
  # run the old event loop, so it is not safe after the event has been armed.
  for listener in "${listeners[@]}"; do
    if [[ "$(sc_restart_budget "$listener")" == exhausted ]]; then
      if ! sc_reset_restart_budget "$listener"; then
        cr_record DEG-06-GW-LISTENER-INFLIGHT INVALID started_at="$started" cleanup=not_required \
          detail="could not prepare $listener's supervisor before creating pending work"
        FAILURES=$((FAILURES + 1))
        return 1
      fi
    fi
  done
  for listener in "${listeners[@]}"; do
    if ! sc_pause "$listener"; then
      sc_run_restores || true
      cr_record DEG-06-GW-LISTENER-INFLIGHT INVALID started_at="$started" cleanup=ok \
        detail="could not stall $listener, so no event could be left pending"
      FAILURES=$((FAILURES + 1))
      return 1
    fi
  done
  local fault_observed; fault_observed="$(cr_now)"

  if ! docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/degraded-gw-event.json"; then
    sc_run_restores || true
    FAILURES=$((FAILURES + 1)); return 1
  fi
  GW_POISON_ARMED=1
  local arm_out status=0
  run_phase arm_out gw-arm || status=$?
  if [[ "$status" -ne 0 ]]; then
    echo "$arm_out" | tail -25
    gw_restore_originals || { FAILURES=$((FAILURES + 1)); return 1; }
    sc_run_restores || true
    cr_record DEG-06-GW-LISTENER-INFLIGHT INVALID started_at="$started" cleanup=ok \
      fault_observed_at="$fault_observed" \
      detail="no identified gateway event could be left pending: $(cr_failure_reason "$arm_out")"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local workloads
  workloads="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/degraded-gw-event.json" 2>/dev/null \
    | grep -oE '0x[0-9a-f]{64}' | sort -u | tr '\n' ' ')"

  # Publish beside the run's results file, wherever the caller configured that
  # to be: a probe with its own CONSENSUS_RESULTS_DIR must not write here.
  local evidence_dir="${RS_PUBLISH_RESULTS:-$(dirname "$(cr_results_file)")}/gateway-${CR_RUN_ID}"
  mkdir -p "$evidence_dir" || return 1
  docker cp "$TEST_CONTAINER:$HANDSHAKE_DIR/degraded-gw-event.json" "$evidence_dir/event.json" || return 1
  local warning_since; warning_since="$(cr_now)"
  log "restarting the gateway listeners with that event still pending"
  local -a identities_before=()
  local restart_failed=0
  for listener in "${listeners[@]}"; do
    identities_before+=("$(sc_identity "$listener")")
  done
  # Queue SIGKILL while each listener remains frozen. Only then unfreeze its
  # cgroup to deliver the fatal signal; the old process cannot ingest the event.
  local position=0
  for listener in "${listeners[@]}"; do
    local before="${identities_before[$position]}"
    position=$((position + 1))
    if [[ "$(sc_state "$listener")" != paused ]]; then
      restart_failed=1
      continue
    fi
    if ! sc_kill "$listener" KILL 1 >/dev/null; then restart_failed=1; continue; fi
    sc_resume "$listener" >/dev/null || { restart_failed=1; continue; }
    sc_wait_replaced "$listener" "$before" 120 >/dev/null || restart_failed=1
  done
  local recovery_observed; recovery_observed="$(cr_now)"
  if [[ "$restart_failed" == 1 ]]; then
    # Some listeners may already have been replaced. Restore the saved
    # digests before resuming any listener still frozen from the partial fault.
    gw_restore_originals || { FAILURES=$((FAILURES + 1)); return 1; }
    sc_run_restores || true
    cr_record DEG-06-GW-LISTENER-INFLIGHT INVALID started_at="$started" cleanup=ok \
      fault_observed_at="$fault_observed" \
      detail="a gateway listener was signalled but never replaced, so the in-flight restart this case asserts about never happened"
    FAILURES=$((FAILURES + 1))
    return 1
  fi

  local verify_out="" status2=0
  gw_wait_exact_event_warnings "$evidence_dir/event.json" "$warning_since" "$evidence_dir" "${listeners[@]}" || status2=1
  if ! gw_restore_originals; then
    cr_record DEG-06-GW-LISTENER-INFLIGHT FAIL started_at="$started" cleanup=failed \
      cleanup_detail="could not restore poisoned gateway digests" detail="gateway event recovery cleanup failed"
    FAILURES=$((FAILURES + 1)); return 1
  fi
  if [[ "$status2" == 0 ]]; then run_phase verify_out gw-verify || status2=$?; fi
  local cleanup_state=ok cleanup_detail=""
  sc_run_restores || { cleanup_state=failed; cleanup_detail="a restore failed"; }
  local -a record=(
    started_at="$started"
    fault_observed_at="$fault_observed"
    recovery_observed_at="$recovery_observed"
    cleanup="$cleanup_state"
  )
  [[ -n "$cleanup_detail" ]] && record+=(cleanup_detail="$cleanup_detail")
  local id
  for id in $workloads; do record+=(workload="$id"); done
  if [[ "$status2" -eq 0 && "$cleanup_state" == ok ]]; then
    degraded_record_pass DEG-06-GW-LISTENER-INFLIGHT "${record[@]}" \
      assert="event-pending-before-restart=pass" \
      artifact="gateway-event-evidence=$evidence_dir" \
      assert="liveness=pass:the replacement compared the identified pending gateway event" \
      assert="safety=pass:exactly one warning matched its handle block hash transaction hash and log index" \
      assert="one-warning-per-replacement=pass" \
      assert="original-digests-restored=pass"
    echo "[DEG-06-GW-LISTENER-INFLIGHT] PASS"
  else
    echo "$verify_out" | tail -25
    local failure_reason; failure_reason="${cleanup_detail:-$(cr_failure_reason "$verify_out")}"
    [[ -n "$failure_reason" ]] || failure_reason="the replacement did not emit exactly one warning for the identified gateway event"
    cr_record DEG-06-GW-LISTENER-INFLIGHT FAIL "${record[@]}" detail="$failure_reason"
    echo "[DEG-06-GW-LISTENER-INFLIGHT] FAIL"
    FAILURES=$((FAILURES + 1))
  fi
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR"
  docker inspect "$TEST_CONTAINER" >/dev/null 2>&1 || die "test container $TEST_CONTAINER is not running"

  local count; count="$(operator_count)"
  [[ "$VICTIM" -lt "$count" ]] || die "victim $VICTIM is outside a $count-operator topology"
  local observed_topology
  observed_topology="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" "$count")" || die "cannot verify active topology"
  eval "$observed_topology"
  export CONSENSUS_SCENARIO CONSENSUS_OPERATORS CONSENSUS_THRESHOLD
  cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
  rs_stage_results || exit 1
  # The image bakes the e2e suite in, so a commit since the last build does not
  # reach the container. A result labelled with a revision whose code never ran
  # is worse than no result.
  suite_identity_assert "$TEST_CONTAINER" || die "the test container is not running this working tree's e2e suite"
  export CONSENSUS_RUN_STARTED_AT="$(cr_now)"

  if gpu_session_active && ! "$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1; then
    die "a queue is served twice before any operator was removed; the fleet is split across backends"
  fi
  "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$count" ||
    die "the fleet does not have exactly one worker per queue before any fault was injected"

  [[ "$CASE" == core || "$CASE" == agreement ]] && simple_case DEG-01-AGREEMENT-QUORUM agreement
  [[ "$CASE" == core || "$CASE" == canary ]] && simple_case MAT-04-CANARY-COMPUTE-DIGEST canary
  [[ "$CASE" == core || "$CASE" == same-block ]] && simple_case DEG-02-SAME-BLOCK-PAIR same-block
  [[ "$CASE" == core || "$CASE" == outage ]] && case_outage
  [[ "$CASE" == gw ]] && case_gw

  # Gated, not merely reported: nothing here kills a worker, so a lease that
  # lapsed did so on its own, and these cases are about what a DEGRADED cluster
  # agrees on -- a claim that reads differently if the work silently changed
  # owner underneath it.
  if ! "$SCRIPT_DIR/consensus-validity.sh" locks --since "$CONSENSUS_RUN_STARTED_AT" --operators "$count"; then
    if [[ "${ALLOW_LOCK_LOSS:-0}" == 1 ]]; then
      echo "  (ALLOW_LOCK_LOSS=1: recorded, not failing)"
    else
      echo "  set ALLOW_LOCK_LOSS=1 to accept this run anyway" >&2
      RS_FINAL_FAILURE=1
      FAILURES=$((FAILURES + 1))
    fi
  fi

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
