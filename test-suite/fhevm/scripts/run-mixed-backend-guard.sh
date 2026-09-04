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
# register scoped the residual ask as "worth one cell in the matrix, not a
# topology", and going to GPU is when it becomes cheap: the split is one
# `docker start` away. This creates it deliberately, asserts both guards notice,
# and puts the queue back.
#
# Usage: scripts/run-mixed-backend-guard.sh [--operator N]
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OPERATOR=1

while [[ $# -gt 0 ]]; do
  case "$1" in
    --operator) OPERATOR="${2:?--operator needs an index}"; shift 2 ;;
    *) echo "usage: run-mixed-backend-guard.sh [--operator N]" >&2; exit 2 ;;
  esac
done

die()  { echo "mixed-backend-guard: $*" >&2; exit 1; }
note() { echo "mixed-backend-guard: $*"; }

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

restored=0
restore() {
  [[ "$restored" == 1 ]] && return 0
  restored=1
  docker stop "$container" >/dev/null 2>&1 || true
  note "put the queue back to one worker"
}
trap restore EXIT

note "starting $container alongside $unit -- deliberately serving one queue twice"
docker start "$container" >/dev/null 2>&1 || die "could not start $container"

# The guard polls container state, so give it a moment to observe the split
# rather than racing it and reporting a pass for a split that had not landed.
detected=0
for _ in $(seq 1 15); do
  if ! output="$("$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts 2>&1)"; then
    detected=1
    break
  fi
  sleep 2
done

if [[ "$detected" != 1 ]]; then
  die "conflicts exited 0 while $container and $unit both served operator $OPERATOR's queue; the guard did not fire, so B-1 can recur undetected"
fi

grep -qiE "tfhe|worker|queue|operator" <<<"$output" ||
  die "conflicts failed but did not name the doubly-served queue, so an operator cannot tell what to fix: $output"
note "guard fired and named the conflict:"
sed 's/^/    /' <<<"$output"

restore
sleep 2
"$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1 ||
  die "conflicts still reports a doubly-served queue after the CPU container was stopped; the guard latches instead of tracking state"
note "guard clears once the queue has one worker again"
note "PASS: the mixed-backend guard fires on a split and clears on repair"
