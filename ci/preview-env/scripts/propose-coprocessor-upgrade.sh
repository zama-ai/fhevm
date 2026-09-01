#!/usr/bin/env bash
# RFC-021 cutover kickoff for preview-env. ACL owner (#9) sends one
# ProtocolConfig.proposeCoprocessorUpgrade, then asserts each party's
# upgrade_state / versioning rows. BCS and GCS binaries already run the FSM.
set -euo pipefail

: "${NAMESPACE:?}"
: "${NB_COPROCESSOR:?}"
: "${HOST_HTTP:?}"
: "${GATEWAY_HTTP:?}"
: "${HOST_CHAIN_ID:?}"

# Anvil Foundry account #9 (host/ACL owner). blockchain-dev overwrites via
# generate-mnemonic.cjs (DEPLOYER_KEY_9 on GITHUB_ENV).
DEPLOYER_KEY_9="${DEPLOYER_KEY_9:-0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6}"
GCS_VERSION="${GCS_VERSION:-v0.15.0}"
# Caller-supplied; contract does not enforce uniqueness. Default the Actions
# run id so a reused namespace can re-propose after a rollback.
PROPOSAL_ID="${PROPOSAL_ID:-${GITHUB_RUN_ID:-1}}"
WINDOW_START_OFFSET="${WINDOW_START_OFFSET:-5}"
# Continuously-mining blockchain-dev (~6s/block) needs a window that covers
# the first e2e DAG; Anvil only advances on txs so the historical 80 is enough
# for unanimity to fire during the suite.
if [[ -z "${WINDOW_END_OFFSET:-}" ]]; then
  if [[ "${USE_BLOCKCHAIN_DEV:-false}" == "true" ]]; then
    WINDOW_END_OFFSET=1500
  else
    WINDOW_END_OFFSET=80
  fi
fi
GW_START_OFFSET="${GW_START_OFFSET:-5}"
TIMEOUT_SECS="${TIMEOUT_SECS:-420}"
# Live DB stores major.minor (compose e2e: v0.15 from v0.15.0).
LIVE_VERSION="$(echo "${GCS_VERSION}" | sed -E 's/^v?([0-9]+\.[0-9]+).*/v\1/')"
# Versioning bump needs in-window host traffic (unanimity). Default off so a
# deploy without automated tests still proves the proposal path.
ASSERT_CUTOVER="${ASSERT_CUTOVER:-false}"
SKIP_PROPOSE="${SKIP_PROPOSE:-false}"
FOUNDRY_IMAGE="${FOUNDRY_IMAGE:-hub.zama.org/ghcr/foundry-rs/foundry:stable}"

protocol_config=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" \
  -o jsonpath='{.data.protocol_config\.address}')
if [[ -z "${protocol_config}" ]]; then
  echo "::error::host-sc-addresses is missing protocol_config.address"
  exit 1
fi

rpc_block() {
  local url="$1"
  local raw hex
  raw=$(bash "$(dirname "${BASH_SOURCE[0]}")/cluster-rpc.sh" "${url}" \
    '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}')
  hex=$(jq -r '.result' <<<"${raw}")
  if [[ ! "${hex}" =~ ^0x[0-9a-fA-F]+$ ]]; then
    echo "::error::eth_blockNumber failed on ${url} (raw='${raw}')" >&2
    exit 1
  fi
  echo $((hex))
}

host_block=$(rpc_block "${HOST_HTTP}")
gw_block=$(rpc_block "${GATEWAY_HTTP}")
start=$((host_block + WINDOW_START_OFFSET))
end=$((host_block + WINDOW_END_OFFSET))
gw_start=$((gw_block + GW_START_OFFSET))
windows="[(${HOST_CHAIN_ID},${start},${end})]"

job="preview-propose-upgrade"
cleanup() {
  kubectl delete pod,secret -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null 2>&1 || true
}
trap cleanup EXIT

if [[ "${SKIP_PROPOSE}" == "true" ]]; then
  echo "Skipping on-chain propose (SKIP_PROPOSE=true)"
else
echo "proposeCoprocessorUpgrade id=${PROPOSAL_ID} version=${GCS_VERSION} windows=${windows} gw_start=${gw_start}"

if [[ -n "${GITHUB_ENV:-}" ]]; then
  {
    echo "UPGRADE_PROPOSAL_ID=${PROPOSAL_ID}"
    echo "UPGRADE_START_BLOCK=${start}"
    echo "UPGRADE_END_BLOCK=${end}"
    echo "UPGRADE_GW_START=${gw_start}"
  } >> "${GITHUB_ENV}"
fi

kubectl create secret generic "${job}" -n "${NAMESPACE}" \
  --from-literal=protocol_config="${protocol_config}" \
  --from-literal=host_http="${HOST_HTTP}" \
  --from-literal=pk="${DEPLOYER_KEY_9}" \
  --from-literal=version="${GCS_VERSION}" \
  --from-literal=windows="${windows}" \
  --from-literal=gw_start="${gw_start}" \
  --from-literal=proposal_id="${PROPOSAL_ID}" \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl delete pod -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null
kubectl apply -n "${NAMESPACE}" -f - <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: ${job}
spec:
  restartPolicy: Never
  activeDeadlineSeconds: 120
  imagePullSecrets:
    - name: registry-credentials
  containers:
    - name: cast
      image: ${FOUNDRY_IMAGE}
      command: ["sh", "-c"]
      args:
        - |
          exec cast send "\$PROTOCOL_CONFIG" \
            --rpc-url "\$HOST_HTTP" \
            --private-key "\$DEPLOYER_KEY" \
            "proposeCoprocessorUpgrade(uint256,string,(uint64,uint64,uint64)[],uint64)" \
            "\$PROPOSAL_ID" "\$GCS_VERSION" "\$WINDOWS" "\$GW_START"
      env:
        - name: PROTOCOL_CONFIG
          valueFrom: {secretKeyRef: {name: ${job}, key: protocol_config}}
        - name: HOST_HTTP
          valueFrom: {secretKeyRef: {name: ${job}, key: host_http}}
        - name: DEPLOYER_KEY
          valueFrom: {secretKeyRef: {name: ${job}, key: pk}}
        - name: GCS_VERSION
          valueFrom: {secretKeyRef: {name: ${job}, key: version}}
        - name: WINDOWS
          valueFrom: {secretKeyRef: {name: ${job}, key: windows}}
        - name: GW_START
          valueFrom: {secretKeyRef: {name: ${job}, key: gw_start}}
        - name: PROPOSAL_ID
          valueFrom: {secretKeyRef: {name: ${job}, key: proposal_id}}
EOF

kubectl wait --for=jsonpath='{.status.phase}'=Succeeded pod/"${job}" -n "${NAMESPACE}" --timeout=90s
kubectl logs -n "${NAMESPACE}" "${job}"
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
  echo "Waiting for party ${i} GCS DryRunStarted..."
  while true; do
    state=$(psql_party "${i}" \
      "SELECT COALESCE(string_agg(state, ','), '') FROM upgrade_state WHERE stack_role='GCS';" \
      || true)
    if [[ "${state}" == *"DryRunStarted"* ]]; then
      echo "party ${i}: ${state}"
      break
    fi
    if (( SECONDS >= deadline )); then
      echo "::error::party ${i} did not reach DryRunStarted (last='${state}')"
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
    if [[ "${version}" == "${LIVE_VERSION}" ]]; then
      echo "party ${i}: versioning=${version}"
      break
    fi
    if (( SECONDS >= deadline )); then
      echo "::error::party ${i} versioning='${version}', expected '${LIVE_VERSION}'"
      psql_party "${i}" "SELECT stack_role, state, status, version FROM upgrade_state;" || true
      exit 1
    fi
    sleep 5
  done
done

echo "RFC-021 cutover asserted on ${NB_COPROCESSOR} operator DB(s)."
