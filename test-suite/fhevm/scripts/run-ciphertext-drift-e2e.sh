#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FAULTY_INSTANCE_INDEX="${FAULTY_INSTANCE_INDEX:-1}"
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
GREP_PATTERN="${GREP_PATTERN:-test user input uint64 \\(non-trivial\\)}"
DRIFT_ALERT_TIMEOUT_SECONDS="${DRIFT_ALERT_TIMEOUT_SECONDS:-180}"
DRIFT_ALERT_POLL_INTERVAL_SECONDS="${DRIFT_ALERT_POLL_INTERVAL_SECONDS:-2}"
HANDLE_FILE="$(mktemp)"
injector_pid=""
LOG_SINCE="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

wait_for_drift_log() {
  local handle_hex="$1"
  local deadline=$((SECONDS + DRIFT_ALERT_TIMEOUT_SECONDS))
  local container
  while [ "$SECONDS" -lt "$deadline" ]; do
    while IFS= read -r container; do
      [ -z "$container" ] && continue
      if docker logs --since "$LOG_SINCE" "$container" 2>&1 \
        | grep -F '"message":"Drift detected: observed multiple digest variants for handle"' \
        | grep -F "\"handle\":\"0x${handle_hex}\"" >/dev/null; then
        echo "$container"
        return 0
      fi
    done < <(docker ps --format '{{.Names}}' | grep -E '^coprocessor([0-9]+)?-gw-listener$' || true)
    sleep "$DRIFT_ALERT_POLL_INTERVAL_SECONDS"
  done
  return 1
}

cleanup() {
  if [ -n "$injector_pid" ] && kill -0 "$injector_pid" >/dev/null 2>&1; then
    kill "$injector_pid" >/dev/null 2>&1 || true
  fi
  rm -f "$HANDLE_FILE"
}

trap cleanup EXIT

bun run "${SCRIPT_DIR}/inject-coprocessor-drift.ts" "$FAULTY_INSTANCE_INDEX" > "$HANDLE_FILE" &
injector_pid=$!

test_exit=0
docker exec \
  -e GATEWAY_RPC_URL= \
  "$TEST_CONTAINER" \
  ./run-tests.sh -n staging -g "$GREP_PATTERN" || test_exit=$?

injector_exit=0
wait "$injector_pid" || injector_exit=$?
injector_pid=""
handle_hex="$(cat "$HANDLE_FILE")"

if [ "$test_exit" -ne 0 ]; then
  exit "$test_exit"
fi

if [ "$injector_exit" -ne 0 ]; then
  echo "drift injector failed with exit code ${injector_exit}" >&2
  exit "$injector_exit"
fi

if ! detecting_container="$(wait_for_drift_log "$handle_hex")"; then
  echo "drift warning was not observed for injected handle ${handle_hex}" >&2
  exit 1
fi

echo "drift detected for handle ${handle_hex} in ${detecting_container}"
