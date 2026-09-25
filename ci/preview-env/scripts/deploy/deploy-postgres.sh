#!/usr/bin/env bash
# Fan-out in-cluster Postgres via the common chart.
# Usage: deploy-postgres.sh <coprocessor|connector|listener|listener-polygon|relayer>
# Env: NAMESPACE, COMMON_CHART, COMMON_CHART_VERSION, NB_COPROCESSOR, NB_KMS_CORE.
set -euo pipefail

kind="${1:?kind}"
case "${kind}" in
  coprocessor)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "postgres-coprocessor-${i}" "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
        -n "${NAMESPACE}" -f ci/preview-env/coprocessor-infra/values-postgres-coprocessor-e2e.yaml \
        --set-string "fullnameOverride=postgres-coprocessor-${i}" --wait &
    done
    wait
    ;;
  connector)
    for i in $(seq 1 "${NB_KMS_CORE}"); do
      helm upgrade --install "postgres-connector-${i}" "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
        -n "${NAMESPACE}" -f ci/preview-env/kms-connector/values-postgres-connector-e2e.yaml \
        --set-string "fullnameOverride=postgres-connector-${i}" --wait &
    done
    wait
    ;;
  listener)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "postgres-listener-${i}" "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
        -n "${NAMESPACE}" -f ci/preview-env/listener/values-postgres-listener-e2e.yaml \
        --set-string "fullnameOverride=postgres-listener-${i}" --wait &
    done
    wait
    ;;
  listener-polygon)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "postgres-listener-polygon-${i}" "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
        -n "${NAMESPACE}" -f ci/preview-env/listener/values-postgres-listener-e2e.yaml \
        --set-string "fullnameOverride=postgres-listener-polygon-${i}" --wait &
    done
    wait
    ;;
  relayer)
    helm upgrade --install postgres-relayer "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
      -n "${NAMESPACE}" -f ci/preview-env/relayer/values-postgres-relayer-e2e.yaml --wait
    ;;
  *)
    echo "::error::unknown postgres kind '${kind}'" >&2
    exit 1
    ;;
esac
