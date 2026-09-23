#!/usr/bin/env bash
# In-namespace Prometheus + Jaeger + Grafana via the common chart.
# Prometheus's RoleBinding SA subject must name the namespace explicitly
# (same yq patch as the test-suite-workflow release).
# Env: NAMESPACE, COMMON_CHART, COMMON_CHART_VERSION.
set -euo pipefail

prom_values=$(mktemp --suffix=.yaml)
cp ci/preview-env/observability/values-prometheus-e2e.yaml "${prom_values}"
NS="${NAMESPACE}" yq -i \
  '(.additionalResources[] | select(.object.kind == "RoleBinding") | .object.subjects[0].namespace) = strenv(NS)' \
  "${prom_values}"

pids=()
helm upgrade --install prometheus "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
  -n "${NAMESPACE}" -f "${prom_values}" &
pids+=($!)
helm upgrade --install jaeger "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
  -n "${NAMESPACE}" -f ci/preview-env/observability/values-jaeger-e2e.yaml &
pids+=($!)
helm upgrade --install grafana "${COMMON_CHART}" --version "${COMMON_CHART_VERSION}" \
  -n "${NAMESPACE}" -f ci/preview-env/observability/values-grafana-e2e.yaml &
pids+=($!)

rc=0
for pid in "${pids[@]}"; do
  wait "${pid}" || rc=1
done
if [[ "${rc}" -ne 0 ]]; then
  echo "::error::at least one observability release failed to install - see the helm output above"
  exit 1
fi
