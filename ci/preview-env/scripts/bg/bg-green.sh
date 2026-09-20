#!/usr/bin/env bash
# Blue/Green QA: bring the Green (GCS) stack into a preview env that runs Blue only.
#
#   bg-green.sh migrate   apply the HEAD database migration to every party while
#                         Blue keeps serving (step 3 of the manual flow)
#   bg-green.sh start     install the Green fleet per party: coprocessor-<i><slot>
#                         (workers, gw-listener, consumer, tx-sender, upgrade-controller,
#                         consensus-detector) and the Green Polygon consumer
#                         coprocessor-polygon-<i><slot> (step 4). Needs `migrate` first.
#
# Values are the ones CI used for this env: the deployed Blue release carries the
# chain-mode patched commonConfig/chains (apply-chain-env.sh), so Green is built
# from the checkout's values-coprocessor-e2e.yaml with those two blocks copied
# from Blue, plus the gcs overlay - the same inputs deploy-coprocessor.sh uses.
# Keep the helm --set lists in sync with deploy-coprocessor.sh and
# deploy-coprocessor-sidecars.sh (polygon-consumer-gcs).
#
# Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg/bg-green.sh migrate|start
# Env: NAMESPACE (required), NB_COPROCESSOR (2),
#      DEPLOY_POLYGON (default: true when the Blue Polygon consumer release exists),
#      GCS_IMAGE_TAG (default: the listener image tag of this env, else the checkout's HEAD SHA),
#      GCS_STACK_VERSION (default: commonConfig.stackVersion of the gcs overlay),
#      COPROCESSOR_CHART (charts/coprocessor of this checkout),
#      BCS_STACK_VERSION (default: what the running live binary prints for --stack-version),
#      ALLOW_ACTIVE_ATTEMPT (unset) - start Green although an attempt is in_progress. Only for a
#        proposal no Green ever picked up: it is refused once a round is past UpgradeActivated,
#        so it cannot clobber a running dry run or cutover,
#      GREEN_SLOT (-gcs), LIVE_RELEASE_PREFIX (coprocessor), LIVE_RELEASE_SUFFIX ("") - the helm
#        slots. A second round in the same env inverts them: the live fleet is in -gcs and the new
#        Green takes the slot the retired one vacated, i.e. GREEN_SLOT="" LIVE_RELEASE_SUFFIX=-gcs.
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
# Which helm slot the incoming fleet occupies, and which release it copies chain config from.
# Blue/green mode is decided by the binary's CONSENSUS_PROTOCOL_VERSION, not by these names, so on
# a second round the roles swap and the new Green takes the slot the retired fleet vacated.
GREEN_SLOT="${GREEN_SLOT--gcs}"
LIVE_RELEASE_PREFIX="${LIVE_RELEASE_PREFIX:-coprocessor}"
LIVE_RELEASE_SUFFIX="${LIVE_RELEASE_SUFFIX:-}"
green_release() { echo "coprocessor-$1${GREEN_SLOT}"; }
live_release()  { echo "${LIVE_RELEASE_PREFIX}-$1${LIVE_RELEASE_SUFFIX}"; }
# Fleet label: the slot name without the leading dash, or "bcs" for the unsuffixed slot.
green_fleet() { echo "$1${GREEN_SLOT:--bcs}"; }
BCS_STACK_VERSION="${BCS_STACK_VERSION:-$(kubectl exec -n "${NAMESPACE}" "deploy/$(live_release 1)-host-listener-consumer" -- host_listener --stack-version)}"

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
values_dir="${root}/ci/preview-env/coprocessor"
COPROCESSOR_CHART="${COPROCESSOR_CHART:-${root}/charts/coprocessor}"
GCS_STACK_VERSION="${GCS_STACK_VERSION:-$(yq -r '.commonConfig.stackVersion' "${values_dir}/values-coprocessor-gcs-e2e.yaml")}"
# Green image tag = the tag the branch's coprocessor images were published under. Blue is pinned to
# the previous release and, on the production path, so are the contracts/relayer/test-suite, so the
# listener (never pinned) is the one deployed component that carries it; the checkout's HEAD is the
# fallback. Override with GCS_IMAGE_TAG whenever the deploy resolved a different tag - notably on a
# CI-only branch, where the coprocessor images are not rebuilt and the base commit carries them.
GCS_IMAGE_TAG="${GCS_IMAGE_TAG:-$(kubectl get deploy -n "${NAMESPACE}" listener-1-host \
  -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null | sed 's/.*://')}"
GCS_IMAGE_TAG="${GCS_IMAGE_TAG:-$(git -C "${root}" rev-parse --short=7 HEAD)}"
# Newest migration the Green migrator applies: from the commit the image tag names
# when this checkout has it (tags are short SHAs), else from the checkout itself.
migrations_dir="coprocessor/fhevm-engine/db-migration/migrations"
if git -C "${root}" cat-file -e "${GCS_IMAGE_TAG}^{commit}" 2>/dev/null; then
  head_migration=$(git -C "${root}" ls-tree --name-only "${GCS_IMAGE_TAG}" "${migrations_dir}/" | sort | tail -1 | xargs basename | cut -d_ -f1)
else
  head_migration=$(find "${root}/${migrations_dir}" -maxdepth 1 -type f | sort | tail -1 | xargs basename | cut -d_ -f1)
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
  helm get values "$(live_release "${party}")" -n "${NAMESPACE}" -o yaml > "${blue}"
  cp "${values_dir}/values-coprocessor-e2e.yaml" "${out}"
  BLUE="${blue}" yq -i '.commonConfig = load(strenv(BLUE)).commonConfig
    | .chains = [load(strenv(BLUE)).chains[] | select(.name == "host")]' "${out}"
  # The checkout's gatewayUrl is the anvil one; CI rewrites it per chain mode in apply-chain-env.sh,
  # which never runs here. Blue's own value is ws:// and the 0.15 tx-sender rejects that, so take
  # the HTTP endpoint from a deployed 0.15 kms-connector instead.
  local gw_http
  gw_http=$(kubectl get deploy -n "${NAMESPACE}" kms-connector-1-kms-connector-tx-sender \
    -o jsonpath='{.spec.template.spec.containers[0].env[?(@.name=="KMS_CONNECTOR_GATEWAY_URL")].value}' 2>/dev/null || true)
  if [[ -n "${gw_http}" ]]; then
    GW="${gw_http}" yq -i '.txSender.config.gatewayUrl.value = strenv(GW)' "${out}"
  else
    echo "::warning::could not resolve the gateway HTTP URL from kms-connector; Green keeps ${values_dir}'s default" >&2
  fi
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
    --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"$(green_fleet "${party}")\"}"
    --set-string "fullnameOverride=$(green_release "${party}")"
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

# The live fleet must be the one this round upgrades from, with no attempt still in progress
# (ingest.rs `can_replace` rejects a proposal while one is) and no schema for the target version.
# ALLOW_ACTIVE_ATTEMPT relaxes the attempt check for a wedged proposal only; see below.
live_ok() {
  local party="$1" v active schema ready
  v=$(psql_party "${party}" "SELECT stack_version FROM versioning;")
  [[ "$(version_mm "${v}")" == "$(version_mm "${BCS_STACK_VERSION}")" ]] \
    || fail "party ${party}: live stack is ${v}, expected ${BCS_STACK_VERSION}; run bg-reset.sh first"
  active=$(psql_party "${party}" "SELECT CASE WHEN to_regclass('upgrade_state') IS NULL THEN 0 ELSE (SELECT count(*) FROM upgrade_state WHERE status = 'in_progress') END;")
  if [[ "${active}" != "0" ]]; then
    # Only UpgradeActivated can be a proposal no Green ever picked up; the later states mean a round
    # is running, and reinstalling Green under it would disrupt a live dry run or cutover.
    local states
    states=$(psql_party "${party}" "SELECT DISTINCT state FROM upgrade_state WHERE status = 'in_progress';")
    [[ "${ALLOW_ACTIVE_ATTEMPT:-}" == "1" && "${states}" == "UpgradeActivated" ]] \
      || fail "party ${party}: ${active} attempt(s) in_progress (${states//$'\n'/,}); clear upgrade_state, or if this is a proposal no Green picked up, set ALLOW_ACTIVE_ATTEMPT=1"
    echo "party ${party}: ${active} attempt(s) in UpgradeActivated; starting Green to pick them up (ALLOW_ACTIVE_ATTEMPT=1)"
  fi
  local green_ver=""
  if helm status "$(green_release "${party}")" -n "${NAMESPACE}" >/dev/null 2>&1; then
    green_ver=$(helm get values "$(green_release "${party}")" -n "${NAMESPACE}" -o json 2>/dev/null \
      | jq -r '.commonConfig.stackVersion // ""')
    [[ "${green_ver}" == "${GCS_STACK_VERSION}" ]] \
      || fail "party ${party}: $(green_release "${party}") already installed at ${green_ver:-unknown}, not ${GCS_STACK_VERSION}"
    echo "party ${party}: Green ${GCS_STACK_VERSION} already installed, resuming"
  else
    schema=$(psql_party "${party}" "SELECT count(*) FROM pg_namespace WHERE nspname = 'gcs-${GCS_STACK_VERSION}';")
    [[ "${schema}" == "0" ]] || fail "party ${party}: schema gcs-${GCS_STACK_VERSION} exists with no Green release; run bg-reset.sh first"
  fi
  ready=$(kubectl get deploy -n "${NAMESPACE}" "$(live_release "${party}")-host-listener-consumer" -o jsonpath='{.status.readyReplicas}')
  [[ "${ready}" == "1" ]] || fail "party ${party}: live consumer not ready"
}

echo "== bg-green ${verb}: ${NAMESPACE}, ${NB_COPROCESSOR} parties, Green image tag ${GCS_IMAGE_TAG}, stack ${GCS_STACK_VERSION}, HEAD migration ${head_migration}"

case "${verb}" in
migrate)
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    live_ok "${i}"
    before=$(psql_party "${i}" "SELECT max(version)||' ('||count(*)||')' FROM _sqlx_migrations;")
    values=$(green_values "${i}")
    green_helm_args "${i}" "${values}"
    job="$(green_release "${i}")-db-migration-1"
    # Same Job the Green release runs as its pre-install hook, rendered without the
    # hook so it runs now, on its own, while Blue serves. `start` re-runs it as a no-op.
    helm template "$(green_release "${i}")" "${COPROCESSOR_CHART}" "${GREEN_ARGS[@]}" \
      --show-only templates/coprocessor-db-migration.yaml > "${work}/migrate-${i}.yaml"
    kubectl delete job "${job}" -n "${NAMESPACE}" --ignore-not-found >/dev/null
    kubectl apply -n "${NAMESPACE}" -f "${work}/migrate-${i}.yaml"
    if ! kubectl wait --for=condition=complete "job/${job}" -n "${NAMESPACE}" --timeout=300s; then
      # Report why: an unbuilt Green tag shows up here as a pull failure, not as a log line.
      kubectl get pod -n "${NAMESPACE}" -l "job-name=${job}" \
        -o jsonpath='{range .items[*]}{.status.containerStatuses[*].state.waiting.reason}{" "}{.status.containerStatuses[*].state.waiting.message}{"\n"}{end}' || true
      kubectl logs -n "${NAMESPACE}" "job/${job}" --tail=50 || true
      fail "party ${i}: migration job did not complete (Green tag ${GCS_IMAGE_TAG}; if the image is missing, the coprocessor was not built at that commit - set GCS_IMAGE_TAG to a commit whose coprocessor images were published)"
    fi
    after=$(psql_party "${i}" "SELECT max(version)||' ('||count(*)||')' FROM _sqlx_migrations;")
    [[ "${after}" == "${head_migration} ("* ]] || fail "party ${i}: DB at ${after}, expected ${head_migration}"
    echo "party ${i}: migrations ${before} -> ${after}; live still $(psql_party "${i}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1') FROM versioning v;")"
  done
  echo "== migrate done: schema at ${head_migration} on every party, the live fleet untouched. Next: bg-green.sh start"
  ;;

start)
  green_releases=()
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    live_ok "${i}"
    at=$(psql_party "${i}" "SELECT max(version) FROM _sqlx_migrations;")
    [[ "${at}" == "${head_migration}" ]] || fail "party ${i}: schema at ${at}, run bg-green.sh migrate first"
  done
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    values=$(green_values "${i}")
    green_helm_args "${i}" "${values}"
    echo "party ${i}: installing $(green_release "${i}")"
    green_releases+=("$(green_release "${i}")")
    helm upgrade --install "$(green_release "${i}")" "${COPROCESSOR_CHART}" "${GREEN_ARGS[@]}" \
      --set-json 'dbMigration.annotations={"helm.sh/hook":"pre-install,pre-upgrade","helm.sh/hook-delete-policy":"before-hook-creation"}' \
      >/dev/null
    # Green pollers: twins of the Blue poller releases (HEAD image, Green fleet). They inject the
    # synthetic host anchors and, in gcs mode, wait for the Green schema the controller creates.
    for rel in "coprocessor-poller-${i}${LIVE_RELEASE_SUFFIX}" $([[ "${DEPLOY_POLYGON}" == "true" ]] && echo "coprocessor-poller-polygon-${i}${LIVE_RELEASE_SUFFIX}"); do
      helm get values "${rel}" -n "${NAMESPACE}" -o yaml > "${work}/${rel}.yaml"
      green_rel="${rel%"${LIVE_RELEASE_SUFFIX}"}${GREEN_SLOT}"
      echo "party ${i}: installing ${green_rel}"
      green_releases+=("${green_rel}")
      helm upgrade --install "${green_rel}" "${COPROCESSOR_CHART}" -n "${NAMESPACE}" \
        -f "${work}/${rel}.yaml" \
        --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"$(green_fleet "${i}")\"}" \
        --set-string "commonConfig.stackVersion=${GCS_STACK_VERSION}" \
        --set-string "hostListenerPollerShared.image.tag=${GCS_IMAGE_TAG}" \
        >/dev/null
    done
    if [[ "${DEPLOY_POLYGON}" == "true" ]]; then
      # Twin of the Blue Polygon consumer: its values, Green image/version/fleet and an
      # own consumer name (the Redis group is <service-name>.<chain-id>).
      helm get values "coprocessor-polygon-${i}${LIVE_RELEASE_SUFFIX}" -n "${NAMESPACE}" -o yaml > "${work}/polygon-${i}.yaml"
      echo "party ${i}: installing coprocessor-polygon-${i}${GREEN_SLOT}"
      green_releases+=("coprocessor-polygon-${i}${GREEN_SLOT}")
      helm upgrade --install "coprocessor-polygon-${i}${GREEN_SLOT}" "${COPROCESSOR_CHART}" -n "${NAMESPACE}" \
        -f "${work}/polygon-${i}.yaml" \
        --set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"$(green_fleet "${i}")\"}" \
        --set-string "commonConfig.stackVersion=${GCS_STACK_VERSION}" \
        --set-string "hostListenerConsumerShared.image.tag=${GCS_IMAGE_TAG}" \
        --set-string "hostListenerConsumerShared.args.serviceName=host-listener-consumer${GREEN_SLOT:--bcs}" \
        >/dev/null
    fi
  done
  # By release rather than by name pattern: with GREEN_SLOT="" the Green names carry no marker.
  for rel in "${green_releases[@]}"; do
    for d in $(helm get manifest "${rel}" -n "${NAMESPACE}" \
                 | yq -r 'select(.kind == "Deployment") | .metadata.name' | grep -v '^---$'); do
      kubectl rollout status "deploy/${d}" -n "${NAMESPACE}" --timeout=300s >/dev/null
    done
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
    # The binary, not the helm label: a Green built from the wrong commit otherwise surfaces only
    # as a rejected proposal later in the round. Exact, since 0.15.0 vs 0.15.1 is the whole point.
    green_bin=$(kubectl exec -n "${NAMESPACE}" "deploy/$(green_release "${i}")-host-listener-consumer" \
      -- host_listener --stack-version 2>/dev/null | tr -d '\r' || true)
    [[ "${green_bin#v}" == "${GCS_STACK_VERSION#v}" ]] \
      || { echo "::error::party ${i}: Green binary reports ${green_bin:-<none>}, expected ${GCS_STACK_VERSION} (image ${GCS_IMAGE_TAG})"; failed=1; }
    live=$(psql_party "${i}" "SELECT stack_version||' in_progress='||(SELECT count(*) FROM upgrade_state WHERE status = 'in_progress') FROM versioning;")
    [[ "$(version_mm "${live%% *}")" == "$(version_mm "${BCS_STACK_VERSION}")" && "${live#* }" == "in_progress=0" ]] || { echo "::error::party ${i}: live state changed: ${live}"; failed=1; }
    for d in upgrade-controller consensus-detector; do
      if kubectl logs -n "${NAMESPACE}" "deploy/$(green_release "${i}")-${d}" --tail=200 2>/dev/null | grep '"level":"ERROR"' >/dev/null; then
        echo "::warning::party ${i}: ${d} logged errors, check: kubectl logs -n ${NAMESPACE} deploy/$(green_release "${i}")-${d}"
      fi
    done
    for p in "coprocessor-poller-${i}${GREEN_SLOT}-host-listener-poller" "coprocessor-poller-polygon-${i}${GREEN_SLOT}-host-listener-poller"; do
      kubectl get deploy -n "${NAMESPACE}" "${p}" >/dev/null 2>&1 || continue
      if kubectl logs -n "${NAMESPACE}" "deploy/${p}" --tail=3 2>/dev/null | grep "waiting for the upgrade-controller to create the GCS schema" >/dev/null; then
        echo "::warning::${p} still waiting for the Green schema"
      fi
    done
  done
  [[ "${failed}" == "0" ]] || fail "Green installed but verification failed, see errors above"
  echo "== start done: Green ${GCS_STACK_VERSION} (${GCS_IMAGE_TAG}) shadowing live ${BCS_STACK_VERSION} on ${NB_COPROCESSOR} parties. Next: propose the upgrade."
  ;;
*)
  fail "unknown verb '${verb}' (migrate|start)"
  ;;
esac
