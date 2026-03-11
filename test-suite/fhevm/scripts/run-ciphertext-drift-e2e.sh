#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FAULTY_INSTANCE_INDEX="${FAULTY_INSTANCE_INDEX:-1}"
FAULTY_TX_SENDER_CONTAINER="${FAULTY_TX_SENDER_CONTAINER:-coprocessor${FAULTY_INSTANCE_INDEX}-transaction-sender}"
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
GREP_PATTERN="${GREP_PATTERN:-test user input uint64 \\(non-trivial\\)}"
METRIC_NAME="coprocessor_gw_listener_drift_detected_counter"
METRIC_TIMEOUT_SECONDS="${DRIFT_METRIC_TIMEOUT_SECONDS:-180}"
METRIC_POLL_INTERVAL_SECONDS="${DRIFT_METRIC_POLL_INTERVAL_SECONDS:-2}"
HANDLE_FILE="$(mktemp)"
injector_pid=""

metric_total() {
  local total=0
  local container
  while IFS= read -r container; do
    [ -z "$container" ] && continue
    local value
    value="$(docker exec "$container" curl -fsS http://127.0.0.1:9100/metrics 2>/dev/null | awk -v metric="$METRIC_NAME" '$1 == metric {sum += $2} END {print sum + 0}')"
    total=$((total + value))
  done < <(docker ps --format '{{.Names}}' | grep -E '^coprocessor([0-9]+)?-gw-listener$' || true)
  echo "$total"
}

wait_for_metric_increment() {
  local baseline="$1"
  local deadline=$((SECONDS + METRIC_TIMEOUT_SECONDS))
  while [ "$SECONDS" -lt "$deadline" ]; do
    local current
    current="$(metric_total)"
    if [ "$current" -gt "$baseline" ]; then
      echo "$current"
      return 0
    fi
    sleep "$METRIC_POLL_INTERVAL_SECONDS"
  done
  return 1
}

cleanup() {
  if [ -n "$injector_pid" ] && kill -0 "$injector_pid" >/dev/null 2>&1; then
    kill "$injector_pid" >/dev/null 2>&1 || true
  fi
  docker start "$FAULTY_TX_SENDER_CONTAINER" >/dev/null 2>&1 || true
  rm -f "$HANDLE_FILE"
}

trap cleanup EXIT

baseline_metric="$(metric_total)"
docker stop "$FAULTY_TX_SENDER_CONTAINER" >/dev/null

"${SCRIPT_DIR}/inject-coprocessor-drift.sh" "$FAULTY_INSTANCE_INDEX" > "$HANDLE_FILE" &
injector_pid=$!

docker exec \
  -e GATEWAY_RPC_URL= \
  "$TEST_CONTAINER" \
  ./run-tests.sh -n staging -g "$GREP_PATTERN"

wait "$injector_pid"
injector_pid=""
handle_hex="$(cat "$HANDLE_FILE")"

docker start "$FAULTY_TX_SENDER_CONTAINER" >/dev/null

if ! updated_metric="$(wait_for_metric_increment "$baseline_metric")"; then
  echo "drift metric did not increase after injecting handle ${handle_hex}" >&2
  exit 1
fi

echo "drift detected for handle ${handle_hex} (${METRIC_NAME}: ${baseline_metric} -> ${updated_metric})"
