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
# restarts at leaf zero. So after a reset, party $1's Merkle record restarts from an empty
# database: this stops its indexer, if deployed, and recreates the database. Call it as a
# statement, not inside $(...), where bash ignores `set -e`.
recreate_solana_merkle_record() {
  local party="$1" indexer pods attempt
  indexer=$(kubectl get "deployment/coprocessor-${party}-solana-merkle-indexer" -n "${NAMESPACE}" \
    --ignore-not-found -o name)
  if [[ -n "${indexer}" ]]; then
    kubectl scale "${indexer}" -n "${NAMESPACE}" --replicas=0 >/dev/null
    for ((attempt=0; attempt<60; attempt++)); do
      pods=$(kubectl get pods -n "${NAMESPACE}" -o name \
        -l "app.kubernetes.io/name=coprocessor-${party}-solana-merkle-indexer")
      [[ -z "${pods}" ]] && break
      sleep 2
    done
    [[ -z "${pods}" ]] || { echo "::error::Solana Merkle indexer ${party} did not stop"; exit 1; }
  fi
  psql_party "${party}" 'DROP DATABASE IF EXISTS solana_merkle WITH (FORCE)' >/dev/null
  psql_party "${party}" 'CREATE DATABASE solana_merkle' >/dev/null
}

# The preview Solana RPC's finalized slot. Taken after a reset, it precedes every store the
# reset's redeploy creates, so it is the Merkle indexers' start slot.
finalized_solana_slot() {
  local rpc_url
  rpc_url=$(kubectl get secret solana-rpc -n "${NAMESPACE}" -o jsonpath='{.data.rpc-url}' | base64 -d)
  curl -fsS "${rpc_url}" -H 'content-type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"getSlot","params":[{"commitment":"finalized"}]}' | jq -er .result
}

# Waits until party $1's Merkle indexer has recorded past start slot $2, which proves its replay
# from that slot ran. The checkpoint table appears only once the indexer has migrated.
wait_solana_merkle_recorded() {
  local party="$1" start_slot="$2" current attempt
  for ((attempt=0; attempt<60; attempt++)); do
    current=$(psql_party "${party}" 'SELECT COALESCE(MAX(slot),0) FROM checkpoint' solana_merkle 2>/dev/null) \
      || current=0
    (( current > start_slot )) && return 0
    sleep 5
  done
  echo "::error::Solana Merkle indexer ${party} has not recorded past start slot ${start_slot}; check its logs"
  exit 1
}
