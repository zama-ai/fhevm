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

# One GCS row per host chain in the proposal (ETH, plus Polygon under deploy_polygon).
expected_chains=1
[[ "${DEPLOY_POLYGON:-false}" == "true" ]] && expected_chains=2

failed=0
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  live_version=$(psql_party "${i}" \
    "SELECT COALESCE(stack_version, '') FROM versioning;" || true)
  rows=$(psql_party "${i}" \
    "SELECT host_chain_id || '|' || COALESCE(state, '') || '|' || COALESCE(status, '') || '|' || COALESCE(last_error, '') FROM upgrade_state WHERE stack_role='GCS' ORDER BY host_chain_id;" || true)
  nrows=$(printf '%s\n' "${rows}" | grep -c . || true)
  if [[ "${nrows}" -ne "${expected_chains}" ]]; then
    echo "::error::party ${i} has ${nrows} GCS upgrade_state row(s), expected ${expected_chains} (one per host chain); versioning=${live_version}"
    failed=1
    continue
  fi

  while IFS='|' read -r chain_id state status last_error; do
    [[ -n "${chain_id}" ]] || continue
    echo "party ${i} chain ${chain_id}: state=${state} status=${status} versioning=${live_version} last_error='${last_error}'"

    if [[ "${state}" == "PAUSED" || "${status}" == "failed" ]]; then
      echo "::error::party ${i} chain ${chain_id} GCS rolled back before in-window work could be asserted (state=${state} status=${status} last_error='${last_error}')"
      failed=1
      continue
    fi
    case "${state}" in
      LIVE)
        # Schema is dropped at cutover. LIVE + completed is the proof GCS
        # shadowed and merged; do not query gcs-*.computations.
        if [[ "${status}" != "completed" ]]; then
          echo "::error::party ${i} chain ${chain_id} GCS LIVE but status='${status}', expected completed"
          failed=1
        fi
        continue
        ;;
      DryRunStarted|UpgradeAuthorized) ;;
      *)
        echo "::error::party ${i} chain ${chain_id} GCS state='${state}', expected DryRunStarted (or UpgradeAuthorized/LIVE if cutover already raced)"
        failed=1
        continue
        ;;
    esac

    count=$(psql_party "${i}" \
      "SELECT count(*) FROM \"${schema}\".computations WHERE host_chain_id = ${chain_id};" || echo "0")
    echo "party ${i} chain ${chain_id}: ${schema}.computations=${count}"
    if [[ "${count}" -lt 1 ]]; then
      echo "::error::party ${i} chain ${chain_id} ${schema}.computations=${count}, expected > 0 during ${state}"
      failed=1
    fi
  done <<<"${rows}"
done

if [[ "${failed}" -ne 0 ]]; then
  exit 1
fi

echo "GCS in-window path ok on ${NB_COPROCESSOR} operator DB(s) (schema ${schema} or LIVE after cutover)."
