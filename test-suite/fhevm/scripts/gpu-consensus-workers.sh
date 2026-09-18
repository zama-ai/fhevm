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
# The state directory is resolved the same way the CLI resolves it. It used to
# be hardcoded to ${REPO_ROOT}/.fhevm, so a stack brought up with a custom
# FHEVM_STATE_DIR was driven by a launcher reading a different directory's
# session marker: the ownership guards then answered about another session, and
# `test-env` published another stack's build manifest as this run's evidence.
readonly STATE_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}"
readonly RUNTIME_DIR="${STATE_DIR}/runtime"
readonly ENV_DIR="${RUNTIME_DIR}/env"
readonly GPU_RUNTIME_DIR="${RUNTIME_DIR}/gpu-consensus-workers"
readonly BIN_DIR="${REPO_ROOT}/coprocessor/fhevm-engine/target/release"
readonly DEVICE="${GPU_CONSENSUS_DEVICE:-0}"
readonly STREAMS_PER_DEVICE="${GPU_CONSENSUS_STREAMS_PER_DEVICE:-16}"
readonly COMPONENTS_PER_BATCH="${GPU_CONSENSUS_COMPONENTS_PER_BATCH:-20}"
readonly WORK_ITEMS_BATCH_SIZE="${GPU_CONSENSUS_WORK_ITEMS_BATCH_SIZE:-100}"
readonly FHE_THREADS="${GPU_CONSENSUS_FHE_THREADS:-8}"
readonly TOKIO_THREADS="${GPU_CONSENSUS_TOKIO_THREADS:-4}"
readonly TEST_FAILPOINTS="${GPU_CONSENSUS_TEST_FAILPOINTS:-0}"
source "${SCRIPT_DIR}/lib/source-revision.sh"
readonly BUILD_MANIFEST="${GPU_RUNTIME_DIR}/build-manifest.env"
readonly NODE_CONFIG="${GPU_RUNTIME_DIR}/node-config.env"
readonly DOCKER_WORKER_STATE_FILE="${GPU_RUNTIME_DIR}/docker-workers-to-restore"
readonly INVOCATION_DIR="${GPU_RUNTIME_DIR}/invocations"
GPU_TRANSITION_COMPLETE=false

usage() {
  cat <<'EOF'
Usage: test-suite/fhevm/scripts/gpu-consensus-workers.sh
  <preflight|build|start|stop|stop-unit|restart-unit|unit-config|verify-restore|status|conflicts|metadata|test-env|device-evidence|capture-activity>

Builds and runs source-matched GPU-feature TFHE, ZK-proof, and SNS workers for
the active three-coprocessor consensus topology. All operators use
GPU_CONSENSUS_DEVICE (default 0) so the test has one homogeneous H100 class.

Optional tuning variables:
  GPU_CONSENSUS_TEST_FAILPOINTS=0  # this consensus layer uses production workers
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

`device-evidence [expected-device-count]` asks the driver which physical GPU
each worker is executing on and fails unless at least that many distinct cards
are in use. Placement intent is not the same as a device having run the work.

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
  systemctl --user show-environment >/dev/null 2>&1 ||
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
readonly GENERATED_TRACKED_SOURCE="$SR_GENERATED_SOURCE"

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
  dirty="$(sr_status "$REPO_ROOT")" || die "cannot establish source revision"
  [[ -n "$dirty" ]] || return 0
  # Separate tracked edits from untracked artifacts because their remedies differ.
  # Stale generated artifacts must not be mistaken for user source changes.
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
    printf 'test_features=%q\n' "$(test_features)"
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
  test_features >/dev/null || return 1
  [[ -f "$BUILD_MANIFEST" ]] || die "missing GPU build manifest; run '$0 build' after the current source revision is committed"
  # shellcheck disable=SC1090
  source "$BUILD_MANIFEST"
  local revision
  revision="$(git -C "$REPO_ROOT" rev-parse HEAD)"
  [[ "${software_revision:-}" == "$revision" ]] || die "GPU binaries were built for ${software_revision:-unknown}, current source is $revision; rebuild"
  [[ "${test_features:-}" == "$(test_features)" ]] || die "GPU build feature set differs from GPU_CONSENSUS_TEST_FAILPOINTS=$TEST_FAILPOINTS; rebuild with the requested feature set"
  [[ "${gpu_feature:-}" == "gpu" ]] || die "build manifest does not prove the GPU feature"
  [[ "${cuda_visible_devices:-}" == "$DEVICE" ]] || die "GPU build manifest selects ${cuda_visible_devices:-unknown}, requested device is $DEVICE"
  [[ "${gpu_uuid:-}" == "$(gpu_uuid)" ]] || die "selected physical GPU changed since the GPU build; rebuild for auditable metadata"
  [[ "${tfhe_worker_sha256:-}" == "$(binary_sha "$BIN_DIR/tfhe_worker")" ]] || die "tfhe_worker differs from recorded GPU build"
  [[ "${zkproof_worker_sha256:-}" == "$(binary_sha "$BIN_DIR/zkproof_worker")" ]] || die "zkproof_worker differs from recorded GPU build"
  [[ "${sns_worker_sha256:-}" == "$(binary_sha "$BIN_DIR/sns_worker")" ]] || die "sns_worker differs from recorded GPU build"
}

build() {
  test_features >/dev/null || return 1
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
      cargo build --release -p tfhe-worker -p zkproof-worker -p sns-worker --features "$(build_features)"
  )
  for binary in tfhe_worker zkproof_worker sns_worker; do
    [[ -x "$BIN_DIR/$binary" ]] || die "GPU build did not produce $BIN_DIR/$binary"
  done
  write_manifest
  echo "gpu-consensus-workers: built source-matched GPU workers; metadata: $BUILD_MANIFEST"
}

test_features() {
  case "$TEST_FAILPOINTS" in
    0) printf '' ;;
    1) die "fault-hook builds are introduced by the subsequent failure-mode coverage branch" ;;
    *) die "GPU_CONSENSUS_TEST_FAILPOINTS must be 0 (production) or 1 (fault hooks)" ;;
  esac
}

build_features() {
  local hooks
  hooks="$(test_features)" || return 1
  printf 'gpu%s' "${hooks:+,$hooks}"
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

# Where one unit's resolved invocation configuration is recorded.
#
# This is the difference between restoring a unit and re-resolving one.
# `restart-unit` used to call `start_unit`, which reads
# GPU_CONSENSUS_<KNOB>_<index> out of the *caller's* environment -- and its
# callers are the degraded suite, the failure matrix and the CLI, none of which
# has those variables. A stopped operator therefore came back on fleet defaults,
# silently converting a deliberately heterogeneous fleet into a uniform one, or
# a second-GPU operator onto card 0. The resolved values are written here when
# the session starts, and restoration reads them back.
unit_config_file() {
  printf '%s/%s.config' "$INVOCATION_DIR" "$(unit_name "$1" "$2")"
}

unit_environment_file() {
  printf '%s/%s.env' "$INVOCATION_DIR" "$(unit_name "$1" "$2")"
}

# EnvironmentFile overrides systemd's Environment/--setenv. Resolve launcher
# overrides into the file itself, last, and report from this same private file.
# Each unit gets its own file so starting SNS cannot overwrite TFHE settings.
write_unit_environment() (
  local kind="$1" index="$2" source="$3" target
  target="$(unit_environment_file "$kind" "$index")"
  mkdir -p "$INVOCATION_DIR" || return 1
  umask 077
  cat "$source" >"$target" || return 1
  printf '\nCUDA_VISIBLE_DEVICES=%s\nFHEVM_GPU_STREAMS_PER_DEVICE=%s\nRUST_BACKTRACE=1\n' \
    "$UNIT_DEVICE" "$UNIT_STREAMS" >>"$target" || return 1
  if [[ "$kind" == tfhe ]]; then
    if [[ -n "${UNIT_ADAPTIVE:-}" ]]; then
      printf 'FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=%s\n' "$UNIT_ADAPTIVE" >>"$target" || return 1
    fi
    if [[ -n "${UNIT_BATCH:-}" ]]; then
      printf 'FHEVM_DCID_BATCH_EXECUTION=%s\n' "$UNIT_BATCH" >>"$target" || return 1
    fi
  fi
)

# Resolve this operator's knobs from the environment and record them.
record_unit_tuning() {
  local kind="$1" index="$2" file
  file="$(unit_config_file "$kind" "$index")"
  mkdir -p "$INVOCATION_DIR"
  umask 077
  {
    printf 'worker_sha256=%q\n' "$(binary_sha "$BIN_DIR/${kind}_worker")"
    printf 'build_test_features=%q\n' "$(test_features)"
    printf 'device=%q\n' "$(node_tuning DEVICE "$index" "$DEVICE")"
    printf 'streams=%q\n' "$(node_tuning STREAMS_PER_DEVICE "$index" "$STREAMS_PER_DEVICE")"
    printf 'work_items=%q\n' "$(node_tuning WORK_ITEMS_BATCH_SIZE "$index" "$WORK_ITEMS_BATCH_SIZE")"
    printf 'chains=%q\n' "$(node_tuning COMPONENTS_PER_BATCH "$index" "$COMPONENTS_PER_BATCH")"
    printf 'fhe_threads=%q\n' "$(node_tuning FHE_THREADS "$index" "$FHE_THREADS")"
    printf 'tokio_threads=%q\n' "$(node_tuning TOKIO_THREADS "$index" "$TOKIO_THREADS")"
    printf 'adaptive=%q\n' "$(node_tuning ADAPTIVE_BATCH_EXECUTION "$index" "")"
    printf 'batch=%q\n' "$(node_tuning BATCH_EXECUTION "$index" "")"
  } >"$file"
}

# Load a recorded configuration into the UNIT_* variables start_unit uses.
load_unit_tuning() {
  local kind="$1" index="$2" file
  file="$(unit_config_file "$kind" "$index")"
  [[ -f "$file" ]] || return 1
  local device streams work_items chains fhe_threads tokio_threads adaptive batch
  # shellcheck disable=SC1090
  source "$file"
  UNIT_DEVICE="$device"
  UNIT_STREAMS="$streams"
  UNIT_WORK_ITEMS="$work_items"
  UNIT_CHAINS="$chains"
  UNIT_FHE_THREADS="$fhe_threads"
  UNIT_TOKIO_THREADS="$tokio_threads"
  UNIT_ADAPTIVE="$adaptive"
  UNIT_BATCH="$batch"
  return 0
}

# Resolve from the environment into the same variables, for the initial start.
resolve_unit_tuning() {
  local index="$1"
  UNIT_DEVICE="$(node_tuning DEVICE "$index" "$DEVICE")"
  UNIT_STREAMS="$(node_tuning STREAMS_PER_DEVICE "$index" "$STREAMS_PER_DEVICE")"
  UNIT_WORK_ITEMS="$(node_tuning WORK_ITEMS_BATCH_SIZE "$index" "$WORK_ITEMS_BATCH_SIZE")"
  UNIT_CHAINS="$(node_tuning COMPONENTS_PER_BATCH "$index" "$COMPONENTS_PER_BATCH")"
  UNIT_FHE_THREADS="$(node_tuning FHE_THREADS "$index" "$FHE_THREADS")"
  UNIT_TOKIO_THREADS="$(node_tuning TOKIO_THREADS "$index" "$TOKIO_THREADS")"
  UNIT_ADAPTIVE="$(node_tuning ADAPTIVE_BATCH_EXECUTION "$index" "")"
  UNIT_BATCH="$(node_tuning BATCH_EXECUTION "$index" "")"
}

# One comparable string for a unit's resolved configuration, so a restoration
# can be asserted identical rather than assumed so.
unit_tuning_signature() {
  printf 'device=%s streams=%s work_items=%s chains=%s fhe_threads=%s tokio_threads=%s adaptive=%s batch=%s' \
    "$UNIT_DEVICE" "$UNIT_STREAMS" "$UNIT_WORK_ITEMS" "$UNIT_CHAINS" \
    "$UNIT_FHE_THREADS" "$UNIT_TOKIO_THREADS" "${UNIT_ADAPTIVE:-default}" "${UNIT_BATCH:-default}"
}

# Starts one unit from the UNIT_* variables a caller has already resolved or
# loaded, so the invocation is a function of the recorded configuration rather
# than of the ambient environment.
start_unit() {
  local kind="$1" index="$2" env_file="$3" unit streams resolved_env
  unit="$(unit_name "$kind" "$index")"
  streams="$UNIT_STREAMS"
  stop_transient_unit "$unit" || return 1

  write_unit_environment "$kind" "$index" "$env_file" || return 1
  resolved_env="$(unit_environment_file "$kind" "$index")"
  local -a args
  case "$kind" in
    tfhe)
      args=(
        --run-bg-worker
        --database-url="$(grep '^DATABASE_URL=' "$env_file" | cut -d= -f2-)"
        --pg-pool-max-connections=10
        --worker-polling-interval-ms=1000
        --work-items-batch-size="$UNIT_WORK_ITEMS"
        --dependence-chains-per-batch="$UNIT_CHAINS"
        --key-cache-size=32
        --coprocessor-fhe-threads="$UNIT_FHE_THREADS"
        --gpu-streams-per-device="$streams"
        --tokio-threads="$UNIT_TOKIO_THREADS"
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
    --description="FHEVM GPU $kind worker operator $index" \
    --property=Restart=on-failure --property=RestartSec=2 \
    --property="EnvironmentFile=$resolved_env" \
    "${BIN_DIR}/${kind}_worker" "${args[@]}" >/dev/null
}

stop_host_workers() {
  local index kind unit state pid failures=0
  # Invocation records survive removal of generated stack environments during
  # `down`, so every unit from the session remains discoverable for cleanup.
  local -a units=()
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do units+=("$(unit_name "$kind" "$index")"); done
  done < <(instance_indexes)
  local config
  for config in "$INVOCATION_DIR"/fhevm-gpu-consensus-*.config; do
    [[ -f "$config" ]] || continue
    unit="${config##*/}"; units+=("${unit%.config}")
  done
  [[ "${#units[@]}" -gt 0 ]] || {
    [[ ! -e "$DOCKER_WORKER_STATE_FILE" && ! -e "$NODE_CONFIG" ]] && return 0
    echo "gpu-consensus-workers: cannot identify units held by this session; preserving ownership" >&2
    return 1
  }
  while IFS= read -r unit; do
    # A collected transient unit is legitimately absent. A failed stop or
    # inspection is otherwise not proof that its process stopped.
    if ! state="$(systemctl --user show "$unit" --property=LoadState --property=ActiveState --property=MainPID 2>/dev/null)"; then
      echo "gpu-consensus-workers: cannot inspect $unit; preserving ownership" >&2
      failures=$((failures + 1)); continue
    fi
    if grep -qx 'LoadState=not-found' <<<"$state" && grep -qx 'MainPID=0' <<<"$state"; then continue; fi
    if ! systemctl --user stop "$unit" >/dev/null 2>&1; then
      echo "gpu-consensus-workers: could not stop $unit; preserving ownership" >&2
      failures=$((failures + 1)); continue
    fi
    state="$(systemctl --user show "$unit" --property=ActiveState --property=MainPID 2>/dev/null)" || {
      failures=$((failures + 1)); continue
    }
    pid="$(sed -n 's/^MainPID=//p' <<<"$state")"
    state="$(sed -n 's/^ActiveState=//p' <<<"$state")"
    if [[ "$pid" != 0 || ( "$state" != inactive && "$state" != failed ) ]]; then
      echo "gpu-consensus-workers: $unit is not quiescent; preserving ownership" >&2
      failures=$((failures + 1))
    fi
  done < <(printf '%s\n' "${units[@]}" | sort -u)
  [[ "$failures" == 0 ]]
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

# Restoration is a transaction: the ownership record is what lets a later run
# know which containers this launcher displaced, so it is discarded only once
# every one of them is verified back. Deleting it after a partial restore left
# a stack with a stopped worker nobody would ever start, and the next
# `ensure_no_active_gpu_session` then reported a clean slate.
restore_recorded_docker_workers() {
  local container failures=0 state inspection
  [[ -f "$DOCKER_WORKER_STATE_FILE" ]] || return 0
  while IFS= read -r container; do
    [[ -n "$container" ]] || continue
    # A container that no longer EXISTS is not a container we failed to hand
    # back: the stack that owned it was torn down, and the next `up` recreates
    # it from compose. Treating absence as a failure kept the ownership record
    # alive, and the record makes `up` refuse -- so the stack could neither be
    # restored nor replaced, with nothing actually holding the queues. Absence
    # and unstartable are different states and only the second is dangerous.
    if ! inspection="$(docker inspect "$container" 2>&1)"; then
      if grep -qiE 'no such (object|container)' <<<"$inspection"; then
        echo "gpu-consensus-workers: displaced container $container no longer exists; nothing to hand back" >&2
      else
        echo "gpu-consensus-workers: cannot inspect displaced container $container; preserving ownership" >&2
        failures=$((failures + 1))
      fi
      continue
    fi
    if ! docker start "$container" >/dev/null 2>&1; then
      echo "gpu-consensus-workers: could not restart displaced container $container" >&2
      failures=$((failures + 1))
      continue
    fi
    state="$(docker inspect -f '{{.State.Status}}' "$container" 2>/dev/null || echo missing)"
    if [[ "$state" != running ]]; then
      echo "gpu-consensus-workers: $container is $state after a start; not treating it as restored" >&2
      failures=$((failures + 1))
    fi
  done <"$DOCKER_WORKER_STATE_FILE"
  if ((failures > 0)); then
    echo "gpu-consensus-workers: $failures displaced container(s) were not restored; keeping the ownership record at $DOCKER_WORKER_STATE_FILE so the next run refuses to start rather than losing track of them" >&2
    return 1
  fi
  return 0
}

# Complete handover only after every GPU process has stopped and every displaced
# Docker owner is restored. Both records remain available after any failure.
restore_docker_session() {
  stop_host_workers || return 1
  restore_recorded_docker_workers || return 1
  conflicts || return 1
  rm -f "$DOCKER_WORKER_STATE_FILE"
  clear_node_config
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
  gpu_session_marker_or_die
  [[ -f "$DOCKER_WORKER_STATE_FILE" ]] || die "GPU ownership record is absent; refusing to revive a stale invocation"
  local displaced
  displaced="$(docker inspect -f '{{.State.Status}}' "$(container_name "$kind" "$index")" 2>/dev/null)" ||
    die "cannot verify the displaced Docker owner before restarting $(unit_name "$kind" "$index")"
  case "$displaced" in exited|created|dead) ;; *) die "Docker owner is $displaced; refusing to put a GPU worker on the same queue" ;; esac
  verify_unit_binary "$kind" "$index" || return 1
  local host_env="$GPU_RUNTIME_DIR/coprocessor.${index}.env"
  [[ -f "$host_env" ]] ||
    die "no generated environment for operator $index at $host_env; restart-unit only works inside a GPU session started by this script"
  # From the RECORD, not from this shell. Re-resolving would take fleet defaults
  # for any knob the caller does not happen to have exported, which is how a
  # heterogeneous operator came back uniform.
  load_unit_tuning "$kind" "$index" ||
    die "no recorded invocation configuration at $(unit_config_file "$kind" "$index"); restoring from the ambient environment would risk reverting this operator to fleet defaults, so refuse instead"
  local signature; signature="$(unit_tuning_signature)"
  start_unit "$kind" "$index" "$host_env"
  record_unit_invocation "$(unit_name "$kind" "$index")"
  echo "gpu-consensus-workers: restarted $(unit_name "$kind" "$index") with recorded configuration [$signature]"
}

verify_unit_binary() {
  local kind="$1" index="$2" worker_sha256 build_test_features
  local file; file="$(unit_config_file "$kind" "$index")"
  [[ -f "$file" ]] || die "missing recorded GPU invocation for $kind/$index"
  # shellcheck disable=SC1090
  source "$file"
  [[ -n "${worker_sha256:-}" && "${build_test_features+x}" ]] ||
    die "GPU invocation lacks recorded feature/binary identity; restart the whole GPU session"
  [[ "$worker_sha256" == "$(binary_sha "$BIN_DIR/${kind}_worker")" ]] ||
    die "GPU worker binary changed since this invocation (recorded hooks: ${build_test_features:-none}); restart the whole GPU session"
}

# Stop one unit while KEEPING its recorded configuration, so it can be restored
# exactly. `systemctl stop` on a transient unit garbage-collects the unit, which
# is why a caller cannot simply pair stop with start.
stop_unit() {
  local kind="${1:?stop-unit needs a kind: tfhe|zkproof|sns}"
  local index="${2:?stop-unit needs an operator index}"
  case "$kind" in tfhe | zkproof | sns) ;; *) die "unknown worker kind: $kind" ;; esac
  local unit; unit="$(unit_name "$kind" "$index")"
  [[ -f "$(unit_config_file "$kind" "$index")" ]] ||
    die "$unit has no recorded configuration; stopping it would make an exact restoration impossible"
  stop_transient_unit "$unit" || return 1
  echo "gpu-consensus-workers: stopped $unit (configuration retained for restoration)"
}

stop_transient_unit() {
  local unit="$1"
  timeout --kill-after=2s 35s systemctl --user stop "$unit" >/dev/null 2>&1 || true
  local deadline=$((SECONDS + 30))
  while ((SECONDS < deadline)); do
    local state load pid
    state="$(timeout 5s systemctl --user show "$unit" --property=ActiveState --value 2>/dev/null)" || state=""
    load="$(timeout 5s systemctl --user show "$unit" --property=LoadState --value 2>/dev/null)" || load=""
    pid="$(timeout 5s systemctl --user show "$unit" --property=MainPID --value 2>/dev/null)" || pid=""
    # systemd-run cannot reuse a still-loaded transient unit name. Neither
    # deactivating nor an unreadable inspection proves the old process is gone.
    [[ "$state" == inactive && "$load" == not-found && "$pid" == 0 ]] && {
      return 0
    }
    sleep 1
  done
  die "$unit did not stop within 30s"
}

# Print what a unit would be restored with, for a caller that wants to compare.
unit_config() {
  local kind="${1:?unit-config needs a kind}" index="${2:?unit-config needs an index}"
  load_unit_tuning "$kind" "$index" || die "no recorded configuration for $(unit_name "$kind" "$index")"
  unit_tuning_signature
  printf '\n'
}

# Stop one unit and bring it back, then require the restored invocation to carry
# the identical recorded configuration.
#
# This is the acceptance case for the restoration path itself: it runs from a
# shell with none of the GPU_CONSENSUS_* overrides set, which is precisely the
# situation in which re-resolving would have silently reverted the operator to
# fleet defaults.
verify_restore() (
  local kind="${1:?verify-restore needs a kind}" index="${2:?verify-restore needs an index}"
  local unit; unit="$(unit_name "$kind" "$index")"
  load_unit_tuning "$kind" "$index" || die "no recorded configuration for $unit"
  local before_signature invocation_before
  before_signature="$(unit_tuning_signature)"
  invocation_before="$(systemctl --user show "$unit" --property=InvocationID --value 2>/dev/null)"
  [[ -n "$invocation_before" ]] || die "$unit is not running, so a restoration cannot be verified against it"
  [[ "$(systemctl --user show "$unit" --property=ActiveState --value)" == active &&
    "$(systemctl --user show "$unit" --property=MainPID --value)" =~ ^[1-9][0-9]*$ ]] ||
    die "$unit has no active process to interrupt"

  # Keep cleanup in this subshell: a failed stop/restart must not leave the
  # originally running operator down for the remaining lifecycle cases. A
  # failed recovery retains the session's ownership/configuration for retry.
  local restore_pending=true restore_status=0
  trap 'restore_status=$?; trap - EXIT; if [[ "$restore_pending" == true ]]; then
    echo "gpu-consensus-workers: restoring $unit after aborted verification" >&2
    if ! (restart_unit "$kind" "$index" && wait_for_units); then
      echo "gpu-consensus-workers: cleanup failed for $unit; ownership retained" >&2
      exit 1
    fi
  fi; exit "$restore_status"' EXIT
  trap 'exit 130' INT
  trap 'exit 143' TERM
  stop_unit "$kind" "$index"
  restart_unit "$kind" "$index"
  wait_for_units || die "$unit did not become ready after restoration"
  restore_pending=false

  load_unit_tuning "$kind" "$index" || die "the recorded configuration for $unit disappeared across the restart"
  local after_signature invocation_after
  after_signature="$(unit_tuning_signature)"
  invocation_after="$(systemctl --user show "$unit" --property=InvocationID --value 2>/dev/null)"
  [[ "$after_signature" == "$before_signature" ]] ||
    die "$unit came back with different configuration:
  before [$before_signature]
  after  [$after_signature]"
  [[ -n "$invocation_after" && "$invocation_after" != "$invocation_before" ]] ||
    die "$unit reports the same systemd invocation ($invocation_after) after a restart; it did not actually restart"
  # And the resolved command line has to match the record, not just the record
  # itself: a restoration that wrote the file correctly and then invoked
  # something else would pass a file-only comparison.
  local resolved
  resolved="$(systemctl --user show "$unit" --property=ExecStart --value 2>/dev/null)"
  grep -q -- "--work-items-batch-size=$UNIT_WORK_ITEMS" <<<"$resolved" ||
    die "$unit was restarted without its recorded work-items-batch-size ($UNIT_WORK_ITEMS): $resolved"
  grep -q -- "--dependence-chains-per-batch=$UNIT_CHAINS" <<<"$resolved" ||
    die "$unit was restarted without its recorded dependence-chains-per-batch ($UNIT_CHAINS): $resolved"
  if ! conflicts; then
    die "$unit's queue is served twice after the restoration"
  fi
  echo "gpu-consensus-workers: $unit restored with identical configuration [$after_signature]"
  echo "gpu-consensus-workers: invocation $invocation_before -> $invocation_after"
)

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
  bun "$SCRIPT_DIR/gpu-key-readiness.ts" || die "GPU workers require ingested compressed key material before handover"
  bun "$SCRIPT_DIR/queue-ownership.ts" 3 --allow-missing --no-blue-green || die "unmanaged or conflicting queue owners prevent GPU handover"

  # Do not let two implementations consume the same queue.  Stopping only
  # these worker roles preserves the live source-matched listeners, DBs, KMS
  # material, contracts, and test-suite state that the CPU gate already proved.
  # From this point the transition is transactional.  If preparation, a
  # Docker stop, systemd startup, or readiness proof fails, the EXIT trap
  # stops any partial GPU set and restores only containers that this launcher
  # observed running before it changed anything.  It never revives a worker an
  # operator had intentionally left down.
  GPU_TRANSITION_COMPLETE=false
  trap 'if [[ "$GPU_TRANSITION_COMPLETE" != true ]]; then restore_docker_session || echo "gpu-consensus-workers: rollback incomplete; ownership retained" >&2; fi' EXIT
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

  # Resolve each operator's knobs ONCE, record them, and start from the record.
  # Everything that later restores a unit reads the same file, so a restoration
  # cannot depend on which shell happens to be calling.
  local kind_iter
  if ! while IFS= read -r index; do
    host_env="$GPU_RUNTIME_DIR/coprocessor.${index}.env"
    resolve_unit_tuning "$index"
    for kind_iter in tfhe zkproof sns; do
      record_unit_tuning "$kind_iter" "$index"
      load_unit_tuning "$kind_iter" "$index"
      start_unit "$kind_iter" "$index" "$host_env"
      record_unit_invocation "$(unit_name "$kind_iter" "$index")"
    done
  done < <(instance_indexes); then
    die "failed to start host GPU workers; restored Docker worker consumers"
  fi

  if ! wait_for_units; then
    status >&2 || true
    die "GPU workers did not become active; restored Docker worker consumers"
  fi
  write_node_config
  bun "$SCRIPT_DIR/queue-ownership.ts" 3 || die "GPU fleet ownership could not be established after handover"
  GPU_TRANSITION_COMPLETE=true
  trap - EXIT
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
# indistinguishable from a consensus defect. For the TFHE worker it would
# diverge ct64 itself.
#
# `start` already stops the containers it displaces.  The reverse direction is
# the gap: these units are transient with `Restart=on-failure`, so they outlive
# a stack teardown and restart themselves, and the next `fhevm-cli up` brings
# the containers back underneath them.  Nothing in either component notices, so
# report it here.
conflicts() {
  local index kind unit container found=0 unit_state container_state active=0
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      unit="$(unit_name "$kind" "$index")"
      container="$(container_name "$kind" "$index")"
      unit_state="$(systemctl --user show "$unit" --property=ActiveState --value 2>/dev/null)" || {
        echo "gpu-consensus-workers: cannot inspect $unit for conflicts" >&2
        found=1; continue
      }
      case "$unit_state" in inactive|failed) continue ;; active|activating|deactivating) ;; *) found=1; continue ;; esac
      active=1
      container_state="$(docker inspect -f '{{.State.Status}}' "$container" 2>&1)" || {
        if ! grep -qiE 'no such (object|container)' <<<"$container_state"; then found=1; fi
        continue
      }
      case "$container_state" in running|paused|restarting) ;; *) continue ;; esac
      printf 'CONFLICT operator=%s kind=%s unit=%s container=%s: both are serving the same queue\n' \
        "$index" "$kind" "$unit" "$container"
      found=1
    done
  done < <(instance_indexes)
  local -a ownership_args=(3 --allow-missing)
  # During restoration the host units are gone while the session marker still
  # protects the incomplete handback. Validate CPU ownership in that phase.
  [[ "$active" == 1 ]] || ownership_args+=(--cpu-only)
  bun "$SCRIPT_DIR/queue-ownership.ts" "${ownership_args[@]}" || found=1
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
# Read the last assignment from the file supplied to this unit, including any
# resolved override. The original generated env may have changed since start.
effective_scheduling_flag() {
  local index="$1" var="$2" file value
  file="$(unit_environment_file tfhe "$index")"
  [[ -f "$file" ]] || { echo "missing resolved environment for TFHE operator $index" >&2; return 1; }
  value="$(sed -n "s/^${var}=//p" "$file" | tail -1)"
  case "$value" in true|false) printf '%s' "$value" ;; '') printf default ;; *) return 1 ;; esac
}

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
    out+=",adaptive:$(effective_scheduling_flag "$index" FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION)" || return 1
    out+=",batch:$(effective_scheduling_flag "$index" FHEVM_DCID_BATCH_EXECUTION)" || return 1
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
  # Rebuilding can replace the file at BIN_DIR while a service still executes
  # the previous inode. Attribute evidence to the executable actually running.
  local index kind unit pid expected actual displaced live=0
  gpu_session_marker_or_die
  while IFS= read -r index; do
    for kind in tfhe zkproof sns; do
      unit="$(unit_name "$kind" "$index")"
      displaced="$(docker inspect -f '{{.State.Status}}' "$(container_name "$kind" "$index")" 2>/dev/null)" ||
        die "cannot verify displaced Docker owner for $unit"
      case "$displaced" in exited|created|dead) ;; *) die "$unit has a $displaced Docker owner; refusing GPU attribution from a stale session marker" ;; esac
      pid="$(systemctl --user show "$unit" --property=MainPID --value)" ||
        die "cannot inspect the running executable for $unit"
      [[ "$pid" =~ ^[0-9]+$ ]] || die "unreadable MainPID for $unit"
      [[ "$pid" != 0 ]] || continue # a fault case may deliberately stop a worker
      live=$((live + 1))
      expected="${kind}_worker_sha256"
      actual="$(binary_sha "/proc/$pid/exe")" || die "cannot hash the running executable for $unit"
      [[ "$actual" == "${!expected}" ]] ||
        die "$unit is executing a different binary from the recorded build; restart the GPU session"
    done
  done < <(instance_indexes)
  [[ "$live" -gt 0 ]] || die "no GPU worker process is running; session marker alone cannot establish a GPU execution class"
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
    local classes
    classes="$(scheduling_classes)" || die "cannot read resolved GPU scheduling configuration"
    printf 'export CONSENSUS_SCHEDULING_CLASSES=%q\n' "$classes"
  fi
}

# Which PHYSICAL device each worker is executing on, from the driver rather than
# from the placement we asked for.
#
# `CUDA_VISIBLE_DEVICES` and the recorded node configuration express intent, and
# `assertDeviceSplit` compares that intent for distinctness -- necessary, and not
# the claim. A device-independence result needs the driver to agree that two
# different cards ran the work, which `--query-compute-apps` answers directly:
# it lists the GPU UUID against the PID holding each compute context.
#
# Prints `unit <name> pid <pid> gpu <uuid>` per worker and exits non-zero unless
# the number of distinct UUIDs matches `--expect-devices`.
device_evidence() {
  local expect="${1:-0}"
  require nvidia-smi
  gpu_session_marker_or_die
  local -A pid_to_uuid=()
  local line pid uuid
  while IFS=, read -r pid uuid; do
    pid="${pid// /}"
    uuid="${uuid// /}"
    [[ -n "$pid" ]] && pid_to_uuid["$pid"]="$uuid"
  done < <(nvidia-smi --query-compute-apps=pid,gpu_uuid --format=csv,noheader 2>/dev/null)

  local index kind unit main_pid found=0
  local -A seen_uuid=()
  while IFS= read -r index; do
    for kind in tfhe sns; do
      unit="$(unit_name "$kind" "$index")"
      main_pid="$(systemctl --user show "$unit" --property=MainPID --value 2>/dev/null)"
      [[ -n "$main_pid" && "$main_pid" != 0 ]] || continue
      uuid="${pid_to_uuid[$main_pid]:-}"
      if [[ -z "$uuid" ]]; then
        # A worker with no CUDA context yet has not executed anything on a
        # device, which is not evidence of placement either way.
        printf 'unit %s pid %s gpu (no compute context)\n' "$unit" "$main_pid"
        continue
      fi
      printf 'unit %s pid %s gpu %s\n' "$unit" "$main_pid" "$uuid"
      seen_uuid["$uuid"]=1
      found=$((found + 1))
    done
  done < <(instance_indexes)

  [[ "$found" -gt 0 ]] || die "no worker holds a CUDA compute context, so no device evidence exists"
  printf 'distinct physical devices executing worker code: %s\n' "${#seen_uuid[@]}"
  if [[ "$expect" -gt 0 && "${#seen_uuid[@]}" -lt "$expect" ]]; then
    die "expected work on $expect distinct device(s), the driver reports ${#seen_uuid[@]}; a split that ran on one card proves nothing about device independence"
  fi
}

gpu_session_marker_or_die() {
  [[ -f "$NODE_CONFIG" ]] || die "no GPU session is active ($NODE_CONFIG is absent)"
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
  stop)
    ensure_user_bus
    restore_docker_session || die "GPU handover did not complete; ownership retained for retry"
    ;;
  stop-unit) ensure_user_bus; shift; stop_unit "$@" ;;
  unit-config) ensure_user_bus; shift; unit_config "$@" ;;
  verify-restore) ensure_user_bus; shift; verify_restore "$@" ;;
  status) ensure_user_bus; status ;;
  conflicts) ensure_user_bus; conflicts ;;
  metadata) ensure_user_bus; metadata ;;
  test-env) ensure_user_bus; test_env ;;
  restart-unit) ensure_user_bus; shift; restart_unit "$@" ;;
  device-evidence) ensure_user_bus; shift; device_evidence "$@" ;;
  capture-activity) ensure_user_bus; capture_activity "$@" ;;
  *) usage; exit 2 ;;
esac
