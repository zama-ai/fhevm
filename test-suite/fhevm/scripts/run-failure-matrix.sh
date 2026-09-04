#!/usr/bin/env bash
# Failure-mode matrix: inject a fault, heal it, then ask whether the operators
# still agree on the bytes.
#
# The coverage register sets one rule for this matrix, and it is the reason the
# script exists in this shape: EVERY CELL ENDS IN THE CONSENSUS ASSERTION, not
# in "the service came back". A restart that recovers into different bytes is
# the failure these cells are for, and a liveness check walks straight past it.
# So each cell runs the consensus probe afterwards, and a cell only passes if
# the probe does.
#
# Fault injection lives here rather than in the tests because the e2e container
# cannot reach the Docker socket -- and handing a test runner the authority to
# stop other services would be a poor trade for the convenience. The container
# submits work and reads databases; this script breaks things.
#
#   run-failure-matrix.sh [--column crash|stall|data|db|all] [--only <cell-id>]
#                         [--list] [--keep-going]
#
# Exit status is 0 only if every selected cell passed. Findings are printed as
# a table at the end and written to a report file for the bug tracker.
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENV_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/env"
# shellcheck source=lib/gpu-session.sh
source "${SCRIPT_DIR}/lib/gpu-session.sh"
gpu_normalise_user_bus

readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
readonly TEST_NETWORK="${TEST_NETWORK:-staging}"
# Quorum is opt-in, and off by default, because of defect B-1: one operator
# emits a different SNS digest for computed handles, so a unanimous topology
# cannot form quorum for them at all. Demanding quorum here would fail every
# cell for the same unrelated reason and the run would measure nothing. The
# byte, digest and provenance comparison across operators still runs and is
# still mandatory -- that is what detects a fault-induced divergence.
readonly EXPECT_QUORUM="${MATRIX_EXPECT_QUORUM:-0}"
readonly REPORT_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/failure-matrix"
readonly SETTLE_SECONDS="${MATRIX_SETTLE_SECONDS:-20}"
readonly RECOVER_SECONDS="${MATRIX_RECOVER_SECONDS:-30}"

COLUMN=all
ONLY=""
LIST_ONLY=0
KEEP_GOING=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --column) COLUMN="${2:?--column needs a value}"; shift 2 ;;
    --only) ONLY="${2:?--only needs a cell id}"; shift 2 ;;
    --list) LIST_ONLY=1; shift ;;
    --keep-going) KEEP_GOING=1; shift ;;
    *) echo "usage: run-failure-matrix.sh [--column crash|stall|data|db|all] [--only <cell>] [--list] [--keep-going]" >&2
       echo "  note: --column all omits the db cells; ask for them explicitly" >&2; exit 2 ;;
  esac
done

die() { echo "failure-matrix: $*" >&2; exit 1; }
log() { printf '\n=== %s\n' "$*"; }

# --------------------------------------------------------------------------
# The matrix.
#
# id | column | service container | fault | heal
#
# `fault` and `heal` name shell functions below. Services absent from the
# running topology are skipped with a recorded reason rather than passed: a
# cell that never ran must not read as a green one.
# --------------------------------------------------------------------------
MATRIX=(
  "host-listener-crash|crash|coprocessor1-host-listener|fault_kill|heal_start"
  "host-listener-poller-crash|crash|coprocessor1-host-listener-poller|fault_kill|heal_start"
  "host-listener-consumer-crash|crash|coprocessor1-host-listener-consumer|fault_kill|heal_start"
  "gw-listener-crash|crash|coprocessor1-gw-listener|fault_kill|heal_start"
  "tfhe-worker-crash|crash|coprocessor1-tfhe-worker|fault_kill|heal_start"
  "zkproof-worker-crash|crash|coprocessor1-zkproof-worker|fault_kill|heal_start"
  "sns-worker-crash|crash|coprocessor1-sns-worker|fault_kill|heal_start"
  "transaction-sender-crash|crash|coprocessor1-transaction-sender|fault_kill|heal_start"
  "consensus-detector-crash|crash|coprocessor1-consensus-detector|fault_kill|heal_start"

  "tfhe-worker-stall|stall|coprocessor1-tfhe-worker|fault_pause|heal_unpause"
  "sns-worker-stall|stall|coprocessor1-sns-worker|fault_pause|heal_unpause"
  "gw-listener-stall|stall|coprocessor1-gw-listener|fault_pause|heal_unpause"
  "host-listener-stall|stall|coprocessor1-host-listener|fault_pause|heal_unpause"
  "transaction-sender-stall|stall|coprocessor1-transaction-sender|fault_pause|heal_unpause"

  "kms-connector-crash|crash|kms-connector-kms-worker|fault_kill|heal_start"
  "relayer-crash|crash|fhevm-relayer|fault_kill|heal_start"
  "upgrade-controller-crash|crash|coprocessor1-upgrade-controller|fault_kill|heal_start"

  "object-storage-outage|data|fhevm-minio|fault_stop|heal_start"
  "broker-outage|data|listener-redis|fault_stop|heal_start"
  "object-storage-stall|data|fhevm-minio|fault_pause|heal_unpause"

  # The shared database is every operator's data layer at once, and taking it
  # away has already been observed to wedge a stack in this environment: the
  # listeners cached the authentication failure and exited, and the fleet did
  # not recover on its own. Two fixes since then should change that -- the
  # host-listener no longer spins forever on a pool its own reconnect closed
  # (D-2), and compose now restarts a service that exits non-zero (L-5) -- and
  # `fleet_recovery_report` says whether it actually did. The cells stay behind
  # their own column until an unattended run has demonstrated it, so one
  # destructive experiment still cannot abort a matrix run that was otherwise
  # going to complete.
  "database-outage|db|coprocessor-and-kms-db|fault_stop|heal_start"
  "database-stall|db|coprocessor-and-kms-db|fault_pause|heal_unpause"
)

# Under the GPU swap the tfhe, zkproof and sns workers are host systemd units and
# their containers are deliberately stopped, so a cell that injects into the
# container would be faulting something that is already down and proving nothing.
# The units are transient `systemd-run --user` services carrying
# `Restart=on-failure` / `RestartSec=2`, the same supervision contract compose
# gives the containers, so the cells stay meaningful -- they just have to be
# addressed differently.
#
# `coprocessor-tfhe-worker` is operator 0 and `coprocessorN-tfhe-worker` is
# operator N; the launcher names units `fhevm-gpu-consensus-<kind>-<index>`.
# systemd --user is addressed through this user's runtime directory, and a
# detached shell can inherit another user's. If that happens while GPU units are
# serving the queues, `gpu_unit_for_container` finds nothing, the cell falls back
# to Docker, and it faults a container the swap already stopped -- passing while
# testing nothing. Normalise the address, then refuse to inject into a target
# that is neither a running container nor an active unit.

require_faultable() {
  local target="$1" unit state
  unit="$(gpu_unit_for_container "$target")"
  [[ -n "$unit" ]] && return 0
  state="$(docker inspect -f '{{.State.Status}}' "$target" 2>/dev/null || echo missing)"
  [[ "$state" == running ]] && return 0
  die "cannot inject into $target: it is $state and no active GPU unit serves it, so the cell would prove nothing"
}




# `systemctl kill` leaves Restart=on-failure free to act, unlike `docker kill`
# which Docker treats as a manual stop and suppresses the policy for. The cell
# still passes either way: it excludes the injected service from the fleet check
# and heals it explicitly, and a unit that restarts itself is the supervision
# working rather than a fault.
fault_kill() {
  require_faultable "$1"
  local unit; unit="$(gpu_unit_for_container "$1")"
  if [[ -n "$unit" ]]; then systemctl --user kill --signal=SIGKILL "$unit" >/dev/null 2>&1
  else docker kill "$1" >/dev/null 2>&1; fi
}
fault_stop() {
  require_faultable "$1"
  local unit; unit="$(gpu_unit_for_container "$1")"
  if [[ -n "$unit" ]]; then systemctl --user stop "$unit" >/dev/null 2>&1
  else docker stop "$1" >/dev/null 2>&1; fi
}
fault_pause() {
  require_faultable "$1"
  local unit pid; unit="$(gpu_unit_for_container "$1")"
  if [[ -n "$unit" ]]; then
    pid="$(unit_main_pid "$unit")"
    [[ -n "$pid" && "$pid" != 0 ]] && kill -STOP "$pid" >/dev/null 2>&1
  else docker pause "$1" >/dev/null 2>&1; fi
}
# `systemctl start` cannot restore a unit that was stopped: the units are
# transient (`systemd-run --collect`), so stopping one garbage-collects it and
# the name stops resolving. Today no worker cell uses fault_stop -- the crash
# cells SIGKILL, which Restart=on-failure handles without collection, and the
# stall cells SIGSTOP -- so this path happens to work. That is luck, not design,
# and the degraded suite already learned it the expensive way (G-9). Route
# through the launcher, which owns the invocation.
heal_start() {
  local unit kind index
  unit="$(gpu_unit_for_container "$1")"
  if [[ -z "$unit" ]]; then docker start "$1" >/dev/null 2>&1; return; fi
  systemctl --user is-active --quiet "$unit" 2>/dev/null && return 0
  kind="${unit#fhevm-gpu-consensus-}"; kind="${kind%-*}"
  index="${unit##*-}"
  "$SCRIPT_DIR/gpu-consensus-workers.sh" restart-unit "$kind" "$index" >/dev/null 2>&1
}
heal_unpause() {
  local unit pid; unit="$(gpu_unit_for_container "$1")"
  if [[ -n "$unit" ]]; then
    pid="$(unit_main_pid "$unit")"
    [[ -n "$pid" && "$pid" != 0 ]] && kill -CONT "$pid" >/dev/null 2>&1
  else docker unpause "$1" >/dev/null 2>&1; fi
}

# `heal_start` on a unit systemd already restarted is a no-op, and a stopped unit
# has no MainPID to resume, so a paused-then-stopped unit needs starting. Both
# heals are therefore safe to call unconditionally, as the cells do.

# Did the fleet put itself back together?
#
# The coprocessor services implement exit-for-restart: on a fatal error they log,
# flush telemetry and exit non-zero, expecting a supervisor. Compose now gives
# them `restart: "on-failure:10"`, so that path finally works -- and this is what
# observes it, because until now no test in this topology could (Consensus Defect
# Log, L-5). Docker suppresses the policy after an explicit `docker kill`/`stop`,
# so the service a cell injected into is excluded: it stays down by design and
# its own heal step brings it back.
#
# Prints nothing when the fleet is intact. Otherwise names what is still down --
# collateral damage that would otherwise surface only as an unexplained probe
# timeout -- and notes any service that restarted itself, which is the recovery
# working rather than a fault.
fleet_recovery_report() {
  local injected="$1" name status restarts down="" recovered=""
  while IFS= read -r name; do
    [[ "$name" == "$injected" ]] && continue
    # A worker whose queue is served by a GPU host unit has its container
    # stopped on purpose. Judge the unit, not the shell it replaced, or every
    # GPU run reports three permanent casualties.
    local gpu_unit; gpu_unit="$(gpu_unit_for_container "$name")"
    if [[ -n "$gpu_unit" ]]; then
      local ustate urestarts
      ustate="$(systemctl --user show "$gpu_unit" --property=ActiveState --value 2>/dev/null)"
      urestarts="$(systemctl --user show "$gpu_unit" --property=NRestarts --value 2>/dev/null)"
      [[ "$ustate" != active ]] && down="$down $name(unit:$ustate)"
      [[ "${urestarts:-0}" -gt 0 ]] && recovered="$recovered $name(unit x$urestarts)"
      continue
    fi
    status="$(docker inspect -f '{{.State.Status}}' "$name" 2>/dev/null || echo missing)"
    restarts="$(docker inspect -f '{{.RestartCount}}' "$name" 2>/dev/null || echo 0)"
    [[ "$status" != running ]] && down="$down $name($status)"
    [[ "${restarts:-0}" -gt 0 ]] && recovered="$recovered $name(x$restarts)"
  done < <(docker ps -a --format '{{.Names}}' \
             | grep -E '^coprocessor[0-9]*-(host-listener|host-listener-poller|host-listener-consumer|gw-listener|tfhe-worker|zkproof-worker|sns-worker|transaction-sender|consensus-detector|upgrade-controller)$')
  [[ -n "$recovered" ]] && printf 'self-recovered:%s' "$recovered"
  [[ -n "$down" && -n "$recovered" ]] && printf '; '
  [[ -n "$down" ]] && printf 'STILL DOWN:%s' "$down"
  return 0
}

env_value() { sed -n "s/^$1=//p" "$2" | tail -1; }

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
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

# The consensus assertion every cell ends in.
run_probe() {
  ensure_test_container_resolves || die "test container cannot resolve service names even after a restart"
  local label="$1" exclude="${2:-}" expect_quorum="${3:-1}" canary="${4:-0}"
  local coprocessor_env="$ENV_DIR/coprocessor.env"
  docker exec \
    -e RUN_CONSENSUS_PROBE=1 \
    -e CONSENSUS_WATCHDOG_DISABLED=1 \
    -e "COPROCESSOR_COUNT=$(operator_count)" \
    -e "PROBE_LABEL=$label" \
    -e "PROBE_EXCLUDE_OPERATORS=$exclude" \
    -e "PROBE_EXPECT_QUORUM=$expect_quorum" \
    -e "PROBE_CANARY=$canary" \
    -e "PROBE_CONTRACT_ADDRESS=${PROBE_CONTRACT_ADDRESS:-}" \
    -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL "$coprocessor_env")" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")" \
    -e "TFHE_WORKER_METRICS_URLS=$(gpu_worker_metrics_urls "$(operator_count)" "$TEST_CONTAINER" coprocessor-and-kms-db)" \
    -e npm_config_update_notifier=false \
    "$TEST_CONTAINER" \
    npx hardhat test test/consensus/consensusProbe.ts --network "$TEST_NETWORK" 2>&1
}

main() {
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR; bring one up first"

  if [[ "$LIST_ONLY" == 1 ]]; then
    printf '%-32s %-7s %s\n' "CELL" "COLUMN" "SERVICE"
    local row
    for row in "${MATRIX[@]}"; do
      IFS='|' read -r id column service _ _ <<<"$row"
      printf '%-32s %-7s %s\n' "$id" "$column" "$service"
    done
    exit 0
  fi

  docker inspect "$TEST_CONTAINER" >/dev/null 2>&1 || die "test container $TEST_CONTAINER is not running"
  mkdir -p "$REPORT_DIR"
  local report="$REPORT_DIR/matrix-$(date -u +%Y%m%dT%H%M%SZ).txt"

  # A baseline first. If the fleet does not agree on a healthy stack, every
  # cell after this would report a fault it did not cause.
  log "baseline (no fault injected)"
  # A doubly-served queue makes every cell's verdict meaningless, and it is the
  # one fault the matrix cannot detect by probing: the fleet looks healthy until
  # two workers disagree. Ask the launcher before spending twenty minutes.
  if gpu_session_active && ! "$SCRIPT_DIR/gpu-consensus-workers.sh" conflicts >/dev/null 2>&1; then
    die "a queue is served twice before any fault was injected; the fleet is split across backends (gpu-consensus-workers.sh conflicts names it)"
  fi

  # Capture rather than pipe: under `pipefail` the pipeline would inherit
  # docker exec's non-zero status even when the probe itself passed, so a
  # healthy baseline could be reported as a broken stack.
  local baseline_out
  # The canary rides the baseline: one deliberate falsification per matrix run.
  # Every cell ends in the same shared comparator, so proving once per run that
  # a poisoned digest is rejected covers all of them -- and doing it per cell
  # would pay that cost twenty-two times for no extra information.
  baseline_out="$(run_probe baseline "" "$EXPECT_QUORUM" 1)"
  echo "$baseline_out" >>"$report"
  if ! grep -qE "^\s+1 passing" <<<"$baseline_out"; then
    echo "$baseline_out" | tail -25
    die "baseline probe failed before any fault was injected. Usually that is the stack disagreeing, but the probe also fails when its own run-validity gates cannot read it -- on GPU that meant worker metrics at container DNS that the swap had stopped. The probe output above says which"
  fi
  echo "baseline: PASS" | tee -a "$report"

  local -a results=()
  local row id column service fault heal status detail
  for row in "${MATRIX[@]}"; do
    IFS='|' read -r id column service fault heal <<<"$row"
    # `all` deliberately excludes the db column; see the note on those cells.
    if [[ "$COLUMN" == all ]]; then
      [[ "$column" == db ]] && continue
    else
      [[ "$COLUMN" == "$column" ]] || continue
    fi
    [[ -z "$ONLY" || "$ONLY" == "$id" ]] || continue

    if ! docker inspect "$service" >/dev/null 2>&1; then
      results+=("$id|SKIP|service $service absent from this topology")
      echo "[$id] SKIP: $service not present" | tee -a "$report"
      continue
    fi

    log "$id — $fault on $service"
    "$fault" "$service"
    sleep "$SETTLE_SECONDS"
    "$heal" "$service"
    sleep "$RECOVER_SECONDS"

    # Read this before the probe: the probe waits, and waiting gives the fleet
    # time to look healthy again, which would hide whether it needed to.
    local recovery
    recovery="$(fleet_recovery_report "$service")"
    [[ -n "$recovery" ]] && echo "[$id] fleet: $recovery" | tee -a "$report"

    # Reported, not gated. The crash column kills a tfhe-worker on purpose,
    # which is exactly how a dependence-chain lease lapses and another worker
    # steals the work -- so lock loss here is the injected fault behaving as
    # designed. It still belongs in the record: a cell that passed while work
    # was recomputed by a second worker proves something stronger than one that
    # passed with a single owner, and a reader cannot tell the two apart
    # afterwards without this line.
    local locks
    locks="$("$SCRIPT_DIR/consensus-validity.sh" locks --report-only 2>&1 | head -1)"
    [[ "$locks" == *"LOCK LOSS"* ]] && echo "[$id] $locks" | tee -a "$report"

    local output
    output="$(run_probe "$id" "" "$EXPECT_QUORUM")"
    echo "$output" >>"$report"
    if grep -qE "^\s+1 passing" <<<"$output"; then
      status=PASS
      detail=""
      if grep -q "SNS-DIGEST-DISAGREEMENT" <<<"$output"; then
        detail="operators disagreed on the SNS digest (recorded, not failed)"
      fi
      # A cell whose consensus assertion passes while a service it never
      # touched is still down is not a green cell: the fleet is degraded and
      # the next cell inherits it.
      if grep -q "STILL DOWN" <<<"$recovery"; then
        status=FAIL
        detail="$recovery"
      elif [[ -n "$recovery" ]]; then
        detail="$recovery"
      fi
    else
      status=FAIL
      detail="$(grep -m1 -E "AssertionError|Error:|timed out" <<<"$output" | sed 's/^\s*//' | cut -c1-160)"
    fi
    results+=("$id|$status|$detail")
    echo "[$id] $status ${detail:+- $detail}" | tee -a "$report"

    if [[ "$status" == FAIL && "$KEEP_GOING" != 1 ]]; then
      echo "stopping at first failure; pass --keep-going to run the whole column" | tee -a "$report"
      break
    fi
  done

  log "results"
  printf '%-32s %-6s %s\n' "CELL" "RESULT" "DETAIL" | tee -a "$report"
  local failures=0
  for entry in "${results[@]}"; do
    IFS='|' read -r id status detail <<<"$entry"
    printf '%-32s %-6s %s\n' "$id" "$status" "$detail" | tee -a "$report"
    [[ "$status" == FAIL ]] && failures=$((failures + 1))
  done
  echo | tee -a "$report"
  echo "report written to $report" | tee -a "$report"
  [[ "$failures" -eq 0 ]]
}

main
