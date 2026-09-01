#!/usr/bin/env bash
# RFC-021: after in-window e2e, GCS must still be in a live dry-run (or already
# past it toward cutover) and must have shadowed at least one FHE computation.
# A PAUSED/failed row or an empty "gcs-<ver>".computations table means the
# window closed without GCS work (or rollback already wiped the schema).
set -euo pipefail

: "${NAMESPACE:?}"
: "${NB_COPROCESSOR:?}"

GCS_VERSION="${GCS_VERSION:-v0.15.0}"
schema="gcs-${GCS_VERSION#v}"

psql_party() {
  local party="$1" sql="$2"
  kubectl exec -n "${NAMESPACE}" "postgres-coprocessor-${party}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "${sql}"
}

failed=0
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  state=$(psql_party "${i}" \
    "SELECT COALESCE(state, '') FROM upgrade_state WHERE stack_role='GCS';" || true)
  status=$(psql_party "${i}" \
    "SELECT COALESCE(status, '') FROM upgrade_state WHERE stack_role='GCS';" || true)
  last_error=$(psql_party "${i}" \
    "SELECT COALESCE(last_error, '') FROM upgrade_state WHERE stack_role='GCS';" || true)
  count=$(psql_party "${i}" \
    "SELECT count(*) FROM \"${schema}\".computations;" || echo "0")

  echo "party ${i}: state=${state} status=${status} ${schema}.computations=${count} last_error='${last_error}'"

  if [[ "${state}" == "PAUSED" || "${status}" == "failed" ]]; then
    echo "::error::party ${i} GCS rolled back before in-window work could be asserted (state=${state} status=${status} last_error='${last_error}')"
    failed=1
    continue
  fi
  case "${state}" in
    DryRunStarted|UpgradeAuthorized|LIVE) ;;
    *)
      echo "::error::party ${i} GCS state='${state}', expected DryRunStarted (or UpgradeAuthorized/LIVE if cutover already raced)"
      failed=1
      continue
      ;;
  esac
  if [[ "${count}" -lt 1 ]]; then
    echo "::error::party ${i} ${schema}.computations=${count}, expected > 0 during DryRunStarted"
    failed=1
  fi
done

if [[ "${failed}" -ne 0 ]]; then
  exit 1
fi

echo "GCS shadowed in-window FHE work on ${NB_COPROCESSOR} operator DB(s) (schema ${schema})."
