#!/usr/bin/env bash
set -euo pipefail

# Wave-1 post-upgrade verifier.
#
# Required:
#   DATABASE_URL=postgresql://...
#
# Optional:
#   KUBECTL_NAMESPACE=<namespace>              # enables Kubernetes Job checks
#   KUBECTL_SELECTOR=app=coprocessor-db-migration
#   EXPECT_BRANCH_ACTIVATION_BLOCK=<height>    # checks host-listener env values when kubectl is enabled
#   DEADLOCKS_BASELINE=<count>                 # warns if pg_stat_database.deadlocks increased
#   DUAL_WRITE_RECENCY_MINUTES=<minutes>       # default: 60; warning-level branch-row recency check

KUBECTL_SELECTOR="${KUBECTL_SELECTOR:-app=coprocessor-db-migration}"
DUAL_WRITE_RECENCY_MINUTES="${DUAL_WRITE_RECENCY_MINUTES:-60}"

failures=0
warnings=0

pass() {
  printf 'PASS  %s\n' "$1"
}

fail() {
  printf 'FAIL  %s\n' "$1"
  failures=$((failures + 1))
}

warn() {
  printf 'WARN  %s\n' "$1"
  warnings=$((warnings + 1))
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    fail "missing required command: $1"
    printf '\nRESULT failed: %d failure(s), %d warning(s)\n' "$failures" "$warnings"
    exit 1
  fi
}

psql_scalar() {
  psql "$DATABASE_URL" -X -v ON_ERROR_STOP=1 -Atc "$1"
}

check_true() {
  local name="$1"
  local sql="$2"
  local result

  if ! result="$(psql_scalar "$sql" 2>&1)"; then
    fail "$name ($result)"
    return
  fi

  if [[ "$result" == "t" ]]; then
    pass "$name"
  else
    fail "$name (got: ${result:-<empty>})"
  fi
}

check_zero() {
  local name="$1"
  local sql="$2"
  local result

  if ! result="$(psql_scalar "$sql" 2>&1)"; then
    fail "$name ($result)"
    return
  fi

  if [[ "$result" == "0" ]]; then
    pass "$name"
  else
    fail "$name (got: $result)"
  fi
}

if [[ -z "${DATABASE_URL:-}" ]]; then
  fail "DATABASE_URL is required"
  printf '\nRESULT failed: %d failure(s), %d warning(s)\n' "$failures" "$warnings"
  exit 1
fi

if ! [[ "$DUAL_WRITE_RECENCY_MINUTES" =~ ^[0-9]+$ ]]; then
  warn "DUAL_WRITE_RECENCY_MINUTES must be an integer; defaulting to 60"
  DUAL_WRITE_RECENCY_MINUTES=60
fi

require_cmd psql

printf 'Wave-1 verifier\n'
printf 'Database: <redacted>\n\n'

check_true "_sqlx_migrations table exists" "
  SELECT to_regclass('public._sqlx_migrations') IS NOT NULL;
"

check_zero "required wave-1/fix migrations are applied successfully" "
  WITH required(version) AS (
    VALUES
      (20260610130000::BIGINT),
      (20260610130100::BIGINT),
      (20260610130200::BIGINT),
      (20260610130300::BIGINT),
      (20260610140000::BIGINT),
      (20260610145000::BIGINT),
      (20260610145100::BIGINT),
      (20260610150000::BIGINT),
      (20260704100000::BIGINT),
      (20260704110000::BIGINT),
      (20260704120000::BIGINT),
      (20260704130000::BIGINT)
  )
  SELECT count(*)
    FROM required r
    LEFT JOIN _sqlx_migrations m
      ON m.version = r.version
     AND m.success = true
   WHERE m.version IS NULL;
"

check_zero "no failed sqlx migrations are recorded" "
  SELECT count(*)
    FROM _sqlx_migrations
   WHERE success = false;
"

check_true "full migration reached at least 20260704130000" "
  SELECT COALESCE(max(version), 0) >= 20260704130000
    FROM _sqlx_migrations
   WHERE success = true;
"

check_true "host_chain_blocks_valid.parent_hash exists" "
  SELECT EXISTS (
    SELECT 1
      FROM information_schema.columns
     WHERE table_schema = 'public'
       AND table_name = 'host_chain_blocks_valid'
       AND column_name = 'parent_hash'
  );
"

check_true "idx_host_chain_blocks_valid_parent_hash exists and is valid" "
  SELECT EXISTS (
    SELECT 1
      FROM pg_class c
      JOIN pg_index i ON i.indexrelid = c.oid
      JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname = 'public'
       AND c.relname = 'idx_host_chain_blocks_valid_parent_hash'
       AND i.indisvalid
       AND i.indisready
  );
"

check_true "branch tables exist" "
  SELECT count(*) = 6
    FROM information_schema.tables
   WHERE table_schema = 'public'
     AND table_name IN (
       'computations_branch',
       'pbs_computations_branch',
       'allowed_handles_branch',
       'ciphertext_digest_branch',
       'ciphertexts_branch',
       'ciphertexts128_branch'
     );
"

check_true "branch context columns exist" "
  WITH required(table_name, column_name) AS (
    VALUES
      ('computations_branch', 'producer_block_hash'),
      ('pbs_computations_branch', 'producer_block_hash'),
      ('pbs_computations_branch', 'block_hash'),
      ('allowed_handles_branch', 'producer_block_hash'),
      ('allowed_handles_branch', 'block_hash'),
      ('ciphertext_digest_branch', 'producer_block_hash'),
      ('ciphertext_digest_branch', 'block_number'),
      ('ciphertext_digest_branch', 'block_hash'),
      ('ciphertexts_branch', 'producer_block_hash'),
      ('ciphertexts_branch', 'block_number'),
      ('ciphertexts128_branch', 'producer_block_hash'),
      ('ciphertexts128_branch', 'block_number')
  )
  SELECT count(*) = 0
    FROM required r
    LEFT JOIN information_schema.columns c
      ON c.table_schema = 'public'
     AND c.table_name = r.table_name
     AND c.column_name = r.column_name
   WHERE c.column_name IS NULL;
"

check_true "wave-1 mirror triggers are installed and enabled" "
  WITH required(table_name, trigger_name) AS (
    VALUES
      ('allowed_handles', 'mirror_allowed_handles_branchless_trigger'),
      ('ciphertext_digest', 'mirror_ciphertext_digest_branchless_ins'),
      ('ciphertext_digest', 'mirror_ciphertext_digest_branchless_del'),
      ('ciphertext_digest', 'mirror_ciphertext_digest_branchless_upd'),
      ('pbs_computations_branch', 'mirror_ciphertext_digest_pbs_context_trigger')
  )
  SELECT count(*) = 0
    FROM required r
    LEFT JOIN pg_trigger t
      ON t.tgname = r.trigger_name
     AND t.tgrelid = format('public.%I', r.table_name)::regclass
     AND NOT t.tgisinternal
     AND t.tgenabled <> 'D'
   WHERE t.oid IS NULL;
"

check_true "digest mirror functions include striped advisory locks from #2999" "
  SELECT pg_get_functiondef('mirror_ciphertext_digest_branchless()'::regprocedure)
           LIKE '%digest-mirror:%'
     AND pg_get_functiondef('mirror_ciphertext_digest_for_pbs_context()'::regprocedure)
           LIKE '%digest-mirror:%';
"

check_true "coprocessor_settlement exists" "
  SELECT to_regclass('public.coprocessor_settlement') IS NOT NULL;
"

check_true "host_chains table exists" "
  SELECT to_regclass('public.host_chains') IS NOT NULL
"

host_chain_rows="$(psql_scalar "
  SELECT count(*) FROM host_chains;
" 2>/dev/null || printf '')"

if [[ -n "$host_chain_rows" && "$host_chain_rows" != "0" ]]; then
  pass "host_chains contains configured rows ($host_chain_rows row(s))"
else
  warn "host_chains has no rows; expected only if this deployment does not seed chains through db-migration"
fi

check_true "branchless digest trigger from 20260610130300 was replaced by split triggers" "
  SELECT NOT EXISTS (
    SELECT 1
      FROM pg_trigger
     WHERE tgname = 'mirror_ciphertext_digest_branchless_trigger'
       AND tgrelid = 'public.ciphertext_digest'::regclass
       AND NOT tgisinternal
       AND tgenabled <> 'D'
  );
"

branch_rows="$(psql_scalar "
  SELECT
    (SELECT count(*) FROM computations_branch)
    + (SELECT count(*) FROM pbs_computations_branch)
    + (SELECT count(*) FROM allowed_handles_branch)
    + (SELECT count(*) FROM ciphertext_digest_branch)
    + (SELECT count(*) FROM ciphertexts_branch)
    + (SELECT count(*) FROM ciphertexts128_branch);
" 2>/dev/null || printf '')"

if [[ -n "$branch_rows" && "$branch_rows" != "0" ]]; then
  pass "branch tables contain rows ($branch_rows total)"
else
  warn "branch tables are currently empty; this is possible before activation or on an idle chain"
fi

latest_branch_write="$(psql_scalar "
  WITH branch_writes(created_at) AS (
    SELECT created_at FROM pbs_computations_branch
    UNION ALL
    SELECT created_at FROM ciphertext_digest_branch
    UNION ALL
    SELECT created_at FROM ciphertexts_branch
    UNION ALL
    SELECT created_at FROM ciphertexts128_branch
  )
  SELECT COALESCE(to_char(max(created_at), 'YYYY-MM-DD"T"HH24:MI:SS.US'), '')
    FROM branch_writes;
" 2>/dev/null || printf '')"

if [[ -z "$latest_branch_write" ]]; then
  warn "no timestamped branch writes found; liveness cannot be inferred without recent traffic"
else
  recent_branch_write="$(psql_scalar "
    WITH branch_writes(created_at) AS (
      SELECT created_at FROM pbs_computations_branch
      UNION ALL
      SELECT created_at FROM ciphertext_digest_branch
      UNION ALL
      SELECT created_at FROM ciphertexts_branch
      UNION ALL
      SELECT created_at FROM ciphertexts128_branch
    )
    SELECT max(created_at) >= now() - make_interval(mins => $DUAL_WRITE_RECENCY_MINUTES)
      FROM branch_writes;
  " 2>/dev/null || printf '')"
  if [[ "$recent_branch_write" == "t" ]]; then
    pass "recent branch dual-write activity observed within ${DUAL_WRITE_RECENCY_MINUTES}m"
  else
    warn "latest timestamped branch write is $latest_branch_write; no branch write observed within ${DUAL_WRITE_RECENCY_MINUTES}m"
  fi
fi

deadlocks="$(psql_scalar "
  SELECT deadlocks
    FROM pg_stat_database
   WHERE datname = current_database();
" 2>/dev/null || printf '')"

if [[ -z "$deadlocks" ]]; then
  warn "could not read pg_stat_database.deadlocks"
elif [[ -n "${DEADLOCKS_BASELINE:-}" ]]; then
  if [[ "$deadlocks" =~ ^[0-9]+$ && "$DEADLOCKS_BASELINE" =~ ^[0-9]+$ ]]; then
    if (( deadlocks > DEADLOCKS_BASELINE )); then
      warn "pg_stat_database.deadlocks increased from $DEADLOCKS_BASELINE to $deadlocks; inspect logs for SQLSTATE 40P01 retry churn"
    else
      pass "pg_stat_database.deadlocks did not increase above baseline ($deadlocks <= $DEADLOCKS_BASELINE)"
    fi
  else
    warn "DEADLOCKS_BASELINE must be an integer to compare deadlock counters"
  fi
elif [[ "$deadlocks" == "0" ]]; then
  pass "pg_stat_database.deadlocks is 0"
else
  warn "pg_stat_database.deadlocks is $deadlocks cumulative; use DEADLOCKS_BASELINE before/after rollout for a clean 40P01 signal"
fi

if [[ -n "${KUBECTL_NAMESPACE:-}" ]]; then
  if command -v kubectl >/dev/null 2>&1; then
    if kubectl -n "$KUBECTL_NAMESPACE" get jobs -l "$KUBECTL_SELECTOR" >/dev/null 2>&1; then
      completed_jobs="$(kubectl -n "$KUBECTL_NAMESPACE" get jobs -l "$KUBECTL_SELECTOR" \
        -o jsonpath='{range .items[*]}{.metadata.name}{" "}{.status.succeeded}{"\n"}{end}' \
        | awk '$2 >= 1 { count++ } END { print count + 0 }')"
      if [[ "$completed_jobs" != "0" ]]; then
        pass "Kubernetes db-migration job completed ($completed_jobs succeeded job(s))"
      else
        fail "Kubernetes db-migration job completed (no succeeded jobs for selector $KUBECTL_SELECTOR)"
      fi
    else
      fail "could not query Kubernetes jobs in namespace $KUBECTL_NAMESPACE"
    fi

    if [[ -n "${EXPECT_BRANCH_ACTIVATION_BLOCK:-}" ]]; then
      mismatches="$(kubectl -n "$KUBECTL_NAMESPACE" get deploy -o jsonpath='{range .items[*]}{.metadata.name}{" "}{range .spec.template.spec.containers[*].env[?(@.name=="FHEVM_BRANCH_ACTIVATION_BLOCK")]}{.value}{end}{"\n"}{end}' \
        | awk -v expected="$EXPECT_BRANCH_ACTIVATION_BLOCK" '
            /host-listener/ {
              seen++;
              if ($2 != expected) {
                bad++;
                print $1 "=" ($2 == "" ? "<unset>" : $2);
              }
            }
            END {
              if (seen == 0) {
                print "__NO_HOST_LISTENER_DEPLOYS__";
              } else if (bad == 0) {
                print "__OK__";
              }
            }')"
      if [[ "$mismatches" == "__OK__" ]]; then
        pass "host-listener deployments use EXPECT_BRANCH_ACTIVATION_BLOCK=$EXPECT_BRANCH_ACTIVATION_BLOCK"
      elif [[ "$mismatches" == "__NO_HOST_LISTENER_DEPLOYS__" ]]; then
        warn "no host-listener deployments found while checking FHEVM_BRANCH_ACTIVATION_BLOCK"
      else
        fail "FHEVM_BRANCH_ACTIVATION_BLOCK mismatch: $mismatches"
      fi
    fi
  else
    warn "KUBECTL_NAMESPACE set but kubectl is not installed; skipped Kubernetes checks"
  fi
else
  warn "KUBECTL_NAMESPACE not set; skipped Kubernetes checks"
fi

printf '\nRESULT '
if [[ "$failures" -eq 0 ]]; then
  printf 'passed: %d failure(s), %d warning(s)\n' "$failures" "$warnings"
  exit 0
fi

printf 'failed: %d failure(s), %d warning(s)\n' "$failures" "$warnings"
exit 1
