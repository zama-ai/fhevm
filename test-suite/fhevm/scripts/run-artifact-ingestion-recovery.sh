#!/usr/bin/env bash
# Full deployed pipeline. Run on an isolated managed three-of-three-backlog stack.
set -uo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
MODE="${1:-overlap}"
case "$MODE" in overlap|cursor) ;; *) echo 'usage: run-artifact-ingestion-recovery.sh [overlap|cursor]' >&2; exit 2 ;; esac
CASE_ID="INGEST-ACL-${MODE^^}"
source "$SCRIPT_DIR/lib/service-control.sh"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/suite-identity.sh"
source "$SCRIPT_DIR/lib/runner-assertions.sh"
source "$SCRIPT_DIR/lib/suite-process.sh"
source "$SCRIPT_DIR/lib/result-staging.sh"
sc_init || exit 1
sp_init || exit 1
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
STATE_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}"
ENV_DIR="$STATE_DIR/runtime/env"
TARGET=coprocessor1-host-listener-poller
DETECTOR=coprocessor1-consensus-detector
PROXY_PID=""
ROUTE_CHANGED=0
NEEDS_HEAL=0
PRIVATE=""
compose=()
fail() { cr_record "$CASE_ID" FAIL started_at="$STARTED" cleanup=failed detail="$*"; echo "artifact-recovery: $*" >&2; exit 1; }
env_value() { sed -n "s/^$1=//p" "$ENV_DIR/coprocessor.env" | tail -1; }
restore_route() {
  [[ "$ROUTE_CHANGED" == 1 ]] || return 0
  docker "${compose[@]}" up -d --no-deps --force-recreate "$TARGET" >/dev/null || return 1
  docker inspect "$TARGET" > "$PRIVATE/restored.json" || return 1
  jq -e -s '.[0][0].Config.Cmd == .[1][0].Config.Cmd and .[0][0].Image == .[1][0].Image and (.[0][0].Config.Env|sort)==(.[1][0].Config.Env|sort)' "$PRIVATE/original.json" "$PRIVATE/restored.json" >/dev/null || return 1
  ROUTE_CHANGED=0
}
cleanup() {
  local status=$? ok=1
  hc_cleanup_signals
  hc_begin_cleanup || { rs_finalize_results 1 failed; exit 1; }
  # Cancel the workload before changing any service state. Proxy is supervised
  # too, so the original upstream route must be restored before resuming writers.
  sp_cancel_all && sp_recover_suite_state || ok=0
  if [[ "$ok" == 1 ]]; then
    [[ -z "$PROXY_PID" ]] || wait "$PROXY_PID" 2>/dev/null || true
    restore_route || ok=0
  fi
  if [[ "$NEEDS_HEAL" == 1 ]]; then
    # An assertion may fail precisely because the bug is still present. Do not
    # let a resumed history publisher seal that partial inventory during cleanup.
    mkdir -p "$(dirname "$SP_CONTAMINATION")"
    printf 'Artifact recovery incomplete; detector remains stopped. Private recovery ledger: %s\n' "$PRIVATE" > "$SP_CONTAMINATION"
    echo "artifact-recovery: keep $DETECTOR stopped until audited replay succeeds, or recreate this isolated stack. Check route restoration and ledger: $PRIVATE" >&2
    ok=0
  fi
  if [[ "$ok" == 1 ]]; then sc_run_restores || ok=0; fi
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
phase() {
  local phase="$1" output=""
  cr_run_suite output "[artifact-recovery/$CASE_ID/$phase] CASE COMPLETE" sp_exec \
    -e RUN_ARTIFACT_RECOVERY=1 -e "ARTIFACT_CASE_ID=$CASE_ID" -e "ARTIFACT_PHASE=$phase" \
    -e "CONSENSUS_HANDSHAKE_DIR=$REMOTE" -e COPROCESSOR_COUNT=3 -e CONSENSUS_THRESHOLD=3 \
    -e CONSENSUS_WATCHDOG_STALL_MS=2400000 \
    -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL)" -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS)" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS)" \
    "$TEST_CONTAINER" npx hardhat test test/consensus/artifactRecovery.ts --network "${TEST_NETWORK:-staging}" > "$EVIDENCE/$phase.log" 2>&1
  local result=$?
  printf '%s\n' "$output" >> "$EVIDENCE/$phase.log"
  cat "$EVIDENCE/$phase.log"
  return "$result"
}
control() {
  docker exec "$TEST_CONTAINER" node -e 'const fs=require("fs");const [dir,mode]=process.argv.slice(1);const w=JSON.parse(fs.readFileSync(dir+"/artifact-workload.json")).payload;fs.writeFileSync(dir+"/control.new",JSON.stringify({mode,until:Date.now()+3600000,transactionHash:w.transactionHash,address:w.acl}));fs.renameSync(dir+"/control.new",dir+"/control")' "$REMOTE" "$1"
}
observed="$(bun "$SCRIPT_DIR/observe-consensus-topology.ts" 3)" || exit 1
eval "$observed"
export CONSENSUS_SCENARIO CONSENSUS_OPERATORS CONSENSUS_THRESHOLD
cr_init "${CONSENSUS_RUN_ID:-}" || exit 1
rs_stage_results || exit 1
STARTED="$(cr_now)"
[[ "$CONSENSUS_SCENARIO" == three-of-three-backlog ]] || fail 'requires the isolated three-of-three-backlog scenario'
sp_case_start "$CASE_ID" || fail 'cannot establish bounded case deadline'
suite_identity_assert "$TEST_CONTAINER" || fail 'test-suite image must contain this checkout'
identity=()
cr_read_run_identity identity || fail 'missing deployed software identity'
umask 077
mkdir -p "$STATE_DIR/runtime/artifact-recovery-private" "$STATE_DIR/runtime/artifact-recovery-evidence" || fail 'cannot create journal roots'
PRIVATE="$(mktemp -d "$STATE_DIR/runtime/artifact-recovery-private/case.XXXXXX")" || fail 'cannot create recovery journal'
EVIDENCE="$(mktemp -d "$STATE_DIR/runtime/artifact-recovery-evidence/case.XXXXXX")" || fail 'cannot create evidence directory'
SC_RESTORE_LOG="$PRIVATE/restores.log"
touch "$SC_RESTORE_LOG" || fail 'cannot initialize restoration ledger'
for service in "$TARGET" "$DETECTOR" coprocessor1-tfhe-worker coprocessor1-sns-worker coprocessor1-transaction-sender; do
  [[ "$(sc_state "$service")" == running ]] || fail "$service must be running before the fault"
done
docker inspect "$TARGET" > "$PRIVATE/original.json" || fail 'cannot snapshot poller'
upstream="$(bun "$SCRIPT_DIR/route-host-poller.ts" "$PRIVATE/original.json")" || fail 'ambiguous poller route'
project="$(jq -r '.[0].Config.Labels["com.docker.compose.project"]' "$PRIVATE/original.json")"
files="$(jq -r '.[0].Config.Labels["com.docker.compose.project.config_files"]' "$PRIVATE/original.json")"
[[ -n "$project" && "$project" != null && -n "$files" && "$files" != null ]] || fail 'original Compose identity unavailable'
compose=(compose -p "$project")
IFS=',' read -r -a config_files <<<"$files"
for file in "${config_files[@]}"; do [[ -f "$file" ]] || fail 'missing original Compose file'; compose+=(-f "$file"); done
[[ ! -f "$ENV_DIR/versions.env" ]] || compose+=(--env-file "$ENV_DIR/versions.env")
network_mode="$(docker inspect -f '{{.HostConfig.NetworkMode}}' "$TEST_CONTAINER")"
[[ "$network_mode" == container:* ]] || fail 'managed suite network namespace required'
proxy_host="$(docker inspect -f '{{.Name}}' "${network_mode#container:}")"
proxy_host="${proxy_host#/}"
[[ "$proxy_host" =~ ^[a-zA-Z0-9_-]+$ ]] || fail 'proxy network owner unavailable'
token="$(od -An -N16 -tx1 /dev/urandom | tr -d ' \n')"
REMOTE="${CONSENSUS_HANDSHAKE_DIR:-/tmp/consensus-handshake}/artifact-$token"
docker exec "$TEST_CONTAINER" mkdir -p "$REMOTE" || fail 'cannot create case-owned handshake'
# All ingestion routes to the victim must remain stopped except our poller.
listeners="$(docker ps --format '{{.Names}}')" || fail 'cannot enumerate listener routes'
while IFS= read -r service; do
  [[ "$service" =~ ^coprocessor1-host-listener(-[a-zA-Z0-9-]+)?$ ]] || continue
  sc_stop "$service" || fail 'cannot isolate victim ingestion'
done <<<"$listeners"
sc_stop "$DETECTOR" || fail 'cannot hold publication before the partial block exists'
NEEDS_HEAL=1
phase seed || fail 'cannot establish the original producer'
control omit-acl || fail 'cannot arm exact ACL omission'
docker cp "$SCRIPT_DIR/lib/host-rpc-proxy.cjs" "$TEST_CONTAINER:$REMOTE/proxy.cjs" || fail 'cannot install RPC fault proxy'
sp_exec "$TEST_CONTAINER" node "$REMOTE/proxy.cjs" "$upstream" "$token" "$REMOTE/control" "$REMOTE/ready" "$REMOTE/requests" > "$EVIDENCE/proxy.log" 2>&1 &
PROXY_PID=$!
ready=""
for ((attempt=0;attempt<30;attempt++)); do
  ready="$(docker exec "$TEST_CONTAINER" cat "$REMOTE/ready" 2>/dev/null)" || ready=""
  [[ -z "$ready" ]] || break
  kill -0 "$PROXY_PID" 2>/dev/null || fail 'proxy exited before startup'
  sleep 1
done
port="$(jq -r '.port' <<<"$ready")"
[[ "$port" =~ ^[1-9][0-9]*$ ]] || fail 'proxy did not bind'
bun "$SCRIPT_DIR/route-host-poller.ts" "$PRIVATE/original.json" "http://$proxy_host:$port/$token" > "$PRIVATE/route.json" || fail 'cannot construct owned route'
# Bound the live overlap and keep it large enough for the blocked-window check.
# The cold case additionally uses the operator cursor rewind below.
bun "$SCRIPT_DIR/artifact-poller-route.ts" "$PRIVATE/route.json" "$MODE" > "$PRIVATE/route.next" || fail 'cannot bound replay overlap'
mv "$PRIVATE/route.next" "$PRIVATE/route.json"
ROUTE_CHANGED=1
docker "${compose[@]}" -f "$PRIVATE/route.json" up -d --no-deps --force-recreate "$TARGET" >/dev/null || fail 'cannot start routed poller'
sc_clear_restore "$TARGET" start || fail 'cannot transfer poller restore ownership'
phase blocked || fail 'original graph did not block behind the omitted ACL'
docker cp "$TEST_CONTAINER:$REMOTE/requests" "$EVIDENCE/fault-requests.jsonl" || fail 'cannot retain fault evidence'
jq -e -s 'any(.[]; .mode=="omit-acl" and .removed>0 and .status==200)' "$EVIDENCE/fault-requests.jsonl" >/dev/null || fail 'no real poller response lost the named ACL logs'
FAULT_AT="$(cr_now)"
before="$(sc_identity "$TARGET")" || fail 'cannot identify faulted listener'
sc_stop "$TARGET" || fail 'cannot stop listener for replay'
if [[ "$MODE" == cursor ]]; then
  phase rewind || fail 'cannot rewind stopped poller to original producer'
else
  phase overlap || fail 'original producer fell outside the startup overlap'
fi
control healthy || fail 'cannot restore complete RPC logs'
sc_start "$TARGET" || fail 'cannot start listener for replay'
phase verify || fail 'original handles did not recover through the complete stack'
NEEDS_HEAL=0
after="$(sc_identity "$TARGET")" || fail 'cannot identify recovered listener'
[[ "$before" != "$after" ]] || fail 'startup replay must cross a process restart'
# Re-deliver the exact original blocks once more, then compare retained bytes.
sc_stop "$TARGET" || fail 'cannot stop poller for duplicate replay'
phase rewind || fail 'cannot rewind duplicate replay'
sc_start "$TARGET" || fail 'cannot resume duplicate replay'
phase duplicate || fail 'duplicate replay changed output or failed to cross the graph'
docker cp "$TEST_CONTAINER:$REMOTE/requests" "$EVIDENCE/requests.jsonl" || fail 'cannot retain replay evidence'
docker cp "$TEST_CONTAINER:$REMOTE/artifact-workload.json" "$EVIDENCE/workload.json" || fail 'cannot retain workload identity'
docker logs --tail 5000 "$TARGET" > "$EVIDENCE/poller.log" 2>&1 || fail 'cannot retain listener log'
workloads=()
while IFS= read -r handle; do workloads+=("workload=$handle"); done < <(jq -r '.payload.handles[]' "$EVIDENCE/workload.json")
[[ "${#workloads[@]}" == 5 ]] || fail 'missing original handles'
restore_route || fail 'original poller configuration not restored'
control shutdown || fail 'cannot stop proxy'
wait "$PROXY_PID" || fail 'proxy failed'
PROXY_PID=""
sc_run_restores || fail 'cannot restore held services'
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$FAULT_AT" recovery_observed_at="$(cr_now)" \
  "assert=transport=pass:exact ACL logs omitted on the victim route; $MODE recovery observed through the actual poller" \
  "process_before=$TARGET=$before" "process_after=$TARGET=$after" "artifact=recovery_evidence=$EVIDENCE" \
  "${identity[@]}" "${workloads[@]}" || fail 'missing executed assertion evidence'
