#!/usr/bin/env bash
# The cases that need no stack, recorded as results.
#
#   harness          HAR-01, HAR-02, HAR-03, MAT-05-CANARY-CLASSES
#   comparator       MAT-05-CANARY-CLASSES
#   rust-regression  REG-01, REG-02
#
# These ran as bare CI steps, so a green leg meant "some commands exited 0" and
# the aggregate reported the cases NOT_RUN. Worse, a test binary that runs zero
# tests exits 0 too: `cargo test` with a filter that matches nothing, or a suite
# whose cases were all removed, is indistinguishable from a passing one by exit
# status alone. Each case here therefore carries a minimum number of tests that
# must actually have run, taken from the count the suite has today.
#
#   scripts/run-stackless-cases.sh [--leg harness|comparator|rust-regression|all]
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly CLI_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
readonly ENGINE_DIR="${REPO_ROOT}/coprocessor/fhevm-engine"

# shellcheck source=lib/case-result.sh
source "${SCRIPT_DIR}/lib/case-result.sh"
source "${SCRIPT_DIR}/lib/runner-assertions.sh"

LEG=all
while [[ $# -gt 0 ]]; do
  case "$1" in
    --leg) LEG="${2:?--leg needs a value}"; shift 2 ;;
    *) echo "usage: run-stackless-cases.sh [--leg harness|comparator|rust-regression|all]" >&2; exit 2 ;;
  esac
done

log() { printf '\n=== %s\n' "$*"; }
FAILURES=0

# How many tests actually ran, from the runner's own summary. Empty when the
# output carries no count, which is itself a reason to refuse the result.
bun_test_count() { sed -n 's/^ *\([0-9]\+\) pass$/\1/p' <<<"$1" | tail -1; }
# `bun test` resolves paths against the working directory, and these runners are
# invoked from anywhere.
bun_test() { (cd "$CLI_DIR" && bun test "$@"); }
mocha_test_count() { sed -n 's/^ *\([0-9]\+\) passing.*/\1/p' <<<"$1" | tail -1; }
# Rustup resolves the pinned toolchain from cwd, not --manifest-path.
cargo_test() { (cd "$ENGINE_DIR" && SQLX_OFFLINE=true cargo test "$@"); }
broker_control_test() {
  (cd "$REPO_ROOT/listener" && SQLX_OFFLINE=true cargo test -p broker --features test-failpoints --lib test_ack_boundary::tests)
}
cargo_test_count() {
  # `test result: ok. 4 passed; 0 failed; ...`, summed over binaries.
  awk '/^test result:/ { for (i = 1; i <= NF; i++) if ($(i+1) == "passed;") total += $i } END { print total + 0 }' <<<"$1"
}

# The exact parent test must execute; unrelated passing tests cannot substitute
# for the process-kill construction (nor can its ignored subprocess helper).
controller_crash_test_count() {
  if ! grep -Fxq 'test tests::cutover_recovers_after_process_kill ... ok' <<<"$1"; then
    echo 0
    return
  fi
  cargo_test_count "$1"
}

transcript_test_count() {
  if ! grep -Fxq 'test dfg::scheduler::tests::rerandomization_binds_handle_opcode_and_ordered_ciphertexts ... ok' <<<"$1"; then
    echo 0
    return
  fi
  cargo_test_count "$1"
}

poller_timeout_test_count() {
  if ! grep -Fxq 'test poller::http_client::tests::silent_peer_times_out_and_same_client_recovers ... ok' <<<"$1"; then
    echo 0
    return
  fi
  cargo_test_count "$1"
}

squash_test_count() {
  if ! grep -Fxq 'test tests::squash_determinism::generated_key_squash_agrees_across_reconstruction_and_threads ... ok' <<<"$1"; then
    echo 0
    return
  fi
  cargo_test_count "$1"
}

# record_case <case-id> <min-tests> <counter> <description> -- <command...>
record_case() {
  local case_id="$1" min="$2" counter="$3" description="$4" kinds="$5"; shift 5
  [[ "$1" == -- ]] && shift
  cr_skip_wrong_scenario "$case_id" && return 0
  local started; started="$(cr_now)"
  log "$case_id: $description"
  local out status=0
  out="$("$@" 2>&1)" || status=$?
  echo "$out" | tail -40 | sed 's/^/    /'

  local count; count="$("$counter" "$out")"
  if [[ "$status" -ne 0 ]]; then
    cr_record "$case_id" FAIL started_at="$started" cleanup=not_required \
      detail="$(cr_failure_reason "$out")" assert="contracts-hold=fail"
    echo "[$case_id] FAIL"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  if [[ -z "$count" || "$count" -lt "$min" ]]; then
    # Exit status 0 with too few tests is the failure mode this guard exists
    # for: a filter that matches nothing, or a suite emptied by a refactor,
    # reports success having established nothing.
    cr_record "$case_id" INVALID started_at="$started" cleanup=not_required \
      detail="the runner exited 0 having run ${count:-no} test(s), fewer than the ${min} this case needs; a suite that runs nothing establishes nothing"
    echo "[$case_id] INVALID: ${count:-no} test(s) ran, expected at least $min"
    FAILURES=$((FAILURES + 1))
    return 1
  fi
  local -a outcomes=(); local kind
  for kind in $kinds; do outcomes+=("assert=$kind=pass:$count selected regression tests ran and passed"); done
  cr_record_checked_pass "$case_id" "${outcomes[@]}" started_at="$started" cleanup=not_required \
    assert="contracts-hold=pass:$count test(s) ran and passed" \
    artifact="tests_run=$count"
  echo "[$case_id] PASS ($count test(s))"
}

harness_leg() {
  # The fault-control contracts print their own count.
  local started status=0 out
  cr_skip_wrong_scenario HAR-01-FAULT-CONTRACTS || {
    started="$(cr_now)"
    log "HAR-01-FAULT-CONTRACTS: the fault layer fails closed"
    out="$("$SCRIPT_DIR/test-fault-contracts.sh" 2>&1)" || status=$?
    echo "$out" | tail -20 | sed 's/^/    /'
    local contracts; contracts="$(sed -n 's/^HAR-01-FAULT-CONTRACTS: PASS (\([0-9]\+\) contract(s))$/\1/p' <<<"$out")"
    if [[ "$status" -ne 0 || -z "$contracts" ]]; then
      cr_record HAR-01-FAULT-CONTRACTS FAIL started_at="$started" cleanup=not_required \
        detail="$(cr_failure_reason "$out")"
      echo "[HAR-01-FAULT-CONTRACTS] FAIL"
      FAILURES=$((FAILURES + 1))
    else
      cr_record_checked_pass HAR-01-FAULT-CONTRACTS started_at="$started" cleanup=not_required \
        assert="safety=pass:$contracts fault-control contracts held" artifact="contracts=$contracts"
      echo "[HAR-01-FAULT-CONTRACTS] PASS ($contracts contract(s))"
    fi
  }

  record_case HAR-02-INVENTORY-AGGREGATE 30 bun_test_count \
    "the inventory and aggregate reject what they claim to reject" "safety" \
    -- bun_test src/consensus

  record_case HAR-03-READINESS-CONTRACTS 20 bun_test_count \
    "readiness and validity gates refuse unevaluable evidence" "safety" \
    -- bun_test src/readiness.test.ts

  comparator_leg

}

comparator_leg() {
  record_case MAT-05-CANARY-CLASSES 30 mocha_test_count \
    "standalone consensus oracles and fault controls reject invalid evidence" "safety" \
    -- bun "$SCRIPT_DIR/run-comparator-contracts.ts"
}

migration_validation_tests() {
  cargo_test -p host-listener --features test-failpoints --lib compressed_material_updates_only_the_compressed_representation &&
    cargo_test -p host-listener --features test-failpoints --lib transport_control_cannot_capture_other_keys_or_outlive_its_budget
}
migration_validation_count() {
  if grep -Fxq 'test kms_generation::database::tests::compressed_material_updates_only_the_compressed_representation ... ok' <<<"$1" &&
     grep -Fxq 'test kms_generation::download_test_control::tests::transport_control_cannot_capture_other_keys_or_outlive_its_budget ... ok' <<<"$1"; then
    cargo_test_count "$1"
  else echo 0; fi
}

rust_regression_leg() {
  record_case REG-06-MIGRATION-VALIDATION 2 migration_validation_count \
    "real parser staging and decoded activation replay are fail-closed; transport scope expires" "safety" \
    -- migration_validation_tests
  # Metric evidence is load-bearing for SCH-01. Pin this exact contract instead
  # of accepting a successful Cargo filter that selected no tests.
  local limiter_out
  if limiter_out="$(cargo_test -p scheduler --lib shared_capacity_blocks_and_records_real_overlap 2>&1)" &&
     grep -Fxq 'test gpu_execution::tests::shared_capacity_blocks_and_records_real_overlap ... ok' <<<"$limiter_out"; then
    echo "$limiter_out"
  else
    echo "$limiter_out"
    echo 'GPU execution permit contract did not pass' >&2
    FAILURES=$((FAILURES + 1))
  fi
  record_case REG-05-HOST-RPC-DEADLINE 1 poller_timeout_test_count \
    "silent HTTP peer reaches the production deadline and the same client recovers" "liveness" \
    -- cargo_test -p host-listener --lib silent_peer_times_out_and_same_client_recovers
  # These controls must pass before the live redelivery leg can be trusted.
  # Broker belongs to the listener workspace, not the engine workspace.
  if ! broker_control_test; then FAILURES=$((FAILURES + 1)); fi
  if ! cargo_test -p host-listener --features test-failpoints --lib consensus_test_control::tests; then
    FAILURES=$((FAILURES + 1))
  fi
  record_case SCH-03-SQUASH-DETERMINISM 1 squash_test_count \
    "generated-key fixed-input CPU squash agrees across reconstruction and threads" "safety sensitivity" \
    -- cargo_test -p sns-worker --lib generated_key_squash_agrees_across_reconstruction_and_threads
  record_case REG-04-RERANDOMIZATION-TRANSCRIPT 1 transcript_test_count \
    "actual rerandomization binds output handle, opcode and ordered ciphertexts" "safety sensitivity" \
    -- cargo_test -p scheduler --lib rerandomization_binds_handle_opcode_and_ordered_ciphertexts
  if ! cargo_test -p tfhe-worker --features test-failpoints --lib test_failpoints::tests; then
    FAILURES=$((FAILURES + 1))
  fi
  record_case FM-UPGRADE-CONTROLLER 42 controller_crash_test_count \
    "controller cutover, write fences, and recovery after a real process kill" "safety liveness" \
    -- cargo_test -p upgrade-controller --lib -- --test-threads=2
  # Both need a database through testcontainers, which needs docker.
  record_case REG-01-LISTENER-POOL-REBIND 4 cargo_test_count \
    "the stack-version listener rebinds and reconciles" "liveness safety" \
    -- cargo_test -p host-listener --test stack_version_listener_tests -- --test-threads=1

  record_case REG-02-TFHE-DAEMON-EXIT 2 cargo_test_count \
    "a fatal daemon failure exits non-zero, and a voluntary one does not" "liveness safety" \
    -- cargo_test -p tfhe-worker --test daemon_exit
}

main() {
  # These cases have no topology: the inventory says scenario `none`, and a
  # result claiming a scenario would be claiming something untrue.
  CONSENSUS_SCENARIO="${CONSENSUS_SCENARIO:-none}"
  CONSENSUS_OPERATORS=0
  CONSENSUS_THRESHOLD=0
  cr_init "${CONSENSUS_RUN_ID:-}" || exit 1

  case "$LEG" in
    harness) harness_leg ;;
    comparator) comparator_leg ;;
    rust-regression) rust_regression_leg ;;
    all) harness_leg; rust_regression_leg ;;
    *) echo "unknown --leg $LEG" >&2; exit 2 ;;
  esac

  log "results"
  echo "run $CR_RUN_ID: $FAILURES failing case(s)"
  echo "structured results: $(cr_results_file)"
  if [[ "${CR_RECORD_FAILURES:-0}" -gt 0 ]]; then
    echo "${CR_RECORD_FAILURES} result(s) were REFUSED and not written; this run has no record of them" >&2
    return 1
  fi
  [[ "$FAILURES" -eq 0 ]]
}

main
