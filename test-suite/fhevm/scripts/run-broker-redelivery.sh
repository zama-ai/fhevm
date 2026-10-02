#!/usr/bin/env bash
# Test only Redis's observed redelivery of the same committed host block.
set -uo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/suite-identity.sh"
source "$SCRIPT_DIR/lib/runner-assertions.sh"
source "$SCRIPT_DIR/lib/suite-process.sh"
source "$SCRIPT_DIR/lib/result-staging.sh"
sc_init || exit 1
sp_init || exit 1
CASE_ID=FM-BROKER-REDELIVERY
TARGET=coprocessor1-host-listener-consumer
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"
STATE_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}"
ENV_DIR="$STATE_DIR/runtime/env"
CONTROL=/tmp/fhevm-test-broker-ack
CONTROL_OWNED=0
cleanup() {
  local status=$? ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  sp_cancel_all && sp_recover_suite_state || ok=0
  sc_run_restores || ok=0
  if [[ "$CONTROL_OWNED" == 1 ]]; then
    # The consumer may be mid-restart (its own back-off after the kill) when
    # cleanup runs; a single failed exec would leave the control files in the
    # container layer and every later run would refuse at the `empty` check.
    local attempt cleared=0
    for attempt in 1 2 3 4 5 6; do
      if docker exec -u 0 "$TARGET" host_listener_consumer --consensus-test-control clear; then cleared=1; break; fi
      sc_start "$TARGET" >/dev/null 2>&1 || true
      sleep 5
    done
    if [[ "$cleared" != 1 ]]; then
      echo "broker ACK control could not be cleared; recover with: docker exec -u 0 $TARGET host_listener_consumer --consensus-test-control clear" >&2
      ok=0
    fi
  fi
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  if [[ "$ok" == 1 ]]; then rs_finalize_results "$status" ok || status=1; sp_dispose || status=1
  else rs_finalize_results 1 failed; status=1; fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
fail() { cr_record "$CASE_ID" FAIL started_at="$STARTED" cleanup=failed detail="$*"; echo "$*" >&2; exit 1; }
env_value() { sed -n "s/^$1=//p" "$ENV_DIR/coprocessor.env" | tail -1; }
ack() {
  docker exec -i "$TEST_CONTAINER" sh -c 'mkdir -p "${1%/*}" && cat > "$1"' sh "$HANDSHAKE_DIR/failure-fault.json" <<JSON
{"name":"failure-fault","ready":true,"payload":{"caseId":"$CASE_ID","applied":true,"faultObservedAt":"$FAULT_AT","recoveryObservedAt":"${1:-}","detail":"owned consumer interruption before ACK"}}
JSON
}
phase() {
  local phase="$1" output=""
  cr_run_suite output "[failure-case/$CASE_ID/$phase] CASE COMPLETE" sp_exec \
    -e RUN_FAILURE_CASE=1 -e "FAILURE_CASE_ID=$CASE_ID" -e FAILURE_WORKLOAD=ingestion -e "FAILURE_PHASE=$phase" \
    -e FAILURE_VICTIM_OPERATOR=1 -e COPROCESSOR_COUNT=3 -e CONSENSUS_THRESHOLD=3 \
    -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" -e CONSENSUS_WATCHDOG_DISABLED=0 -e CONSENSUS_WATCHDOG_STALL_MS=2400000 \
    -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL)" -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS)" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS)" \
    "$TEST_CONTAINER" npx hardhat test test/consensus/failureCase.ts --network "${TEST_NETWORK:-staging}" > "$EVIDENCE/$phase.log" 2>&1
  local result=$?
  printf '%s\n' "$output" >> "$EVIDENCE/$phase.log"
  cat "$EVIDENCE/$phase.log"
  return "$result"
}
observed="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" 3)" || exit 1
eval "$observed"
export CONSENSUS_SCENARIO CONSENSUS_OPERATORS CONSENSUS_THRESHOLD
cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
rs_stage_results || exit 1
cr_skip_wrong_scenario "$CASE_ID" && exit 0
STARTED="$(cr_now)"
sp_case_start "$CASE_ID" || fail 'cannot establish deadline'
suite_identity_assert "$TEST_CONTAINER" || fail 'suite source mismatch'
identity=()
cr_read_run_identity identity || fail 'missing build identity'
command="$(docker inspect -f '{{range .Config.Cmd}}{{println .}}{{end}}' "$TARGET")" || fail 'consumer absent'
grep -q -- '--url=redis://' <<<"$command" || fail 'this case requires the managed Redis stream backend'
[[ "$(sc_state "$TARGET")" == running ]] || fail 'consumer is not healthy'
control_status=0
docker exec "$TARGET" host_listener_consumer --consensus-test-control empty || control_status=$?
case "$control_status" in
  0) ;;
  # clap rejects the unknown flag with 2: the binary has no control at all.
  2) fail 'the consumer image was built without host-listener/test-failpoints; this case needs the fault-enabled build' ;;
  *) fail "prior ACK control requires recovery: docker exec -u 0 $TARGET host_listener_consumer --consensus-test-control clear" ;;
esac
if [[ "$(sc_restart_budget "$TARGET")" == exhausted ]]; then sc_reset_restart_budget "$TARGET" || fail 'cannot renew restart budget'; fi
mkdir -p "$STATE_DIR/runtime/broker-redelivery" || fail 'cannot create evidence root'
EVIDENCE="$(mktemp -d "$STATE_DIR/runtime/broker-redelivery/case.XXXXXX")" || fail 'cannot create evidence directory'
sc_pause "$TARGET" || fail 'cannot hold consumer before the target exists'
for other in coprocessor1-host-listener coprocessor1-host-listener-poller; do sc_stop "$other" || fail 'cannot isolate broker delivery'; done
FAULT_AT="$(cr_now)"
ack || fail 'cannot acknowledge ingestion hold'
phase arm || fail 'named transaction did not arm behind the held consumer'
json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/failure-workload.json")" || fail 'workload receipt missing'
block="$(jq -r '.payload.detail.blockHash' <<<"$json")"
handle="$(jq -r '.payload.handles[0]' <<<"$json")"
transaction="$(jq -r '.payload.transactionHashes[0]' <<<"$json")"
[[ "$block" =~ ^0x[0-9a-f]{64}$ && "$handle" =~ ^0x[0-9a-f]{64}$ && "$transaction" =~ ^0x[0-9a-f]{64}$ ]] || fail 'incomplete selected block/workload identity'
jq -n --arg block "$block" --argjson until "$(($(date +%s)+600))" '{block_hash:$block,until:$until}' > "$EVIDENCE/control.json"
CONTROL_OWNED=1
docker cp "$EVIDENCE/control.json" "$TARGET:$CONTROL" || fail 'cannot install ACK boundary control'
sc_resume "$TARGET" || fail 'cannot deliver the selected block'
deadline=$((SECONDS + 240))
first=""
for (( ; SECONDS < deadline; )); do
  first="$(docker exec "$TARGET" host_listener_consumer --consensus-test-control observed 2>/dev/null)" || first=""
  if jq -e --arg block "$block" --arg tx "$transaction" '.redelivered==false and .payload.flow=="LIVE" and .payload.block_hash==$block and any(.payload.transactions[]; .hash==$tx)' <<<"$first" >/dev/null 2>&1; then break; fi
  hc_time_left || fail 'case deadline expired'
  sleep 1
done
jq -e --arg block "$block" --arg tx "$transaction" '.redelivered==false and .payload.flow=="LIVE" and .payload.block_hash==$block and any(.payload.transactions[]; .hash==$tx)' <<<"$first" >/dev/null || fail 'selected successful handler never reached the pre-ACK gate'
printf '%s\n' "$first" > "$EVIDENCE/first.json"
rows="$(docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor_1 -At -v ON_ERROR_STOP=1 -c "SELECT count(*) FROM computations WHERE transaction_id=decode('${transaction#0x}','hex') AND output_handle=decode('${handle#0x}','hex')")" || fail 'cannot inspect committed ingestion'
[[ "$rows" == 1 ]] || fail 'target ingestion was not committed before the missing ACK'
stream="$(jq -r '.queue' <<<"$first")"
message_id="$(jq -r '.message_id' <<<"$first")"
[[ "$stream" =~ ^[a-zA-Z0-9._:-]+$ && "$message_id" =~ ^[0-9]+-[0-9]+$ ]] || fail 'invalid broker message identity'
groups="$(docker exec listener-redis redis-cli --json XINFO GROUPS "$stream")" || fail 'cannot inspect consumer groups'
group="$(jq -er 'if length!=1 then error("ambiguous stream group") else .[0] | if type=="object" then .name else . as $row | [range(0;length;2) | {key:$row[.],value:$row[.+1]}] | from_entries | .name end end' <<<"$groups")" || fail 'cannot identify the unique consumer group'
pending() { docker exec listener-redis redis-cli --json XPENDING "$stream" "$group" "$message_id" "$message_id" 1; }
pending > "$EVIDENCE/pending-before.json" || fail 'XPENDING unavailable before interruption'
# The normal pending drain uses XREADGROUP before suppressing an in-flight
# duplicate, so Redis may already have incremented this counter while our
# successful handler is held. Require an increase over the observed baseline,
# together with the replacement's second successful-handler receipt.
pending_before="$(jq -er --arg id "$message_id" 'if length==1 and .[0][0]==$id and .[0][3]>=1 then .[0][3] else error("not pending") end' "$EVIDENCE/pending-before.json")" || fail 'the target was not pending before interruption'
if docker exec "$TARGET" host_listener_consumer --consensus-test-control redelivered >/dev/null 2>&1; then
  fail 'a second handler reached the gate before the selected interruption'
fi
(( $(date +%s) < $(jq -r .until "$EVIDENCE/control.json") )) || fail 'ACK gate expired before interruption'
before="$(sc_kill "$TARGET")" || fail 'cannot interrupt pre-ACK consumer'
sc_wait_replaced "$TARGET" "$before" 120 || fail 'consumer was not automatically replaced'
after="$(sc_identity "$TARGET")"
deadline=$((SECONDS + 180))
redelivered=""
for (( ; SECONDS < deadline; )); do
  redelivered="$(docker exec "$TARGET" host_listener_consumer --consensus-test-control redelivered 2>/dev/null)" || redelivered=""
  if jq -e '.redelivered==true' <<<"$redelivered" >/dev/null 2>&1; then break; fi
  hc_time_left || fail 'case deadline expired'
  sleep 1
done
printf '%s\n' "$redelivered" > "$EVIDENCE/redelivered.json"
jq -e -s '.[0].redelivered==false and .[1].redelivered==true and .[0].queue==.[1].queue and .[0].message_id==.[1].message_id and .[0].payload==.[1].payload' "$EVIDENCE/first.json" "$EVIDENCE/redelivered.json" >/dev/null || fail 'broker did not redeliver the identical queue payload to the replacement'
pending > "$EVIDENCE/pending-after.json" || fail 'XPENDING unavailable after replacement'
jq -e --arg id "$message_id" --argjson baseline "$pending_before" 'length==1 and .[0][0]==$id and .[0][3]>$baseline' "$EVIDENCE/pending-after.json" >/dev/null || fail 'broker pending-entry count did not increase after replacement'
(( $(date +%s) < $(jq -r .until "$EVIDENCE/control.json") )) || fail 'ACK gate expired before redelivery observation'
docker exec -u 0 "$TARGET" host_listener_consumer --consensus-test-control clear || fail 'cannot disarm ACK control'
CONTROL_OWNED=0
deadline=$((SECONDS+60))
until [[ "$(pending)" == '[]' ]]; do
  ((SECONDS < deadline)) || fail 'replacement never acknowledged the replayed entry'
  sleep 1
done
# Only restore the alternate paths after the replacement has handled the same
# pending payload and Redis confirms its ACK. The shared ingestion verifier also
# requires the poller cursor to catch up; keeping that poller stopped would make
# a successful broker recovery fail on an unrelated, deliberately frozen cursor.
for other in coprocessor1-host-listener coprocessor1-host-listener-poller; do
  sc_start "$other" || fail 'cannot restore alternate ingestion after verified redelivery'
done
ack "$(cr_now)" || fail 'cannot acknowledge actual redelivery recovery'
phase verify || fail 'original work did not survive duplicate ingestion'
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$FAULT_AT" recovery_observed_at="$(cr_now)" \
  "workload=$handle" "process_before=$TARGET=$before" "process_after=$TARGET=$after" \
  "assert=redelivery=pass:committed target ingestion preceded process death without ACK; replacement handled an identical same-queue payload with the independently queried Redis pending-entry delivery count" \
  "artifact=broker_evidence=$EVIDENCE" "${identity[@]}" || fail 'missing recovery assertions'
