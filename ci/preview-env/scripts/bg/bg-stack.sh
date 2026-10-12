#!/usr/bin/env bash
# Upgrade the non-coprocessor components (relayer, kms-connector, test-suite) to the target
# release, in place, mid-round.
#
# Why this exists: Blue/Green covers the coprocessor only, because it is the one component that
# cannot be swapped with downtime. Everything else upgrades conventionally, and on the production
# path it must upgrade *before* the coprocessor cutover — otherwise the decryption checkpoints
# measure the old component's behaviour rather than the cutover. Concretely: the 0.14 kms-connector
# cross-checks the attested ciphertext digest against the on-chain SnsCiphertextMaterial
# (`compare_onchain`), which 0.15 dropped in favour of RFC-023 off-chain attestation consensus.
# With the connector left at 0.14, handles the Blue stack committed inside the upgrade window stop
# decrypting at cutover (fhevm-internal#2040).
#
# Run it in step 4 of the round, together with `bg-contracts.sh upgrade`.
#
# Verbs:
#   status   print the deployed tag of every component this script manages
#   upgrade  helm-upgrade each release to TARGET_TAG, reusing its existing values
#
# Env:
#   NAMESPACE (required)
#   TARGET_TAG (default: the coprocessor Green tag, else the listener image tag)
#   COMPONENTS (default: "kms-connector relayer test-suite")
#   KMS_CONNECTOR_CHART (default: the checkout's charts/kms-connector)
#   COMMON_CHART / COMMON_CHART_VERSION (default: the values the deploy workflow uses; pulling it
#     needs registry credentials, so this half only works where those are available)
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
KMS_CONNECTOR_CHART="${KMS_CONNECTOR_CHART:-${root}/charts/kms-connector}"
COMMON_CHART="${COMMON_CHART:-oci://hub.zama.org/ghcr/zama-zws/helm-charts/common}"
COMMON_CHART_VERSION="${COMMON_CHART_VERSION:-0.3.3}"
COMPONENTS="${COMPONENTS:-kms-connector relayer test-suite}"
# Same precedence as bg-green.sh: the Green coprocessor carries the tag the round upgrades to.
GREEN_SLOT="${GREEN_SLOT--gcs}"
fail() { echo "::error::$*" >&2; exit 1; }
# Green is not up yet: derive the tag the deploy resolved for the coprocessor. Rebuilt on this
# branch means HEAD's short SHA, otherwise the merge-base's. The listener is a separate component
# with its own change detection, so its tag matches the coprocessor's only by coincidence.
coprocessor_tag() {
  local base
  base=$(git -C "${root}" merge-base HEAD origin/main 2>/dev/null || true)
  [[ -n "${base}" ]] || return 1
  if git -C "${root}" diff --quiet "${base}" HEAD -- coprocessor/ 2>/dev/null; then
    git -C "${root}" rev-parse --short=7 "${base}"
  else
    git -C "${root}" rev-parse --short=7 HEAD
  fi
}
# `|| true`: before Green is started this lookup fails, and under `set -e` + pipefail a bare
# substitution would kill the script here.
TARGET_TAG="${TARGET_TAG:-$(kubectl get deploy -n "${NAMESPACE}" "coprocessor-1${GREEN_SLOT}-tx-sender" \
  -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null | sed 's/.*://' || true)}"
TARGET_TAG="${TARGET_TAG:-$(coprocessor_tag || true)}"
[[ -n "${TARGET_TAG}" ]] || fail "could not resolve TARGET_TAG (no Green fleet and no merge-base with origin/main); pass TARGET_TAG=<Green tag>"

verb="${1:-status}"

nb_kms=$(helm list -n "${NAMESPACE}" -o json | jq '[.[] | select(.name | test("^kms-connector-[0-9]+$"))] | length')

# Deployed tag of one container, by deployment name.
deployed_tag() {
  # `|| true`: pipefail would turn a missing deployment into a fatal error.
  kubectl get deploy -n "${NAMESPACE}" "$1" \
    -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null | sed 's/.*://' || true
}

case "${verb}" in
status)
  echo "== bg-stack status: ${NAMESPACE}, target ${TARGET_TAG}"
  for i in $(seq 1 "${nb_kms}"); do
    for c in gw-listener kms-worker tx-sender; do
      t=$(deployed_tag "kms-connector-${i}-kms-connector-${c}")
      printf '   kms-connector-%s %-12s %s%s\n' "${i}" "${c}" "${t:-<none>}" \
        "$([[ "${t}" == "${TARGET_TAG}" ]] && echo "" || echo "   (not at target)")"
    done
  done
  t=$(deployed_tag relayer)
  printf '   %-14s %s%s\n' "relayer" "${t:-<none>}" \
    "$([[ "${t}" == "${TARGET_TAG}" ]] && echo "" || echo "   (not at target)")"
  # The idle test-suite runs as a Job, not a Deployment.
  t=$(kubectl get job -n "${NAMESPACE}" test-suite \
    -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null | sed 's/.*://' || true)
  printf '   %-14s %s%s\n' "test-suite" "${t:-<none>}" \
    "$([[ "${t}" == "${TARGET_TAG}" ]] && echo "" || echo "   (not at target)")"
  # Versioned separately from fhevm and upgraded by its own procedure; shown for context only.
  kt=$(kubectl get statefulset -n "${NAMESPACE}" kms-core-1-core \
    -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null | sed 's/.*://')
  printf '   %-14s %s   (separate release train, not upgraded here)\n' "kms-core" "${kt:-<none>}"
  ;;

upgrade)
  echo "== bg-stack upgrade: ${NAMESPACE} -> ${TARGET_TAG}, components: ${COMPONENTS}"
  reg=hub.zama.org/ghcr/zama-ai/fhevm

  if [[ " ${COMPONENTS} " == *" kms-connector "* ]]; then
    [[ "${nb_kms}" -gt 0 ]] || fail "no kms-connector releases found"
    for i in $(seq 1 "${nb_kms}"); do
      echo "kms-connector-${i}: upgrading to ${TARGET_TAG}"
      # --reuse-values keeps every address, endpoint and wallet the deploy resolved; only the
      # six image tags move. The chart runs its own db-migration, so a connector schema change
      # between the releases is applied here.
      helm upgrade "kms-connector-${i}" "${KMS_CONNECTOR_CHART}" -n "${NAMESPACE}" --reuse-values \
        --set-string "kmsConnectorDbMigration.image.name=${reg}/kms-connector/db-migration" \
        --set-string "kmsConnectorDbMigration.image.tag=${TARGET_TAG}" \
        --set-string "kmsConnectorGwListener.image.name=${reg}/kms-connector/gw-listener" \
        --set-string "kmsConnectorGwListener.image.tag=${TARGET_TAG}" \
        --set-string "kmsConnectorKmsWorker.image.name=${reg}/kms-connector/kms-worker" \
        --set-string "kmsConnectorKmsWorker.image.tag=${TARGET_TAG}" \
        --set-string "kmsConnectorTxSender.image.name=${reg}/kms-connector/tx-sender" \
        --set-string "kmsConnectorTxSender.image.tag=${TARGET_TAG}" \
        --set-string "kmsConnectorEndpoint.image.name=${reg}/kms-connector/endpoint" \
        --set-string "kmsConnectorEndpoint.image.tag=${TARGET_TAG}" \
        --set-string "kmsConnectorProxy.image.name=${reg}/kms-connector/proxy" \
        --set-string "kmsConnectorProxy.image.tag=${TARGET_TAG}" >/dev/null
    done
    for i in $(seq 1 "${nb_kms}"); do
      for c in gw-listener kms-worker tx-sender endpoint proxy; do
        # A release predating endpoint/proxy has no such Deployment; nothing to wait for.
        kubectl get deploy -n "${NAMESPACE}" "kms-connector-${i}-kms-connector-${c}" >/dev/null 2>&1 || continue
        kubectl rollout status -n "${NAMESPACE}" "deploy/kms-connector-${i}-kms-connector-${c}" \
          --timeout=300s >/dev/null || fail "kms-connector-${i}-${c} did not become ready"
      done
    done
    echo "kms-connector: ${nb_kms} release(s) at ${TARGET_TAG}"
  fi

  for comp in relayer test-suite; do
    [[ " ${COMPONENTS} " == *" ${comp} "* ]] || continue
    echo "${comp}: upgrading to ${TARGET_TAG}"
    helm upgrade "${comp}" "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
      -n "${NAMESPACE}" --reuse-values --set-string "image.tag=${TARGET_TAG}" >/dev/null \
      || fail "${comp}: helm upgrade failed (pulling ${COMMON_CHART} needs registry credentials)"
  done
  [[ " ${COMPONENTS} " == *" relayer "* ]] && { kubectl rollout status -n "${NAMESPACE}" \
    deploy/relayer --timeout=300s >/dev/null || fail "relayer did not become ready"; }

  echo "== upgrade done"
  "${BASH_SOURCE[0]}" status
  ;;

*)
  fail "unknown verb '${verb}' (status|upgrade)"
  ;;
esac
