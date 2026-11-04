#!/usr/bin/env bash
# Relayer migrate, relayer, relayer-http, and idle test-suite (common chart).
# Usage: deploy-relayer.sh <migrate|relayer|relayer-http|test-suite>
# Env: NAMESPACE, COMMON_CHART, COMMON_CHART_VERSION, TAGS_JSON,
# NB_KMS_CORE, DEPLOY_POLYGON, POLYGON_HTTP, POLYGON_CHAIN_ID, RPC_SECRET_NAME (relayer only),
# HOST_CHAIN_ID (relayer-http only).
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source-path=SCRIPTDIR
# shellcheck source=../lib.sh
source "${script_dir}/../lib.sh"

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
    # The relayer seeds /v2/keyurl at startup and exits if it cannot, and its poller
    # reads the host chain at the FINALIZED block (relayer/src/host/keyurl_poller.rs),
    # so on Sepolia getCrsMaterials reverts CrsNotGenerated for ~20 min after
    # ActivateCrs lands at head. Without gating here the e2e suites open against a
    # CrashLoopBackOff pod and every test fails on ECONNREFUSED to relayer:3000,
    # which looks exactly like a product regression. Anvil clears this in seconds.
    # Only meaningful because values-relayer-e2e.yaml sets a readinessProbe: with no
    # probe a pod is Ready as soon as the container starts and this returns at once.
    # rollout status also gives up the instant the Deployment reports
    # ProgressDeadlineExceeded, whatever --timeout says, and the common chart leaves
    # that at the 600s default - so the gate failed at 10m against a measured 21m
    # wait. Widen it past the timeout below; progressDeadlineSeconds is not part of
    # the pod template, so patching it starts no new rollout.
    kubectl patch deployment relayer -n "${NAMESPACE}" \
      -p '{"spec":{"progressDeadlineSeconds":2400}}'
    kubectl rollout status deployment/relayer -n "${NAMESPACE}" --timeout=35m
    # Publish HTTP :3000 on the zws-dev tailnet (IngressClass tailscale). Metrics
    # and the admin endpoint stay ClusterIP-only. Sets RELAYER_TS_URL for the
    # deploy summary / PR comment (fhevm-internal#2000).
    bash "${script_dir}/expose-relayer-tailscale.sh"
    ;;
  relayer-http)
    # The config is the local stack's template with the overlay's preview-only
    # fields merged over it, then filled in from the deploy as
    # renderRelayerHttpConfig (test-suite/fhevm/src/generate/config.ts) does.
    # One endpoint per party from the kms-connector-http ConfigMap that
    # deploy-kms-connector.sh publishes: its proxy URL, and as audience the
    # tx-sender address that proxy accepts (the lists are aligned by party).
    endpoints=$(kubectl get configmap kms-connector-http -n "${NAMESPACE}" -o json | jq -c '
      .data | [(."endpoint-urls" // "" | split(",")), (."tx-sender-addresses" // "" | split(","))]
      | transpose | to_entries | map({name: "kms_\(.key + 1)", url: .value[0], audience: .value[1]})')
    if ! jq -e 'length > 0 and all(.[]; .url and .audience)' <<<"${endpoints}" >/dev/null; then
      echo "::error::kms-connector-http endpoint-urls and tx-sender-addresses are empty or not one per party" >&2
      exit 1
    fi
    n=$(jq length <<<"${endpoints}")
    protocol_config=$(kubectl get configmap host-sc-addresses -n "${NAMESPACE}" -o jsonpath='{.data.protocol_config\.address}')
    require_nonempty "${protocol_config}" "host-sc-addresses[protocol_config.address] is empty"
    require_nonempty "${HOST_CHAIN_ID:-}" "HOST_CHAIN_ID is not set"
    chains="[${HOST_CHAIN_ID}]"
    if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
      chains="[${HOST_CHAIN_ID}, ${POLYGON_CHAIN_ID:-80002}]"
    fi
    # Same quorums as the relayer: public decrypt t+1 matching answers, user
    # decrypt n-t shares. FhevmSig is signed for the first host chain, as the
    # local stack does.
    public_t=$(kms_majority "${n}")
    user_t=$(kms_reconstruct "${n}")
    http_values=$(mktemp --suffix=.yaml)
    cp ci/preview-env/relayer/values-relayer-http-e2e.yaml "${http_values}"
    TEMPLATE=test-suite/fhevm/templates/config/relayer-http.yaml ENDPOINTS="${endpoints}" \
      CHAINS="${chains}" CHAIN_ID="${HOST_CHAIN_ID}" PROTOCOL_CONFIG="${protocol_config}" \
      PUBLIC_T="${public_t}" USER_T="${user_t}" yq -i '
      .configmapFiles.files."config.yaml" |= (load(strenv(TEMPLATE)) * . |
        .http.supported_chain_ids = env(CHAINS) |
        .kms_aggregator.public_decrypt.threshold = env(PUBLIC_T) |
        .kms_aggregator.user_decrypt.threshold = env(USER_T) |
        .kms_aggregator.auth.chain_id = env(CHAIN_ID) |
        .kms_aggregator.auth.protocol_config = strenv(PROTOCOL_CONFIG) |
        .kms_aggregator.endpoints = env(ENDPOINTS)
      )' "${http_values}"
    echo "relayer-http: ${n} endpoints, public/user thresholds ${public_t}/${user_t}, chains ${chains}"
    # No dependency at startup (/healthz is ready as soon as it serves), so a
    # short wait is enough.
    helm upgrade --install relayer-http "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
      -n "${NAMESPACE}" -f "${http_values}" \
      --set-string "image.tag=$(jq -r .relayer_http <<<"${TAGS_JSON}")" \
      --wait --timeout=5m
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
