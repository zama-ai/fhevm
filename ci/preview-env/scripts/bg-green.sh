#!/usr/bin/env bash
# Blue/Green QA: bring the Green (GCS) stack into a preview env that runs Blue only.
#
#   bg-green.sh migrate   apply the HEAD database migration to every party while
#                         Blue keeps serving (step 3 of the manual flow)
#   bg-green.sh start     install the Green fleet per party: coprocessor-<i>-gcs
#                         (workers, gw-listener, consumer, tx-sender, upgrade-controller,
#                         consensus-detector) and the Green Polygon consumer
#                         coprocessor-polygon-<i>-gcs (step 4). Needs `migrate` first.
#
# Values are the ones CI used for this env: the deployed Blue release carries the
# chain-mode patched commonConfig/chains (apply-chain-env.sh), so Green is built
# from the checkout's values-coprocessor-e2e.yaml with those two blocks copied
# from Blue, plus the gcs overlay - the same inputs deploy-coprocessor.sh uses.
# Keep the helm --set lists in sync with deploy-coprocessor.sh and
# deploy-coprocessor-sidecars.sh (polygon-consumer-gcs).
#
# Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg-green.sh migrate|start
# Env: NAMESPACE (required), NB_COPROCESSOR (2),
#      DEPLOY_POLYGON (default: true when the Blue Polygon consumer release exists),
#      GCS_IMAGE_TAG (default: the listener image tag of this env, else the checkout's HEAD SHA),
#      GCS_STACK_VERSION (default: commonConfig.stackVersion of the gcs overlay),
#      COPROCESSOR_CHART (charts/coprocessor of this checkout),
#      BCS_STACK_VERSION (default: what the running Blue binary prints for --stack-version).
set -euo pipefail

verb="${1:?usage: bg-green.sh migrate|start}"
: "${NAMESPACE:?}"
NB_COPROCESSOR="${NB_COPROCESSOR:-2}"
if [[ -z "${DEPLOY_POLYGON:-}" ]]; then
  DEPLOY_POLYGON=false
  helm status coprocessor-polygon-1 -n "${NAMESPACE}" >/dev/null 2>&1 && DEPLOY_POLYGON=true
fi
# The versioning row holds the string the Blue binary was compiled with (a 0.14.1
# image still prints 0.14.0), so read it from the binary rather than from a pin.
BCS_STACK_VERSION="${BCS_STACK_VERSION:-$(kubectl exec -n "${NAMESPACE}" deploy/coprocessor-1-host-listener-consumer -- host_listener --stack-version)}"

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
values_dir="${root}/ci/preview-env/coprocessor"
COPROCESSOR_CHART="${COPROCESSOR_CHART:-${root}/charts/coprocessor}"
GCS_STACK_VERSION="${GCS_STACK_VERSION:-$(yq -r '.commonConfig.stackVersion' "${values_dir}/values-coprocessor-gcs-e2e.yaml")}"
# Green image tag = the tag the branch's coprocessor images were published under. Blue is pinned to
# the previous release and, on the production path, so are the contracts/relayer/test-suite, so the
# listener (never pinned) is the one deployed component that carries it; the checkout's HEAD is the
# fallback. Override with GCS_IMAGE_TAG whenever the deploy resolved a different tag.
GCS_IMAGE_TAG="${GCS_IMAGE_TAG:-$(kubectl get deploy -n "${NAMESPACE}" listener-1-host \
  -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null | sed 's/.*://')}"
GCS_IMAGE_TAG="${GCS_IMAGE_TAG:-$(git -C "${root}" rev-parse --short=7 HEAD)}"
# Newest migration the Green migrator applies: from the commit the image tag names
# when this checkout has it (tags are short SHAs), else from the checkout itself.
migrations_dir="coprocessor/fhevm-engine/db-migration/migrations"
if git -C "${root}" cat-file -e "${GCS_IMAGE_TAG}^{commit}" 2>/dev/null; then
  head_migration=$(git -C "${root}" ls-tree --name-only "${GCS_IMAGE_TAG}" "${migrations_dir}/" | sort | tail -1 | xargs basename | cut -d_ -f1)
else
  head_migration=$(ls "${root}/${migrations_dir}" | sort | tail -1 | cut -d_ -f1)
fi
work=$(mktemp -d)
trap 'rm -rf "${work}"' EXIT

fail() { echo "::error::$*" >&2; exit 1; }
version_mm() { sed -E 's/^v//; s/^([0-9]+\.[0-9]+).*/\1/' <<<"$1"; }

psql_party() {
  local party="$1" sql="$2"
  kubectl exec -n "${NAMESPACE}" "postgres-coprocessor-${party}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -v ON_ERROR_STOP=1 -tAqc "${sql}"
}

# Green values for party i: checkout e2e values + Blue's patched commonConfig and host chain.
green_values() {
  local party="$1"
  local blue="${work}/blue-${party}.yaml" out="${work}/green-${party}.yaml"
  helm get values "coprocessor-${party}" -n "${NAMESPACE}" -o yaml > "${blue}"
  cp "${values_dir}/values-coprocessor-e2e.yaml" "${out}"
  BLUE="${blue}" yq -i '.commonConfig = load(strenv(BLUE)).commonConfig
    | .chains = [load(strenv(BLUE)).chains[] | select(.name == "host")]' "${out}"
  echo "${out}"
}

# helm args shared by `migrate` (template) and `start` (install), mirroring deploy-coprocessor.sh.
green_helm_args() {
  local party="$1" values="$2" privkey canonical
  local blue="${work}/blue-${party}.yaml"
  privkey=$(yq -r '.txSender.wallet.secret.value' "${blue}")
  canonical=$(yq -r '.commonConfig.canonicalProtocolConfigChainId' "${blue}")
  [[ -n "${privkey}" && "${privkey}" != "null" ]] || fail "party ${party}: no tx-sender key in the Blue release values"
  GREEN_ARGS=(
    -n "${NAMESPACE}"
    -f "${values}"
    -f "${values_dir}/values-coprocessor-gcs-e2e.yaml"
    --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${party}-gcs\"}"
    --set-string "fullnameOverride=coprocessor-${party}-gcs"
    --set-string "commonConfig.stackVersion=${GCS_STACK_VERSION}"
    --set-string "commonConfig.canonicalProtocolConfigChainId=${canonical}"
    --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-${party}:5432"
    --set-string "hostListenerShared.serviceAccountName=coprocessor-${party}"
    --set-string "hostListenerPollerShared.serviceAccountName=coprocessor-${party}"
    --set-string "hostListenerCatchupOnlyShared.serviceAccountName=coprocessor-${party}"
    --set-string "hostListenerConsumerShared.serviceAccountName=coprocessor-${party}"
    --set-string "tfheWorker.serviceAccountName=coprocessor-${party}"
    --set-string "zkProofWorker.serviceAccountName=coprocessor-${party}"
    --set-string "snsWorker.serviceAccountName=coprocessor-${party}"
    --set-string "upgradeController.serviceAccountName=coprocessor-${party}"
    --set-string "consensusDetector.serviceAccountName=coprocessor-${party}"
    --set "snsWorker.config.s3BucketName.valueFrom.configMapKeyRef.name=coprocessor-${party}"
    --set-string "hostListenerConsumerShared.config.brokerUrl.value=redis://coprocessor-redis-${party}-master:6379"
    --set-string "txSender.wallet.secret.value=${privkey}"
    --set-string "snsWorker.wallet.secret.value=${privkey}"
  )
  for img in dbMigration gwListener hostListenerShared hostListenerPollerShared hostListenerCatchupOnlyShared \
             hostListenerConsumerShared snsWorker tfheWorker txSender zkProofWorker upgradeController consensusDetector; do
    GREEN_ARGS+=(--set-string "${img}.image.tag=${GCS_IMAGE_TAG}")
  done
}

blue_live() {
  local party="$1" v
  # Pre-migration the row is 'v0.14' with no consensus_version column; compare major.minor and default to 1.
  v=$(psql_party "${party}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1')||' upgrade_state='||(SELECT count(*) FROM upgrade_state)||' gcs_schemas='||(SELECT count(*) FROM pg_namespace WHERE nspname LIKE 'gcs%') FROM versioning v;")
  [[ "$(version_mm "${v%%/*}")" == "$(version_mm "${BCS_STACK_VERSION}")" && "${v#*/}" == "1 upgrade_state=0 gcs_schemas=0" ]] \
    || fail "party ${party} is not in the Blue-only state (${v}); run bg-reset.sh first"
  helm status "coprocessor-${party}-gcs" -n "${NAMESPACE}" >/dev/null 2>&1 \
    && fail "party ${party}: Green release already installed"
  local ready
  ready=$(kubectl get deploy -n "${NAMESPACE}" "coprocessor-${party}-host-listener-consumer" -o jsonpath='{.status.readyReplicas}')
  [[ "${ready}" == "1" ]] || fail "party ${party}: Blue consumer not ready"
}

echo "== bg-green ${verb}: ${NAMESPACE}, ${NB_COPROCESSOR} parties, Green image tag ${GCS_IMAGE_TAG}, stack ${GCS_STACK_VERSION}, HEAD migration ${head_migration}"

case "${verb}" in
migrate)
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    blue_live "${i}"
    before=$(psql_party "${i}" "SELECT max(version)||' ('||count(*)||')' FROM _sqlx_migrations;")
    values=$(green_values "${i}")
    green_helm_args "${i}" "${values}"
    job="coprocessor-${i}-gcs-db-migration-1"
    # Same Job the Green release runs as its pre-install hook, rendered without the
    # hook so it runs now, on its own, while Blue serves. `start` re-runs it as a no-op.
    helm template "coprocessor-${i}-gcs" "${COPROCESSOR_CHART}" "${GREEN_ARGS[@]}" \
      --show-only templates/coprocessor-db-migration.yaml > "${work}/migrate-${i}.yaml"
    kubectl delete job "${job}" -n "${NAMESPACE}" --ignore-not-found >/dev/null
    kubectl apply -n "${NAMESPACE}" -f "${work}/migrate-${i}.yaml"
    if ! kubectl wait --for=condition=complete "job/${job}" -n "${NAMESPACE}" --timeout=300s; then
      kubectl logs -n "${NAMESPACE}" "job/${job}" --tail=50 || true
      fail "party ${i}: migration job did not complete"
    fi
    after=$(psql_party "${i}" "SELECT max(version)||' ('||count(*)||')' FROM _sqlx_migrations;")
    [[ "${after}" == "${head_migration} ("* ]] || fail "party ${i}: DB at ${after}, expected ${head_migration}"
    echo "party ${i}: migrations ${before} -> ${after}; Blue still $(psql_party "${i}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1') FROM versioning v;")"
  done
  echo "== migrate done: schema at ${head_migration} on every party, Blue untouched. Next: bg-green.sh start"
  ;;

start)
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    blue_live "${i}"
    at=$(psql_party "${i}" "SELECT max(version) FROM _sqlx_migrations;")
    [[ "${at}" == "${head_migration}" ]] || fail "party ${i}: schema at ${at}, run bg-green.sh migrate first"
  done
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    values=$(green_values "${i}")
    green_helm_args "${i}" "${values}"
    echo "party ${i}: installing coprocessor-${i}-gcs"
    helm upgrade --install "coprocessor-${i}-gcs" "${COPROCESSOR_CHART}" "${GREEN_ARGS[@]}" \
      --set-json 'dbMigration.annotations={"helm.sh/hook":"pre-install,pre-upgrade","helm.sh/hook-delete-policy":"before-hook-creation"}' \
      >/dev/null
    # Green pollers: twins of the Blue poller releases (HEAD image, Green fleet). They inject the
    # synthetic host anchors and, in gcs mode, wait for the Green schema the controller creates.
    for rel in "coprocessor-poller-${i}" $([[ "${DEPLOY_POLYGON}" == "true" ]] && echo "coprocessor-poller-polygon-${i}"); do
      helm get values "${rel}" -n "${NAMESPACE}" -o yaml > "${work}/${rel}.yaml"
      echo "party ${i}: installing ${rel}-gcs"
      helm upgrade --install "${rel}-gcs" "${COPROCESSOR_CHART}" -n "${NAMESPACE}" \
        -f "${work}/${rel}.yaml" \
        --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-gcs\"}" \
        --set-string "commonConfig.stackVersion=${GCS_STACK_VERSION}" \
        --set-string "hostListenerPollerShared.image.tag=${GCS_IMAGE_TAG}" \
        >/dev/null
    done
    if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
      # Twin of the Blue Polygon consumer: its values, Green image/version/fleet and an
      # own consumer name (the Redis group is <service-name>.<chain-id>).
      helm get values "coprocessor-polygon-${i}" -n "${NAMESPACE}" -o yaml > "${work}/polygon-${i}.yaml"
      echo "party ${i}: installing coprocessor-polygon-${i}-gcs"
      helm upgrade --install "coprocessor-polygon-${i}-gcs" "${COPROCESSOR_CHART}" -n "${NAMESPACE}" \
        -f "${work}/polygon-${i}.yaml" \
        --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-gcs\"}" \
        --set-string "commonConfig.stackVersion=${GCS_STACK_VERSION}" \
        --set-string "hostListenerConsumerShared.image.tag=${GCS_IMAGE_TAG}" \
        --set-string "hostListenerConsumerShared.args.serviceName=host-listener-consumer-gcs" \
        >/dev/null
    fi
  done
  for d in $(kubectl get deploy -n "${NAMESPACE}" -o name | grep -- "-gcs-"); do
    kubectl rollout status "${d}" -n "${NAMESPACE}" --timeout=300s >/dev/null
  done

  # Verify: Green schema created by the controller, Green consumers shadow-ingesting
  # on every chain, pollers unparked, Blue still live at the Blue version.
  schema="gcs-${GCS_STACK_VERSION}"
  expected_chains=1; [[ "${DEPLOY_POLYGON}" == "true" ]] && expected_chains=2
  failed=0
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    chains=0
    for _ in $(seq 1 36); do
      chains=$(psql_party "${i}" "SELECT CASE WHEN to_regclass('\"${schema}\".host_chain_blocks_valid') IS NULL THEN 0 ELSE (SELECT count(DISTINCT chain_id) FROM \"${schema}\".host_chain_blocks_valid WHERE created_at > now() - interval '60 seconds') END;")
      [[ "${chains}" -ge "${expected_chains}" ]] && break
      sleep 5
    done
    if [[ "${chains}" -ge "${expected_chains}" ]]; then
      echo "party ${i}: Green schema ${schema} present, Green consumers ingesting on ${chains} chain(s)"
    else
      echo "::error::party ${i}: Green ingests on ${chains}/${expected_chains} chain(s) in ${schema} after 3 min"; failed=1
    fi
    blue=$(psql_party "${i}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1')||' upgrade_state='||(SELECT count(*) FROM upgrade_state) FROM versioning v;")
    [[ "$(version_mm "${blue%%/*}")" == "$(version_mm "${BCS_STACK_VERSION}")" && "${blue#*/}" == "1 upgrade_state=0" ]] || { echo "::error::party ${i}: Blue state changed: ${blue}"; failed=1; }
    for d in upgrade-controller consensus-detector; do
      if kubectl logs -n "${NAMESPACE}" "deploy/coprocessor-${i}-gcs-${d}" --tail=200 2>/dev/null | grep '"level":"ERROR"' >/dev/null; then
        echo "::warning::party ${i}: ${d} logged errors, check: kubectl logs -n ${NAMESPACE} deploy/coprocessor-${i}-gcs-${d}"
      fi
    done
    for p in "coprocessor-poller-${i}-gcs-host-listener-poller" "coprocessor-poller-polygon-${i}-gcs-host-listener-poller"; do
      kubectl get deploy -n "${NAMESPACE}" "${p}" >/dev/null 2>&1 || continue
      if kubectl logs -n "${NAMESPACE}" "deploy/${p}" --tail=3 2>/dev/null | grep "waiting for the upgrade-controller to create the GCS schema" >/dev/null; then
        echo "::warning::${p} still waiting for the Green schema"
      fi
    done
  done
  [[ "${failed}" == "0" ]] || fail "Green installed but verification failed, see errors above"
  echo "== start done: Green ${GCS_STACK_VERSION} (${GCS_IMAGE_TAG}) shadowing Blue ${BCS_STACK_VERSION} on ${NB_COPROCESSOR} parties. Next: propose the upgrade."
  ;;
*)
  fail "unknown verb '${verb}' (migrate|start)"
  ;;
esac
