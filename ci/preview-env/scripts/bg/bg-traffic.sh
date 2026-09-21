#!/usr/bin/env bash
# Blue/Green QA: organic ERC-20 traffic on one token per host chain for a whole round.
#
#   bg-traffic.sh setup    one long-lived traffic pod per chain (same e2e image, env and contract
#                          patch as the Argo test pods), then deploy + mint the round's token once
#   bg-traffic.sh start    start the transfer/mint/decrypt loop in each pod (background)
#   bg-traffic.sh status   per chain: loop alive?, counters, last error; balances that can starve
#                          a round (alice per chain; relayer, KMS and coprocessor signers on the gateway)
#   bg-traffic.sh verify   decrypt every holder's balance + totalSupply against the tracked values
#   bg-traffic.sh stop     ask every loop to finish its current step and exit; print final counters
#   bg-traffic.sh teardown delete the traffic pods; the round state stays in ConfigMap bg-traffic-state-<chain>
#                          so a later setup reuses the same token (delete that ConfigMap to start a new token)
#
# The loop itself is test-suite/e2e/scripts/erc20-traffic.ts (copied into the pod at setup until
# it ships in the image). Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg/bg-traffic.sh <verb>
# Env: NAMESPACE (required), DEPLOY_POLYGON (default: true when the env has a Polygon host chain),
#      TRAFFIC_INTERVAL_SECS (60), TRAFFIC_DECRYPT_EVERY (2), TRAFFIC_MINT_EVERY (10),
#      TRAFFIC_MAX_TRANSFER (1000), TRAFFIC_MAX_ITERATIONS (0 = until stop),
#      GATEWAY_RPC_URL (optional, for the gateway balances in `status`).
set -euo pipefail

verb="${1:?usage: bg-traffic.sh setup|start|status|verify|burst on|burst off|stop|teardown}"
: "${NAMESPACE:?}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
script_src="${root}/test-suite/e2e/scripts/erc20-traffic.ts"
work=$(mktemp -d)
trap 'rm -rf "${work}"' EXIT
fail() { echo "::error::$*" >&2; exit 1; }

# Run one query against party 1's coprocessor DB, empty on any error.
copro_psql() {
  kubectl exec -n "${NAMESPACE}" postgres-coprocessor-1-0 -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "$1" 2>/dev/null | tr -d '\r' | head -1
}

# Chains are identified by their chain id throughout, so this works on every chain mode. The set
# comes from the environment itself (the same host_chains table bg-checkpoints.sh reads) rather
# than being hardcoded, and CHAINS=<ids> overrides it.
if [[ -n "${CHAINS:-}" ]]; then
  read -r -a chains <<<"${CHAINS}"
else
  # shellcheck disable=SC2207 # word splitting is what we want: psql prints one id per line
  chains=($(kubectl exec -n "${NAMESPACE}" postgres-coprocessor-1-0 -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc \
    "SELECT chain_id FROM host_chains ORDER BY chain_id;" 2>/dev/null | tr -d '\r'))
fi
[[ ${#chains[@]} -gt 0 ]] || fail "no host chains found in ${NAMESPACE} (set CHAINS=<chain id>... to override)"

# Chain id -> the hardhat network name configured in test-suite/e2e/hardhat.config.ts.
hh_network() {
  case "$1" in
    12345) echo staging ;;
    1337) echo zwsDev ;;
    11155111) echo sepolia ;;
    80002) echo polygonAmoy ;;
    *) fail "chain ${1}: no hardhat network known for this chain id" ;;
  esac
}
# Polygon is always the second host chain and carries its own e2e workflow release.
wf_release() { [[ "$1" == "80002" ]] && echo test-suite-workflow-polygon || echo test-suite-workflow; }
pod_name() { echo "bg-traffic-$(hh_network "$1" | tr '[:upper:]' '[:lower:]')"; }

# First block of this chain's upgrade window, 0 when no proposal exists.
window_start_block() {
  local v
  v=$(copro_psql "SELECT COALESCE(start_block,0) FROM upgrade_state
                   WHERE stack_role='GCS' AND host_chain_id=$1 LIMIT 1;")
  echo "${v:-0}"
}

# When the flip completed, empty while the upgrade has not cut over. NOT the window's end_block,
# which is the planned end and sits hours past the cutover.
cutover_at() {
  copro_psql "SELECT to_char(updated_at AT TIME ZONE 'UTC','YYYY-MM-DD\"T\"HH24:MI:SS.MS\"Z\"')
                FROM upgrade_state WHERE state='LIVE' AND status='completed' LIMIT 1;"
}

# Loop parameters forwarded into the pod.
traffic_env=(
  "TRAFFIC_INTERVAL_SECS=${TRAFFIC_INTERVAL_SECS:-60}"
  "TRAFFIC_DECRYPT_EVERY=${TRAFFIC_DECRYPT_EVERY:-2}"
  "TRAFFIC_MINT_EVERY=${TRAFFIC_MINT_EVERY:-10}"
  "TRAFFIC_MAX_TRANSFER=${TRAFFIC_MAX_TRANSFER:-1000}"
  "TRAFFIC_MAX_ITERATIONS=${TRAFFIC_MAX_ITERATIONS:-0}"
  "TRAFFIC_BURST_INTERVAL_SECS=${TRAFFIC_BURST_INTERVAL_SECS:-10}"
)

# Run erc20-traffic.ts in the chain's pod with TRAFFIC_CMD=$2 (+ extra env words).
run_traffic() {
  local chain="$1" tcmd="$2"; shift 2
  # shellcheck disable=SC2016 # single quotes are deliberate: "$0" is expanded by the pod's shell
  kubectl exec -n "${NAMESPACE}" "$(pod_name "${chain}")" -- \
    env "TRAFFIC_CMD=${tcmd}" "${traffic_env[@]}" "$@" \
    sh -c 'cd /app/test-suite/e2e && npx hardhat run --no-compile scripts/erc20-traffic.ts --network "$0"' "$(hh_network "${chain}")"
}

# Normalized pod source for a chain: image, env, resources, workingDir, pod-level scheduling and the
# preamble (the contract-address patch + compile the e2e image needs before any hardhat run).
# Automated envs have the Argo e2e workflow releases; the manual QA mode does not, so fall back to
# the idle test-suite Job, which carries the same env for the ETH host chain.
pod_source() {
  local chain="$1" rel
  local out="${work}/src-${chain}.json"
  rel=$(wf_release "${chain}")
  if helm status "${rel}" -n "${NAMESPACE}" >/dev/null 2>&1; then
    helm get manifest "${rel}" -n "${NAMESPACE}" | yq 'select(.kind == "Workflow")' -o=json > "${work}/wf-${chain}.json"
    jq '(.spec.templates[] | select(.name == "run-test") | .script) as $s
        | {image: $s.image, env: $s.env, resources: $s.resources, workingDir: ($s.workingDir // null),
           serviceAccountName: .spec.serviceAccountName, imagePullSecrets: .spec.imagePullSecrets,
           tolerations: .spec.tolerations, nodeSelector: null, preamble: $s.source}' \
      "${work}/wf-${chain}.json" > "${out}"
  else
    kubectl get job -n "${NAMESPACE}" test-suite -o json > "${work}/job.json" \
      || fail "${chain}: neither the ${rel} release nor the idle test-suite Job exists in ${NAMESPACE}"
    jq '.spec.template.spec as $s | $s.containers[0] as $c
        | {image: $c.image, env: $c.env, resources: $c.resources, workingDir: ($c.workingDir // null),
           serviceAccountName: ($s.serviceAccountName // null), imagePullSecrets: $s.imagePullSecrets,
           tolerations: $s.tolerations, nodeSelector: ($s.nodeSelector // null),
           preamble: (if (($c.args // []) | length) > 0 then $c.args[-1] else $c.command[-1] end)}' \
      "${work}/job.json" > "${out}"
    if [[ "${chain}" == "80002" ]]; then
      # The idle Job is wired to the first host chain: point the RPC, chain id and the HOST-chain
      # contract addresses at Polygon (gateway addresses stay as they are).
      local rpc
      rpc=$(kubectl get secret -n "${NAMESPACE}" rpc -o jsonpath='{.data.polygon-rpc-url}' | base64 -d)
      [[ -n "${rpc}" ]] || fail "chain 80002: the rpc Secret has no polygon-rpc-url"
      jq --arg rpc "${rpc}" '
        .env = (.env | map(
          if .name == "RPC_URL" then {name: .name, value: $rpc}
          elif .name == "CHAIN_ID_HOST" then {name: .name, value: "80002"}
          elif (.name | test("^(ACL|FHEVM_EXECUTOR|HCU_LIMIT|INPUT_VERIFIER|KMS_VERIFIER|PROTOCOL_CONFIG)_CONTRACT_ADDRESS$"))
            then (.valueFrom.configMapKeyRef.name = "polygon-sc-addresses")
          else . end))
        | .env += [{name: "POLYGON_AMOY_RPC_URL", value: $rpc}]' "${out}" > "${out}.tmp" && mv "${out}.tmp" "${out}"
    fi
  fi
  echo "${out}"
}

# Pod = that source with the test run replaced by an idle wait loop (PID 1 reaps finished loops).
render_pod() {
  local chain="$1" out="$2" src
  src=$(pod_source "${chain}")
  jq -r '.preamble' "${src}" | sed -n '1,/^npx hardhat compile/p' > "${work}/preamble-${chain}.sh"
  grep "npx hardhat compile" "${work}/preamble-${chain}.sh" >/dev/null \
    || fail "${chain}: could not extract the contract-patch/compile preamble"
  printf '\necho "traffic pod ready"\nmkdir -p /data/erc20-traffic\nwhile true; do sleep 30; done\n' >> "${work}/preamble-${chain}.sh"
  POD="$(pod_name "${chain}")" NS="${NAMESPACE}" CHAIN="${chain}" PREAMBLE="$(cat "${work}/preamble-${chain}.sh")" \
  jq '{apiVersion: "v1", kind: "Pod",
       metadata: {name: env.POD, namespace: env.NS,
                  labels: {"app.kubernetes.io/name": "bg-traffic", "fhevm.zama.ai/chain": env.CHAIN}},
       spec: ({restartPolicy: "Never",
               volumes: [{name: "data", emptyDir: {}}],
               containers: [({name: "traffic", image: .image, command: ["/bin/bash", "-c", env.PREAMBLE],
                              env: .env, resources: .resources,
                              volumeMounts: [{name: "data", mountPath: "/data"}]}
                             + (if .workingDir then {workingDir: .workingDir} else {} end))]}
              + (if .serviceAccountName then {serviceAccountName: .serviceAccountName} else {} end)
              + (if .imagePullSecrets then {imagePullSecrets: .imagePullSecrets} else {} end)
              + (if .tolerations then {tolerations: .tolerations} else {} end)
              + (if .nodeSelector then {nodeSelector: .nodeSelector} else {} end))}' \
    "${src}" > "${out}"
}

ensure_pod() {
  local chain="$1" pod
  pod=$(pod_name "${chain}")
  if ! kubectl get pod -n "${NAMESPACE}" "${pod}" >/dev/null 2>&1; then
    render_pod "${chain}" "${work}/pod-${chain}.yaml"
    kubectl apply -n "${NAMESPACE}" -f "${work}/pod-${chain}.yaml" >/dev/null
    echo "$(hh_network "${chain}"): created pod ${pod}"
  fi
  kubectl wait --for=condition=Ready "pod/${pod}" -n "${NAMESPACE}" --timeout=600s >/dev/null
  # Compile is part of the preamble; the pod is Ready before it finishes, so wait for the marker.
  for _ in $(seq 1 90); do
    kubectl logs -n "${NAMESPACE}" "${pod}" --tail=5 2>/dev/null | grep "traffic pod ready" >/dev/null && break
    sleep 5
  done
  kubectl logs -n "${NAMESPACE}" "${pod}" --tail=5 2>/dev/null | grep "traffic pod ready" >/dev/null || fail "${chain}: pod did not finish compiling in time"
  kubectl cp "${script_src}" "${NAMESPACE}/${pod}:/app/test-suite/e2e/scripts/erc20-traffic.ts"
  [[ "$(state_json "${chain}")" == "{}" ]] && restore_state "${chain}"
}

state_json() {
  local out
  out=$(kubectl exec -n "${NAMESPACE}" "$(pod_name "$1")" -- cat "/data/erc20-traffic/$(hh_network "$1").json" 2>/dev/null || true)
  [[ -n "${out}" ]] && echo "${out}" || echo '{}'
}
# Alive = the pid exists and is not a zombie (state field of /proc/<pid>/stat; BusyBox ps has no -p).
loop_pid() {
  # shellcheck disable=SC2016 # single quotes are deliberate: $p runs in the pod's shell, not here
  kubectl exec -n "${NAMESPACE}" "$(pod_name "$1")" -- sh -c \
    'p=$(cat /data/erc20-traffic/loop.pid 2>/dev/null) && [ -n "$p" ] && [ -r "/proc/$p/stat" ] && [ "$(awk "{print \$3}" "/proc/$p/stat")" != "Z" ] && echo "$p"' 2>/dev/null || true
}
# The round's state (token address, expected balances, counters) is snapshotted into a ConfigMap so
# a recreated pod keeps using the same token instead of deploying a new one.
snapshot_state() {
  local chain="$1" json
  json=$(state_json "${chain}")
  # Never overwrite a good snapshot with an empty or half-written state.
  jq -e '.contractAddress' <<<"${json}" >/dev/null 2>&1 || return 0
  kubectl create configmap "bg-traffic-state-${chain}" -n "${NAMESPACE}" --from-literal=state.json="${json}" \
    --dry-run=client -o yaml | kubectl apply -n "${NAMESPACE}" -f - >/dev/null
}
restore_state() {
  local chain="$1" json
  json=$(kubectl get configmap "bg-traffic-state-${chain}" -n "${NAMESPACE}" -o json 2>/dev/null | jq -r '.data["state.json"] // empty' || true)
  [[ -n "${json}" ]] || return 0
  kubectl exec -i -n "${NAMESPACE}" "$(pod_name "${chain}")" -- sh -c 'mkdir -p /data/erc20-traffic && cat > /data/erc20-traffic/'"$(hh_network "${chain}")"'.json' <<<"${json}"
  echo "${chain}: restored round state from ConfigMap bg-traffic-state-${chain}"
}

case "${verb}" in
setup)
  for c in "${chains[@]}"; do
    ensure_pod "${c}"
    run_traffic "${c}" setup 2>&1 | grep --line-buffered -E "\[traffic|Error|error" || true
    snapshot_state "${c}"
  done
  ;;
start)
  for c in "${chains[@]}"; do
    [[ -n "$(loop_pid "${c}")" ]] && { echo "$(hh_network "${c}"): loop already running"; continue; }
    net=$(hh_network "${c}")
    kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- \
      env "${traffic_env[@]}" sh -c 'cd /app/test-suite/e2e && rm -f /data/erc20-traffic/'"${net}"'.stop && \
        TRAFFIC_CMD=loop nohup npx hardhat run --no-compile scripts/erc20-traffic.ts --network '"${net}"' \
        >> /data/erc20-traffic/'"${net}"'.log 2>&1 & echo $! > /data/erc20-traffic/loop.pid'
    echo "$(hh_network "${c}"): loop started (pid $(kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- cat /data/erc20-traffic/loop.pid)), log /data/erc20-traffic/${net}.log"
  done
  ;;
status)
  for c in "${chains[@]}"; do
    pid=$(loop_pid "${c}")
    echo "== $(hh_network "${c}"): loop $([[ -n "${pid}" ]] && echo "running (pid ${pid})" || echo "not running")"
    snapshot_state "${c}"
    state_json "${c}" | jq -r 'if .contractAddress then
      "   token \(.contractAddress) owner \(.owner)",
      "   iterations \(.counters.iterations)  transfers \(.counters.transfers)  mints \(.counters.mints)  decrypts \(.counters.decrypts)  mismatches \(.counters.decryptMismatches)  retries \(.counters.retries)  failures \(.counters.failures)",
      "   expected alice=\(.expected.alice) bob=\(.expected.bob) carol=\(.expected.carol) minted=\(.mintedTotal)",
      "   last iteration \(.lastIterationAt // "-")  last error \(.lastError // "-") \(.lastErrorAt // "")"
      else "   no state (run setup)" end'
  done
  echo "== balances"
  # Display only: never let a missing wallet secret fail `status`.
  mnemonic=$(kubectl get secret -n "${NAMESPACE}" preview-wallets-mnemonic \
    -o jsonpath='{.data.mnemonic}' 2>/dev/null | base64 -d 2>/dev/null || true)
  if [[ -z "${mnemonic}" ]]; then
    echo "   (no preview-wallets-mnemonic secret — balances not checked)"
    exit 0
  fi
  alice=$(cast wallet address --mnemonic "${mnemonic}" --mnemonic-index 0)
  for c in "${chains[@]}"; do
    # The pod's own RPC_URL works on every chain mode; the rpc Secret only exists on testnets.
    rpc=$(kubectl get pod -n "${NAMESPACE}" "$(pod_name "${c}")" \
      -o jsonpath='{.spec.containers[0].env[?(@.name=="RPC_URL")].value}' 2>/dev/null || true)
    if [[ -z "${rpc}" ]]; then
      echo "   $(hh_network "${c}"): (no RPC_URL on the traffic pod — balance not checked)"
      continue
    fi
    echo "   $(hh_network "${c}"): alice ${alice} = $(cast balance --ether "${alice}" --rpc-url "${rpc}" 2>/dev/null || echo '?') (pays deploys, mints, transfers)"
  done
  if [[ -n "${GATEWAY_RPC_URL:-}" ]]; then
    relayer=$(cast wallet address --mnemonic "${mnemonic}" --mnemonic-index 3)
    echo "   gateway: relayer ${relayer} = $(cast balance --ether "${relayer}" --rpc-url "${GATEWAY_RPC_URL}" 2>/dev/null || echo '?')"
    for i in $(seq 1 "$(helm list -n "${NAMESPACE}" -o json | jq '[.[] | select(.name | test("^kms-connector-[0-9]+$"))] | length')"); do
      pk=$(kubectl get secret -n "${NAMESPACE}" "$(kubectl get deploy -n "${NAMESPACE}" "kms-connector-${i}-kms-connector-tx-sender" -o json | jq -r '.spec.template.spec.containers[0].env[] | select(.name=="KMS_CONNECTOR_PRIVATE_KEY") | .valueFrom.secretKeyRef.name')" -o jsonpath='{.data.*}' | base64 -d)
      a=$(cast wallet address --private-key "${pk}")
      echo "   gateway: kms tx-sender ${i} ${a} = $(cast balance --ether "${a}" --rpc-url "${GATEWAY_RPC_URL}" 2>/dev/null || echo '?')"
    done
    for i in $(seq 1 "$(helm list -n "${NAMESPACE}" -o json | jq '[.[] | select(.name | test("^coprocessor-[0-9]+$"))] | length')"); do
      pk=$(helm get values "coprocessor-${i}" -n "${NAMESPACE}" -o json | jq -r '.txSender.wallet.secret.value')
      a=$(cast wallet address --private-key "${pk}")
      echo "   gateway: coprocessor ${i} tx-sender ${a} = $(cast balance --ether "${a}" --rpc-url "${GATEWAY_RPC_URL}" 2>/dev/null || echo '?')"
    done
  else
    echo "   (set GATEWAY_RPC_URL to also print the relayer, KMS and coprocessor signer balances on the gateway)"
  fi
  ;;
verify)
  # Current balances, plus - once a proposal exists - every handle the round wrote, with the ones
  # inside the upgrade window flagged. Before the proposal there is no window and this is the
  # quick snapshot check.
  rc=0
  for c in "${chains[@]}"; do
    run_traffic "${c}" verify \
      "TRAFFIC_WINDOW_START=$(window_start_block "${c}")" "TRAFFIC_CUTOVER_AT=$(cutover_at)" 2>&1 \
      | grep --line-buffered -E "\[traffic|Error" || rc=1
  done
  [[ "${rc}" == "0" ]] || fail "verify failed on at least one chain"
  ;;
burst)
  # More writes per minute while the upgrade window is open, so several balances land inside it.
  # The loop re-reads the flag every iteration, so this works on a running loop.
  case "${2:-on}" in
  on)
    for c in "${chains[@]}"; do
      kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- \
        touch "/data/erc20-traffic/$(hh_network "${c}").burst"
      echo "== $(hh_network "${c}"): burst on (${TRAFFIC_BURST_INTERVAL_SECS:-10}s between steps)"
    done
    ;;
  off)
    for c in "${chains[@]}"; do
      kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- \
        rm -f "/data/erc20-traffic/$(hh_network "${c}").burst"
      echo "== $(hh_network "${c}"): burst off"
    done
    ;;
  *) fail "usage: bg-traffic.sh burst on|off" ;;
  esac
  ;;
stop)
  for c in "${chains[@]}"; do
    net=$(hh_network "${c}")
    kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- touch "/data/erc20-traffic/${net}.stop"
    for _ in $(seq 1 120); do
      [[ -z "$(loop_pid "${c}")" ]] && break
      sleep 5
    done
    [[ -z "$(loop_pid "${c}")" ]] || fail "$(hh_network "${c}"): loop still running after 10 min"
    echo "== $(hh_network "${c}"): loop stopped"
    snapshot_state "${c}"
    state_json "${c}" | jq -r '"   iterations \(.counters.iterations)  transfers \(.counters.transfers)  mints \(.counters.mints)  decrypts \(.counters.decrypts)  mismatches \(.counters.decryptMismatches)  retries \(.counters.retries)  failures \(.counters.failures)"'
  done
  ;;
teardown)
  for c in "${chains[@]}"; do
    [[ -n "$(loop_pid "${c}")" ]] && fail "$(hh_network "${c}"): loop still running; stop first"
    kubectl delete pod -n "${NAMESPACE}" "$(pod_name "${c}")" --ignore-not-found
    # The snapshot too, or `setup` restores it and reuses a token whose ciphertexts
    # a bg-reset.sh in between has already truncated. Keep it with KEEP_STATE=true.
    [[ "${KEEP_STATE:-false}" == "true" ]] ||
      kubectl delete configmap -n "${NAMESPACE}" "bg-traffic-state-${c}" --ignore-not-found
  done
  ;;
*)
  fail "unknown verb '${verb}'"
  ;;
esac
