#!/usr/bin/env bash
# Shared helpers. Source after your own `set -euo pipefail`.

# kms BFT-MPC thresholds for N parties: t=floor((N-1)/3), majority=t+1,
# reconstruct=N-t. TODO: reconfirm scaling against kms's formula at N > 4.
kms_t()           { echo "$(( ( $1 - 1 ) / 3 ))"; }
kms_majority()    { echo "$(( $(kms_t "$1") + 1 ))"; }
kms_reconstruct() { echo "$(( $1 - $(kms_t "$1") ))"; }

# Coprocessor consensus is a simple majority of NC parties.
coproc_threshold() { echo "$(( $1 / 2 + 1 ))"; }

# Fail with a ::error:: annotation when VALUE is empty or jq's "null" sentinel.
require_nonempty() {
  if [[ -z "${1:-}" || "${1:-}" == "null" ]]; then
    echo "::error::${2}" >&2
    exit 1
  fi
}

# Block until the External Secrets Operator has materialized the Secret behind NAME.
wait_external_secret() {
  local name="$1"
  kubectl wait -n "${NAMESPACE}" "externalsecret/${name}" --for=condition=Ready --timeout=120s \
    || { kubectl describe -n "${NAMESPACE}" "externalsecret/${name}" | tail -20; exit 1; }
}

# The per-party preview Postgres instances use the shared ephemeral credentials.
psql_party() {
  local party="$1" sql="$2" database="${3:-fhevm_e2e}"
  kubectl exec -n "${NAMESPACE}" "postgres-coprocessor-${party}-0" -- \
    env PGPASSWORD=zama psql -U zama -d "${database}" -tAqc "${sql}"
}

# A Solana reset closes every zama-host account, and a store created again at the same address
# restarts at leaf zero. So each party's Merkle record restarts from an empty database at a block
# after the reset: stop the indexer, recreate the database, and print a fresh confirmed slot for
# solanaHostListener.merkleIndexer.startSlot. Arguments: the coprocessor party numbers.
reset_solana_merkle_records() {
  local party indexer rpc_url
  for party in "$@"; do
    indexer="deployment/coprocessor-${party}-solana-merkle-indexer"
    if [[ -n $(kubectl get "${indexer}" -n "${NAMESPACE}" --ignore-not-found -o name) ]]; then
      kubectl scale "${indexer}" -n "${NAMESPACE}" --replicas=0 >/dev/null
      kubectl wait pod -n "${NAMESPACE}" -l "app.kubernetes.io/name=coprocessor-${party}-solana-merkle-indexer" \
        --for=delete --timeout=2m >/dev/null
    fi
    psql_party "${party}" 'DROP DATABASE IF EXISTS solana_merkle WITH (FORCE)' >/dev/null
    psql_party "${party}" 'CREATE DATABASE solana_merkle' >/dev/null
  done
  rpc_url=$(kubectl get secret solana-rpc -n "${NAMESPACE}" -o jsonpath='{.data.rpc-url}' | base64 -d)
  curl -fsS "${rpc_url}" -H 'content-type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"getSlot","params":[{"commitment":"confirmed"}]}' | jq -er .result
}
