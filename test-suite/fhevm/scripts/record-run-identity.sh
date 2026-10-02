#!/usr/bin/env bash
# What code a consensus run actually executed.
#
# A local checkout SHA is not that. `--build` builds repo-owned images from the
# working tree and everything else is pulled, so a result labelled with the
# branch SHA can be describing published images built from another revision --
# and on a GPU run the workers are host binaries built separately again. So this
# records the resolved identity of every image the stack is running plus the
# hashes of any host worker binaries, in a form the inventory's result records
# carry as `artifactIdentities`.
#
#   record-run-identity.sh [--format env|args|cr]
#
# `env` prints `NAME=value` lines for a log or an artifact; `args` prints
# `--artifact-identity NAME=value` arguments for `consensus-inventory.ts record`;
# `cr` prints `artifact=NAME=value` lines, which is the form `cr_record` takes.
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly STATE_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}"
readonly GPU_RUNTIME_DIR="${STATE_DIR}/runtime/gpu-consensus-workers"

FORMAT=env
while [[ $# -gt 0 ]]; do
  case "$1" in
    --format) FORMAT="${2:?--format needs env, args or cr}"; shift 2 ;;
    *) echo "usage: record-run-identity.sh [--format env|args|cr]" >&2; exit 2 ;;
  esac
done

emit() {
  local name="$1" value="$2"
  [[ -n "$value" ]] || return 0
  case "$FORMAT" in
    args) printf -- '--artifact-identity\n%s=%s\n' "$name" "$value" ;;
    cr) printf 'artifact=%s=%s\n' "$name" "$value" ;;
    *) printf '%s=%s\n' "$name" "$value" ;;
  esac
}

# The revision, and whether the tree it was read from was clean. A dirty tree
# cannot honestly be labelled with a commit: the binaries were built from
# something, and the label is evidence.
source "$SCRIPT_DIR/lib/source-revision.sh"
revision="$(sr_revision "$REPO_ROOT")" || exit 1
emit checkout_revision "$revision"
emit build_mode "${CONSENSUS_BUILD_MODE:-unspecified}"
emit software_class "${CONSENSUS_SOFTWARE_CLASS:-unverified}"
# Which Cargo features the checkout images were built with. A fault-enabled
# build and the production feature set are both "checkout"; only this tells
# them apart.
emit build_features "${CONSENSUS_BUILD_FEATURES:-${FHEVM_CONSENSUS_TEST_FEATURES:-none}}"

# Resolved image ids for every container in the project, keyed by container
# name: this is what `up` actually started, whether built or pulled.
if command -v docker >/dev/null 2>&1; then
  containers="$(docker ps -a --format '{{.Names}}')" || exit 1
  while IFS= read -r container; do
    [[ -n "$container" ]] || continue
    image="$(docker inspect -f '{{.Config.Image}}' "$container" 2>/dev/null)" || exit 1
    # Tags can move after this container starts. Its immutable .Image field is
    # the identity of the running filesystem; resolving Config.Image again
    # would attribute a later retagged build to the older running container.
    id="$(docker inspect -f '{{.Image}}' "$container" 2>/dev/null)" || exit 1
    [[ "$id" == sha256:* ]] || { echo "missing image identity for $container" >&2; exit 1; }
    digest="$(docker image inspect -f '{{if .RepoDigests}}{{index .RepoDigests 0}}{{end}}' "$id" 2>/dev/null)" || digest=""
    emit "image_${container}" "$id${digest:+ $digest} (${image})"
  done < <(printf '%s\n' "$containers" | grep -E '^(coprocessor|kms-connector|fhevm|gateway|host|listener)' | sort)
fi

# Host worker binaries, when a GPU session is serving the queues. The build
# manifest already records their hashes; republish them here so one artifact
# carries the whole identity of the run.
if [[ -f "${GPU_RUNTIME_DIR}/build-manifest.env" ]]; then
  while IFS='=' read -r key value; do
    [[ -n "$key" ]] || continue
    case "$key" in
      software_revision|*_sha256|gpu_*|hardware_*) emit "gpu_${key}" "${value//\'/}" ;;
      test_features) value="${value//\'/}"; emit gpu_test_features "${value:-none}" ;;
    esac
  done <"${GPU_RUNTIME_DIR}/build-manifest.env"
fi
if [[ -f "${GPU_RUNTIME_DIR}/node-config.env" ]]; then
  emit gpu_node_config_sha256 "$(sha256sum "${GPU_RUNTIME_DIR}/node-config.env" 2>/dev/null | cut -d' ' -f1)"
fi
