#!/usr/bin/env bash
# Install BCS (and optional GCS) coprocessor releases.
# Env: NAMESPACE, NB_COPROCESSOR, BLUE_GREEN, GPU, DEPLOY_POLYGON, OBSERVABILITY,
# AUTOMATED_TESTS, COPROCESSOR_CHART, TAGS_JSON, BCS_IMAGE_TAG, GPU_IMAGE_TAG,
# OTLP_ENDPOINT, COPROC_WALLETS_JSON, GCS_STACK_VERSION (pod label for the GCS
# fleet; optional).
set -euo pipefail

polygon_args=()
if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
  polygon_args=(-f ci/preview-env/coprocessor/values-coprocessor-polygon-e2e.yaml)
fi
bcs_overlay=()
if [[ "${BLUE_GREEN}" == "true" ]]; then
  bcs_overlay=(-f ci/preview-env/coprocessor/values-coprocessor-bcs-e2e.yaml)
fi
# GPU overlay for HEAD FHE workers (tfhe / sns / zkproof), when GPU=true:
#   preview-env-gpu + preview-env-e2e-tests
#     -> coprocessor-<i> (this release is HEAD)
#   preview-env-gpu + preview-env-blue-green + preview-env-e2e-tests
#     -> coprocessor-<i>-gcs (Green is HEAD)
# Blue (BCS, pinned CPU images) never takes the overlay or a cuda tag.
gpu_overlay=()
if [[ "${GPU:-false}" == "true" ]]; then
  gpu_overlay=(-f ci/preview-env/coprocessor/values-coprocessor-gpu-e2e.yaml)
  if [[ "${BLUE_GREEN}" == "true" ]]; then
    echo "GPU: Green FHE workers on coprocessor-gpu; Blue stays on the CPU pool"
  else
    echo "GPU: FHE workers on coprocessor-gpu"
  fi
fi
coproc_gpu_overlay=()
gcs_gpu_overlay=()
if [[ "${GPU:-false}" == "true" ]]; then
  if [[ "${BLUE_GREEN}" == "true" ]]; then
    gcs_gpu_overlay=("${gpu_overlay[@]}")
  else
    coproc_gpu_overlay=("${gpu_overlay[@]}")
  fi
fi

# tfhe / sns / zkproof share one published GPU tag. Do not derive it from
# resolve-tags: GPU images are built separately. Dispatch passes GPU_IMAGE_TAG
# from the workers_tag input.
if [[ "${GPU:-false}" == "true" && -z "${GPU_IMAGE_TAG:-}" ]]; then
  echo "::error::GPU_IMAGE_TAG is required when GPU=true (dispatch workers_tag)"
  exit 1
fi

worker_image_tag() {
  local key="$1"
  if [[ "${GPU:-false}" == "true" ]]; then
    printf '%s' "${GPU_IMAGE_TAG}"
    return
  fi
  jq -r ".${key}" <<<"${TAGS_JSON}"
}

# Chart 0.7.x stores tolerations as {enabled, items} and bakes the gateway URL
# into args. A list tolerations value overwrites that table and the template
# then crashes on .tolerations.enabled. The same chart never reads
# commonConfig.gatewayUrl, extraArgs, or txSender.wallet.
coprocessor_chart_is_legacy() {
  yq -e '.tfheWorker.tolerations | has("enabled")' "${COPROCESSOR_CHART}/values.yaml" >/dev/null 2>&1
}

ensure_legacy_secret() {
  local name="$1" key="$2" value="$3"
  if kubectl get secret "${name}" -n "${NAMESPACE}" >/dev/null 2>&1; then
    return 0
  fi
  kubectl create secret generic "${name}" -n "${NAMESPACE}" \
    --from-literal="${key}=${value}" >/dev/null
}

# Rewrites the overlay for a 0.7.x chart. Sets values_files.
prepare_legacy_coprocessor_values() {
  local party="$1" privkey="$2"
  local adapted legacy chart db_url host_tag
  adapted="$(mktemp)"
  legacy="$(mktemp)"
  chart="${COPROCESSOR_CHART}/values.yaml"
  db_url="postgresql://zama:zama@postgres-coprocessor-${party}:5432/fhevm_e2e"
  cp ci/preview-env/coprocessor/values-coprocessor-e2e.yaml "${adapted}"
  yq -i '
    (.. | select(type == "!!map" and has("tolerations") and (.tolerations | type == "!!seq"))).tolerations |= {"enabled": true, "items": .} |
    (.. | select(type == "!!map" and has("affinity") and (.affinity | type == "!!map") and (.affinity | has("nodeAffinity")) and ((.affinity | has("enabled")) | not))).affinity.enabled = true
  ' "${adapted}"
  # The chart's config job would pull ghcr.io kube-utils and write a minio URL.
  # Create the secrets it owns, then leave the job off.
  ensure_legacy_secret "coprocessor-db-url" "coprocessor-db-url" "${db_url}"
  ensure_legacy_secret "coprocessor-api-key" "coprocessor-api-key" "$(openssl rand -hex 16)"
  ensure_legacy_secret "coprocessor-key" "coprocessor.hex" "0x$(openssl rand -hex 32)"
  # db url must follow this party; the other two secrets stay once created.
  kubectl create secret generic coprocessor-db-url -n "${NAMESPACE}" \
    --from-literal="coprocessor-db-url=${db_url}" \
    --dry-run=client -o yaml | kubectl apply -f - >/dev/null
  CHART="${chart}" PRIV="${privkey}" CM="coprocessor-${party}" yq -n '
    .config.enabled = false |
    .hostListener.enabled = true |
    .hostListener.serviceAccountName = strenv(CM) |
    .hostListener.env = [
      {"name": "DATABASE_URL", "valueFrom": {"secretKeyRef": {"name": "coprocessor-db-url", "key": "coprocessor-db-url"}}},
      {"name": "TENANT_API_KEY", "valueFrom": {"secretKeyRef": {"name": "coprocessor-api-key", "key": "coprocessor-api-key"}}},
      {"name": "ETHEREUM_RPC_URL", "value": "ws://anvil-host-anvil-node:8545"},
      {"name": "ACL_CONTRACT_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "host-sc-addresses", "key": "acl.address"}}},
      {"name": "FHEVM_EXECUTOR_CONTRACT_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "host-sc-addresses", "key": "fhevm_executor.address"}}}
    ] |
    .gwListener.env = [
      {"name": "DATABASE_URL", "valueFrom": {"secretKeyRef": {"name": "coprocessor-db-url", "key": "coprocessor-db-url"}}},
      {"name": "INPUT_VERIFICATION_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "gw-sc-addresses", "key": "input_verification.address"}}},
      {"name": "KMS_GENERATION_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "gw-sc-addresses", "key": "kms_generation.address"}}}
    ] |
    .dbMigration.enabled = false |
    .gwListener.args = load(strenv(CHART)).gwListener.args |
    .txSender.args = load(strenv(CHART)).txSender.args |
    .txSender.env = load(strenv(CHART)).txSender.env |
    .tfheWorker.args = load(strenv(CHART)).tfheWorker.args |
    .snsWorker.args = load(strenv(CHART)).snsWorker.args |
    .snsWorker.env = load(strenv(CHART)).snsWorker.env + [
      {"name": "S3_BUCKET_NAME", "valueFrom": {"configMapKeyRef": {"name": strenv(CM), "key": "S3_BUCKET_NAME"}}}
    ]
  ' > "${legacy}"
  PRIV="${privkey}" yq -i '
    (.. | select(type == "!!str")) |= sub("ws://gateway-rpc-node:8548"; "ws://anvil-gateway-anvil-node:8546") |
    (.. | select(type == "!!str")) |= sub("gateway-sc-addresses"; "gw-sc-addresses") |
    (.txSender.env[] | select(.name == "TX_SENDER_PRIVATE_KEY").value) = strenv(PRIV) |
    .txSender.env += [{"name": "ETHEREUM_RPC_URL", "value": "http://anvil-host-anvil-node:8545"}] |
    .gwListener.env += [{"name": "CHAIN_ID", "value": "12345"}] |
    .tfheWorker.args[] |= sub("^--run-bg-worker=true$"; "--run-bg-worker") |
    .tfheWorker.args[] |= sub("^--run-server=true$"; "--run-server") |
    .tfheWorker.args[] |= sub("^--generate-fhe-keys=true$"; "--generate-fhe-keys") |
    .tfheWorker.args[] |= sub("^--disable-dcid-locking=true$"; "--disable-dcid-locking") |
    del(.tfheWorker.args[] | select(. == "--run-server=false" or . == "--generate-fhe-keys=false" or . == "--disable-dcid-locking=false")) |
    .snsWorker.args[] |= sub("^--enable-compssion=true$"; "--enable-compression") |
    .snsWorker.args[] |= sub("^--enable-compression=true$"; "--enable-compression") |
    .snsWorker.args[] |= sub("^--lifo=true$"; "--lifo") |
    del(.snsWorker.args[] | select(. == "--lifo=false"))
  ' "${legacy}"
  host_tag="$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
  if [[ "${BLUE_GREEN:-false}" == "true" ]]; then
    host_tag="${BCS_IMAGE_TAG}"
  fi
  HOST_TAG="${host_tag}" yq -i '
    .hostListener.image.name = "hub.zama.org/ghcr/zama-ai/fhevm/coprocessor/host-listener" |
    .hostListener.image.tag = strenv(HOST_TAG)
  ' "${legacy}"
  values_files=(-f "${adapted}" -f "${legacy}")
}

# Chart 0.7.10's migration Job inits with netdata/wget:latest, which is not a
# real image, so sqlx never runs. The migration binary itself inserts a fake
# key and only needs the tenant env.
run_legacy_coprocessor_migration() {
  local party="$1" tag job
  tag="$(jq -r .coprocessor_db_migration <<<"${TAGS_JSON}")"
  if [[ "${BLUE_GREEN:-false}" == "true" ]]; then
    tag="${BCS_IMAGE_TAG}"
  fi
  job="coprocessor-${party}-db-init"
  echo "coprocessor chart 0.7.x: migrating database without the wget init"
  kubectl delete job "${job}" -n "${NAMESPACE}" --ignore-not-found
  kubectl apply -n "${NAMESPACE}" -f - <<EOF
apiVersion: batch/v1
kind: Job
metadata:
  name: ${job}
spec:
  backoffLimit: 2
  template:
    spec:
      restartPolicy: Never
      imagePullSecrets:
        - name: registry-credentials
      containers:
        - name: db-migration
          image: hub.zama.org/ghcr/zama-ai/fhevm/coprocessor/db-migration:${tag}
          command: ["/initialize_db.sh"]
          env:
            - name: DATABASE_URL
              value: "postgresql://zama:zama@postgres-coprocessor-${party}:5432/fhevm_e2e"
            - name: TENANT_API_KEY
              valueFrom:
                secretKeyRef:
                  name: coprocessor-api-key
                  key: coprocessor-api-key
            - name: CHAIN_ID
              value: "12345"
            - name: ACL_CONTRACT_ADDRESS
              valueFrom:
                configMapKeyRef:
                  name: host-sc-addresses
                  key: acl.address
            - name: INPUT_VERIFIER_ADDRESS
              valueFrom:
                configMapKeyRef:
                  name: host-sc-addresses
                  key: input_verifier.address
EOF
  kubectl wait --for=condition=complete "job/${job}" -n "${NAMESPACE}" --timeout=180s
}

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
    # The detector's wallet is on by default and its secret ref points at a
    # coprocessor-tx-sender Secret that only exists where infra provisions one.
    # Here every signer takes the party key inline, same as bg-green.sh does.
    --set-string "consensusDetector.wallet.secret.value=${privkey}"
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
    # The pinned release creates the database, so the seeded versions are the
    # ones its binaries run. The GCS release migrates on top of it.
    --set-string "dbMigration.image.tag=${tag}"
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

values_files=(-f ci/preview-env/coprocessor/values-coprocessor-e2e.yaml)
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
      --set-string "dbMigration.image.tag=$(jq -r .coprocessor_db_migration <<<"${TAGS_JSON}")"
      --set-string "gwListener.image.tag=$(jq -r .coprocessor_gw_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerPollerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerCatchupOnlyShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "hostListenerConsumerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
      --set-string "snsWorker.image.tag=$(worker_image_tag coprocessor_sns_worker)"
      --set-string "tfheWorker.image.tag=$(worker_image_tag coprocessor_tfhe_worker)"
      --set-string "txSender.image.tag=$(jq -r .coprocessor_tx_sender <<<"${TAGS_JSON}")"
      --set-string "zkProofWorker.image.tag=$(worker_image_tag coprocessor_zkproof_worker)"
    )
  fi
  party_common "${i}" "${privkey}"
  if coprocessor_chart_is_legacy; then
    echo "coprocessor chart 0.7.x: adapting tolerations, gateway URL, and host listener"
    prepare_legacy_coprocessor_values "${i}" "${privkey}"
  fi
  fleet_json=()
  if [[ "${BLUE_GREEN}" == "true" ]]; then
    fleet_json=(--set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-bcs\"}")
  fi
  helm upgrade --install "coprocessor-${i}" "${COPROCESSOR_CHART}" \
    -n "${NAMESPACE}" "${values_files[@]}" \
    "${polygon_args[@]}" \
    "${bcs_overlay[@]}" \
    ${coproc_gpu_overlay[@]+"${coproc_gpu_overlay[@]}"} \
    "${fleet_json[@]}" \
    --set-string "fullnameOverride=coprocessor-${i}" \
    "${common_args[@]}" \
    "${bcs_images[@]}"
  if coprocessor_chart_is_legacy; then
    run_legacy_coprocessor_migration "${i}"
  fi
done

if [[ "${BLUE_GREEN}" == "true" ]]; then
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    kubectl wait --for=condition=complete \
      "job/coprocessor-${i}-db-migration-1" -n "${NAMESPACE}" --timeout=180s \
      || kubectl wait --for=condition=complete \
        -l app=coprocessor-db-migration,app.kubernetes.io/name="coprocessor-${i}-db-migration" \
        job -n "${NAMESPACE}" --timeout=180s
  done
  # BCS zkproof (0.14.x) snapshots host_chains once at process start.
  # helm applies the Deployment in parallel with db-migration, so a party
  # whose seed finishes a second later keeps chain_id = ANY('{}') forever.
  # Restart after seed so both parties see the host chain.
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    kubectl rollout restart "deploy/coprocessor-${i}-zkproof-worker" -n "${NAMESPACE}"
    kubectl rollout status "deploy/coprocessor-${i}-zkproof-worker" -n "${NAMESPACE}" --timeout=180s
  done
  # Manual Blue/Green QA mode (no automated_tests): Blue only. Green (migration, fleet,
  # upgrade-controller, consensus-detector) is started later with bg-green.sh.
  if [[ "${AUTOMATED_TESTS}" != "true" ]]; then
    echo "Blue/Green manual mode: GCS release not installed (start it with ci/preview-env/scripts/bg/bg-green.sh)"
  else
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      privkey=$(jq -r --argjson party "${i}" '.[] | select(.party == $party) | .privateKey' <<<"${COPROC_WALLETS_JSON}")
      party_common "${i}" "${privkey}" "-gcs"
      gcs_images=(
        # HEAD migrator on the database the BCS release created. Runs as a Helm
        # hook so it finishes before the fleet starts: a service that reads
        # versioning before the consensus column exists treats it as unseeded
        # and would come up blue.
        --set-string "dbMigration.image.tag=$(jq -r .coprocessor_db_migration <<<"${TAGS_JSON}")"
        --set-json 'dbMigration.annotations={"helm.sh/hook":"pre-install,pre-upgrade","helm.sh/hook-delete-policy":"before-hook-creation"}'
        --set-string "gwListener.image.tag=$(jq -r .coprocessor_gw_listener <<<"${TAGS_JSON}")"
        --set-string "hostListenerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
        --set-string "hostListenerPollerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
        --set-string "hostListenerCatchupOnlyShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
        --set-string "hostListenerConsumerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"
        --set-string "snsWorker.image.tag=$(worker_image_tag coprocessor_sns_worker)"
        --set-string "tfheWorker.image.tag=$(worker_image_tag coprocessor_tfhe_worker)"
        --set-string "txSender.image.tag=$(jq -r .coprocessor_tx_sender <<<"${TAGS_JSON}")"
        --set-string "zkProofWorker.image.tag=$(worker_image_tag coprocessor_zkproof_worker)"
        --set-string "upgradeController.image.tag=$(jq -r .coprocessor_tfhe_worker <<<"${TAGS_JSON}")"
        --set-string "consensusDetector.image.tag=$(jq -r .coprocessor_tfhe_worker <<<"${TAGS_JSON}")"
      )
      # The label follows the compiled release when the workflow passes it; the
      # overlay's value is the fallback for a manual run.
      gcs_version_args=()
      if [[ -n "${GCS_STACK_VERSION:-}" ]]; then
        gcs_version_args=(--set-string "commonConfig.stackVersion=${GCS_STACK_VERSION}")
      fi
      helm upgrade --install "coprocessor-${i}-gcs" "${COPROCESSOR_CHART}" \
        -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-e2e.yaml \
        -f ci/preview-env/coprocessor/values-coprocessor-gcs-e2e.yaml \
        ${gcs_gpu_overlay[@]+"${gcs_gpu_overlay[@]}"} \
        --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-gcs\"}" \
        --set-string "fullnameOverride=coprocessor-${i}-gcs" \
        --set-string "upgradeController.serviceAccountName=coprocessor-${i}" \
        --set-string "consensusDetector.serviceAccountName=coprocessor-${i}" \
        --set "snsWorker.config.s3BucketName.valueFrom.configMapKeyRef.name=coprocessor-${i}" \
        "${common_args[@]}" \
        "${gcs_images[@]}" \
        ${gcs_version_args[@]+"${gcs_version_args[@]}"}
    done
    # Hold the detector so the first e2e stays on Blue; the workflow enables it afterwards.
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      kubectl scale "deploy/coprocessor-${i}-gcs-consensus-detector" \
        -n "${NAMESPACE}" --replicas=0
    done
  fi
fi
sleep 60
