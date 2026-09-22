#!/usr/bin/env bash
# Source this before changing the fixed shared devnet programs. Keep ownership across redeploys.
solana_owner_namespace=fhevm-ci-solana-owner
solana_acquire() {
  local uid owner
  uid=$(kubectl get namespace "$NAMESPACE" -o jsonpath='{.metadata.uid}')
  [[ -n "$uid" ]] || return 1
  if ! kubectl get namespace "$solana_owner_namespace" >/dev/null 2>&1; then
    # Refuse to adopt shared state while an older workflow still has it mounted.
    if kubectl get secrets -A --field-selector metadata.name=solana-deployer -o json | NAMESPACE="$NAMESPACE" python3 -c 'import json,os,sys; sys.exit(any(x["metadata"]["namespace"] != os.environ["NAMESPACE"] for x in json.load(sys.stdin)["items"]))'; then
      kubectl create namespace "$solana_owner_namespace" --dry-run=client -o json | OWNER_UID="$uid" OWNER_NAMESPACE="$NAMESPACE" python3 -c 'import json,os,sys; x=json.load(sys.stdin); x["metadata"]["annotations"]={"solana-preview-owner":os.environ["OWNER_NAMESPACE"],"solana-preview-owner-uid":os.environ["OWNER_UID"]}; print(json.dumps(x))' | kubectl create -f - >/dev/null || return 1
    else
      echo 'Another preview has Solana credentials; recover and relinquish it first.' >&2; return 1
    fi
  fi
  owner=$(kubectl get namespace "$solana_owner_namespace" -o jsonpath='{.metadata.annotations.solana-preview-owner-uid}')
  [[ "$owner" == "$uid" ]] || { echo 'Shared Solana programs belong to another preview; refusing changes.' >&2; return 1; }
  kubectl create configmap solana-operation -n "$solana_owner_namespace" --from-literal="namespace=$NAMESPACE" >/dev/null || { echo 'Cannot acquire the Solana operation lock; inspect the Kubernetes error and existing lock before retrying.' >&2; return 1; }
  export SOLANA_OPERATION_HELD=1
}
solana_release_operation() {
  # Cancellation of kubectl wait does not stop a remote deployment or recovery Job.
  # Retain the operation lock until an operator verifies that every such Job has ended.
  if ! kubectl get jobs -n "$NAMESPACE" -o json | python3 -c 'import json,sys; jobs=json.load(sys.stdin)["items"]; live=[j for j in jobs if any("/solana-programs:" in c["image"] for c in j["spec"]["template"]["spec"]["containers"]) and not any(c["type"] in ("Complete","Failed") and c["status"] == "True" for c in j.get("status",{}).get("conditions",[]))]; sys.exit(bool(live))'; then
    echo 'Solana operation lock retained: a Job may still be running. Verify termination before releasing it.' >&2
    return 0
  fi
  kubectl delete configmap solana-operation -n "$solana_owner_namespace" --ignore-not-found >/dev/null
}
