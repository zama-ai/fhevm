#!/usr/bin/env bash
# Per-party Crossplane S3 (coprocessor-infra-<i>).
# Env: NAMESPACE, NB_COPROCESSOR, COPROCESSOR_INFRA_CHART, COPROCESSOR_INFRA_CHART_VERSION.
set -euo pipefail

for i in $(seq 1 "${NB_COPROCESSOR}"); do
  helm upgrade --install "coprocessor-infra-${i}" "${COPROCESSOR_INFRA_CHART}" --version "${COPROCESSOR_INFRA_CHART_VERSION}" \
    -n "${NAMESPACE}" -f ci/preview-env/coprocessor-infra/values-coprocessor-infra-e2e.yaml \
    --set "coprocessorBucket.irsa.serviceAccountName=coprocessor-${i}" \
    --set "coprocessorBucket.configMapTargetRef.name=coprocessor-${i}" --wait &
done
wait
