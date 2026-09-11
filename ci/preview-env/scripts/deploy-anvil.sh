#!/usr/bin/env bash
# Per-namespace Anvil chains (skipped when USE_BLOCKCHAIN_DEV=true).
# Usage: deploy-anvil.sh <host|gateway|host-polygon>
# Env: NAMESPACE, ANVIL_NODE_CHART.
set -euo pipefail

persist_state=false
[[ "${SOLANA_ACTION:-off}" == off ]] || persist_state=true
kind="${1:?kind}"
case "${kind}" in
  host)
    helm upgrade --install anvil-host "${ANVIL_NODE_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/host-chain/values-anvil-host-e2e.yaml --set "persistState=$persist_state" --wait
    ;;
  gateway)
    helm upgrade --install anvil-gateway "${ANVIL_NODE_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/gateway-chain/values-anvil-gateway-e2e.yaml --set "persistState=$persist_state" --wait
    ;;
  host-polygon)
    helm upgrade --install anvil-host-polygon "${ANVIL_NODE_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/host-chain/values-anvil-host-polygon-e2e.yaml --set "persistState=$persist_state" --wait
    ;;
  *)
    echo "::error::unknown anvil kind '${kind}'" >&2
    exit 1
    ;;
esac
