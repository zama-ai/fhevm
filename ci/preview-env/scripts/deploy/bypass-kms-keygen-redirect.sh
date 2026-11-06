#!/usr/bin/env bash
# KEYGEN=insecure, after trigger-keygen: point each kms-worker straight at its
# kms-core and uninstall kms-insecure-keygen-redirect-<i>. Only keygen needs the
# redirect, and it is a bottleneck for every other kms-worker call. A later
# insecure keygen needs deploy-kms-connector.sh re-run to restore it.
# Env: NAMESPACE, NB_KMS_CORE, KMS_CONNECTOR_CHART, DEPLOY_POLYGON.
set -euo pipefail

# Re-apply each release's values, re-removing the chart's empty polygon entry.
polygon_args=()
if [[ "${DEPLOY_POLYGON:-false}" != "true" ]]; then
  polygon_args=(--set "commonConfig.hostChains.polygon=null")
fi
values_file=$(mktemp --suffix=.yaml)
trap 'rm -f "${values_file}"' EXIT

for i in $(seq 1 "${NB_KMS_CORE}"); do
  helm get values "kms-connector-${i}" -n "${NAMESPACE}" -o yaml > "${values_file}"
  helm upgrade "kms-connector-${i}" "${KMS_CONNECTOR_CHART}" -n "${NAMESPACE}" --reset-values \
    -f "${values_file}" \
    "${polygon_args[@]}" \
    --set-string "kmsConnectorKmsWorker.config.kmsCoreEndpoints=http://kms-core-${i}-core-${i}:50100"
  kubectl rollout status "deploy/kms-connector-${i}-kms-connector-kms-worker" -n "${NAMESPACE}" --timeout=5m
  helm uninstall "kms-insecure-keygen-redirect-${i}" -n "${NAMESPACE}"
done
