#!/usr/bin/env bash
# Install contracts chart releases (gateway/host/register/keygen).
# Usage: deploy-contracts.sh <gateway|host|host-polygon|add-host-chains|add-host-chains-polygon|trigger-keygen>
# Env: NAMESPACE, CONTRACTS_CHART, TAGS_JSON. VALUES_FILE required for gateway/host/host-polygon.
# CONTRACTS_DEPLOY_TIMEOUT / KEYGEN_TIMEOUT (helm --timeout) stretch on 12s public testnets.
set -euo pipefail

deploy_timeout="${CONTRACTS_DEPLOY_TIMEOUT:-10m}"
# The Default DKG is multi-hour; Test params and the insecure keygen (key and
# CRS in about 6 min) finish well within 45m.
if [[ "${FHE_PARAMS:?}" == "default" && "${KEYGEN:?}" == "secure" ]]; then
  keygen_timeout="${KEYGEN_TIMEOUT:-310m}"
else
  keygen_timeout="${KEYGEN_TIMEOUT:-45m}"
fi

kind="${1:?kind}"
case "${kind}" in
  gateway)
    helm upgrade --install gateway-contracts "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${VALUES_FILE}" \
      --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout="${deploy_timeout}"
    ;;
  host)
    # --with-kms-generation arrived with the v0.13 canonical-host split.
    # Older images reject it (HH305). v0.10 deploys KMSGeneration from gateway-contracts.
    host_tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")
    if [[ "${host_tag}" =~ ^v([0-9]+)\.([0-9]+)\. ]] && (( BASH_REMATCH[1] == 0 && BASH_REMATCH[2] < 13 )); then
      echo "host-contracts ${host_tag} has no --with-kms-generation; deploying without it"
      yq -i '.scDeploy.deployCommands = ["npx hardhat task:deployAllHostContracts"]' "${VALUES_FILE}"
    fi
    helm upgrade --install host-contracts "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${VALUES_FILE}" \
      --set-string "scDeploy.image.tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout="${deploy_timeout}"
    ;;
  host-polygon)
    helm upgrade --install host-contracts-polygon "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${VALUES_FILE}" \
      --set-string "scDeploy.image.tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout="${deploy_timeout}"
    ;;
  add-host-chains)
    helm upgrade --install gateway-add-host-chains "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/gateway-chain/values-gateway-add-host-chains-e2e.yaml \
      --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout="${deploy_timeout}"
    ;;
  add-host-chains-polygon)
    helm upgrade --install gateway-add-host-chains-polygon "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/gateway-chain/values-gateway-add-host-chains-polygon-e2e.yaml \
      --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout="${deploy_timeout}"
    ;;
  trigger-keygen)
    helm upgrade --install host-trigger-keygen "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/host-chain/values-host-trigger-keygen-e2e.yaml \
      --set-string "scDeploy.image.tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout="${keygen_timeout}"
    ;;
  *)
    echo "::error::unknown contracts kind '${kind}'" >&2
    exit 1
    ;;
esac
