#!/usr/bin/env bash
# Per-party coprocessor sidecars: Polygon consumer, ETH poller, Polygon poller.
# Usage: deploy-coprocessor-sidecars.sh <polygon-consumer|poller|poller-polygon>
# Env: NAMESPACE, NB_COPROCESSOR, COPROCESSOR_CHART, TAGS_JSON, OBSERVABILITY, OTLP_ENDPOINT.
set -euo pipefail

kind="${1:?kind}"
case "${kind}" in
  polygon-consumer)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "coprocessor-polygon-${i}" "${COPROCESSOR_CHART}" \
        -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-polygon-consumer-e2e.yaml \
        --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-${i}:5432" \
        --set-string "hostListenerConsumerShared.serviceAccountName=coprocessor-${i}" \
        --set-string "hostListenerConsumerShared.config.brokerUrl.value=redis://coprocessor-redis-${i}-master:6379" \
        --set-string "hostListenerConsumerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")" &
    done
    wait
    ;;
  poller)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      tracing=()
      if [[ "${OBSERVABILITY}" == "true" ]]; then
        tracing=(
          --set "commonConfig.tracing.enabled=true"
          --set-string "commonConfig.tracing.endpoint=${OTLP_ENDPOINT}"
          --set-string "hostListenerPollerShared.tracing.service=host-listener-poller-${i}"
        )
      fi
      helm upgrade --install "coprocessor-poller-${i}" "${COPROCESSOR_CHART}" \
        -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-poller-e2e.yaml \
        --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-${i}:5432" \
        --set-string "hostListenerPollerShared.serviceAccountName=coprocessor-${i}" \
        --set-string "hostListenerPollerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")" \
        "${tracing[@]}" &
    done
    wait
    ;;
  poller-polygon)
    for i in $(seq 1 "${NB_COPROCESSOR}"); do
      helm upgrade --install "coprocessor-poller-polygon-${i}" "${COPROCESSOR_CHART}" \
        -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-poller-polygon-e2e.yaml \
        --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-${i}:5432" \
        --set-string "hostListenerPollerShared.serviceAccountName=coprocessor-${i}" \
        --set-string "hostListenerPollerShared.image.tag=$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")" &
    done
    wait
    ;;
  *)
    echo "::error::unknown coprocessor-sidecar kind '${kind}'" >&2
    exit 1
    ;;
esac
