#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly LOG_DIR="${SCRIPT_DIR}/.logs"
readonly DEFAULT_COUNT=1
readonly BASE_METRICS_PORT=9100

usage() {
  cat <<'EOF'
Usage: ./run_with_localnet.sh [count] [-- extra tfhe-compute-node args]

Spawns one or more local tfhe-compute-node replicas.

Arguments:
  count   Number of replicas to start. Defaults to 1.

Any arguments after `--` are passed through to each replica.
EOF
}

if [[ -f "${SCRIPT_DIR}/.env" ]]; then
  # shellcheck disable=SC1091
  source "${SCRIPT_DIR}/.env"
fi

count="${DEFAULT_COUNT}"
declare -a passthrough_args=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    --)
      shift
      passthrough_args=("$@")
      break
      ;;
    *)
      if [[ "$count" != "${DEFAULT_COUNT}" ]]; then
        echo "Only one replica count argument is allowed." >&2
        usage >&2
        exit 1
      fi
      count="$1"
      shift
      ;;
  esac
done

if ! [[ "${count}" =~ ^[1-9][0-9]*$ ]]; then
  echo "Replica count must be a positive integer." >&2
  exit 1
fi

mkdir -p "${LOG_DIR}"

declare -a pids=()

cleanup() {
  local pid
  for pid in "${pids[@]:-}"; do
    if kill -0 "${pid}" 2>/dev/null; then
      kill "${pid}" 2>/dev/null || true
    fi
  done
  wait || true
}

trap cleanup INT TERM EXIT

for ((i = 1; i <= count; i++)); do
  replica_log="${LOG_DIR}/replica_${i}.log"
  worker_id="$(cat /proc/sys/kernel/random/uuid)"
  metrics_port="$((BASE_METRICS_PORT + i - 1))"

  echo "Starting replica ${i}/${count} -> ${replica_log}"

  (
    cd "${SCRIPT_DIR}"
    # Load the test env in the worker process before starting the replica.
    # shellcheck disable=SC1091
    source "${SCRIPT_DIR}/../.env-test"
    exec cargo run --release -- \
      --worker-id="${worker_id}" \
      --service-name="tfhe-compute-node-replica-${i}" \
      --metrics-addr="0.0.0.0:${metrics_port}" \
      "${passthrough_args[@]}"
  ) >"${replica_log}" 2>&1 &

  pids+=("$!")
done

echo "Replica PIDs: ${pids[*]}"
echo "Logs directory: ${LOG_DIR}"

wait
