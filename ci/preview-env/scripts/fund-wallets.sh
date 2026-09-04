#!/usr/bin/env bash
# Fund every address this preview will sign with, on both blockchain-dev
# faucets, from an in-cluster Job (ClusterIP DNS). No-op on Anvil.
#
# Env: NAMESPACE, HOST_HTTP, GATEWAY_HTTP, HOST_FAUCET, GATEWAY_FAUCET,
#      WALLETS_JSON, COPROC_WALLETS_JSON, ROLES_JSON_PATH
set -euo pipefail

if [[ "${USE_BLOCKCHAIN_DEV:-false}" != "true" ]]; then
  echo "Anvil mode: accounts are prefunded, skipping faucet."
  exit 0
fi

: "${NAMESPACE:?}"
: "${HOST_HTTP:?}"
: "${GATEWAY_HTTP:?}"
: "${HOST_FAUCET:?}"
: "${GATEWAY_FAUCET:?}"
: "${ROLES_JSON_PATH:?}"

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
python_src="${script_dir}/fund-wallets.py"

addresses=$(python3 - <<'PY'
import json, os
addrs = []
roles = json.load(open(os.environ["ROLES_JSON_PATH"]))["roles"]
for idx in ("0", "1", "2", "3", "4", "9"):
    addrs.append(roles[idx]["address"])
for key in ("WALLETS_JSON", "COPROC_WALLETS_JSON"):
    raw = os.environ.get(key) or "[]"
    for w in json.loads(raw):
        addrs.append(w["address"])
# unique, preserve order
seen = set()
out = []
for a in addrs:
    if a.lower() not in seen:
        seen.add(a.lower())
        out.append(a)
print("\n".join(out))
PY
)

echo "Funding ${addresses} via in-cluster Job..."
kubectl create configmap preview-fund-script -n "${NAMESPACE}" \
  --from-file=fund-wallets.py="${python_src}" \
  --from-literal=addresses="${addresses}" \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl delete job preview-fund-wallets -n "${NAMESPACE}" --ignore-not-found
kubectl apply -n "${NAMESPACE}" -f - <<EOF
apiVersion: batch/v1
kind: Job
metadata:
  name: preview-fund-wallets
spec:
  backoffLimit: 1
  ttlSecondsAfterFinished: 600
  template:
    spec:
      restartPolicy: Never
      imagePullSecrets:
        - name: registry-credentials
      containers:
        - name: fund
          image: hub.zama.org/docker.io/library/python:3.12-alpine
          env:
            - name: HOST_HTTP
              value: "${HOST_HTTP}"
            - name: GATEWAY_HTTP
              value: "${GATEWAY_HTTP}"
            - name: HOST_FAUCET
              value: "${HOST_FAUCET}"
            - name: GATEWAY_FAUCET
              value: "${GATEWAY_FAUCET}"
            - name: HOST_FLOOR_WEI
              value: "500000000000000000"
            - name: GATEWAY_FLOOR_WEI
              value: "200000000000000000"
          volumeMounts:
            - name: script
              mountPath: /fund
          command: ["python", "/fund/fund-wallets.py"]
      volumes:
        - name: script
          configMap:
            name: preview-fund-script
EOF

kubectl wait -n "${NAMESPACE}" --for=condition=complete "job/preview-fund-wallets" --timeout=10m \
  || { kubectl logs -n "${NAMESPACE}" "job/preview-fund-wallets" || true; exit 1; }
kubectl logs -n "${NAMESPACE}" "job/preview-fund-wallets"
