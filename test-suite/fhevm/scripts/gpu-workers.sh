#!/usr/bin/env bash
# Start host-built GPU coprocessor workers for the active local FHEVM stack.
#
# The generated runtime is intentionally kept under .fhevm, so this script is
# tracked separately and reconstructs its small host-only environment on each
# start.  Docker worker containers must not run alongside these processes:
# both implementations consume the same Postgres work queues.
set -euo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly RUNTIME_DIR="${REPO_ROOT}/.fhevm/runtime"
readonly ENV_DIR="${RUNTIME_DIR}/env"
readonly GPU_RUNTIME_DIR="${RUNTIME_DIR}/gpu-workers"
readonly BIN_DIR="${REPO_ROOT}/coprocessor/fhevm-engine/target/release"

usage() {
  cat <<'EOF'
Usage: GPU_WORKER_LAYOUT=<split|single-gpu1|dual-sns> test-suite/fhevm/scripts/gpu-workers.sh <start|stop|status>

Starts host-built TFHE, ZK-proof, and SNS workers for every active coprocessor
instance. Layouts are:
  split        (default): TFHE/ZK on GPU 0; SNS on GPU 1.
  single-gpu1: all workers on GPU 1.
  dual-sns:    TFHE/ZK and one SNS on GPU 0; a second SNS on GPU 1.
EOF
}

die() {
  echo "gpu-workers: $*" >&2
  exit 1
}

require() {
  command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

unit_name() {
  local kind="$1" index="$2" suffix="${3:-}"
  printf 'fhevm-gpu-%s%s-%s' "$kind" "${suffix:+-$suffix}" "$index"
}

container_name() {
  local kind="$1" index="$2"
  if [[ "$index" == "0" ]]; then
    printf 'coprocessor-%s-worker' "$kind"
  else
    printf 'coprocessor%s-%s-worker' "$index" "$kind"
  fi
}

env_file_for() {
  local index="$1"
  if [[ "$index" == "0" ]]; then
    printf '%s/coprocessor.env' "$ENV_DIR"
  else
    printf '%s/coprocessor.%s.env' "$ENV_DIR" "$index"
  fi
}

instance_indexes() {
  local path name
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] || continue
    name="$(basename "$path")"
    if [[ "$name" == "coprocessor.env" ]]; then
      echo 0
    else
      echo "${name#coprocessor.}" | sed 's/\.env$//'
    fi
  done | sort -n
}

gpu_count() {
  nvidia-smi --query-gpu=index --format=csv,noheader | wc -l | tr -d ' '
}

write_host_env() {
  local index="$1" source="$2" target="$3" minio_ip
  minio_ip="$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' fhevm-minio)"
  [[ -n "$minio_ip" ]] || die "cannot determine fhevm-minio container IP"

  sed \
    -e 's|postgresql://postgres:postgres@db:5432/|postgresql://postgres:postgres@localhost:5432/|g' \
    -e "s|http://minio:9000|http://${minio_ip}:9000|g" \
    -e "s|http://[0-9.]*:9000|http://${minio_ip}:9000|g" \
    "$source" | sed -E '/^(DATABASE_URL|RPC_HTTP_URL|RPC_WS_URL|GATEWAY_URL|GATEWAY_WS_URL|AWS_ENDPOINT_URL)=/d' >"$target"
  cat >>"$target" <<EOF
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/$(grep '^DATABASE_URL=' "$source" | sed 's|.*/||')
RPC_HTTP_URL=http://localhost:8545
RPC_WS_URL=ws://localhost:8545
GATEWAY_URL=http://localhost:8546
GATEWAY_WS_URL=ws://localhost:8546
AWS_ENDPOINT_URL=http://${minio_ip}:9000
EOF
}

start_unit() {
  local kind="$1" index="$2" gpu="$3" env_file="$4" suffix="${5:-}" unit port_offset=0
  unit="$(unit_name "$kind" "$index" "$suffix")"
  [[ -z "$suffix" ]] || port_offset=200
  systemctl --user stop "$unit" >/dev/null 2>&1 || true

  local -a args
  case "$kind" in
    tfhe)
      args=(
        --run-bg-worker
        --database-url="$(grep '^DATABASE_URL=' "$env_file" | cut -d= -f2-)"
        --pg-pool-max-connections=10
        --worker-polling-interval-ms=1000
        --key-cache-size=32
        --dependence-chains-per-batch=1000
        --coprocessor-fhe-threads=16
        --tokio-threads=16
        --health-check-port=$((18080 + index * 10 + port_offset))
        --metrics-addr=0.0.0.0:$((19100 + index * 10 + port_offset))
      )
      ;;
    zkproof)
      args=(
        --database-url="$(grep '^DATABASE_URL=' "$env_file" | cut -d= -f2-)"
        --pg-listen-channel=event_zkpok_new_work
        --pg-notify-channel=event_zkpok_computed
        --pg-polling-interval=5
        --pg-pool-connections=5
        --worker-thread-count=4
        --health-check-port=$((18081 + index * 10 + port_offset))
        --metrics-addr=0.0.0.0:$((19101 + index * 10 + port_offset))
      )
      ;;
    sns)
      args=(
        --database-url="$(grep '^DATABASE_URL=' "$env_file" | cut -d= -f2-)"
        --pg-listen-channels event_pbs_computations event_ciphertext_computed
        --pg-notify-channel event_ciphertext128_computed
        --work-items-batch-size=1000
        --pg-polling-interval=30
        --pg-pool-connections=10
        --bucket-name-ct64=ct64
        --bucket-name-ct128=ct128
        --s3-max-concurrent-uploads=100
        --s3-max-retries-per-upload=100
        --s3-max-backoff=10s
        --s3-max-retries-timeout=120s
        --s3-recheck-duration=2s
        --s3-regular-recheck-duration=120s
        --enable-compression
        --signer-type=private-key
        --private-key="$(grep '^TX_SENDER_PRIVATE_KEY=' "$env_file" | cut -d= -f2-)"
        --health-check-port=$((18082 + index * 10 + port_offset))
        --metrics-addr=0.0.0.0:$((19102 + index * 10 + port_offset))
      )
      ;;
    *) die "unknown worker kind: $kind" ;;
  esac

  systemd-run --user --collect --unit="$unit" \
    --property=Restart=on-failure --property=RestartSec=2 \
    --property="EnvironmentFile=$env_file" \
    --setenv="CUDA_VISIBLE_DEVICES=$gpu" --setenv=RUST_BACKTRACE=1 \
    "${BIN_DIR}/${kind}_worker" "${args[@]}" >/dev/null
}

start() {
  require docker
  require nvidia-smi
  require systemd-run
  [[ -f "$ENV_DIR/coprocessor.env" ]] || die "missing active coprocessor environment"
  for binary in tfhe_worker zkproof_worker sns_worker; do
    [[ -x "$BIN_DIR/$binary" ]] || die "missing GPU binary: $BIN_DIR/$binary"
  done
  local gpus
  gpus="$(gpu_count)"
  [[ "$gpus" -gt 0 ]] || die "no NVIDIA GPU detected"
  mkdir -p "$GPU_RUNTIME_DIR"

  local layout="${GPU_WORKER_LAYOUT:-split}"
  case "$layout" in
    split|single-gpu1|dual-sns) ;;
    *) die "unknown GPU_WORKER_LAYOUT=$layout (expected split, single-gpu1, or dual-sns)" ;;
  esac
  [[ "$gpus" -ge 2 ]] || die "the benchmark GPU layouts require at least two NVIDIA GPUs"

  local index source host_env
  while IFS= read -r index; do
    source="$(env_file_for "$index")"
    host_env="$GPU_RUNTIME_DIR/coprocessor.${index}.env"
    write_host_env "$index" "$source" "$host_env"
    # Pin by worker kind rather than instance so every active coprocessor uses
    # the same benchmark layout.
    docker stop "$(container_name tfhe "$index")" "$(container_name zkproof "$index")" "$(container_name sns "$index")" >/dev/null 2>&1 || true
    for kind in tfhe zkproof sns; do
      systemctl --user stop "$(unit_name "$kind" "$index")" "$(unit_name "$kind" "$index" extra)" >/dev/null 2>&1 || true
    done
    case "$layout" in
      split)
        start_unit tfhe "$index" 0 "$host_env"
        start_unit zkproof "$index" 0 "$host_env"
        start_unit sns "$index" 1 "$host_env"
        ;;
      single-gpu1)
        start_unit tfhe "$index" 1 "$host_env"
        start_unit zkproof "$index" 1 "$host_env"
        start_unit sns "$index" 1 "$host_env"
        ;;
      dual-sns)
        start_unit tfhe "$index" 0 "$host_env"
        start_unit zkproof "$index" 0 "$host_env"
        start_unit sns "$index" 0 "$host_env"
        start_unit sns "$index" 1 "$host_env" extra
        ;;
    esac
  done < <(instance_indexes)
  status
}

stop() {
  local index kind
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      systemctl --user stop "$(unit_name "$kind" "$index")" "$(unit_name "$kind" "$index" extra)" >/dev/null 2>&1 || true
    done
  done < <(instance_indexes)
}

status() {
  local index kind
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      for suffix in "" extra; do
        systemctl --user show "$(unit_name "$kind" "$index" "$suffix")" \
          --property=Id --property=ActiveState --property=SubState --property=MainPID \
          2>/dev/null || true
      done
    done
  done < <(instance_indexes)
}

case "${1:-}" in
  start) start ;;
  stop) stop ;;
  status) status ;;
  *) usage; exit 2 ;;
esac
