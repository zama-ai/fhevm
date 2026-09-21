#!/usr/bin/env bash
# Blue/Green QA: make two operators disagree, for section 5 "Operators disagree".
#
#   bg-drift.sh arm      install the injector on one party and arm it
#   bg-drift.sh status   report whether it is armed and how many hashes it has corrupted
#   bg-drift.sh disarm   remove the trigger, function and bookkeeping table
#
# Each operator publishes its state_hash to its own S3 bucket and every detector compares them per
# block. Corrupting what one operator writes makes that comparison fail, which is the divergence the
# dry run exists to catch. Three properties are load-bearing: it targets {gcs}.state_hash, the value
# actually compared, not ciphertext_digest, which belongs to the drift detector; it corrupts every
# hash, because the detector anchors on the first block that agrees; and it keeps the value valid
# hex of the same length while skipping the empty-block hash, which the uploader withholds
# deliberately - break either and the operator publishes nothing, testing absence instead.
#
# Re-arm after every rollback: a rollback drops and recreates the GCS schema, taking the trigger
# with it, and a retry without re-arming silently tests nothing.
#
# Destructive. Only on a preview environment that will be destroyed afterwards.
#
# Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg/bg-drift.sh arm|status|disarm
# Env: NAMESPACE (required), PARTY (2: the operator to corrupt),
#      GCS_STACK_VERSION (default: read from the gcs-* schema Green created)
set -euo pipefail

verb="${1:?usage: bg-drift.sh arm|status|disarm}"
: "${NAMESPACE:?}"
PARTY="${PARTY:-2}"

fail() { echo "::error::$*" >&2; exit 1; }
psql_party() {
  kubectl exec -i -n "${NAMESPACE}" "postgres-coprocessor-${PARTY}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -v ON_ERROR_STOP=1 "$@"
}

# The Green schema names the version; reading it back cannot disagree with the running fleet.
GCS_STACK_VERSION="${GCS_STACK_VERSION:-$(psql_party -tAqc \
  "SELECT replace(nspname, 'gcs-', '') FROM pg_namespace WHERE nspname LIKE 'gcs-%' LIMIT 1;")}"
[[ -n "${GCS_STACK_VERSION}" ]] \
  || fail "no gcs-* schema on party ${PARTY}: Green is not running, so there is nothing to corrupt"
schema="gcs-${GCS_STACK_VERSION}"

case "${verb}" in
arm)
  psql_party -v schema="${schema}" <<'SQL'
CREATE TABLE IF NOT EXISTS public.consensus_drift_state (
  id BOOLEAN PRIMARY KEY DEFAULT TRUE,
  enabled BOOLEAN NOT NULL,
  corrupted BIGINT NOT NULL DEFAULT 0
);
INSERT INTO public.consensus_drift_state (id, enabled, corrupted) VALUES (TRUE, TRUE, 0)
ON CONFLICT (id) DO UPDATE SET enabled = EXCLUDED.enabled, corrupted = 0;
CREATE OR REPLACE FUNCTION public.inject_state_hash_drift()
RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE armed BOOLEAN;
BEGIN
  SELECT enabled INTO armed FROM public.consensus_drift_state WHERE id = TRUE;
  IF NOT COALESCE(armed, FALSE) THEN RETURN NEW; END IF;
  -- sha256(''): the uploader withholds this one, so corrupting it would make this operator
  -- publish blocks the others withhold - disagreement by presence, not by value.
  IF NEW.state_hash IS NOT NULL
     AND NEW.state_hash <> 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855' THEN
    NEW.state_hash := overlay(NEW.state_hash placing
      CASE WHEN substr(NEW.state_hash,1,1) = '0' THEN '1' ELSE '0' END from 1 for 1);
    UPDATE public.consensus_drift_state SET corrupted = corrupted + 1 WHERE id = TRUE;
  END IF;
  RETURN NEW;
END; $$;
SQL
  psql_party -v schema="${schema}" <<'SQL'
DROP TRIGGER IF EXISTS state_hash_drift_injector ON :"schema".state_hash;
CREATE TRIGGER state_hash_drift_injector
BEFORE INSERT ON :"schema".state_hash
FOR EACH ROW EXECUTE FUNCTION public.inject_state_hash_drift();
SQL
  echo "== bg-drift armed on party ${PARTY}, schema ${schema}"
  ;;
status)
  psql_party -tAqc "SELECT 'armed='||enabled||' corrupted='||corrupted FROM public.consensus_drift_state;"
  ;;
disarm)
  psql_party -v schema="${schema}" <<'SQL'
DROP TRIGGER IF EXISTS state_hash_drift_injector ON :"schema".state_hash;
DROP FUNCTION IF EXISTS public.inject_state_hash_drift();
DROP TABLE IF EXISTS public.consensus_drift_state;
SQL
  echo "== bg-drift removed from party ${PARTY}"
  ;;
*) fail "unknown verb ${verb}: use arm|status|disarm" ;;
esac
