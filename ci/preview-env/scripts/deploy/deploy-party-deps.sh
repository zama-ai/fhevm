#!/usr/bin/env bash
# Per-party Redis + listener (ETH, optional Polygon / BNB).
# Usage: deploy-party-deps.sh <redis|listener|listener-polygon|listener-bnb>
# Env: NAMESPACE, NB_COPROCESSOR, REDIS_CHART, REDIS_CHART_VERSION,
# LISTENER_CHART, TAGS_JSON.
set -euo pipefail

kind="${1:?kind}"
case "${kind}" in
  redis)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "coprocessor-redis-${i}" "${REDIS_CHART}" --version "${REDIS_CHART_VERSION}" \
        -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-redis-e2e.yaml \
        --set-string "fullnameOverride=coprocessor-redis-${i}" --wait &
    done
    wait
    ;;
  listener)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "listener-${i}" "${LISTENER_CHART}" \
        -n "${NAMESPACE}" -f ci/preview-env/listener/values-listener-e2e.yaml \
        --set-string "image.tag=$(jq -r .listener <<<"${TAGS_JSON}")" \
        --set-string "env[0].value=postgresql://zama:zama@postgres-listener-${i}:5432/listener?sslmode=disable" \
        --set-string "env[1].value=redis://coprocessor-redis-${i}-master:6379" &
    done
    wait
    ;;
  listener-polygon|listener-bnb)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "${kind}-${i}" "${LISTENER_CHART}" \
        -n "${NAMESPACE}" -f "ci/preview-env/listener/values-${kind}-e2e.yaml" \
        --set-string "image.tag=$(jq -r .listener <<<"${TAGS_JSON}")" \
        --set-string "env[0].value=postgresql://zama:zama@postgres-${kind}-${i}:5432/listener?sslmode=disable" \
        --set-string "env[1].value=redis://coprocessor-redis-${i}-master:6379" &
    done
    wait
    ;;
  *)
    echo "::error::unknown party-dep kind '${kind}'" >&2
    exit 1
    ;;
esac
