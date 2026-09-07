#!/usr/bin/env bash
# Install one kms-connector release per KMS party.
# Env: NAMESPACE, NB_KMS_CORE, DEPLOY_POLYGON, OBSERVABILITY, OTLP_ENDPOINT,
# KMS_CONNECTOR_CHART, TAGS_JSON.
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source=lib.sh
source "${script_dir}/lib.sh"

decryption=$(kubectl get configmap gw-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.decryption\.address}')
gateway_config=$(kubectl get configmap gw-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.gateway_config\.address}')
kms_generation=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.kms_generation\.address}')
protocol_config=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.protocol_config\.address}')
acl=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.acl\.address}')
require_nonempty "${decryption}" "gw-sc-addresses[decryption.address] is empty"
require_nonempty "${gateway_config}" "gw-sc-addresses[gateway_config.address] is empty"
require_nonempty "${kms_generation}" "host-sc-addresses[kms_generation.address] is empty"
require_nonempty "${protocol_config}" "host-sc-addresses[protocol_config.address] is empty"
require_nonempty "${acl}" "host-sc-addresses[acl.address] is empty"

polygon_args=()
if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
  polygon_acl=$(kubectl get configmap polygon-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.acl\.address}')
  require_nonempty "${polygon_acl}" "polygon-sc-addresses[acl.address] is empty"
  polygon_args=(-f ci/preview-env/kms-connector/values-kms-connector-polygon-e2e.yaml
    --set-string "commonConfig.polygonContractAddresses.acl=${polygon_acl}")
fi

tracing=()
if [[ "${OBSERVABILITY}" == "true" ]]; then
  tracing=(
    --set "commonConfig.tracing.enabled=true"
    --set-string "commonConfig.tracing.endpoint=${OTLP_ENDPOINT}"
  )
fi

for i in $(seq 1 "${NB_KMS_CORE}"); do
  helm upgrade --install "kms-connector-${i}" "${KMS_CONNECTOR_CHART}" \
    -n "${NAMESPACE}" -f ci/preview-env/kms-connector/values-kms-connector-e2e.yaml \
    "${polygon_args[@]}" \
    --set-string "commonConfig.gatewayContractAddresses.decryption=${decryption}" \
    --set-string "commonConfig.gatewayContractAddresses.gatewayConfig=${gateway_config}" \
    --set-string "commonConfig.ethereumContractAddresses.kmsGeneration=${kms_generation}" \
    --set-string "commonConfig.ethereumContractAddresses.acl=${acl}" \
    --set-string "commonConfig.ethereumContractAddresses.protocolConfig=${protocol_config}" \
    --set-string "commonConfig.databaseUrl=postgresql://zama:zama@postgres-connector-${i}:5432/connector" \
    --set-string "kmsConnectorKmsWorker.config.kmsCoreEndpoints=http://kms-core-${i}-core-${i}:50100" \
    --set-string "kmsConnectorTxSender.wallet.secret.name=kms-connector-tx-sender-${i}" \
    --set-string "kmsConnectorDbMigration.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/db-migration" \
    --set-string "kmsConnectorDbMigration.image.tag=$(jq -r .kms_connector_db_migration <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorGwListener.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/gw-listener" \
    --set-string "kmsConnectorGwListener.image.tag=$(jq -r .kms_connector_gw_listener <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorKmsWorker.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/kms-worker" \
    --set-string "kmsConnectorKmsWorker.image.tag=$(jq -r .kms_connector_kms_worker <<<"${TAGS_JSON}")" \
    --set-string "kmsConnectorTxSender.image.name=hub.zama.org/ghcr/zama-ai/fhevm/kms-connector/tx-sender" \
    --set-string "kmsConnectorTxSender.image.tag=$(jq -r .kms_connector_tx_sender <<<"${TAGS_JSON}")" \
    "${tracing[@]}"
done
sleep 60
