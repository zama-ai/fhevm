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

# The launcher's own command is not the only thing that must refuse. A run
# starts through the readiness path, and B-1 got as far as producing disagreeing
# ct128 because nothing on that path objected. Asserted while the split is still
# in place, which is the only moment the answer means anything.
if "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$(operator_count)" >/dev/null 2>&1; then
  die "the readiness exclusivity gate reported one worker per queue while $container and $unit both served operator $OPERATOR; only the launcher command noticed, so an ordinary run would proceed into a split fleet"
fi
note "the readiness exclusivity gate refuses the split too, not only the launcher command"

restore || die "could not restore the single-owner queue"
sleep 2
"$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1 ||
  die "conflicts still reports a doubly-served queue after the CPU container was stopped; the guard latches instead of tracking state"
note "guard clears once the queue has one worker again"
note "PASS: the mixed-backend guard fires on a split and clears on repair"
