#!/usr/bin/env bash
# RFC-020's crash-retry clause, driven host-side.
#
#   "a crash or retry re-executes the whole transaction batch; determinism makes
#    duplicate execution byte-identical and first-write-wins makes it harmless."
#
# Nothing asserted that. This kills one operator's tfhe-worker *while rows are
# still incomplete*, so the worker comes back to work it had already started and
# executes it a second time -- then the in-container half
# (`test/consensus/crashRetryConsensus.ts`) asserts the clause: same bytes as
# the operators that never crashed, every transaction completed with no errored
# rows, and exactly one canonical row per handle.
#
#   run-crash-retry-consensus.sh [--victim <operator-index>] [--handles <n>]
#
# The distinction from the failure matrix's `tfhe-worker-crash` cell is the
# timing. That cell kills a worker, heals it, and then asks whether the
# operators agree -- which catches recovery into different bytes but never
# exercises duplicate execution of the same work, because nothing was in flight.
# Here the kill has to land inside the window where computations are pending, so
# the script watches the victim's database and fires the moment it sees them.
#
# The worker is expected back on its own: compose gives the coprocessor services
# `restart: "on-failure:10"`, and `docker kill` counts as a manual stop that
# suppresses the policy, so this uses SIGABRT to look like a crash rather than
# an operator intervention. If it does not return, the script starts it and says
# so, because a run where the recovery had to be done by hand is not evidence
# about automatic recovery.
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENV_DIR="${REPO_ROOT}/.fhevm/runtime/env"
# shellcheck source=lib/gpu-session.sh
source "${SCRIPT_DIR}/lib/gpu-session.sh"
gpu_normalise_user_bus

readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
readonly DB_CONTAINER="${DB_CONTAINER:-coprocessor-and-kms-db}"
readonly TEST_NETWORK="${TEST_NETWORK:-staging}"

VICTIM=1
HANDLES=6
while [[ $# -gt 0 ]]; do
  case "$1" in
    --victim) VICTIM="${2:?--victim needs an operator index}"; shift 2 ;;
    --handles) HANDLES="${2:?--handles needs a count}"; shift 2 ;;
    *) echo "usage: run-crash-retry-consensus.sh [--victim <index>] [--handles <n>]" >&2; exit 2 ;;
  esac
done

die() { echo "crash-retry: $*" >&2; exit 1; }
log() { printf '\n=== %s\n' "$*"; }

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

env_value() { sed -n "s/^$1=//p" "$2" | tail -1; }

# Under the GPU swap the victim is a host systemd unit, not a container: the
# container is stopped by design, so `docker inspect` hands back pid 0 and the
# run would fall through to signalling nothing. The unit is a transient
# `systemd-run --user` service with Restart=on-failure, the same supervision
# contract the container had, so aborting its main process tests the same
# automatic recovery. It is our own user's unit, so no sudo is needed.
# Same reason as the failure matrix: a detached shell can inherit another user's
# bus address, and then the GPU unit is invisible and the run signals a stopped
# container's pid 0 instead of the worker.



victim_container() {
  [[ "$VICTIM" == 0 ]] && echo "coprocessor-tfhe-worker" || echo "coprocessor${VICTIM}-tfhe-worker"
}

victim_database() {
  [[ "$VICTIM" == 0 ]] && echo "coprocessor" || echo "coprocessor_${VICTIM}"
}

pending_computations() {
  docker exec "$DB_CONTAINER" psql -U postgres -d "$(victim_database)" -tAc \
    'SELECT COUNT(*) FROM computations WHERE is_completed = false AND is_error = false' 2>/dev/null | tr -d ' '
}

# Checked up front, because a missing table looks exactly like "no work in
# flight": the count query fails, the poll reads zero forever, and the run would
# report that it never saw work rather than that it could not look. A stack
# whose operator databases do not carry `computations` is not one this test can
# time a kill against.
require_pending_visibility() {
  local db; db="$(victim_database)"
  local probe
  probe="$(docker exec "$DB_CONTAINER" psql -U postgres -d "$db" -tAc \
    "SELECT COUNT(*) FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
      WHERE c.relname = 'computations' AND n.nspname = 'public'" 2>&1 | tr -d ' ')"
  [[ "$probe" == 1 ]] || die "database $db has no public.computations table, so in-flight work cannot be observed and the kill cannot be timed (got: ${probe:-<query failed>})"
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR; bring one up first"
  local count; count="$(operator_count)"
  [[ "$count" -ge 2 ]] || die "this needs at least two operators: one to crash and one to compare against"
  [[ "$VICTIM" -lt "$count" ]] || die "victim $VICTIM is outside a $count-operator topology"

  local container; container="$(victim_container)"
  docker inspect "$container" >/dev/null 2>&1 || die "$container is not present"
  require_pending_visibility

  local coprocessor_env="$ENV_DIR/coprocessor.env"
  local restarts_before
  # Baseline from the same counter the recovery check will read, or the
  # comparison mixes a unit's NRestarts with a container's RestartCount.
  if [[ -n "$(gpu_unit_for_container "$container")" ]]; then
    restarts_before="$(systemctl --user show "$(gpu_unit_for_container "$container")" --property=NRestarts --value 2>/dev/null || echo 0)"
  else
    restarts_before="$(docker inspect -f '{{.RestartCount}}' "$container" 2>/dev/null || echo 0)"
  fi

  log "starting the workload; victim is $container (operator $VICTIM)"
  # The suite runs in the background so the kill can land while it is minting.
  local suite_log; suite_log="$(mktemp)"
  docker exec \
    -e RUN_CRASH_RETRY_CONSENSUS=1 \
    -e "COPROCESSOR_COUNT=$count" \
    -e "CRASH_VICTIM_OPERATOR=$VICTIM" \
    -e "CRASH_RETRY_HANDLES=$HANDLES" \
    -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL "$coprocessor_env")" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")" \
    -e npm_config_update_notifier=false \
    "$TEST_CONTAINER" \
    npx hardhat test test/consensus/crashRetryConsensus.ts --network "$TEST_NETWORK" >"$suite_log" 2>&1 &
  local suite_pid=$!

  log "waiting for computations to be in flight on $(victim_database)"
  local pending killed=0 i
  for i in $(seq 1 240); do
    kill -0 "$suite_pid" 2>/dev/null || break
    pending="$(pending_computations)"
    if [[ "${pending:-0}" -gt 0 ]]; then
      # The signal has to come from OUTSIDE Docker's API. `docker kill` and
      # `docker stop` are recorded as manual interventions and suppress the
      # restart policy (measured: RestartCount stays 0), which would turn this
      # into a test of manual recovery instead of the automatic recovery L-5
      # added. `docker exec ... sh -c kill` is no good either: the production
      # runtime bases are distroless and have no shell, so it works only by
      # accident of which base a local build used.
      #
      # Signalling the container's main process from the host is neither: to
      # Docker the process simply died on its own with a non-zero status, so the
      # policy applies exactly as it would for a real crash.
      echo "  $pending computation(s) pending; aborting the worker mid-batch"
      gpu_unit="$(gpu_unit_for_container "$container")"
      if [[ -n "$gpu_unit" ]]; then
        main_pid="$(gpu_unit_main_pid "$gpu_unit")"
        echo "  victim is the GPU host unit $gpu_unit (pid ${main_pid:-none})"
      else
        main_pid="$(docker inspect -f '{{.State.Pid}}' "$container" 2>/dev/null)"
      fi
      if [[ -n "$gpu_unit" && -n "$main_pid" && "$main_pid" != 0 ]] && kill -ABRT "$main_pid" 2>/dev/null; then
        echo "  sent SIGABRT to GPU unit pid $main_pid"
      elif [[ -z "$gpu_unit" && -n "$main_pid" && "$main_pid" != 0 ]] && sudo -n kill -ABRT "$main_pid" 2>/dev/null; then
        echo "  sent SIGABRT to host pid $main_pid"
      elif docker exec "$container" sh -c 'kill -ABRT 1' >/dev/null 2>&1; then
        echo "  signalled from inside the container (no host signal available)"
      else
        # Last resort, and it changes what the run proves -- say so rather than
        # letting a manual-recovery run be read as an automatic one.
        docker kill --signal=ABRT "$container" >/dev/null 2>&1 || die "could not signal $container"
        echo "  WARNING: signalled through the Docker API, which suppresses the restart policy;" >&2
        echo "  this run tests manual recovery, not automatic recovery" >&2
      fi
      killed=1
      break
    fi
    sleep 1
  done
  [[ "$killed" == 1 ]] || { wait "$suite_pid"; cat "$suite_log"; die "never saw work in flight, so nothing was crashed mid-batch; the run proves nothing about the retry path"; }

  log "waiting for the worker to come back on its own"
  # Recovery has to be judged on whatever is actually serving the queue. Under a
  # GPU session the victim is a systemd unit and its container is stopped by
  # design, so inspecting the container reported "did not recover" and the
  # remedy below started a CPU worker beside the CUDA units -- a doubly-served
  # queue, which is B-1. The sampler caught the consequence: worker containers
  # 0 -> 1, then two disagreeing handles thirty seconds later, and the three
  # suites that followed failed their baselines on a fleet this script had split.
  local back=0 gpu_unit restarts_after
  gpu_unit="$(gpu_unit_for_container "$container")"
  if [[ -n "$gpu_unit" ]]; then
    for i in $(seq 1 60); do
      [[ "$(systemctl --user show "$gpu_unit" --property=ActiveState --value 2>/dev/null)" == active ]] && { back=1; break; }
      sleep 2
    done
    restarts_after="$(systemctl --user show "$gpu_unit" --property=NRestarts --value 2>/dev/null || echo 0)"
  else
    for i in $(seq 1 60); do
      [[ "$(docker inspect -f '{{.State.Status}}' "$container" 2>/dev/null)" == running ]] && { back=1; break; }
      sleep 2
    done
    restarts_after="$(docker inspect -f '{{.RestartCount}}' "$container" 2>/dev/null || echo 0)"
  fi
  if [[ "$back" == 1 && "${restarts_after:-0}" -gt "$restarts_before" ]]; then
    echo "  recovered automatically (restarts $restarts_before -> $restarts_after)"
  elif [[ "$back" == 1 ]]; then
    echo "  running again, but the restart count did not move ($restarts_after); it may not have actually died"
  elif [[ -n "$gpu_unit" ]]; then
    # Never start the container here: it would serve the same queue as the unit.
    die "$gpu_unit did not come back after the abort, and starting its container instead would split the fleet across backends; the GPU session needs repair (gpu-consensus-workers.sh restart-unit)"
  else
    echo "  did NOT recover automatically; starting it by hand so the suite can finish" >&2
    echo "  NOTE: this run says nothing about automatic recovery -- see L-5 in the Consensus Defect Log" >&2
    docker start "$container" >/dev/null 2>&1 || die "could not restart $container"
  fi

  log "waiting for the suite to finish"
  wait "$suite_pid"; local suite_status=$?
  cat "$suite_log"; rm -f "$suite_log"

  # Lock loss is expected here and is the evidence, not a fault: the crashed
  # worker's lease lapsed while it held work.
  "$SCRIPT_DIR/consensus-validity.sh" locks --report-only

  [[ "$suite_status" -eq 0 ]] || die "the crash-retry assertions failed"
  log "crash-retry: PASS"
}

main
