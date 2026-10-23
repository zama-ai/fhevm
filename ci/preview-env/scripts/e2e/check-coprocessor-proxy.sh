#!/usr/bin/env bash
# Cluster checks of coprocessor-proxy that the e2e suite cannot make from inside a test:
#   - every party's proxy rolled out with all its replicas Ready;
#   - a rolling restart of every proxy under a steady load of signed requests fails none of them,
#     and every new pod became Ready only after its first read of the KMS tx-senders;
#   - the NetworkPolicy: a test-suite pod reaches each prf-service's metrics port (9100) but
#     not its PRF port (8090). Reported only: whether zws-dev enforces NetworkPolicy is unknown.
# The load runs the "through a rolling restart" test of test-suite/e2e/test/coprocessorProxy in a
# pod of the test-suite image.
# Env: NAMESPACE, NB_COPROCESSOR, E2E_IMAGE, HOST_CHAIN_ID.
set -euo pipefail

: "${NAMESPACE:?}" "${NB_COPROCESSOR:?}" "${E2E_IMAGE:?}" "${HOST_CHAIN_ID:?}"
# Every party's proxy restarts at once and must finish within restart_seconds, its old pods gone
# after draining for up to their grace period. The load stops once they are gone; it is bounded by
# that budget and the 5 s wait for its start.
restart_seconds=600
load_seconds=$((restart_seconds + 60))
pod=coprocessor-proxy-check
stop_file=/tmp/coprocessor-proxy-load-stop
deployments=()
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  deployments+=("coprocessor-${i}-coprocessor-proxy")
done

for deploy in "${deployments[@]}"; do
  kubectl rollout status "deploy/${deploy}" -n "${NAMESPACE}" --timeout=300s
  ready=$(kubectl get "deploy/${deploy}" -n "${NAMESPACE}" -o jsonpath='{.status.readyReplicas}')
  wanted=$(kubectl get "deploy/${deploy}" -n "${NAMESPACE}" -o jsonpath='{.spec.replicas}')
  if [[ "${ready}" != "${wanted}" ]]; then
    echo "::error::${deploy}: ${ready:-0} of ${wanted} replicas Ready"
    exit 1
  fi
  echo "${deploy}: ${ready} of ${wanted} replicas Ready"
done

# Probes each prf-service's PRF and metrics ports, then runs the load. JavaScript, not for the shell:
# shellcheck disable=SC2016
netcheck_js='
const net = require("net");
const hosts = process.env.PRF_SERVICE_METRICS_URLS.split(",").map((url) => new URL(url).hostname);
const probe = (host, port) => new Promise((resolve) => {
  const socket = net.connect({ host, port });
  socket.setTimeout(3000);
  socket.on("connect", () => { socket.destroy(); resolve("open"); });
  socket.on("timeout", () => { socket.destroy(); resolve("blocked"); });
  socket.on("error", (err) => resolve(`blocked (${err.code})`));
});
(async () => {
  for (const host of hosts) {
    for (const port of [8090, 9100]) console.log(`networkpolicy: ${host}:${port} ${await probe(host, port)}`);
  }
})();
'
kubectl delete pod "${pod}" -n "${NAMESPACE}" --ignore-not-found --wait=true
# The container's shell, not this one, expands $NETCHECK_JS.
# shellcheck disable=SC2016
NETCHECK_JS="${netcheck_js}" POD="${pod}" LOAD_SECONDS="${load_seconds}" STOP_FILE="${stop_file}" yq -n '
{
  "apiVersion": "v1",
  "kind": "Pod",
  "metadata": {"name": strenv(POD), "labels": {"app": "fhevm-test-suite"}},
  "spec": {
    "restartPolicy": "Never",
    "imagePullSecrets": [{"name": "registry-credentials"}],
    "volumes": [{"name": "ca", "configMap": {"name": "coprocessor-proxy", "items": [{"key": "ca.crt", "path": "ca.crt"}]}}],
    "containers": [{
      "name": "check",
      "image": strenv(E2E_IMAGE),
      "workingDir": "/app/test-suite/e2e",
      "command": ["sh", "-c", "node -e \"$NETCHECK_JS\" && npx mocha -r ts-node/register --grep \"answers every signed request\" test/coprocessorProxy/coprocessorProxy.ts"],
      "volumeMounts": [{"name": "ca", "mountPath": "/etc/coprocessor-proxy", "readOnly": true}],
      "env": [
        {"name": "NETCHECK_JS", "value": strenv(NETCHECK_JS)},
        {"name": "TS_NODE_TRANSPILE_ONLY", "value": "true"},
        {"name": "COPROCESSOR_PROXY_LOAD_SECONDS", "value": strenv(LOAD_SECONDS)},
        {"name": "COPROCESSOR_PROXY_LOAD_STOP_FILE", "value": strenv(STOP_FILE)},
        {"name": "COPROCESSOR_PROXY_CA_CERT", "value": "/etc/coprocessor-proxy/ca.crt"},
        {"name": "CHAIN_ID_HOST", "value": strenv(HOST_CHAIN_ID)},
        {"name": "PROTOCOL_CONFIG_CONTRACT_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "host-sc-addresses", "key": "protocol_config.address"}}},
        {"name": "COPROCESSOR_PROXY_URLS", "valueFrom": {"configMapKeyRef": {"name": "coprocessor-proxy", "key": "urls"}}},
        {"name": "COPROCESSOR_SIGNER_ADDRESSES", "valueFrom": {"configMapKeyRef": {"name": "coprocessor-proxy", "key": "signer-addresses"}}},
        {"name": "PRF_SERVICE_METRICS_URLS", "valueFrom": {"configMapKeyRef": {"name": "coprocessor-proxy", "key": "prf-service-metrics-urls"}}},
        {"name": "COPROCESSOR_PROXY_METRICS_URLS", "valueFrom": {"configMapKeyRef": {"name": "coprocessor-proxy", "key": "metrics-urls"}}},
        {"name": "KMS_CONNECTOR_PRIVATE_KEY", "valueFrom": {"secretKeyRef": {"name": "kms-connector-tx-sender-1", "key": "kms-wallet"}}}
      ]
    }]
  }
}' | kubectl apply -n "${NAMESPACE}" -f -

started_load() { kubectl logs "${pod}" -n "${NAMESPACE}" 2>/dev/null | grep -q '\[coprocessor-proxy\] load started'; }
phase() { kubectl get pod "${pod}" -n "${NAMESPACE}" -o jsonpath='{.status.phase}'; }
deadline=$((SECONDS + 600))
until started_load; do
  if [[ "$(phase)" == "Failed" || "$(phase)" == "Succeeded" || ${SECONDS} -ge ${deadline} ]]; then
    kubectl logs "${pod}" -n "${NAMESPACE}" || true
    echo "::error::${pod} did not start its load (phase $(phase))"
    exit 1
  fi
  sleep 5
done
kubectl logs "${pod}" -n "${NAMESPACE}" | grep '^networkpolicy:'

old_pods=()
for deploy in "${deployments[@]}"; do
  while IFS= read -r p; do
    old_pods+=("${p}")
  done < <(kubectl get pods -n "${NAMESPACE}" -l "app=coprocessor-proxy,app.kubernetes.io/name=${deploy}" -o name)
done
restart_started=$(date -u +%s)
restart_deadline=$((SECONDS + restart_seconds))
# A zero timeout would wait forever.
remaining() { local left=$((restart_deadline - SECONDS)); echo "$((left > 0 ? left : 1))s"; }
for deploy in "${deployments[@]}"; do
  kubectl rollout restart "deploy/${deploy}" -n "${NAMESPACE}"
done
for deploy in "${deployments[@]}"; do
  kubectl rollout status "deploy/${deploy}" -n "${NAMESPACE}" --timeout="$(remaining)"
done
# An old pod keeps serving its held requests until it exits.
kubectl wait --for=delete "${old_pods[@]}" -n "${NAMESPACE}" --timeout="$(remaining)"
# The load must still be running, or it did not cover the whole restart.
if [[ "$(phase)" != "Running" ]]; then
  kubectl logs "${pod}" -n "${NAMESPACE}" || true
  echo "::error::the load ended before the rolling restart did (phase $(phase))"
  exit 1
fi
kubectl exec "${pod}" -n "${NAMESPACE}" -- touch "${stop_file}"

# Every pod that started during the restart became Ready only after its first read.
not_ready_before_read=true
for deploy in "${deployments[@]}"; do
  for p in $(kubectl get pods -n "${NAMESPACE}" -l "app=coprocessor-proxy,app.kubernetes.io/name=${deploy}" -o name); do
    started=$(date -u -d "$(kubectl get "${p}" -n "${NAMESPACE}" -o jsonpath='{.metadata.creationTimestamp}')" +%s)
    if (( started < restart_started )); then
      continue
    fi
    ready_at=$(kubectl get "${p}" -n "${NAMESPACE}" -o jsonpath='{.status.conditions[?(@.type=="Ready")].lastTransitionTime}')
    first_read=$(kubectl logs "${p}" -n "${NAMESPACE}" --timestamps | grep -m1 'KMS tx-sender set updated' | cut -d' ' -f1 || true)
    if [[ -z "${first_read}" ]]; then
      echo "::error::${p}: Ready at ${ready_at} with no read of the KMS tx-senders in its log"
      not_ready_before_read=false
    elif (( $(date -u -d "${ready_at}" +%s) < $(date -u -d "${first_read}" +%s) )); then
      echo "::error::${p}: Ready at ${ready_at}, before its first read at ${first_read}"
      not_ready_before_read=false
    else
      echo "${p}: first read at ${first_read}, Ready at ${ready_at}"
    fi
  done
done

deadline=$((SECONDS + load_seconds + 300))
until [[ "$(phase)" == "Succeeded" || "$(phase)" == "Failed" || ${SECONDS} -ge ${deadline} ]]; do
  sleep 10
done
kubectl logs "${pod}" -n "${NAMESPACE}" | grep -v '^networkpolicy:' || true
load_phase=$(phase)

# Report only. Enforced: 8090 blocked and 9100 open on every prf-service.
logs=$(kubectl logs "${pod}" -n "${NAMESPACE}" | grep '^networkpolicy:')
if grep -q ':8090 open' <<<"${logs}"; then
  echo "NetworkPolicy NOT enforced: a test-suite pod reached a prf-service's PRF port"
elif grep -q ':9100 blocked' <<<"${logs}"; then
  echo "NetworkPolicy inconclusive: a test-suite pod could not reach a prf-service's metrics port either"
else
  echo "NetworkPolicy enforced: only the metrics port of each prf-service is reachable from a test-suite pod"
fi

if [[ "${load_phase}" != "Succeeded" ]]; then
  echo "::error::the load through the rolling restart failed (phase ${load_phase})"
  exit 1
fi
if [[ "${not_ready_before_read}" != "true" ]]; then
  exit 1
fi
echo "coprocessor-proxy: rolling restart answered every request; no pod was Ready before its first read"
