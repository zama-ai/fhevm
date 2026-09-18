#!/usr/bin/env bash
# Sensitivity controls: prove the suite notices when the decisive behaviour is
# removed.
#
# A repaired false-green is only repaired if its repair is falsifiable. Most of
# the classes are covered by contract tests that inject the failure directly --
# `test-fault-contracts.sh` for a no-op injection and an unreadable log source,
# `comparator.test.ts` for absent SNS evidence and every other mismatch class.
# The controls here are the ones that need a live stack or a production
# mutation, and each of them REQUIRES the corresponding case to fail.
#
# Every control restores what it changed. A control that cannot restore says so
# and exits non-zero rather than leaving the tree or the stack modified.
#
#   consensus-sensitivity.sh --control <name>|all [--list]
#
# Controls:
#   no-fault-observed   the crash case cannot catch the victim holding its
#                       target work -> INVALID, never PASS
#   no-live-target      a fault aimed at an already-stopped service -> INVALID
#   listener-no-rebind  revert the pool rebind -> REG-01 fails
#   listener-no-reconcile
#                       revert the startup reconcile -> REG-01 fails
#   daemon-exit-zero    revert exit-for-restart -> REG-02 fails
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENGINE_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"

CONTROL=""
LIST=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --control) CONTROL="${2:?--control needs a name}"; shift 2 ;;
    --list) LIST=1; shift ;;
    *) echo "usage: consensus-sensitivity.sh --control <name>|all [--list]" >&2; exit 2 ;;
  esac
done

CONTROLS=(no-fault-observed no-live-target listener-no-rebind listener-no-reconcile daemon-exit-zero)
if [[ "$LIST" == 1 ]]; then
  printf '%s\n' "${CONTROLS[@]}"
  exit 0
fi
[[ -n "$CONTROL" ]] || { echo "--control is required (or --list)" >&2; exit 2; }

source "$SCRIPT_DIR/lib/service-control.sh"
sc_init || exit 1
MUTATED_FILES=()
SENSITIVITY_CHILD_PID=""
sensitivity_capture() {
  local -n captured="$1"; shift
  local output status=0
  output="$(mktemp)" || return 1
  "$@" >"$output" 2>&1 &
  SENSITIVITY_CHILD_PID=$!
  wait "$SENSITIVITY_CHILD_PID" || status=$?
  SENSITIVITY_CHILD_PID=""
  captured="$(cat "$output")"
  rm -f "$output"
  return "$status"
}
sensitivity_cleanup() {
  local status=$? file
  hc_cleanup_signals
  hc_begin_cleanup || exit 1
  if [[ -n "$SENSITIVITY_CHILD_PID" ]]; then
    kill -TERM "$SENSITIVITY_CHILD_PID" 2>/dev/null || true
    # Give the delegated runner its own cancellation/SQL recovery before healing
    # this control. If it cannot finish, retain ownership and fail closed.
    if ! hc_run tail --pid="$SENSITIVITY_CHILD_PID" -f /dev/null; then
      echo "sensitivity: delegated recovery still running; retaining $SC_RESTORE_LOG" >&2
      exit 1
    fi
    wait "$SENSITIVITY_CHILD_PID" 2>/dev/null || true
    SENSITIVITY_CHILD_PID=""
  fi
  sc_run_restores || status=1
  for file in "${MUTATED_FILES[@]}"; do restore_tree "$file" || status=1; done
  if [[ "$status" == 0 ]]; then rm -f "$SC_RESTORE_LOG"; fi
  exit "$status"
}
trap sensitivity_cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

FAILURES=0
log() { printf '\n=== %s\n' "$*"; }
report() {
  local name="$1" expected="$2" actual="$3" detail="$4"
  if [[ "$expected" == "$actual" ]]; then
    printf '  ok    %s: %s\n' "$name" "$detail"
  else
    printf '  FAIL  %s: expected %s, got %s (%s)\n' "$name" "$expected" "$actual" "$detail" >&2
    FAILURES=$((FAILURES + 1))
  fi
}

# The tree must be clean before a production mutation, or the restore cannot be
# distinguished from the developer's own edits.
require_clean_tree() {
  local dirty
  dirty="$(git -C "$REPO_ROOT" status --porcelain -- "$1")" || return 1
  [[ -z "$dirty" ]] || {
    echo "refusing to mutate $1: it already has uncommitted changes" >&2
    exit 1
  }
}

restore_tree() {
  git -C "$REPO_ROOT" checkout -- "$1" || return 1
  local dirty
  dirty="$(git -C "$REPO_ROOT" status --porcelain -- "$1")" || return 1
  [[ -z "$dirty" ]] || {
    echo "FAILED TO RESTORE $1; the tree is left modified" >&2
    exit 1
  }
  echo "  restored $1"
}

# --------------------------------------------------------------------------
# Stack controls
# --------------------------------------------------------------------------

# The crash case must not report PASS when it never caught the victim holding
# its target work. A one-second acquisition budget cannot, by construction.
# The recorded state of one case, read from a results directory. Empty when the
# case recorded nothing at all -- which is itself a failure of these controls.
record_field() {
  local dir="$1" case_id="$2" field="$3"
  python3 - "$dir" "$case_id" "$field" <<'PY'
import json, os, sys

directory, case_id, field = sys.argv[1:4]
value = ""
for name in sorted(os.listdir(directory)) if os.path.isdir(directory) else []:
    if not name.endswith(".jsonl"):
        continue
    with open(os.path.join(directory, name)) as handle:
        for line in handle:
            line = line.strip()
            if not line:
                continue
            try:
                record = json.loads(line)
            except json.JSONDecodeError:
                continue
            if record.get("caseId") == case_id:
                value = str(record.get(field) or "")
print(value)
PY
}

control_no_fault_observed() {
  log "no-fault-observed: the crash case cannot observe its own precondition"
  # The window has to be one the observation cannot win. A one-second window is
  # not that: by the time the runner reaches the acquisition poll the workload
  # has been flowing for half a minute, so the very first observation catches
  # the victim holding a chain and the case passes for real. Zero is the only
  # window that makes the precondition unobservable without touching the
  # production path -- which is exactly the state this control is about.
  #
  # The control's own records go to their own directory. `readCaseResults` on a
  # directory reads every file in it, so a control that wrote an INVALID beside
  # a run's results would turn that run's aggregate into a conflict.
  local results_dir out status=0
  results_dir="$(mktemp -d)"
  sensitivity_capture out env "CONSENSUS_RESULTS_DIR=$results_dir" CRASH_ACQUIRE_TIMEOUT=0 \
    "$SCRIPT_DIR/run-crash-retry-consensus.sh" --victim 1 --boundary before-commit || status=$?

  # The state is read from the record, not from stdout: refusing to claim an
  # outcome means saying INVALID where the aggregate reads it, and a runner that
  # only printed its refusal would still let a false green through the gate.
  local state detail
  state="$(record_field "$results_dir" CR-01-INTERRUPT-BEFORE-COMMIT state)"
  detail="$(record_field "$results_dir" CR-01-INTERRUPT-BEFORE-COMMIT detail)"
  rm -rf "$results_dir"

  report no-fault-observed INVALID "${state:-none}" "runner exited $status"
  if grep -q "never observed operator" <<<"$detail"; then
    printf '  ok    no-fault-observed: recorded why -- %s\n' "$detail"
  else
    report no-fault-observed "an explanatory reason" "${detail:-none}" \
      "the record does not say the victim was never caught holding the work"
  fi
}

# A fault aimed at a service that is already stopped proves nothing, and the
# matrix must refuse rather than heal it and pass.
control_no_live_target() {
  log "no-live-target: a fault aimed at an already-stopped service"
  sc_stop coprocessor1-zkproof-worker || {
    echo "cannot establish the stopped-target control with recoverable ownership" >&2
    return 1
  }
  # As above: the control's own record goes to its own directory, so an INVALID
  # produced on purpose never lands beside a run's results.
  local results_dir out status=0
  results_dir="$(mktemp -d)"
  sensitivity_capture out env "CONSENSUS_RESULTS_DIR=$results_dir" \
    "$SCRIPT_DIR/run-failure-matrix.sh" --case FM-ZKPROOF-CRASH --victim 1 || status=$?
  sc_run_restores || {
    echo "no-live-target: restoration failed; refusing further controls" >&2
    return 1
  }
  local state detail
  state="$(record_field "$results_dir" FM-ZKPROOF-CRASH state)"
  detail="$(record_field "$results_dir" FM-ZKPROOF-CRASH detail)"
  rm -rf "$results_dir"
  report no-live-target INVALID "${state:-none}" "runner exited $status"
  if [[ "$status" != 0 && "$detail" == 'coprocessor1-zkproof-worker was not live before the fault (state=stopped)' ]]; then
    printf '  ok    no-live-target: recorded why -- %s\n' "$detail"
  else
    report no-live-target "the exact stopped-target precondition" "${detail:-none}" \
      "the record does not say the target was already stopped"
  fi
}

# --------------------------------------------------------------------------
# Production mutations
# --------------------------------------------------------------------------

run_rust_test() {
  local package="$1" test_name="$2"
  local output status=0
  output="$(cd "$ENGINE_DIR" && SQLX_OFFLINE=true cargo test -p "$package" --test "$test_name" -- --test-threads=1 2>&1)" || status=$?
  [[ "$status" -eq 0 ]] && return 0
  # Compilation/tooling failures are not evidence that a regression was detected.
  if grep -q '^test result: FAILED' <<<"$output"; then
    echo "$output" | tail -25
    return 1
  fi
  echo "$output" >&2
  return 2
}

control_listener_no_rebind() {
  log "listener-no-rebind: the supervisor keeps the pool it captured once"
  local file="coprocessor/fhevm-engine/host-listener/src/database/tfhe_event_propagate.rs"
  require_clean_tree "$file" || return 1
  MUTATED_FILES+=("$file")
  python3 - "$REPO_ROOT/$file" <<'PY'
import sys
path = sys.argv[1]
text = open(path).read()
old = """        while !cancel.is_cancelled() {
            let pool = db.pool().await;"""
new = """        let pool = db.pool().await;
        while !cancel.is_cancelled() {
            let pool = pool.clone();"""
assert old in text, "the rebind shape changed; update this control"
open(path, "w").write(text.replace(old, new, 1))
PY
  if [[ $? -ne 0 ]]; then
    report listener-no-rebind "mutation applied" "mutation failed" "update the control for the current source"
    return 1
  fi
  local status=0
  run_rust_test host-listener stack_version_listener_tests || status=$?
  report listener-no-rebind "1" "$status" \
    "REG-01 must fail without the rebind (cargo exited $status)"
  restore_tree "$file" || return 1
  MUTATED_FILES=()
}

control_listener_no_reconcile() {
  log "listener-no-reconcile: no reconcile after re-subscribing"
  local file="coprocessor/fhevm-engine/fhevm-engine-common/src/versioning.rs"
  require_clean_tree "$file" || return 1
  MUTATED_FILES+=("$file")
  python3 - "$REPO_ROOT/$file" <<'PY'
import sys
path = sys.argv[1]
text = open(path).read()
old = """                reconcile_stack_mode(&pool, &mode).await?;
"""
assert old in text, "the startup reconcile shape changed; update this control"
open(path, "w").write(text.replace(old, "", 1))
PY
  if [[ $? -ne 0 ]]; then
    report listener-no-reconcile "mutation applied" "mutation failed" "update the control for the current source"
    return 1
  fi
  local status=0
  run_rust_test host-listener stack_version_listener_tests || status=$?
  report listener-no-reconcile "1" "$status" \
    "REG-01 must fail without the startup reconcile (cargo exited $status)"
  restore_tree "$file" || return 1
  MUTATED_FILES=()
}

control_daemon_exit_zero() {
  log "daemon-exit-zero: the daemon logs its fatal error and exits 0"
  local file="coprocessor/fhevm-engine/tfhe-worker/src/lib.rs"
  require_clean_tree "$file" || return 1
  MUTATED_FILES+=("$file")
  python3 - "$REPO_ROOT/$file" <<'PY'
import sys
path = sys.argv[1]
text = open(path).read()
old = """    if fatal {
        std::process::exit(1);
    }"""
assert old in text, "the exit-for-restart shape changed; update this control"
open(path, "w").write(text.replace(old, "    let _ = fatal;", 1))
PY
  if [[ $? -ne 0 ]]; then
    report daemon-exit-zero "mutation applied" "mutation failed" "update the control for the current source"
    return 1
  fi
  local status=0
  run_rust_test tfhe-worker daemon_exit || status=$?
  report daemon-exit-zero "1" "$status" \
    "REG-02 must fail without exit-for-restart (cargo exited $status)"
  restore_tree "$file" || return 1
  MUTATED_FILES=()
}

run_control() {
  case "$1" in
    no-fault-observed) control_no_fault_observed ;;
    no-live-target) control_no_live_target ;;
    listener-no-rebind) control_listener_no_rebind ;;
    listener-no-reconcile) control_listener_no_reconcile ;;
    daemon-exit-zero) control_daemon_exit_zero ;;
    *) echo "unknown control $1 (see --list)" >&2; exit 2 ;;
  esac
}

if [[ "$CONTROL" == all ]]; then
  for name in "${CONTROLS[@]}"; do run_control "$name" || exit 1; done
else
  run_control "$CONTROL" || exit 1
fi

log "sensitivity"
if [[ "$FAILURES" -eq 0 ]]; then
  echo "every control behaved as required: removing the decisive behaviour is detected"
  exit 0
fi
echo "$FAILURES control(s) did NOT detect the removal; the corresponding repair is not falsifiable" >&2
exit 1
