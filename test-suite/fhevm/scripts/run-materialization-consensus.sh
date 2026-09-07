#!/usr/bin/env bash
# Run the materialization byte-consensus gate against a live multi-coprocessor
# stack, with one command instead of a hand-assembled environment.
#
# The gate itself (test-suite/e2e/test/consensus/materializationConsensus.ts) is
# opt-in and reads its whole contract from the environment: how many operators
# there are, which execution class they share, and where the gateway and the
# per-operator databases live.  Assembling that by hand is how a run ends up
# quietly measuring the wrong thing -- a stale address, a threshold that does
# not match the topology, or a scheduling claim nothing checked.
#
# Everything here is therefore DISCOVERED from the running stack rather than
# asserted on the command line.  In particular the per-operator scheduling
# configuration is read back off the containers that are actually running, not
# from the scenario file that was supposed to have produced them: the two
# disagree exactly when a stack was generated from a different scenario than
# the operator believes, which is the case the check exists to catch.
#
#   run-materialization-consensus.sh [--heterogeneous] [--suite <name>]
#
# With --heterogeneous the gate additionally refuses to run unless the
# operators really are scheduling differently.  See
# scenarios/three-of-three-heterogeneous-scheduling.yaml.
#
# --suite fork runs the fork byte-consensus gate instead, which needs the
# `three-of-three-fork` topology (two operators on the canonical Anvil, one on
# the fork).  Discovery is identical for both gates, which is why they share a
# runner rather than duplicating it.
set -euo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENV_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/env"
# shellcheck source=lib/gpu-session.sh
source "${SCRIPT_DIR}/lib/gpu-session.sh"
gpu_normalise_user_bus
readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
# `staging` resolves its RPC from RPC_URL, which the test container already has
# pointed at the in-network host chain.  NOT `localCoprocessor`: that one is
# hardcoded to localhost:8746 for running Hardhat on the host against a
# forwarded port, so inside the container it fails with HH108 before any test
# body runs.  The consensus README still documents the localCoprocessor form.
readonly TEST_NETWORK="${TEST_NETWORK:-staging}"

EXPECT_HETEROGENEOUS=0
EXPECT_DEVICE_SPLIT=0
SUITE=materialization
while [[ $# -gt 0 ]]; do
  case "$1" in
    --heterogeneous)
      EXPECT_HETEROGENEOUS=1
      shift
      ;;
    # Device placement is a separate axis from scheduling: a fleet can schedule
    # identically on two cards, or differently on one. Asking for it explicitly
    # means a run that did not split fails instead of reporting independence it
    # never exercised.
    --device-split)
      EXPECT_DEVICE_SPLIT=1
      shift
      ;;
    --suite)
      [[ $# -ge 2 ]] || {
        echo "--suite needs a value: materialization, fork, reorg or degraded" >&2
        exit 2
      }
      SUITE="$2"
      shift 2
      ;;
    *)
      echo "usage: run-materialization-consensus.sh [--heterogeneous] [--device-split] [--suite materialization|fork|reorg|degraded]" >&2
      exit 2
      ;;
  esac
done
case "$SUITE" in
  materialization | fork | reorg | degraded) ;;
  *)
    echo "unknown suite $SUITE (expected materialization, fork, reorg or degraded)" >&2
    exit 2
    ;;
esac
readonly EXPECT_HETEROGENEOUS EXPECT_DEVICE_SPLIT SUITE

die() {
  echo "run-materialization-consensus: $*" >&2
  exit 1
}

# Operator indexes present in the generated environment: `coprocessor.env` is
# operator 0 and `coprocessor.<n>.env` is operator n.
operator_indexes() {
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

env_value() {
  local key="$1" file="$2"
  sed -n "s/^${key}=//p" "$file" | tail -1
}

container_for() {
  local index="$1"
  [[ "$index" == 0 ]] && echo "coprocessor-tfhe-worker" || echo "coprocessor${index}-tfhe-worker"
}

# One canonical scheduling class per operator, read off the running container:
# the flags it was actually started with and the two environment-only booleans.
# Absent flags are reported as `default` rather than guessed, so a class never
# claims a value the worker was not given.
observed_scheduling_classes() {
  local index container cmd env_json first=true out="" value
  while IFS= read -r index; do
    container="$(container_for "$index")"
    docker inspect "$container" >/dev/null 2>&1 ||
      die "cannot read scheduling configuration: container $container is not present"
    cmd="$(docker inspect --format '{{range .Config.Cmd}}{{println .}}{{end}}' "$container")"
    env_json="$(docker inspect --format '{{range .Config.Env}}{{println .}}{{end}}' "$container")"

    [[ "$first" == true ]] || out+=";"
    first=false
    out+="${index}="

    value="$(sed -n 's/^--work-items-batch-size=//p' <<<"$cmd" | tail -1)"
    out+="window:${value:-default}"
    value="$(sed -n 's/^--dependence-chains-per-batch=//p' <<<"$cmd" | tail -1)"
    out+=",chains:${value:-default}"
    value="$(sed -n 's/^--coprocessor-fhe-threads=//p' <<<"$cmd" | tail -1)"
    out+=",threads:${value:-default}"
    value="$(sed -n 's/^--gpu-streams-per-device=//p' <<<"$cmd" | tail -1)"
    out+=",streams:${value:-default}"
    value="$(sed -n 's/^FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=//p' <<<"$env_json" | tail -1)"
    out+=",adaptive:${value:-default}"
    value="$(sed -n 's/^FHEVM_DCID_BATCH_EXECUTION=//p' <<<"$env_json" | tail -1)"
    out+=",batch:${value:-default}"
  done < <(operator_indexes)
  printf '%s' "$out"
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack environment at $ENV_DIR; bring a stack up first"
  docker inspect "$TEST_CONTAINER" >/dev/null 2>&1 ||
    die "test container $TEST_CONTAINER is not running"

  local -a indexes=()
  mapfile -t indexes < <(operator_indexes)
  local count="${#indexes[@]}"
  [[ "$count" -ge 2 ]] || die "the consensus gate needs at least two operators, found $count"

  local coprocessor_env="$ENV_DIR/coprocessor.env"
  [[ -f "$coprocessor_env" ]] || die "missing $coprocessor_env"
  local gateway_url ciphertext_commits gateway_config
  gateway_url="$(env_value GATEWAY_URL "$coprocessor_env")"
  ciphertext_commits="$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")"
  gateway_config="$(env_value GATEWAY_CONFIG_ADDRESS "$coprocessor_env")"
  [[ -n "$gateway_url" ]] || die "GATEWAY_URL is not set in $coprocessor_env"
  [[ -n "$ciphertext_commits" ]] || die "CIPHERTEXT_COMMITS_ADDRESS is not set in $coprocessor_env"
  [[ -n "$gateway_config" ]] || die "GATEWAY_CONFIG_ADDRESS is not set in $coprocessor_env"

  # The GPU launcher already publishes an audited execution class derived from
  # its build manifest -- binary hashes, GPU UUID, the revision the binaries
  # were built from.  Prefer it over anything reconstructed here, so a GPU run
  # is labelled by what was built rather than by what the working tree says.
  local -a class_env=()
  if [[ -f "$GPU_NODE_CONFIG" ]]; then
    echo "run-materialization-consensus: GPU host workers detected; taking the execution class from the build manifest"
    local exported
    exported="$("$SCRIPT_DIR/gpu-consensus-workers.sh" test-env)" ||
      die "gpu-consensus-workers.sh test-env failed"
    eval "$exported"
    class_env=(
      -e "CONSENSUS_SOFTWARE_REVISION=$CONSENSUS_SOFTWARE_REVISION"
      -e "CONSENSUS_BACKEND_CLASS=$CONSENSUS_BACKEND_CLASS"
      -e "CONSENSUS_HARDWARE_CLASS=$CONSENSUS_HARDWARE_CLASS"
      -e "CONSENSUS_SCHEDULING_CLASSES=${CONSENSUS_SCHEDULING_CLASSES:-}"
      -e "CONSENSUS_DEVICE_COUNT=${CONSENSUS_DEVICE_COUNT:-}"
    )

    # The swap stopped the compose tfhe-workers, so the deferred-transaction
    # probe can no longer reach them at container DNS: it reported every
    # operator unreachable and failed the run, which reads as a wedged fleet.
    local metrics_urls
    metrics_urls="$(gpu_worker_metrics_urls "$count" "$TEST_CONTAINER" coprocessor-and-kms-db)"
    if [[ -n "$metrics_urls" ]]; then
      class_env+=( -e "TFHE_WORKER_METRICS_URLS=$metrics_urls" )
      echo "run-materialization-consensus: host worker metrics at $metrics_urls"
    else
      echo "run-materialization-consensus: WARNING could not resolve the bridge gateway, so the" \
           "deferred-transaction gate will look for host workers at container DNS and report" \
           "them unreachable" >&2
    fi
  else
    local revision
    revision="$(git -C "$REPO_ROOT" rev-parse HEAD)"
    # A dirty tree cannot honestly be labelled with a revision: the binaries in
    # the containers were built from some commit, and the gate records the
    # label as evidence.  Docker topologies run published images, so the marker
    # is explicit rather than silently wrong.
    if [[ -n "$(git -C "$REPO_ROOT" status --porcelain)" ]]; then
      revision="${revision}-dirty"
    fi
    class_env=(
      -e "CONSENSUS_SOFTWARE_REVISION=$revision"
      -e "CONSENSUS_BACKEND_CLASS=cpu"
      -e "CONSENSUS_HARDWARE_CLASS=cpu-$(uname -m)"
      -e "CONSENSUS_SCHEDULING_CLASSES=$(observed_scheduling_classes)"
    )
  fi

  echo "run-materialization-consensus: ${SUITE} suite, ${count} operators, unanimous threshold, gateway ${gateway_url}"
  local index
  while IFS= read -r index; do
    echo "  operator ${index}: $(container_for "$index")"
  done < <(operator_indexes)

  local suite_file suite_flag
  local -a watchdog_env=()
  if [[ "$SUITE" == fork ]]; then
    suite_file=test/consensus/forkConsensus.ts
    suite_flag=RUN_FORK_CONSENSUS
    # A fork topology has operators on competing branches, so per-branch
    # handles never reach a fleet-wide quorum and the global watchdog reports
    # the topology itself as a failure. The suite asserts per branch instead.
    watchdog_env=(-e CONSENSUS_WATCHDOG_DISABLED=1)
  elif [[ "$SUITE" == reorg ]]; then
    suite_file=test/consensus/reorgConsensus.ts
    suite_flag=RUN_REORG_CONSENSUS
    # The watchdog runs here. It used to be disabled because B-1 made it fire on
    # every run of this stack; B-1 is closed -- it was two squash backends
    # serving one queue on the test host -- and this suite takes no operator
    # down and leaves every one of them on the same chain, so a fleet-wide drift
    # check has nothing topological to trip over. If it fires now, that is a
    # finding rather than noise.
  elif [[ "$SUITE" == degraded ]]; then
    suite_file=test/consensus/degradedConsensus.ts
    suite_flag=RUN_DEGRADED_CONSENSUS
    # Still disabled, but no longer because of B-1 (closed). These cases hold an
    # operator down on purpose, so a fleet-wide drift check reports the
    # topology the suite deliberately created rather than a defect -- the same
    # reason the fork topology disables it. The suite asserts over the
    # survivors instead, which is what a degraded-cluster claim is about.
    watchdog_env=(-e CONSENSUS_WATCHDOG_DISABLED=1)
  else
    suite_file=test/consensus/materializationConsensus.ts
    suite_flag=RUN_MATERIALIZATION_CONSENSUS
  fi

  # Two things the suite cannot determine from inside the container.
  #
  # Whether a consensus-detector exists: C7 asserts its behaviour, and a bundle
  # without one made C7 fail for a missing service rather than a missing signal.
  local detector_present=0
  docker inspect "$(container_for 1 2>/dev/null || echo coprocessor1-consensus-detector)" >/dev/null 2>&1 || true
  docker inspect coprocessor1-consensus-detector >/dev/null 2>&1 && detector_present=1
  # Recorded in the run log as a topology fact. No suite branches on it since C7
  # was retired: the detector's presence no longer changes what is asserted.
  echo "consensus-detector present: $detector_present"

  # Whether the RFC-023 attestation readiness probe can run at all: it performs
  # its HEAD by spawning a container, and the test container deliberately has no
  # Docker socket (defect L-3). Detected rather than assumed, and the suite is
  # told to omit that probe only -- byte consensus, digests and the plaintext
  # oracle still run. Silence here would be worse than the omission.
  local attestation_readiness=probe
  if ! docker exec "$TEST_CONTAINER" docker info >/dev/null 2>&1; then
    attestation_readiness=skip
    echo "NOTE: the test container cannot reach the Docker socket, so the RFC-023"
    echo "      attestation-readiness probe is omitted from this run (L-3)."
  fi

  docker exec \
    -e "${suite_flag}=1" \
    -e "KMS_ATTESTATION_READINESS=$attestation_readiness" \
    -e "COPROCESSOR_COUNT=$count" \
    -e "CONSENSUS_THRESHOLD=$count" \
    -e "GATEWAY_RPC_URL=$gateway_url" \
    -e "GATEWAY_CONFIG_ADDRESS=$gateway_config" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$ciphertext_commits" \
    -e "EXPECT_HETEROGENEOUS_SCHEDULING=$EXPECT_HETEROGENEOUS" \
    -e "EXPECT_DEVICE_SPLIT=$EXPECT_DEVICE_SPLIT" \
    "${class_env[@]}" \
    "${watchdog_env[@]}" \
    -e npm_config_update_notifier=false \
    "$TEST_CONTAINER" \
    npx hardhat test "$suite_file" --network "$TEST_NETWORK"

  # The thorough gate is the one whose numbers get quoted, so it is also the one
  # where an unnoticed change of work owner matters most.
  if ! "$SCRIPT_DIR/consensus-validity.sh" locks; then
    [[ "${ALLOW_LOCK_LOSS:-0}" == 1 ]] || die "lock loss during the run; set ALLOW_LOCK_LOSS=1 to accept it"
    echo "  (ALLOW_LOCK_LOSS=1: recorded, not failing)"
  fi
}

main
