#!/usr/bin/env bash
# RFC-020's crash-retry clause, driven host-side.
#
#   "a crash or retry re-executes the whole transaction batch; determinism makes
#    duplicate execution byte-identical and first-write-wins makes it harmless."
#
# The claim is about IDENTIFIED work. So this runner does not watch a
# whole-database pending count and fire at the first non-zero reading, which is
# what it used to do: that count includes fixture setup and unrelated traffic,
# so the interruption could land before the victim had touched anything the
# suite later compared -- and the run then established that a restarted worker
# can execute new work, which is a liveness check with a twenty-minute price.
#
# Instead the in-container half publishes its target transactions through a
# structured handshake, and this runner waits until the victim has DEMONSTRABLY
# acquired one of them:
#
#   * `dependence_chain.worker_id` non-null and `status = 'processing'` on the
#     chain that owns a target computation, while that computation is still
#     incomplete -- the victim holds the work and has not finished it;
#   * `worker_id` is a fresh UUID per worker PROCESS (`daemon_cli.rs`), so the
#     retry is provable: the same chain acquired again under a different
#     `worker_id`, which after a crash is the restarted process reaching it
#     through the acquisition query's `expired_lock` arm.
#
# Test-feature hooks hold acquired work before commit or after commit/before release.
# The expired-lease case freezes an acquired worker until its real lease expires.
# Usage: run-crash-retry-consensus.sh --boundary before-commit|after-commit|expired-lease
set -uo pipefail

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd)"
readonly ENV_DIR="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/env"
# shellcheck source=lib/service-control.sh
source "${SCRIPT_DIR}/lib/service-control.sh"
# shellcheck source=lib/case-result.sh
source "${SCRIPT_DIR}/lib/case-result.sh"
# shellcheck source=lib/suite-identity.sh
source "${SCRIPT_DIR}/lib/suite-identity.sh"
source "${SCRIPT_DIR}/lib/runner-assertions.sh"
source "${SCRIPT_DIR}/lib/suite-process.sh"
source "${SCRIPT_DIR}/lib/crash-controls.sh"
sc_init || exit 1
sp_init || exit 1

readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
readonly DB_CONTAINER="${DB_CONTAINER:-coprocessor-and-kms-db}"
readonly TEST_NETWORK="${TEST_NETWORK:-staging}"
readonly HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"
# How long to wait for the victim to be caught holding the target work. The
# suite mints for a while, and acquisition is a one-second polling loop.
readonly ACQUIRE_TIMEOUT="${CRASH_ACQUIRE_TIMEOUT:-420}"

VICTIM=1
HANDLES=2
BOUNDARY=before-commit
while [[ $# -gt 0 ]]; do
  case "$1" in
    --victim) VICTIM="${2:?--victim needs an operator index}"; shift 2 ;;
    --handles) HANDLES="${2:?--handles needs a count}"; shift 2 ;;
    --boundary) BOUNDARY="${2:?--boundary needs a value}"; shift 2 ;;
    *) echo "usage: run-crash-retry-consensus.sh [--victim <index>] [--handles <n>] [--boundary before-commit|after-commit|expired-lease]" >&2; exit 2 ;;
  esac
done
case "$BOUNDARY" in
  before-commit | after-commit | expired-lease) ;;
  *) echo "unknown boundary $BOUNDARY" >&2; exit 2 ;;
esac
case "$BOUNDARY" in
  before-commit) CASE_ID=CR-01-INTERRUPT-BEFORE-COMMIT ;;
  after-commit)  CASE_ID=CR-02-INTERRUPT-AFTER-COMMIT ;;
  expired-lease) CASE_ID=CR-03-EXPIRED-LEASE-RECLAIM ;;
esac

die() {
  echo "crash-retry: $*" >&2
  if [[ -n "${CR_TERMINAL_FILE:-}" ]] && ! grep -Fxq "$CASE_ID" "$CR_TERMINAL_FILE" 2>/dev/null; then
    CRASH_PENDING_FAILURE="$*"
  fi
  exit 1
}
CRASH_CONTROLS_ARMED=0
# Deleting the test control row releases a surviving worker on every exit path.
crash_record_abort() {
  local cleanup="$1"
  [[ -n "${CR_TERMINAL_FILE:-}" ]] || return 0
  grep -Fxq "$CASE_ID" "$CR_TERMINAL_FILE" 2>/dev/null && return 0
  cr_record "$CASE_ID" INVALID started_at="${CRASH_CASE_STARTED:-$(cr_now)}" cleanup="$cleanup" \
    detail="${CRASH_PENDING_FAILURE:-runner exited before its crash precondition and recovery could be established}"
}
cleanup_crash() {
  local status=$? cleanup_ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { crash_finalize 1 failed; exit 1; }
  if ! sp_cancel_all || ! sp_recover_suite_state; then retain_crash_cleanup; crash_finalize 1 failed; exit 1; fi
  [[ -z "${SUITE_PID:-}" ]] || { wait "$SUITE_PID" 2>/dev/null || true; SUITE_PID=""; }
  if [[ "$CRASH_CONTROLS_ARMED" == 1 ]]; then
    # Commit the release separately. Audit DDL can wait for a paused worker's
    # table lock, and must never roll this release back in the same transaction.
    if ! cc_disable_failpoints "$DB_CONTAINER" "$(victim_database)" >/dev/null; then
      retain_crash_cleanup; crash_finalize 1 failed; exit 1
    fi
  fi
  if ! sc_run_restores; then retain_crash_cleanup; crash_finalize 1 failed; exit 1; fi
  if [[ "$CRASH_CONTROLS_ARMED" == 1 ]]; then
    cc_drop_audit "$DB_CONTAINER" "$(victim_database)" >/dev/null || cleanup_ok=0
  fi
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  if [[ "$cleanup_ok" == 1 ]]; then
    crash_finalize "$status" ok || status=1
    if [[ "$status" == 0 || ! -f "$SP_RUNTIME_DIR/cleanup-failed" ]]; then sp_dispose || status=1; fi
  else
    retain_crash_cleanup; crash_finalize 1 failed; status=1
  fi
  exit "$status"
}

# A standalone run owns final publication; a delegated run publishes into its
# parent's staging directory. Neither can expose PASS before SQL cleanup ends.
crash_finalize() {
  local status="$1" cleanup="$2" state=keep detail="" published
  if [[ -z "${CRASH_STAGED_RESULTS:-}" ]]; then
    [[ "$status" == 0 ]] || crash_record_abort "$cleanup"
    return "$status"
  fi
  if [[ "$status" != 0 || "$cleanup" == failed ]]; then
    state=FAIL; detail="crash runner exited $status; final cleanup was $cleanup"
    crash_record_abort "$cleanup" || status=1
  elif ! bun "$SCRIPT_DIR/finalize-case-results.ts" --require-case "$CRASH_STAGED_RESULTS" "$CASE_ID" "$CR_RUN_ID"; then
    state=FAIL; detail="crash runner exited successfully without a successful staged verdict for $CASE_ID"; status=1
  fi
  published="$(timeout --kill-after=2s 20s bun "$SCRIPT_DIR/finalize-case-results.ts" "$CRASH_STAGED_RESULTS" "$CRASH_PUBLISH_RESULTS" "$state" "$cleanup" "$detail")" || {
    touch "$SP_RUNTIME_DIR/cleanup-failed"
    CONSENSUS_RESULTS_DIR="$CRASH_PUBLISH_RESULTS" cr_record "$CASE_ID" FAIL cleanup="$cleanup" detail="could not finalize crash evidence" || true
    return 1
  }
  if [[ "$state" != keep ]] && ! grep -Fxq "$CR_RUN_ID:$CASE_ID" <<<"$published"; then
    CONSENSUS_RESULTS_DIR="$CRASH_PUBLISH_RESULTS" cr_record "$CASE_ID" FAIL cleanup="$cleanup" detail="$detail" || status=1
  fi
  return "$status"
}

retain_crash_cleanup() {
  touch "$SP_RUNTIME_DIR/cleanup-failed"
  mkdir -p "$(dirname "$SP_CONTAMINATION")"
  printf 'phase_registry=%s\nrestore_log=%s\ncrash_database=%s\n' \
    "$SP_PHASE_REGISTRY" "$SC_RESTORE_LOG" "$(victim_database)" > "$SP_CONTAMINATION"
  echo "crash-retry: SQL/service cleanup failed; retaining recovery ownership" >&2
}
trap cleanup_crash EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

log() { printf '\n=== %s\n' "$*"; }

operator_count() {
  local n=0 path
  for path in "$ENV_DIR"/coprocessor.env "$ENV_DIR"/coprocessor.[0-9]*.env; do
    [[ -f "$path" ]] && n=$((n + 1))
  done
  echo "$n"
}

env_value() { sed -n "s/^$1=//p" "$2" | tail -1; }

victim_container() {
  [[ "$VICTIM" == 0 ]] && echo "coprocessor-tfhe-worker" || echo "coprocessor${VICTIM}-tfhe-worker"
}

victim_database() {
  [[ "$VICTIM" == 0 ]] && echo "coprocessor" || echo "coprocessor_${VICTIM}"
}

psql_victim() {
  docker exec "$DB_CONTAINER" psql -U postgres -v ON_ERROR_STOP=1 -d "$(victim_database)" -tAc "$1" 2>&1
}

# Checked up front, because a missing table looks exactly like "no work in
# flight": the query fails, the poll reads nothing forever, and the run would
# report that it never saw work rather than that it could not look.
require_observability() {
  local probe
  probe="$(psql_victim "SELECT COUNT(*) FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
      WHERE c.relname IN ('computations','dependence_chain') AND n.nspname = 'public'" | tr -d ' ')"
  [[ "$probe" == 2 ]] || die "database $(victim_database) is missing public.computations or public.dependence_chain, so acquisition cannot be observed and the interruption cannot be aimed (got: ${probe:-<query failed>})"
}

# Read the handshake the suite publishes inside the test container.
read_target() {
  docker exec "$TEST_CONTAINER" cat "${HANDSHAKE_DIR}/crash-target.json" 2>/dev/null
}

# Write our acknowledgement back, so the suite can state the evidence too and
# refuse to assert recovery of work that was never interrupted.
publish_ack() {
  local applied="$1" detail="$2"
  local payload
  payload="$(printf '{"name":"crash-fault","ready":true,"payload":{"applied":%s,"detail":%s,"processBefore":%s,"processAfter":%s,"faultObservedAt":%s,"recoveryObservedAt":%s,"dependenceChainId":%s,"workerIdBefore":%s,"workerIdAfter":%s}}' \
    "$applied" \
    "$(json_string "$detail")" \
    "$(json_string "${IDENTITY_BEFORE:-}")" \
    "$(json_string "${IDENTITY_AFTER:-}")" \
    "$(json_string "${FAULT_OBSERVED_AT:-}")" \
    "$(json_string "${RECOVERY_OBSERVED_AT:-}")" \
    "$(json_string "${TARGET_CHAIN:-}")" \
    "$(json_string "${TARGET_WORKER:-}")" \
    "$(json_string "${TARGET_WORKER_AFTER:-}")")"
  docker exec -i "$TEST_CONTAINER" sh -c "mkdir -p '$HANDSHAKE_DIR' && cat > '$HANDSHAKE_DIR/crash-fault.json'" <<<"$payload"
}

json_string() {
  local value="${1:-}"
  value="${value//\\/\\\\}"
  value="${value//\"/\\\"}"
  value="${value//$'\n'/ }"
  printf '"%s"' "$value"
}

# The SQL that turns "the victim is busy" into "the victim holds THIS work".
#
# `transaction_id` in `computations` is the producing transaction hash, so the
# published hashes address the rows directly.
target_hashes_sql() {
  local hashes="$1" out="" hash
  for hash in $hashes; do
    out+="${out:+,}decode('${hash#0x}','hex')"
  done
  printf 'ARRAY[%s]::bytea[]' "$out"
}

# One line: chain id, worker id, how many target computations that chain still
# owes, and how many it has completed.
observe_acquisition() {
  local array="$1"
  psql_victim "
    SELECT encode(dc.dependence_chain_id,'hex'),
           COALESCE(dc.worker_id::text,''),
           dc.status,
           COUNT(*) FILTER (WHERE NOT c.is_completed AND NOT c.is_error),
           COUNT(*) FILTER (WHERE c.is_completed)
      FROM computations c
      JOIN dependence_chain dc ON dc.dependence_chain_id = c.dependence_chain_id
     WHERE c.transaction_id = ANY($array)
     GROUP BY 1,2,3
     ORDER BY 4 DESC"
}

# Preserve committed acquisition transitions so a fast release cannot hide
# the replacement worker's UUID from polling. This audit exists only in this
# test database and does not modify the worker or the acquisition transaction.
prepare_claim_audit() {
  psql_victim "$(cat <<'SQL'
CREATE TABLE IF NOT EXISTS public.consensus_test_claims (
  dependence_chain_id bytea NOT NULL, worker_id uuid NOT NULL, claimed_at timestamptz NOT NULL DEFAULT clock_timestamp()
);
TRUNCATE public.consensus_test_claims;
-- Serial scheduling can commit only part of a chain per batch. Hold the
-- after-commit hook only once the selected producer chain is fully durable,
-- matching the case's before-lock-release assertion at every batch size.
CREATE OR REPLACE FUNCTION public.consensus_test_complete_boundary() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  IF EXISTS (SELECT 1 FROM computations WHERE dependence_chain_id = NEW.dependence_chain_id AND NOT is_completed) THEN
    RETURN NULL;
  END IF;
  RETURN NEW;
END;
$$;
DROP TRIGGER IF EXISTS consensus_test_complete_boundary ON public.consensus_test_failpoints;
CREATE TRIGGER consensus_test_complete_boundary BEFORE UPDATE ON public.consensus_test_failpoints
FOR EACH ROW WHEN (NEW.stage = 'after-commit' AND NEW.reached)
EXECUTE FUNCTION public.consensus_test_complete_boundary();
CREATE OR REPLACE FUNCTION public.consensus_test_claim_audit() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  INSERT INTO public.consensus_test_claims(dependence_chain_id, worker_id) VALUES (NEW.dependence_chain_id, NEW.worker_id);
  RETURN NEW;
END;
$$;
DROP TRIGGER IF EXISTS consensus_test_claim_audit ON dependence_chain;
CREATE TRIGGER consensus_test_claim_audit AFTER UPDATE OF worker_id ON dependence_chain
FOR EACH ROW WHEN (NEW.worker_id IS NOT NULL AND NEW.worker_id IS DISTINCT FROM OLD.worker_id)
EXECUTE FUNCTION public.consensus_test_claim_audit();
SQL
)"
}

# Has the chain that owned the target work been re-acquired by a DIFFERENT
# worker process?
chain_owner() {
  local chain="$1"
  psql_victim "SELECT COALESCE(worker_id::text,''), status, (lock_expires_at < NOW()) FROM dependence_chain WHERE dependence_chain_id = decode('$chain','hex')"
}

target_progress() {
  local array="$1"
  psql_victim "SELECT COUNT(*) FILTER (WHERE NOT is_completed AND NOT is_error), COUNT(*) FILTER (WHERE is_completed), COUNT(*) FILTER (WHERE is_error) FROM computations WHERE transaction_id = ANY($array)"
}

# Hash each persisted payload separately and retain its key, so recovery cannot
# silently replace, remove, or duplicate the producer's committed ciphertexts.
committed_ciphertexts() {
  psql_victim "SELECT encode(ct.handle,'hex'), ct.ciphertext_version, md5(ct.ciphertext)
    FROM ciphertexts ct WHERE EXISTS (SELECT 1 FROM computations c
      WHERE c.output_handle=ct.handle AND c.dependence_chain_id=decode('$1','hex'))
    ORDER BY ct.handle, ct.ciphertext_version"
}

# What one chain still owes, and what it has finished. The post-commit boundary
# is a statement about a single chain, and while its release is blocked the
# worker cannot advance any OTHER chain -- so asking about the whole published
# set there waits for something the hold itself prevents.
chain_progress() {
  local chain="$1"
  psql_victim "
    SELECT COUNT(*) FILTER (WHERE NOT is_completed AND NOT is_error),
           COUNT(*) FILTER (WHERE is_completed)
      FROM computations
     WHERE dependence_chain_id = decode('$chain','hex')"
}

# Is the session that holds the boundary open still there?


# Chains this operator still holds a live lease on, most recently acquired
# first. Asked AFTER the owner is frozen: a frozen process cannot release, so
# whatever it holds then it keeps, which is what makes the expired-lease
# boundary deterministic instead of a race against the release.
held_chains() {
  psql_victim "
    SELECT encode(dependence_chain_id,'hex'),
           COALESCE(worker_id::text,''),
           status,
           COALESCE(round(EXTRACT(epoch FROM (lock_expires_at - NOW()))::numeric, 1)::text, 'null')
      FROM dependence_chain
     WHERE worker_id IS NOT NULL
       AND lock_expires_at IS NOT NULL
     ORDER BY lock_acquired_at DESC"
}

# A full description of one chain, for a message that says what was seen rather
# than only that something was not seen.
chain_state() {
  local chain="$1"
  psql_victim "
    SELECT status,
           COALESCE(worker_id::text,'(none)'),
           COALESCE(lock_expires_at::text,'(null)'),
           NOW()::text
      FROM dependence_chain
     WHERE dependence_chain_id = decode('$chain','hex')"
}

# Hold the post-commit boundary open with the system's own row lock.
#
# The worker commits its computations and then, in a SEPARATE transaction,
# releases the chain and discharges its dependents. The state CR-02 names lives
# between those two commits, which is too brief for polling to reliably
# observe from outside the worker.
#
# So it is held rather than raced: this takes the `dependence_chain` row lock
# the release itself must take, which any concurrent client may hold, and the
# release blocks on it. Nothing about the state is fabricated -- the worker
# commits its work exactly as it would have, and the chain sits owned,
# `processing`, with every computation committed, for as long as the lock is
# held. The lease TTL is 30s and the hold is seconds, so the lease cannot lapse
# underneath it; if it somehow did, the chain would be reclaimed and the case
# reports INVALID rather than a false pass.
main() {
  # Early aborts still need a scenario label for their INVALID record, and the
  # topology is only verified further down. Read the label from the stack that
  # is actually up instead of assuming three-of-three; the observation below
  # re-verifies it, and an unreadable state leaves cr_init to refuse.
  if [[ -z "${CONSENSUS_SCENARIO:-}" ]]; then
    CONSENSUS_SCENARIO="$(jq -r '(.scenarioSourcePath // .scenario.sourcePath // "") | split("/") | last | sub("\\.ya?ml$"; "")' \
      "${FHEVM_STATE_DIR:-${REPO_ROOT:-.}/.fhevm}/state/state.json" 2>/dev/null || true)"
  fi
  cr_init "${CONSENSUS_RUN_ID:-}" || die "cannot identify the source revision"
  CRASH_PUBLISH_RESULTS="$(dirname "$(cr_results_file)")"
  CRASH_STAGED_RESULTS="$(mktemp -d "$SP_RUNTIME_DIR/crash-results.XXXXXX")" || die "cannot stage crash evidence"
  export CONSENSUS_RESULTS_DIR="$CRASH_STAGED_RESULTS"
  CR_TERMINAL_FILE="$SP_RUNTIME_DIR/recorded-cases"
  touch "$CR_TERMINAL_FILE"
  CRASH_CASE_STARTED="$(cr_now)"
  command -v docker >/dev/null || die "docker is required"
  [[ -d "$ENV_DIR" ]] || die "no generated stack at $ENV_DIR; bring one up first"
  local count; count="$(operator_count)"
  [[ "$count" -ge 2 ]] || die "this needs at least two operators: one to interrupt and one to compare against"
  [[ "$VICTIM" -lt "$count" ]] || die "victim $VICTIM is outside a $count-operator topology"

  # The image bakes the e2e suite in, so a commit since the last build does not
  # reach the container. A result labelled with a revision whose code never ran
  # is worse than no result.
  suite_identity_assert "$TEST_CONTAINER" || die "the test container is not running this working tree's e2e suite"
  # A stack that is not this case's declared scenario declines it rather than
  # producing an outcome the inventory cannot accept.
  # Required quorum evidence must be configured before any worker or SQL fault.
  local coprocessor_env="$ENV_DIR/coprocessor.env"
  local gateway_url gateway_config ciphertext_commits
  gateway_url="$(env_value GATEWAY_URL "$coprocessor_env")"
  gateway_config="$(env_value GATEWAY_CONFIG_ADDRESS "$coprocessor_env")"
  ciphertext_commits="$(env_value CIPHERTEXT_COMMITS_ADDRESS "$coprocessor_env")"
  [[ -n "$gateway_url" ]] || die "required quorum configuration GATEWAY_URL is missing"
  [[ -n "$gateway_config" ]] || die "required quorum configuration GATEWAY_CONFIG_ADDRESS is missing"
  [[ -n "$ciphertext_commits" ]] || die "required quorum configuration CIPHERTEXT_COMMITS_ADDRESS is missing"
  local observed_topology
  observed_topology="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" "$count")" || die "cannot verify active topology"
  eval "$observed_topology"
  export CONSENSUS_SCENARIO CONSENSUS_OPERATORS CONSENSUS_THRESHOLD
  CR_OPERATORS="$CONSENSUS_OPERATORS"; CR_THRESHOLD="$CONSENSUS_THRESHOLD"; CR_SCENARIO="$CONSENSUS_SCENARIO"
  cr_skip_wrong_scenario "$CASE_ID" && return 0
  local case_started; case_started="$(cr_now)"
  sp_case_start "$CASE_ID" || die "cannot establish case deadline"
  export CONSENSUS_RUN_STARTED_AT="$case_started"

  local container; container="$(victim_container)"
  docker inspect "$container" >/dev/null 2>&1 || die "$container is not present"
  [[ "$(sc_state "$container")" == running ]] ||
    die "$container is $(sc_state "$container"); a crash case needs a live worker to interrupt"
  if [[ "$(sc_restart_budget "$container")" == exhausted ]]; then
    sc_register_restore "$container" start || die "cannot record restart restoration"
    sc_reset_restart_budget "$container" || die "could not renew victim restart budget before minting"
  fi
  require_observability
  CRASH_CONTROLS_ARMED=1
  psql_victim "CREATE TABLE IF NOT EXISTS public.consensus_test_failpoints(stage text PRIMARY KEY, dependence_chain_id bytea NOT NULL, reached boolean NOT NULL DEFAULT false, observed_at timestamptz); TRUNCATE public.consensus_test_failpoints" >/dev/null || die "could not prepare test hooks"
  prepare_claim_audit >/dev/null || die "could not prepare committed acquisition audit"
  sc_pause "$container" || die "could not pause victim before minting"

  # Clear stale handshake files, or a previous run's target would be read as
  # this one's and the interruption would be aimed at work that no longer
  # exists.
  docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/crash-*.json" >/dev/null 2>&1 || true

  IDENTITY_BEFORE="$(sc_identity "$container")"
  local restarts_before; restarts_before="$(sc_restart_count "$container")"

  log "starting the workload; victim is $container (operator $VICTIM), boundary $BOUNDARY"
  local suite_log; suite_log="$(mktemp)"
  sp_exec \
    -e RUN_CRASH_RETRY_CONSENSUS=1 \
    -e "COPROCESSOR_COUNT=$count" \
    -e "CRASH_VICTIM_OPERATOR=$VICTIM" \
    -e "CRASH_RETRY_HANDLES=$HANDLES" \
    -e "CONSENSUS_THRESHOLD=$CONSENSUS_THRESHOLD" \
    -e "CRASH_BOUNDARY=$BOUNDARY" \
    -e "CONSENSUS_ASSERTION_CASE_ID=$([[ "$BOUNDARY" == before-commit ]] && echo FM-TFHE-CRASH)" \
    -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" \
    `# The victim is killed on purpose here: it has to be replaced, reclaim its` \
    `# expired lease and redo the work before it can submit, which the` \
    `# watchdog's ordinary stall budget does not accommodate.` \
    -e "CONSENSUS_WATCHDOG_STALL_MS=$((12 * 60 * 1000))" \
    -e "GATEWAY_RPC_URL=$gateway_url" \
    -e "GATEWAY_CONFIG_ADDRESS=$gateway_config" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$ciphertext_commits" \
    -e npm_config_update_notifier=false \
    "$TEST_CONTAINER" \
    npx hardhat test test/consensus/crashRetryConsensus.ts --network "$TEST_NETWORK" >"$suite_log" 2>&1 &
  local suite_pid=$!
  SUITE_PID="$suite_pid"

  # Any exit from here leaves the suite waiting on an acknowledgement, so say
  # so rather than letting it time out with no explanation.
  finish_unapplied() {
    publish_ack false "$1"
    sc_run_restores || true
  }

  log "waiting for the suite to publish its target transactions"
  local target_json="" i
  for ((i = 0; i < 300; i++)); do
    kill -0 "$suite_pid" 2>/dev/null || break
    target_json="$(read_target)"
    [[ -n "$target_json" ]] && break
    sleep 2
  done
  if [[ -z "$target_json" ]]; then
    finish_unapplied "the suite never published a crash target"
    wait "$suite_pid"; cat "$suite_log"; rm -f "$suite_log"
    cr_record "$CASE_ID" INVALID detail="the suite never published its target transactions, so no fault could be aimed" cleanup=ok started_at="$case_started"
    die "no crash target was published; nothing could be aimed at"
  fi
  local hashes
  hashes="$(grep -oE '0x[0-9a-f]{64}' <<<"$target_json" | sort -u | tr '\n' ' ')"
  [[ -n "$hashes" ]] || { finish_unapplied "the published target carried no transaction hashes"; die "the published crash target carried no transaction hashes"; }
  local array; array="$(target_hashes_sql "$hashes")"
  echo "  target transactions: $(wc -w <<<"$hashes") published"

  # The victim was paused before minting, so none of the selected work can race the hook.
  local deadline=$((SECONDS + ACQUIRE_TIMEOUT)) observation="" chain worker status pending completed
  local dependent_filter="" committed_before=""
  if [[ "$BOUNDARY" == after-commit ]]; then
    dependent_filter="AND EXISTS (SELECT 1 FROM dependence_chain parent JOIN dependence_chain child ON child.dependence_chain_id = ANY(parent.dependents) WHERE parent.dependence_chain_id = computations.dependence_chain_id AND child.dependency_count > 0)"
  fi
  TARGET_CHAIN=""
  while ((SECONDS < deadline)); do
    TARGET_CHAIN="$(psql_victim "SELECT encode(dependence_chain_id,'hex') FROM computations WHERE transaction_id = ANY($array) AND dependence_chain_id IS NOT NULL AND NOT is_completed $dependent_filter ORDER BY block_number, transaction_id LIMIT 1")"
    [[ "$TARGET_CHAIN" =~ ^[0-9a-f]{64}$ ]] && break
    sleep 1
  done
  [[ "$TARGET_CHAIN" =~ ^[0-9a-f]{64}$ ]] || { finish_unapplied "no pending target chain"; die "never observed operator $VICTIM holding a pending target chain within the acquisition window"; }
  local hook_stage="$BOUNDARY"
  [[ "$BOUNDARY" == expired-lease ]] && hook_stage=before-commit
  psql_victim "INSERT INTO public.consensus_test_failpoints(stage,dependence_chain_id) VALUES ('$hook_stage',decode('$TARGET_CHAIN','hex'))" >/dev/null || die "could not arm test hook"
  sc_resume "$container" || die "could not resume victim into the test hook"
  while ((SECONDS < deadline)); do
    [[ "$(psql_victim "SELECT reached FROM public.consensus_test_failpoints WHERE stage='$hook_stage'")" == t ]] && break
    sleep 0.2
  done
  [[ "$(psql_victim "SELECT reached FROM public.consensus_test_failpoints WHERE stage='$hook_stage'")" == t ]] || {
    finish_unapplied "test hook was not reached; build the worker with test-failpoints"
    die "deterministic boundary was not observed"
  }
  observation="$(observe_acquisition "$array")"
  TARGET_WORKER=""
  while IFS='|' read -r chain worker status pending completed; do
    [[ "$chain" == "$TARGET_CHAIN" ]] || continue
    [[ "$status" == processing && -n "$worker" ]] || die "hook did not hold an acquired target"
    if [[ "$BOUNDARY" == after-commit ]]; then
      [[ "$pending" == 0 && "$completed" -gt 0 ]] || die "post-commit hook lacks durable completed work"
    else
      [[ "$pending" -gt 0 ]] || die "pre-commit hook has no incomplete target work"
    fi
    TARGET_WORKER="$worker"
  done <<<"$observation"
  [[ -n "$TARGET_WORKER" ]] || die "missing target owner at deterministic boundary"
  if [[ "$BOUNDARY" == after-commit ]]; then
    local gated
    gated="$(psql_victim "SELECT count(*) FROM dependence_chain parent JOIN dependence_chain child ON child.dependence_chain_id = ANY(parent.dependents) WHERE parent.dependence_chain_id=decode('$TARGET_CHAIN','hex') AND child.dependency_count > 0")"
    [[ "$gated" -gt 0 ]] || die "post-commit boundary has no gated dependent"
    committed_before="$(committed_ciphertexts "$TARGET_CHAIN")"
    [[ -n "$committed_before" ]] || die "post-commit boundary has no persisted ciphertexts"
    echo "  durable producer still gates $gated dependent(s); captured committed ciphertext fingerprints"
  fi
  FAULT_OBSERVED_AT="$(cr_now)"
  echo "  operator $VICTIM holds chain $TARGET_CHAIN as worker $TARGET_WORKER (boundary $BOUNDARY)"

  # --- apply the fault ----------------------------------------------------
  local fault_detail=""
  case "$BOUNDARY" in
    before-commit | after-commit)
      if ! sc_kill "$container" KILL >/dev/null; then
        finish_unapplied "could not signal $container from the host"
        die "could not signal $container"
      fi
      fault_detail="SIGKILL to $container's main process from the host ($SC_KILL_PATH)"
      # Let the blocked release go with the process that owned it: the chain is
      # left committed and owned by a worker that no longer exists, which is
      # exactly what a crash in that window leaves behind.

      ;;
    expired-lease)
      # Freeze FIRST, then choose the chain from what the frozen worker
      # actually still holds.
      #
      # A chain selected before the freeze may already be released when the
      # pause lands, leaving no lease to expire. Selecting after the freeze
      # guarantees the observed owner cannot release it during the check.
      sc_pause "$container" || {
        finish_unapplied "could not stall $container"
        die "could not stall $container"
      }
      fault_detail="SIGSTOP to $container's main process while it held chain"
      local frozen_chain="" frozen_worker="" frozen_status="" frozen_ttl=""
      local h_chain h_worker h_status h_ttl
      while IFS='|' read -r h_chain h_worker h_status h_ttl; do
        [[ -n "$h_chain" ]] || continue
        # Prefer a chain carrying this run's own work; any live lease this
        # worker holds still exercises the reclaim path, but the case's claim
        # is about the target work, so that is what it aims at.
        if [[ "$h_chain" == "$TARGET_CHAIN" ]]; then
          frozen_chain="$h_chain"; frozen_worker="$h_worker"; frozen_status="$h_status"; frozen_ttl="$h_ttl"
          break
        fi
        [[ -z "$frozen_chain" ]] && {
          frozen_chain="$h_chain"; frozen_worker="$h_worker"; frozen_status="$h_status"; frozen_ttl="$h_ttl"
        }
      done <<<"$(held_chains)"

      if [[ -z "$frozen_chain" ]]; then
        local seen; seen="$(chain_state "$TARGET_CHAIN")"
        sc_resume "$container" || true
        finish_unapplied "operator $VICTIM held no leased chain once it was frozen"
        cr_record "$CASE_ID" INVALID cleanup=ok started_at="$case_started" \
          detail="operator $VICTIM held no chain with a live lease under the freeze; chain $TARGET_CHAIN was ${seen:-unreadable} (status|owner|lock_expires_at|now). Nothing could expire, so the reclaim path was not exercised"
        die "no lease was held under the freeze; the reclaim path could not be exercised"
      fi
      TARGET_CHAIN="$frozen_chain"
      TARGET_WORKER="$frozen_worker"
      fault_detail="SIGSTOP to $container's main process while it held chain $TARGET_CHAIN"
      echo "  under the freeze, operator $VICTIM holds $TARGET_CHAIN ($frozen_status, ${frozen_ttl}s of lease left)"

      log "holding the stall until the lease on $TARGET_CHAIN lapses"
      local lapsed=0 lease_deadline=$((SECONDS + 180)) expired
      while ((SECONDS < lease_deadline)); do
        IFS='|' read -r worker status expired <<<"$(chain_owner "$TARGET_CHAIN")"
        if [[ "$expired" == t ]]; then lapsed=1; break; fi
        sleep 2
      done
      if [[ "$lapsed" != 1 ]]; then
        local seen; seen="$(chain_state "$TARGET_CHAIN")"
        sc_resume "$container" || true
        finish_unapplied "the lease on $TARGET_CHAIN never lapsed while its owner was frozen"
        cr_record "$CASE_ID" INVALID cleanup=ok started_at="$case_started" \
          detail="the lease on $TARGET_CHAIN did not lapse within 180s of freezing its owner; observed status|owner|lock_expires_at|now = ${seen:-unreadable}"
        die "the lease never lapsed, so the reclaim path was not exercised"
      fi
      echo "  lease on $TARGET_CHAIN has expired while still owned by $TARGET_WORKER"
      # Kill the frozen worker rather than resuming it, because resuming it
      # establishes nothing: one worker serves each operator's queue, so the
      # resumed process still matched its own `worker_id`, extended its lease,
      # and finished its own work -- which the case then recorded as a
      # "reclaim". Killing it makes the claim real. The supervisor starts a NEW
      # process with a different worker_id, and that process can only take this
      # chain through the expired-lock acquisition arm, which is the path the
      # case is about. The signal is delivered when the frozen process is
      # resumed, which `sc_kill` does through its restore.
      if ! sc_kill "$container" KILL 1 >/dev/null; then
        sc_resume "$container" || true
        finish_unapplied "could not signal the frozen $container"
        cr_record "$CASE_ID" INVALID cleanup=ok started_at="$case_started" \
          detail="the frozen worker could not be signalled, so no different claimant could take the expired lease"
        die "could not signal the frozen worker"
      fi
      fault_detail="$fault_detail, then SIGKILL once the lease had lapsed so a DIFFERENT process must reclaim it"
      sc_resume "$container" >/dev/null 2>&1 || true
      ;;
  esac

  # --- observe the recovery ----------------------------------------------
  local identity_after=""
    log "waiting for the supervisor to replace the worker process"
    if ! identity_after="$(sc_wait_replaced "$container" "$IDENTITY_BEFORE" 120)"; then
      finish_unapplied "the worker process was signalled but never replaced; automatic recovery did not happen"
      wait "$suite_pid"; cat "$suite_log"; rm -f "$suite_log"
      cr_record "$CASE_ID" FAIL \
        detail="the interrupted worker was never replaced by its supervisor; this run cannot report automatic recovery" \
        fault_observed_at="$FAULT_OBSERVED_AT" process_before="$container=$IDENTITY_BEFORE" \
        cleanup=ok started_at="$case_started" workload="$TARGET_CHAIN"
      die "no automatic recovery"
    fi
    IDENTITY_AFTER="$identity_after"
    RESTARTS_AFTER="$(sc_restart_count "$container")"
    RESTARTS_BEFORE="$restarts_before"
    [[ "${RESTARTS_AFTER:-0}" -gt "${restarts_before:-0}" ]] ||
      echo "  WARNING the restart counter did not move ($restarts_before -> $RESTARTS_AFTER); the process identity did change" >&2
    echo "  replaced: $IDENTITY_BEFORE -> $IDENTITY_AFTER (restarts $restarts_before -> ${RESTARTS_AFTER:-?})"

  log "waiting for the same work to be reclaimed"
  local reclaimed=0 reclaim_deadline=$((SECONDS + 300)) new_worker new_status
  while ((SECONDS < reclaim_deadline)); do
    new_worker="$(psql_victim "SELECT worker_id::text FROM public.consensus_test_claims WHERE dependence_chain_id = decode('$TARGET_CHAIN','hex') AND worker_id <> '$TARGET_WORKER'::uuid AND claimed_at >= '$FAULT_OBSERVED_AT'::timestamptz ORDER BY claimed_at LIMIT 1")"
    if [[ -n "$new_worker" && "$new_worker" != "$TARGET_WORKER" ]]; then
      reclaimed=1
      TARGET_WORKER_AFTER="$new_worker"
      echo "  chain $TARGET_CHAIN reclaimed by a different worker process ($TARGET_WORKER -> $new_worker)"
      break
    fi
    sleep 2
  done
  RECOVERY_OBSERVED_AT="$(cr_now)"
  if [[ "$reclaimed" != 1 ]]; then
    finish_unapplied "chain $TARGET_CHAIN was never reclaimed or retired after the fault"
    wait "$suite_pid"; cat "$suite_log"; rm -f "$suite_log"
    cr_record "$CASE_ID" FAIL \
      detail="chain $TARGET_CHAIN was neither reclaimed by another worker process nor retired within 300s of the fault" \
      fault_observed_at="$FAULT_OBSERVED_AT" workload="$TARGET_CHAIN" cleanup=ok started_at="$case_started"
    die "the interrupted work was not retried"
  fi

  publish_ack true "$fault_detail; chain $TARGET_CHAIN reclaimed after the fault"

  log "waiting for the suite's assertions"
  wait "$suite_pid"; local suite_status=$?
  cat "$suite_log"
  local suite_out; suite_out="$(cat "$suite_log")"
  rm -f "$suite_log"

  # Lock loss is expected for the lease case and is the evidence; elsewhere it
  # is recorded and attributed rather than suppressed.
  # `--expect-loss` asks whether a LIVE worker reported losing a lock it was
  # extending. Nothing extends a dead worker's locks, so this construction --
  # which kills the owner and lets a new process take the expired lease --
  # cannot produce that line, and demanding it would fail a case whose evidence
  # is the ownership change on the chain itself. Recorded, not gated.
  local lock_args=(locks --since "$case_started" --operators "$count" --report-only)
  local lock_status=0 lock_out
  lock_out="$("$SCRIPT_DIR/consensus-validity.sh" "${lock_args[@]}" 2>&1)" || lock_status=$?
  echo "$lock_out"

  local cleanup_state=ok cleanup_detail=""
  if ! sc_run_restores; then
    cleanup_state=failed
    cleanup_detail="one or more service restores failed; the stack is contaminated"
  fi
  if ! "$SCRIPT_DIR/consensus-validity.sh" exclusivity --operators "$count" >/dev/null 2>&1; then
    cleanup_state=failed
    cleanup_detail="${cleanup_detail:+$cleanup_detail; }a queue is not served by exactly one worker after the case"
  fi

  local -a record=(
    started_at="$case_started"
    workload="chain:$TARGET_CHAIN"
    fault_observed_at="$FAULT_OBSERVED_AT"
    recovery_observed_at="$RECOVERY_OBSERVED_AT"
    process_before="$container=$IDENTITY_BEFORE"
    process_after="$container=$IDENTITY_AFTER"
    cleanup="$cleanup_state"
  )
  [[ -n "$cleanup_detail" ]] && record+=(cleanup_detail="$cleanup_detail")
  local hash
  for hash in $hashes; do record+=(workload="$hash"); done

  if [[ "$suite_status" -ne 0 ]]; then
    cr_record "$CASE_ID" FAIL "${record[@]}" \
      assert="recovery=fail:$(cr_failure_reason "$suite_out")" \
      detail="the crash-retry assertions failed: $(cr_failure_reason "$suite_out")"
    die "the crash-retry assertions failed"
  fi
  if [[ "$lock_status" -ne 0 ]]; then
    cr_record "$CASE_ID" FAIL "${record[@]}" \
      assert="lock-evidence=fail" \
      detail="the lock-extension gate failed: $(head -1 <<<"$lock_out")"
    die "the lock-extension gate failed"
  fi
  if [[ "$cleanup_state" != ok ]]; then
    cr_record "$CASE_ID" FAIL "${record[@]}" detail="$cleanup_detail"
    die "$cleanup_detail"
  fi
  if ! grep -qF "[crash-retry] CASE COMPLETE" <<<"$suite_out"; then
    cr_record "$CASE_ID" INVALID "${record[@]}" \
      detail="the suite exited 0 without printing its completion marker, so its assertions did not all run"
    die "the suite did not run its assertions to completion"
  fi

  if [[ "$BOUNDARY" == after-commit ]]; then
    local committed_after
    committed_after="$(committed_ciphertexts "$TARGET_CHAIN")"
    if [[ "$committed_after" != "$committed_before" ]]; then
      cr_record "$CASE_ID" FAIL "${record[@]}" detail="recovery changed the producer's committed ciphertexts"
      die "committed ciphertexts changed after recovery"
    fi
    record+=(assert="committed-bytes-preserved=pass")
  fi

  cr_record_checked_pass "$CASE_ID" "${record[@]}" \
    assert="fault=pass:$fault_detail" \
    assert="retry-observed=pass:chain $TARGET_CHAIN reclaimed after the fault" \
    assert="byte-agreement=pass" \
    assert="atomic-completion=pass" \
    assert="no-stranded-work=pass"
  log "$CASE_ID: PASS (results in $(cr_results_file))"

  # REG-03 is the stack-level half of the daemon-exit fix: REG-02 establishes
  # that the process exits non-zero on a fatal failure, and this establishes
  # that a supervisor then replaces it and the selected pending work recovers.
  # That is the same evidence this run just produced, recorded from it rather
  # than by killing a second worker to say the same thing again. Only the
  # before-commit boundary supplies this shared assertion without another run.
  if [[ "$BOUNDARY" == before-commit ]]; then
    cr_record_checked_pass FM-TFHE-CRASH "${record[@]}" assert="fault=pass:$fault_detail" \
      assert="acquired-work-recovers=pass:shared CR-01 interruption evidence"
    cr_record_checked_pass REG-03-SUPERVISED-DAEMON-RECOVERY "${record[@]}" assert="fault=pass:$fault_detail" \
      assert="supervisor-restart-observed=pass:$IDENTITY_BEFORE -> $IDENTITY_AFTER (restart count ${RESTARTS_BEFORE:-?} -> ${RESTARTS_AFTER:-?}), not a manual start" \
      assert="pending-work-recovered=pass:chain $TARGET_CHAIN was reclaimed and completed"
    log "REG-03-SUPERVISED-DAEMON-RECOVERY: PASS (same evidence)"
  fi
  [[ "${CR_RECORD_FAILURES:-0}" == 0 ]] || die "case result recording failed"
}

main
