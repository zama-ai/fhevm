#!/usr/bin/env bash
# Install contracts chart releases (gateway/host/register/keygen).
# Usage: deploy-contracts.sh <gateway|host|host-polygon|add-host-chains|add-host-chains-polygon|trigger-keygen>
# Env: NAMESPACE, CONTRACTS_CHART, TAGS_JSON. VALUES_FILE required for gateway/host/host-polygon.
# CONTRACTS_DEPLOY_TIMEOUT / KEYGEN_TIMEOUT (helm --timeout) stretch on 12s public testnets.
set -euo pipefail

deploy_timeout="${CONTRACTS_DEPLOY_TIMEOUT:-10m}"
# The Default DKG is about a day: BitGen alone ran past 11h, then the DKG and
# CRS still have to finish. Test params and the insecure keygen fit in 45m.
# v0.12 and v0.13 previews call KeyGen on kms-core directly, so Default
# preprocessing is the real DKG even when the workflow flag is "insecure".
# 26h covers the 24h ActivateKey wait plus CRS. The pod script's own wait
# (KEYGEN_WAIT_TIMEOUT_MS) is raised to match below; the values default is 15m.
if [[ "${FHE_PARAMS:?}" == "default" && ( "${KEYGEN:?}" == "secure" || "${KMS_CORE_TAG:-}" =~ ^v0\.1[23]\. ) ]]; then
  keygen_timeout="${KEYGEN_TIMEOUT:-26h}"
  long_keygen="true"
else
  keygen_timeout="${KEYGEN_TIMEOUT:-45m}"
  long_keygen="false"
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
      # Anvil keeps the in-namespace gateway. External chains (blockchain-dev,
      # testnets) already patched this file to the host RPC; keygen for these
      # contracts runs on the gateway, so put the gateway RPC and its owner back.
      if [[ "${EXTERNAL_CHAINS:-false}" == "true" ]]; then
        : "${GATEWAY_HTTP:?}"
        : "${DEPLOYER_KEY_0:?}"
        : "${GATEWAY_CHAIN_ID:?}"
        keygen_rpc="${GATEWAY_HTTP}"
        keygen_key="${DEPLOYER_KEY_0}"
        keygen_chain_id="${GATEWAY_CHAIN_ID}"
      else
        keygen_rpc="http://anvil-gateway-anvil-node:8546"
        keygen_key="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
        keygen_chain_id="54321"
      fi
      KEYGEN_EXPORT="${KEYGEN_EXPORT}" KEYGEN_RPC="${keygen_rpc}" KEYGEN_KEY="${keygen_key}" KEYGEN_CHAIN_ID="${keygen_chain_id}" yq -i '
        .scDeploy.image.name = "hub.zama.org/ghcr/zama-ai/fhevm/gateway-contracts" |
        .persistence.volumeClaim.name = "gateway-sc-deploy-workdir" |
        (.scDeploy.env[] | select(.name == "RPC_URL")) = {"name": "RPC_URL", "value": strenv(KEYGEN_RPC)} |
        (.scDeploy.env[] | select(.name == "DEPLOYER_PRIVATE_KEY")) = {"name": "DEPLOYER_PRIVATE_KEY", "value": strenv(KEYGEN_KEY)} |
        (.scDeploy.env[] | select(.name == "CHAIN_ID").value) = strenv(KEYGEN_CHAIN_ID) |
        .scDeploy.deployCommands[0] = strenv(KEYGEN_EXPORT)
      ' "${keygen_values}"
      keygen_tag=$(jq -r .gateway_contracts <<<"${TAGS_JSON}")
    fi
    if [[ "${long_keygen}" == "true" ]]; then
      NAME="KEYGEN_WAIT_TIMEOUT_MS" VALUE="${KEYGEN_WAIT_TIMEOUT_MS:-86400000}" yq -i \
        '.scDeploy.env |= (map(select(.name != strenv(NAME))) + [{"name": strenv(NAME), "value": strenv(VALUE)}])' \
        "${keygen_values}"
      NAME="CRSGEN_WAIT_TIMEOUT_MS" VALUE="${CRSGEN_WAIT_TIMEOUT_MS:-14400000}" yq -i \
        '.scDeploy.env |= (map(select(.name != strenv(NAME))) + [{"name": strenv(NAME), "value": strenv(VALUE)}])' \
        "${keygen_values}"
    fi
    # Chart 0.7.3 omits backoffLimit, so Kubernetes retries the Job 6 times.
    # Each retry runs task:triggerKeygen again and reverts with KeygenOngoing.
    # Patch as soon as the Job exists.
    helm upgrade --install host-trigger-keygen "${CONTRACTS_CHART}" \
      -n "${NAMESPACE}" -f "${keygen_values}" \
      --set-string "scDeploy.image.tag=${keygen_tag}" \
      --wait --wait-for-jobs --timeout="${keygen_timeout}" &
    helm_pid=$!
    for _ in $(seq 1 60); do
      job_name=$(kubectl get jobs -n "${NAMESPACE}" -o name | sed -n 's|^job/||p' | grep -E '^host-trigger-keygen-deploy' | head -1 || true)
      if [[ -n "${job_name}" ]]; then
        kubectl patch job "${job_name}" -n "${NAMESPACE}" --type merge -p '{"spec":{"backoffLimit":0}}'
        break
      fi
      sleep 2
    done
    wait "${helm_pid}"
    ;;
  *)
    echo "::error::unknown contracts kind '${kind}'" >&2
    exit 1
    ;;
esac
