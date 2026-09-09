#!/usr/bin/env bash
# eRPC public-RPC proxy for chain_mode=testnets: the listener chart's erpc with no listeners (erpc/values-erpc-e2e.yaml).
# Env: NAMESPACE, LISTENER_CHART.
set -euo pipefail

: "${NAMESPACE:?}"
: "${LISTENER_CHART:?}"

helm upgrade --install preview-erpc "${LISTENER_CHART}" \
  -n "${NAMESPACE}" -f ci/preview-env/erpc/values-erpc-e2e.yaml \
  --wait --timeout=5m

# Smoke eth_chainId through the proxy now, not 40 minutes later inside a hardhat Job.
for chain_id in 11155111 80002; do
  url="http://preview-erpc:4000/listener-indexer/evm/${chain_id}"
  raw=$(bash "$(dirname "${BASH_SOURCE[0]}")/cluster-rpc.sh" "${url}" \
    '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}')
  got=$(jq -r '.result' <<<"${raw}")
  if [[ "$((got))" != "${chain_id}" ]]; then
    echo "::error::eRPC ${url} returned chainId ${got}, expected ${chain_id}"
    exit 1
  fi
  echo "eRPC ok: ${url} -> chainId ${chain_id}"
done
