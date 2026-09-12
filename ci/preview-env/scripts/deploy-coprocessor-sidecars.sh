#!/usr/bin/env bash
# Per-party coprocessor sidecars: Polygon consumer (BCS + GCS twins under blue-green), ETH poller, Polygon poller.
# Usage: deploy-coprocessor-sidecars.sh <polygon-consumer|polygon-consumer-gcs|poller|poller-polygon>
# Env: NAMESPACE, NB_COPROCESSOR, COPROCESSOR_CHART, TAGS_JSON, OBSERVABILITY, OTLP_ENDPOINT;
#      blue-green also BLUE_GREEN, BCS_IMAGE_TAG, GCS_STACK_VERSION.
set -euo pipefail

kind="${1:?kind}"
head_tag="$(jq -r .coprocessor_host_listener <<<"${TAGS_JSON}")"

# polygon_consumer <release-suffix> <image-tag> <fleet|""> [extra helm args...]
polygon_consumer() {
  local suffix="$1" tag="$2" fleet="$3"; shift 3
  for i in $(seq 1 "${NB_COPROCESSOR}"); do
    fleet_args=()
    [[ -z "${fleet}" ]] || fleet_args=(--set-json "commonConfig.extraSelectorLabels={\"fhevm.zama.ai/fleet\":\"${i}-${fleet}\"}")
    helm upgrade --install "coprocessor-polygon-${i}${suffix}" "${COPROCESSOR_CHART}" \
      -n "${NAMESPACE}" -f ci/preview-env/coprocessor/values-coprocessor-polygon-consumer-e2e.yaml \
      --set-string "commonConfig.databaseEndpoint.value=postgres-coprocessor-${i}:5432" \
      --set-string "hostListenerConsumerShared.serviceAccountName=coprocessor-${i}" \
      --set-string "hostListenerConsumerShared.config.brokerUrl.value=redis://coprocessor-redis-${i}-master:6379" \
      --set-string "hostListenerConsumerShared.image.tag=${tag}" \
      ${fleet_args[@]+"${fleet_args[@]}"} "$@" &
  done
  wait
}

case "${kind}" in
  polygon-consumer)
    if [[ "${BLUE_GREEN:-false}" == "true" ]]; then
      polygon_consumer "" "${BCS_IMAGE_TAG:?}" "bcs"
    else
      polygon_consumer "" "${head_tag}" ""
    fi
    ;;
  polygon-consumer-gcs)
    # Own consumer name: the Redis consumer id is <service-name>.<chain-id>, so the default would split the stream with BCS.
    gcs_version="${GCS_STACK_VERSION:-$(yq -r '.commonConfig.stackVersion' ci/preview-env/coprocessor/values-coprocessor-gcs-e2e.yaml)}"
    polygon_consumer "-gcs" "${head_tag}" "gcs" \
      --set-string "commonConfig.stackVersion=${gcs_version}" \
      --set-string "hostListenerConsumerShared.args.serviceName=host-listener-consumer-gcs"
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
