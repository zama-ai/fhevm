#!/usr/bin/env bash
# Patch preview-env Helm values in the working tree for an external chain mode (blockchain-dev | testnets); no-op on Anvil.
# Keys/mnemonic/RPCs come from env (resolve-chain.sh + generate-mnemonic.cjs). With RPC_SECRET_NAME set (testnets),
# host RPC URLs are wired as secretKeyRef into Secret ${RPC_SECRET_NAME} instead of literals.
set -euo pipefail

if [[ "${EXTERNAL_CHAINS:-false}" != "true" ]]; then
  echo "Anvil mode: leaving values overlays unchanged."
  exit 0
fi

: "${CHAIN_MODE:?}"
: "${HOST_HTTP:?}"
: "${HOST_WS:?}"
: "${GATEWAY_HTTP:?}"
: "${GATEWAY_WS:?}"
: "${HOST_CHAIN_ID:?}"
: "${GATEWAY_CHAIN_ID:?}"
: "${POLLER_SEED_START_BLOCK:?}"
: "${HOST_FINALITY_DEPTH:?}"
: "${HOST_FINALITY_LAG:?}"
: "${MNEMONIC:?}"
: "${DEPLOYER_KEY_0:?}"
: "${DEPLOYER_KEY_3:?}"
: "${DEPLOYER_KEY_9:?}"
: "${HARDHAT_NETWORK_TESTS:?}"

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

set_named_env() {
  # $1 file, $2 yq path to env array (e.g. .scDeploy.env), $3 name, $4 value
  local file="$1" arr="$2" name="$3" value="$4"
  local n
  n=$(NAME="${name}" yq "[${arr}[] | select(.name == strenv(NAME))] | length" "${file}")
  if [[ "${n}" == "0" ]]; then
    NAME="${name}" VALUE="${value}" yq -i \
      "${arr} += [{\"name\": strenv(NAME), \"value\": strenv(VALUE)}]" \
      "${file}"
  else
    NAME="${name}" VALUE="${value}" yq -i \
      "(${arr}[] | select(.name == strenv(NAME))).value = strenv(VALUE)" \
      "${file}"
  fi
}

# Upsert an env entry as valueFrom.secretKeyRef (drops any literal value): $1 file, $2 yq path to env array, $3 name, $4 secret key.
set_named_env_secret() {
  local file="$1" arr="$2" name="$3" key="$4"
  NAME="${name}" yq -i "${arr} |= map(select(.name != strenv(NAME)))" "${file}"
  NAME="${name}" SECRET="${RPC_SECRET_NAME}" KEY="${key}" yq -i \
    "${arr} += [{\"name\": strenv(NAME), \"valueFrom\": {\"secretKeyRef\": {\"name\": strenv(SECRET), \"key\": strenv(KEY)}}}]" \
    "${file}"
}
# Point a coprocessor chains[] entry at the RPC Secret: $1 file, $2 chain name, $3 http key, $4 ws key.
set_chain_urls_secret() {
  local file="$1" chain="$2" http_key="$3" ws_key="$4"
  CHAIN="${chain}" SECRET="${RPC_SECRET_NAME}" HTTP_KEY="${http_key}" WS_KEY="${ws_key}" yq -i '
    (.chains[] | select(.name == strenv(CHAIN))) |= (
      del(.httpUrl) | del(.wsUrl) |
      .httpUrlValueFrom = {"secretKeyRef": {"name": strenv(SECRET), "key": strenv(HTTP_KEY)}} |
      .wsUrlValueFrom = {"secretKeyRef": {"name": strenv(SECRET), "key": strenv(WS_KEY)}}
    )' "${file}"
}
rpc_from_secret() { [[ -n "${RPC_SECRET_NAME:-}" ]]; }

# Replace a poller extraArgs flag (--name=value) wholesale so a stale default never survives.
set_poller_flag() {
  local file="$1" flag="$2" value="$3"
  FLAG="${flag}" VALUE="${value}" yq -i '
    .hostListenerPollerShared.extraArgs |=
      map(select(test("^" + strenv(FLAG) + "=") | not)) + [strenv(FLAG) + "=" + strenv(VALUE)]
  ' "${file}"
}

# Workflow run-test env is nested; match by .name anywhere under a given file
# would collide on DEPLOYER_PRIVATE_KEY. Keep file-specific helpers instead.

echo "Patching preview-env values for ${CHAIN_MODE} (host ${HOST_CHAIN_ID}, gateway ${GATEWAY_CHAIN_ID}, poller seed ${POLLER_SEED_START_BLOCK})"

# --- gateway contracts ---
gw="${root}/gateway-chain/values-gateway-contracts-e2e.yaml"
set_named_env "${gw}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
set_named_env "${gw}" ".scDeploy.env" RPC_URL "${GATEWAY_HTTP}"
set_named_env "${gw}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_0}"
set_named_env "${gw}" ".scDeploy.env" TX_SENDER_PRIVATE_KEY "${DEPLOYER_KEY_3}"
set_named_env "${gw}" ".scDeploy.env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"

add="${root}/gateway-chain/values-gateway-add-host-chains-e2e.yaml"
set_named_env "${add}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
set_named_env "${add}" ".scDeploy.env" RPC_URL "${GATEWAY_HTTP}"
set_named_env "${add}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_0}"
set_named_env "${add}" ".scDeploy.env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
set_named_env "${add}" ".scDeploy.env" HOST_CHAIN_CHAIN_ID_0 "${HOST_CHAIN_ID}"

# --- host contracts / keygen ---
host="${root}/host-chain/values-host-contracts-e2e.yaml"
set_named_env "${host}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
if rpc_from_secret; then set_named_env_secret "${host}" ".scDeploy.env" RPC_URL ethereum-rpc-url; else set_named_env "${host}" ".scDeploy.env" RPC_URL "${HOST_HTTP}"; fi
set_named_env "${host}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_9}"
set_named_env "${host}" ".scDeploy.env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
set_named_env "${host}" ".scDeploy.env" CHAIN_ID "${HOST_CHAIN_ID}"

keygen="${root}/host-chain/values-host-trigger-keygen-e2e.yaml"
set_named_env "${keygen}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
if rpc_from_secret; then set_named_env_secret "${keygen}" ".scDeploy.env" RPC_URL ethereum-rpc-url; else set_named_env "${keygen}" ".scDeploy.env" RPC_URL "${HOST_HTTP}"; fi
set_named_env "${keygen}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_9}"
set_named_env "${keygen}" ".scDeploy.env" CHAIN_ID "${HOST_CHAIN_ID}"

# --- listener ---
lis="${root}/listener/values-listener-e2e.yaml"
HOST_CHAIN_ID="${HOST_CHAIN_ID}" HOST_HTTP="${HOST_HTTP}" HOST_FINALITY_DEPTH="${HOST_FINALITY_DEPTH}" yq -i '
  .listeners[0].config.blockchain.chain_id = env(HOST_CHAIN_ID) |
  .listeners[0].config.blockchain.rpc_url = strenv(HOST_HTTP) |
  .listeners[0].config.blockchain.finality_depth = env(HOST_FINALITY_DEPTH)
' "${lis}"
# Secret mode: the env override wins over config.yaml, so the literal above never reaches a pod.
if rpc_from_secret; then
  yq -i '.listeners[0].config.blockchain.rpc_url = "set-by-APP_BLOCKCHAIN__RPC_URL"' "${lis}"
  set_named_env_secret "${lis}" ".env" APP_BLOCKCHAIN__RPC_URL ethereum-rpc-url
fi

# --- coprocessor + poller ---
for f in "${root}/coprocessor/values-coprocessor-e2e.yaml" \
         "${root}/coprocessor/values-coprocessor-poller-e2e.yaml"; do
  HOST_HTTP="${HOST_HTTP}" HOST_WS="${HOST_WS}" GATEWAY_WS="${GATEWAY_WS}" \
  HOST_CHAIN_ID="${HOST_CHAIN_ID}" yq -i '
    .commonConfig.hostChainHttpUrl = strenv(HOST_HTTP) |
    .commonConfig.hostChainWsUrl = strenv(HOST_WS) |
    (.chains[] | select(.name == "host") | .chainId) = strenv(HOST_CHAIN_ID) |
    (.chains[] | select(.name == "host") | .httpUrl) = strenv(HOST_HTTP) |
    (.chains[] | select(.name == "host") | .wsUrl) = strenv(HOST_WS)
  ' "${f}"
done
for f in "${root}/coprocessor/values-coprocessor-bcs-e2e.yaml" \
         "${root}/coprocessor/values-coprocessor-gcs-e2e.yaml" \
         "${root}/coprocessor/values-coprocessor-poller-e2e.yaml"; do
  HOST_CHAIN_ID="${HOST_CHAIN_ID}" yq -i \
    '.commonConfig.canonicalProtocolConfigChainId = strenv(HOST_CHAIN_ID)' \
    "${f}"
done
GATEWAY_WS="${GATEWAY_WS}" yq -i '.commonConfig.gatewayUrl.value = strenv(GATEWAY_WS)' \
  "${root}/coprocessor/values-coprocessor-e2e.yaml"
if rpc_from_secret; then
  for f in "${root}/coprocessor/values-coprocessor-e2e.yaml" "${root}/coprocessor/values-coprocessor-poller-e2e.yaml"; do
    set_chain_urls_secret "${f}" host ethereum-rpc-url ethereum-rpc-ws-url
    # Only the (disabled) built-in hostListener reads these; keep the real URL out of the values file.
    yq -i '.commonConfig.hostChainHttpUrl = "unused-on-testnets" | .commonConfig.hostChainWsUrl = "unused-on-testnets"' "${f}"
  done
fi

poller="${root}/coprocessor/values-coprocessor-poller-e2e.yaml"
set_poller_flag "${poller}" "--seed-start-block" "${POLLER_SEED_START_BLOCK}"
set_poller_flag "${poller}" "--finality-lag" "${HOST_FINALITY_LAG}"

# --- kms-connector ---
kms="${root}/kms-connector/values-kms-connector-e2e.yaml"
GATEWAY_HTTP="${GATEWAY_HTTP}" HOST_HTTP="${HOST_HTTP}" \
GATEWAY_CHAIN_ID="${GATEWAY_CHAIN_ID}" HOST_CHAIN_ID="${HOST_CHAIN_ID}" yq -i '
  .commonConfig.gatewayUrl = strenv(GATEWAY_HTTP) |
  .commonConfig.ethereumUrl = strenv(HOST_HTTP) |
  .commonConfig.gatewayChainId = strenv(GATEWAY_CHAIN_ID) |
  .commonConfig.ethereumChainId = strenv(HOST_CHAIN_ID)
' "${kms}"
if rpc_from_secret; then
  set_named_env_secret "${kms}" ".commonConfig.env" RPC_ETH_URL ethereum-rpc-url
  # shellcheck disable=SC2016  # literal for k8s env / shell expansion inside the pod
  yq -i '.commonConfig.ethereumUrl = "$(RPC_ETH_URL)"' "${kms}"
fi

# --- relayer ---
rel="${root}/relayer/values-relayer-e2e.yaml"
set_named_env "${rel}" ".env" APP_HOST_CHAINS__0__CHAIN_ID "${HOST_CHAIN_ID}"
if rpc_from_secret; then
  set_named_env_secret "${rel}" ".env" APP_HOST_CHAINS__0__URL ethereum-rpc-url
  set_named_env_secret "${rel}" ".env" APP_PROTOCOL_CONFIG__ETHEREUM_HTTP_RPC_URL ethereum-rpc-url
else
  set_named_env "${rel}" ".env" APP_HOST_CHAINS__0__URL "${HOST_HTTP}"
  set_named_env "${rel}" ".env" APP_PROTOCOL_CONFIG__ETHEREUM_HTTP_RPC_URL "${HOST_HTTP}"
fi
set_named_env "${rel}" ".env" APP_GATEWAY__BLOCKCHAIN_RPC__HTTP_URL "${GATEWAY_HTTP}"
set_named_env "${rel}" ".env" APP_GATEWAY__BLOCKCHAIN_RPC__READ_HTTP_URL "${GATEWAY_HTTP}"
set_named_env "${rel}" ".env" APP_GATEWAY__BLOCKCHAIN_RPC__CHAIN_ID "${GATEWAY_CHAIN_ID}"
set_named_env "${rel}" ".env" APP_GATEWAY__LISTENER_POOL__LISTENERS__0__URL "${GATEWAY_WS}"
set_named_env "${rel}" ".env" APP_GATEWAY__LISTENER_POOL__LISTENERS__1__URL "${GATEWAY_HTTP}"
set_named_env "${rel}" ".env" APP_GATEWAY__TX_ENGINE__SIGNER__PRIVATE_KEY "${DEPLOYER_KEY_3}"

# --- idle test-suite Job ---
ts="${root}/test-suite/values-test-suite-e2e.yaml"
set_named_env "${ts}" ".env" MNEMONIC "${MNEMONIC}"
set_named_env "${ts}" ".env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
set_named_env "${ts}" ".env" CHAIN_ID_HOST "${HOST_CHAIN_ID}"
if rpc_from_secret; then set_named_env_secret "${ts}" ".env" RPC_URL ethereum-rpc-url; else set_named_env "${ts}" ".env" RPC_URL "${HOST_HTTP}"; fi
set_named_env "${ts}" ".env" GATEWAY_RPC_URL "${GATEWAY_HTTP}"
set_named_env "${ts}" ".env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_9}"
set_named_env "${ts}" ".env" GATEWAY_DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_0}"
set_named_env "${ts}" ".env" HARDHAT_NETWORK "${HARDHAT_NETWORK_TESTS}"
set_named_env "${ts}" ".env" NETWORK "${HARDHAT_NETWORK_TESTS}"

# --- Argo workflow run-test env (fhevm-sdk + relayer-sdk) ---
wf_run_test_env='.additionalResources[] | select(.object.kind == "Workflow") | .object.spec.templates[] | select(.name == "run-test") | .script.env'
set_wf_named() {
  local file="$1" name="$2" value="$3"
  local n
  n=$(NAME="${name}" yq "[${wf_run_test_env}[] | select(.name == strenv(NAME))] | length" "${file}")
  if [[ "${n}" == "0" ]]; then
    NAME="${name}" VALUE="${value}" yq -i \
      "(${wf_run_test_env}) += [{\"name\": strenv(NAME), \"value\": strenv(VALUE)}]" \
      "${file}"
  else
    NAME="${name}" VALUE="${value}" yq -i \
      "(${wf_run_test_env}[] | select(.name == strenv(NAME))).value = strenv(VALUE)" \
      "${file}"
  fi
}
set_wf_named_secret() {
  local file="$1" name="$2" key="$3"
  NAME="${name}" yq -i "(${wf_run_test_env}) |= map(select(.name != strenv(NAME)))" "${file}"
  NAME="${name}" SECRET="${RPC_SECRET_NAME}" KEY="${key}" yq -i \
    "(${wf_run_test_env}) += [{\"name\": strenv(NAME), \"valueFrom\": {\"secretKeyRef\": {\"name\": strenv(SECRET), \"key\": strenv(KEY)}}}]" \
    "${file}"
}
# Shared identity/gateway env for every e2e workflow; the host RPC is per chain ($2 = literal URL, $3 = secret key).
patch_workflow_common() {
  local file="$1" host_rpc="$2" host_rpc_key="$3"
  set_wf_named "${file}" MNEMONIC "${MNEMONIC}"
  set_wf_named "${file}" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
  if rpc_from_secret; then set_wf_named_secret "${file}" RPC_URL "${host_rpc_key}"; else set_wf_named "${file}" RPC_URL "${host_rpc}"; fi
  set_wf_named "${file}" GATEWAY_RPC_URL "${GATEWAY_HTTP}"
  set_wf_named "${file}" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_9}"
  set_wf_named "${file}" GATEWAY_DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_0}"
}
patch_workflow_env() {
  local file="$1"
  patch_workflow_common "${file}" "${HOST_HTTP}" ethereum-rpc-url
  set_wf_named "${file}" CHAIN_ID_HOST "${HOST_CHAIN_ID}"
  set_wf_named "${file}" NETWORK "${HARDHAT_NETWORK_TESTS}"
  set_wf_named "${file}" HARDHAT_NETWORK "${HARDHAT_NETWORK_TESTS}"
}

patch_workflow_env "${root}/test-suite/values-test-suite-workflow-e2e.yaml"
patch_workflow_env "${root}/test-suite/values-test-suite-workflow-relayer-sdk-e2e.yaml"

# --- second host chain (Polygon Amoy on public RPC; only testnets combines this) ---
if [[ "${DEPLOY_POLYGON:-false}" == "true" ]]; then
  : "${POLYGON_HTTP:?}"
  : "${POLYGON_WS:?}"
  : "${POLYGON_CHAIN_ID:?}"
  : "${POLYGON_FINALITY_DEPTH:?}"
  : "${POLYGON_FINALITY_LAG:?}"
  echo "Patching Polygon overlays (chain ${POLYGON_CHAIN_ID} at ${POLYGON_HTTP})"

  phost="${root}/host-chain/values-host-contracts-polygon-e2e.yaml"
  set_named_env "${phost}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
  set_named_env "${phost}" ".scDeploy.env" CHAIN_ID "${POLYGON_CHAIN_ID}"
  set_named_env "${phost}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_9}"
  set_named_env "${phost}" ".scDeploy.env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
  # deployCommands' canonical-ProtocolConfig export must hit the ETH host RPC: literal, or CANONICAL_RPC_URL from the Secret.
  if rpc_from_secret; then
    set_named_env_secret "${phost}" ".scDeploy.env" RPC_URL polygon-rpc-url
    set_named_env_secret "${phost}" ".scDeploy.env" CANONICAL_RPC_URL ethereum-rpc-url
    # $$ = literal $ for Go regexp replacement; the pod's shell expands ${CANONICAL_RPC_URL} later.
    # shellcheck disable=SC2016
    yq -i '.scDeploy.deployCommands[] |= sub("http://anvil-host-anvil-node:8545"; "$${CANONICAL_RPC_URL}")' "${phost}"
  else
    set_named_env "${phost}" ".scDeploy.env" RPC_URL "${POLYGON_HTTP}"
    HOST_HTTP="${HOST_HTTP}" yq -i \
      '.scDeploy.deployCommands[] |= sub("http://anvil-host-anvil-node:8545"; strenv(HOST_HTTP))' \
      "${phost}"
  fi

  padd="${root}/gateway-chain/values-gateway-add-host-chains-polygon-e2e.yaml"
  set_named_env "${padd}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
  set_named_env "${padd}" ".scDeploy.env" RPC_URL "${GATEWAY_HTTP}"
  set_named_env "${padd}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_0}"
  set_named_env "${padd}" ".scDeploy.env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
  set_named_env "${padd}" ".scDeploy.env" HOST_CHAIN_CHAIN_ID_0 "${POLYGON_CHAIN_ID}"

  plis="${root}/listener/values-listener-polygon-e2e.yaml"
  POLYGON_CHAIN_ID="${POLYGON_CHAIN_ID}" POLYGON_HTTP="${POLYGON_HTTP}" POLYGON_FINALITY_DEPTH="${POLYGON_FINALITY_DEPTH}" yq -i '
    .listeners[0].config.blockchain.chain_id = env(POLYGON_CHAIN_ID) |
    .listeners[0].config.blockchain.rpc_url = strenv(POLYGON_HTTP) |
    .listeners[0].config.blockchain.finality_depth = env(POLYGON_FINALITY_DEPTH)
  ' "${plis}"
  if rpc_from_secret; then
    yq -i '.listeners[0].config.blockchain.rpc_url = "set-by-APP_BLOCKCHAIN__RPC_URL"' "${plis}"
    set_named_env_secret "${plis}" ".env" APP_BLOCKCHAIN__RPC_URL polygon-rpc-url
  fi

  # Patch whichever of the "host"/"polygon" chains[] entries each coprocessor overlay carries.
  for f in "${root}/coprocessor/values-coprocessor-polygon-e2e.yaml" \
           "${root}/coprocessor/values-coprocessor-polygon-consumer-e2e.yaml" \
           "${root}/coprocessor/values-coprocessor-poller-polygon-e2e.yaml"; do
    HOST_HTTP="${HOST_HTTP}" HOST_WS="${HOST_WS}" HOST_CHAIN_ID="${HOST_CHAIN_ID}" \
    POLYGON_HTTP="${POLYGON_HTTP}" POLYGON_WS="${POLYGON_WS}" POLYGON_CHAIN_ID="${POLYGON_CHAIN_ID}" yq -i '
      .commonConfig.canonicalProtocolConfigChainId = strenv(HOST_CHAIN_ID) |
      (.chains[] | select(.name == "host") | .chainId) = strenv(HOST_CHAIN_ID) |
      (.chains[] | select(.name == "host") | .httpUrl) = strenv(HOST_HTTP) |
      (.chains[] | select(.name == "host") | .wsUrl) = strenv(HOST_WS) |
      (.chains[] | select(.name == "polygon") | .chainId) = strenv(POLYGON_CHAIN_ID) |
      (.chains[] | select(.name == "polygon") | .httpUrl) = strenv(POLYGON_HTTP) |
      (.chains[] | select(.name == "polygon") | .wsUrl) = strenv(POLYGON_WS)
    ' "${f}"
  done
  ppoller="${root}/coprocessor/values-coprocessor-poller-polygon-e2e.yaml"
  POLYGON_HTTP="${POLYGON_HTTP}" POLYGON_WS="${POLYGON_WS}" yq -i '
    .commonConfig.hostChainHttpUrl = strenv(POLYGON_HTTP) |
    .commonConfig.hostChainWsUrl = strenv(POLYGON_WS)
  ' "${ppoller}"
  if rpc_from_secret; then
    for f in "${root}/coprocessor/values-coprocessor-polygon-e2e.yaml" \
             "${root}/coprocessor/values-coprocessor-polygon-consumer-e2e.yaml" "${ppoller}"; do
      yq -e '.chains[] | select(.name == "host")' "${f}" >/dev/null 2>&1 && set_chain_urls_secret "${f}" host ethereum-rpc-url ethereum-rpc-ws-url
      set_chain_urls_secret "${f}" polygon polygon-rpc-url polygon-rpc-ws-url
    done
    yq -i '.commonConfig.hostChainHttpUrl = "unused-on-testnets" | .commonConfig.hostChainWsUrl = "unused-on-testnets"' "${ppoller}"
  fi
  set_poller_flag "${ppoller}" "--seed-start-block" "${POLLER_SEED_START_BLOCK}"
  set_poller_flag "${ppoller}" "--finality-lag" "${POLYGON_FINALITY_LAG}"

  pkms="${root}/kms-connector/values-kms-connector-polygon-e2e.yaml"
  POLYGON_HTTP="${POLYGON_HTTP}" POLYGON_CHAIN_ID="${POLYGON_CHAIN_ID}" yq -i '
    .commonConfig.polygonUrl = strenv(POLYGON_HTTP) |
    .commonConfig.polygonChainId = strenv(POLYGON_CHAIN_ID)
  ' "${pkms}"
  if rpc_from_secret; then
    set_named_env_secret "${kms}" ".commonConfig.env" RPC_POLYGON_URL polygon-rpc-url
    # shellcheck disable=SC2016  # literal for k8s env / shell expansion inside the pod
    yq -i '.commonConfig.polygonUrl = "$(RPC_POLYGON_URL)"' "${pkms}"
  fi

  # The Polygon DAG picks its network via `-n polygonAmoy`, so no NETWORK override here.
  pwf="${root}/test-suite/values-test-suite-workflow-polygon-e2e.yaml"
  patch_workflow_common "${pwf}" "${POLYGON_HTTP}" polygon-rpc-url
  set_wf_named "${pwf}" CHAIN_ID_HOST "${POLYGON_CHAIN_ID}"
fi

echo "Patched ${CHAIN_MODE} values overlays."
