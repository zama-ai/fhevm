#!/usr/bin/env bash
# RFC-021 cutover kickoff for preview-env. Runs host-contracts' task:proposeCoprocessorUpgrade
# (the real window-selection tool: samples block times per chain, one window per host chain plus
# the gateway start) as the ACL owner (#9) from a host-contracts pod, then asserts each party's
# upgrade_state / versioning rows. BCS and GCS binaries already run the FSM.
# Env: NAMESPACE, NB_COPROCESSOR, CHAIN_MODE, HOST_HTTP, GATEWAY_HTTP, HOST_CHAIN_ID, TAGS_JSON;
#      with DEPLOY_POLYGON also POLYGON_HTTP, POLYGON_CHAIN_ID.
set -euo pipefail

: "${NAMESPACE:?}"
: "${NB_COPROCESSOR:?}"
: "${HOST_HTTP:?}"
: "${GATEWAY_HTTP:?}"
: "${HOST_CHAIN_ID:?}"
CHAIN_MODE="${CHAIN_MODE:-anvil}"

# Anvil Foundry account #9 (host/ACL owner). External chains overwrite via
# generate-mnemonic.cjs (DEPLOYER_KEY_9 on GITHUB_ENV).
DEPLOYER_KEY_9="${DEPLOYER_KEY_9:-0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6}"
GCS_VERSION="${GCS_VERSION:-v0.15.0}"
# Caller-supplied; contract does not enforce uniqueness. Default the Actions
# run id so a reused namespace can re-propose after a rollback.
PROPOSAL_ID="${PROPOSAL_ID:-${GITHUB_RUN_ID:-1}}"
# Host window = [start + START_LEAD_SECS, + WINDOW_DURATION], where `start` is read inside the pod
# and not here. See the container args below: everything between this script and hardhat's first RPC
# call - scheduling, node provisioning, image pull - is time the window would otherwise spend already
# running, and the tool refuses to broadcast a startBlock that is behind the tip. gwStartBlock is
# pinned to the gateway tip and needs no lead.
#
# What remains inside the pod is hardhat's own boot plus the tool's block sampling, tens of seconds
# at worst, so one lead suits every chain. The projection divides the lead by the chain's block time
# and the buffer check multiplies it straight back out, so what the lead has to cover is wall-clock
# either way: a slower chain does not need a larger one.
START_LEAD_SECS="${START_LEAD_SECS:-60}"
# The window must still be open when the traffic that carries the cutover arrives, and that traffic
# comes from the e2e DAG the workflow only deploys after this script has returned from its
# DryRunStarted wait (preview-env-deploy.yml: propose, e2e, then ASSERT_CUTOVER=true here again).
# A window that closes in between yields no unanimity and no version bump. Length costs nothing -
# the cutover fires on unanimity inside the window, not at its end - so default to outlasting the
# deploy and let the QA cases that need a closed window set their own.
WINDOW_DURATION="${WINDOW_DURATION:-5h}"
# The tool refuses to broadcast when startBlock is closer to the tip than this; the lead is the guard here.
BUFFER="${BUFFER:-0}"
# Anything else the hardhat task accepts, appended verbatim - e.g. --use-internal-proxy-address
# true. Empty by default; the six flags above cover a normal round.
PROPOSE_EXTRA_ARGS="${PROPOSE_EXTRA_ARGS:-}"
TIMEOUT_SECS="${TIMEOUT_SECS:-420}"
# How long the propose pod gets to reach a terminal phase, scheduling included. Doubles as the
# pod's own activeDeadlineSeconds so a hung hardhat is cut off rather than outliving the wait.
POD_WAIT_SECS="${POD_WAIT_SECS:-600}"
# UC writes the binary stack version (v0.15.0). Compose e2e stored v0.15.
# Compare major.minor so either form counts as cutover.
version_major_minor() {
  echo "$1" | sed -E 's/^v?([0-9]+\.[0-9]+).*/v\1/'
}
LIVE_VERSION="$(version_major_minor "${GCS_VERSION}")"
# Versioning bump needs in-window host traffic (unanimity). Default off so a
# deploy without automated tests still proves the proposal path.
ASSERT_CUTOVER="${ASSERT_CUTOVER:-false}"
SKIP_PROPOSE="${SKIP_PROPOSE:-false}"
# PROPOSE_DRY_RUN=true runs the calldata-only task (prints the windows report, broadcasts nothing, skips the DB waits).
PROPOSE_DRY_RUN="${PROPOSE_DRY_RUN:-false}"
host_contracts_tag=$(jq -r '.host_contracts // "latest"' <<<"${TAGS_JSON:-null}")
HOST_CONTRACTS_IMAGE="${HOST_CONTRACTS_IMAGE:-hub.zama.org/ghcr/zama-ai/fhevm/host-contracts:${host_contracts_tag}}"

protocol_config=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" \
  -o jsonpath='{.data.protocol_config\.address}')
if [[ -z "${protocol_config}" ]]; then
  echo "::error::host-sc-addresses is missing protocol_config.address"
  exit 1
fi

expected_chains=1
[[ "${DEPLOY_POLYGON:-false}" == "true" ]] && expected_chains=2

# Map the preview's chains onto the tool's environment registry (tasks/utils/environments.ts).
# Every mode passes its chain list explicitly through `local` + LOCAL_HOST_CHAINS: a preview runs
# exactly the chains it deployed, and `ingest.rs` rejects a proposal whose chain set is not exactly
# equal to `host_chains`. testnets used to inherit `devnet`, which has since grown past Sepolia +
# Amoy (Hoodi 560048, BSC 97), so every proposal carried chains no operator knew and was rejected.
# Fallback block times mirror devnet's for the chains we keep; they only apply if sampling fails.
tool_env_args=()
case "${CHAIN_MODE}" in
  testnets)
    tool_env="local"
    hardhat_network="sepolia"
    : "${POLYGON_HTTP:?}" "${POLYGON_CHAIN_ID:?}"
    local_chains=$(jq -cn --argjson id "${HOST_CHAIN_ID}" --arg url "${HOST_HTTP}" \
      '[{chainId: $id, rpcUrl: $url, fallbackBlockTimeSeconds: 12}]')
    local_chains=$(jq -c --argjson id "${POLYGON_CHAIN_ID}" --arg url "${POLYGON_HTTP}" \
      '. + [{chainId: $id, rpcUrl: $url, fallbackBlockTimeSeconds: 1.5}]' <<<"${local_chains}")
    tool_env_args=(
      --from-literal=LOCAL_HOST_CHAINS="${local_chains}"
      --from-literal=LOCAL_GATEWAY_RPC_URL="${GATEWAY_HTTP}"
    )
    ;;
  anvil|blockchain-dev)
    tool_env="local"
    if [[ "${CHAIN_MODE}" == "anvil" ]]; then hardhat_network="staging"; fallback=1; else hardhat_network="zwsDev"; fallback=5; fi
    local_chains=$(jq -cn --argjson id "${HOST_CHAIN_ID}" --arg url "${HOST_HTTP}" --argjson fb "${fallback}" \
      '[{chainId: $id, rpcUrl: $url, fallbackBlockTimeSeconds: $fb}]')
    if [[ "${DEPLOY_POLYGON:-false}" == "true" ]]; then
      : "${POLYGON_HTTP:?}" "${POLYGON_CHAIN_ID:?}"
      local_chains=$(jq -c --argjson id "${POLYGON_CHAIN_ID}" --arg url "${POLYGON_HTTP}" --argjson fb "${fallback}" \
        '. + [{chainId: $id, rpcUrl: $url, fallbackBlockTimeSeconds: $fb}]' <<<"${local_chains}")
    fi
    tool_env_args=(
      --from-literal=LOCAL_HOST_CHAINS="${local_chains}"
      --from-literal=LOCAL_GATEWAY_RPC_URL="${GATEWAY_HTTP}"
    )
    ;;
  *)
    echo "::error::unknown CHAIN_MODE '${CHAIN_MODE}'"
    exit 1
    ;;
esac

job="preview-propose-upgrade"
cleanup() {
  kubectl delete pod,secret -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null 2>&1 || true
}
trap cleanup EXIT

if [[ "${SKIP_PROPOSE}" == "true" ]]; then
  echo "Skipping on-chain propose (SKIP_PROPOSE=true)"
else
task="task:proposeCoprocessorUpgrade"
[[ "${PROPOSE_DRY_RUN}" != "true" ]] || task="task:buildProposeCoprocessorUpgradeCalldata"
echo "${task} id=${PROPOSAL_ID} version=${GCS_VERSION} environment=${tool_env} network=${hardhat_network} start=+${START_LEAD_SECS}s (read in-pod) duration=${WINDOW_DURATION} buffer=${BUFFER}"

# Secrets and RPC URLs (QuickNode keys) travel as env vars; the window parameters are plain args.
kubectl create secret generic "${job}" -n "${NAMESPACE}" \
  --from-literal=DEPLOYER_PRIVATE_KEY="${DEPLOYER_KEY_9}" \
  --from-literal=PROTOCOL_CONFIG_CONTRACT_ADDRESS="${protocol_config}" \
  --from-literal=RPC_URL="${HOST_HTTP}" \
  --from-literal=CHAIN_ID="${HOST_CHAIN_ID}" \
  "${tool_env_args[@]}" \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl delete pod -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null
kubectl apply -n "${NAMESPACE}" -f - <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: ${job}
spec:
  restartPolicy: Never
  activeDeadlineSeconds: ${POD_WAIT_SECS}
  # Pin onto zws-pool, the same general-purpose nodepool the in-cluster
  # Postgres, Redis and listener releases use.
  #
  # This is not a preference, it is the only way this pod reliably gets a core.
  # It requests one, and every Karpenter nodepool on the cluster is tainted
  # with its own name. Karpenter provisions only from pools whose taint a pod
  # tolerates, so tolerating none means no node is ever created for this one
  # and it is confined to whatever untainted capacity happens to be idle. The
  # sibling preview pods get away with that by requesting nothing at all; this
  # one sits Pending until the wait below times out.
  nodeSelector:
    karpenter.sh/nodepool: zws-pool
  tolerations:
    - key: "karpenter.sh/nodepool"
      operator: "Equal"
      value: "zws-pool"
      effect: "NoSchedule"
  imagePullSecrets:
    - name: registry-credentials
  containers:
    - name: hardhat
      image: ${HOST_CONTRACTS_IMAGE}
      command: ["/bin/bash", "-c"]
      args:
        - |
          set -euo pipefail
          # The clock is read here rather than on the runner. This pod can spend minutes
          # being scheduled onto a node Karpenter has to provision and pulling an image
          # that node has never seen, and on the runner all of that came out of the lead:
          # the window opened while the pod was still Pending, and the tool then refused
          # to broadcast a startBlock already behind the tip. Reading it here leaves only
          # hardhat's boot and the tool's own block sampling in front of the tip read the
          # lead is measured against.
          start_time=\$(node -e 'console.log(new Date(Date.now() + Number(process.argv[1]) * 1000).toISOString().slice(0, 19) + "Z")' ${START_LEAD_SECS})
          echo "window opens \${start_time} (now + ${START_LEAD_SECS}s, read in-pod)"
          npx hardhat --network ${hardhat_network} ${task} \\
            --environment ${tool_env} \\
            --start-time "\${start_time}" \\
            --duration ${WINDOW_DURATION} \\
            --buffer ${BUFFER} \\
            --proposal-id ${PROPOSAL_ID} \\
            --software-version ${GCS_VERSION} ${PROPOSE_EXTRA_ARGS}
      envFrom:
        - secretRef:
            name: ${job}
      resources:
        requests: {cpu: "1", memory: 2Gi}
        limits: {cpu: "2", memory: 4Gi}
EOF

# `kubectl wait --for=jsonpath=...=Succeeded` cannot also be told "or Failed", so a pod that errors
# out in the first minute still sits here until the timeout expires before anyone sees its report.
# Poll both terminal phases instead; a refused proposal now surfaces in seconds.
pod_deadline=$((SECONDS + POD_WAIT_SECS))
while true; do
  phase=$(kubectl get pod -n "${NAMESPACE}" "${job}" -o jsonpath='{.status.phase}' 2>/dev/null || true)
  [[ "${phase}" == "Succeeded" ]] && break
  if [[ "${phase}" == "Failed" ]]; then
    kubectl logs -n "${NAMESPACE}" "${job}" || true
    echo "::error::${task} did not succeed (see report above)"
    exit 1
  fi
  if (( SECONDS >= pod_deadline )); then
    # Pending this long is a scheduling verdict, not slowness; the events carry the reason.
    kubectl describe pod -n "${NAMESPACE}" "${job}" | tail -n 20 || true
    kubectl logs -n "${NAMESPACE}" "${job}" || true
    echo "::error::${task} did not finish within ${POD_WAIT_SECS}s (phase='${phase:-unknown}')"
    exit 1
  fi
  sleep 5
done
kubectl logs -n "${NAMESPACE}" "${job}"
if [[ "${PROPOSE_DRY_RUN}" == "true" ]]; then
  echo "Dry run only (PROPOSE_DRY_RUN=true): nothing broadcast."
  exit 0
fi
fi

psql_party() {
  local party="$1" sql="$2"
  kubectl exec -n "${NAMESPACE}" "postgres-coprocessor-${party}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "${sql}"
}

# After in-window e2e, cutover may already have flipped the row to
# UpgradeAuthorized/LIVE. Skip the DryRunStarted gate when we only want
# the versioning assert.
if [[ "${SKIP_PROPOSE}" != "true" || "${ASSERT_CUTOVER}" != "true" ]]; then
deadline=$((SECONDS + TIMEOUT_SECS))
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  echo "Waiting for party ${i} GCS DryRunStarted on ${expected_chains} host chain(s)..."
  while true; do
    # One GCS row per host chain; every one must be past activation.
    state=$(psql_party "${i}" \
      "SELECT COALESCE(string_agg(host_chain_id || ':' || state, ',' ORDER BY host_chain_id), '') FROM upgrade_state WHERE stack_role='GCS';" \
      || true)
    ready=$(psql_party "${i}" \
      "SELECT count(*) FROM upgrade_state WHERE stack_role='GCS' AND state IN ('DryRunStarted','UpgradeAuthorized','LIVE');" \
      || echo 0)
    if [[ "${ready}" -ge "${expected_chains}" ]]; then
      echo "party ${i}: ${state}"
      break
    fi
    if (( SECONDS >= deadline )); then
      echo "::error::party ${i} did not reach DryRunStarted on all ${expected_chains} chain(s) (last='${state}')"
      # The dry run starts once every chain's consumer has processed start_block; on Amoy the consumer can trail head by hours.
      psql_party "${i}" "SELECT 'chain '||u.host_chain_id||': consumer at '||COALESCE(MAX(c.block_number),0)||', start_block '||u.start_block FROM upgrade_state u LEFT JOIN host_chain_consumer_blocks c ON c.chain_id = u.host_chain_id WHERE u.stack_role='GCS' GROUP BY u.host_chain_id, u.start_block;" || true
      exit 1
    fi
    sleep 5
  done
done
fi

if [[ "${ASSERT_CUTOVER}" != "true" ]]; then
  echo "DryRunStarted on ${NB_COPROCESSOR} operator DB(s). Set ASSERT_CUTOVER=true after in-window traffic to wait for ${LIVE_VERSION}."
  exit 0
fi

deadline=$((SECONDS + TIMEOUT_SECS))
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  echo "Waiting for party ${i} versioning=${LIVE_VERSION}..."
  while true; do
    version=$(psql_party "${i}" "SELECT stack_version FROM versioning;" || true)
    if [[ "$(version_major_minor "${version}")" == "${LIVE_VERSION}" ]]; then
      echo "party ${i}: versioning=${version}"
      break
    fi
    if (( SECONDS >= deadline )); then
      echo "::error::party ${i} versioning='${version}', expected ${LIVE_VERSION} (major.minor)"
      psql_party "${i}" "SELECT stack_role, host_chain_id, state, status, version FROM upgrade_state;" || true
      exit 1
    fi
    sleep 5
  done
done

echo "RFC-021 cutover asserted on ${NB_COPROCESSOR} operator DB(s)."
