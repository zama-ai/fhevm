#!/usr/bin/env bash
# Assert the guards that stop a fleet from splitting across squash backends.
#
# B-1 is the one consensus failure this project caused itself: a CPU worker
# container and a CUDA host unit both served one operator's queue, claiming rows
# with FOR UPDATE SKIP LOCKED, and produced different-but-valid ct128 for the same
# input. Twenty-four handles disagreed and the cause took an intervention
# experiment in both directions to establish.
#
# Guards were added afterwards, but nothing ever proved they fire. The coverage
# inventory scoped the residual ask as "worth one cell in the matrix, not a
# topology", and going to GPU is when it becomes cheap: the split is one
# `docker start` away. This creates it deliberately, asserts both guards notice,
# and puts the queue back.
#
# Usage: scripts/run-mixed-backend-guard.sh [--operator N]
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
sc_init || exit 1
OPERATOR=1

while [[ $# -gt 0 ]]; do
  case "$1" in
    --operator) OPERATOR="${2:?--operator needs an index}"; shift 2 ;;
    *) echo "usage: run-mixed-backend-guard.sh [--operator N]" >&2; exit 2 ;;
  esac
done

die()  { echo "mixed-backend-guard: $*" >&2; exit 1; }
note() { echo "mixed-backend-guard: $*"; }

# How many operators this stack actually has, read from the generated
# environment rather than assumed, so the exclusivity gate is asked about the
# fleet that exists.
operator_count() {
  local n=0 path env_dir
  env_dir="${FHEVM_STATE_DIR:-$(cd -- "${SCRIPT_DIR}/../../.." && pwd)/.fhevm}/runtime/env"
  for path in "$env_dir"/coprocessor.env "$env_dir"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

runtime="/run/user/$(id -u)"
[[ -d "$runtime" ]] && {
  export XDG_RUNTIME_DIR="$runtime"
  export DBUS_SESSION_BUS_ADDRESS="unix:path=${runtime}/bus"
}

if [[ "$OPERATOR" == 0 ]]; then
  container="coprocessor-tfhe-worker"
else
  container="coprocessor${OPERATOR}-tfhe-worker"
fi
unit="fhevm-gpu-consensus-tfhe-${OPERATOR}"

[[ "$(systemctl --user show "$unit" --property=ActiveState --value 2>/dev/null)" == active ]] ||
  die "$unit is not active; this gate needs GPU host workers serving the queues (run gpu-consensus-workers.sh start first)"

# Baseline: the guard must be quiet before the split, or a later non-zero exit
# proves nothing about the split we are about to create.
"$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1 ||
  die "conflicts already reports a doubly-served queue before the split was created; fix the stack first"
note "baseline clean: no queue served twice"

restore() {
  sc_run_restores || { echo "mixed-backend-guard: cleanup failed; owners retained in $SC_RESTORE_LOG" >&2; return 1; }
  note "put the queue back to one worker"
}
cleanup_guard() {
  local status=$?
  hc_cleanup_signals
  hc_begin_cleanup || exit 1
  restore || status=1
  if [[ "$status" == 0 && "$SC_RESTORE_LOG_OWNED" == 1 ]]; then rm -f "$SC_RESTORE_LOG"; fi
  exit "$status"
}
trap cleanup_guard EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

# Keep the original CPU container stopped again, even if start partially acts
# and fails or the guard process is interrupted before it observes the split.
sc_register_restore "$container" stop-container || die "cannot record removal of the introduced worker"
note "starting $container alongside $unit -- deliberately serving one queue twice"
docker start "$container" >/dev/null 2>&1 || die "could not start $container"
# The lifecycle recorder reads this stamp: the fault exists from this moment,
# not from whenever the parent gets around to writing the record.
printf 'GPU_FAULT_OBSERVED_AT=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

# The guard polls container state, so give it a moment to observe the split
# rather than racing it and reporting a pass for a split that had not landed.
#
# Only the launcher's own conflict line proves detection. A transient
# `systemctl show` or `docker inspect` failure also exits non-zero and also
# mentions workers and queues, so neither the exit status nor a loose keyword
# match may stand in for the named queue.
conflict_line="CONFLICT operator=${OPERATOR} kind=tfhe unit=${unit} container=${container}:"
detected=0
output=""
# Contract tests shorten the window; production keeps thirty seconds.
poll_attempts="${MIXED_GUARD_POLL_ATTEMPTS:-15}"
poll_seconds="${MIXED_GUARD_POLL_SECONDS:-2}"
for _ in $(seq 1 "$poll_attempts"); do
  if output="$("$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts 2>&1)"; then
    sleep "$poll_seconds"
    continue
  fi
  if grep -qF -- "$conflict_line" <<<"$output"; then
    detected=1
    break
  fi
  note "conflicts exited non-zero without naming the split yet: $(tail -1 <<<"$output")"
  sleep "$poll_seconds"
done

if [[ "$detected" != 1 ]]; then
  die "conflicts never printed '$conflict_line' while $container and $unit both served operator $OPERATOR's queue; the guard did not fire (last output: $(tail -3 <<<"$output" | tr '\n' ' ')), so B-1 can recur undetected"
fi
note "guard fired and named the conflict:"
sed 's/^/    /' <<<"$output"

# The launcher's own command is not the only thing that must refuse. A run
# starts through the readiness path, and B-1 got as far as producing disagreeing
# ct128 because nothing on that path objected. Asserted while the split is still
# in place, which is the only moment the answer means anything.
# The same rule applies here: the gate must refuse BECAUSE of this queue. An
# inspection error is also a non-zero exit and would otherwise count as refusal.
refusal_line="${container}: both Docker and GPU unit serve its queue"
if exclusivity_output="$("$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$(operator_count)" 2>&1)"; then
  die "the readiness exclusivity gate reported one worker per queue while $container and $unit both served operator $OPERATOR; only the launcher command noticed, so an ordinary run would proceed into a split fleet"
fi
grep -qF -- "$refusal_line" <<<"$exclusivity_output" ||
  die "the readiness exclusivity gate exited non-zero without naming the doubly-served queue ('$refusal_line'); that is an inspection failure, not a refusal: $(tail -3 <<<"$exclusivity_output" | tr '\n' ' ')"
note "the readiness exclusivity gate refuses the split too, not only the launcher command"

restore || die "could not restore the single-owner queue"
sleep 2
"$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1 ||
  die "conflicts still reports a doubly-served queue after the CPU container was stopped; the guard latches instead of tracking state"
note "guard clears once the queue has one worker again"
note "PASS: the mixed-backend guard fires on a split and clears on repair"
