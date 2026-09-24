#!/usr/bin/env bash
# Isolate a single object write; never fill the host's shared storage.
set -uo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
CASE_ID=FM-STORAGE-WRITE-BACKPRESSURE
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
STATE_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}"
ENV_DIR="$STATE_DIR/runtime/env"
TARGET=coprocessor1-sns-worker
PROXY_PID=""
ROUTE_CHANGED=0
compose=()
PRIVATE=""
restore_route() {
  [[ "$ROUTE_CHANGED" == 1 ]] || return 0
  docker "${compose[@]}" up -d --no-deps --force-recreate "$TARGET" >/dev/null || return 1
  docker inspect "$TARGET" > "$PRIVATE/restored.json" || return 1
  jq -e -s '.[0][0].Config.Cmd == .[1][0].Config.Cmd and .[0][0].Image == .[1][0].Image and (.[0][0].Config.Env | sort) == (.[1][0].Config.Env | sort)' "$PRIVATE/original.json" "$PRIVATE/restored.json" >/dev/null || return 1
  ROUTE_CHANGED=0
}
cleanup() {
  local status=$? ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  # Restore the writer before terminating its temporary endpoint.
  restore_route || ok=0
  sp_cancel_all && sp_recover_suite_state || ok=0
  [[ -z "$PROXY_PID" ]] || wait "$PROXY_PID" 2>/dev/null || true
  sc_run_restores || ok=0
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  if [[ "$ok" == 1 ]]; then
    rs_finalize_results "$status" ok || status=1
    sp_dispose || status=1
  else
    rs_finalize_results 1 failed; status=1
    echo "restore the saved route and service ledger in $PRIVATE before reusing this stack" >&2
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
fail() { cr_record "$CASE_ID" FAIL started_at="$STARTED" cleanup=failed detail="$*"; echo "$*" >&2; exit 1; }
env_value() { sed -n "s/^$1=//p" "$ENV_DIR/coprocessor.env" | tail -1; }
control() {
  docker exec "$TEST_CONTAINER" node -e 'const fs=require("fs");const [file,mode,path]=process.argv.slice(1);fs.writeFileSync(file+".new",JSON.stringify({mode,path,until:Date.now()+1200000}));fs.renameSync(file+".new",file)' "$REMOTE/control" "$1" "${OBJECT_PATH:-}"
}
ack() {
  docker exec -i "$TEST_CONTAINER" sh -c 'cat > "$1"' sh "$HANDSHAKE_DIR/failure-fault.json" <<JSON
{"name":"failure-fault","ready":true,"payload":{"caseId":"$CASE_ID","applied":true,"faultObservedAt":"$FAULT_AT","recoveryObservedAt":"${1:-}","detail":"named object write rejection observed before publication"}}
JSON
}
phase() {
  local phase="$1" output=""
  cr_run_suite output "[failure-case/$CASE_ID/$phase] CASE COMPLETE" sp_exec \
    -e RUN_FAILURE_CASE=1 -e "FAILURE_CASE_ID=$CASE_ID" -e FAILURE_WORKLOAD=sns-noisy -e "FAILURE_PHASE=$phase" \
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
sp_case_start "$CASE_ID" || fail 'cannot establish case deadline'
suite_identity_assert "$TEST_CONTAINER" || fail 'suite source differs'
identity=()
cr_read_run_identity identity || fail 'missing source identity'
[[ "$(sc_kind "$TARGET")" == container && "$(sc_state "$TARGET")" == running ]] || fail 'requires one healthy managed CPU SNS worker'
umask 077
mkdir -p "$STATE_DIR/runtime/storage-pressure-private" "$STATE_DIR/runtime/storage-pressure-evidence" || fail 'cannot create journal roots'
PRIVATE="$(mktemp -d "$STATE_DIR/runtime/storage-pressure-private/case.XXXXXX")" || fail 'cannot create private recovery directory'
SC_RESTORE_LOG="$PRIVATE/restores.log"
touch "$SC_RESTORE_LOG" || fail 'cannot initialize private restoration ledger'
EVIDENCE="$(mktemp -d "$STATE_DIR/runtime/storage-pressure-evidence/case.XXXXXX")" || fail 'cannot create evidence directory'
docker inspect "$TARGET" > "$PRIVATE/original.json" || fail 'SNS worker absent'
upstream="$(bun "$SCRIPT_DIR/route-storage-writer.ts" "$PRIVATE/original.json")" || fail 'ambiguous original endpoint'
project="$(jq -r '.[0].Config.Labels["com.docker.compose.project"]' "$PRIVATE/original.json")"
files="$(jq -r '.[0].Config.Labels["com.docker.compose.project.config_files"]' "$PRIVATE/original.json")"
[[ -n "$project" && "$project" != null && -n "$files" && "$files" != null ]] || fail 'original compose identity unavailable'
compose=(compose -p "$project")
IFS=',' read -r -a config_files <<<"$files"
for file in "${config_files[@]}"; do [[ -f "$file" ]] || fail 'original compose file missing'; compose+=(-f "$file"); done
[[ ! -f "$ENV_DIR/versions.env" ]] || compose+=(--env-file "$ENV_DIR/versions.env")
network_mode="$(docker inspect -f '{{.HostConfig.NetworkMode}}' "$TEST_CONTAINER")"
[[ "$network_mode" == container:* ]] || fail 'proxy requires the managed suite network namespace'
# A hostname makes the S3 SDK prepend the bucket name, which Docker DNS does
# not resolve. Keep the IP-literal endpoint used by the original local store so
# both bucket probes and named PUTs retain path-style addressing.
proxy_host="$(docker inspect "${network_mode#container:}" | jq -er '.[0].NetworkSettings.Networks | [.[].IPAddress | select(length > 0)] | if length == 1 then .[0] else error("ambiguous proxy network") end')" || fail 'proxy network address unavailable'
[[ "$proxy_host" =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}$ ]] || fail 'proxy requires an IPv4 address'
sc_stop "$TARGET" || fail 'cannot hold SNS before arming'
phase arm || fail 'could not retain computed work before squash and upload'
json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/failure-workload.json")" || fail 'target receipt unavailable'
handle="$(jq -r '.payload.handles | if length==1 then .[0] else empty end' <<<"$json")"
[[ "$handle" =~ ^0x[0-9a-f]{64}$ ]] || fail 'one named object is required'
OBJECT_PATH="/coproc-1/ct128/${handle#0x}/1"
REMOTE="$HANDSHAKE_DIR/storage-${handle#0x}"
docker exec "$TEST_CONTAINER" mkdir -p "$REMOTE" || fail 'cannot create proxy directory'
docker cp "$SCRIPT_DIR/lib/storage-write-proxy.cjs" "$TEST_CONTAINER:$REMOTE/proxy.cjs" || fail 'cannot install proxy'
control reject || fail 'cannot initialize write rejection'
sp_exec "$TEST_CONTAINER" node "$REMOTE/proxy.cjs" "$upstream" "$REMOTE/control" "$REMOTE/ready" "$REMOTE/requests" > "$EVIDENCE/proxy.log" 2>&1 &
PROXY_PID=$!
ready=""
for ((attempt=0;attempt<30;attempt++)); do
  ready="$(docker exec "$TEST_CONTAINER" cat "$REMOTE/ready" 2>/dev/null)" || ready=""
  [[ -z "$ready" ]] || break
  kill -0 "$PROXY_PID" 2>/dev/null || fail 'proxy failed to start'
  sleep 1
done
port="$(jq -r '.port' <<<"$ready")"
[[ "$port" =~ ^[1-9][0-9]+$ ]] || fail 'proxy never became ready'
bun "$SCRIPT_DIR/route-storage-writer.ts" "$PRIVATE/original.json" "http://$proxy_host:$port" > "$PRIVATE/route.json" || fail 'cannot prepare scoped writer endpoint'
ROUTE_CHANGED=1
docker "${compose[@]}" -f "$PRIVATE/route.json" up -d --no-deps --force-recreate "$TARGET" >/dev/null || fail 'cannot route isolated writer'
sc_clear_restore "$TARGET" start || fail 'cannot transfer writer route ownership'
deadline=$((SECONDS + 600))
while true; do
  requests="$(docker exec "$TEST_CONTAINER" cat "$REMOTE/requests" 2>/dev/null)" || requests=""
  if jq -e -s --arg path "$OBJECT_PATH" '[.[] | select(.mode=="rejected" and .path==$path and .status==507)] | length>=2' <<<"$requests" >/dev/null 2>&1; then break; fi
  ((SECONDS < deadline)) && hc_time_left || fail 'no repeated write-capacity rejection for the named object'
  sleep 1
done
FAULT_AT="$(jq -r -s --arg path "$OBJECT_PATH" '[.[] | select(.mode=="rejected" and .path==$path)][0].at' <<<"$requests")"
# A recorded HTTP failure is insufficient if the uploader nevertheless marked
# the same handle publishable. Repeat the negative check with the fault active.
for checkpoint in 1 2; do
  state="$(docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor_1 -At -v ON_ERROR_STOP=1 -c "SELECT (SELECT count(*) FROM ciphertexts128 WHERE handle=decode('${handle#0x}','hex'))::text || '|' || EXISTS(SELECT 1 FROM ciphertext_digest WHERE handle=decode('${handle#0x}','hex') AND (ciphertext128 IS NOT NULL OR txn_is_sent))::text")" || fail 'cannot inspect pending upload'
  [[ "$state" == '1|false' ]] || fail "premature publication or lost pending squash: $state"
  [[ "$checkpoint" == 2 ]] || sleep 5
done
[[ "$(sc_state "$TARGET")" == running ]] || fail 'SNS worker did not survive rejected uploads'
control healthy || fail 'cannot release storage capacity'
ack "$(cr_now)" || fail 'cannot acknowledge recovery'
phase verify || fail 'original object did not recover and decrypt'
docker cp "$TEST_CONTAINER:$REMOTE/requests" "$EVIDENCE/requests.log" || fail 'cannot retain write evidence'
jq -e -s --arg path "$OBJECT_PATH" 'any(.[]; .mode=="forwarded" and .path==$path and .status>=200 and .status<300)' "$EVIDENCE/requests.log" >/dev/null || fail 'the original object was not successfully uploaded through the recovered route'
restore_route || fail 'original writer image, command or environment was not restored'
control shutdown || fail 'proxy did not accept shutdown'
wait "$PROXY_PID" || fail 'proxy exited unsuccessfully'
PROXY_PID=""
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$FAULT_AT" recovery_observed_at="$(cr_now)" \
  "workload=$handle" "assert=backpressure=pass:repeated named PUT rejections retained the pending squash without publication, followed by a successful PUT through the same endpoint" \
  "artifact=storage_evidence=$EVIDENCE" "${identity[@]}" || fail 'missing original-work recovery assertions'
