#!/usr/bin/env bash
# Fault only freshly produced named objects; retain every original before writes.
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
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
HANDSHAKE_DIR="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}"
ENV_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/env"
CASE_ID=STORAGE-01-INTEGRITY
SUITE_PID=""
JOURNALS=()
EVIDENCE_DIR=""
restore_objects() {
  local journal failed=0
  for journal in "${JOURNALS[@]}"; do
    hc_run bun "$SCRIPT_DIR/object-store-control.ts" restore "$journal" || failed=1
  done
  [[ "$failed" == 0 ]]
}
cleanup() {
  local status=$? ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  sp_cancel_all || ok=0
  sp_recover_suite_state || ok=0
  [[ -z "$SUITE_PID" ]] || wait "$SUITE_PID" 2>/dev/null || true
  restore_objects || ok=0
  if [[ "$ok" != 1 ]]; then
    rs_finalize_results 1 failed
    echo "object recovery failed; retain $SP_RUNTIME_DIR and discard this stack until restored" >&2
    exit 1
  fi
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  rs_finalize_results "$status" ok || status=1
  sp_dispose || status=1
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
fail() { cr_record "$CASE_ID" FAIL started_at="$STARTED" cleanup=failed detail="$*"; echo "$*" >&2; exit 1; }
ack() {
  local name="$1"
  docker exec -i "$TEST_CONTAINER" sh -c 'cat > "$1"' sh "$HANDSHAKE_DIR/$name.json" <<JSON
{"name":"$name","ready":true,"payload":{"applied":true,"detail":"host verified $name"}}
JSON
}
wait_handshake() {
  local name="$1" deadline=$((SECONDS + 600)) json
  while ((SECONDS < deadline)); do
    json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/$name.json" 2>/dev/null)" || json=""
    if [[ -n "$json" ]]; then printf '%s' "$json"; return 0; fi
    kill -0 "$SUITE_PID" 2>/dev/null || return 1
    hc_time_left || return 1
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
mkdir -p "${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/object-integrity" || fail 'cannot create evidence root'
EVIDENCE_DIR="$(mktemp -d "${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/object-integrity/case.XXXXXX")" || fail 'cannot create evidence directory'
sp_case_start "$CASE_ID" || fail 'cannot establish object case deadline'
suite_identity_assert "$TEST_CONTAINER" || fail 'suite source differs'
identity=()
cr_read_run_identity identity || fail 'missing build identity'
docker exec "$TEST_CONTAINER" sh -c 'mkdir -p "$1"; rm -f "$1"/object-target-*.json "$1"/object-armed-*.json "$1"/object-request-*.json "$1"/object-restored-*.json' sh "$HANDSHAKE_DIR" || fail 'cannot clear prior handshakes'
LOG="$EVIDENCE_DIR/object-suite.log"
sp_exec -e RUN_OBJECT_INTEGRITY=1 -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" \
  -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL)" -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS)" \
  -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS)" \
  "$TEST_CONTAINER" npx hardhat test test/consensus/objectIntegrity.ts --network "${TEST_NETWORK:-staging}" > "$LOG" 2>&1 &
SUITE_PID=$!
workloads=()
fault_time=""
for mode in missing truncated wrong-handle wrong-key wrong-format; do
  json="$(wait_handshake "object-target-$mode")" || fail "no $mode object target"
  handle="$(jq -r '.payload.handle' <<<"$json")"
  alternate="$(jq -r '.payload.alternate' <<<"$json")"
  journal="$EVIDENCE_DIR/objects-$mode.json"
  JOURNALS+=("$journal")
  hc_run bun "$SCRIPT_DIR/object-store-control.ts" arm "$journal" "$mode" "$handle" "$alternate" || fail "$mode object mutation not observed"
  since="$(cr_now)"
  [[ -n "$fault_time" ]] || fault_time="$since"
  workloads+=("workload=$handle")
  ack "object-armed-$mode" || fail 'cannot acknowledge object fault'
  json="$(wait_handshake "object-request-$mode")" || fail 'no accepted decryption request'
  job="$(jq -r '.payload.jobId' <<<"$json")"
  [[ "$job" =~ ^[0-9a-f-]{36}$ ]] || fail 'invalid request ID'
  deadline=$((SECONDS + 180))
  service=fhevm-relayer
  expected_status=queued
  pattern='Failed to fetch attestation: HEAD request failed: status 404'
  case "$mode" in
    truncated)
      service=kms-connector-kms-worker
      expected_status=receipt_received
      pattern='ciphertext digest verification|digest mismatch'
      ;;
    wrong-handle|wrong-key|wrong-format)
      pattern='Discarding invalid attestation: signer mismatch'
      ;;
  esac
  observed_failure=0
  while ((SECONDS < deadline)); do
    logs="$(sc_logs_since "$service" "$since")" || fail "$service logs unavailable"
    if grep -F "$handle" <<<"$logs" | grep -Eiq "$pattern"; then observed_failure=1; break; fi
    kill -0 "$SUITE_PID" 2>/dev/null || fail 'client exited before the intended rejection'
    hc_time_left || fail 'object case deadline expired'
    sleep 1
  done
  [[ "$observed_failure" == 1 ]] || fail "$mode never reached the selected retrieval/verifier failure"
  # No successful client job may be claimed while all serving objects are bad.
  status="$(docker exec fhevm-relayer-db psql -U postgres -d relayer_db -At -v ON_ERROR_STOP=1 -c "SELECT req_status FROM user_decrypt_req WHERE ext_job_id='$job'::uuid")" || fail 'cannot inspect original client request'
  # Missing objects and rejected signatures are retried by relayer readiness;
  # truncated bytes pass that check and are retried by KMS after submission.
  # Terminal disagreement requires conflicting valid attestations, not these
  # invalid signatures. Restore while the same request remains pending.
  [[ "$status" == "$expected_status" ]] || fail "$mode request state $status, expected $expected_status"
  printf '%s\n' "$service $job $status" > "$EVIDENCE_DIR/request-$mode.txt"
  grep -F "$handle" <<<"$logs" | grep -Ei "$pattern" > "$EVIDENCE_DIR/rejection-$mode.log"
  hc_run bun "$SCRIPT_DIR/object-store-control.ts" restore "$journal" || fail 'original objects were not restored'
  ack "object-restored-$mode" || fail 'cannot acknowledge object restoration'
done
status=0
wait "$SUITE_PID" || status=$?
SUITE_PID=""
cat "$LOG"
[[ "$status" == 0 ]] || fail 'object rejection or restored-request recovery failed'
grep -Fq '[object-integrity] CASE COMPLETE' "$LOG" || fail 'missing complete object coverage marker'
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$fault_time" \
  recovery_observed_at="$(cr_now)" "artifact=object_evidence=$EVIDENCE_DIR" "${workloads[@]}" "${identity[@]}" || fail 'object assertion evidence was incomplete'
