#!/usr/bin/env bash
# v0.14 full-environment Artifact recovery on an isolated two-of-two stack.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
[[ "${ARTIFACT_RECOVERY_ISOLATED:-}" == 1 ]] || { echo 'Use only on an isolated two-of-two test deployment; set ARTIFACT_RECOVERY_ISOLATED=1.' >&2; exit 2; }
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
TARGET=coprocessor1-host-listener-poller
STATE_DIR="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}"
umask 077
mkdir -p "$STATE_DIR/runtime/artifact-recovery"
PRIVATE="$(mktemp -d "$STATE_DIR/runtime/artifact-recovery/case.XXXXXX")"
TOKEN="$(od -An -N16 -tx1 /dev/urandom | tr -d ' \n')"
REMOTE="/tmp/artifact-recovery-$TOKEN"
CLONE="artifact-poller-$TOKEN"
DEADLINE=$(( $(date +%s) + 1800 ))
PHASES=()
STOPPED=()
CLONE_CREATED=0
COMPLETE=0
PROXY_PID=""
# Every Docker operation is bounded. The in-container supervisor also kills the
# phase's process group: timing out a docker exec client alone is insufficient.
dk() { timeout --kill-after=2s 45s docker "$@"; }
cleanup() {
  local status=$? ok=1 token
  trap '' INT TERM
  set +e
  for token in "${PHASES[@]}"; do
    dk exec "$TEST_CONTAINER" node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" cancel "$token" || ok=0
  done
  if [[ "$CLONE_CREATED" == 1 ]]; then dk rm -f "$CLONE" >/dev/null || ok=0; fi
  if [[ -n "$PROXY_PID" ]]; then kill "$PROXY_PID" 2>/dev/null; wait "$PROXY_PID" 2>/dev/null; fi
  # Do not restart writers while an old phase could still rewind the cursor.
  if [[ "$ok" == 1 ]]; then
    for service in "${STOPPED[@]}"; do dk start "$service" >/dev/null || ok=0; done
  fi
  if [[ "$ok" != 1 || "$COMPLETE" != 1 ]]; then
    printf 'Recovery run incomplete. Isolated environment may need recreation. Private ledger: %s\n' "$PRIVATE" >&2
    status=1
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
# The ordinary watchdog treats deliberately blocked consensus as a stall. Each
# phase uses its own handle-specific unanimous quorum assertions instead.
phase() {
  local name="$1" token="${TOKEN}_$1" remaining=$((DEADLINE - $(date +%s)))
  ((remaining > 0)) || return 124
  PHASES+=("$token")
  timeout --kill-after=2s "$((remaining + 5))s" docker exec --env-file "$PRIVATE/phase.env" \
    -e GATEWAY_RPC_URL= -e CIPHERTEXT_COMMITS_ADDRESS= \
    -e RUN_ARTIFACT_RECOVERY=1 -e "ARTIFACT_PHASE=$name" -e "ARTIFACT_DIR=$REMOTE" \
    -e "CONSENSUS_RUN_ID=$TOKEN" -e "ARTIFACT_GATEWAY_START=$GATEWAY_START" \
    "$TEST_CONTAINER" node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run "$token" "$((DEADLINE * 1000))" \
    npx hardhat test test/ingestionRecovery/artifactRecovery.ts --network "${TEST_NETWORK:-staging}" \
    2>&1 | tee "$PRIVATE/$name.log"
  rg -q "\[artifact-recovery/INGEST-ACL-CURSOR/$name\] CASE COMPLETE" "$PRIVATE/$name.log"
}
control() {
  dk exec "$TEST_CONTAINER" node -e 'const fs=require("fs");const [d,mode]=process.argv.slice(1);const w=JSON.parse(fs.readFileSync(d+"/artifact-workload.json")).payload;fs.writeFileSync(d+"/control.tmp",JSON.stringify({mode,until:Date.now()+3600000,transactionHash:w.transactionHash,address:w.acl}));fs.renameSync(d+"/control.tmp",d+"/control")' "$REMOTE" "$1"
}
for pair in "victim:$TARGET" 'reference:coprocessor-host-listener-poller' "suite:$TEST_CONTAINER" 'sender0:coprocessor-transaction-sender' 'sender1:coprocessor1-transaction-sender'; do
  dk inspect "${pair#*:}" > "$PRIVATE/${pair%%:*}.json"
done
node "$SCRIPT_DIR/lib/artifact-poller-config.cjs" "$PRIVATE"
# Refuse an old test-suite image. This case must exercise this exact workload.
for file in test/ingestionRecovery/artifactRecovery.ts test/ingestionRecovery/support.ts contracts/consensus/ArtifactRecoveryFixture.sol; do
  dk exec "$TEST_CONTAINER" cat "/app/test-suite/e2e/$file" > "$PRIVATE/source-check"
  cmp "$PRIVATE/source-check" "$REPO_ROOT/test-suite/e2e/$file"
done
for service in coprocessor-tfhe-worker coprocessor1-tfhe-worker coprocessor-sns-worker coprocessor1-sns-worker coprocessor-transaction-sender coprocessor1-transaction-sender; do
  [[ "$(dk inspect -f '{{.State.Running}}' "$service")" == true ]]
done
# Stop every default victim ingestion route before seed; workers keep running.
mapfile -t listeners < <(dk ps --format '{{.Names}}' | rg '^coprocessor1-host-listener($|-)')
[[ " ${listeners[*]} " == *" $TARGET "* ]]
for service in "${listeners[@]}"; do
  STOPPED+=("$service")
  printf '%s\n' "$service" >> "$PRIVATE/stopped-services"
  dk stop -t 5 "$service" >/dev/null
done
dk exec "$TEST_CONTAINER" mkdir -p "$REMOTE"
dk cp "$SCRIPT_DIR/lib/host-rpc-proxy.cjs" "$TEST_CONTAINER:$REMOTE/proxy.cjs"
# Capture Gateway height before seed; subsequent quorum scans are bounded.
GATEWAY_START="$(dk exec --env-file "$PRIVATE/phase.env" "$TEST_CONTAINER" node -e 'const {JsonRpcProvider}=require("ethers");const p=new JsonRpcProvider(process.env.ARTIFACT_GATEWAY_RPC_URL);p.getBlockNumber().then(n=>console.log(n)).finally(()=>p.destroy())')"
[[ "$GATEWAY_START" =~ ^[0-9]+$ ]]
phase seed
control omit-acl
upstream="$(node -p 'JSON.parse(require("fs").readFileSync(process.argv[1])).upstream' "$PRIVATE/config.json")"
PHASES+=("${TOKEN}_proxy")
timeout --kill-after=2s "$((DEADLINE - $(date +%s) + 5))s" docker exec "$TEST_CONTAINER" \
  node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run "${TOKEN}_proxy" "$((DEADLINE * 1000))" \
  node "$REMOTE/proxy.cjs" "$upstream" "$TOKEN" "$REMOTE/control" "$REMOTE/ready" "$REMOTE/requests.jsonl" \
  > "$PRIVATE/proxy.log" 2>&1 &
PROXY_PID=$!
port=""
for ((attempt=0;attempt<30;attempt++)); do
  port="$(dk exec "$TEST_CONTAINER" cat "$REMOTE/ready" 2>/dev/null)" && break
  sleep 1
done
[[ "$port" =~ ^[0-9]+$ ]]
proxy_host="$(node -p 'JSON.parse(require("fs").readFileSync(process.argv[1])).proxyHost' "$PRIVATE/config.json")"
node "$SCRIPT_DIR/lib/artifact-poller-config.cjs" "$PRIVATE" "http://$proxy_host:$port/$TOKEN"
mapfile -d '' -t clone_args < "$PRIVATE/run-args"
CLONE_CREATED=1
dk run -d --name "$CLONE" "${clone_args[@]}" >/dev/null
phase blocked
dk exec "$TEST_CONTAINER" node -e 'const fs=require("fs");const rows=fs.readFileSync(process.argv[1],"utf8").trim().split("\n").map(JSON.parse);if(!rows.some(r=>r.mode==="omit-acl"&&r.removed>0&&r.status===200))process.exit(1)' "$REMOTE/requests.jsonl"
dk stop -t 5 "$CLONE" >/dev/null
phase rewind
control healthy
dk start "$CLONE" >/dev/null
phase verify
dk stop -t 5 "$CLONE" >/dev/null
phase rewind
dk start "$CLONE" >/dev/null
phase duplicate
dk cp "$TEST_CONTAINER:$REMOTE/artifact-workload.json" "$PRIVATE/workload.json"
dk cp "$TEST_CONTAINER:$REMOTE/requests.jsonl" "$PRIVATE/requests.jsonl"
COMPLETE=1
echo "Artifact recovery passed (two operators, Gateway, KMS, duplicate replay). Private logs: $PRIVATE"
