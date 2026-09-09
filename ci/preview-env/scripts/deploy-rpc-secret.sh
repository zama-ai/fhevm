#!/usr/bin/env bash
# chain_mode=testnets: materialise Secret `rpc` (Sepolia + Amoy RPC URLs + treasury key) from AWS Secrets
# Manager via ExternalSecret, then export them (masked) for the runner-side steps that cannot mount it.
# Env: NAMESPACE, RPC_SECRET_NAME.
set -euo pipefail

: "${NAMESPACE:?}"
: "${RPC_SECRET_NAME:?}"

kubectl apply -n "${NAMESPACE}" -f ci/preview-env/testnets/externalsecret-rpc.yaml
kubectl wait -n "${NAMESPACE}" externalsecret/preview-rpc --for=condition=Ready --timeout=120s \
  || { kubectl describe -n "${NAMESPACE}" externalsecret/preview-rpc | tail -20; exit 1; }

read_key() {
  local value
  value=$(kubectl get secret -n "${NAMESPACE}" "${RPC_SECRET_NAME}" -o jsonpath="{.data.$1}" | base64 -d)
  [[ -n "${value}" ]] || { echo "::error::secret ${RPC_SECRET_NAME} has an empty $1" >&2; exit 1; }
  echo "::add-mask::${value}"
  echo "${value}"
}

{
  echo "HOST_HTTP=$(read_key ethereum-rpc-url)"
  echo "HOST_WS=$(read_key ethereum-rpc-ws-url)"
  echo "POLYGON_HTTP=$(read_key polygon-rpc-url)"
  echo "POLYGON_WS=$(read_key polygon-rpc-ws-url)"
  echo "FUNDER_PRIVATE_KEY=$(read_key funder-private-key)"
} >> "${GITHUB_ENV}"
echo "Secret ${RPC_SECRET_NAME} synced (Sepolia + Amoy HTTP/WS, funder key); values exported masked."
