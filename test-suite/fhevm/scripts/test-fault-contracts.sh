#!/usr/bin/env bash
# Contract tests for the fault-control layer (inventory case
# HAR-01-FAULT-CONTRACTS).
#
# These do not need a stack. They need the opposite: a fake `docker` whose
# answers the test controls, so each fail-closed rule can be driven directly.
# The rules matter because the previous fault injection had no such tests and
# two of its cells were silent no-ops for weeks -- `fault_pause` called a
# helper that did not exist, the failure was swallowed by a `2>/dev/null`, and
# the cells reported PASS against a worker nothing had touched.
#
# Every case here asserts a NON-pass outcome. That is the direction that goes
# wrong quietly: a fault that fails to land looks exactly like a fault the
# system survived.
#
#   scripts/test-fault-contracts.sh
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"

# A private state directory, marked as a GPU session, so the unit-backed paths
# are reachable without a GPU. `gpu-session.sh` resolves the marker path when it
# is sourced, so this has to be set first.
FAKE_STATE_DIR="$(mktemp -d)"
export FHEVM_STATE_DIR="$FAKE_STATE_DIR"
mkdir -p "$FHEVM_STATE_DIR/runtime/gpu-consensus-workers"
: >"$FHEVM_STATE_DIR/runtime/gpu-consensus-workers/node-config.env"

FAILURES=0
CASES=0

check() {
  local what="$1" expected="$2" actual="$3"
  CASES=$((CASES + 1))
  if [[ "$expected" == "$actual" ]]; then
    printf '  ok    %s\n' "$what"
  else
    printf '  FAIL  %s (expected %s, got %s)\n' "$what" "$expected" "$actual" >&2
    FAILURES=$((FAILURES + 1))
  fi
}

# --------------------------------------------------------------------------
# A fake docker, whose per-container state the test sets through files.
#
# Only the subcommands service-control.sh uses are implemented, and anything
# else is an error rather than a silent success: a helper that grew a new
# docker call would otherwise pass these tests without being covered.
# --------------------------------------------------------------------------
FAKE_BIN="$(mktemp -d)"
FAKE_STATE="$(mktemp -d)"
cat >"$FAKE_BIN/docker" <<'FAKE'
#!/usr/bin/env bash
state_file="$FAKE_STATE/state"
touch "$state_file"
get() { sed -n "s/^$1=//p" "$state_file" | tail -1; }
set_state() { sed -i "/^$1=/d" "$state_file"; printf '%s=%s\n' "$1" "$2" >>"$state_file"; }

case "$1" in
  inspect)
    shift
    format=""
    args=()
    while [[ $# -gt 0 ]]; do
      case "$1" in
        -f|--format) format="$2"; shift 2 ;;
        *) args+=("$1"); shift ;;
      esac
    done
    name="${args[0]:-}"
    status="$(get "${name}.status")"
    [[ -n "$status" ]] || exit 1
    case "$format" in
      '{{.State.Status}}') echo "$status" ;;
      '{{.State.Pid}}') echo "$(get "${name}.pid")" ;;
      '{{.State.StartedAt}}') echo "$(get "${name}.started")" ;;
      '{{.RestartCount}}') echo "$(get "${name}.restarts")" ;;
      '') echo "{}" ;;
      *) echo "" ;;
    esac
    ;;
  stop) set_state "$2.status" exited ;;
  start) set_state "$2.status" running ;;
  pause) if [[ "$(get "$2.status")" == running ]]; then set_state "$2.status" paused; else exit 1; fi ;;
  unpause) if [[ "$(get "$2.status")" == paused ]]; then set_state "$2.status" running; else exit 1; fi ;;
  ps) echo "" ;;
  logs) echo "" ;;
  *) echo "fake docker: unhandled subcommand $1" >&2; exit 97 ;;
esac
FAKE
chmod +x "$FAKE_BIN/docker"
export FAKE_STATE
export PATH="$FAKE_BIN:$PATH"

set_container() {
  local name="$1" status="$2" pid="${3:-0}"
  sed -i "/^${name}\./d" "$FAKE_STATE/state" 2>/dev/null || true
  {
    printf '%s.status=%s\n' "$name" "$status"
    printf '%s.pid=%s\n' "$name" "$pid"
    printf '%s.started=%s\n' "$name" "2026-09-09T00:00:00Z"
    printf '%s.restarts=0\n' "$name"
  } >>"$FAKE_STATE/state"
}

# shellcheck source=lib/service-control.sh
source "${SCRIPT_DIR}/lib/service-control.sh"
# shellcheck source=lib/case-result.sh
source "${SCRIPT_DIR}/lib/case-result.sh"
sc_init

echo "service-control fail-closed contracts"

# A target that is not there cannot carry a fault.
: >"$FAKE_STATE/state"
sc_require_faultable ghost-container >/dev/null 2>&1
check "an absent target is not faultable" 1 $?

# Neither can one that is already stopped: injecting there proves nothing, and
# the cell would report on a service that was never running.
set_container stopped-worker exited 0
sc_require_faultable stopped-worker >/dev/null 2>&1
check "a stopped target is not faultable" 1 $?

# Nor one that is already paused.
set_container paused-worker paused 4242
sc_require_faultable paused-worker >/dev/null 2>&1
check "an already-paused target is not faultable" 1 $?

# A running one is.
set_container live-worker running "$$"
sc_require_faultable live-worker >/dev/null 2>&1
check "a running target is faultable" 0 $?

echo
echo "GPU unit contracts (the path whose stall injection was a silent no-op)"

# A fake systemctl, so the unit-backed paths can be driven without a GPU. Only
# the properties service-control.sh reads are answered.
cat >"$FAKE_BIN/systemctl" <<'FAKESYSTEMCTL'
#!/usr/bin/env bash
state_file="$FAKE_STATE/units"
touch "$state_file"
get() { sed -n "s/^$1=//p" "$state_file" | tail -1; }
# systemctl --user show <unit> --property=X --value
unit=""
property=""
for arg in "$@"; do
  case "$arg" in
    --property=*) property="${arg#--property=}" ;;
    --user|--value|show|is-active) ;;
    *) [[ -z "$unit" ]] && unit="$arg" ;;
  esac
done
value="$(get "${unit}.${property}")"
[[ -n "$value" ]] || exit 1
echo "$value"
FAKESYSTEMCTL
chmod +x "$FAKE_BIN/systemctl"
: >"$FAKE_STATE/units"

set_unit() {
  local unit="$1" state="$2" pid="$3"
  sed -i "/^${unit}\./d" "$FAKE_STATE/units" 2>/dev/null || true
  {
    printf '%s.ActiveState=%s\n' "$unit" "$state"
    printf '%s.MainPID=%s\n' "$unit" "$pid"
    printf '%s.InvocationID=%s\n' "$unit" "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    printf '%s.NRestarts=0\n' "$unit"
  } >>"$FAKE_STATE/units"
}

# The GPU session marker makes a worker container resolve to its unit, which is
# what the failure matrix must fault instead of the container the swap stopped.
check "a worker container resolves to its GPU unit under a session" unit "$(sc_kind coprocessor1-tfhe-worker)"
check "a non-worker container stays a container" container "$(sc_kind coprocessor1-gw-listener)"

# An active unit with MainPID 0 cannot be stalled, and the attempt must fail
# rather than return success having signalled nothing. This is the exact defect:
# the old code called a helper that did not exist, discarded the error, and the
# cell passed against an uninterrupted worker.
set_unit fhevm-gpu-consensus-tfhe-1 active 0
: >"$SC_RESTORE_LOG"
sc_pause coprocessor1-tfhe-worker >/dev/null 2>&1
check "stalling a unit with no MainPID fails" 1 $?
check "a stall refused before mutation registers no restore" 0 "$(wc -l <"$SC_RESTORE_LOG" | tr -d ' ')"

# An inactive unit is not faultable either, however healthy its container looks:
# a session that stopped the unit must not be faulted through the container it
# displaced.
set_unit fhevm-gpu-consensus-tfhe-2 inactive 0
set_container coprocessor2-tfhe-worker running "$$"
sc_require_faultable coprocessor2-tfhe-worker >/dev/null 2>&1
check "an inactive unit is not faultable even with a running container" 1 $?

# And the real thing: a live process is genuinely stopped and genuinely
# resumed, verified against the kernel rather than against the command status.
sleep 600 &
VICTIM_PID=$!
set_unit fhevm-gpu-consensus-tfhe-0 active "$VICTIM_PID"
: >"$SC_RESTORE_LOG"
sc_pause coprocessor-tfhe-worker >/dev/null 2>&1
check "a unit with a live MainPID is stalled" 0 $?
check "the stalled process is observed stopped by the kernel" T "$(sc_proc_state "$VICTIM_PID")"
check "a successful stall registers its restore" 1 "$(wc -l <"$SC_RESTORE_LOG" | tr -d ' ')"
sc_resume coprocessor-tfhe-worker >/dev/null 2>&1
check "the stalled process is resumed" 0 $?
check "the resumed process is no longer stopped" "" "$(sc_proc_state "$VICTIM_PID" | grep -x T)"
check "a completed restore is deregistered" 0 "$(wc -l <"$SC_RESTORE_LOG" | tr -d ' ')"
kill -9 "$VICTIM_PID" 2>/dev/null || true

echo
echo "container fail-closed contracts"

# A pause that the fake accepts but that does not change the observed state must
# still fail: the postcondition is read back, not assumed from the exit status.
cat >"$FAKE_BIN/docker" <<'FAKE'
#!/usr/bin/env bash
# `pause` reports success and changes nothing, which is the shape of a no-op
# injection.
case "$1" in
  inspect)
    shift; format=""
    while [[ $# -gt 0 ]]; do case "$1" in -f|--format) format="$2"; shift 2 ;; *) name="$1"; shift ;; esac; done
    case "$format" in
      '{{.State.Status}}') echo running ;;
      '{{.State.Pid}}') echo 0 ;;
      *) echo "" ;;
    esac
    ;;
  pause) exit 0 ;;
  stop|start|unpause) exit 0 ;;
  *) exit 0 ;;
esac
FAKE
chmod +x "$FAKE_BIN/docker"
SC_DEFAULT_TIMEOUT=2
sc_pause silently-ignored >/dev/null 2>&1
check "a pause that changes nothing fails on its postcondition" 1 $?

# A stop that does not stop anything must fail too.
sc_stop silently-ignored >/dev/null 2>&1
check "a stop that changes nothing fails on its postcondition" 1 $?

# And a process that was never replaced must not read as recovered.
sc_wait_replaced silently-ignored "container=silently-ignored pid=0 started= restarts=" 2 >/dev/null 2>&1
check "an unchanged process identity is not a recovery" 1 $?

# A cleanup that cannot complete has to be reported, not swallowed: the next
# case on the same stack inherits whatever was left behind.
cat >"$FAKE_BIN/docker" <<'FAKE'
#!/usr/bin/env bash
case "$1" in
  inspect)
    shift; format=""
    while [[ $# -gt 0 ]]; do case "$1" in -f|--format) format="$2"; shift 2 ;; *) shift ;; esac; done
    case "$format" in
      '{{.State.Status}}') echo exited ;;
      '{{.State.Pid}}') echo 0 ;;
      *) echo "" ;;
    esac
    ;;
  start) exit 1 ;;
  *) exit 0 ;;
esac
FAKE
chmod +x "$FAKE_BIN/docker"
: >"$SC_RESTORE_LOG"
sc_register_restore wont-start start
sc_run_restores >/dev/null 2>&1
check "a failed restore reports cleanup failure" 1 $?
check "a failed restore records contamination" 1 "${SC_CLEANUP_FAILED:-0}"
check "a failed restore retains its target" 1 "$(wc -l <"$SC_RESTORE_LOG" | tr -d ' ')"
sc_run_restores >/dev/null 2>&1
check "a repeated failed restore cannot become an empty success" 1 $?
# The same saved ownership is sufficient to retry once Docker recovers.
sed -i 's/echo exited/echo running/;s/echo 0/echo 123/;s/start) exit 1/start) exit 0/' "$FAKE_BIN/docker"
sc_run_restores >/dev/null 2>&1
check "a recovered Docker can retry the saved restore" 0 $?
check "only the successful retry removes its record" 0 "$(wc -l <"$SC_RESTORE_LOG" | tr -d ' ')"


echo
echo "suite-verdict contracts"

# `1 passing` is not a verdict. An after-hook that fails after a test body
# passes leaves the string in the output and the process exiting non-zero, and
# the runners used to grep for the string.
out=""
cr_run_suite out "" bash -c 'echo "  1 passing"; exit 1' >/dev/null 2>&1
check "a non-zero exit fails even with '1 passing' in the output" 1 $?

# A suite that exits 0 without running its case must not pass either.
cr_run_suite out "MARKER" bash -c 'echo "  1 passing"; exit 0' >/dev/null 2>&1
check "a missing completion marker fails" 3 $?

# A suite that skipped everything exits 0 too, and mocha says so.
cr_run_suite out "" bash -c 'echo "  0 passing"; echo "  1 pending"; exit 0' >/dev/null 2>&1
check "an all-pending suite is not a pass" 4 $?

# And the happy path is still a pass.
cr_run_suite out "MARKER" bash -c 'echo "  1 passing"; echo MARKER; exit 0' >/dev/null 2>&1
check "a suite that exits 0 and prints its marker passes" 0 $?

echo
echo "validity-gate contracts (an unevaluable gate is INVALID, not clean)"

# The GPU session marker is cleared for this block, so the worker names resolve
# to containers and the gate reads `docker logs`. The unit-backed reading path
# is the one a GPU session exercises, and it is covered by GPU-04 on real
# journals -- a fake journalctl would test the fake.
rm -f "$FHEVM_STATE_DIR/runtime/gpu-consensus-workers/node-config.env"

# The lock-extension gate used to read `docker logs` unconditionally and turn a
# failed read into `0` matches, so a missing container, a permission error and a
# genuinely quiet worker were the same answer. Its refusals are now exit 3,
# distinct from a gate that ran and failed (exit 1).
#
# A fake docker that reports NO tfhe-worker at all: the gate has nothing to read
# and must say so.
cat >"$FAKE_BIN/docker" <<'FAKE'
#!/usr/bin/env bash
case "$1" in
  ps) echo "" ;;
  inspect) exit 1 ;;
  *) exit 0 ;;
esac
FAKE
chmod +x "$FAKE_BIN/docker"
CONSENSUS_RUN_STARTED_AT="2026-01-01T00:00:00Z" "$SCRIPT_DIR/consensus-validity.sh" locks >/dev/null 2>&1
check "the lock gate is INVALID when no worker is present" 3 $?

# A fleet that is one worker short of the count the caller declared: the gate is
# not reading the fleet under test.
cat >"$FAKE_BIN/docker" <<'FAKE'
#!/usr/bin/env bash
case "$1" in
  ps) printf 'coprocessor-tfhe-worker
coprocessor1-tfhe-worker
' ;;
  logs) echo "" ;;
  inspect) echo running ;;
  *) exit 0 ;;
esac
FAKE
chmod +x "$FAKE_BIN/docker"
CONSENSUS_RUN_STARTED_AT="2026-01-01T00:00:00Z"   "$SCRIPT_DIR/consensus-validity.sh" locks --operators 3 >/dev/null 2>&1
check "the lock gate is INVALID when an expected worker is missing" 3 $?

# And with no run window at all: counting lock losses from earlier suites on the
# same stack contaminated every later clean run.
env -u CONSENSUS_RUN_STARTED_AT "$SCRIPT_DIR/consensus-validity.sh" locks --operators 2 >/dev/null 2>&1
check "the lock gate is INVALID with no run window" 3 $?

# A quiet fleet inside a window is clean.
CONSENSUS_RUN_STARTED_AT="2026-01-01T00:00:00Z"   "$SCRIPT_DIR/consensus-validity.sh" locks --operators 2 >/dev/null 2>&1
check "a quiet fleet in a window is clean" 0 $?

# A worker that DID report lock loss fails the gate, and passes it when the case
# expects one.
cat >"$FAKE_BIN/docker" <<'FAKE'
#!/usr/bin/env bash
case "$1" in
  ps) printf 'coprocessor-tfhe-worker
coprocessor1-tfhe-worker
' ;;
  logs) echo "WARN Not all locks extended for worker" ;;
  inspect) echo running ;;
  *) exit 0 ;;
esac
FAKE
chmod +x "$FAKE_BIN/docker"
CONSENSUS_RUN_STARTED_AT="2026-01-01T00:00:00Z"   "$SCRIPT_DIR/consensus-validity.sh" locks --operators 2 >/dev/null 2>&1
check "reported lock loss fails an ordinary run" 1 $?
CONSENSUS_RUN_STARTED_AT="2026-01-01T00:00:00Z"   "$SCRIPT_DIR/consensus-validity.sh" locks --operators 2 --expect-loss >/dev/null 2>&1
check "reported lock loss passes a case that expects it" 0 $?
CONSENSUS_RUN_STARTED_AT="2026-01-01T00:00:00Z"   "$SCRIPT_DIR/consensus-validity.sh" locks --operators 2 --report-only >/dev/null 2>&1
check "reported lock loss is recorded rather than gated under --report-only" 0 $?

# A case that EXPECTS a lapsed lease and sees none has not exercised its path.
cat >"$FAKE_BIN/docker" <<'FAKE'
#!/usr/bin/env bash
case "$1" in
  ps) printf 'coprocessor-tfhe-worker
coprocessor1-tfhe-worker
' ;;
  logs) echo "" ;;
  inspect) echo running ;;
  *) exit 0 ;;
esac
FAKE
chmod +x "$FAKE_BIN/docker"
CONSENSUS_RUN_STARTED_AT="2026-01-01T00:00:00Z"   "$SCRIPT_DIR/consensus-validity.sh" locks --operators 2 --expect-loss >/dev/null 2>&1
check "a case expecting lock loss fails when none occurred" 1 $?

echo
echo "suite-identity contracts (a stale suite is not a passing one)"

# shellcheck source=lib/suite-identity.sh
source "${SCRIPT_DIR}/lib/suite-identity.sh"

# The fake docker answers no `exec`, so the container's copy cannot be read.
# That must be a refusal: a result cannot be labelled with code that could not
# be identified, and "I could not look" is not "they match".
TEST_CONTAINER=ghost-container
suite_identity_assert ghost-container >/dev/null 2>&1
check "an unreadable container suite is a mismatch, not a pass" 1 $?

# The digest itself must be stable and non-empty for the tree under test, or
# the comparison above would be vacuous -- two empty strings match.
host_identity="$(suite_identity_host)"
[[ "${#host_identity}" == 64 ]]
check "the working tree's suite hashes to a digest" 0 $?

echo
echo "inventory record contracts"

# A recorded result the aggregate would reject must be refused at write time,
# where the runner can still say something useful.
CR_RUN_ID="fault-contract-$$"
CR_REVISION="0000000000000000000000000000000000000000"
CR_BACKEND_CLASS=cpu
CR_HARDWARE_CLASS=test
CR_SCENARIO=none
CR_OPERATORS=0
CR_THRESHOLD=0
export CONSENSUS_RESULTS_DIR="$(mktemp -d)"
cr_record MAT-01-BOUNDARY-FANOUT FAIL >/dev/null 2>&1
check "a runner FAIL with no output receives a fallback diagnostic" 0 $?
grep -q 'the case ended FAIL without diagnostic output' "$CONSENSUS_RESULTS_DIR/$CR_RUN_ID.jsonl"
check "the fallback failure diagnostic is persisted" 0 $?
# The wrapper fills missing diagnostics; the raw schema still refuses them.
bun "$CR_INVENTORY_CLI" record --run "raw-$CR_RUN_ID" --case MAT-01-BOUNDARY-FANOUT \
  --state FAIL --revision "$CR_REVISION" --backend-class cpu --hardware-class test \
  --scenario none --cleanup not_required --detail "" --quiet >/dev/null 2>&1
check "the direct CLI refuses a FAIL with an empty diagnostic" 2 $?
cr_record NOT-A-REAL-CASE PASS >/dev/null 2>&1
check "a result for an unknown case id is refused" 2 $?
cr_record MAT-01-BOUNDARY-FANOUT PASS cleanup=ok >/dev/null 2>&1
check "a PASS missing its required assertion kinds is refused" 2 $?
cr_record MAT-01-BOUNDARY-FANOUT PASS cleanup=ok \
  assert="bytes=pass" assert="digest=pass" assert="provenance=pass" \
  assert="liveness=pass" assert="quorum=pass" >/dev/null 2>&1
check "a well-formed record is accepted" 0 $?

# The failure that must never be quiet: a record the schema refuses is not
# written, and a runner that ignored the status would print a verdict for a
# case with no row, leaving the aggregate to classify it as NOT_RUN.
CR_RECORD_FAILURES=0
cr_record MAT-01-BOUNDARY-FANOUT PASS cleanup=ok assert="x=not_applicable" >/dev/null 2>&1
check "a record the schema refuses returns non-zero" 2 $?
check "and it is counted, so the run can fail on it" 1 "${CR_RECORD_FAILURES:-0}"

# A stack that is not the scenario the inventory declares must decline the case,
# not run it and record an outcome the aggregate would then reject.
CR_SCENARIO=two-of-three
cr_skip_wrong_scenario MAT-01-BOUNDARY-FANOUT >/dev/null 2>&1
check "a case declared under another scenario is declined" 0 $?
grep -q 'NOT_APPLICABLE' "$CONSENSUS_RESULTS_DIR/$CR_RUN_ID.jsonl"
check "the decline is recorded rather than silent" 0 $?
CR_SCENARIO=three-of-three
cr_skip_wrong_scenario MAT-01-BOUNDARY-FANOUT >/dev/null 2>&1
check "a case on its declared scenario is not declined" 1 $?
CR_SCENARIO=none

rm -rf "$FAKE_BIN" "$FAKE_STATE" "$FAKE_STATE_DIR" "$CONSENSUS_RESULTS_DIR"

echo
if [[ "$FAILURES" -eq 0 ]]; then
  echo "HAR-01-FAULT-CONTRACTS: PASS ($CASES contract(s))"
  exit 0
fi
echo "HAR-01-FAULT-CONTRACTS: FAIL ($FAILURES of $CASES contract(s) did not hold)" >&2
exit 1
