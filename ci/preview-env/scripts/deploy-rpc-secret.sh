#!/usr/bin/env bash
# chain_mode=testnets: install sync-secrets (same chart gitops uses) so ESO
# materialises Secret `rpc` (Sepolia + Amoy URLs) and Secret `funder`
# (treasury key). Export them masked for runner-side steps that cannot mount.
# Env: NAMESPACE, SYNC_SECRETS_CHART, SYNC_SECRETS_CHART_VERSION,
# RPC_SECRET_NAME, FUNDER_SECRET_NAME.
set -euo pipefail

: "${NAMESPACE:?}"
: "${SYNC_SECRETS_CHART:?}"
: "${SYNC_SECRETS_CHART_VERSION:?}"
: "${RPC_SECRET_NAME:?}"
: "${FUNDER_SECRET_NAME:?}"

helm upgrade --install preview-rpc "${SYNC_SECRETS_CHART}" --version "${SYNC_SECRETS_CHART_VERSION}" \
  -n "${NAMESPACE}" -f ci/preview-env/testnets/values-rpc.yaml
helm upgrade --install preview-funder "${SYNC_SECRETS_CHART}" --version "${SYNC_SECRETS_CHART_VERSION}" \
  -n "${NAMESPACE}" -f ci/preview-env/testnets/values-funder.yaml

wait_es() {
  local name="$1"
  kubectl wait -n "${NAMESPACE}" "externalsecret/${name}" --for=condition=Ready --timeout=120s \
    || { kubectl describe -n "${NAMESPACE}" "externalsecret/${name}" | tail -20; exit 1; }
}

wait_es preview-rpc
wait_es preview-funder

read_key() {
  local secret="$1" key="$2" value
  value=$(kubectl get secret -n "${NAMESPACE}" "${secret}" -o json \
    | jq -r --arg k "${key}" '.data[$k] // empty')
  [[ -n "${value}" ]] || { echo "::error::secret ${secret} has an empty ${key}" >&2; exit 1; }
  value=$(printf '%s' "${value}" | base64 -d)
  echo "::add-mask::${value}"
  echo "${value}"
}

{
  echo "HOST_HTTP=$(read_key "${RPC_SECRET_NAME}" ethereum-rpc-url)"
  echo "HOST_WS=$(read_key "${RPC_SECRET_NAME}" ethereum-rpc-ws-url)"
  echo "POLYGON_HTTP=$(read_key "${RPC_SECRET_NAME}" polygon-rpc-url)"
  echo "POLYGON_WS=$(read_key "${RPC_SECRET_NAME}" polygon-rpc-ws-url)"
  echo "FUNDER_PRIVATE_KEY=$(read_key "${FUNDER_SECRET_NAME}" private-key)"
} >> "${GITHUB_ENV}"
echo "Secrets ${RPC_SECRET_NAME} (Sepolia + Amoy HTTP/WS) and ${FUNDER_SECRET_NAME} (treasury key) synced via sync-secrets; values exported masked."
