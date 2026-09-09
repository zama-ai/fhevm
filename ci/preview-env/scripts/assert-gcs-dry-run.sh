#!/usr/bin/env bash
# RFC-021: after in-window e2e, GCS must have either shadowed FHE work in
# "gcs-<ver>".computations, or already cut over (LIVE). Cutover drops that
# schema, so a LIVE row is the success path — not an empty-table failure.
# PAUSED/failed still means the window closed without a cutover.
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
  live_version=$(psql_party "${i}" \
    "SELECT COALESCE(stack_version, '') FROM versioning;" || true)

  echo "party ${i}: state=${state} status=${status} versioning=${live_version} last_error='${last_error}'"

  if [[ "${state}" == "PAUSED" || "${status}" == "failed" ]]; then
    echo "::error::party ${i} GCS rolled back before in-window work could be asserted (state=${state} status=${status} last_error='${last_error}')"
    failed=1
    continue
  fi
  case "${state}" in
    LIVE)
      # Schema is dropped at cutover. LIVE + completed is the proof GCS
      # shadowed and merged; do not query gcs-*.computations.
      if [[ "${status}" != "completed" ]]; then
        echo "::error::party ${i} GCS LIVE but status='${status}', expected completed"
        failed=1
      fi
      continue
      ;;
    DryRunStarted|UpgradeAuthorized) ;;
    *)
      echo "::error::party ${i} GCS state='${state}', expected DryRunStarted (or UpgradeAuthorized/LIVE if cutover already raced)"
      failed=1
      continue
      ;;
  esac

  count=$(psql_party "${i}" \
    "SELECT count(*) FROM \"${schema}\".computations;" || echo "0")
  echo "party ${i}: ${schema}.computations=${count}"
  if [[ "${count}" -lt 1 ]]; then
    echo "::error::party ${i} ${schema}.computations=${count}, expected > 0 during ${state}"
    failed=1
  fi
done

if [[ "${failed}" -ne 0 ]]; then
  exit 1
fi

echo "GCS in-window path ok on ${NB_COPROCESSOR} operator DB(s) (schema ${schema} or LIVE after cutover)."
