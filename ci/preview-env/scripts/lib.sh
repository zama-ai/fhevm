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

# Block until the External Secrets Operator has materialised the Secret behind NAME.
wait_external_secret() {
  local name="$1"
  kubectl wait -n "${NAMESPACE}" "externalsecret/${name}" --for=condition=Ready --timeout=120s \
    || { kubectl describe -n "${NAMESPACE}" "externalsecret/${name}" | tail -20; exit 1; }
}

# The per-party preview Postgres instances use the shared ephemeral credentials.
psql_party() {
  local party="$1" sql="$2"
  kubectl exec -n "${NAMESPACE}" "postgres-coprocessor-${party}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "${sql}"
}
