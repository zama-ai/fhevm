#!/usr/bin/env bash
# Per-namespace Anvil chains (skipped when EXTERNAL_CHAINS=true: blockchain-dev / testnets).
# Usage: deploy-anvil.sh <host|gateway|host-polygon>
# Env: NAMESPACE, ANVIL_NODE_CHART.
set -euo pipefail

kind="${1:?kind}"
case "${kind}" in
  host)
    helm upgrade --install anvil-host "${ANVIL_NODE_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/host-chain/values-anvil-host-e2e.yaml --wait
    ;;
  gateway)
    helm upgrade --install anvil-gateway "${ANVIL_NODE_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/gateway-chain/values-anvil-gateway-e2e.yaml --wait
    ;;
  host-polygon)
    helm upgrade --install anvil-host-polygon "${ANVIL_NODE_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/host-chain/values-anvil-host-polygon-e2e.yaml --wait
    ;;
  *)
    echo "::error::unknown anvil kind '${kind}'" >&2
    exit 1
    ;;
esac
