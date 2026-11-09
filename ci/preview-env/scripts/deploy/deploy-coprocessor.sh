#!/usr/bin/env bash
# Install BCS (and optional GCS) coprocessor releases.
# Env: NAMESPACE, NB_COPROCESSOR, BLUE_GREEN, GPU, DEPLOY_POLYGON, OBSERVABILITY,
# AUTOMATED_TESTS, COPROCESSOR_CHART, TAGS_JSON, BCS_IMAGE_TAG, GPU_IMAGE_TAG,
# OTLP_ENDPOINT, COPROC_WALLETS_JSON, GCS_STACK_VERSION (pod label for the GCS
# fleet; optional), COMMON_CHART, COMMON_CHART_VERSION, COPROCESSOR_VERSION_PIN (optional).
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source-path=SCRIPTDIR
# shellcheck source=../lib.sh
source "${script_dir}/../lib.sh"

######################################################################
# coprocessor-proxy and prf-service, except on blue-green stacks: one of each
# per party, in its coprocessor-<i> release (values-coprocessor-proxy-e2e.yaml).
# What the e2e suite needs (test-suite/e2e/test/coprocessorProxy) goes in
# the coprocessor-proxy ConfigMap:
#   urls                      COPROCESSOR_PROXY_URLS, one https proxy URL per party
#   signer-addresses          COPROCESSOR_SIGNER_ADDRESSES, the audience of each
#   prf-service-metrics-urls  PRF_SERVICE_METRICS_URLS, each prf-service's request metric
#   metrics-urls              COPROCESSOR_PROXY_METRICS_URLS
#   ca.crt                    the proxies' certificate
######################################################################
# Must match charts/coprocessor/templates/_helpers.tpl (coprocessorProxyName:
# "<release>-coprocessor-proxy", prfServiceName: "<release>-prf-service") and the ports.
proxy_service() { echo "coprocessor-${1}-coprocessor-proxy"; }
prf_service() { echo "coprocessor-${1}-prf-service"; }
coprocessor_values=ci/preview-env/coprocessor/values-coprocessor-e2e.yaml
proxy_values=ci/preview-env/coprocessor/values-coprocessor-proxy-e2e.yaml
proxy_tls_secret=coprocessor-proxy-tls  # tls.secretName in values-coprocessor-proxy-e2e.yaml
coproc_signer() { jq -r --argjson party "${1}" '.[] | select(.party == $party) | .address' <<<"${COPROC_WALLETS_JSON}"; }
with_proxy="false"
# A pinned release can predate pads, which prf-service and the seed need.
if [[ "${BLUE_GREEN}" != "true" && -z "${COPROCESSOR_VERSION_PIN:-}" ]]; then
  with_proxy="true"
else
  # Without it, the suite skips and does not call the proxies of an earlier deploy.
  kubectl delete configmap coprocessor-proxy -n "${NAMESPACE}" --ignore-not-found
fi

# Runs psql in party i's database as the owner.
psql_owner() {
  local i="$1"
  shift
  kubectl exec -i -n "${NAMESPACE}" "postgres-coprocessor-${i}-0" -- \
    env PGPASSWORD="$(yq -r '.commonConfig.databasePassword.value' "${coprocessor_values}")" \
    psql -U "$(yq -r '.commonConfig.databaseUser.value' "${coprocessor_values}")" -d fhevm_e2e -v ON_ERROR_STOP=1 -qtA "$@"
}

# Creates the prf_service role before the release. The migration then grants its rights on pads.
create_prf_service_role() {
  local i="$1"
  kubectl wait --for=condition=ready "pod/postgres-coprocessor-${i}-0" -n "${NAMESPACE}" --timeout=300s
  psql_owner "${i}" \
    -v user="$(yq -r '.prfService.databaseUser.value' "${proxy_values}")" \
    -v password="$(yq -r '.prfService.databasePassword.value' "${proxy_values}")" <<'SQL'
SELECT format('CREATE ROLE %I LOGIN', :'user') WHERE NOT EXISTS (SELECT FROM pg_roles WHERE rolname = :'user') \gexec
SELECT format('ALTER ROLE %I PASSWORD %L', :'user', :'password') \gexec
SQL
}

# Writes the suite's PRF output into party i's database after the migration. The script comes from
# this checkout, so an older test-suite image can also run it.
seed_prf_output() {
  local i="$1" pod="coprocessor-${1}-seed-prf-output" revision
  revision=$(helm status "coprocessor-${i}" -n "${NAMESPACE}" -o json | jq -r .version)
  kubectl wait --for=condition=complete "job/coprocessor-${i}-db-migration-${revision}" -n "${NAMESPACE}" --timeout=300s
  kubectl delete pod "${pod}" -n "${NAMESPACE}" --ignore-not-found --wait=true
  POD="${pod}" IMAGE="hub.zama.org/ghcr/zama-ai/fhevm/test-suite/e2e:$(jq -r .test_suite <<<"${TAGS_JSON}")" \
    DATABASE_URL="postgresql://$(yq -r '.commonConfig.databaseUser.value' "${coprocessor_values}"):$(yq -r '.commonConfig.databasePassword.value' "${coprocessor_values}")@postgres-coprocessor-${i}:5432/fhevm_e2e" \
    yq -n '{
      "apiVersion": "v1",
      "kind": "Pod",
      "metadata": {"name": strenv(POD)},
      "spec": {
        "restartPolicy": "Never",
        "imagePullSecrets": [{"name": "registry-credentials"}],
        "containers": [{
          "name": "seed",
          "image": strenv(IMAGE),
          "workingDir": "/app/test-suite/e2e",
          "command": ["node", "scripts/seed-prf-output.ts"],
          "env": [{"name": "DATABASE_URL", "value": strenv(DATABASE_URL)}],
          "volumeMounts": [{
            "name": "seed",
            "mountPath": "/app/test-suite/e2e/scripts/seed-prf-output.ts",
            "subPath": "seed-prf-output.ts",
            "readOnly": true
          }]
        }],
        "volumes": [{"name": "seed", "configMap": {"name": "prf-output-seed"}}]
      }
    }' | kubectl apply -n "${NAMESPACE}" -f -
  if ! kubectl wait --for=jsonpath='{.status.phase}'=Succeeded "pod/${pod}" -n "${NAMESPACE}" --timeout=300s; then
    kubectl logs "${pod}" -n "${NAMESPACE}" || true
    echo "::error::${pod} did not write the PRF output"
    exit 1
  fi
  kubectl logs "${pod}" -n "${NAMESPACE}"
  kubectl delete pod "${pod}" -n "${NAMESPACE}" --wait=false
}

# One self-signed certificate for every party's proxy, as for the kms-connector proxies.
if [[ "${with_proxy}" == "true" ]] && ! kubectl get secret "${proxy_tls_secret}" -n "${NAMESPACE}" >/dev/null 2>&1; then
  sans=""
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    svc=$(proxy_service "${i}")
    sans="${sans:+${sans},}DNS:${svc},DNS:${svc}.${NAMESPACE}.svc,DNS:${svc}.${NAMESPACE}.svc.cluster.local"
  done
  tls_dir=$(mktemp -d)
  openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days 30 \
    -keyout "${tls_dir}/tls.key" -out "${tls_dir}/tls.crt" -subj "/CN=$(proxy_service 1)" \
    -addext "subjectAltName=${sans}" \
    -addext "basicConstraints=critical,CA:TRUE" \
    -addext "keyUsage=critical,digitalSignature,keyCertSign" \
    -addext "extendedKeyUsage=serverAuth" 2>/dev/null
  kubectl create secret tls "${proxy_tls_secret}" -n "${NAMESPACE}" \
    --cert="${tls_dir}/tls.crt" --key="${tls_dir}/tls.key"
  rm -rf "${tls_dir}"
  echo "Created ${proxy_tls_secret} (SANs: ${sans})"
fi

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
  fleet_json=()
  if [[ "${BLUE_GREEN}" == "true" ]]; then
    fleet_json=(--set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-bcs\"}")
  fi
  proxy_args=()
  if [[ "${with_proxy}" == "true" ]]; then
    signer=$(coproc_signer "${i}")
    require_nonempty "${signer}" "no coprocessor signer address for party ${i} in COPROC_WALLETS_JSON"
    # Removes the fake prf-service of an earlier deploy.
    if helm status "coprocessor-${i}-prf-service-fake" -n "${NAMESPACE}" >/dev/null 2>&1; then
      helm uninstall "coprocessor-${i}-prf-service-fake" -n "${NAMESPACE}"
    fi
    create_prf_service_role "${i}"
    proxy_args=(
      -f "${proxy_values}"
      --set-string "coprocessorProxy.image.tag=$(jq -r .coprocessor_proxy <<<"${TAGS_JSON}")"
      --set-string "coprocessorProxy.coprocessorSignerAddress=${signer}"
      --set-string "prfService.image.tag=$(jq -r .coprocessor_prf_service <<<"${TAGS_JSON}")"
      # Only this party's prf-service. The chart default selects all parties.
      --set-json "coprocessorProxy.prfServiceNetworkPolicy.podSelector={\"app\":\"coprocessor-prf-service\",\"app.kubernetes.io/name\":\"$(prf_service "${i}")\"}"
    )
  fi
  helm upgrade --install "coprocessor-${i}" "${COPROCESSOR_CHART}" \
    -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-e2e.yaml \
    "${polygon_args[@]}" \
    "${bcs_overlay[@]}" \
    ${coproc_gpu_overlay[@]+"${coproc_gpu_overlay[@]}"} \
    ${proxy_args[@]+"${proxy_args[@]}"} \
    "${fleet_json[@]}" \
    --set-string "fullnameOverride=coprocessor-${i}" \
    "${common_args[@]}" \
    "${bcs_images[@]}"
done

if [[ "${with_proxy}" == "true" ]]; then
  kubectl create configmap prf-output-seed -n "${NAMESPACE}" \
    --from-file=seed-prf-output.ts=test-suite/e2e/scripts/seed-prf-output.ts \
    --dry-run=client -o yaml | kubectl apply -f -
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    seed_prf_output "${i}"
  done
  urls="" signers="" prf_metrics_urls="" metrics_urls=""
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    urls="${urls:+${urls},}https://$(proxy_service "${i}"):8443"
    signers="${signers:+${signers},}$(coproc_signer "${i}")"
    prf_metrics_urls="${prf_metrics_urls:+${prf_metrics_urls},}http://$(prf_service "${i}"):9100/metrics"
    metrics_urls="${metrics_urls:+${metrics_urls},}http://$(proxy_service "${i}"):9100/metrics"
  done
  ca_file=$(mktemp)
  kubectl get secret "${proxy_tls_secret}" -n "${NAMESPACE}" -o jsonpath='{.data.tls\.crt}' | base64 -d > "${ca_file}"
  kubectl create configmap coprocessor-proxy -n "${NAMESPACE}" \
    --from-literal=urls="${urls}" \
    --from-literal=signer-addresses="${signers}" \
    --from-literal=prf-service-metrics-urls="${prf_metrics_urls}" \
    --from-literal=metrics-urls="${metrics_urls}" \
    --from-file=ca.crt="${ca_file}" \
    --dry-run=client -o yaml | kubectl apply -f -
  rm -f "${ca_file}"
  echo "coprocessor-proxy: urls=${urls} signers=${signers}"
fi

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
