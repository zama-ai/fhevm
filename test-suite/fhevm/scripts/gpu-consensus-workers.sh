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
readonly WORK_ITEMS_BATCH_SIZE="${GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE:-100}"
readonly FHE_THREADS="${GPU_CONSENSUS_FHE_THREADS:-8}"
readonly TOKIO_THREADS="${GPU_CONSENSUS_TOKIO_THREADS:-4}"
readonly BUILD_MANIFEST="${GPU_RUNTIME_DIR}/build-manifest.env"
readonly NODE_CONFIG="${GPU_RUNTIME_DIR}/node-config.env"
readonly DOCKER_WORKER_STATE_FILE="${GPU_RUNTIME_DIR}/docker-workers-to-restore"
readonly INVOCATION_DIR="${GPU_RUNTIME_DIR}/invocations"
GPU_TRANSITION_COMPLETE=false

usage() {
  cat <<'EOF'
Usage: test-suite/fhevm/scripts/gpu-consensus-workers.sh <preflight|build|start|stop|restart-unit|status|conflicts|metadata|test-env|capture-activity>

Builds and runs source-matched GPU-feature TFHE, ZK-proof, and SNS workers for
the active three-coprocessor consensus topology. All operators use
GPU_CONSENSUS_DEVICE (default 0) so the test has one homogeneous H100 class.

Optional tuning variables:
  GPU_CONSENSUS_DEVICE=0
  GPU_CONSENSUS_STREAMS_PER_DEVICE=16
  GPU_CONSENSUS_COMPONENTS_PER_BATCH=20
  GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE=100
  GPU_CONSENSUS_FHE_THREADS=8
  GPU_CONSENSUS_TOKIO_THREADS=4

Any of those may be overridden for a single operator by appending its index:
GPU_CONSENSUS_<KNOB>_<index>, e.g. GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE_1=1.
Two boolean knobs exist only in per-node form, because the worker reads them
from the environment rather than the command line:
  GPU_CONSENSUS_ADAPTIVE_BATCH_EXECUTION_<index>=true|false
  GPU_CONSENSUS_BATCH_EXECUTION_<index>=true|false

Deliberately heterogeneous scheduling is a determinism axis, not a
misconfiguration: RFC 020 makes result bytes a function of on-chain data alone,
so three operators scheduling differently must still agree byte for byte. With
no per-node override set the fleet is homogeneous and the run is unchanged.

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

# systemd --user is addressed through this user's own runtime directory. A
# detached or re-parented shell can inherit another user's values -- observed as
# XDG_RUNTIME_DIR=/run/user/0 while running as uid 1000 -- and then every
# systemctl call fails with "Failed to connect to bus: Permission denied", which
# reads as a permissions problem with the units rather than a wrong address. It
# cost a GPU leg 20 minutes of bring-up before failing at the swap. Derive the
# address from the running uid instead of trusting the environment.
ensure_user_bus() {
  local uid runtime
  uid="$(id -u)"
  runtime="/run/user/${uid}"
  [[ -d "$runtime" ]] ||
    die "no user runtime directory at $runtime, so systemd --user is unavailable for uid $uid (enable lingering: loginctl enable-linger $(id -un))"
  export XDG_RUNTIME_DIR="$runtime"
  export DBUS_SESSION_BUS_ADDRESS="unix:path=${runtime}/bus"
  systemctl --user is-system-running >/dev/null 2>&1 ||
    die "systemd --user is not reachable at $DBUS_SESSION_BUS_ADDRESS (enable lingering: loginctl enable-linger $(id -un))"
}

gpu_uuid() {
  gpu_uuid_of "$DEVICE"
}

gpu_name() {
  gpu_name_of "$DEVICE"
}

gpu_uuid_of() {
  nvidia-smi --id="$1" --query-gpu=uuid --format=csv,noheader | tr -d '[:space:]'
}

gpu_name_of() {
  nvidia-smi --id="$1" --query-gpu=name --format=csv,noheader | sed 's/^ *//;s/ *$//'
}

# Every CUDA device the fleet actually uses, ascending.
#
# `$DEVICE` is only the fleet default: GPU_CONSENSUS_DEVICE_<index> can put an
# operator on another card, and the resolved per-node values are recorded in the
# node config. Reading them back is the only way to describe the fleet honestly.
device_set() {
  if [[ -f "$NODE_CONFIG" ]]; then
    sed -n 's/^operator_[0-9]\+_device=//p' "$NODE_CONFIG" | tr -d "'\"" | sort -u -n
  else
    printf '%s\n' "$DEVICE"
  fi
}

# The hardware class names the devices in use, not the default one.
#
# This used to be emitted as `gpu-homogeneous-<name>-<uuid>` unconditionally,
# with the name and UUID read from `--id="$DEVICE"`. A fleet split across two
# cards was therefore attested as homogeneous on the first card's UUID. The class
# is RFC-023 evidence, so a confidently wrong label is worse than a missing one:
# a reader cannot tell a genuinely single-GPU run from a split one, and the byte
# oracle for "same hardware" would be applied to a fleet that had none.
hardware_class() {
  local -a devices=() names=() uuids=()
  local d
  while IFS= read -r d; do
    [[ -n "$d" ]] || continue
    devices+=("$d")
    names+=("$(gpu_name_of "$d" | tr ' ' '_')")
    uuids+=("$(gpu_uuid_of "$d")")
  done < <(device_set)
  [[ "${#devices[@]}" -gt 0 ]] || { printf 'gpu-unknown'; return 0; }
  local name_set uuid_join
  name_set="$(printf '%s\n' "${names[@]}" | sort -u | paste -sd'+' -)"
  uuid_join="$(printf '%s\n' "${uuids[@]}" | paste -sd'+' -)"
  if [[ "${#devices[@]}" -eq 1 ]]; then
    printf 'gpu-homogeneous-%s-%s' "$name_set" "$uuid_join"
  else
    printf 'gpu-split-%s-%s' "$name_set" "$uuid_join"
  fi
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

# Resolve one knob for one operator: GPU_CONSENSUS_<KNOB>_<index> when set,
# otherwise the fleet-wide value.  Printing nothing for an unset knob with no
# fleet default is meaningful -- callers use it to mean "leave the binary's own
# default alone" rather than forcing a value.
node_tuning() {
  local knob="$1" index="$2" fleet="$3" name
  name="GPU_CONSENSUS_${knob}_${index}"
  printf '%s' "${!name:-$fleet}"
}

# Validate per-node overrides wherever they appear, without depending on a live
# topology, so `build` rejects a typo as readily as `start` does.
validate_node_overrides() {
  local name value knob
  while IFS= read -r name; do
    value="${!name}"
    [[ -n "$value" ]] || continue
    knob="${name#GPU_CONSENSUS_}"
    knob="${knob%_*}"
    case "$knob" in
      STREAMS_PER_DEVICE | COMPONENTS_PER_BATCH | WORK_ITEMS_BATCH_SIZE | FHE_THREADS | TOKIO_THREADS)
        require_positive_integer "$name" "$value"
        ;;
      DEVICE)
        [[ "$value" =~ ^[0-9]+$ ]] || die "$name must be a GPU index (got $value)"
        ;;
      ADAPTIVE_BATCH_EXECUTION | BATCH_EXECUTION)
        [[ "$value" == true || "$value" == false ]] ||
          die "$name must be true or false (got $value)"
        ;;
      *) die "unknown per-node override $name" ;;
    esac
  done < <(compgen -v | grep -E '^GPU_CONSENSUS_[A-Z_]+_[0-9]+$' || true)

  # `validate_tuning` refuses a fleet-wide batch/chain pair that inverts,
  # because the adaptive work window then turns itself off at runtime and the
  # run measures non-adaptive scheduling while claiming to measure the shipped
  # configuration. Per-node overrides can invert that pair for a single
  # operator, which is the same fault one node at a time -- and here it is
  # worse, since a deliberately heterogeneous run is exactly where a silently
  # non-adaptive node would be read as evidence about adaptive scheduling.
  local index items chains
  while IFS= read -r index; do
    items="$(node_tuning WORK_ITEMS_BATCH_SIZE "$index" "$WORK_ITEMS_BATCH_SIZE")"
    chains="$(node_tuning COMPONENTS_PER_BATCH "$index" "$COMPONENTS_PER_BATCH")"
    if (( items < chains )); then
      die "operator $index resolves work-items-batch-size ($items) below dependence-chains-per-batch ($chains); an inverted pair disables the adaptive work window at runtime"
    fi
  done < <(instance_indexes)
}

validate_tuning() {
  require_positive_integer GPU_CONSENSUS_STREAMS_PER_DEVICE "$STREAMS_PER_DEVICE"
  require_positive_integer GPU_CONSENSUS_COMPONENTS_PER_BATCH "$COMPONENTS_PER_BATCH"
  require_positive_integer GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE "$WORK_ITEMS_BATCH_SIZE"
  # The adaptive work window gives each acquired chain
  # ceil(work-items-batch-size / acquired-chains) transactions and turns itself
  # OFF at runtime once more chains are acquired than the window admits.  An
  # inverted pair therefore measures non-adaptive scheduling while claiming to
  # measure the shipped configuration -- silently, and only under enough load
  # to fill the batch.  Refuse the pair rather than record a misleading run.
  if (( WORK_ITEMS_BATCH_SIZE < COMPONENTS_PER_BATCH )); then
    die "GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE ($WORK_ITEMS_BATCH_SIZE) must be >= GPU_CONSENSUS_COMPONENTS_PER_BATCH ($COMPONENTS_PER_BATCH); an inverted pair disables the adaptive work window at runtime"
  fi
  require_positive_integer GPU_CONSENSUS_FHE_THREADS "$FHE_THREADS"
  require_positive_integer GPU_CONSENSUS_TOKIO_THREADS "$TOKIO_THREADS"
  validate_node_overrides
}

# The one generated file that is tracked, and why it is excluded below.
#
# `E2ECoprocessorConfigLocal.sol` is rendered per stack from discovered
# addresses, with a `block.chainid` branch per host chain.  It is nonetheless
# tracked, because a dozen e2e test contracts import it by relative path and
# `test-suite/e2e/Dockerfile` runs `npx hardhat compile` at image build time --
# un-tracking it would break any image build that has not generated first.
readonly GENERATED_TRACKED_SOURCE="test-suite/e2e/contracts/E2ECoprocessorConfigLocal.sol"

require_clean_source() {
  # A manifest names a Git revision, not an arbitrary dirty tree.  Refuse to
  # label a binary as that revision if tracked or untracked source changes
  # could have participated in its build.  Runtime reports under `.fhevm` are
  # ignored by Git and therefore do not prevent a later GPU gate.
  #
  # One documented exception: the generated address config above.  It is
  # test-contract configuration, not workspace source -- it cannot participate
  # in building a coprocessor binary, so it cannot invalidate the revision this
  # manifest names.  Without the exception every freshly booted stack needed a
  # disposable commit before a GPU gate could run, which is a branch-surgery
  # step on every consensus campaign.  The manifest records whether it applied,
  # so the exception is visible in the evidence rather than assumed.
  local dirty
  dirty="$(git -C "$REPO_ROOT" status --porcelain -- ":(exclude)$GENERATED_TRACKED_SOURCE")"
  [[ -n "$dirty" ]] || return 0
  # Separate the two cases, because they need different actions and the message
  # used to be identical for both. A leg once died 522s into a bring-up over two
  # directories of stale Solidity artifacts from contracts deleted upstream, and
  # the message read as "you have uncommitted work" (F-5).
  local untracked modified
  untracked="$(grep '^?? ' <<<"$dirty" | sed 's/^?? //')"
  modified="$(grep -v '^?? ' <<<"$dirty")"
  {
    echo "source tree is dirty; a manifest names a revision, so consensus evidence cannot be built from it."
    [[ -z "$modified" ]] || { echo "  tracked files modified -- commit or stash:"; sed 's/^/    /' <<<"$modified"; }
    [[ -z "$untracked" ]] || {
      echo "  untracked paths -- often build output that no longer belongs to any target:"
      sed 's/^/    /' <<<"$untracked"
      echo "  (if they are build droppings, delete them; if they are new source, commit or ignore them)"
    }
  } >&2
  exit 1
}

# Did the generated address config differ from the committed revision?
generated_source_state() {
  if [[ -n "$(git -C "$REPO_ROOT" status --porcelain -- "$GENERATED_TRACKED_SOURCE")" ]]; then
    printf 'regenerated-for-this-stack'
  else
    printf 'matches-revision'
  fi
}

write_manifest() {
  local revision
  revision="$(git -C "$REPO_ROOT" rev-parse HEAD)"
  umask 077
  mkdir -p "$GPU_RUNTIME_DIR"
  {
    printf 'software_revision=%q\n' "$revision"
    printf 'e2e_address_config=%q\n' "$(generated_source_state)"
    printf 'gpu_feature=%q\n' gpu
    printf 'cuda_path=%q\n' "${CUDA_PATH:-/usr/local/cuda}"
    printf 'cuda_visible_devices=%q\n' "$DEVICE"
    printf 'gpu_name=%q\n' "$(gpu_name)"
    printf 'gpu_uuid=%q\n' "$(gpu_uuid)"
    printf 'gpu_streams_per_device=%q\n' "$STREAMS_PER_DEVICE"
    printf 'components_per_batch=%q\n' "$COMPONENTS_PER_BATCH"
    printf 'work_items_batch_size=%q\n' "$WORK_ITEMS_BATCH_SIZE"
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
  local kind="$1" index="$2" env_file="$3" unit device streams
  unit="$(unit_name "$kind" "$index")"
  device="$(node_tuning DEVICE "$index" "$DEVICE")"
  streams="$(node_tuning STREAMS_PER_DEVICE "$index" "$STREAMS_PER_DEVICE")"
  systemctl --user stop "$unit" >/dev/null 2>&1 || true

  local -a args scheduling_env=()
  case "$kind" in
    tfhe)
      args=(
        --run-bg-worker
        --database-url="$(grep '^DATABASE_URL=' "$env_file" | cut -d= -f2-)"
        --pg-pool-max-connections=10
        --worker-polling-interval-ms=1000
        --work-items-batch-size="$(node_tuning WORK_ITEMS_BATCH_SIZE "$index" "$WORK_ITEMS_BATCH_SIZE")"
        --dependence-chains-per-batch="$(node_tuning COMPONENTS_PER_BATCH "$index" "$COMPONENTS_PER_BATCH")"
        --key-cache-size=32
        --coprocessor-fhe-threads="$(node_tuning FHE_THREADS "$index" "$FHE_THREADS")"
        --gpu-streams-per-device="$streams"
        --tokio-threads="$TOKIO_THREADS"
        --health-check-port=$((18080 + index * 10))
        --metrics-addr=0.0.0.0:$((19100 + index * 10))
      )
      # `--dcid-adaptive-batch-execution` and `--dcid-batch-execution` are
      # clap flags: passing one can only turn it ON, so an operator that must
      # run with it OFF can only be configured through the environment.
      local knob value
      for knob in ADAPTIVE_BATCH_EXECUTION BATCH_EXECUTION; do
        value="$(node_tuning "$knob" "$index" "")"
        if [[ -n "$value" ]]; then
          scheduling_env+=(--setenv="FHEVM_DCID_${knob}=$value")
        fi
      done
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
    --setenv="CUDA_VISIBLE_DEVICES=$device" --setenv="FHEVM_GPU_STREAMS_PER_DEVICE=$streams" --setenv=RUST_BACKTRACE=1 \
    "${scheduling_env[@]}" \
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

# Refuse to swap away a heterogeneity the units will not inherit.
#
# The scenario expresses per-operator scheduling as *instance args* on the
# compose workers. `node_tuning` reads GPU_CONSENSUS_<KNOB>_<index> from the
# environment and knows nothing about them, so swapping silently replaces a
# deliberately heterogeneous fleet with a uniform one -- and the gate only says
# so after a full bring-up, a build and a swap: "every operator scheduled
# identically" (F-8). The compose workers are still running at this point, so
# their flags can be read and compared.
require_scenario_tuning_carried() {
  local index container cmd win chains
  local -A wins=() chainses=()
  while IFS= read -r index; do
    container="$(container_name tfhe "$index")"
    cmd="$(docker inspect --format '{{range .Config.Cmd}}{{println .}}{{end}}' "$container" 2>/dev/null)" || continue
    win="$(sed -n 's/^--work-items-batch-size=//p' <<<"$cmd" | tail -1)"
    chains="$(sed -n 's/^--dependence-chains-per-batch=//p' <<<"$cmd" | tail -1)"
    wins[$index]="${win:-default}"
    chainses[$index]="${chains:-default}"
  done < <(instance_indexes)

  local distinct_wins distinct_chains
  distinct_wins="$(printf '%s\n' "${wins[@]}" | sort -u | grep -c . || true)"
  distinct_chains="$(printf '%s\n' "${chainses[@]}" | sort -u | grep -c . || true)"
  [[ "${distinct_wins:-1}" -gt 1 || "${distinct_chains:-1}" -gt 1 ]] || return 0

  # The scenario is heterogeneous. Every operator that differs needs its own
  # override, or that operator's unit silently takes the fleet default.
  local missing="" name
  while IFS= read -r index; do
    name="GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE_${index}"
    [[ -n "${!name:-}" ]] || missing+=" ${name}=${wins[$index]}"
    name="GPU_CONSENSUS_COMPONENTS_PER_BATCH_${index}"
    [[ -n "${!name:-}" ]] || missing+=" ${name}=${chainses[$index]}"
  done < <(instance_indexes)
  [[ -n "$missing" ]] || return 0

  die "this scenario schedules operators differently, but the host units would not inherit it:
the compose workers run$(for i in "${!wins[@]}"; do printf ' [%s]=%s/%s' "$i" "${wins[$i]}" "${chainses[$i]}"; done)
and node_tuning reads only the environment. Export the missing overrides and start again:
 export$missing
Swapping without them replaces a heterogeneous fleet with a uniform one, and the gate reports
'every operator scheduled identically' only after the build and swap have already been paid for."
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

# The node config is the marker that says "host GPU workers are serving this
# stack": run-materialization-consensus.sh keys its GPU detection off the file's
# existence. Left behind after a stop, it makes every later suite -- on any
# topology, GPU or not -- believe host workers are in play, and the build
# manifest's revision check then aborts them all the moment anything is
# committed. That is how a fork-topology run died in 0s reporting a GPU revision
# mismatch. The build manifest is deliberately kept: it is build evidence and
# what makes a rebuild cacheable. This file is session state, not evidence.
clear_node_config() {
  rm -f "$NODE_CONFIG"
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

# Bring one worker role back after it was stopped.
#
# `systemctl start` cannot do this: the units are transient
# (`systemd-run --collect`), so stopping one garbage-collects it and the name no
# longer resolves -- `Unit fhevm-gpu-consensus-tfhe-2.service not found`.
# Restoring needs the original invocation: the generated environment file, the
# CUDA device, the stream count and the per-node tuning, none of which a caller
# outside this script has.
#
# Without it, a suite that takes an operator offline cannot put it back, and
# every later suite on the same stack silently runs an operator short. That is
# not a hypothetical: the degraded suite's C4a stopped operator 2's three units
# and the fleet stayed at six units for the rest of the leg, so crash-retry and
# the failure matrix failed for want of a third submitter rather than for
# anything they were testing.
restart_unit() {
  local kind="${1:?restart-unit needs a kind: tfhe|zkproof|sns}"
  local index="${2:?restart-unit needs an operator index}"
  case "$kind" in tfhe | zkproof | sns) ;; *) die "unknown worker kind: $kind" ;; esac
  local host_env="$GPU_RUNTIME_DIR/coprocessor.${index}.env"
  [[ -f "$host_env" ]] ||
    die "no generated environment for operator $index at $host_env; restart-unit only works inside a GPU session started by this script"
  start_unit "$kind" "$index" "$host_env"
  record_unit_invocation "$(unit_name "$kind" "$index")"
  echo "gpu-consensus-workers: restarted $(unit_name "$kind" "$index")"
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
  require_scenario_tuning_carried
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
  write_node_config
  echo "gpu-consensus-workers: all 3 operators run on $(gpu_name) ($(gpu_uuid)), CUDA device $DEVICE"
  if ! fleet_is_homogeneous; then
    echo "gpu-consensus-workers: per-operator scheduling overrides are ACTIVE; resolved configuration recorded in $NODE_CONFIG"
    grep -v '^homogeneous=' "$NODE_CONFIG" | sed 's/^/  /'
  fi
}

# Every worker kind here claims rows from one queue per operator database with
# `FOR UPDATE SKIP LOCKED`.  That is exactly right for one worker and silently
# wrong for two: each row is served by whichever process won it, so a host unit
# and a Docker container running side by side split the queue between two
# *different builds*.  For the SNS worker that means one operator holding a mix
# of CPU-squashed and GPU-squashed ct128 for handles whose ct64 is identical --
# indistinguishable from a consensus defect, and it cost a full investigation to
# tell apart from one (Consensus Defect Log, B-1/L-6).  For the TFHE worker it
# would diverge ct64 itself.
#
# `start` already stops the containers it displaces.  The reverse direction is
# the gap: these units are transient with `Restart=on-failure`, so they outlive
# a stack teardown and restart themselves, and the next `fhevm-cli up` brings
# the containers back underneath them.  Nothing in either component notices, so
# report it here.
conflicts() {
  local index kind unit container found=0
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      unit="$(unit_name "$kind" "$index")"
      container="$(container_name "$kind" "$index")"
      [[ "$(systemctl --user show "$unit" --property=ActiveState --value 2>/dev/null)" == "active" ]] || continue
      [[ "$(docker inspect -f '{{.State.Status}}' "$container" 2>/dev/null)" == "running" ]] || continue
      printf 'CONFLICT operator=%s kind=%s unit=%s container=%s: both are serving the same queue\n' \
        "$index" "$kind" "$unit" "$container"
      found=1
    done
  done < <(instance_indexes)
  return "$found"
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

  # `status` is what someone runs when the stack is behaving oddly, so it must
  # not report three healthy units while the containers are double-writing.
  if ! conflicts; then
    printf '\nRun `%s stop` to hand the queues back to the containers, or stop the\n' "$0"
    printf 'containers if the host workers are the ones you want.\n'
    return 1
  fi
}

# The build manifest describes the binaries; this describes how they were
# scheduled.  A heterogeneous-configuration run is only evidence if the
# heterogeneity is recorded -- otherwise a green result is indistinguishable
# from one where the overrides were never picked up.
write_node_config() {
  local index
  umask 077
  mkdir -p "$GPU_RUNTIME_DIR"
  {
    printf 'homogeneous=%q\n' "$(fleet_is_homogeneous && echo true || echo false)"
    while IFS= read -r index; do
      printf 'operator_%s_device=%q\n' "$index" "$(node_tuning DEVICE "$index" "$DEVICE")"
      printf 'operator_%s_gpu_streams_per_device=%q\n' "$index" "$(node_tuning STREAMS_PER_DEVICE "$index" "$STREAMS_PER_DEVICE")"
      printf 'operator_%s_work_items_batch_size=%q\n' "$index" "$(node_tuning WORK_ITEMS_BATCH_SIZE "$index" "$WORK_ITEMS_BATCH_SIZE")"
      printf 'operator_%s_dependence_chains_per_batch=%q\n' "$index" "$(node_tuning COMPONENTS_PER_BATCH "$index" "$COMPONENTS_PER_BATCH")"
      printf 'operator_%s_coprocessor_fhe_threads=%q\n' "$index" "$(node_tuning FHE_THREADS "$index" "$FHE_THREADS")"
      printf 'operator_%s_adaptive_batch_execution=%q\n' "$index" "$(node_tuning ADAPTIVE_BATCH_EXECUTION "$index" default)"
      printf 'operator_%s_batch_execution=%q\n' "$index" "$(node_tuning BATCH_EXECUTION "$index" default)"
    done < <(instance_indexes)
  } >"$NODE_CONFIG"
}

fleet_is_homogeneous() {
  compgen -v | grep -qE '^GPU_CONSENSUS_[A-Z_]+_[0-9]+$' && return 1
  return 0
}

# One canonical class string per operator, derived from what `start` actually
# resolved rather than from the current shell -- `test-env` usually runs in a
# different shell from `start`, where the override variables are long gone.
# The consensus gate compares these for distinctness: a run that claims
# heterogeneous scheduling but whose operators share a class is a vacuous pass.
scheduling_classes() {
  [[ -f "$NODE_CONFIG" ]] || return 1
  local index first=true out="" key
  # shellcheck disable=SC1090
  source "$NODE_CONFIG"
  while IFS= read -r index; do
    [[ "$first" == true ]] || out+=";"
    first=false
    out+="${index}="
    key="operator_${index}_device";                       out+="device:${!key}"
    key="operator_${index}_gpu_streams_per_device";       out+=",streams:${!key}"
    key="operator_${index}_work_items_batch_size";        out+=",window:${!key}"
    key="operator_${index}_dependence_chains_per_batch";  out+=",chains:${!key}"
    key="operator_${index}_coprocessor_fhe_threads";      out+=",threads:${!key}"
    key="operator_${index}_adaptive_batch_execution";     out+=",adaptive:${!key}"
    key="operator_${index}_batch_execution";              out+=",batch:${!key}"
  done < <(grep -o '^operator_[0-9]\+_device' "$NODE_CONFIG" | sed 's/^operator_//; s/_device$//' | sort -n)
  printf '%s' "$out"
}

metadata() {
  [[ -f "$BUILD_MANIFEST" ]] || die "missing GPU build manifest"
  cat "$BUILD_MANIFEST"
  [[ -f "$NODE_CONFIG" ]] && cat "$NODE_CONFIG"
  return 0
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
  printf 'export CONSENSUS_HARDWARE_CLASS=%q\n' "$(hardware_class)"
  # Exported separately so a gate can assert a device-split run really was split
  # without having to parse the class string.
  printf 'export CONSENSUS_DEVICE_COUNT=%q\n' "$(device_set | grep -c .)"
  printf 'export CONSENSUS_GPU_BUILD_MANIFEST_SHA256=%q\n' "$(binary_sha "$BUILD_MANIFEST")"
  # Scheduling configuration is not part of the hardware class: the operators
  # remain one backend/hardware class whether or not they schedule alike, and
  # the byte oracle applies unchanged.  It is exported separately so the gate
  # can assert the fleet really was heterogeneous when a run claims it.
  if [[ -f "$NODE_CONFIG" ]]; then
    printf 'export CONSENSUS_SCHEDULING_CLASSES=%q\n' "$(scheduling_classes)"
  fi
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
  # Check what `build` will check, without building: callers can run this before
  # a bring-up instead of discovering a dirty tree twenty minutes later.
  preflight)
    require docker
    require nvidia-smi
    require git
    require_clean_source
    [[ -x "${CUDA_PATH:-/usr/local/cuda}/bin/nvcc" ]] || die "nvcc is unavailable under CUDA_PATH=${CUDA_PATH:-/usr/local/cuda}"
    echo "gpu-consensus-workers: preflight OK (tree clean, nvcc present, $(gpu_count) GPU(s))"
    ;;
  build) build ;;
  start) ensure_user_bus; start ;;
  stop) ensure_user_bus; stop_host_workers; restore_recorded_docker_workers; clear_node_config ;;
  status) ensure_user_bus; status ;;
  conflicts) ensure_user_bus; conflicts ;;
  metadata) ensure_user_bus; metadata ;;
  test-env) ensure_user_bus; test_env ;;
  restart-unit) ensure_user_bus; shift; restart_unit "$@" ;;
  capture-activity) ensure_user_bus; capture_activity "$@" ;;
  *) usage; exit 2 ;;
esac
