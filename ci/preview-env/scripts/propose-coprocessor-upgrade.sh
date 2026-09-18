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
GCS_VERSION="${GCS_VERSION:-v0.15.1}"
# Caller-supplied; contract does not enforce uniqueness. Default the Actions
# run id so a reused namespace can re-propose after a rollback.
PROPOSAL_ID="${PROPOSAL_ID:-${GITHUB_RUN_ID:-1}}"
# Window = [now + START_LEAD_SECS, + WINDOW_DURATION]. Anvil only mines on txs, so keep the lead
# tiny (tip+5 at the 1s fallback) and the historical 80-block window; continuously mining chains
# get a window that covers the first e2e DAG.
if [[ "${EXTERNAL_CHAINS:-false}" == "true" ]]; then
  START_LEAD_SECS="${START_LEAD_SECS:-60}"
  WINDOW_DURATION="${WINDOW_DURATION:-5h}"
else
  START_LEAD_SECS="${START_LEAD_SECS:-5}"
  WINDOW_DURATION="${WINDOW_DURATION:-80s}"
fi
# The tool refuses to broadcast when startBlock is closer to the tip than this; the lead is the guard here.
BUFFER="${BUFFER:-0}"
# Anything else the hardhat task accepts, appended verbatim - e.g. --use-internal-proxy-address
# true. Empty by default; the six flags above cover a normal round.
PROPOSE_EXTRA_ARGS="${PROPOSE_EXTRA_ARGS:-}"
TIMEOUT_SECS="${TIMEOUT_SECS:-420}"
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

# Map the preview's chains onto the tool's environment registry (tasks/utils/environments.ts):
# testnets is exactly `devnet` (Sepolia + Amoy); anvil / blockchain-dev use `local` with the chain list passed as JSON.
tool_env_args=()
case "${CHAIN_MODE}" in
  testnets)
    tool_env="devnet"
    hardhat_network="sepolia"
    : "${POLYGON_HTTP:?}"
    tool_env_args=(
      --from-literal=SEPOLIA_RPC_URL="${HOST_HTTP}"
      --from-literal=POLYGON_AMOY_RPC_URL="${POLYGON_HTTP}"
      --from-literal=GATEWAY_DEVNET_RPC_URL="${GATEWAY_HTTP}"
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
start_time=$(python3 -c 'import datetime,sys; print((datetime.datetime.now(datetime.timezone.utc)+datetime.timedelta(seconds=int(sys.argv[1]))).strftime("%Y-%m-%dT%H:%M:%SZ"))' "${START_LEAD_SECS}")
echo "${task} id=${PROPOSAL_ID} version=${GCS_VERSION} environment=${tool_env} network=${hardhat_network} start=${start_time} duration=${WINDOW_DURATION} buffer=${BUFFER}"

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
  activeDeadlineSeconds: 600
  imagePullSecrets:
    - name: registry-credentials
  containers:
    - name: hardhat
      image: ${HOST_CONTRACTS_IMAGE}
      command: ["/bin/bash", "-c"]
      args:
        - >-
          npx hardhat --network ${hardhat_network} ${task}
          --environment ${tool_env}
          --start-time ${start_time}
          --duration ${WINDOW_DURATION}
          --buffer ${BUFFER}
          --proposal-id ${PROPOSAL_ID}
          --software-version ${GCS_VERSION}
          ${PROPOSE_EXTRA_ARGS}
      envFrom:
        - secretRef:
            name: ${job}
      resources:
        requests: {cpu: "1", memory: 2Gi}
        limits: {cpu: "2", memory: 4Gi}
EOF

if ! kubectl wait --for=jsonpath='{.status.phase}'=Succeeded pod/"${job}" -n "${NAMESPACE}" --timeout=600s; then
  kubectl logs -n "${NAMESPACE}" "${job}" || true
  echo "::error::${task} did not succeed (see report above)"
  exit 1
fi
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
