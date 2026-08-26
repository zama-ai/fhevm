#!/usr/bin/env bash
# JSON-RPC from inside the preview namespace so ClusterIP DNS works.
# Usage: cluster-rpc.sh <url> [json-body]
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"
url="${1:?url required}"
body="${2:-}"
job="preview-rpc-$(date +%s)"

kubectl create configmap "${job}" -n "${NAMESPACE}" \
  --from-literal=url="${url}" \
  --from-literal=body="${body}" \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl delete pod -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null 2>&1 || true
kubectl apply -n "${NAMESPACE}" -f - <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: ${job}
spec:
  restartPolicy: Never
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
            wget -qO- --header="Content-Type: application/json" --post-data="\${RPC_BODY}" "\${RPC_URL}"
          else
            wget -qO- "\${RPC_URL}"
          fi
          echo
EOF

for _ in $(seq 1 60); do
  phase=$(kubectl get pod -n "${NAMESPACE}" "${job}" -o jsonpath='{.status.phase}' 2>/dev/null || echo Pending)
  case "${phase}" in
    Succeeded) break ;;
    Failed)
      kubectl logs -n "${NAMESPACE}" "${job}" || true
      kubectl delete pod,configmap -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null
      echo "::error::cluster-rpc pod ${job} failed"
      exit 1
      ;;
  esac
  sleep 2
done
if [[ "${phase}" != "Succeeded" ]]; then
  kubectl logs -n "${NAMESPACE}" "${job}" || true
  kubectl delete pod,configmap -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null
  echo "::error::cluster-rpc pod ${job} timed out (phase=${phase})"
  exit 1
fi
kubectl logs -n "${NAMESPACE}" "${job}"
kubectl delete pod,configmap -n "${NAMESPACE}" "${job}" --ignore-not-found >/dev/null
