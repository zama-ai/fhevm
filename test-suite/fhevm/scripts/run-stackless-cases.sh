#!/usr/bin/env bash
# The cases that need no stack, recorded as results.
#
#   harness          HAR-02, HAR-03, MAT-05-CANARY-CLASSES
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
cargo_test_count() {
  # `test result: ok. 4 passed; 0 failed; ...`, summed over binaries.
  awk '/^test result:/ { for (i = 1; i <= NF; i++) if ($(i+1) == "passed;") total += $i } END { print total + 0 }' <<<"$1"
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
    "standalone consensus oracles reject invalid evidence" "safety" \
    -- bun "$SCRIPT_DIR/run-comparator-contracts.ts"
}

rust_regression_leg() {
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
