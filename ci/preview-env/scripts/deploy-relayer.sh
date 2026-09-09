#!/usr/bin/env bash
# Relayer migrate, relayer, and idle test-suite (common chart).
# Usage: deploy-relayer.sh <migrate|relayer|test-suite>
# Env: NAMESPACE, COMMON_CHART, COMMON_CHART_VERSION, TAGS_JSON,
# NB_KMS_CORE, DEPLOY_POLYGON, POLYGON_HTTP, POLYGON_CHAIN_ID, RPC_SECRET_NAME (relayer only).
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
source "${script_dir}/lib.sh"

kind="${1:?kind}"
case "${kind}" in
  migrate)
    helm upgrade --install relayer-migrate "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
      -n "${NAMESPACE}" -f ci/preview-env/relayer/values-relayer-migrate-e2e.yaml \
      --set-string "image.tag=$(jq -r .relayer_migrate <<<"${TAGS_JSON}")" \
      --wait --wait-for-jobs --timeout=5m
    ;;
  relayer)
    # user_decrypt share reconstruction threshold MUST match the deployed
    # KMS party count (n-t). values-relayer-e2e.yaml is only a placeholder.
    reconstruct=$(kms_reconstruct "${NB_KMS_CORE}")
    relayer_values=$(mktemp --suffix=.yaml)
    cp ci/preview-env/relayer/values-relayer-e2e.yaml "${relayer_values}"
    UDST="${reconstruct}" yq -i \
      '.env += [{"name": "APP_GATEWAY__CONTRACTS__USER_DECRYPT_SHARES_THRESHOLD", "value": strenv(UDST)}]' \
      "${relayer_values}"
    echo "Relayer user_decrypt_shares_threshold=${reconstruct} (KMS parties: ${NB_KMS_CORE})"
    if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
      # Second host chain: the Polygon anvil URL, or the `rpc` Secret on testnets (RPC_SECRET_NAME set).
      if [[ -n "${RPC_SECRET_NAME:-}" ]]; then
        url_entry='{"name": "APP_HOST_CHAINS__1__URL", "valueFrom": {"secretKeyRef": {"name": "'"${RPC_SECRET_NAME}"'", "key": "polygon-rpc-url"}}}'
      else
        url_entry='{"name": "APP_HOST_CHAINS__1__URL", "value": "'"${POLYGON_HTTP:-http://anvil-host-polygon-anvil-node:8545}"'"}'
      fi
      POLYGON_ID="${POLYGON_CHAIN_ID:-80002}" URL_ENTRY="${url_entry}" yq -i '.env += [
        {"name": "APP_HOST_CHAINS__1__CHAIN_ID", "value": strenv(POLYGON_ID)},
        (strenv(URL_ENTRY) | fromjson),
        {"name": "APP_HOST_CHAINS__1__ACL_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "polygon-sc-addresses", "key": "acl.address"}}}
      ] | (.env[] | select(has("value")) | .value) style="double"' "${relayer_values}"
    fi
    helm upgrade --install relayer "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
      -n "${NAMESPACE}" -f "${relayer_values}" \
      --set-string "image.tag=$(jq -r .relayer <<<"${TAGS_JSON}")"
    ;;
  test-suite)
    helm upgrade --install test-suite "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
      -n "${NAMESPACE}" -f ci/preview-env/test-suite/values-test-suite-e2e.yaml \
      --set-string "image.tag=$(jq -r .test_suite <<<"${TAGS_JSON}")"
    ;;
  *)
    echo "::error::unknown relayer kind '${kind}'" >&2
    exit 1
    ;;
esac
