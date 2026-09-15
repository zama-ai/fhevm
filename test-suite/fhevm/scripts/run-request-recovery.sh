#!/usr/bin/env bash
# Keep one SDK request alive across a relayer or KMS-worker process crash.
set -uo pipefail
readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/suite-identity.sh"
source "${SCRIPT_DIR}/lib/runner-assertions.sh"
source "${SCRIPT_DIR}/lib/suite-process.sh"
source "${SCRIPT_DIR}/lib/result-staging.sh"
sc_init || exit 1
sp_init || exit 1
readonly TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
readonly DB_CONTAINER="${DB_CONTAINER:-coprocessor-and-kms-db}"
readonly RELAYER_DB_CONTAINER="${RELAYER_DB_CONTAINER:-fhevm-relayer-db}"
readonly HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"
readonly ENV_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/env"
CASE_ID="${1:?supply FM-RELAYER-CRASH or FM-KMS-CONNECTOR-CRASH}"
case "$CASE_ID" in
  FM-RELAYER-CRASH) TARGET=fhevm-relayer ;;
  FM-KMS-CONNECTOR-CRASH) TARGET=kms-connector-kms-worker ;;
  *) echo "unknown request case $CASE_ID" >&2; exit 2 ;;
esac
KMS=kms-connector-kms-worker
SUITE_PID=""
BASELINE_OWNED=0
BASELINE="${SC_CASE_BASELINE:-}"
if [[ -z "$BASELINE" ]]; then BASELINE="$(mktemp)"; BASELINE_OWNED=1; fi
SUITE_LOG="$(mktemp)"
ack() {
  docker exec -i "$TEST_CONTAINER" sh -c "cat > '$HANDSHAKE_DIR/request-fault.json'" <<JSON
{"name":"request-fault","ready":true,"payload":{"applied":$1,"detail":"$2"}}
JSON
}
cleanup() {
  local status=$? cleanup_ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  # Never heal a held owner until both the original client and its journal are safe.
  if ! sp_cancel_all || ! sp_recover_suite_state; then
    rs_finalize_results 1 failed; exit 1
  fi
  [[ -z "$SUITE_PID" ]] || { wait "$SUITE_PID" 2>/dev/null || true; SUITE_PID=""; }
  sc_run_restores || cleanup_ok=0
  sc_restore_running "$BASELINE" || cleanup_ok=0
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  if [[ "$cleanup_ok" == 1 ]]; then
    rs_finalize_results "$status" ok || { [[ "$status" != 0 ]] || status=1; }
    if [[ ! -f "$SP_RUNTIME_DIR/cleanup-failed" ]]; then
      sp_dispose || status=1
      [[ "$BASELINE_OWNED" != 1 ]] || rm -f "$BASELINE"
      rm -f "$SUITE_LOG"
    fi
  else
    rs_finalize_results 1 failed
    status=1
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

fail() {
  cr_record "$CASE_ID" FAIL started_at="$STARTED" cleanup=failed detail="$*"
  echo "request-recovery: $*" >&2
  exit 1
}
env_value() { sed -n "s/^$1=//p" "$ENV_DIR/coprocessor.env" | tail -1; }
CONSENSUS_SCENARIO="${CONSENSUS_SCENARIO:-three-of-three}"
CONSENSUS_OPERATORS=3
CONSENSUS_THRESHOLD="${CONSENSUS_THRESHOLD:-3}"
cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
  rs_stage_results || exit 1
cr_skip_wrong_scenario "$CASE_ID" && exit 0
STARTED="$(cr_now)"
sp_case_start "$CASE_ID" || fail "cannot establish case deadline"
suite_identity_assert "$TEST_CONTAINER" || fail 'suite source mismatch'
sc_snapshot_running > "$BASELINE" || fail 'cannot snapshot owners'
# Renew the supervisor before pending work exists: resetting it after arming
# would run the old request owner instead of testing crash recovery.
if [[ "$(sc_restart_budget "$TARGET")" == exhausted ]]; then
  sc_register_restore "$TARGET" start || fail "cannot record restart restoration"
  sc_reset_restart_budget "$TARGET" || fail 'cannot renew request-owner restart budget before arming'
fi
# KMS is the durable downstream hold for both cases. The relayer remains live
# long enough to accept and submit the request before its own crash is applied.
sc_pause "$KMS" || fail 'cannot hold KMS processing'
docker exec "$TEST_CONTAINER" sh -c "rm -f '$HANDSHAKE_DIR'/request-*.json"
sp_exec \
  -e RUN_REQUEST_RECOVERY=1 -e "FAILURE_CASE_ID=$CASE_ID" \
  -e COPROCESSOR_COUNT=3 -e CONSENSUS_WATCHDOG_DISABLED=0 -e CONSENSUS_WATCHDOG_STALL_MS=2400000 \
  -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" \
  -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL)" \
  -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS)" \
  -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS)" \
  "$TEST_CONTAINER" npx hardhat test test/consensus/requestRecovery.ts --network "${TEST_NETWORK:-staging}" > "$SUITE_LOG" 2>&1 &
SUITE_PID=$!
json=""
deadline=$((SECONDS + 600))
while ((SECONDS < deadline)); do
  json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/request-target.json" 2>/dev/null)"
  [[ -n "$json" ]] && break
  kill -0 "$SUITE_PID" 2>/dev/null || { cat "$SUITE_LOG"; fail 'SDK request exited before acceptance'; }
  sleep 1
done
job="$(jq -r '.payload.jobId // empty' <<< "$json")"
handle="$(jq -r '.payload.handle // empty' <<< "$json")"
[[ "$job" =~ ^[0-9a-fA-F-]{36}$ && "$handle" =~ ^0x[0-9a-f]{64}$ ]] || fail 'no accepted request identity'
deadline=$((SECONDS + 120))
reference=""
while ((SECONDS < deadline)); do
  row="$(docker exec "$RELAYER_DB_CONTAINER" psql -U postgres -d relayer_db -tAc "SELECT req_status, encode(gw_reference_id,'hex') FROM user_decrypt_req WHERE ext_job_id='$job'::uuid")"
  IFS='|' read -r status reference <<< "$row"
  if [[ "$status" == receipt_received && "$reference" =~ ^[0-9a-f]{64}$ ]]; then
    # The relayer stores the gateway U256 big-endian; KMS event rows use little-endian.
    kms_reference="$(bun -e 'console.log(Buffer.from(process.argv[1], "hex").reverse().toString("hex"))' "$reference")"
    pending="$(docker exec "$DB_CONTAINER" psql -U postgres -d kms-connector -tAc "SELECT count(*) FROM user_decryption_requests WHERE decryption_id=decode('$kms_reference','hex') AND status='pending' AND NOT already_sent")"
    [[ "$pending" == 1 ]] && break
  fi
  sleep 1
done
[[ "${pending:-}" == 1 ]] || fail "accepted job $job was not observed awaiting KMS"
echo "request-recovery: accepted job $job, gateway request $reference is durably pending"
[[ "$TARGET" == "$KMS" ]] || sc_pause "$TARGET" || fail 'cannot freeze accepted request owner'
before="$(sc_identity "$TARGET")"
sc_kill "$TARGET" KILL 1 >/dev/null || fail 'kill failed'
fault_at="$(cr_now)"
sc_clear_restore "$TARGET" resume || fail "cannot retain recovery ledger"
auto_restart=pass
policy="$(docker inspect -f '{{.HostConfig.RestartPolicy.Name}}' "$TARGET")" || fail 'cannot read restart policy'
if [[ "$policy" == no ]]; then
  sc_wait_state "$TARGET" stopped 30 && sc_start "$TARGET" || fail 'request owner did not restart'
  auto_restart=not_evaluated
fi
after="$(sc_wait_replaced "$TARGET" "$before" 120)" || fail 'request owner was not replaced'
[[ "$TARGET" == "$KMS" ]] || sc_resume "$KMS" || fail 'cannot release KMS'
ack true 'accepted request owner was killed and restarted' || fail 'cannot release client polling'
wait "$SUITE_PID"; suite_status=$?
SUITE_PID=""
cat "$SUITE_LOG"
[[ "$suite_status" == 0 ]] || fail 'the original SDK request failed after recovery'
grep -qF "[request-recovery] $CASE_ID CASE COMPLETE" "$SUITE_LOG" || fail 'missing original-request completion assertion'
final="$(docker exec "$RELAYER_DB_CONTAINER" psql -U postgres -d relayer_db -tAc "SELECT req_status FROM user_decrypt_req WHERE ext_job_id='$job'::uuid")"
[[ "$final" == completed ]] || fail "original job $job did not complete ($final)"
sc_run_restores && sc_restore_running "$BASELINE" || fail 'service restoration failed'
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok \
  workload="$job" workload="$reference" workload="$kms_reference" workload="$handle" \
  fault_observed_at="$fault_at" recovery_observed_at="$(cr_now)" \
  process_before="$TARGET=$before" process_after="$TARGET=$after" \
  assert="original-request=pass:one accepted POST, same job completed, plaintext 12" \
  assert="automatic-restart=$auto_restart"
[[ "${CR_RECORD_FAILURES:-0}" == 0 ]]
