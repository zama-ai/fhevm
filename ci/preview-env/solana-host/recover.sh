#!/usr/bin/env bash
# Recover before reset/destroy. A failed recovery leaves namespace secrets available for retry.
set +x
set -euo pipefail
umask 077
: "${NAMESPACE:?}"
[[ "$NAMESPACE" == fhevm-ci-* ]] || { echo 'Not a preview namespace' >&2; exit 1; }
mode=${1:-reset}
[[ "$mode" == reset || "$mode" == recover ]] || exit 2
deployer_secret=$(kubectl get secret solana-deployer -n "$NAMESPACE" --ignore-not-found -o name)
if [[ -z "$deployer_secret" ]]; then
  echo 'No Solana deployer in this namespace; no Solana recovery required.'
  exit 0
fi
# shellcheck source=ci/preview-env/solana-host/ownership.sh
source "$(dirname "${BASH_SOURCE[0]}")/ownership.sh"
if [[ "${SOLANA_OPERATION_HELD:-}" != 1 ]]; then
  solana_acquire
  release_on_exit() {
    if [[ "$1" != 0 || "${SOLANA_KEEP_OPERATION:-}" != 1 ]]; then solana_release_operation; fi
  }
  trap 'release_on_exit "$?"' EXIT
fi
kubectl get secret solana-recovery -n "$NAMESPACE" -o name >/dev/null

image=${SOLANA_RECOVERY_IMAGE:-}
if [[ -z "$image" ]]; then
  # An in-place recovery fix can be newer than the program deployment Jobs.
  image=$(kubectl get jobs -n "$NAMESPACE" -o json | python3 -c '
import json,sys
candidates=[]
for job in json.load(sys.stdin)["items"]:
    metadata=job["metadata"]
    for container in job["spec"]["template"]["spec"]["containers"]:
        if "/solana-programs:" in container["image"]:
            candidates.append((metadata["name"].startswith("solana-recovery-"), metadata["creationTimestamp"], container["image"]))
print(max(candidates)[2] if candidates else "")')
fi
[[ -n "$image" ]] || { echo 'SOLANA_RECOVERY_IMAGE required; refusing teardown' >&2; exit 1; }
# Receipts are public, durable across failed Jobs, and retained until namespace destruction.
if [[ -z $(kubectl get configmap solana-recovery-journal -n "$NAMESPACE" --ignore-not-found -o name) ]]; then
  kubectl create configmap solana-recovery-journal -n "$NAMESPACE" >/dev/null
fi
kubectl apply -n "$NAMESPACE" -f - >/dev/null <<'RBAC'
apiVersion: v1
kind: ServiceAccount
metadata:
  name: solana-recovery
imagePullSecrets:
  - name: registry-credentials
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: solana-recovery-journal
rules:
  - apiGroups: [""]
    resources: ["configmaps"]
    resourceNames: ["solana-recovery-journal"]
    verbs: ["get", "patch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: solana-recovery-journal
subjects:
  - kind: ServiceAccount
    name: solana-recovery
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: Role
  name: solana-recovery-journal
RBAC
export SOLANA_RECOVERY_IMAGE="$image" SOLANA_RECOVERY_MODE="$mode"
job="solana-recovery-$(date +%s)"
export SOLANA_RECOVERY_JOB="$job"
python3 - <<'PY' | kubectl apply -f - >/dev/null
import os,json
secret=lambda name,key:{'valueFrom':{'secretKeyRef':{'name':name,'key':key}}}
env=[{'name':'SOLANA_PREVIEW_NAMESPACE','value':os.environ['NAMESPACE']},{'name':'SOLANA_RPC_URL',**secret('solana-rpc','rpc-url')},{'name':'SOLANA_DEPLOYER_KEYPAIR_JSON',**secret('solana-deployer','deployer.json')},{'name':'SOLANA_RECOVERY_DIR','value':'/recovery'},{'name':'SOLANA_ENVIRONMENT','value':'preview-env'}]
mode=os.environ['SOLANA_RECOVERY_MODE']
command='set -eu; umask 077; cp /recovery-source/*.json /recovery/; chmod 600 /recovery/*.json; '
if mode=='reset': command+='node /app/cli.mjs environment recover-funding; node /app/cli.mjs environment prepare-reset; '
command+=f'node /app/cli.mjs environment {mode}'
pod={'serviceAccountName':'solana-recovery','restartPolicy':'Never','nodeSelector':{'kubernetes.io/arch':'amd64'},'securityContext':{'runAsUser':10000,'runAsGroup':10001,'fsGroup':10001},'containers':[{'name':'recover','image':os.environ['SOLANA_RECOVERY_IMAGE'],'command':['bash','-c',command],'env':env,'volumeMounts':[{'name':'keys','mountPath':'/recovery-source','readOnly':True},{'name':'work','mountPath':'/recovery'}]}],'volumes':[{'name':'keys','secret':{'secretName':'solana-recovery','defaultMode':0o440}},{'name':'work','emptyDir':{}}]}
print(json.dumps({'apiVersion':'batch/v1','kind':'Job','metadata':{'name':os.environ['SOLANA_RECOVERY_JOB'],'namespace':os.environ['NAMESPACE']},'spec':{'backoffLimit':0,'activeDeadlineSeconds':2400,'template':{'spec':pod}}}))
PY
if ! kubectl wait -n "$NAMESPACE" --for=condition=complete "job/$job" --timeout=40m; then
  kubectl logs -n "$NAMESPACE" "job/$job"
  echo "Recovery failed or timed out: $job. Namespace and signing keys retained; retry after diagnosis." >&2
  exit 1
fi
# The CLI emits public addresses and lamport accounting only, never command diagnostics or keys.
kubectl logs -n "$NAMESPACE" "job/$job"
