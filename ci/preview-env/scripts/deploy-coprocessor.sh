#!/usr/bin/env bash
# Install BCS (and optional GCS) coprocessor releases.
# Env: NAMESPACE, NB_COPROCESSOR, BLUE_GREEN, DEPLOY_POLYGON, OBSERVABILITY,
# AUTOMATED_TESTS, COPROCESSOR_CHART, TAGS_JSON, BCS_IMAGE_TAG, OTLP_ENDPOINT,
# COPROC_WALLETS_JSON.
set -euo pipefail

polygon_args=()
if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
  polygon_args=(-f ci/preview-env/coprocessor/values-coprocessor-polygon-e2e.yaml)
fi
bcs_overlay=()
if [[ "${BLUE_GREEN}" == "true" ]]; then
  bcs_overlay=(-f ci/preview-env/coprocessor/values-coprocessor-bcs-e2e.yaml)
fi

party_common() {
  local i="$1" privkey="$2" suffix="${3:-}"
  common_args=(
    --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-${i}:5432"
    --set-string "hostListenerShared.serviceAccountName=coprocessor-${i}"
    --set-string "hostListenerPollerShared.serviceAccountName=coprocessor-${i}"
    --set-string "hostListenerCatchupOnlyShared.serviceAccountName=coprocessor-${i}"
    --set-string "hostListenerConsumerShared.serviceAccountName=coprocessor-${i}"
    --set-string "tfheWorker.serviceAccountName=coprocessor-${i}"
    --set-string "zkProofWorker.serviceAccountName=coprocessor-${i}"
    --set-string "snsWorker.serviceAccountName=coprocessor-${i}"
    --set "snsWorker.config.s3BucketName.valueFrom.configMapKeyRef.name=coprocessor-${i}"
    --set-string "hostListenerConsumerShared.config.brokerUrl.value=redis://coprocessor-redis-${i}-master:6379"
    --set-string "txSender.wallet.secret.value=${privkey}"
    --set-string "snsWorker.wallet.secret.value=${privkey}"
  )
  if [[ "${OBSERVABILITY}" == "true" ]]; then
    common_args+=(
      --set "commonConfig.tracing.enabled=true"
      --set-string "commonConfig.tracing.endpoint=${OTLP_ENDPOINT}"
      --set-string "gwListener.tracing.service=gw-listener-${i}${suffix}"
      --set-string "tfheWorker.tracing.service=tfhe-worker-${i}${suffix}"
      --set-string "zkProofWorker.tracing.service=zkproof-worker-${i}${suffix}"
      --set-string "snsWorker.tracing.service=sns-worker-${i}${suffix}"
      --set-string "txSender.tracing.service=tx-sender-${i}${suffix}"
      --set-string "hostListenerConsumerShared.tracing.service=host-listener-consumer-${i}${suffix}"
    )
  fi
}

image_tags() {
  local tag="$1"
  bcs_images=(
    --set-string "gwListener.image.tag=${tag}"
    --set-string "hostListenerShared.image.tag=${tag}"
    --set-string "hostListenerPollerShared.image.tag=${tag}"
    --set-string "hostListenerCatchupOnlyShared.image.tag=${tag}"
    --set-string "hostListenerConsumerShared.image.tag=${tag}"
    --set-string "snsWorker.image.tag=${tag}"
    --set-string "tfheWorker.image.tag=${tag}"
    --set-string "txSender.image.tag=${tag}"
    --set-string "zkProofWorker.image.tag=${tag}"
  )
}

for i in $(seq 1 "${NB_COPROCESSOR}"); do
  privkey=$(jq -r --argjson party "${i}" '.[] | select(.party == $party) | .privateKey' <<<"${COPROC_WALLETS_JSON}")
  if [[ -z "${privkey}" || "${privkey}" == "null" ]]; then
    echo "::error::no derived coprocessor wallet for party ${i} in coproc_wallets_json=${COPROC_WALLETS_JSON}"
    exit 1
  fi
  if [[ "${BLUE_GREEN}" == "true" ]]; then
    image_tags "${BCS_IMAGE_TAG}"
  else
    bcs_images=(
      --set-string "gwListener.image.tag=$(jq -r .coprocessor_gw_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerPollerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerCatchupOnlyShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerConsumerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "snsWorker.image.tag=$(jq -r .coprocessor_sns_worker <<<"${TAGS_JSON}")"
      --set-string "tfheWorker.image.tag=$(jq -r .coprocessor_tfhe_worker <<<"${TAGS_JSON}")"
      --set-string "txSender.image.tag=$(jq -r .coprocessor_tx_sender <<<"${TAGS_JSON}")"
      --set-string "zkProofWorker.image.tag=$(jq -r .coprocessor_zkproof_worker <<<"${TAGS_JSON}")"
    )
  fi
  party_common "${i}" "${privkey}"
  fleet_json=()
  if [[ "${BLUE_GREEN}" == "true" ]]; then
    fleet_json=(--set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-bcs\"}")
  fi
  helm upgrade --install "coprocessor-${i}" "${COPROCESSOR_CHART}" \
    -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-e2e.yaml \
    "${polygon_args[@]}" \
    "${bcs_overlay[@]}" \
    "${fleet_json[@]}" \
    --set-string "fullnameOverride=coprocessor-${i}" \
    --set-string "dbMigration.image.tag=$(jq -r .coprocessor_db_migration <<<"${TAGS_JSON}")" \
    "${common_args[@]}" \
    "${bcs_images[@]}"
done

if [[ "${BLUE_GREEN}" == "true" ]]; then
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    kubectl wait --for=condition=complete \
      "job/coprocessor-${i}-db-migration-1" -n "${NAMESPACE}" --timeout=180s \
      || kubectl wait --for=condition=complete \
        -l app=coprocessor-db-migration,app.kubernetes.io/name="coprocessor-${i}-db-migration" \
        job -n "${NAMESPACE}" --timeout=180s
  done
  # BCS zkproof (v0.14.0-7) snapshots host_chains once at process start.
  # helm applies the Deployment in parallel with db-migration, so a party
  # whose seed finishes a second later keeps chain_id = ANY('{}') forever.
  # Restart after seed so both parties see the host chain.
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    kubectl rollout restart "deploy/coprocessor-${i}-zkproof-worker" -n "${NAMESPACE}"
    kubectl rollout status "deploy/coprocessor-${i}-zkproof-worker" -n "${NAMESPACE}" --timeout=180s
  done
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    privkey=$(jq -r --argjson party "${i}" '.[] | select(.party == $party) | .privateKey' <<<"${COPROC_WALLETS_JSON}")
    party_common "${i}" "${privkey}" "-gcs"
    gcs_images=(
      --set-string "gwListener.image.tag=$(jq -r .coprocessor_gw_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerPollerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerCatchupOnlyShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerConsumerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "snsWorker.image.tag=$(jq -r .coprocessor_sns_worker <<<"${TAGS_JSON}")"
      --set-string "tfheWorker.image.tag=$(jq -r .coprocessor_tfhe_worker <<<"${TAGS_JSON}")"
      --set-string "txSender.image.tag=$(jq -r .coprocessor_tx_sender <<<"${TAGS_JSON}")"
      --set-string "zkProofWorker.image.tag=$(jq -r .coprocessor_zkproof_worker <<<"${TAGS_JSON}")"
      --set-string "upgradeController.image.tag=$(jq -r .coprocessor_tfhe_worker <<<"${TAGS_JSON}")"
      --set-string "consensusDetector.image.tag=$(jq -r .coprocessor_tfhe_worker <<<"${TAGS_JSON}")"
    )
    helm upgrade --install "coprocessor-${i}-gcs" "${COPROCESSOR_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-e2e.yaml \
      -f ci/preview-env/coprocessor/values-coprocessor-gcs-e2e.yaml \
      --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-gcs\"}" \
      --set-string "fullnameOverride=coprocessor-${i}-gcs" \
      --set-string "upgradeController.serviceAccountName=coprocessor-${i}" \
      --set-string "consensusDetector.serviceAccountName=coprocessor-${i}" \
      --set "snsWorker.config.s3BucketName.valueFrom.configMapKeyRef.name=coprocessor-${i}" \
      "${common_args[@]}" \
      "${gcs_images[@]}"
  done
  if [[ "${AUTOMATED_TESTS}" == "true" ]]; then
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      kubectl scale "deploy/coprocessor-${i}-gcs-consensus-detector" \
        -n "${NAMESPACE}" --replicas=0
    done
  fi
fi
sleep 60
