#!/usr/bin/env bash
# The proxy only intercepts one isolated operator's HTTP log polling route.
set -uo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
if [[ "$#" == 0 ]]; then
  failures=0
  for mode in 429 503 408 reset stall tls-trust; do bash "$0" "$mode" || failures=$((failures + 1)); done
  exit "$((failures > 0))"
fi
MODE="$1"
case "$MODE" in 429|503|408|reset|stall|tls-trust) ;; *) exit 2 ;; esac
CASE_ID="FM-HOST-RPC-${MODE^^}"
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
TARGET=coprocessor1-host-listener-poller
PROXY_PID=""
ROUTE_CHANGED=0
compose=()
PRIVATE=""
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
  if [[ "$ROUTE_CHANGED" == 1 ]]; then
    docker logs --tail 5000 "$TARGET" > "$EVIDENCE/poller.log" 2>&1 || true
  fi
  # Restore the poller before terminating the proxy it currently depends on.
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
  docker exec "$TEST_CONTAINER" node -e 'const fs=require("fs");const [file,mode]=process.argv.slice(1);fs.writeFileSync(file+".new",JSON.stringify({mode,until:Date.now()+1200000}));fs.renameSync(file+".new",file)' "$REMOTE/control" "$1"
}
ack() {
  docker exec -i "$TEST_CONTAINER" sh -c 'cat > "$1"' sh "$HANDSHAKE_DIR/failure-fault.json" <<JSON
{"name":"failure-fault","ready":true,"payload":{"caseId":"$CASE_ID","applied":true,"faultObservedAt":"$FAULT_AT","recoveryObservedAt":"${1:-}","detail":"isolated RPC fault observed on the selected poller"}}
JSON
}
phase() {
  local phase="$1" output=""
  cr_run_suite output "[failure-case/$CASE_ID/$phase] CASE COMPLETE" sp_exec \
    -e RUN_FAILURE_CASE=1 -e "FAILURE_CASE_ID=$CASE_ID" -e FAILURE_WORKLOAD=ingestion-backlog -e "FAILURE_PHASE=$phase" \
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
umask 077
mkdir -p "$STATE_DIR/runtime/host-rpc-private" "$STATE_DIR/runtime/host-rpc-evidence" || fail 'cannot create journal root'
PRIVATE="$(mktemp -d "$STATE_DIR/runtime/host-rpc-private/case.XXXXXX")" || fail 'cannot create private recovery directory'
SC_RESTORE_LOG="$PRIVATE/restores.log"
touch "$SC_RESTORE_LOG" || fail 'cannot initialize private restoration ledger'
EVIDENCE="$(mktemp -d "$STATE_DIR/runtime/host-rpc-evidence/case.XXXXXX")" || fail 'cannot create public evidence directory'
docker inspect "$TARGET" > "$PRIVATE/original.json" || fail 'poller absent'
args="$(jq -r '.[0].Config.Cmd[]' "$PRIVATE/original.json")"
grep -qx -- '--batch-size=4' <<<"$args" || fail 'poller page bound is not four'
worker_args="$(docker inspect -f '{{range .Config.Cmd}}{{println .}}{{end}}' coprocessor1-tfhe-worker)" || fail 'worker absent'
grep -qx -- '--work-items-batch-size=4' <<<"$worker_args" || fail 'worker acquisition bound is not four'
upstream="$(bun "$SCRIPT_DIR/route-host-poller.ts" "$PRIVATE/original.json")" || fail 'ambiguous original RPC route'
project="$(jq -r '.[0].Config.Labels["com.docker.compose.project"]' "$PRIVATE/original.json")"
files="$(jq -r '.[0].Config.Labels["com.docker.compose.project.config_files"]' "$PRIVATE/original.json")"
[[ -n "$project" && "$project" != null && -n "$files" && "$files" != null ]] || fail 'original compose identity unavailable'
compose=(compose -p "$project")
IFS=',' read -r -a config_files <<<"$files"
for file in "${config_files[@]}"; do [[ -f "$file" ]] || fail 'original compose file missing'; compose+=(-f "$file"); done
[[ ! -f "$ENV_DIR/versions.env" ]] || compose+=(--env-file "$ENV_DIR/versions.env")
network_mode="$(docker inspect -f '{{.HostConfig.NetworkMode}}' "$TEST_CONTAINER")"
[[ "$network_mode" == container:* ]] || fail 'proxy requires the managed suite network namespace'
proxy_host="$(docker inspect -f '{{.Name}}' "${network_mode#container:}")"
proxy_host="${proxy_host#/}"
[[ "$proxy_host" =~ ^[a-zA-Z0-9_-]+$ ]] || fail 'proxy network owner unavailable'
token="$(od -An -N16 -tx1 /dev/urandom | tr -d ' \n')"
REMOTE="$HANDSHAKE_DIR/rpc-$token"
docker exec "$TEST_CONTAINER" mkdir -p "$REMOTE" || fail 'cannot create isolated proxy directory'
docker cp "$SCRIPT_DIR/lib/host-rpc-proxy.cjs" "$TEST_CONTAINER:$REMOTE/proxy.cjs" || fail 'cannot install proxy'
tls_args=()
protocol=http
if [[ "$MODE" == tls-trust ]]; then
  protocol=https
  mkdir "$PRIVATE/tls"
  openssl req -x509 -newkey rsa:2048 -nodes -days 2 -subj /CN=isolated-host-rpc-ca -keyout "$PRIVATE/tls/ca.key" -out "$PRIVATE/tls/ca.crt" >/dev/null 2>&1 || fail 'cannot create isolated CA'
  openssl req -newkey rsa:2048 -nodes -subj "/CN=$proxy_host" -keyout "$PRIVATE/tls/server.key" -out "$PRIVATE/tls/server.csr" >/dev/null 2>&1 || fail 'cannot create HTTPS key'
  printf 'subjectAltName=DNS:%s\nextendedKeyUsage=serverAuth\nbasicConstraints=CA:FALSE\n' "$proxy_host" > "$PRIVATE/tls/extensions"
  openssl x509 -req -in "$PRIVATE/tls/server.csr" -CA "$PRIVATE/tls/ca.crt" -CAkey "$PRIVATE/tls/ca.key" -CAcreateserial -days 2 -extfile "$PRIVATE/tls/extensions" -out "$PRIVATE/tls/server.crt" >/dev/null 2>&1 || fail 'cannot sign HTTPS endpoint'
  openssl req -x509 -newkey rsa:2048 -nodes -days 2 -subj "/CN=$proxy_host" -addext "subjectAltName=DNS:$proxy_host" -addext 'basicConstraints=critical,CA:FALSE' -addext 'extendedKeyUsage=serverAuth' -keyout "$PRIVATE/tls/bad.key" -out "$PRIVATE/tls/bad.crt" >/dev/null 2>&1 || fail 'cannot create untrusted certificate'
  # The worker sees only the public CA, never a private key.
  chmod 644 "$PRIVATE/tls/ca.crt"
  docker cp "$PRIVATE/tls" "$TEST_CONTAINER:$REMOTE/tls" || fail 'cannot install TLS endpoint keys'
  # docker cp creates root-owned files. Keep the private modes, but let the
  # unprivileged proxy user read its isolated endpoint keys.
  proxy_uid="$(docker exec "$TEST_CONTAINER" id -u)" || fail 'cannot identify proxy user'
  proxy_gid="$(docker exec "$TEST_CONTAINER" id -g)" || fail 'cannot identify proxy group'
  docker exec -u 0 "$TEST_CONTAINER" chown -R "$proxy_uid:$proxy_gid" "$REMOTE/tls" || fail 'cannot assign TLS endpoint keys'
  tls_args=("$REMOTE/tls")
fi
control healthy || fail 'cannot initialize proxy control'
sp_exec "$TEST_CONTAINER" node "$REMOTE/proxy.cjs" "$upstream" "$token" "$REMOTE/control" "$REMOTE/ready" "$REMOTE/requests" "${tls_args[@]}" > "$EVIDENCE/proxy.log" 2>&1 &
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
bun "$SCRIPT_DIR/route-host-poller.ts" "$PRIVATE/original.json" "$protocol://$proxy_host:$port/$token" > "$PRIVATE/route.json" || fail 'cannot prepare scoped route'
if [[ "$MODE" == tls-trust ]]; then
  jq --arg ca "$PRIVATE/tls/ca.crt" '.services["coprocessor1-host-listener-poller"] += {environment:{SSL_CERT_FILE:"/tmp/consensus-host-ca.crt"},volumes:[{type:"bind",source:$ca,target:"/tmp/consensus-host-ca.crt",read_only:true}]}' "$PRIVATE/route.json" > "$PRIVATE/route.next"
  mv "$PRIVATE/route.next" "$PRIVATE/route.json"
fi
listeners="$(docker ps --format '{{.Names}}')" || fail 'cannot enumerate redundant listeners'
while IFS= read -r service; do
  [[ "$service" =~ ^coprocessor1-host-listener(-[a-zA-Z0-9-]+)?$ ]] || continue
  sc_stop "$service" || fail 'cannot isolate listener routes'
done <<<"$listeners"
ROUTE_CHANGED=1
docker "${compose[@]}" -f "$PRIVATE/route.json" up -d --no-deps --force-recreate "$TARGET" >/dev/null || fail 'cannot route isolated poller'
# Discharge only this poller's stop entry; alternate ingestion stays stopped.
sc_clear_restore "$TARGET" start || fail 'cannot transfer poller route ownership'
if [[ "$MODE" == tls-trust ]]; then
  deadline=$((SECONDS + 90))
  until docker exec "$TEST_CONTAINER" cat "$REMOTE/requests" 2>/dev/null | jq -e -s 'any(.[]; .mode=="healthy" and .method=="eth_getLogs" and .status==200)' >/dev/null; do
    ((SECONDS < deadline)) && hc_time_left || fail 'valid HTTPS was never accepted by the actual poller'
    sleep 1
  done
fi
control "$MODE" || fail 'cannot arm transport fault'
FAULT_AT="$(cr_now)"
deadline=$((SECONDS + 90))
until docker exec "$TEST_CONTAINER" cat "$REMOTE/requests" 2>/dev/null | jq -e -s --arg mode "$MODE" 'any(.[]; .mode==$mode and (.method=="eth_getLogs" or $mode=="tls-trust"))' >/dev/null; do
  ((SECONDS < deadline)) && hc_time_left || fail 'no actual poller request reached the proxy'
  sleep 1
done
if [[ "$MODE" == tls-trust ]]; then
  FAULT_AT="$(docker exec "$TEST_CONTAINER" cat "$REMOTE/requests" | jq -r -s '[.[]|select(.mode=="tls-trust")][0].at')" || fail 'missing observed TLS rejection time'
fi
ack || fail 'cannot acknowledge observed routing'
phase arm || fail 'backlog did not arm behind the RPC fault'
# Require a failed log request after all target receipts, not just at startup.
armed_at="$(cr_now)"
deadline=$((SECONDS + 180))
requests=""
for (( ; SECONDS < deadline; )); do
  requests="$(docker exec "$TEST_CONTAINER" cat "$REMOTE/requests")" || fail 'proxy evidence unavailable'
  if jq -e -s --arg mode "$MODE" --arg at "$armed_at" 'any(.[]; .mode==$mode and (.method=="eth_getLogs" or $mode=="tls-trust") and .at >= $at)' <<<"$requests" >/dev/null; then break; fi
  hc_time_left || fail 'case deadline expired'
  sleep 1
done
jq -e -s --arg mode "$MODE" --arg at "$armed_at" 'any(.[]; .mode==$mode and (.method=="eth_getLogs" or $mode=="tls-trust") and .at >= $at)' <<<"$requests" >/dev/null || fail 'no request failure while the named backlog was outstanding'
if [[ "$MODE" == stall ]]; then
  # An accepted connection without a reply is not yet evidence of a client
  # timeout. Observe the close and a later retry while still withholding bytes.
  deadline=$((SECONDS + 180))
  while true; do
    requests="$(docker exec "$TEST_CONTAINER" cat "$REMOTE/requests")" || fail 'timeout evidence unavailable'
    # Evaluate the pair against the complete request log, not just the close.
    if jq -e -s --arg at "$armed_at" '. as $events | any(.[]; .mode=="stall-aborted" and .at >= $at and .elapsedMs>=1000 and (.at as $closed | any($events[]; .mode=="stall" and .at > $closed)))' <<<"$requests" >/dev/null; then break; fi
    ((SECONDS < deadline)) && hc_time_left || fail 'no client cancellation and subsequent retry while responses were withheld'
    sleep 1
  done
fi
if [[ "$MODE" == tls-trust ]]; then
  jq -e -s --arg at "$FAULT_AT" 'all(.[]; .at < $at or .mode!="healthy")' <<<"$requests" >/dev/null || fail 'poller accepted an untrusted endpoint'
fi
docker inspect "$TARGET" | jq '.[0] | {id: .Id, status: .State.Status, restarts: .RestartCount}' > "$EVIDENCE/poller-before-recovery.json" || fail 'cannot observe faulted poller'
if [[ "$MODE" == tls-trust ]]; then
  # Certificate failures exhaust the poller's bounded RPC budget and exit for
  # its supervisor. Recovery must work without a test-initiated restart, but
  # catching Docker in its normal restart backoff is not itself a failure.
  jq -e '.status == "running" or .status == "restarting"' "$EVIDENCE/poller-before-recovery.json" >/dev/null || fail 'poller exhausted supervisor recovery before trust was restored'
else
  [[ "$(sc_state "$TARGET")" == running ]] || fail 'poller died rather than remaining a retrying route'
fi
control healthy || fail 'cannot restore RPC responses'
ack "$(cr_now)" || fail 'cannot acknowledge recovery'
phase verify || fail 'original paginated backlog did not recover through the restored route'
docker inspect "$TARGET" | jq '.[0] | {id: .Id, status: .State.Status, restarts: .RestartCount}' > "$EVIDENCE/poller-after-recovery.json" || fail 'cannot observe recovered poller'
jq -e -s '.[0].id == .[1].id and .[1].status == "running"' "$EVIDENCE/poller-before-recovery.json" "$EVIDENCE/poller-after-recovery.json" >/dev/null || fail 'poller did not recover automatically in the original routed container'
docker logs --tail 5000 "$TARGET" > "$EVIDENCE/poller.log" 2>&1 || fail 'cannot retain poller recovery log'
docker cp "$TEST_CONTAINER:$REMOTE/requests" "$EVIDENCE/requests.log" || fail 'cannot retain request evidence'
jq -e -s 'any(.[]; .mode=="healthy" and .status==200)' "$EVIDENCE/requests.log" >/dev/null || fail 'no successful log request through the restored route'
json="$(docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/failure-workload.json")" || fail 'workload receipt unavailable'
workloads=()
while IFS= read -r handle; do workloads+=("workload=$handle"); done < <(jq -r '.payload.handles[]' <<<"$json")
[[ "${#workloads[@]}" == 12 ]] || fail 'incomplete named backlog'
restore_route || fail 'original route or image was not restored'
control shutdown || fail 'proxy did not accept shutdown'
wait "$PROXY_PID" || fail 'proxy exited unsuccessfully'
PROXY_PID=""
cr_record_checked_pass "$CASE_ID" started_at="$STARTED" cleanup=ok fault_observed_at="$FAULT_AT" recovery_observed_at="$(cr_now)" \
  "assert=transport=pass:observed $MODE on actual log polling with all twelve targets outstanding, then successful forwarding through the same route" \
  "artifact=rpc_evidence=$EVIDENCE" "${identity[@]}" "${workloads[@]}" || fail 'missing backlog assertion evidence'
