#!/usr/bin/env bash
# Distinguish three computing operators from one, two and three visible senders.
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
CASE_ID=DEG-07-SUBMISSION-PARTITION
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"
STATE_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}"
ENV_DIR="$STATE_DIR/runtime/env"
cleanup() {
  local status=$? ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  sp_cancel_all && sp_recover_suite_state || ok=0
  sc_run_restores || ok=0
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
phase() {
  local phase="$1" output=""
  cr_run_suite output "[submission-partition/$phase] CASE COMPLETE" sp_exec \
    -e RUN_SUBMISSION_PARTITION=1 -e "SUBMISSION_PARTITION_PHASE=$phase" \
    -e COPROCESSOR_COUNT=3 -e CONSENSUS_THRESHOLD=2 \
    -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" -e CONSENSUS_WATCHDOG_DISABLED=0 -e CONSENSUS_WATCHDOG_STALL_MS=2400000 \
    -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL)" -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS)" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS)" \
    "$TEST_CONTAINER" npx hardhat test test/consensus/submissionPartition.ts --network "${TEST_NETWORK:-staging}" > "$EVIDENCE/$phase.log" 2>&1
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
[[ "$CONSENSUS_THRESHOLD" == 2 ]] || fail 'requires the actual threshold-two topology'
sp_case_start "$CASE_ID" || fail 'cannot establish case deadline'
suite_identity_assert "$TEST_CONTAINER" || fail 'suite source differs'
identity=()
cr_read_run_identity identity || fail 'missing source identity'
mkdir -p "$STATE_DIR/runtime/submission-partition" || fail 'cannot create evidence root'
EVIDENCE="$(mktemp -d "$STATE_DIR/runtime/submission-partition/case.XXXXXX")" || fail 'cannot create evidence directory'
docker exec "$TEST_CONTAINER" sh -c 'mkdir -p "$1"; rm -f "$1/submission-partition.json"' sh "$HANDSHAKE_DIR" || fail 'cannot clear prior target receipt'
for operator in 1 2; do sc_stop "coprocessor$operator-transaction-sender" || fail 'cannot isolate sender'; done
FAULT_AT="$(cr_now)"
phase arm || fail 'no valid one-sender negative observation with all operators computing'
for operator in 1 2; do [[ "$(sc_state "coprocessor$operator-transaction-sender")" == stopped ]] || fail 'sender returned during the negative observation'; done
sc_start coprocessor1-transaction-sender || fail 'cannot restore the second sender'
[[ "$(sc_state coprocessor2-transaction-sender)" == stopped ]] || fail 'third sender returned before the majority checkpoint'
phase majority || fail 'the original handle did not reach threshold with two senders'
[[ "$(sc_state coprocessor2-transaction-sender)" == stopped ]] || fail 'third sender returned during the majority checkpoint'
sc_start coprocessor2-transaction-sender || fail 'cannot complete asymmetric rejoin'
phase rejoin || fail 'original and fresh work did not converge and decrypt after rejoin'
json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/submission-partition.json")" || fail 'original target receipt missing'
handle="$(jq -r '.payload.handle' <<<"$json")"
[[ "$handle" =~ ^0x[0-9a-f]{64}$ ]] || fail 'invalid original workload identity'
printf '%s\n' "$json" > "$EVIDENCE/workload.json"
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$FAULT_AT" recovery_observed_at="$(cr_now)" \
  "workload=$handle" "artifact=partition_evidence=$EVIDENCE" "${identity[@]}" || fail 'missing per-phase assertions'
