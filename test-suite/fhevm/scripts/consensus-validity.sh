#!/usr/bin/env bash
# Run-validity gates that need host access, and therefore cannot live in the
# e2e container.
#
# The rest of the gates (key/CRS material, the deferred-transactions gauge,
# chain liveness) run inside the suites, in
# `test-suite/e2e/test/consensus/validity.ts`. This file carries the ones only
# the host can see: whether any tfhe-worker lost dependence-chain locks during
# the run, and whether each operator's queue is served by exactly one worker.
#
#   consensus-validity.sh locks [--since <timestamp>] [--report-only]
#                               [--expect-loss] [--operators <n>]
#   consensus-validity.sh exclusivity [--operators <n>]
#
# Exit status: 0 clean, 1 the gate failed, 2 usage error, 3 INVALID -- the
# evidence could not be collected, which is not the same as clean and must not
# be reported as such.
#
# Why lock loss is a validity gate rather than a test. "Not all locks extended"
# means another worker stole a dependence chain whose lease had lapsed and
# recomputed it. RFC-020 says that must be byte-identical, so it is not
# automatically a consensus failure -- but it does mean the run was not the
# clean single-owner case it appears to be, and a measurement that silently
# included stolen work is not the measurement anyone thinks they are reading.
#
# Two things this gate used to get wrong, and both made it report clean when it
# could not see.
#
# It read `docker logs` unconditionally. A GPU session stops the worker
# containers on purpose and runs the workers as host units, so the gate was
# reading the last CPU worker's historical output while the units under test
# logged to the journal -- "no worker reported lock loss" about processes it had
# never looked at.
#
# And `count="$(docker logs ... | grep -c)"` turns a failed read into `0`. A
# missing container, a permission error and a genuinely quiet worker were the
# same answer.
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
# shellcheck source=lib/service-control.sh
source "${SCRIPT_DIR}/lib/service-control.sh"
sc_init || exit 1

readonly MESSAGE="Not all locks extended"
SINCE=""
REPORT_ONLY=0
EXPECT_LOSS=0
EXPECTED_OPERATORS=""

usage() {
  cat >&2 <<'EOF'
usage: consensus-validity.sh locks [--since <timestamp>] [--report-only] [--expect-loss] [--operators <n>]
       consensus-validity.sh exclusivity [--operators <n>]
EOF
  exit 2
}

invalid() {
  echo "validity: INVALID $*" >&2
  exit 3
}

# The tfhe-worker of every operator, as a container name. Under a GPU session
# `sc_*` resolves each of these to the unit actually serving that queue, so the
# same list works on both backends.
worker_targets() {
  docker ps -a --format '{{.Names}}' \
    | grep -E '^coprocessor[0-9]*-tfhe-worker$' \
    | sort
}

# ---------------------------------------------------------------------------
# locks
# ---------------------------------------------------------------------------

gate_locks() {
  local -a targets=()
  mapfile -t targets < <(worker_targets)
  if [[ "${#targets[@]}" -eq 0 ]]; then
    invalid "no tfhe-worker is present on this host, so the lock-extension gate has nothing to read"
  fi
  if [[ -n "$EXPECTED_OPERATORS" && "${#targets[@]}" -ne "$EXPECTED_OPERATORS" ]]; then
    invalid "expected $EXPECTED_OPERATORS tfhe-worker(s) but found ${#targets[@]} (${targets[*]}); a missing worker means the gate is not reading the fleet under test"
  fi

  # Default the window to this process's start rather than to all of history:
  # a lock loss from an earlier suite on the same stack is not this run's
  # evidence, and counting it contaminated every later clean run.
  local since="$SINCE"
  if [[ -z "$since" ]]; then
    since="${CONSENSUS_RUN_STARTED_AT:-}"
  fi
  if [[ -z "$since" ]]; then
    invalid "no run window given; pass --since or export CONSENSUS_RUN_STARTED_AT, or the gate would count lock losses from earlier suites on this stack as this run's"
  fi

  local target kind logs status count total=0 detail="" sources=""
  for target in "${targets[@]}"; do
    kind="$(sc_kind "$target")"
    sources="$sources $target($kind)"
    logs="$(sc_logs_since "$target" "$since" 2>/dev/null)"
    status=$?
    if [[ "$status" -ne 0 ]]; then
      invalid "could not read $kind logs for $target since $since (exit $status); an unreadable source is not a quiet worker"
    fi
    # A unit under a GPU session that has restarted has more than one
    # invocation in the window, and `sc_logs_since` deliberately reads the unit
    # rather than one invocation: the invocation a crash case killed is exactly
    # the one that would have reported the loss.
    count="$(grep -c -- "$MESSAGE" <<<"$logs")" || count=0
    if [[ "${count:-0}" -gt 0 ]]; then
      total=$((total + count))
      detail="$detail $target($kind x$count)"
    fi
  done

  echo "validity: read lock-extension evidence since $since from${sources}"

  if [[ "$EXPECT_LOSS" -eq 1 ]]; then
    if [[ "$total" -eq 0 ]]; then
      echo "validity: FAIL this case expected a lease to lapse and be stolen, and no worker reported one" >&2
      return 1
    fi
    echo "validity: lock loss OBSERVED and attributed, as this case requires:$detail"
    return 0
  fi

  if [[ "$total" -eq 0 ]]; then
    echo "validity: no tfhe-worker reported lock loss in the run window"
    return 0
  fi
  echo "validity: LOCK LOSS reported:$detail"
  echo "  Another worker stole a dependence chain whose lease lapsed and recomputed it. Bytes must"
  echo "  still match (RFC-020), but this run is not the clean single-owner case it looks like."
  [[ "$REPORT_ONLY" -eq 1 ]] && return 0
  return 1
}

# ---------------------------------------------------------------------------
# exclusivity
# ---------------------------------------------------------------------------

# One worker per queue, judged by the QUEUE rather than by the executable name.
#
# A process called `tfhe_worker` on an unrelated database is not a conflict, and
# a second worker on the same database is one whatever it is called. So the
# check is: for each operator, how many live things are configured against that
# operator's database.
gate_exclusivity() {
  bun "$SCRIPT_DIR/queue-ownership.ts" "$EXPECTED_OPERATORS"
}

[[ $# -ge 1 ]] || usage
GATE="$1"; shift
while [[ $# -gt 0 ]]; do
  case "$1" in
    --since) SINCE="${2:?--since needs a value}"; shift 2 ;;
    --report-only) REPORT_ONLY=1; shift ;;
    --expect-loss) EXPECT_LOSS=1; shift ;;
    --operators) EXPECTED_OPERATORS="${2:?--operators needs a count}"; shift 2 ;;
    *) usage ;;
  esac
done

if [[ -f "${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/failure-matrix/uncancelled-phase" ]]; then
  invalid "a previous test process could not be stopped; recover its recorded faults before running another suite"
fi

case "$GATE" in
  locks) gate_locks ;;
  exclusivity) gate_exclusivity ;;
  *) usage ;;
esac
