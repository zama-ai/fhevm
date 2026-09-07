#!/usr/bin/env bash
# E1's degraded-cluster cases that need an operator taken away.
#
# C4a and C6 cannot live in the e2e container: it has no access to the Docker
# socket, and a test runner able to stop the services it is judging would be a
# poor arrangement anyway. So the fault is applied here and the assertion is
# made by the consensus probe inside the container, between the steps.
#
#   C4a  one coprocessor offline. The survivors must still agree with each
#        other, and a unanimous topology must NOT reach quorum -- if it does,
#        the threshold is not what the topology claims.
#   C4b  the same fleet after the operator returns: quorum forms again, and
#        the returning operator converges on the bytes the others already had.
#   C6   gw-listener restarted while work is in flight: the fleet converges
#        and reaches quorum regardless.
#
#   run-degraded-consensus.sh [--case c4|c6|all]
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENV_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/env"
# shellcheck source=lib/gpu-session.sh
source "${SCRIPT_DIR}/lib/gpu-session.sh"
gpu_normalise_user_bus


# Taking an operator offline has to take down whatever is actually serving its
# queues. Under the GPU swap the three workers are host systemd units and their
# containers are already stopped, so stopping containers would leave the operator
# computing -- C4a would assert "survivors agree while one is offline" against a
# fleet of three. Worse, C4b restores by `docker start`, which would bring a CPU
# worker up beside a live CUDA unit: the mixed-backend split of B-1, created
# mid-run, on a topology whose whole purpose is byte agreement.
#
# Keyed on the GPU session marker rather than a unit's current state, so a unit
# stopped by C4a is still recognised as a unit when C4b restores it.


service_down() {
  local unit; unit="$(gpu_unit_for_container "$1")"
  if [[ -n "$unit" ]]; then systemctl --user stop "$unit" >/dev/null 2>&1
  else docker stop "$1" >/dev/null 2>&1; fi
}

# Restoring a GPU worker is not the mirror image of stopping it. The units are
# transient, so a stopped one is garbage-collected and `systemctl start` fails
# with "Unit not found"; only the launcher knows the invocation to recreate it.
# Getting this wrong did not just fail C4b -- it left the fleet an operator short
# for every suite that followed on the same stack.
service_up() {
  local unit kind index
  unit="$(gpu_unit_for_container "$1")"
  if [[ -z "$unit" ]]; then docker start "$1" >/dev/null 2>&1; return; fi
  kind="${unit#fhevm-gpu-consensus-}"; kind="${kind%-*}"
  index="${unit##*-}"
  "$SCRIPT_DIR/gpu-consensus-workers.sh" restart-unit "$kind" "$index" >/dev/null 2>&1
}

readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
readonly TEST_NETWORK="${TEST_NETWORK:-staging}"
# Quorum is opt-in, and off by default, because of defect B-1: one operator
# emits a different SNS digest for computed handles, so a unanimous topology
# cannot form quorum for them at all. Demanding quorum here would fail every
# cell for the same unrelated reason and the run would measure nothing. The
# byte, digest and provenance comparison across operators still runs and is
# still mandatory -- that is what detects a fault-induced divergence.
readonly EXPECT_QUORUM="${DEGRADED_EXPECT_QUORUM:-0}"
# Operator taken down for C4a/C4b. Index 2 by default so operators 0 and 1
# remain as the comparison pair.
readonly VICTIM="${DEGRADED_VICTIM_OPERATOR:-2}"

CASE=all
[[ $# -gt 0 ]] && case "$1" in
  --case) CASE="${2:?--case needs a value}" ;;
  *) echo "usage: run-degraded-consensus.sh [--case c4|c6|all]" >&2; exit 2 ;;
esac

die() { echo "degraded-consensus: $*" >&2; exit 1; }
log() { printf '\n=== %s\n' "$*"; }

env_value() { sed -n "s/^$1=//p" "$2" | tail -1; }

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

# Container names for one operator's worker set.

operator_services() {
  local index="$1" prefix
  [[ "$index" == 0 ]] && prefix="coprocessor" || prefix="coprocessor${index}"
  echo "${prefix}-tfhe-worker ${prefix}-zkproof-worker ${prefix}-sns-worker ${prefix}-transaction-sender ${prefix}-host-listener ${prefix}-gw-listener"
}

# Docker's embedded resolver inside the test container gets wedged by repeated
# container churn: after enough stop/start cycles it stops resolving ANY name,
# including `db`, and every later probe then fails with EAI_AGAIN. That is the
# harness breaking, not the fleet, and it produced three false FAILs before it
# was understood. Check the resolver before each probe and restart the
# container if it has died -- a cell must fail for its own fault, not for this.
ensure_test_container_resolves() {
  docker exec "$TEST_CONTAINER" getent hosts db >/dev/null 2>&1 && return 0
  echo "  (test container resolver is wedged; restarting it)" >&2
  docker restart "$TEST_CONTAINER" >/dev/null 2>&1
  local i
  for i in $(seq 1 24); do
    docker exec "$TEST_CONTAINER" getent hosts db >/dev/null 2>&1 && return 0
    sleep 5
  done
  return 1
}

run_probe() {
  ensure_test_container_resolves || die "test container cannot resolve service names even after a restart"
  local label="$1" exclude="$2" expect_quorum="$3"
  local coprocessor_env="$ENV_DIR/coprocessor.env"
  docker exec \
    -e RUN_CONSENSUS_PROBE=1 \
    -e CONSENSUS_WATCHDOG_DISABLED=1 \
    -e "COPROCESSOR_COUNT=$(operator_count)" \
    -e "PROBE_LABEL=$label" \
    -e "PROBE_EXCLUDE_OPERATORS=$exclude" \
    -e "PROBE_EXPECT_QUORUM=$expect_quorum" \
    -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL "$coprocessor_env")" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")" \
    -e "TFHE_WORKER_METRICS_URLS=$(gpu_worker_metrics_urls "$(operator_count)" "$TEST_CONTAINER" coprocessor-and-kms-db)" \
    -e npm_config_update_notifier=false \
    "$TEST_CONTAINER" \
    npx hardhat test test/consensus/consensusProbe.ts --network "$TEST_NETWORK" 2>&1
}

passed() { grep -qE "^\s+1 passing" <<<"$1"; }

results=()
record() { results+=("$1|$2|${3:-}"); echo "[$1] $2 ${3:+- $3}"; }

case_c4() {
  local -a services
  read -r -a services <<<"$(operator_services "$VICTIM")"
  log "C4a — taking operator $VICTIM offline (${#services[@]} services)"
  local svc
  for svc in "${services[@]}"; do service_down "$svc"; done
  sleep 15

  # Survivors must still agree with each other. Quorum must NOT form: the
  # topology is unanimous, so a missing operator means a missing submission.
  local out
  out="$(run_probe c4a-degraded "$VICTIM" 0)"
  if passed "$out"; then
    record C4a-survivors-agree PASS
  else
    record C4a-survivors-agree FAIL "$(grep -m1 -E 'AssertionError|Error:|timed out' <<<"$out" | cut -c1-160)"
  fi

  log "C4b — restoring operator $VICTIM"
  for svc in "${services[@]}"; do service_up "$svc"; done
  sleep 45

  out="$(run_probe c4b-recovered "" "$EXPECT_QUORUM")"
  if passed "$out"; then
    record C4b-quorum-returns PASS
  else
    record C4b-quorum-returns FAIL "$(grep -m1 -E 'AssertionError|Error:|timed out' <<<"$out" | cut -c1-160)"
  fi
}

case_c6() {
  log "C6 — restarting every gw-listener while work is in flight"
  local index prefix
  for index in $(seq 0 $(($(operator_count) - 1))); do
    [[ "$index" == 0 ]] && prefix="coprocessor" || prefix="coprocessor${index}"
    docker restart "${prefix}-gw-listener" >/dev/null 2>&1
  done
  sleep 30
  local out
  out="$(run_probe c6-gw-listener-restart "" "$EXPECT_QUORUM")"
  if passed "$out"; then
    record C6-gw-listener-restart PASS
  else
    record C6-gw-listener-restart FAIL "$(grep -m1 -E 'AssertionError|Error:|timed out' <<<"$out" | cut -c1-160)"
  fi
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR"
  docker inspect "$TEST_CONTAINER" >/dev/null 2>&1 || die "test container $TEST_CONTAINER is not running"

  log "baseline"
  local base
  base="$(run_probe baseline "" "$EXPECT_QUORUM")"
  # Same reason as the failure matrix: a queue served twice invalidates every
  # cell, and probing cannot see it.
  if gpu_session_active && ! "$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1; then
    die "a queue is served twice before any operator was removed; the fleet is split across backends"
  fi
  passed "$base" || die "baseline probe failed before any operator was removed. Usually that is the fleet disagreeing, but the probe also fails when its own run-validity gates cannot read the stack -- on GPU that meant worker metrics at container DNS that the swap had stopped. The probe output above says which: $(grep -m1 -E 'InvalidRunError|AssertionError' <<<"$base" | cut -c1-160)"
  record baseline PASS

  [[ "$CASE" == all || "$CASE" == c4 ]] && case_c4
  [[ "$CASE" == all || "$CASE" == c6 ]] && case_c6

  log "results"
  local failures=0 entry id status detail
  for entry in "${results[@]}"; do
    IFS='|' read -r id status detail <<<"$entry"
    printf '%-28s %-6s %s\n' "$id" "$status" "$detail"
    [[ "$status" == FAIL ]] && failures=$((failures + 1))
  done

  # Gated, not merely reported: unlike the failure matrix, nothing here kills a
  # worker, so a lease that lapsed did so on its own. Work was then recomputed
  # by a second worker, and these cases are about what a *degraded cluster*
  # agrees on -- a claim that reads differently if the work silently changed
  # owner underneath it. ALLOW_LOCK_LOSS=1 for a run that expects it anyway.
  if ! "$SCRIPT_DIR/consensus-validity.sh" locks; then
    if [[ "${ALLOW_LOCK_LOSS:-0}" == 1 ]]; then
      echo "  (ALLOW_LOCK_LOSS=1: recorded, not failing)"
    else
      echo "  set ALLOW_LOCK_LOSS=1 to accept this run anyway" >&2
      failures=$((failures + 1))
    fi
  fi

  [[ "$failures" -eq 0 ]]
}

main
