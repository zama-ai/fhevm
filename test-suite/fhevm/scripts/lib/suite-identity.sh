# shellcheck shell=bash
# Is the test container running the suite in this working tree?
#
# Sourced, not executed. Callers must have REPO_ROOT set and a TEST_CONTAINER.
#
# The test-suite image bakes `test-suite/e2e` in; nothing is mounted. So a
# commit to an e2e suite after the last `up --build` changes the working tree
# and changes nothing about what runs, and the runner then labels the result
# with a revision whose code never executed. Check the baked source explicitly
# and fail closed before attributing a result to this checkout.
#
# `fhevm-cli upgrade test-suite` rebuilds the group in place.

# A hash over every source file of the e2e suite, relative to its root, so the
# host and the container produce comparable digests. Compiled output and
# dependencies are excluded: they are derived, and they differ for reasons that
# do not change what the suite asserts.
suite_identity_host() {
  (
    cd "$REPO_ROOT/test-suite/e2e" 2>/dev/null || return 1
    # Matrix phases export these functions to a fresh Bash process. Keep the
    # roots in the function: Bash arrays do not survive that export boundary.
    find test contracts -type f \( -name '*.ts' -o -name '*.sol' \) -not -path '*/node_modules/*' |
      LC_ALL=C sort | xargs sha256sum | sha256sum | cut -d' ' -f1
  )
}

suite_identity_container() {
  local container="$1"
  docker exec "$container" sh -c \
    'cd /app/test-suite/e2e && find test contracts -type f \( -name "*.ts" -o -name "*.sol" \) -not -path "*/node_modules/*" | LC_ALL=C sort | xargs sha256sum | sha256sum' \
    2>/dev/null | cut -d' ' -f1
}

# The files that differ, so the message says what to look at rather than only
# that something is wrong.
suite_identity_diff() {
  local container="$1" host_list container_list
  host_list="$(cd "$REPO_ROOT/test-suite/e2e" && find test contracts -type f \( -name '*.ts' -o -name '*.sol' \) -not -path '*/node_modules/*' | LC_ALL=C sort | xargs sha256sum)"
  container_list="$(docker exec "$container" sh -c \
    'cd /app/test-suite/e2e && find test contracts -type f \( -name "*.ts" -o -name "*.sol" \) -not -path "*/node_modules/*" | LC_ALL=C sort | xargs sha256sum' 2>/dev/null)"
  diff <(echo "$host_list") <(echo "$container_list") | sed -n 's/^[<>] *[0-9a-f]* *//p' | LC_ALL=C sort -u
}

# Fails closed: an unreadable container copy is a mismatch, not a pass.
suite_identity_assert() {
  local contamination="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/failure-matrix/uncancelled-phase"
  if [[ -f "$contamination" ]]; then
    echo "suite-identity: a previous test process could not be stopped; recover the recorded faults before clearing $contamination" >&2
    return 1
  fi
  local container="${1:-$TEST_CONTAINER}" host_hash container_hash
  host_hash="$(suite_identity_host)"
  container_hash="$(suite_identity_container "$container")"
  if [[ -z "$host_hash" ]]; then
    echo "suite-identity: cannot hash the e2e suite in $REPO_ROOT/test-suite/e2e" >&2
    return 1
  fi
  if [[ -z "$container_hash" ]]; then
    echo "suite-identity: cannot read the e2e suite inside $container; a result cannot be labelled with code that could not be identified" >&2
    return 1
  fi
  if [[ "$host_hash" != "$container_hash" ]]; then
    echo "suite-identity: $container is running a DIFFERENT e2e suite from this working tree" >&2
    echo "  working tree: $host_hash" >&2
    echo "  container:    $container_hash" >&2
    echo "  differing files:" >&2
    suite_identity_diff "$container" | sed 's/^/    /' >&2
    echo "  the image bakes test-suite/e2e in, so a commit since the last build does not reach it." >&2
    echo "  rebuild that group in place:  ./fhevm-cli upgrade test-suite" >&2
    return 1
  fi
  echo "suite identity: $container runs this working tree's e2e suite ($host_hash)"
  return 0
}
