#!/usr/bin/env bash
# No physical allocation: constrain one opted-in process's reservation admission.
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
gpu_normalise_user_bus
CASE_ID=SCH-04-GPU-RESERVATION
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"
STATE_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}"
ENV_DIR="$STATE_DIR/runtime/env"
SUITE_PID=""
CONTROL=""
cleanup() {
  local status=$? ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  # Release admission before cancellation; the expiring control is also bounded
  # if this owner is killed without an EXIT trap.
  [[ -z "$CONTROL" ]] || rm -f "$CONTROL" "$CONTROL.observed" || ok=0
  sp_cancel_all && sp_recover_suite_state || ok=0
  [[ -z "$SUITE_PID" ]] || wait "$SUITE_PID" 2>/dev/null || true
  sc_run_restores || ok=0
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  if [[ "$ok" == 1 ]]; then
    rs_finalize_results "$status" ok || status=1
    sp_dispose || status=1
  else rs_finalize_results 1 failed; status=1; fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
fail() { cr_record "$CASE_ID" FAIL started_at="$STARTED" cleanup=failed detail="$*"; echo "$*" >&2; exit 1; }
ack() {
  docker exec -i "$TEST_CONTAINER" sh -c 'cat > "$1"' sh "$HANDSHAKE_DIR/$1.json" <<JSON
{"name":"$1","ready":true,"payload":{"applied":true,"detail":"host verified $1"}}
JSON
}
wait_handshake() {
  local name="$1" json deadline=$((SECONDS + 480))
  while ((SECONDS < deadline)); do
    json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/$name.json" 2>/dev/null)" || json=""
    if [[ -n "$json" ]]; then printf '%s' "$json"; return 0; fi
    kill -0 "$SUITE_PID" 2>/dev/null && hc_time_left || return 1
    sleep 1
  done
  return 1
}
env_value() { sed -n "s/^$1=//p" "$ENV_DIR/coprocessor.env" | tail -1; }
observed="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" 3)" || exit 1
eval "$observed"
export CONSENSUS_SCENARIO CONSENSUS_OPERATORS CONSENSUS_THRESHOLD
cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
rs_stage_results || exit 1
cr_skip_wrong_scenario "$CASE_ID" && exit 0
STARTED="$(cr_now)"
sp_case_start "$CASE_ID" || fail 'cannot establish deadline'
suite_identity_assert "$TEST_CONTAINER" || fail 'suite source differs'
identity=()
cr_read_run_identity identity || fail 'missing build identity'
unit=fhevm-gpu-consensus-tfhe-1
pid="$(gpu_unit_main_pid "$unit")"
[[ "$pid" =~ ^[1-9][0-9]+$ && -d "/proc/$pid" ]] || fail 'victim GPU process is absent'
config="$STATE_DIR/runtime/gpu-consensus-workers/invocations/$unit.config"
grep -q '^build_test_features=.*tfhe-worker/test-failpoints' "$config" || fail 'GPU worker was not built with the opt-in hook'
candidate_control="/tmp/fhevm-test-gpu-reservation-$pid"
[[ ! -e "$candidate_control" && ! -e "$candidate_control.observed" ]] || fail 'previous pressure control requires recovery'
metrics="$(gpu_worker_metrics_urls 3 "$TEST_CONTAINER" coprocessor-and-kms-db)"
[[ -n "$metrics" ]] || fail 'GPU metrics routing unavailable'
docker exec "$TEST_CONTAINER" sh -c 'mkdir -p "$1"; rm -f "$1"/pressure-*.json' sh "$HANDSHAKE_DIR" || fail 'cannot clear handshakes'
mkdir -p "$STATE_DIR/runtime/gpu-pressure" || fail 'cannot create evidence root'
EVIDENCE="$(mktemp -d "$STATE_DIR/runtime/gpu-pressure/case.XXXXXX")" || fail 'cannot create evidence directory'
sp_exec -e RUN_GPU_PRESSURE=1 -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" -e "TFHE_WORKER_METRICS_URLS=$metrics" \
  -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL)" -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS)" \
  -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS)" \
  "$TEST_CONTAINER" npx hardhat test test/consensus/gpuPressure.ts --network "${TEST_NETWORK:-staging}" > "$EVIDENCE/suite.log" 2>&1 &
SUITE_PID=$!
wait_handshake pressure-ready >/dev/null || fail 'no healthy prepared input'
since="$(cr_now)"
sc_pause coprocessor1-tfhe-worker || fail 'cannot hold victim while binding transaction control'
CONTROL="$candidate_control"
printf '%s\n' "$(($(date +%s) + 600))" > "$CONTROL" || fail 'cannot install expiring admission limit'
ack pressure-armed || fail 'cannot acknowledge admission limit'
json="$(wait_handshake pressure-target)" || fail 'no named pressure target'
handle="$(jq -r '.payload.handle' <<<"$json")"
[[ "$handle" =~ ^0x[0-9a-f]{64}$ ]] || fail 'invalid pressure target'
transaction="$(jq -r '.payload.transactionHash' <<<"$json")"
[[ "$transaction" =~ ^0x[0-9a-f]{64}$ ]] || fail 'invalid pressure transaction'
printf '%s %s\n' "$(($(date +%s) + 600))" "${transaction#0x}" > "$CONTROL" || fail 'cannot select one transaction'
sc_resume coprocessor1-tfhe-worker || fail 'cannot resume selected reservation owner'
deadline=$((SECONDS + 420))
observed_retry=0
while ((SECONDS < deadline)); do
  [[ "$(gpu_unit_main_pid "$unit")" == "$pid" ]] || fail 'pressure owner changed; cannot attribute this control'
  logs="$(sc_logs_since coprocessor1-tfhe-worker "$since")" || fail 'worker journal unavailable'
  if [[ -s "$CONTROL.observed" ]] && grep -Fq "transaction=${transaction#0x}" "$CONTROL.observed" && grep -F "${handle#0x}" <<<"$logs" | grep -Fq 'transient GPU memory reservation failure; leaving computation pending for retry'; then observed_retry=1; break; fi
  kill -0 "$SUITE_PID" 2>/dev/null && hc_time_left || fail 'suite exited before named reservation retry'
  sleep 1
done
[[ "$observed_retry" == 1 ]] || fail 'named target never reached the actual reservation timeout/retry path'
cp "$CONTROL.observed" "$EVIDENCE/admission.txt" || fail 'cannot retain admission receipt'
grep -F "${handle#0x}" <<<"$logs" > "$EVIDENCE/retry.log"
ack pressure-observed || fail 'cannot acknowledge observed retry'
wait_handshake pressure-release >/dev/null || fail 'pending/no-publication checks did not complete'
[[ "$(gpu_unit_main_pid "$unit")" == "$pid" ]] || fail 'worker changed during same-worker progress checkpoints'
[[ -e "$CONTROL" && $(date +%s) -lt $(cut -d' ' -f1 "$CONTROL") ]] || fail 'pressure expired before the negative checkpoint'
rm -f "$CONTROL" "$CONTROL.observed" || fail 'cannot release admission control'
ack pressure-restored || fail 'cannot acknowledge release'
status=0
wait "$SUITE_PID" || status=$?
SUITE_PID=""
cat "$EVIDENCE/suite.log"
[[ "$status" == 0 ]] && grep -Fq '[gpu-pressure] CASE COMPLETE' "$EVIDENCE/suite.log" || fail 'original GPU work did not recover'
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$since" \
  recovery_observed_at="$(cr_now)" "workload=$handle" "artifact=pressure_evidence=$EVIDENCE" "${identity[@]}" || fail 'pressure assertions missing'
