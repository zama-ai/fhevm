#!/usr/bin/env bash
# chain_mode=testnets: install sync-secrets (same chart gitops uses) so ESO
# materialises Secret `rpc` (Sepolia + Amoy URLs) and the two faucet keys
# (`eth-faucet`, `polygon-faucet`). Export them masked for runner-side steps
# that cannot mount.
# Env: NAMESPACE, SYNC_SECRETS_CHART, SYNC_SECRETS_CHART_VERSION,
# RPC_SECRET_NAME, ETH_FAUCET_SECRET_NAME, POLYGON_FAUCET_SECRET_NAME.
set -euo pipefail

: "${NAMESPACE:?}"
: "${SYNC_SECRETS_CHART:?}"
: "${SYNC_SECRETS_CHART_VERSION:?}"
: "${RPC_SECRET_NAME:?}"
: "${ETH_FAUCET_SECRET_NAME:?}"
: "${POLYGON_FAUCET_SECRET_NAME:?}"

helm upgrade --install preview-rpc "${SYNC_SECRETS_CHART}" --version "${SYNC_SECRETS_CHART_VERSION}" \
  -n "${NAMESPACE}" -f ci/preview-env/testnets/values-rpc.yaml
helm upgrade --install preview-eth-faucet "${SYNC_SECRETS_CHART}" --version "${SYNC_SECRETS_CHART_VERSION}" \
  -n "${NAMESPACE}" -f ci/preview-env/testnets/values-eth-faucet.yaml
helm upgrade --install preview-polygon-faucet "${SYNC_SECRETS_CHART}" --version "${SYNC_SECRETS_CHART_VERSION}" \
  -n "${NAMESPACE}" -f ci/preview-env/testnets/values-polygon-faucet.yaml

wait_es() {
  local name="$1"
  kubectl wait -n "${NAMESPACE}" "externalsecret/${name}" --for=condition=Ready --timeout=120s \
    || { kubectl describe -n "${NAMESPACE}" "externalsecret/${name}" | tail -20; exit 1; }
}

wait_es preview-rpc
wait_es preview-eth-faucet
wait_es preview-polygon-faucet

# Mask and write straight to GITHUB_ENV: ::add-mask:: must reach the step's own
# stdout, so it cannot be emitted from inside a command substitution.
export_key() {
  local var="$1" secret="$2" key="$3" encoded value
  encoded=$(kubectl get secret -n "${NAMESPACE}" "${secret}" -o json \
    | jq -r --arg k "${key}" '.data[$k] // empty')
  [[ -n "${encoded}" ]] || { echo "::error::secret ${secret} has no ${key}" >&2; exit 1; }
  # $() already drops trailing newlines, which GITHUB_ENV would reject anyway.
  value=$(printf '%s' "${encoded}" | base64 -d)
  [[ -n "${value}" ]] || { echo "::error::secret ${secret} has an empty ${key}" >&2; exit 1; }
  echo "::add-mask::${value}"
  echo "${var}=${value}" >> "${GITHUB_ENV}"
}

export_key HOST_HTTP "${RPC_SECRET_NAME}" ethereum-rpc-url
export_key HOST_WS "${RPC_SECRET_NAME}" ethereum-rpc-ws-url
export_key POLYGON_HTTP "${RPC_SECRET_NAME}" polygon-rpc-url
export_key POLYGON_WS "${RPC_SECRET_NAME}" polygon-rpc-ws-url
export_key ETH_FUNDER_PRIVATE_KEY "${ETH_FAUCET_SECRET_NAME}" private-key
export_key POLYGON_FUNDER_PRIVATE_KEY "${POLYGON_FAUCET_SECRET_NAME}" private-key
echo "Secrets ${RPC_SECRET_NAME} (Sepolia + Amoy HTTP/WS), ${ETH_FAUCET_SECRET_NAME}, ${POLYGON_FAUCET_SECRET_NAME} synced via sync-secrets; values exported masked."
