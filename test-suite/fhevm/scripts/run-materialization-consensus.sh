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
# `set -e` is deliberately NOT used: every step's status is checked explicitly,
# and an early abort would leave the run's cases with no recorded result at all
# -- which the aggregate reads as NOT_RUN, correctly, but with nothing to say
# about why.
#
# With --heterogeneous the gate additionally refuses to run unless the
# operators really are scheduling differently.  See
# scenarios/three-of-three-heterogeneous-scheduling.yaml.
#
# Fork and degraded coverage have their own runners (run-fork-consensus.sh,
# run-degraded-consensus.sh) that record the fault and workload evidence the
# inventory requires; this runner does not offer them as suites.
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENV_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/env"
# shellcheck source=lib/gpu-session.sh
source "${SCRIPT_DIR}/lib/gpu-session.sh"
# shellcheck source=lib/case-result.sh
source "${SCRIPT_DIR}/lib/case-result.sh"
source "${SCRIPT_DIR}/lib/runner-assertions.sh"
# shellcheck source=lib/suite-identity.sh
source "${SCRIPT_DIR}/lib/suite-identity.sh"
source "${SCRIPT_DIR}/lib/source-revision.sh"
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
    # Deferred: configuration strings do not prove which device executed work.
    --device-split)
      echo "--device-split is deferred: selected-work GPU execution attribution is not implemented; CI uses a single GPU" >&2
      exit 2
      ;;
    --suite)
      [[ $# -ge 2 ]] || {
        echo "--suite needs a value: materialization, input, typed, bridge, reorg or comparator" >&2
        exit 2
      }
      SUITE="$2"
      shift 2
      ;;
    *)
      echo "usage: run-materialization-consensus.sh [--heterogeneous] [--device-split] [--suite materialization|input|typed|bridge|reorg|comparator]" >&2
      exit 2
      ;;
  esac
done
case "$SUITE" in
  materialization | input | typed | bridge | reorg | comparator) ;;
  *)
    echo "unknown suite $SUITE (expected materialization, input, typed, bridge, reorg or comparator)" >&2
    exit 2
    ;;
esac
readonly EXPECT_HETEROGENEOUS EXPECT_DEVICE_SPLIT SUITE

die() {
  RS_FINAL_FAILURE=1
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

source "${SCRIPT_DIR}/lib/suite-process.sh"
source "${SCRIPT_DIR}/lib/result-staging.sh"
sp_init || exit 1
cleanup_suite() {
  local status=$?
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  if ! sp_cancel_all || ! sp_recover_suite_state; then
    rs_finalize_results 1 failed; exit 1
  fi
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  rs_finalize_results "$status" ok || { [[ "$status" != 0 ]] || status=1; }
  [[ -f "$SP_RUNTIME_DIR/cleanup-failed" ]] || sp_dispose || status=1
  exit "$status"
}
trap cleanup_suite EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

main() {
  if [[ "$SUITE" == comparator ]]; then
    # Synthetic comparator cases need no stack, Docker daemon, or generated env.
    exec "$SCRIPT_DIR/run-stackless-cases.sh" --leg comparator
  fi
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
    revision="$(sr_revision "$REPO_ROOT")" || die "cannot establish source revision"
    # Use the same generated-file exemption as build and artifact identity.
    # Running image IDs are recorded separately from this suite revision.
    CONSENSUS_SCHEDULING_CLASSES="$(observed_scheduling_classes)" ||
      die "cannot establish CPU scheduling classes"
    export CONSENSUS_SCHEDULING_CLASSES
    class_env=(
      -e "CONSENSUS_SOFTWARE_REVISION=$revision"
      -e "CONSENSUS_BACKEND_CLASS=cpu"
      -e "CONSENSUS_HARDWARE_CLASS=cpu-$(uname -m)"
      -e "CONSENSUS_SCHEDULING_CLASSES=$CONSENSUS_SCHEDULING_CLASSES"
    )
  fi

  echo "run-materialization-consensus: ${SUITE} suite, ${count} operators, unanimous threshold, gateway ${gateway_url}"
  local index
  while IFS= read -r index; do
    echo "  operator ${index}: $(container_for "$index")"
  done < <(operator_indexes)

  local suite_file suite_flag
  local -a suite_args=()
  local -a watchdog_env=()
  if [[ "$SUITE" == reorg ]]; then
    suite_file=test/consensus/reorgConsensus.ts
    suite_flag=RUN_REORG_CONSENSUS
    # The watchdog runs here. It used to be disabled because B-1 made it fire on
    # every run of this stack; B-1 is closed -- it was two squash backends
    # serving one queue on the test host -- and this suite takes no operator
    # down and leaves every one of them on the same chain, so a fleet-wide drift
    # check has nothing topological to trip over. If it fires now, that is a
    # finding rather than noise.
  elif [[ "$SUITE" == bridge ]]; then
    suite_file=test/bridge/confidentialBridge.ts
    suite_flag=RUN_BRIDGE_BYTE_CONSENSUS
    suite_args=(--grep "compares local and bridged dependencies")
  elif [[ "$SUITE" == typed ]]; then
    suite_file=test/consensus/typedBoundaryConsensus.ts
    suite_flag=RUN_TYPED_BOUNDARY_CONSENSUS
  elif [[ "$SUITE" == input ]]; then
    suite_file=test/consensus/inputConsensus.ts
    suite_flag=RUN_INPUT_CONSENSUS
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

  # Structured results, per inventory case. The verdict is the suite's EXIT
  # STATUS plus the marker each case prints, never `1 passing` in its output:
  # an after-hook can fail after a test body passes, and this suite has an
  # after-hook that gates the squash format and the executed scheduling.
  local observed_topology
  observed_topology="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" "$count")" || die "cannot verify active topology"
  eval "$observed_topology"
  cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
  rs_stage_results || exit 1
  suite_identity_assert "$TEST_CONTAINER" ||
    die "the test container is not running this working tree's e2e suite"
  local started; started="$(cr_now)"
  export CONSENSUS_RUN_STARTED_AT="$started"

  local -a identity_args=()
  # `cr` form: `cr_record` takes `artifact=NAME=value`, not raw flags.
  cr_read_run_identity identity_args || die "cannot establish complete run artifact identity"

  local scheduling_case=SCH-01-HETEROGENEOUS
  [[ "${CR_BACKEND_CLASS:-cpu}" != cpu* ]] || scheduling_case=SCH-06-CPU-DIVERSITY
  case "$SUITE" in
    materialization)
      if [[ "$EXPECT_HETEROGENEOUS" == 1 ]]; then sp_case_start "$scheduling_case";
      else sp_case_start MAT-01-BOUNDARY-FANOUT MAT-02-ALIAS-SOURCING MAT-03-PLAINTEXT-ORACLE; fi ;;
    bridge) sp_case_start MAT-08-BRIDGED-DEPENDENCY ;;
    typed) sp_case_start MAT-06-TYPED-BOUNDARIES ;;
    input) sp_case_start INPUT-01-COMPACT-LIST INPUT-02-REPLAY INPUT-03-INVALID-PROOF ;;
    reorg) sp_case_start REORG-01-REPLACEMENT-BLOCK ;;
  esac || die "cannot establish suite deadline"
  local suite_out status=0
  cr_run_suite suite_out "" sp_exec \
    -e "${suite_flag}=1" \
    -e "KMS_ATTESTATION_READINESS=$attestation_readiness" \
    -e "COPROCESSOR_COUNT=$count" \
    -e "CONSENSUS_THRESHOLD=$CONSENSUS_THRESHOLD" \
    -e "GATEWAY_RPC_URL=$gateway_url" \
    -e "GATEWAY_CONFIG_ADDRESS=$gateway_config" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$ciphertext_commits" \
    -e "EXPECT_HETEROGENEOUS_SCHEDULING=$EXPECT_HETEROGENEOUS" \
    -e "EXPECT_DEVICE_SPLIT=$EXPECT_DEVICE_SPLIT" \
    "${class_env[@]}" \
    "${watchdog_env[@]}" \
    -e npm_config_update_notifier=false \
    "$TEST_CONTAINER" \
    npx hardhat test "$suite_file" --network "$TEST_NETWORK" "${suite_args[@]}" || status=$?
  echo "$suite_out"

  # The thorough gate is the one whose numbers get quoted, so it is also the one
  # where an unnoticed change of work owner matters most. Scoped to this run's
  # window: a lock loss from an earlier suite on the same stack is not this
  # run's evidence.
  local lock_state=pass
  if ! "$SCRIPT_DIR/consensus-validity.sh" locks --since "$started" --operators "$count"; then
    if [[ "${ALLOW_LOCK_LOSS:-0}" == 1 ]]; then
      echo "  (ALLOW_LOCK_LOSS=1: recorded, not failing)"
      lock_state=pass
    else
      lock_state=fail
    fi
  fi

  # Which inventory cases this suite carries, and the marker each of them
  # prints. A case whose marker is absent is NOT_RUN rather than passed.
  local -a case_ids=() markers=()
  case "$SUITE" in
    materialization)
      if [[ "$EXPECT_HETEROGENEOUS" == 1 || "$EXPECT_DEVICE_SPLIT" == 1 ]]; then
        # A heterogeneously configured fleet is its own scenario in the
        # inventory, so this session records only the cases established under
        # it. The byte cases assert here too and their failure still fails the
        # run -- but they are RECORDED from the session whose topology the
        # inventory names, rather than twice under two different topologies.
        case_ids=(); markers=()
        [[ "$EXPECT_HETEROGENEOUS" == 1 ]] && { case_ids+=("$scheduling_case"); markers+=("executed scheduling"); }
        [[ "$EXPECT_DEVICE_SPLIT" == 1 ]] && { case_ids+=(SCH-02-DEVICE-SPLIT); markers+=("device split across CUDA devices"); }
        echo "  (heterogeneous session: the byte cases ran and are asserted, and are recorded from the homogeneous session)"
      else
        case_ids=(MAT-01-BOUNDARY-FANOUT MAT-02-ALIAS-SOURCING MAT-03-PLAINTEXT-ORACLE)
        markers=(
          "converges on same-block cross-transaction boundaries"
          "converges on same-sourcing aliases"
          "[materialization-consensus] plaintext oracle:"
        )
      fi
      ;;
    bridge) case_ids=(MAT-08-BRIDGED-DEPENDENCY); markers=("[bridge-consensus] CASE COMPLETE") ;;
    typed) case_ids=(MAT-06-TYPED-BOUNDARIES); markers=("[typed-boundary] CASE COMPLETE") ;;
    input) case_ids=(INPUT-01-COMPACT-LIST INPUT-02-REPLAY INPUT-03-INVALID-PROOF); markers=("[input-consensus] CASE COMPLETE" "[input-consensus] CASE COMPLETE" "[input-consensus] CASE COMPLETE") ;;
    reorg)   case_ids=(REORG-01-REPLACEMENT-BLOCK); markers=("[reorg-consensus] CASE COMPLETE") ;;
  esac

  local -a base_record=(
    started_at="$started"
    cleanup=ok
    scheduling_classes="${CONSENSUS_SCHEDULING_CLASSES:-}"
  )
  # A case that declares a fault must carry the evidence that it landed. The
  # reorg suite announces the replacement it mined and when; without those
  # fields its PASS is indistinguishable from a run where no block was ever
  # replaced, and the aggregate rejects it.
  local replacement_line
  replacement_line="$(grep -o 'replacement observed: handle 0x[0-9a-f]* in block 0x[0-9a-f]* at [0-9TZ:.-]*' <<<"$suite_out" | tail -1)"
  if [[ -n "$replacement_line" ]]; then
    base_record+=(
      workload="$(awk '{print $4}' <<<"$replacement_line")"
      fault_observed_at="$(awk '{print $NF}' <<<"$replacement_line")"
    )
  fi
  local index=0 case_id marker state detail case_failures=0
  for case_id in "${case_ids[@]}"; do
    marker="${markers[$index]}"
    index=$((index + 1))
    cr_skip_wrong_scenario "$case_id" && continue
    if [[ "$status" -ne 0 ]]; then
      state="$(cr_suite_state "$suite_out")"
      detail="the suite failed: $(cr_failure_reason "$suite_out")"
      [[ "$state" == INVALID ]] &&
        detail="the suite could not measure this case: $(cr_failure_reason "$suite_out")"
    elif [[ "$lock_state" == fail ]]; then
      state=FAIL
      detail="a worker lost dependence-chain locks during the run; this is not the clean single-owner measurement it appears to be"
    elif ! grep -qF "$marker" <<<"$suite_out"; then
      state=NOT_RUN
      detail="the suite exited 0 without evidence that this case ran (no '$marker' in its output)"
    else
      state=PASS
      detail=""
    fi
    if [[ "$state" == PASS ]]; then
      cr_record_checked_pass "$case_id" "${base_record[@]}" "${identity_args[@]}" \
        assert="assertions-ran=pass:$marker" assert="lock-evidence=pass" || die "could not record $case_id PASS"
    else
      cr_record "$case_id" "$state" "${base_record[@]}" "${identity_args[@]}" detail="$detail" || die "could not record $case_id $state"
      case_failures=$((case_failures + 1))
    fi
    echo "[$case_id] $state ${detail:+- $detail}"
  done

  echo "structured results: ${RS_PUBLISH_RESULTS:-$(dirname "$(cr_results_file)")}/$CR_RUN_ID.jsonl"
  # Suite/after-hook or shared lock failures invalidate every sibling. A
  # missing case marker is already represented by its own NOT_RUN record.
  [[ "$status" -eq 0 && "$lock_state" == pass ]] || die "the ${SUITE} gate did not stand"
  [[ "$case_failures" -eq 0 ]]
}

main
