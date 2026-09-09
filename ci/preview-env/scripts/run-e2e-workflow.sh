#!/usr/bin/env bash
# Deploy (or re-deploy) the automated e2e Argo Workflows and wait for them.
# Used twice on the RFC-021 path: once during DryRunStarted, once after cutover.
# Env: NAMESPACE, E2E_IMAGE, COMMON_CHART, COMMON_CHART_VERSION,
#      RELAYER_SDK_VERSION (optional), DEPLOY_POLYGON, E2E_PHASE (annotation).
set -euo pipefail

: "${NAMESPACE:?}"
: "${E2E_IMAGE:?}"
: "${COMMON_CHART:?}"
: "${COMMON_CHART_VERSION:?}"

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(cd "${script_dir}/.." && pwd)
E2E_PHASE="${E2E_PHASE:-default}"

patch_image() {
  yq -i \
    '(.additionalResources[] | select(.object.kind == "Workflow") | .object.spec.templates[] | select(.name == "run-test") | .script.image) = strenv(E2E_IMAGE)' \
    "$1"
}

# Force helm to re-apply Workflows on a second phase (identical values would
# otherwise be a no-op after we delete the completed Workflow objects).
patch_phase() {
  E2E_PHASE="${E2E_PHASE}" yq -i \
    '(.additionalResources[] | select(.object.kind == "Workflow") | .object.metadata.annotations["preview.zama.ai/e2e-phase"]) = strenv(E2E_PHASE)' \
    "$1"
}

kubectl delete workflow e2e-test-suite-fhevm-sdk e2e-test-suite-relayer-sdk e2e-test-suite-polygon \
  -n "${NAMESPACE}" --ignore-not-found

f1=$(mktemp --suffix=.yaml)
cp "${root}/test-suite/values-test-suite-workflow-e2e.yaml" "${f1}"
NS="${NAMESPACE}" yq -i \
  '(.additionalResources[] | select(.object.kind == "RoleBinding") | .object.subjects[0].namespace) = strenv(NS)' \
  "${f1}"
patch_image "${f1}"
patch_phase "${f1}"
echo "Deploying @fhevm/sdk workflow (image ${E2E_IMAGE}, phase=${E2E_PHASE})..."
helm upgrade --install test-suite-workflow "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
  -n "${NAMESPACE}" -f "${f1}"
bash "${script_dir}/wait-workflow.sh" e2e-test-suite-fhevm-sdk

if [[ -n "${RELAYER_SDK_VERSION:-}" ]]; then
  f2=$(mktemp --suffix=.yaml)
  cp "${root}/test-suite/values-test-suite-workflow-relayer-sdk-e2e.yaml" "${f2}"
  patch_image "${f2}"
  patch_phase "${f2}"
  yq -i \
    '(.additionalResources[] | select(.object.kind == "Workflow") | .object.spec.templates[] | select(.name == "run-test") | .script.env[] | select(.name == "RELAYER_SDK_VERSION") | .value) = strenv(RELAYER_SDK_VERSION)' \
    "${f2}"
  echo "Deploying @zama-fhe/relayer-sdk workflow (image ${E2E_IMAGE}, phase=${E2E_PHASE})..."
  helm upgrade --install test-suite-workflow-relayer "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
    -n "${NAMESPACE}" -f "${f2}"
  bash "${script_dir}/wait-workflow.sh" e2e-test-suite-relayer-sdk
else
  echo "RELAYER_SDK_VERSION empty - skipping the @zama-fhe/relayer-sdk suite."
fi

if [[ "${DEPLOY_POLYGON:-false}" == "true" ]]; then
  f3=$(mktemp --suffix=.yaml)
  cp "${root}/test-suite/values-test-suite-workflow-polygon-e2e.yaml" "${f3}"
  patch_image "${f3}"
  patch_phase "${f3}"
  echo "Deploying Polygon @fhevm/sdk workflow (image ${E2E_IMAGE}, phase=${E2E_PHASE})..."
  helm upgrade --install test-suite-workflow-polygon "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
    -n "${NAMESPACE}" -f "${f3}"
  bash "${script_dir}/wait-workflow.sh" e2e-test-suite-polygon
fi
