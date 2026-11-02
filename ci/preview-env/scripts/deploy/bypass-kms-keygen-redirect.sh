#!/usr/bin/env bash
# KEYGEN=insecure, after trigger-keygen: point each kms-worker straight at its
# kms-core and uninstall kms-insecure-keygen-redirect-<i>. Only keygen needs the
# redirect, and it is a bottleneck for every other kms-worker call. A later
# insecure keygen needs deploy-kms-connector.sh re-run to restore it.
# Env: NAMESPACE, NB_KMS_CORE, KMS_CONNECTOR_CHART.
set -euo pipefail

for i in $(seq 1 "${NB_KMS_CORE}"); do
  helm upgrade "kms-connector-${i}" "${KMS_CONNECTOR_CHART}" -n "${NAMESPACE}" --reuse-values \
    --set-string "kmsConnectorKmsWorker.config.kmsCoreEndpoints=http://kms-core-${i}-core-${i}:50100"
  kubectl rollout status "deploy/kms-connector-${i}-kms-connector-kms-worker" -n "${NAMESPACE}" --timeout=5m
  helm uninstall "kms-insecure-keygen-redirect-${i}" -n "${NAMESPACE}"
done
