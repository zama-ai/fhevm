#!/usr/bin/env bash
# Same test suite and legacy queue fields with either old or fixed sender images.
set -uo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
if [[ "$#" == 0 ]]; then
  failures=0
  for mode in nonce-low accepted-timeout; do bash "$0" "$mode" || failures=$((failures + 1)); done
  exit "$((failures > 0))"
fi
MODE="$1"
case "$MODE" in nonce-low|accepted-timeout) ;; *) echo 'expected nonce-low or accepted-timeout' >&2; exit 2 ;; esac
CASE_ID="FM-SENDER-RPC-${MODE^^}"
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
TARGET=coprocessor1-transaction-sender
PROXY_PID=""; ROUTE_CHANGED=0; PRIVATE=""; REMOTE=""; HANDLES='[]'
compose=()
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
  if [[ -n "${EVIDENCE:-}" && "$ROUTE_CHANGED" == 1 ]]; then
    docker logs --tail 5000 "$TARGET" > "$EVIDENCE/sender.log" 2>&1 || true
    docker cp "$TEST_CONTAINER:$REMOTE/events" "$EVIDENCE/requests.jsonl" >/dev/null 2>&1 || true
  fi
  sp_cancel_all && sp_recover_suite_state || ok=0
  restore_route || ok=0
  [[ -z "$PROXY_PID" ]] || wait "$PROXY_PID" 2>/dev/null || true
  sc_run_restores || ok=0
  [[ "$SP_FORCED_STOP" == 0 ]] || status=1
  if [[ "$ok" == 1 ]]; then rs_finalize_results "$status" ok || status=1; sp_dispose || status=1
  else rs_finalize_results 1 failed; status=1; echo "restore the saved sender route in $PRIVATE before reusing this stack" >&2; fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
fail() { cr_record "$CASE_ID" FAIL started_at="$STARTED" cleanup=failed detail="$*"; echo "$*" >&2; exit 1; }
env_value() { sed -n "s/^$1=//p" "$ENV_DIR/coprocessor.env" | tail -1; }
control() {
  docker exec "$TEST_CONTAINER" node -e 'const fs=require("fs");const [file,mode,handles]=process.argv.slice(1);fs.writeFileSync(file+".new",JSON.stringify({mode,handles:JSON.parse(handles),until:Date.now()+1200000}));fs.renameSync(file+".new",file)' "$REMOTE/control" "$1" "$HANDLES"
}
phase() {
  local phase="$1" output=""
  cr_run_suite output "[sender-rpc/$CASE_ID/$phase] CASE COMPLETE" sp_exec \
    -e RUN_SENDER_RPC_RECOVERY=1 -e "SENDER_RPC_MODE=$MODE" -e "SENDER_RPC_PHASE=$phase" \
    -e "SENDER_RPC_EVENTS=$REMOTE/events" -e COPROCESSOR_COUNT=3 -e CONSENSUS_THRESHOLD=3 \
    -e "CONSENSUS_HANDSHAKE_DIR=$HANDSHAKE_DIR" -e CONSENSUS_WATCHDOG_DISABLED=0 -e CONSENSUS_WATCHDOG_STALL_MS=2400000 \
    -e "GATEWAY_RPC_URL=$(env_value GATEWAY_URL)" -e "GATEWAY_CONFIG_ADDRESS=$(env_value GATEWAY_CONFIG_ADDRESS)" \
    -e "CIPHERTEXT_COMMITS_ADDRESS=$(env_value CIPHERTEXT_COMMITS_ADDRESS)" \
    "$TEST_CONTAINER" npx hardhat test test/consensus/senderRpcRecovery.ts --network "${TEST_NETWORK:-staging}" > "$EVIDENCE/$phase.log" 2>&1
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
umask 077
mkdir -p "$STATE_DIR/runtime/sender-rpc-private" "$STATE_DIR/runtime/sender-rpc-evidence" || fail 'cannot create journal root'
PRIVATE="$(mktemp -d "$STATE_DIR/runtime/sender-rpc-private/case.XXXXXX")" || fail 'cannot create private recovery directory'
SC_RESTORE_LOG="$PRIVATE/restores.log"; touch "$SC_RESTORE_LOG" || fail 'cannot initialize restoration ledger'
EVIDENCE="$(mktemp -d "$STATE_DIR/runtime/sender-rpc-evidence/case.XXXXXX")" || fail 'cannot create evidence directory'
[[ "$(sc_state "$TARGET")" == running ]] || fail 'victim sender must initially be running'
docker inspect "$TARGET" > "$PRIVATE/original.json" || fail 'sender absent'
upstream="$(bun "$SCRIPT_DIR/route-sender-rpc.ts" "$PRIVATE/original.json")" || fail 'ambiguous original sender route'
image="$(jq -r '.[0].Image' "$PRIVATE/original.json")"
revision="$(docker image inspect --format '{{index .Config.Labels "org.opencontainers.image.revision"}}' "$image")" || fail 'cannot identify tested sender image'
[[ "$revision" != "<no value>" ]] || revision=""
reference="$(jq -r '.[0].Config.Image' "$PRIVATE/original.json")"
jq -n --arg reference "$reference" --arg image "$image" --arg revision "$revision" '{reference:$reference,image:$image,revision:$revision,retryLimit:3,timeoutSeconds:4}' > "$EVIDENCE/sender-image.json"
project="$(jq -r '.[0].Config.Labels["com.docker.compose.project"]' "$PRIVATE/original.json")"
files="$(jq -r '.[0].Config.Labels["com.docker.compose.project.config_files"]' "$PRIVATE/original.json")"
[[ -n "$project" && "$project" != null && -n "$files" && "$files" != null ]] || fail 'original compose identity unavailable'
compose=(compose -p "$project")
IFS=',' read -r -a config_files <<<"$files"
for file in "${config_files[@]}"; do [[ -f "$file" ]] || fail 'original compose file missing'; compose+=(-f "$file"); done
[[ ! -f "$ENV_DIR/versions.env" ]] || compose+=(--env-file "$ENV_DIR/versions.env")
network_mode="$(docker inspect -f '{{.HostConfig.NetworkMode}}' "$TEST_CONTAINER")"
[[ "$network_mode" == container:* ]] || fail 'proxy requires managed suite network namespace'
proxy_host="$(docker inspect -f '{{.Name}}' "${network_mode#container:}")"; proxy_host="${proxy_host#/}"
[[ "$proxy_host" =~ ^[a-zA-Z0-9_-]+$ ]] || fail 'proxy network owner unavailable'
token="$(od -An -N16 -tx1 /dev/urandom | tr -d ' \n')"
REMOTE="$HANDSHAKE_DIR/sender-rpc-$token"
docker exec "$TEST_CONTAINER" sh -c 'mkdir -p "$1"; : > "$1/events"; rm -f "$2/sender-rpc-workload.json" "$2/sender-rpc-fault.json"' sh "$REMOTE" "$HANDSHAKE_DIR" || fail 'cannot initialize isolated proxy evidence'
sc_stop "$TARGET" || fail 'cannot hold sender before minting named work'
phase arm || fail 'named material did not become ready with the sender stopped'
workload="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/sender-rpc-workload.json")" || fail 'workload receipt absent'
printf '%s\n' "$workload" > "$EVIDENCE/workload.json"
HANDLES="$(jq -c '.payload.targets | map(.handle)' <<<"$workload")"
[[ "$(jq 'length' <<<"$HANDLES")" == 12 ]] || fail 'incomplete named workload'
docker cp "$SCRIPT_DIR/lib/sender-rpc-proxy.cjs" "$TEST_CONTAINER:$REMOTE/proxy.cjs" || fail 'cannot install proxy'
control "$MODE" || fail 'cannot arm fault'
sp_exec "$TEST_CONTAINER" node "$REMOTE/proxy.cjs" "$upstream" "$token" "$REMOTE/control" "$REMOTE/ready" "$REMOTE/events" > "$EVIDENCE/proxy.log" 2>&1 &
PROXY_PID=$!
ready=""
for ((attempt=0;attempt<30;attempt++)); do
  ready="$(docker exec "$TEST_CONTAINER" cat "$REMOTE/ready" 2>/dev/null)" || ready=""
  [[ -z "$ready" ]] || break
  kill -0 "$PROXY_PID" 2>/dev/null || fail 'proxy failed to start'
  sleep 1
done
port="$(jq -r '.port' <<<"$ready")"; [[ "$port" =~ ^[1-9][0-9]*$ ]] || fail 'proxy never became ready'
bun "$SCRIPT_DIR/route-sender-rpc.ts" "$PRIVATE/original.json" "http://$proxy_host:$port/$token" > "$PRIVATE/route.json" || fail 'cannot prepare scoped sender route'
ROUTE_CHANGED=1
docker "${compose[@]}" -f "$PRIVATE/route.json" up -d --no-deps --force-recreate "$TARGET" >/dev/null || fail 'cannot route isolated sender'
sc_clear_restore "$TARGET" start || fail 'cannot transfer sender route ownership'
before="$(docker inspect -f '{{.Id}} {{.RestartCount}}' "$TARGET")" || fail 'cannot observe routed sender'
phase fault || fail 'required faults did not reach every original material submission'
docker cp "$TEST_CONTAINER:$HANDSHAKE_DIR/sender-rpc-fault.json" "$EVIDENCE/queue-after-fault.json" || fail 'missing queue diagnostics'
docker cp "$TEST_CONTAINER:$REMOTE/events" "$EVIDENCE/requests.jsonl" || fail 'missing RPC evidence'
FAULT_AT="$(jq -r -s '[.[]|select(.mode=="nonce-low" or .mode=="accepted-response-dropped")][0].at' "$EVIDENCE/requests.jsonl")"
[[ -n "$FAULT_AT" && "$FAULT_AT" != null ]] || fail 'missing actual fault observation'
control healthy || fail 'cannot restore RPC forwarding'
RECOVERED_AT="$(cr_now)"
phase verify || fail 'original sender backlog did not drain after RPC recovery; inspect verify.log and queue-after-fault.json'
after="$(docker inspect -f '{{.Id}} {{.RestartCount}}' "$TARGET")" || fail 'cannot observe recovered sender'
[[ "$before" == "$after" && "$(sc_state "$TARGET")" == running ]] || fail 'sender restarted instead of recovering in place'
docker cp "$TEST_CONTAINER:$REMOTE/events" "$EVIDENCE/requests.jsonl" || fail 'cannot retain final RPC evidence'
workloads=(); while IFS= read -r handle; do workloads+=("workload=$handle"); done < <(jq -r '.[]' <<<"$HANDLES")
restore_route || fail 'original sender command or image was not restored'
control shutdown || fail 'cannot stop proxy'
wait "$PROXY_PID" || fail 'proxy exited unsuccessfully'; PROXY_PID=""
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$FAULT_AT" recovery_observed_at="$RECOVERED_AT" \
  "artifact=sender_rpc_evidence=$EVIDENCE" "${identity[@]}" "${workloads[@]}" || fail 'missing executed assertions'
