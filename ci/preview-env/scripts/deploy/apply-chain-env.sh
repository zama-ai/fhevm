#!/usr/bin/env bash
# Patch preview-env Helm values in the working tree.
# External chain modes (blockchain-dev | testnets) rewrite RPC/mnemonic overlays.
# FHE_PARAMS / KEYGEN other than the values' default (default / insecure) patch
# the keygen values. Anvil with the default is a no-op.
set -euo pipefail

if [[ "${EXTERNAL_CHAINS:-false}" != "true" && "${FHE_PARAMS:?}" == "default" && "${KEYGEN:?}" == "insecure" ]]; then
  echo "Anvil mode: leaving values overlays unchanged."
  exit 0
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

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

# The values hold Default params. The insecure keygen and Test params fit the
# trigger script's 15 min waits; the four-party Default DKG needs hours.
keygen="${root}/host-chain/values-host-trigger-keygen-e2e.yaml"
if [[ "${FHE_PARAMS}" == "test" ]]; then
  set_named_env "${keygen}" ".scDeploy.env" KEYGEN_PARAMS_TYPE 1
elif [[ "${KEYGEN}" == "secure" ]]; then
  set_named_env "${keygen}" ".scDeploy.env" KEYGEN_WAIT_TIMEOUT_MS "${KEYGEN_WAIT_TIMEOUT_MS:-14400000}"
  set_named_env "${keygen}" ".scDeploy.env" CRSGEN_WAIT_TIMEOUT_MS "${CRSGEN_WAIT_TIMEOUT_MS:-3600000}"
fi
if [[ "${EXTERNAL_CHAINS:-false}" != "true" ]]; then
  echo "Anvil: fhe_params=${FHE_PARAMS} keygen=${KEYGEN}."
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

set_named_env "${keygen}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
if rpc_from_secret; then set_named_env_secret "${keygen}" ".scDeploy.env" RPC_URL ethereum-rpc-url; else set_named_env "${keygen}" ".scDeploy.env" RPC_URL "${HOST_HTTP}"; fi
set_named_env "${keygen}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_9}"
set_named_env "${keygen}" ".scDeploy.env" CHAIN_ID "${HOST_CHAIN_ID}"
if [[ "${CHAIN_MODE}" == "testnets" && ! ( "${FHE_PARAMS}" == "default" && "${KEYGEN}" == "secure" ) ]]; then
  # gw-listener pins its Ethereum reads to the FINALIZED block (BlockId::finalized() in
  # kms-connector/crates/gw-listener/src/core/ethereum.rs - not configurable), and Sepolia
  # finalizes ~14 min behind head. Keygen needs two such round trips (PrepKeygenRequest ->
  # response tx -> KeygenRequest -> response tx); measured end to end on Sepolia:
  # PrepKeygenRequest 19:55:12 -> ActivateKey 20:38:24, i.e. 43 min. 60m leaves headroom
  # for finality degrading further. Crsgen is a single round trip, so ~25 min; 40m keeps
  # a genuine failure there from burning the full keygen budget.
  # KEYGEN_TIMEOUT (resolve-chain.sh) must exceed their sum - both waits share one pod.
  set_named_env "${keygen}" ".scDeploy.env" KEYGEN_WAIT_TIMEOUT_MS "${KEYGEN_WAIT_TIMEOUT_MS:-3600000}"
  set_named_env "${keygen}" ".scDeploy.env" CRSGEN_WAIT_TIMEOUT_MS "${CRSGEN_WAIT_TIMEOUT_MS:-2400000}"
fi

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
         "${root}/coprocessor/values-coprocessor-poller-e2e.yaml" \
         "${root}/coprocessor/values-coprocessor-proxy-e2e.yaml"; do
  HOST_CHAIN_ID="${HOST_CHAIN_ID}" yq -i \
    '.commonConfig.canonicalProtocolConfigChainId = strenv(HOST_CHAIN_ID)' \
    "${f}"
done
GATEWAY_WS="${GATEWAY_WS}" yq -i '.commonConfig.gatewayUrl.value = strenv(GATEWAY_WS)' \
  "${root}/coprocessor/values-coprocessor-e2e.yaml"
# v0.14.2+ tx-sender fails fast on a ws:// URL ("Gateway URL is not usable by this
# build"); commonConfig stays ws:// for the gw-listener. Chart 0.13.8 ignores
# txSender.config.gatewayUrl and only renders the template --gateway-url flag, so
# the HTTP URL is also the extraArgs entry (the template flag is disabled). The
# BCS overlay clears that arg and pins the v0.14.1 sender back to ws://.
GATEWAY_HTTP="${GATEWAY_HTTP}" yq -i '
  .txSender.config.gatewayUrl.value = strenv(GATEWAY_HTTP) |
  .txSender.extraArgs = ["--gateway-url=" + strenv(GATEWAY_HTTP)]
' "${root}/coprocessor/values-coprocessor-e2e.yaml"
GATEWAY_WS="${GATEWAY_WS}" yq -i '.txSender.config.gatewayUrl.value = strenv(GATEWAY_WS)' \
  "${root}/coprocessor/values-coprocessor-bcs-e2e.yaml"
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
set_kms_host_chain() {
  # $1 file, $2 entry name, $3 url, $4 chain id
  # Chart 2.x reads hostChains.<name>. Chart 1.5.3 (v0.14.2) reads the flat
  # ethereumUrl / ethereumChainId / polygonUrl / polygonChainId keys.
  local file="$1" name="$2" url="$3" chain_id="$4"
  NAME="${name}" URL="${url}" CHAIN_ID="${chain_id}" yq -i '
    .commonConfig.hostChains[strenv(NAME)].url = strenv(URL) |
    .commonConfig.hostChains[strenv(NAME)].chainId = strenv(CHAIN_ID)
  ' "${file}"
  case "${name}" in
    ethereum)
      URL="${url}" CHAIN_ID="${chain_id}" yq -i '
        .commonConfig.ethereumUrl = strenv(URL) |
        .commonConfig.ethereumChainId = strenv(CHAIN_ID)
      ' "${file}"
      ;;
    polygon)
      URL="${url}" CHAIN_ID="${chain_id}" yq -i '
        .commonConfig.polygonUrl = strenv(URL) |
        .commonConfig.polygonChainId = strenv(CHAIN_ID)
      ' "${file}"
      ;;
  esac
}
GATEWAY_HTTP="${GATEWAY_HTTP}" GATEWAY_CHAIN_ID="${GATEWAY_CHAIN_ID}" yq -i '
  .commonConfig.gatewayUrl = strenv(GATEWAY_HTTP) |
  .commonConfig.gatewayChainId = strenv(GATEWAY_CHAIN_ID)
' "${kms}"
set_kms_host_chain "${kms}" ethereum "${HOST_HTTP}" "${HOST_CHAIN_ID}"
if rpc_from_secret; then
  set_named_env_secret "${kms}" ".commonConfig.env" RPC_ETH_URL ethereum-rpc-url
  # shellcheck disable=SC2016  # literal for k8s env / shell expansion inside the pod
  set_kms_host_chain "${kms}" ethereum '$(RPC_ETH_URL)' "${HOST_CHAIN_ID}"
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

# --- extra host chains (Polygon Amoy / BSC testnet on public RPC; only testnets combines these) ---
# Every extra host chain uses the same overlay set, named after its slot (polygon, bnb):
# values-*-<slot>-e2e.yaml, chains[] entry <slot>, Secret keys <slot>-rpc-url / <slot>-rpc-ws-url,
# kms-connector hostChains.<slot> + RPC_<SLOT>_URL. $1 slot, $2 display name, then http, ws,
# chain id, listener finality depth, poller finality lag.
patch_extra_host_chain() {
  local slot="$1" label="$2" http="$3" ws="$4" chain_id="$5" finality_depth="$6" finality_lag="$7"
  local rpc_key="${slot}-rpc-url" rpc_ws_key="${slot}-rpc-ws-url"
  local kms_env
  kms_env="RPC_$(tr '[:lower:]' '[:upper:]' <<<"${slot}")_URL"
  echo "Patching ${label} overlays (chain ${chain_id} at ${http})"

  local xhost="${root}/host-chain/values-host-contracts-${slot}-e2e.yaml"
  set_named_env "${xhost}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
  set_named_env "${xhost}" ".scDeploy.env" CHAIN_ID "${chain_id}"
  set_named_env "${xhost}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_9}"
  set_named_env "${xhost}" ".scDeploy.env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
  # deployCommands' canonical-ProtocolConfig export must hit the ETH host RPC: literal, or CANONICAL_RPC_URL from the Secret.
  if rpc_from_secret; then
    set_named_env_secret "${xhost}" ".scDeploy.env" RPC_URL "${rpc_key}"
    set_named_env_secret "${xhost}" ".scDeploy.env" CANONICAL_RPC_URL ethereum-rpc-url
    # $$ = literal $ for Go regexp replacement; the pod's shell expands ${CANONICAL_RPC_URL} later.
    # shellcheck disable=SC2016
    yq -i '.scDeploy.deployCommands[] |= sub("http://anvil-host-anvil-node:8545"; "$${CANONICAL_RPC_URL}")' "${xhost}"
  else
    set_named_env "${xhost}" ".scDeploy.env" RPC_URL "${http}"
    HOST_HTTP="${HOST_HTTP}" yq -i \
      '.scDeploy.deployCommands[] |= sub("http://anvil-host-anvil-node:8545"; strenv(HOST_HTTP))' \
      "${xhost}"
  fi

  local xadd="${root}/gateway-chain/values-gateway-add-host-chains-${slot}-e2e.yaml"
  set_named_env "${xadd}" ".scDeploy.env" MNEMONIC "${MNEMONIC}"
  set_named_env "${xadd}" ".scDeploy.env" RPC_URL "${GATEWAY_HTTP}"
  set_named_env "${xadd}" ".scDeploy.env" DEPLOYER_PRIVATE_KEY "${DEPLOYER_KEY_0}"
  set_named_env "${xadd}" ".scDeploy.env" CHAIN_ID_GATEWAY "${GATEWAY_CHAIN_ID}"
  set_named_env "${xadd}" ".scDeploy.env" HOST_CHAIN_CHAIN_ID_0 "${chain_id}"

  local xlis="${root}/listener/values-listener-${slot}-e2e.yaml"
  X_CHAIN_ID="${chain_id}" X_HTTP="${http}" X_FINALITY_DEPTH="${finality_depth}" yq -i '
    .listeners[0].config.blockchain.chain_id = env(X_CHAIN_ID) |
    .listeners[0].config.blockchain.rpc_url = strenv(X_HTTP) |
    .listeners[0].config.blockchain.finality_depth = env(X_FINALITY_DEPTH)
  ' "${xlis}"
  if rpc_from_secret; then
    yq -i '.listeners[0].config.blockchain.rpc_url = "set-by-APP_BLOCKCHAIN__RPC_URL"' "${xlis}"
    set_named_env_secret "${xlis}" ".env" APP_BLOCKCHAIN__RPC_URL "${rpc_key}"
  fi

  # Each coprocessor overlay carries only the <slot> chains[] entry (deploy-coprocessor.sh
  # takes the seed's `host` entry from the base file patched above).
  local xpoller="${root}/coprocessor/values-coprocessor-poller-${slot}-e2e.yaml"
  local xcopro=("${root}/coprocessor/values-coprocessor-${slot}-e2e.yaml"
                "${root}/coprocessor/values-coprocessor-${slot}-consumer-e2e.yaml"
                "${xpoller}")
  local f
  for f in "${xcopro[@]}"; do
    HOST_CHAIN_ID="${HOST_CHAIN_ID}" SLOT="${slot}" X_HTTP="${http}" X_WS="${ws}" X_CHAIN_ID="${chain_id}" yq -i '
      .commonConfig.canonicalProtocolConfigChainId = strenv(HOST_CHAIN_ID) |
      (.chains[] | select(.name == strenv(SLOT)) | .chainId) = strenv(X_CHAIN_ID) |
      (.chains[] | select(.name == strenv(SLOT)) | .httpUrl) = strenv(X_HTTP) |
      (.chains[] | select(.name == strenv(SLOT)) | .wsUrl) = strenv(X_WS)
    ' "${f}"
  done
  X_HTTP="${http}" X_WS="${ws}" yq -i '
    .commonConfig.hostChainHttpUrl = strenv(X_HTTP) |
    .commonConfig.hostChainWsUrl = strenv(X_WS)
  ' "${xpoller}"
  if rpc_from_secret; then
    for f in "${xcopro[@]}"; do
      set_chain_urls_secret "${f}" "${slot}" "${rpc_key}" "${rpc_ws_key}"
    done
    yq -i '.commonConfig.hostChainHttpUrl = "unused-on-testnets" | .commonConfig.hostChainWsUrl = "unused-on-testnets"' "${xpoller}"
  fi
  set_poller_flag "${xpoller}" "--seed-start-block" "${POLLER_SEED_START_BLOCK}"
  set_poller_flag "${xpoller}" "--finality-lag" "${finality_lag}"

  local xkms="${root}/kms-connector/values-kms-connector-${slot}-e2e.yaml"
  set_kms_host_chain "${xkms}" "${slot}" "${http}" "${chain_id}"
  if rpc_from_secret; then
    set_named_env_secret "${kms}" ".commonConfig.env" "${kms_env}" "${rpc_key}"
    set_kms_host_chain "${xkms}" "${slot}" "\$(${kms_env})" "${chain_id}"
  fi

  # The DAG picks its hardhat network via `-n <network>`, so no NETWORK override here.
  local xwf="${root}/test-suite/values-test-suite-workflow-${slot}-e2e.yaml"
  patch_workflow_common "${xwf}" "${http}" "${rpc_key}"
  set_wf_named "${xwf}" CHAIN_ID_HOST "${chain_id}"
}

if [[ "${DEPLOY_POLYGON:-false}" == "true" ]]; then
  patch_extra_host_chain polygon Polygon "${POLYGON_HTTP:?}" "${POLYGON_WS:?}" "${POLYGON_CHAIN_ID:?}" \
    "${POLYGON_FINALITY_DEPTH:?}" "${POLYGON_FINALITY_LAG:?}"
fi
if [[ "${DEPLOY_BNB:-false}" == "true" ]]; then
  patch_extra_host_chain bnb BNB "${BNB_HTTP:?}" "${BNB_WS:?}" "${BNB_CHAIN_ID:?}" \
    "${BNB_FINALITY_DEPTH:?}" "${BNB_FINALITY_LAG:?}"
fi

echo "Patched ${CHAIN_MODE} values overlays."
