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
# it ships in the image). Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg-traffic.sh <verb>
# Env: NAMESPACE (required), DEPLOY_POLYGON (default: true when the Polygon test workflow exists),
#      TRAFFIC_INTERVAL_SECS (60), TRAFFIC_DECRYPT_EVERY (2), TRAFFIC_MINT_EVERY (10),
#      TRAFFIC_MAX_TRANSFER (1000), TRAFFIC_MAX_ITERATIONS (0 = until stop),
#      GATEWAY_RPC_URL (optional, for the gateway balances in `status`).
set -euo pipefail

verb="${1:?usage: bg-traffic.sh setup|start|status|verify|stop|teardown}"
: "${NAMESPACE:?}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
script_src="${root}/test-suite/e2e/scripts/erc20-traffic.ts"
work=$(mktemp -d)
trap 'rm -rf "${work}"' EXIT
fail() { echo "::error::$*" >&2; exit 1; }

if [[ -z "${DEPLOY_POLYGON:-}" ]]; then
  DEPLOY_POLYGON=false
  helm status test-suite-workflow-polygon -n "${NAMESPACE}" >/dev/null 2>&1 && DEPLOY_POLYGON=true
fi
# chain key -> hardhat network / Argo workflow release carrying the per-chain env
chains=(sepolia)
[[ "${DEPLOY_POLYGON}" == "true" ]] && chains+=(amoy)
hh_network() { case "$1" in sepolia) echo sepolia ;; amoy) echo polygonAmoy ;; esac; }
wf_release() { case "$1" in sepolia) echo test-suite-workflow ;; amoy) echo test-suite-workflow-polygon ;; esac; }
pod_name() { echo "bg-traffic-$1"; }

# Loop parameters forwarded into the pod.
traffic_env=(
  "TRAFFIC_INTERVAL_SECS=${TRAFFIC_INTERVAL_SECS:-60}"
  "TRAFFIC_DECRYPT_EVERY=${TRAFFIC_DECRYPT_EVERY:-2}"
  "TRAFFIC_MINT_EVERY=${TRAFFIC_MINT_EVERY:-10}"
  "TRAFFIC_MAX_TRANSFER=${TRAFFIC_MAX_TRANSFER:-1000}"
  "TRAFFIC_MAX_ITERATIONS=${TRAFFIC_MAX_ITERATIONS:-0}"
)

# Run erc20-traffic.ts in the chain's pod with TRAFFIC_CMD=$2 (+ extra env words).
run_traffic() {
  local chain="$1" tcmd="$2"; shift 2
  kubectl exec -n "${NAMESPACE}" "$(pod_name "${chain}")" -- \
    env "TRAFFIC_CMD=${tcmd}" "${traffic_env[@]}" "$@" \
    sh -c 'cd /app/test-suite/e2e && npx hardhat run --no-compile scripts/erc20-traffic.ts --network "$0"' "$(hh_network "${chain}")"
}

# Pod spec = the Argo run-test template of that chain (image, env, resources, the contract-address
# patch + compile preamble) with the test command replaced by an idle `sleep infinity`.
render_pod() {
  local chain="$1" out="$2"
  local manifest="${work}/wf-${chain}.yaml"
  helm get manifest "$(wf_release "${chain}")" -n "${NAMESPACE}" > "${manifest}.all" \
    || fail "${chain}: release $(wf_release "${chain}") not found; the e2e workflow must have been deployed once"
  # Keep only the Workflow document: a select() inside the object builder below would still emit
  # an empty Pod for every other document of the multi-doc manifest.
  yq 'select(.kind == "Workflow")' "${manifest}.all" > "${manifest}"
  # Preamble: everything before the test run (patches E2ECoprocessorConfigLocal.sol, compiles).
  yq '.spec.templates[] | select(.name == "run-test") | .script.source' "${manifest}" \
    | sed -n '1,/^npx hardhat compile/p' > "${work}/preamble-${chain}.sh"
  grep -q "npx hardhat compile" "${work}/preamble-${chain}.sh" || fail "${chain}: could not extract the compile preamble from the workflow"
  # Idle as PID 1 in a wait loop so finished background loops are reaped instead of left as zombies.
  printf '\necho "traffic pod ready"\nmkdir -p /data/erc20-traffic\nwhile true; do sleep 30; done\n' >> "${work}/preamble-${chain}.sh"
  POD="$(pod_name "${chain}")" NS="${NAMESPACE}" CHAIN="${chain}" PREAMBLE="$(cat "${work}/preamble-${chain}.sh")" \
  yq '. as $wf
    | ($wf.spec.templates[] | select(.name == "run-test") | .script) as $s
    | {
        "apiVersion": "v1", "kind": "Pod",
        "metadata": {"name": strenv(POD), "namespace": strenv(NS),
                     "labels": {"app.kubernetes.io/name": "bg-traffic", "fhevm.zama.ai/chain": strenv(CHAIN)}},
        "spec": {
          "restartPolicy": "Never",
          "serviceAccountName": $wf.spec.serviceAccountName,
          "imagePullSecrets": $wf.spec.imagePullSecrets,
          "tolerations": $wf.spec.tolerations,
          "volumes": [{"name": "data", "emptyDir": {}}],
          "containers": [{
            "name": "traffic", "image": $s.image, "command": ["/bin/bash", "-c", strenv(PREAMBLE)],
            "env": $s.env, "resources": $s.resources,
            "volumeMounts": [{"name": "data", "mountPath": "/data"}]
          }]
        }
      }' "${manifest}" > "${out}"
}

ensure_pod() {
  local chain="$1" pod
  pod=$(pod_name "${chain}")
  if ! kubectl get pod -n "${NAMESPACE}" "${pod}" >/dev/null 2>&1; then
    render_pod "${chain}" "${work}/pod-${chain}.yaml"
    kubectl apply -n "${NAMESPACE}" -f "${work}/pod-${chain}.yaml" >/dev/null
    echo "${chain}: created pod ${pod}"
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
    run_traffic "${c}" setup 2>&1 | grep -E "\[traffic|Error|error" || true
    snapshot_state "${c}"
  done
  ;;
start)
  for c in "${chains[@]}"; do
    [[ -n "$(loop_pid "${c}")" ]] && { echo "${c}: loop already running"; continue; }
    net=$(hh_network "${c}")
    kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- \
      env "${traffic_env[@]}" sh -c 'cd /app/test-suite/e2e && rm -f /data/erc20-traffic/'"${net}"'.stop && \
        TRAFFIC_CMD=loop nohup npx hardhat run --no-compile scripts/erc20-traffic.ts --network '"${net}"' \
        >> /data/erc20-traffic/'"${net}"'.log 2>&1 & echo $! > /data/erc20-traffic/loop.pid'
    echo "${c}: loop started (pid $(kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- cat /data/erc20-traffic/loop.pid)), log /data/erc20-traffic/${net}.log"
  done
  ;;
status)
  for c in "${chains[@]}"; do
    pid=$(loop_pid "${c}")
    echo "== ${c}: loop $([[ -n "${pid}" ]] && echo "running (pid ${pid})" || echo "not running")"
    snapshot_state "${c}"
    state_json "${c}" | jq -r 'if .contractAddress then
      "   token \(.contractAddress) owner \(.owner)",
      "   iterations \(.counters.iterations)  transfers \(.counters.transfers)  mints \(.counters.mints)  decrypts \(.counters.decrypts)  mismatches \(.counters.decryptMismatches)  retries \(.counters.retries)  failures \(.counters.failures)",
      "   expected alice=\(.expected.alice) bob=\(.expected.bob) carol=\(.expected.carol) minted=\(.mintedTotal)",
      "   last iteration \(.lastIterationAt // "-")  last error \(.lastError // "-") \(.lastErrorAt // "")"
      else "   no state (run setup)" end'
  done
  echo "== balances"
  mnemonic=$(kubectl get secret -n "${NAMESPACE}" preview-wallets-mnemonic -o jsonpath='{.data.mnemonic}' | base64 -d)
  alice=$(cast wallet address --mnemonic "${mnemonic}" --mnemonic-index 0)
  for c in "${chains[@]}"; do
    key=$([[ "${c}" == amoy ]] && echo polygon-rpc-url || echo ethereum-rpc-url)
    rpc=$(kubectl get secret -n "${NAMESPACE}" rpc -o jsonpath="{.data.${key}}" | base64 -d)
    echo "   ${c}: alice ${alice} = $(cast balance --ether "${alice}" --rpc-url "${rpc}" 2>/dev/null || echo '?') (pays deploys, mints, transfers)"
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
  rc=0
  for c in "${chains[@]}"; do
    run_traffic "${c}" verify 2>&1 | grep -E "\[traffic|Error" || rc=1
  done
  [[ "${rc}" == "0" ]] || fail "verify failed on at least one chain"
  ;;
stop)
  for c in "${chains[@]}"; do
    net=$(hh_network "${c}")
    kubectl exec -n "${NAMESPACE}" "$(pod_name "${c}")" -- touch "/data/erc20-traffic/${net}.stop"
    for _ in $(seq 1 120); do
      [[ -z "$(loop_pid "${c}")" ]] && break
      sleep 5
    done
    [[ -z "$(loop_pid "${c}")" ]] || fail "${c}: loop still running after 10 min"
    echo "== ${c}: loop stopped"
    snapshot_state "${c}"
    state_json "${c}" | jq -r '"   iterations \(.counters.iterations)  transfers \(.counters.transfers)  mints \(.counters.mints)  decrypts \(.counters.decrypts)  mismatches \(.counters.decryptMismatches)  retries \(.counters.retries)  failures \(.counters.failures)"'
  done
  ;;
teardown)
  for c in "${chains[@]}"; do
    [[ -n "$(loop_pid "${c}")" ]] && fail "${c}: loop still running; stop first"
    kubectl delete pod -n "${NAMESPACE}" "$(pod_name "${c}")" --ignore-not-found
  done
  ;;
*)
  fail "unknown verb '${verb}'"
  ;;
esac
