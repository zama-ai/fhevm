# shellcheck shell=bash
# A killed delegated runner cannot clean its own SQL controls. Both the child
# and matrix parent use these idempotent operations, in this order:
# disable (commit), restore workers, then remove the audit triggers.

cc_sql() {
  local container="$1" database="$2" statement="$3"
  hc_run timeout --kill-after=2s 25s docker exec -e PGCONNECT_TIMEOUT=5 "$container" \
    psql -U postgres -d "$database" -v ON_ERROR_STOP=1 -c \
    "SET lock_timeout=5000; SET statement_timeout=10000; $statement"
}

cc_disable_failpoints() {
  cc_sql "$1" "$2" 'DO $$ BEGIN
    IF to_regclass('\''public.consensus_test_failpoints'\'') IS NOT NULL THEN
      DELETE FROM public.consensus_test_failpoints;
    END IF;
  END $$;'
}

cc_drop_audit() {
  cc_sql "$1" "$2" 'DO $$ BEGIN
    IF to_regclass('\''public.dependence_chain'\'') IS NOT NULL THEN
      DROP TRIGGER IF EXISTS consensus_test_claim_audit ON public.dependence_chain;
    END IF;
    IF to_regclass('\''public.consensus_test_failpoints'\'') IS NOT NULL THEN
      DROP TRIGGER IF EXISTS consensus_test_complete_boundary ON public.consensus_test_failpoints;
    END IF;
  END $$;
  DROP FUNCTION IF EXISTS public.consensus_test_claim_audit();
  DROP TABLE IF EXISTS public.consensus_test_claims;
  DROP FUNCTION IF EXISTS public.consensus_test_complete_boundary();'
}
