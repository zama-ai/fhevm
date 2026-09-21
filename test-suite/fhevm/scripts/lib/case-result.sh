# shellcheck shell=bash
# Recording a case result, from a shell runner.
#
# Sourced, not executed. Callers must have SCRIPT_DIR and REPO_ROOT set, and
# should call `cr_init <run-id>` once before recording anything.
#
# The runners used to decide whether a case had passed by grepping the test
# output for `1 passing`. That string survives an after-hook failure, so a case
# whose cleanup or format gate blew up afterwards still read as green -- and
# nothing recorded whether the fault had landed at all. Results are structured
# records now, written through `consensus-inventory.ts record`, which refuses a
# record the aggregate would reject.
#
# `cr_run_suite` is the replacement for the grep: it runs a suite, keeps the
# process EXIT STATUS as the verdict, and additionally requires the suite's own
# structured marker line when one is expected.

source "${SCRIPT_DIR}/lib/source-revision.sh"

readonly CR_INVENTORY_CLI="${SCRIPT_DIR}/consensus-inventory.ts"

cr_init() {
  CR_RUN_ID="${1:-${CONSENSUS_RUN_ID:-$(date -u +%Y%m%dT%H%M%SZ)-$$}}"
  CR_REVISION="$(sr_revision "$REPO_ROOT")" || return 1
  CR_BACKEND_CLASS="${CONSENSUS_BACKEND_CLASS:-cpu}"
  CR_HARDWARE_CLASS="${CONSENSUS_HARDWARE_CLASS:-cpu-$(uname -m)}"
  CR_SCENARIO="${CONSENSUS_SCENARIO:-unknown}"
  if [[ "$CR_SCENARIO" == unknown || -z "$CR_SCENARIO" ]]; then
    echo "case-result: CONSENSUS_SCENARIO must identify the running topology" >&2
    return 2
  fi
  CR_OPERATORS="${CONSENSUS_OPERATORS:-0}"
  CR_THRESHOLD="${CONSENSUS_THRESHOLD:-0}"
  # Delegated runners are part of this run even when the caller omitted an ID.
  CONSENSUS_RUN_ID="$CR_RUN_ID"
  export CR_RUN_ID CONSENSUS_RUN_ID
  echo "consensus run id: $CR_RUN_ID (revision $CR_REVISION)"
}

# The scenario the inventory says a case is established under.
#
# Read from the inventory rather than restated in a runner: a runner that
# records a result from a topology the case is not declared under writes a row
# the aggregate rejects, and that reads as a broken run instead of as what it
# is -- a case this stack cannot establish. Restating a case's topology in bash
# is the same class of bug as a test asserting what it does not check.
cr_declared_scenario() {
  bun "$CR_INVENTORY_CLI" show "$1" 2>/dev/null | sed -n 's/^topology: *\([^ ]*\).*/\1/p' | head -1
}

# Records NOT_APPLICABLE and returns 0 when the running stack is not the
# scenario the inventory declares for this case, so the caller can skip it.
# Returns 1 when the case belongs on this stack and should run.
cr_skip_wrong_scenario() {
  local case_id="$1" want
  want="$(cr_declared_scenario "$case_id")"
  # `none` means the case has no topology at all -- a contract test, not a
  # fleet -- so it is runnable wherever it is invoked.
  [[ -n "$want" && "$want" != unknown && "$want" != none && "$want" != "${CR_SCENARIO:-unknown}" ]] || return 1
  cr_record "$case_id" NOT_APPLICABLE started_at="$(cr_now)" cleanup=not_required \
    detail="the inventory establishes this case under the $want scenario; this stack is ${CR_SCENARIO:-unknown}, which cannot establish it"
  echo "[$case_id] NOT_APPLICABLE: needs the $want scenario, this stack is ${CR_SCENARIO:-unknown}"
  return 0
}

# cr_record <case-id> <state> [key=value ...]
#
# Recognised keys: detail, workload (repeatable), fault_observed_at,
# recovery_observed_at, process_before (repeatable, target=identity),
# process_after (repeatable), assert (repeatable, name=outcome[:detail]),
# cleanup, cleanup_detail, artifact (repeatable, name=value),
# scheduling_classes, started_at.
cr_record() {
  local case_id="$1" state="$2"; shift 2
  local -a args=(
    record
    --run "$CR_RUN_ID"
    --case "$case_id"
    --state "$state"
    --revision "$CR_REVISION"
    --backend-class "$CR_BACKEND_CLASS"
    --hardware-class "$CR_HARDWARE_CLASS"
    --scenario "$CR_SCENARIO"
  )
  [[ "$CR_OPERATORS" != 0 ]] && args+=(--operators "$CR_OPERATORS" --threshold "$CR_THRESHOLD")
  [[ -n "${CONSENSUS_SCHEDULING_CLASSES:-}" ]] && args+=(--scheduling-classes "$CONSENSUS_SCHEDULING_CLASSES")

  local pair key value detail_seen=0
  for pair in "$@"; do
    key="${pair%%=*}"
    value="${pair#*=}"
    case "$key" in
      detail) value="$(cr_redact_diagnostics "$value")"; [[ -n "$value" ]] && { args+=(--detail "$value"); detail_seen=1; } ;;
      workload) args+=(--workload "$value") ;;
      fault_observed_at) args+=(--fault-observed-at "$value") ;;
      recovery_observed_at) args+=(--recovery-observed-at "$value") ;;
      process_before) args+=(--process-before "$value") ;;
      process_after) args+=(--process-after "$value") ;;
      assert) args+=(--assert "$(cr_redact_diagnostics "$value")") ;;
      cleanup) args+=(--cleanup "$value") ;;
      cleanup_detail) args+=(--cleanup-detail "$(cr_redact_diagnostics "$value")") ;;
      artifact) args+=(--artifact-identity "$value") ;;
      scheduling_classes) args+=(--scheduling-classes "$value") ;;
      started_at) args+=(--started-at "$value") ;;
      *) echo "case-result: unknown key $key" >&2; return 2 ;;
    esac
  done
  # A rejected record is the one failure that must never be quiet: the runner
  # would print its verdict, the row would not exist, and the aggregate would
  # report the case NOT_RUN while the log said PASS.
  if [[ "$detail_seen" == 0 && ( "$state" == FAIL || "$state" == INVALID ) ]]; then
    args+=(--detail "the case ended $state without diagnostic output; inspect its phase and recovery logs")
  fi
  local record_status=0
  bun "$CR_INVENTORY_CLI" "${args[@]}" --quiet || record_status=$?
  if [[ "$record_status" -ne 0 ]]; then
    CR_RECORD_FAILURES=$((${CR_RECORD_FAILURES:-0} + 1))
    printf 'case-result: REFUSED to record %s %s -- the result was NOT written\n' "$case_id" "$state" >&2
  fi
  # The CLI's own status, so a caller can still tell a rejection from a crash.
  if [[ "$record_status" == 0 && -n "${CR_TERMINAL_FILE:-}" ]]; then
    printf '%s\n' "$case_id" >> "$CR_TERMINAL_FILE"
  fi
  return "$record_status"
}

cr_now() { date -u +%Y-%m-%dT%H:%M:%SZ; }

# Where the run's results file lives, for artifact upload.
cr_results_file() {
  local dir="${CONSENSUS_RESULTS_DIR:-${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/consensus-results}"
  printf '%s/%s.jsonl' "$dir" "$CR_RUN_ID"
}

# cr_run_suite <output-var> <marker-or-empty> <command...>
#
# Runs a suite and returns its EXIT STATUS. `1 passing` in mocha output is not
# the verdict: an after-hook can fail after a test body passes, and the process
# then exits non-zero while the string is still there. When `marker` is given,
# the suite must also have printed that marker -- proof that the assertion the
# case is about actually executed, rather than the suite skipping itself green.
cr_run_suite() {
  local -n cr_out="$1"; shift
  local marker="$1"; shift
  local status=0 cr_output_file
  cr_output_file="$(mktemp)" || return 1
  # A builtin wait is interruptible; waiting inside command substitution defers
  # the runner's signal trap until the very workload it must cancel returns.
  "$@" >"$cr_output_file" 2>&1 &
  CR_SUITE_PID=$!
  wait "$CR_SUITE_PID" || status=$?
  CR_SUITE_PID=""
  cr_out="$(cat "$cr_output_file")"
  rm -f "$cr_output_file"
  if [[ "$status" -ne 0 ]]; then
    return "$status"
  fi
  if [[ -n "$marker" ]] && ! grep -qF "$marker" <<<"$cr_out"; then
    cr_out="${cr_out}
case-result: the suite exited 0 but never printed its marker '${marker}', so the assertion did not run"
    return 3
  fi
  # A suite that skipped every test exits 0 too. Mocha says so; require that it
  # did not.
  if grep -qE '^\s+[0-9]+ pending' <<<"$cr_out" && ! grep -qE '^\s+[1-9][0-9]* passing' <<<"$cr_out"; then
    cr_out="${cr_out}
case-result: the suite exited 0 with everything pending; a skipped suite is NOT_RUN, not a pass"
    return 4
  fi
  return 0
}

# The state a non-zero suite exit deserves.
#
# A suite that raised `InvalidRunError` said, in its own words, that it could
# not measure the property -- an idle operator, an unreadable metric, a workload
# that never drove the difference it compares. Recording that as FAIL reports a
# broken system when what happened is an undirected test, which is the same
# false claim as a PASS with nothing behind it, pointed the other way. INVALID
# is not a softer verdict: the aggregate refuses it for a required case exactly
# as it refuses a FAIL.
cr_suite_state() {
  if grep -q "InvalidRunError" <<<"$1"; then printf 'INVALID'; else printf 'FAIL'; fi
}

# Runner diagnostics may contain database URLs from assertions. Remove userinfo
# before truncation, so a long credential cannot survive as a URL fragment. The
# direct record CLI still rejects secrets in structured, unsanitized evidence.
cr_redact_diagnostics() {
  printf '%s' "$1" | sed -E 's#(postgres(ql)?://)[^@[:space:]]+@#\1[redacted]@#gI'
}

# First line worth showing a reader from a failed suite's output.
cr_failure_reason() {
  local reason diagnostic
  diagnostic="$(cr_redact_diagnostics "$1")"
  reason="$(grep -m1 -E "InvalidRunError|AssertionError|Error:|timed out|case-result:" <<<"$diagnostic" \
    | sed 's/^[[:space:]]*//' | cut -c1-200)" || true
  [[ -n "$reason" ]] || reason="$(sed '/^[[:space:]]*$/d' <<<"$diagnostic" | head -1 | cut -c1-200)"
  printf '%s\n' "${reason:-suite exited unsuccessfully without diagnostic output}"
}

# Process substitutions hide the producer's exit status from mapfile. Capture
# first, and refuse partial output from an unsuccessful identity inspection.
cr_read_run_identity() {
  local -n destination="$1"
  local output
  output="$("$SCRIPT_DIR/record-run-identity.sh" --format cr)" || return 1
  [[ -n "$output" ]] || return 1
  mapfile -t destination <<<"$output"
}
