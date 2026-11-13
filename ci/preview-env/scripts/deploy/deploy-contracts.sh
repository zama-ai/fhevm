#!/usr/bin/env bash
# Install contracts chart releases (gateway/host/register/keygen).
# Usage: deploy-contracts.sh <gateway|host|host-polygon|add-host-chains|add-host-chains-polygon|trigger-keygen>
# Env: NAMESPACE, CONTRACTS_CHART, TAGS_JSON. VALUES_FILE required for gateway/host/host-polygon.
# CONTRACTS_DEPLOY_TIMEOUT / KEYGEN_TIMEOUT (helm --timeout) stretch on 12s public testnets.
set -euo pipefail

deploy_timeout="${CONTRACTS_DEPLOY_TIMEOUT:-10m}"
# The Default DKG is multi-hour; Test params and the insecure keygen (key and
# CRS in about 6 min) finish well within 45m. v0.12 and v0.13 have no
# InsecureKeyGenPreproc, so Default preprocessing is the real DKG even when
# KeyGen itself is insecure.
if [[ "${FHE_PARAMS:?}" == "default" && ( "${KEYGEN:?}" == "secure" || "${KMS_CORE_TAG:-}" =~ ^v0\.1[23]\. ) ]]; then
  keygen_timeout="${KEYGEN_TIMEOUT:-310m}"
else
  keygen_timeout="${KEYGEN_TIMEOUT:-45m}"
fi

kind="${1:?kind}"

# --with-kms-generation and host-side task:triggerKeygen arrived with the v0.13
# canonical-host split. A vX.Y.Z pin older than that still deploys KMSGeneration
# from gateway-contracts and triggers keygen there. Non-semver tags are current.
host_contracts_before_v013() {
  local tag="$1"
  [[ "${tag}" =~ ^v([0-9]+)\.([0-9]+)\. ]] || return 1
  (( 10#${BASH_REMATCH[1]} == 0 && 10#${BASH_REMATCH[2]} < 13 ))
}

case "${kind}" in
  gateway)
    helm upgrade --install gateway-contracts "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${VALUES_FILE}" \
      --set-string "scDeploy.image.tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout="${deploy_timeout}"
    ;;
  host)
    host_tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")
    if host_contracts_before_v013 "${host_tag}"; then
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
    keygen_values="ci/preview-env/host-chain/values-host-trigger-keygen-e2e.yaml"
    keygen_tag=$(jq -r .host_contracts <<<"${TAGS_JSON}")
    host_tag="${keygen_tag}"
    if host_contracts_before_v013 "${host_tag}"; then
      # KMSGeneration is on the gateway chain. keygen() is onlyGatewayOwner, and
      # the gateway deploy uses Hardhat account #0 (values-gateway-contracts-e2e.yaml).
      echo "host-contracts ${host_tag}: triggering keygen from gateway-contracts"
      keygen_values="$(mktemp)"
      cp ci/preview-env/host-chain/values-host-trigger-keygen-e2e.yaml "${keygen_values}"
      KEYGEN_EXPORT="$(cat <<'EOF'
export KMS_GENERATION_CONTRACT_ADDRESS=$(grep --color=never '^KMS_GENERATION_ADDRESS=' addresses/.env.gateway | cut -d'=' -f2 | tr -d '[:space:]')
if [[ -z "${KMS_GENERATION_CONTRACT_ADDRESS}" ]]; then
  echo "::error::KMS_GENERATION_ADDRESS could not be extracted from addresses/.env.gateway - refusing to start the consensus poll with a broken address"
  exit 1
fi
echo "Resolved KMS_GENERATION_CONTRACT_ADDRESS=${KMS_GENERATION_CONTRACT_ADDRESS}"
EOF
)"
      KEYGEN_EXPORT="${KEYGEN_EXPORT}" yq -i '
        .scDeploy.image.name = "hub.zama.org/ghcr/zama-ai/fhevm/gateway-contracts" |
        .persistence.volumeClaim.name = "gateway-sc-deploy-workdir" |
        (.scDeploy.env[] | select(.name == "RPC_URL").value) = "http://anvil-gateway-anvil-node:8546" |
        (.scDeploy.env[] | select(.name == "DEPLOYER_PRIVATE_KEY").value) = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" |
        .scDeploy.deployCommands[0] = strenv(KEYGEN_EXPORT)
      ' "${keygen_values}"
      keygen_tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")
    fi
    helm upgrade --install host-trigger-keygen "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${keygen_values}" \
      --set-string "scDeploy.image.tag=${keygen_tag}" \
      --wait --wait-for-jobs --timeout="${keygen_timeout}"
    ;;
  *)
    echo "::error::unknown contracts kind '${kind}'" >&2
    exit 1
    ;;
esac
