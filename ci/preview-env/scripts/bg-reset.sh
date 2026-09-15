#!/usr/bin/env bash
# Blue/Green QA: reset the coprocessor layer of a preview env back to the Blue
# (v0.14) state after a cutover, so the same env can run another upgrade round.
#
# Environment reset only, not a protocol rollback: contracts, KMS, wallets, keys
# and CRS are untouched, no keygen re-runs. Per party it uninstalls the Green
# releases, stops Blue, clears the work tables in ONE transaction, puts
# versioning back to the Blue version, restarts Blue and checks it came up live.
# The Redis broker is left alone: consumers delete acknowledged entries, so the
# streams only hold unconsumed blocks and Blue's groups resume at the tip.
# Idempotent: if it aborts half-way, fix the cause and run it again.
#
# Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg-reset.sh
# Env: NAMESPACE (required), NB_COPROCESSOR (2), DEPLOY_POLYGON (true),
#      BCS_STACK_VERSION (default: what the running Blue binary prints for
#      --stack-version, which is also what the Blue migration job bootstrapped),
#      QUIET_SECS (120: refuse if any party received work in the last N seconds),
#      FORCE (false: skip the traffic checks), DRY_RUN (false: checks only).
set -euo pipefail

: "${NAMESPACE:?}"
NB_COPROCESSOR="${NB_COPROCESSOR:-2}"
if [[ -z "${DEPLOY_POLYGON:-}" ]]; then
  DEPLOY_POLYGON=false
  helm status coprocessor-polygon-1 -n "${NAMESPACE}" >/dev/null 2>&1 && DEPLOY_POLYGON=true
fi
# Read the version string from the Blue binary (a 0.14.1 image still prints 0.14.0),
# not from a pin: the retired Blue pods are still there at this point.
BCS_STACK_VERSION="${BCS_STACK_VERSION:-$(kubectl exec -n "${NAMESPACE}" deploy/coprocessor-1-host-listener-consumer -- host_listener --stack-version)}"
QUIET_SECS="${QUIET_SECS:-120}"
FORCE="${FORCE:-false}"
DRY_RUN="${DRY_RUN:-false}"

# Tables cleared per party: every table the upgrade-controller duplicates into
# the Green schema plus the reorg/bridge/delegation/settlement side tables.
# Source of truth: coprocessor/fhevm-engine/upgrade-controller/src/coprocessor_tables.rs.
# Kept: _sqlx_migrations, keys, crs, tenants, host_chains, kms_*_activation_events,
# host_chain_blocks_valid, host_chain_consumer_blocks, host_listener_poller_state,
# gw_listener_last_block. No foreign keys link the two groups.
WORK_TABLES="ciphertexts, ciphertexts128, ciphertext_digest, computations, pbs_computations,
  verify_proofs, dependence_chain, state_hash, input_handles, transactions, allowed_handles,
  fallback_granted_events, handle_bridged_events, computations_branch, pbs_computations_branch,
  allowed_handles_branch, ciphertext_digest_branch, ciphertexts_branch, ciphertexts128_branch,
  coprocessor_settlement, bridge_handle_events, delegate_user_decrypt, drift_revert_signal,
  input_blobs"

trap 'echo "::error::bg-reset aborted at line ${LINENO}. Blue may be scaled to zero: fix the cause and re-run (idempotent)." >&2' ERR

fail() { echo "::error::$*" >&2; exit 1; }

psql_party() {
  local party="$1" sql="$2"
  kubectl exec -n "${NAMESPACE}" "postgres-coprocessor-${party}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -v ON_ERROR_STOP=1 -tAqc "${sql}"
}

# Blue deployment names of one party: main release + Polygon consumer + both pollers, never
# Green. Listed once, up front, into ${work}/blue-<party>: a transient kubectl failure here
# must abort, not silently skip a party half-way through.
work=$(mktemp -d)
trap 'rm -rf "${work}"' EXIT
list_blue_deployments() {
  local party="$1" all
  all=$(kubectl get deploy -n "${NAMESPACE}" -o name) || fail "kubectl get deploy failed"
  sed 's#^deployment.apps/##' <<<"${all}" \
    | grep -E "^coprocessor-(${party}|polygon-${party}|poller-${party}|poller-polygon-${party})-" \
    | grep -v -- "-gcs-" > "${work}/blue-${party}" || true
  [[ -s "${work}/blue-${party}" ]] || fail "party ${party}: no Blue deployments found in ${NAMESPACE}"
}
blue_deployments() { cat "${work}/blue-$1"; }

# Logs of the (single, fresh) pod behind a deployment, matched by pod-name prefix:
# `kubectl logs deploy/` picks by selector and the Polygon/ETH consumers share labels.
deploy_logs() {
  local name="$1" pod
  pod=$(kubectl get pods -n "${NAMESPACE}" --no-headers -o custom-columns=N:.metadata.name \
    | grep -E "^${name}-[a-z0-9-]+$" | head -1 || true)
  [[ -n "${pod}" ]] && kubectl logs -n "${NAMESPACE}" "${pod}" 2>/dev/null || true
}

echo "== bg-reset: ${NAMESPACE}, ${NB_COPROCESSOR} parties, polygon=${DEPLOY_POLYGON}, target versioning ${BCS_STACK_VERSION}/1"
for i in $(seq 1 "${NB_COPROCESSOR}"); do list_blue_deployments "${i}"; done

# ---- 0. preconditions: no traffic -------------------------------------------
running_wf=$(kubectl get workflows -n "${NAMESPACE}" --no-headers -o custom-columns=N:.metadata.name,P:.status.phase 2>/dev/null \
  | awk '$2=="Running"{print $1}' || true)
if [[ -n "${running_wf}" && "${FORCE}" != "true" ]]; then
  fail "e2e workflow(s) still running: $(tr '\n' ' ' <<<"${running_wf}"). Stop traffic first (FORCE=true to override)."
fi
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  recent=$(psql_party "${i}" "SELECT (SELECT count(*) FROM computations WHERE created_at > now() - interval '${QUIET_SECS} seconds') + (SELECT count(*) FROM verify_proofs WHERE created_at > now() - interval '${QUIET_SECS} seconds');")
  if [[ "${recent}" != "0" && "${FORCE}" != "true" ]]; then
    fail "party ${i} received ${recent} op(s)/input(s) in the last ${QUIET_SECS}s. Stop traffic first (FORCE=true to override)."
  fi
  echo "party ${i}: versioning=$(psql_party "${i}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1') FROM versioning v;")" \
       "upgrade_state=$(psql_party "${i}" "SELECT count(*)||' row(s), proposal id(s): '||coalesce(string_agg(DISTINCT proposal_id::text, ','), '-') FROM upgrade_state;")"
done
echo "The next round needs a proposal id above every id listed (the on-chain proposals stay completed)."

if [[ "${DRY_RUN}" == "true" ]]; then
  echo "DRY_RUN: would uninstall the Green releases, stop these deployments, clear the work tables and restart:"
  for i in $(seq 1 "${NB_COPROCESSOR}"); do blue_deployments "${i}"; done
  exit 0
fi

# ---- 1. Green releases off ----------------------------------------------------
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  for rel in "coprocessor-${i}-gcs" "coprocessor-polygon-${i}-gcs" "coprocessor-poller-${i}-gcs" "coprocessor-poller-polygon-${i}-gcs"; do
    if helm status "${rel}" -n "${NAMESPACE}" >/dev/null 2>&1; then
      helm uninstall "${rel}" -n "${NAMESPACE}" --wait --timeout 5m
    else
      echo "release ${rel} not installed, skipping"
    fi
  done
  # Helm hook Jobs (pre-install migration) outlive the release.
  kubectl delete job -n "${NAMESPACE}" -l "app.kubernetes.io/name=coprocessor-${i}-gcs-db-migration" --ignore-not-found
done

# ---- 2. Blue and pollers off, then no DB sessions -----------------------------
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  for d in $(blue_deployments "${i}"); do
    kubectl scale "deploy/${d}" -n "${NAMESPACE}" --replicas=0
  done
done
left=""
for _ in $(seq 1 60); do
  left=$(kubectl get pods -n "${NAMESPACE}" --no-headers 2>/dev/null \
    | grep -E "^coprocessor-(poller-polygon-|poller-|polygon-)?[0-9]+-" | grep -v "db-migration" | wc -l | tr -d ' ' || true)
  [[ "${left}" == "0" ]] && break
  sleep 5
done
[[ "${left}" == "0" ]] || fail "coprocessor pods still running after 5 min"
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  sessions=$(psql_party "${i}" "SELECT count(*) FROM pg_stat_activity WHERE datname='fhevm_e2e' AND pid<>pg_backend_pid() AND application_name<>'psql';")
  [[ "${sessions}" == "0" ]] || fail "party ${i} still has ${sessions} DB session(s)"
done

# ---- 3. database reset, one transaction per party -----------------------------
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  echo "party ${i}: clearing work tables, deleting upgrade_state, dropping any Green schema, versioning -> ${BCS_STACK_VERSION}/1"
  kubectl exec -i -n "${NAMESPACE}" "postgres-coprocessor-${i}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -v ON_ERROR_STOP=1 -q <<SQL
BEGIN;
TRUNCATE TABLE ${WORK_TABLES};
DELETE FROM upgrade_state;
-- A dry run that ended without cutover (timeout, or Green uninstalled) leaves
-- "gcs-<ver>" behind; the next controller would reuse its stale tables.
DO \$\$
DECLARE s text;
BEGIN
  FOR s IN SELECT nspname FROM pg_namespace WHERE nspname LIKE 'gcs%' LOOP
    EXECUTE format('DROP SCHEMA %I CASCADE', s);
  END LOOP;
END
\$\$;
UPDATE versioning SET stack_version = '${BCS_STACK_VERSION}', consensus_version = 1, updated_at = now()
 WHERE singleton = TRUE;
COMMIT;
SQL
  got=$(psql_party "${i}" "SELECT stack_version||'/'||consensus_version||' upgrade_state='||(SELECT count(*) FROM upgrade_state)||' computations='||(SELECT count(*) FROM computations)||' gcs_schemas='||(SELECT count(*) FROM pg_namespace WHERE nspname LIKE 'gcs%')||' keys='||(SELECT count(*) FROM keys)||' crs='||(SELECT count(*) FROM crs)||' host_chains='||(SELECT count(*) FROM host_chains) FROM versioning;")
  echo "party ${i}: ${got}"
  [[ "${got}" == "${BCS_STACK_VERSION}/1 upgrade_state=0 computations=0 gcs_schemas=0 keys="* ]] || fail "party ${i} post-reset state unexpected"
  [[ "${got}" == *" keys=0 "* || "${got}" == *" crs=0 "* ]] && fail "party ${i} lost keys/CRS - this should be impossible, stop and inspect"
done

# ---- 4. Blue and pollers back -------------------------------------------------
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  for d in $(blue_deployments "${i}"); do
    kubectl scale "deploy/${d}" -n "${NAMESPACE}" --replicas=1
  done
done
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  for d in $(blue_deployments "${i}"); do
    kubectl rollout status "deploy/${d}" -n "${NAMESPACE}" --timeout=300s >/dev/null
  done
done

# ---- 5. verify: Blue live, pollers parked, consumers ingesting ----------------
failed=0
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  blue_tag=$(kubectl get deploy -n "${NAMESPACE}" "coprocessor-${i}-host-listener-consumer" \
    -o jsonpath='{.spec.template.spec.containers[0].image}' | sed 's/.*://')
  for d in $(blue_deployments "${i}"); do
    mode=""
    for _ in $(seq 1 12); do
      mode=$(deploy_logs "${d}" | grep -m1 -oE '"gcs_mode":(true|false)' | cut -d: -f2 || true)
      [[ -n "${mode}" ]] && break
      sleep 5
    done
    # tx-sender does not log the resolution; every other Blue service must be live (false).
    # Pollers depend on the mode: the automated flow runs them on the HEAD image, where they
    # resolve true and wait for the Green schema; manual blue-green runs them on the Blue image,
    # where they are live like the rest. Tell the two apart by the image, not by assumption.
    if [[ "${d}" == *poller* ]]; then
      poller_tag=$(kubectl get deploy -n "${NAMESPACE}" "${d}" \
        -o jsonpath='{.spec.template.spec.containers[0].image}' | sed 's/.*://')
      if [[ "${poller_tag}" == "${blue_tag}" ]]; then
        [[ "${mode}" == "false" ]] || { echo "::error::${d}: Blue-image poller, expected gcs_mode=false, got '${mode}'"; failed=1; }
      else
        [[ "${mode}" == "true" || -z "${mode}" ]] || { echo "::error::${d}: HEAD-image poller, expected gcs_mode=true, got ${mode}"; failed=1; }
      fi
    elif [[ "${d}" != *tx-sender* ]]; then
      [[ "${mode}" == "false" ]] || { echo "::error::${d}: expected gcs_mode=false, got '${mode}'"; failed=1; }
    fi
    if deploy_logs "${d}" | grep "retired stack" >/dev/null; then
      echo "::error::${d}: still sees itself as retired"; failed=1
    fi
    echo "${d}: gcs_mode=${mode:-n/a}"
  done
done
expected_chains=1; [[ "${DEPLOY_POLYGON}" == "true" ]] && expected_chains=2
for i in $(seq 1 "${NB_COPROCESSOR}"); do
  chains=0
  for _ in $(seq 1 24); do
    chains=$(psql_party "${i}" "SELECT count(DISTINCT chain_id) FROM host_chain_blocks_valid WHERE created_at > now() - interval '60 seconds';")
    [[ "${chains}" -ge "${expected_chains}" ]] && break
    sleep 5
  done
  if [[ "${chains}" -ge "${expected_chains}" ]]; then
    echo "party ${i}: consumers ingesting on ${chains} chain(s)"
  else
    echo "::error::party ${i}: only ${chains}/${expected_chains} chain(s) ingested a block in the last 60s"; failed=1
  fi
done
[[ "${failed}" == "0" ]] || fail "reset finished but verification failed, see errors above"
echo "== bg-reset done: Blue ${BCS_STACK_VERSION} live on ${NB_COPROCESSOR} parties. Next: traffic setup, then bg-green.sh for the next round."
