#!/usr/bin/env bash
# Install contracts chart releases (gateway/host/register/keygen).
# Usage: deploy-contracts.sh <gateway|host|host-polygon|add-host-chains|add-host-chains-polygon|trigger-keygen>
# Env: NAMESPACE, CONTRACTS_CHART, TAGS_JSON. VALUES_FILE required for gateway/host/host-polygon.
set -euo pipefail

kind="${1:?kind}"
case "${kind}" in
  gateway)
    helm upgrade --install gateway-contracts "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${VALUES_FILE}" \
      --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout=10m
    ;;
  host)
    helm upgrade --install host-contracts "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${VALUES_FILE}" \
      --set-string "scDeploy.image.tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout=10m
    ;;
  host-polygon)
    helm upgrade --install host-contracts-polygon "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${VALUES_FILE}" \
      --set-string "scDeploy.image.tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout=10m
    ;;
  add-host-chains)
    helm upgrade --install gateway-add-host-chains "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/gateway-chain/values-gateway-add-host-chains-e2e.yaml \
      --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout=10m
    ;;
  add-host-chains-polygon)
    helm upgrade --install gateway-add-host-chains-polygon "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/gateway-chain/values-gateway-add-host-chains-polygon-e2e.yaml \
      --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout=10m
    ;;
  trigger-keygen)
    helm upgrade --install host-trigger-keygen "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/host-chain/values-host-trigger-keygen-e2e.yaml \
      --set-string "scDeploy.image.tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout=45m
    ;;
  *)
    echo "::error::unknown contracts kind '${kind}'" >&2
    exit 1
    ;;
esac
