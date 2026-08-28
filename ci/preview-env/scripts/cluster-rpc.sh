#!/usr/bin/env bash
# JSON-RPC from inside the preview namespace so ClusterIP DNS works.
# Prints the RPC JSON body to stdout and nothing else.
# Usage: cluster-rpc.sh <url> [json-body]
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"
url="${1:?url required}"
body="${2:-}"
job="preview-rpc-$(date +%s)-${RANDOM}"

cleanup() {
  kubectl delete pod,configmap -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null 2>&1 || true
}
trap cleanup EXIT

kubectl create configmap "${job}" -n "${NAMESPACE}" \
  --from-literal=url="${url}" \
  --from-literal=body="${body}" \
  --dry-run=client -o yaml | kubectl apply -f - >/dev/null

kubectl delete pod -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null 2>&1 || true
kubectl apply -n "${NAMESPACE}" -f - >/dev/null <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: ${job}
spec:
  restartPolicy: Never
  activeDeadlineSeconds: 60
  imagePullSecrets:
    - name: registry-credentials
  containers:
    - name: wget
      image: hub.zama.org/docker.io/library/busybox:1.36
      env:
        - name: RPC_URL
          valueFrom: {configMapKeyRef: {name: ${job}, key: url}}
        - name: RPC_BODY
          valueFrom: {configMapKeyRef: {name: ${job}, key: body}}
      command: ["sh", "-c"]
      args:
        - |
          set -e
          if [ -n "\${RPC_BODY}" ]; then
            wget -T 30 -qO- --header="Content-Type: application/json" --post-data="\${RPC_BODY}" "\${RPC_URL}"
          else
            wget -T 30 -qO- "\${RPC_URL}"
          fi
EOF

phase="Pending"
for _ in $(seq 1 60); do
  phase=$(kubectl get pod -n "${NAMESPACE}" "${job}" -o jsonpath='{.status.phase}' 2>/dev/null || echo Pending)
  case "${phase}" in
    Succeeded) break ;;
    Failed)
      kubectl logs -n "${NAMESPACE}" "${job}" >&2 || true
      kubectl describe pod -n "${NAMESPACE}" "${job}" >&2 || true
      echo "::error::cluster-rpc pod ${job} failed calling ${url}" >&2
      exit 1
      ;;
  esac
  sleep 2
done
if [[ "${phase}" != "Succeeded" ]]; then
  kubectl logs -n "${NAMESPACE}" "${job}" >&2 || true
  kubectl describe pod -n "${NAMESPACE}" "${job}" >&2 || true
  echo "::error::cluster-rpc pod ${job} timed out calling ${url} (phase=${phase})" >&2
  exit 1
fi

# kubectl logs may append a trailing newline; keep the last non-empty line so
# callers can jq the RPC object without hitting a blank tail -1.
json=$(kubectl logs -n "${NAMESPACE}" "${job}" | tr -d '\r' | awk 'NF { line=$0 } END { print line }')
if [[ -z "${json}" ]]; then
  echo "::error::cluster-rpc ${url} returned empty logs" >&2
  exit 1
fi
if ! jq -e 'type == "object"' >/dev/null <<<"${json}"; then
  echo "::error::cluster-rpc ${url} returned non-JSON: ${json}" >&2
  exit 1
fi
if jq -e '.error != null' >/dev/null <<<"${json}"; then
  echo "::error::cluster-rpc ${url} JSON-RPC error: ${json}" >&2
  exit 1
fi
if ! jq -e '.result != null' >/dev/null <<<"${json}"; then
  echo "::error::cluster-rpc ${url} missing result: ${json}" >&2
  exit 1
fi
printf '%s\n' "${json}"
