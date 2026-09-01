#!/usr/bin/env bash
# Run the three-coprocessor materialization consensus gate with host-built GPU workers.
#
# Docker's ordinary local topology is deliberately CPU-only.  Merely setting
# CUDA_VISIBLE_DEVICES on those containers would not make their CPU binaries
# GPU binaries, and adding a Docker GPU override would make the source/runtime
# relationship harder to audit.  This launcher therefore follows the proven
# benchmark lifecycle: it leaves listeners, databases, KMS, and contracts in
# the source-matched public-runtime stack, stops *only* Docker worker consumers,
# and replaces them with GPU-feature host binaries using the same generated
# per-coprocessor environments.
#
# All three operators are pinned to one selected physical H100.  The consensus
# oracle requires byte equality only within an identical software/backend/
# hardware class.  Sharing one device is intentional here: it rules out a
# silent cross-device/hardware comparison while keeping each operator an
# independent process with an independent database and signer.
set -euo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly RUNTIME_DIR="${REPO_ROOT}/.fhevm/runtime"
readonly ENV_DIR="${RUNTIME_DIR}/env"
readonly GPU_RUNTIME_DIR="${RUNTIME_DIR}/gpu-consensus-workers"
readonly BIN_DIR="${REPO_ROOT}/coprocessor/fhevm-engine/target/release"
readonly DEVICE="${GPU_CONSENSUS_DEVICE:-0}"
readonly STREAMS_PER_DEVICE="${GPU_CONSENSUS_STREAMS_PER_DEVICE:-16}"
readonly COMPONENTS_PER_BATCH="${GPU_CONSENSUS_COMPONENTS_PER_BATCH:-20}"
readonly FHE_THREADS="${GPU_CONSENSUS_FHE_THREADS:-8}"
readonly TOKIO_THREADS="${GPU_CONSENSUS_TOKIO_THREADS:-4}"
readonly BUILD_MANIFEST="${GPU_RUNTIME_DIR}/build-manifest.env"
readonly DOCKER_WORKER_STATE_FILE="${GPU_RUNTIME_DIR}/docker-workers-to-restore"
readonly INVOCATION_DIR="${GPU_RUNTIME_DIR}/invocations"
GPU_TRANSITION_COMPLETE=false

usage() {
  cat <<'EOF'
Usage: test-suite/fhevm/scripts/gpu-consensus-workers.sh <build|start|stop|status|metadata|test-env|capture-activity>

Builds and runs source-matched GPU-feature TFHE, ZK-proof, and SNS workers for
the active three-coprocessor consensus topology. All operators use
GPU_CONSENSUS_DEVICE (default 0) so the test has one homogeneous H100 class.

Optional tuning variables:
  GPU_CONSENSUS_DEVICE=0
  GPU_CONSENSUS_STREAMS_PER_DEVICE=16
  GPU_CONSENSUS_COMPONENTS_PER_BATCH=20
  GPU_CONSENSUS_FHE_THREADS=8
  GPU_CONSENSUS_TOKIO_THREADS=4

`capture-activity [output-path] [seconds]` records nvidia-smi process mapping
and pmon utilization for the selected GPU. Start it in the background before
the one-transfer smoke, then retain its output alongside the test report.
EOF
}

die() {
  echo "gpu-consensus-workers: $*" >&2
  exit 1
}

require() {
  command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

unit_name() {
  local kind="$1" index="$2"
  printf 'fhevm-gpu-consensus-%s-%s' "$kind" "$index"
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

require_three_operator_topology() {
  local -a indexes
  mapfile -t indexes < <(instance_indexes)
  [[ "${indexes[*]}" == "0 1 2" ]] || die "expected active 3-of-3 environments (0 1 2), found: ${indexes[*]:-(none)}"
}

gpu_uuid() {
  nvidia-smi --id="$DEVICE" --query-gpu=uuid --format=csv,noheader | tr -d '[:space:]'
}

gpu_name() {
  nvidia-smi --id="$DEVICE" --query-gpu=name --format=csv,noheader | sed 's/^ *//;s/ *$//'
}

gpu_count() {
  nvidia-smi --query-gpu=index --format=csv,noheader | wc -l | tr -d ' '
}

binary_sha() {
  sha256sum "$1" | awk '{print $1}'
}

require_positive_integer() {
  local name="$1" value="$2"
  [[ "$value" =~ ^[1-9][0-9]*$ ]] || die "$name must be a positive integer (got $value)"
}

validate_tuning() {
  require_positive_integer GPU_CONSENSUS_STREAMS_PER_DEVICE "$STREAMS_PER_DEVICE"
  require_positive_integer GPU_CONSENSUS_COMPONENTS_PER_BATCH "$COMPONENTS_PER_BATCH"
  require_positive_integer GPU_CONSENSUS_FHE_THREADS "$FHE_THREADS"
  require_positive_integer GPU_CONSENSUS_TOKIO_THREADS "$TOKIO_THREADS"
}

require_clean_source() {
  # A manifest names a Git revision, not an arbitrary dirty tree.  Refuse to
  # label a binary as that revision if tracked or untracked source changes
  # could have participated in its build.  Runtime reports under `.fhevm` are
  # ignored by Git and therefore do not prevent a later GPU gate.
  [[ -z "$(git -C "$REPO_ROOT" status --porcelain)" ]] || die "source tree is dirty; commit or remove changes before producing consensus evidence"
}

write_manifest() {
  local revision
  revision="$(git -C "$REPO_ROOT" rev-parse HEAD)"
  umask 077
  mkdir -p "$GPU_RUNTIME_DIR"
  {
    printf 'software_revision=%q\n' "$revision"
    printf 'gpu_feature=%q\n' gpu
    printf 'cuda_path=%q\n' "${CUDA_PATH:-/usr/local/cuda}"
    printf 'cuda_visible_devices=%q\n' "$DEVICE"
    printf 'gpu_name=%q\n' "$(gpu_name)"
    printf 'gpu_uuid=%q\n' "$(gpu_uuid)"
    printf 'gpu_streams_per_device=%q\n' "$STREAMS_PER_DEVICE"
    printf 'components_per_batch=%q\n' "$COMPONENTS_PER_BATCH"
    printf 'coprocessor_fhe_threads=%q\n' "$FHE_THREADS"
    printf 'tokio_threads=%q\n' "$TOKIO_THREADS"
    printf 'pg_pool_max_connections=%q\n' 10
    printf 'tfhe_worker_sha256=%q\n' "$(binary_sha "$BIN_DIR/tfhe_worker")"
    printf 'zkproof_worker_sha256=%q\n' "$(binary_sha "$BIN_DIR/zkproof_worker")"
    printf 'sns_worker_sha256=%q\n' "$(binary_sha "$BIN_DIR/sns_worker")"
  } >"$BUILD_MANIFEST"
}

verify_build_manifest() {
  [[ -f "$BUILD_MANIFEST" ]] || die "missing GPU build manifest; run '$0 build' after the current source revision is committed"
  # shellcheck disable=SC1090
  source "$BUILD_MANIFEST"
  local revision
  revision="$(git -C "$REPO_ROOT" rev-parse HEAD)"
  [[ "${software_revision:-}" == "$revision" ]] || die "GPU binaries were built for ${software_revision:-unknown}, current source is $revision; rebuild"
  [[ "${gpu_feature:-}" == "gpu" ]] || die "build manifest does not prove the GPU feature"
  [[ "${cuda_visible_devices:-}" == "$DEVICE" ]] || die "GPU build manifest selects ${cuda_visible_devices:-unknown}, requested device is $DEVICE"
  [[ "${gpu_uuid:-}" == "$(gpu_uuid)" ]] || die "selected physical GPU changed since the GPU build; rebuild for auditable metadata"
  [[ "${tfhe_worker_sha256:-}" == "$(binary_sha "$BIN_DIR/tfhe_worker")" ]] || die "tfhe_worker differs from recorded GPU build"
  [[ "${zkproof_worker_sha256:-}" == "$(binary_sha "$BIN_DIR/zkproof_worker")" ]] || die "zkproof_worker differs from recorded GPU build"
  [[ "${sns_worker_sha256:-}" == "$(binary_sha "$BIN_DIR/sns_worker")" ]] || die "sns_worker differs from recorded GPU build"
}

build() {
  require cargo
  require nvidia-smi
  require_three_operator_topology
  require_clean_source
  validate_tuning
  [[ -x "${CUDA_PATH:-/usr/local/cuda}/bin/nvcc" ]] || die "nvcc is unavailable under CUDA_PATH=${CUDA_PATH:-/usr/local/cuda}"
  [[ "$DEVICE" =~ ^[0-9]+$ ]] || die "GPU_CONSENSUS_DEVICE must be a numeric CUDA device index"
  [[ "$DEVICE" -lt "$(gpu_count)" ]] || die "GPU_CONSENSUS_DEVICE=$DEVICE is not present"

  # Keep the exact feature selection in this script and in the persisted
  # manifest. Runtime CUDA_VISIBLE_DEVICES alone cannot turn a CPU build into
  # a GPU build, so the build itself is part of the consensus evidence.
  (
    cd "$REPO_ROOT/coprocessor/fhevm-engine"
    CUDA_PATH="${CUDA_PATH:-/usr/local/cuda}" \
      cargo build --release -p tfhe-worker -p zkproof-worker -p sns-worker --features gpu
  )
  for binary in tfhe_worker zkproof_worker sns_worker; do
    [[ -x "$BIN_DIR/$binary" ]] || die "GPU build did not produce $BIN_DIR/$binary"
  done
  write_manifest
  echo "gpu-consensus-workers: built source-matched GPU workers; metadata: $BUILD_MANIFEST"
}

write_host_env() {
  local index="$1" source="$2" target="$3" minio_ip database_name
  minio_ip="$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' fhevm-minio)"
  [[ -n "$minio_ip" ]] || die "cannot determine fhevm-minio container IP"
  database_name="$(grep '^DATABASE_URL=' "$source" | sed 's|.*/||')"
  [[ -n "$database_name" ]] || die "cannot determine database name from $source"

  umask 077
  sed \
    -e 's|postgresql://postgres:postgres@db:5432/|postgresql://postgres:postgres@localhost:5432/|g' \
    -e "s|http://minio:9000|http://${minio_ip}:9000|g" \
    -e "s|http://[0-9.]*:9000|http://${minio_ip}:9000|g" \
    "$source" | sed -E '/^(DATABASE_URL|RPC_HTTP_URL|RPC_WS_URL|GATEWAY_URL|GATEWAY_WS_URL|AWS_ENDPOINT_URL)=/d' >"$target"
  cat >>"$target" <<EOF
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/${database_name}
RPC_HTTP_URL=http://localhost:8545
RPC_WS_URL=ws://localhost:8545
GATEWAY_URL=http://localhost:8546
GATEWAY_WS_URL=ws://localhost:8546
AWS_ENDPOINT_URL=http://${minio_ip}:9000
EOF
}

start_unit() {
  local kind="$1" index="$2" env_file="$3" unit
  unit="$(unit_name "$kind" "$index")"
  systemctl --user stop "$unit" >/dev/null 2>&1 || true

  local -a args
  case "$kind" in
    tfhe)
      args=(
        --run-bg-worker
        --database-url="$(grep '^DATABASE_URL=' "$env_file" | cut -d= -f2-)"
        --pg-pool-max-connections=10
        --worker-polling-interval-ms=1000
        --work-items-batch-size=10
        --dependence-chains-per-batch="$COMPONENTS_PER_BATCH"
        --key-cache-size=32
        --coprocessor-fhe-threads="$FHE_THREADS"
        --gpu-streams-per-device="$STREAMS_PER_DEVICE"
        --tokio-threads="$TOKIO_THREADS"
        --health-check-port=$((18080 + index * 10))
        --metrics-addr=0.0.0.0:$((19100 + index * 10))
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
        --health-check-port=$((18081 + index * 10))
        --metrics-addr=0.0.0.0:$((19101 + index * 10))
      )
      ;;
    sns)
      args=(
        --database-url="$(grep '^DATABASE_URL=' "$env_file" | cut -d= -f2-)"
        --pg-listen-channels event_pbs_computations event_ciphertext_computed
        --pg-notify-channel event_ciphertext128_computed
        --work-items-batch-size=20
        --pg-polling-interval=30
        --pg-pool-connections=10
        --bucket-name="$(grep '^BUCKET_NAME=' "$env_file" | cut -d= -f2-)"
        --s3-max-concurrent-uploads=100
        --s3-max-retries-per-upload=100
        --s3-max-backoff=10s
        --s3-max-retries-timeout=120s
        --s3-recheck-duration=2s
        --s3-regular-recheck-duration=120s
        --enable-compression
        --signer-type=private-key
        --private-key="$(grep '^TX_SENDER_PRIVATE_KEY=' "$env_file" | cut -d= -f2-)"
        --health-check-port=$((18082 + index * 10))
        --metrics-addr=0.0.0.0:$((19102 + index * 10))
      )
      ;;
    *) die "unknown worker kind: $kind" ;;
  esac

  systemd-run --user --collect --unit="$unit" \
    --property=Restart=on-failure --property=RestartSec=2 \
    --property="EnvironmentFile=$env_file" \
    --setenv="CUDA_VISIBLE_DEVICES=$DEVICE" --setenv="FHEVM_GPU_STREAMS_PER_DEVICE=$STREAMS_PER_DEVICE" --setenv=RUST_BACKTRACE=1 \
    "${BIN_DIR}/${kind}_worker" "${args[@]}" >/dev/null
}

stop_host_workers() {
  local index kind
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      systemctl --user stop "$(unit_name "$kind" "$index")" >/dev/null 2>&1 || true
    done
  done < <(instance_indexes)
}

record_running_docker_workers() {
  local index kind container running
  umask 077
  : >"$DOCKER_WORKER_STATE_FILE"
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      container="$(container_name "$kind" "$index")"
      # Do not treat an unknown inspect result as "stopped".  Otherwise a
      # renamed/missing Docker container could still consume work while this
      # launcher starts a host worker for the same database queue.
      running="$(docker inspect -f '{{.State.Running}}' "$container")" || die "cannot inspect expected Docker worker $container"
      if [[ "$running" == "true" ]]; then
        printf '%s\n' "$container" >>"$DOCKER_WORKER_STATE_FILE"
      fi
    done
  done < <(instance_indexes)
}

stop_recorded_docker_workers() {
  local container
  [[ -f "$DOCKER_WORKER_STATE_FILE" ]] || return 0
  while IFS= read -r container; do
    [[ -n "$container" ]] || continue
    docker stop "$container" >/dev/null
  done <"$DOCKER_WORKER_STATE_FILE"
}

restore_recorded_docker_workers() {
  local container
  [[ -f "$DOCKER_WORKER_STATE_FILE" ]] || return 0
  while IFS= read -r container; do
    [[ -n "$container" ]] || continue
    docker start "$container" >/dev/null
  done <"$DOCKER_WORKER_STATE_FILE"
  rm -f "$DOCKER_WORKER_STATE_FILE"
}

ensure_no_active_gpu_session() {
  local index kind unit state
  [[ ! -e "$DOCKER_WORKER_STATE_FILE" ]] || die "an earlier GPU session is still active or was not restored; run '$0 stop' before starting again"
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      unit="$(unit_name "$kind" "$index")"
      state="$(systemctl --user show "$unit" --property=ActiveState --value 2>/dev/null || true)"
      [[ "$state" != "active" ]] || die "GPU unit $unit is already active; run '$0 stop' before starting again"
    done
  done < <(instance_indexes)
}

tfhe_health_port() {
  local index="$1"
  echo $((18080 + index * 10))
}

tfhe_gpu_log_ready() {
  local index="$1" unit invocation_file invocation_id logs
  unit="$(unit_name tfhe "$index")"
  invocation_file="${INVOCATION_DIR}/${unit}"
  [[ -s "$invocation_file" ]] || return 1
  invocation_id="$(<"$invocation_file")"
  logs="$(journalctl --user _SYSTEMD_INVOCATION_ID="$invocation_id" --no-pager -o cat 2>/dev/null || true)"
  # `log_backend` is emitted by the current TFHE worker immediately before it
  # starts its work loop.  This is deliberately checked in addition to
  # CUDA_VISIBLE_DEVICES: the latter expresses placement intent, while this
  # line proves the running binary selected the GPU backend.
  grep -Eq 'gpu_enabled["=:[:space:]]+true' <<<"$logs"
}

record_unit_invocation() {
  local unit="$1" invocation_id
  mkdir -p "$INVOCATION_DIR"
  for _ in {1..20}; do
    invocation_id="$(systemctl --user show "$unit" --property=InvocationID --value 2>/dev/null || true)"
    if [[ -n "$invocation_id" && "$invocation_id" != "00000000000000000000000000000000" ]]; then
      printf '%s\n' "$invocation_id" >"${INVOCATION_DIR}/${unit}"
      return 0
    fi
    sleep 0.1
  done
  die "could not record systemd invocation ID for $unit"
}

wait_for_units() {
  local deadline=$((SECONDS + 60)) index kind unit state port
  while (( SECONDS < deadline )); do
    local all_running=true
    while IFS= read -r index; do
      for kind in tfhe zkproof sns; do
        unit="$(unit_name "$kind" "$index")"
        state="$(systemctl --user show "$unit" --property=ActiveState --value 2>/dev/null || true)"
        [[ "$state" == "active" ]] || all_running=false
      done
      port="$(tfhe_health_port "$index")"
      curl --fail --silent --show-error --max-time 2 "http://127.0.0.1:${port}/healthz" >/dev/null || all_running=false
      tfhe_gpu_log_ready "$index" || all_running=false
    done < <(instance_indexes)
    if "$all_running"; then
      return 0
    fi
    sleep 1
  done
  return 1
}

start() {
  require docker
  require nvidia-smi
  require systemctl
  require systemd-run
  require sha256sum
  require curl
  require_three_operator_topology
  require_clean_source
  validate_tuning
  verify_build_manifest
  mkdir -p "$GPU_RUNTIME_DIR"
  ensure_no_active_gpu_session

  # Do not let two implementations consume the same queue.  Stopping only
  # these worker roles preserves the live source-matched listeners, DBs, KMS
  # material, contracts, and test-suite state that the CPU gate already proved.
  # From this point the transition is transactional.  If preparation, a
  # Docker stop, systemd startup, or readiness proof fails, the EXIT trap
  # stops any partial GPU set and restores only containers that this launcher
  # observed running before it changed anything.  It never revives a worker an
  # operator had intentionally left down.
  GPU_TRANSITION_COMPLETE=false
  trap 'if [[ "$GPU_TRANSITION_COMPLETE" != true ]]; then stop_host_workers; restore_recorded_docker_workers; fi' EXIT
  record_running_docker_workers
  stop_host_workers
  local index kind source host_env
  while IFS= read -r index; do
    source="$(env_file_for "$index")"
    host_env="$GPU_RUNTIME_DIR/coprocessor.${index}.env"
    write_host_env "$index" "$source" "$host_env"
  done < <(instance_indexes)
  stop_recorded_docker_workers

  if ! while IFS= read -r index; do
    host_env="$GPU_RUNTIME_DIR/coprocessor.${index}.env"
    start_unit tfhe "$index" "$host_env"
    record_unit_invocation "$(unit_name tfhe "$index")"
    start_unit zkproof "$index" "$host_env"
    record_unit_invocation "$(unit_name zkproof "$index")"
    start_unit sns "$index" "$host_env"
    record_unit_invocation "$(unit_name sns "$index")"
  done < <(instance_indexes); then
    die "failed to start host GPU workers; restored Docker worker consumers"
  fi

  if ! wait_for_units; then
    status >&2 || true
    die "GPU workers did not become active; restored Docker worker consumers"
  fi
  GPU_TRANSITION_COMPLETE=true
  trap - EXIT
  echo "gpu-consensus-workers: all 3 operators run on $(gpu_name) ($(gpu_uuid)), CUDA device $DEVICE"
}

status() {
  local index kind
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      systemctl --user show "$(unit_name "$kind" "$index")" \
        --property=Id --property=ActiveState --property=SubState --property=MainPID \
        2>/dev/null || true
    done
  done < <(instance_indexes)
}

metadata() {
  [[ -f "$BUILD_MANIFEST" ]] || die "missing GPU build manifest"
  cat "$BUILD_MANIFEST"
}

test_env() {
  require_clean_source
  verify_build_manifest
  # Quote every value so callers can safely use `eval "$(... test-env)"` to
  # feed the Hardhat container only metadata derived from the audited build
  # manifest, rather than manually asserted CPU/GPU labels.
  # shellcheck disable=SC1090
  source "$BUILD_MANIFEST"
  printf 'export CONSENSUS_SOFTWARE_REVISION=%q\n' "$software_revision"
  printf 'export CONSENSUS_BACKEND_CLASS=%q\n' "gpu-cuda"
  printf 'export CONSENSUS_HARDWARE_CLASS=%q\n' "gpu-homogeneous-${gpu_name// /_}-${gpu_uuid}"
  printf 'export CONSENSUS_GPU_BUILD_MANIFEST_SHA256=%q\n' "$(binary_sha "$BUILD_MANIFEST")"
}

capture_activity() {
  require nvidia-smi
  local output="${2:-${GPU_RUNTIME_DIR}/nvidia-smi-pmon-$(date -u +%Y%m%dT%H%M%SZ).log}"
  local seconds="${3:-120}"
  [[ "$seconds" =~ ^[1-9][0-9]*$ ]] || die "activity capture duration must be a positive integer"
  mkdir -p "$(dirname "$output")"
  {
    printf '# captured_at=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf '# selected_gpu_index=%s\n' "$DEVICE"
    printf '# selected_gpu_name=%s\n' "$(gpu_name)"
    printf '# selected_gpu_uuid=%s\n' "$(gpu_uuid)"
    printf '# host GPU worker PIDs at capture start\n'
    status
    printf '# nvidia-smi compute-process mapping\n'
    nvidia-smi --id="$DEVICE" --query-compute-apps=pid,process_name,used_gpu_memory --format=csv,noheader || true
    printf '# nvidia-smi pmon (utilization and memory), %s one-second samples\n' "$seconds"
    nvidia-smi pmon -i "$DEVICE" -s um -d 1 -c "$seconds"
  } >"$output"
  echo "gpu-consensus-workers: wrote GPU activity capture to $output"
}

case "${1:-}" in
  build) build ;;
  start) start ;;
  stop) stop_host_workers; restore_recorded_docker_workers ;;
  status) status ;;
  metadata) metadata ;;
  test-env) test_env ;;
  capture-activity) capture_activity "$@" ;;
  *) usage; exit 2 ;;
esac
