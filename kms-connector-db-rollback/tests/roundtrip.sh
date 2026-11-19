#!/usr/bin/env bash
#
# End-to-end test of the rollback tool against a throwaway Postgres.
#
# Requirements: Docker, and sqlx-cli 0.8.6 on the PATH
#   cargo install sqlx-cli --version 0.8.6 --locked --no-default-features --features rustls,postgres
#
# What it proves:
#   1. Round trip. Every migration is applied with the connector's own files
#      (kms-connector/connector-db/migrations, the way the connector-db image
#      applies them), then reverted one by one with migrate.sh. After each
#      revert the schema must be identical to the one recorded before that
#      migration.
#   2. Full rollback with data, in a single command, followed by a re-upgrade.
#   3. Rollback to each release target with rows shaped like the ones each
#      connector version writes, finished ones included.
#   4. Every down that refuses to run stops the rollback and changes nothing,
#      and deletes only what it says when forced.
#   5. Guards. The tool refuses to run when the image and the database are out
#      of sync.
#
# Usage: tests/roundtrip.sh
#   PG_IMAGE=postgres:15-alpine tests/roundtrip.sh   to test another Postgres

set -euo pipefail

HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
COMPONENT_DIR=$(dirname "$HERE")
REPO_ROOT=$(dirname "$COMPONENT_DIR")
CONNECTOR_MIGRATIONS="$REPO_ROOT/kms-connector/connector-db/migrations"
ROLLBACK_MIGRATIONS="$COMPONENT_DIR/migrations"
MIGRATE="$COMPONENT_DIR/scripts/migrate.sh"

CONTAINER=kms-connector-db-rollback-test
PORT=${PORT:-55440}
PG_IMAGE=${PG_IMAGE:-postgres:17-alpine}
export DATABASE_URL="postgres://postgres:postgres@localhost:$PORT/connector"
export MAX_ATTEMPTS=5

WORK=$(mktemp -d)
FAILURES=0

cleanup() {
    docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
    rm -rf "$WORK"
}
trap cleanup EXIT

pass() { echo "  ok    $1"; }
fail() { echo "  FAIL  $1"; FAILURES=$((FAILURES + 1)); }

psql_do() {
    docker exec -i "$CONTAINER" psql -U postgres -d connector -v ON_ERROR_STOP=1 -At "$@"
}

snapshot() {
    psql_do -f - <"$HERE/schema_snapshot.sql" >"$1"
}

# Recreates the database, so each scenario starts from nothing.
reset_db() {
    docker exec "$CONTAINER" psql -U postgres -q -c 'DROP DATABASE IF EXISTS connector WITH (FORCE)' -c 'CREATE DATABASE connector' >/dev/null 2>&1
}

# Applies the connector's own migrations up to a version, as the connector-db
# image does.
apply_up_to() {
    sqlx migrate run --no-dotenv --source "$CONNECTOR_MIGRATIONS" --target-version "$1" >/dev/null
}

apply_all() {
    sqlx migrate run --no-dotenv --source "$CONNECTOR_MIGRATIONS" >/dev/null
}

db_version() {
    psql_do -c 'SELECT coalesce(max(version), 0) FROM _sqlx_migrations'
}

count() {
    psql_do -c "SELECT count(*) FROM $1"
}

# check <label> <actual> <expected>
check() {
    if [[ "$2" == "$3" ]]; then pass "$1"; else fail "$1: expected '$3', got '$2'"; fi
}

# Compares the current schema with the one recorded after a version was applied.
check_schema() {
    local version=$1 label=$2
    snapshot "$WORK/current"
    if diff -u "$WORK/up_$version" "$WORK/current" >"$WORK/diff_current"; then
        pass "$label"
    else
        fail "$label"
        sed 's/^/        /' "$WORK/diff_current"
    fi
}

# Runs migrate.sh expecting a failure whose output contains a given text.
expect_failure() {
    local label=$1 expected=$2
    shift 2
    local output
    if output=$("$@" 2>&1); then
        fail "$label: expected a failure, got success: $output"
    elif [[ "$output" == *"$expected"* ]]; then
        pass "$label"
    else
        fail "$label: failed, but without '$expected': $output"
    fi
}

expect_success() {
    local label=$1
    shift
    local output
    if output=$("$@" 2>&1); then
        pass "$label"
    else
        fail "$label: $output"
    fi
}

# -------------------------------------------------------------------------
# Setup
# -------------------------------------------------------------------------

command -v sqlx >/dev/null || { echo "sqlx-cli is required, see the header of this script" >&2; exit 1; }

echo "== The rollback copies are identical to the connector's migrations"
for file in "$CONNECTOR_MIGRATIONS"/*.sql; do
    name=$(basename "$file" .sql)
    if cmp -s "$file" "$ROLLBACK_MIGRATIONS/$name.up.sql"; then
        pass "$name"
    else
        fail "$name: missing from or different in $ROLLBACK_MIGRATIONS"
    fi
done

VERSIONS=()
for file in "$CONNECTOR_MIGRATIONS"/*.sql; do
    name=$(basename "$file")
    VERSIONS+=("${name%%_*}")
done
FIRST=${VERSIONS[0]}
LAST=${VERSIONS[${#VERSIONS[@]} - 1]}

docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
docker run -d --name "$CONTAINER" -e POSTGRES_PASSWORD=postgres -p "$PORT:5432" "$PG_IMAGE" >/dev/null
until docker exec "$CONTAINER" pg_isready -U postgres >/dev/null 2>&1; do sleep 1; done
sleep 1
echo "== Postgres: $(docker exec "$CONTAINER" psql -U postgres -At -c 'SHOW server_version')"

# -------------------------------------------------------------------------
# 1. Round trip, one migration at a time
# -------------------------------------------------------------------------

echo "== Round trip: schema after each down equals the schema before its up"
reset_db
for version in "${VERSIONS[@]}"; do
    apply_up_to "$version"
    snapshot "$WORK/up_$version"
done

for ((i = ${#VERSIONS[@]} - 1; i > 0; i--)); do
    reverted=${VERSIONS[i]}
    target=${VERSIONS[i - 1]}
    if ! output=$("$MIGRATE" rollback "$target" 2>&1); then
        fail "revert $reverted: $output"
        break
    fi
    check_schema "$target" "revert $reverted -> schema matches $target"
done

echo "== Upgrade again from the baseline"
if apply_all 2>"$WORK/reapply_error"; then
    check_schema "$LAST" "re-applied every migration, schema matches $LAST"
else
    fail "re-applying the migrations failed: $(cat "$WORK/reapply_error")"
fi

# -------------------------------------------------------------------------
# Fixtures
# -------------------------------------------------------------------------

V014_TARGET=20260701000000 # last migration of v0.14.x
V013_TARGET=20260407130028 # last migration of v0.13.x

MATERIAL="ARRAY[ROW('\xaa', '\xbb', '\xcc', ARRAY['\xdd'::bytea])::sns_ciphertext_material]"
ZERO_KEY="'\x0000000000000000000000000000000000000000000000000000000000000000'"
LONG_AGO="'2026-01-01 00:00:00+00'"

# Rows written the way v0.14 writes them: decryptions stored with their
# material, finished key generations, polling positions. The connector never
# deletes finished rows, so a real database is mostly made of them.
seed_v014_rows() {
    psql_do -q <<SQL
INSERT INTO public_decryption_requests (decryption_id, sns_ct_materials, extra_data, created_at, otlp_context, status) VALUES
    ('\x01', $MATERIAL, '\x', now(), '\x00', 'pending'),
    ('\x02', $MATERIAL, '\x', now(), '\x00', 'under_process'),
    ('\x03', $MATERIAL, '\x', now(), '\x00', 'failed');
INSERT INTO user_decryption_requests (decryption_id, sns_ct_materials, user_address, public_key, extra_data, created_at, otlp_context, status) VALUES
    ('\x11', $MATERIAL, '\x01', '\x02', '\x', now(), '\x00', 'pending');
-- A finished unified (RFC016) request.
INSERT INTO user_decryption_requests (decryption_id, sns_ct_materials, user_address, public_key, extra_data, created_at, otlp_context, status, signature, allowed_contracts) VALUES
    ('\x12', $MATERIAL, '\x01', '\x02', '\x', now(), '\x00', 'completed', '\x07', '{}');
INSERT INTO public_decryption_responses (decryption_id, decrypted_result, signature, extra_data, created_at, otlp_context, status) VALUES
    ('\x02', '\x01', '\x02', '\x', now(), '\x00', 'completed');
INSERT INTO crsgen_requests (crs_id, max_bit_length, params_type, created_at, otlp_context, status) VALUES
    ('\x41', '\x0800', 'Test', now(), '\x00', 'aborted');
INSERT INTO keygen_responses (key_id, key_digests, signature, created_at, otlp_context, status) VALUES
    ('\x31', ARRAY[ROW('Server', '\x01')::key_digest, ROW('Public', '\x02')::key_digest], '\x05', now(), '\x00', 'completed');
UPDATE last_block_polled SET block_number = 100;
UPDATE last_block_polled_by_chain SET block_number = 700 WHERE chain_name = 'ethereum';
UPDATE last_block_polled_by_chain SET block_number = 900 WHERE chain_name = 'gateway';
SQL
    # Give every row an old `updated_at`, to detect a down that touches it.
    psql_do -q <<SQL
SET session_replication_role = replica;
UPDATE public_decryption_requests SET updated_at = $LONG_AGO;
UPDATE user_decryption_requests SET updated_at = $LONG_AGO;
UPDATE crsgen_requests SET updated_at = $LONG_AGO;
SQL
}

# Rows only v0.15 writes: requests stored with handles only, HTTP-originated
# work, error responses. All of them finished.
seed_v015_finished_rows() {
    psql_do -q <<SQL
INSERT INTO public_decryption_requests (decryption_id, extra_data, created_at, otlp_context, status, ct_handles) VALUES
    ('\x05', '\x', now(), '\x00', 'completed', ARRAY['\xaa'::bytea]),
    ('\x06', '\x', now(), '\x00', 'failed', ARRAY['\xaa'::bytea]);
INSERT INTO public_decryption_requests (decryption_id, extra_data, created_at, otlp_context, status, ct_handles, source) VALUES
    ('\x07', '\x', now(), '\x00', 'completed', ARRAY['\xaa'::bytea], 'http');
INSERT INTO user_decryption_requests (decryption_id, user_address, public_key, extra_data, created_at, otlp_context, status, ct_handles, signature, allowed_contracts, source) VALUES
    ('\x15', '\x01', '\x02', '\x', now(), '\x00', 'completed', ARRAY['\xaa'::bytea], '\x07', '{}', 'onchain'),
    ('\x16', '\x01', '\x02', '\x', now(), '\x00', 'failed', ARRAY['\xaa'::bytea], '\x07', '{}', 'http');
INSERT INTO public_decryption_responses (decryption_id, decrypted_result, signature, extra_data, created_at, otlp_context, status) VALUES
    ('\x05', '\x01', '\x02', '\x', now(), '\x00', 'completed');
INSERT INTO public_decryption_responses (decryption_id, decrypted_result, signature, extra_data, created_at, otlp_context, status, source) VALUES
    ('\x07', '\x01', '\x02', '\x', now(), '\x00', 'pending', 'http');
INSERT INTO public_decryption_responses (decryption_id, extra_data, created_at, otlp_context, error_code) VALUES
    ('\x06', '\x', now(), '\x00', 'rejected');
INSERT INTO user_decryption_responses (decryption_id, user_decrypted_shares, signature, extra_data, created_at, otlp_context, status) VALUES
    ('\x15', '\x01', '\x02', '\x', now(), '\x00', 'completed');
INSERT INTO user_decryption_responses (decryption_id, extra_data, created_at, otlp_context, error_code, source) VALUES
    ('\x16', '\x', now(), '\x00', 'rejected', 'http');
SET session_replication_role = replica;
UPDATE public_decryption_requests SET updated_at = $LONG_AGO;
UPDATE user_decryption_requests SET updated_at = $LONG_AGO;
SQL
}

request_rows() {
    psql_do -c "
        SELECT 'public', encode(decryption_id, 'hex'), status::text, updated_at::text FROM public_decryption_requests
        UNION ALL
        SELECT 'user', encode(decryption_id, 'hex'), status::text, updated_at::text FROM user_decryption_requests
        UNION ALL
        SELECT 'crsgen', encode(crs_id, 'hex'), status::text, updated_at::text FROM crsgen_requests
        UNION ALL
        SELECT 'keygen_response', encode(key_id, 'hex'), status::text, key_digests::text FROM keygen_responses
        ORDER BY 1, 2"
}

ids() {
    psql_do -c "SELECT coalesce(string_agg(encode($2, 'hex'), ',' ORDER BY $2), '') FROM $1"
}

# -------------------------------------------------------------------------
# 2. Rollback to a release, with the data a running connector leaves behind
# -------------------------------------------------------------------------

echo "== Rollback from the newest migration to v0.14.x, without --force"
reset_db
apply_all
seed_v014_rows
seed_v015_finished_rows
psql_do -q <<'SQL'
-- c1: live context, one live epoch and one destroyed epoch.
-- c2: destroyed context that still has a live epoch cached.
-- c3: live context whose only cached epoch was destroyed.
INSERT INTO kms_context (id, is_valid, created_at, updated_at) VALUES
    ('\xc1', TRUE, now(), now()),
    ('\xc2', FALSE, now(), now()),
    ('\xc3', TRUE, now(), now());
INSERT INTO kms_epoch (id, context_id, is_valid, created_at, updated_at) VALUES
    ('\xe1', '\xc1', TRUE, now(), now()),
    ('\xe2', '\xc1', FALSE, now(), now()),
    ('\xe3', NULL, FALSE, now(), now()),
    ('\xe4', '\xc2', TRUE, now(), now()),
    ('\xe5', '\xc3', FALSE, now(), now());
INSERT INTO kms_context_destroyed (context_id, created_at, otlp_context) VALUES ('\xc9', now(), '\x00');
INSERT INTO new_kms_context_responses (context_id, created_at, otlp_context, signature, extra_data) VALUES ('\xc1', now(), '\x00', '\x01', '\x');
SQL
request_rows >"$WORK/rows_before"

expect_success "rollback $LAST -> $V014_TARGET" "$MIGRATE" rollback "$V014_TARGET"
check_schema "$V014_TARGET" "schema matches $V014_TARGET"
check "migration history is at $V014_TARGET" "$(db_version)" "$V014_TARGET"

request_rows >"$WORK/rows_after"
if diff -u "$WORK/rows_before" "$WORK/rows_after" >"$WORK/diff_rows"; then
    pass "every request is kept, with its status and updated_at"
else
    fail "requests changed"
    sed 's/^/        /' "$WORK/diff_rows"
fi

check "finished handle-only requests get an empty material list" \
    "$(psql_do -c "SELECT string_agg(encode(decryption_id, 'hex') || '=' || cardinality(sns_ct_materials), ',' ORDER BY decryption_id) FROM public_decryption_requests")" \
    "01=1,02=1,03=1,05=0,06=0,07=0"
check "HTTP-originated and error responses are deleted, the others are kept" \
    "$(ids public_decryption_responses decryption_id)/$(ids user_decryption_responses decryption_id)" \
    "02,05/15"
check "no response is left for the previous tx-sender to submit by mistake" \
    "$(psql_do -c "SELECT count(*) FROM public_decryption_responses WHERE status = 'pending'")" 0

# The previous version rejects every request of a context that has one invalid
# row: only a destroyed context may have one.
check "validation cache: live epochs only, carrying the validity of their context" \
    "$(psql_do -c "SELECT string_agg(encode(id, 'hex') || '/' || encode(epoch_id, 'hex') || '=' || is_valid, ',' ORDER BY id, epoch_id) FROM kms_context")" \
    "c1/e1=true,c2/e4=false"
check "the per-chain polling cursor is untouched" \
    "$(psql_do -c "SELECT string_agg(chain_name || '=' || block_number, ',' ORDER BY chain_name) FROM last_block_polled_by_chain")" \
    "ethereum=700,gateway=900"
check "context responses survive without their signature" "$(count new_kms_context_responses)" 1

if apply_all 2>"$WORK/reapply_error"; then
    check_schema "$LAST" "upgraded again, schema matches $LAST"
else
    fail "upgrading again failed: $(cat "$WORK/reapply_error")"
fi

echo "== Rollback from v0.14.x to v0.13.x, without --force"
reset_db
apply_up_to "$V014_TARGET"
seed_v014_rows
psql_do -q <<'SQL'
-- c1: destroyed after its newest row was written. c2: live.
-- c3: an older row is invalid, a newer one is valid: still destroyed.
INSERT INTO kms_context (id, epoch_id, is_valid, created_at, updated_at) VALUES
    ('\xc1', '\xe1', TRUE, now(), now() - interval '1 hour'),
    ('\xc1', '\xe2', FALSE, now(), now()),
    ('\xc2', '\xe1', TRUE, now(), now()),
    ('\xc3', '\xe1', FALSE, now(), now() - interval '1 hour'),
    ('\xc3', '\xe2', TRUE, now(), now());
INSERT INTO abort_crsgen_requests (crs_id, created_at, otlp_context) VALUES ('\x41', now(), '\x00');
SQL

expect_success "rollback $V014_TARGET -> $V013_TARGET" "$MIGRATE" rollback "$V013_TARGET"
check_schema "$V013_TARGET" "schema matches $V013_TARGET"

check "an aborted request becomes failed, without a new updated_at" \
    "$(psql_do -c "SELECT status || ' ' || (updated_at = $LONG_AGO) FROM crsgen_requests WHERE crs_id = '\x41'")" "failed true"
check "other statuses are unchanged" \
    "$(psql_do -c "SELECT string_agg(status::text, ',' ORDER BY decryption_id) FROM public_decryption_requests")" \
    "pending,completed,failed"
check "a finished unified request is kept" "$(ids user_decryption_requests decryption_id)" "11,12"
check "the polling position is copied back to the per-event cursor" \
    "$(psql_do -c "SELECT string_agg(event_type || '=' || block_number, ',' ORDER BY event_type) FROM last_block_polled")" \
    "PublicDecryptionRequest=900,UserDecryptionRequest=900,PrepKeygenRequest=700,KeygenRequest=700,CrsgenRequest=700,PrssInit=100,KeyReshareSameSet=100"
check "validation cache: one row per context, invalid if any of its rows was" \
    "$(psql_do -c "SELECT string_agg(encode(id, 'hex') || '=' || is_valid, ',' ORDER BY id) FROM kms_context")" \
    "c1=false,c2=true,c3=false"

if apply_all 2>"$WORK/reapply_error"; then
    check_schema "$LAST" "upgraded again, schema matches $LAST"
else
    fail "upgrading again failed: $(cat "$WORK/reapply_error")"
fi

# -------------------------------------------------------------------------
# 3. Downs that refuse to run, and what --force makes them do
# -------------------------------------------------------------------------

# refusal <version> <text expected in the error>
#
# Runs a rollback to the baseline and expects it to stop at <version>, with
# the schema of that version and every row still there.
ROW_COUNTS="
    SELECT (SELECT count(*) FROM public_decryption_requests) || '/' ||
           (SELECT count(*) FROM user_decryption_requests) || '/' ||
           (SELECT count(*) FROM public_decryption_responses) || '/' ||
           (SELECT count(*) FROM keygen_requests) || '/' ||
           (SELECT count(*) FROM crsgen_requests)"

refusal() {
    local version=$1 expected=$2 before
    before=$(psql_do -c "$ROW_COUNTS")
    expect_failure "$version refuses to run" "$expected" "$MIGRATE" rollback "$FIRST"
    check "$version: the rollback stopped there" "$(db_version)" "$version"
    check_schema "$version" "$version: the refused down changed nothing in the schema"
    check "$version: the refused down deleted nothing" "$(psql_do -c "$ROW_COUNTS")" "$before"
}

echo "== Downs that refuse to run, then --force"
reset_db
apply_all
seed_v014_rows
seed_v015_finished_rows
psql_do -q <<SQL
-- 20260828000000: in-flight HTTP-originated requests.
INSERT INTO public_decryption_requests (decryption_id, extra_data, created_at, otlp_context, ct_handles, source, status) VALUES
    ('\xa1', '\x', now(), '\x00', ARRAY['\xaa'::bytea], 'http', 'pending');
INSERT INTO user_decryption_requests (decryption_id, user_address, public_key, extra_data, created_at, otlp_context, ct_handles, signature, allowed_contracts, source, status) VALUES
    ('\xa2', '\x01', '\x02', '\x', now(), '\x00', ARRAY['\xaa'::bytea], '\x07', '{}', 'http', 'under_process');
-- 20260805100000: an in-flight keygen that targets an existing key.
INSERT INTO keygen_requests (prep_keygen_id, key_id, created_at, otlp_context, existing_key_id, status) VALUES
    ('\xb1', '\xb2', now(), '\x00', '\x01', 'pending'),
    ('\xb3', '\xb4', now(), '\x00', $ZERO_KEY, 'pending'),
    ('\xb5', '\xb6', now(), '\x00', '\x01', 'completed');
-- 20260713000000: in-flight on-chain requests stored with handles only.
INSERT INTO public_decryption_requests (decryption_id, extra_data, created_at, otlp_context, ct_handles, status) VALUES
    ('\xd1', '\x', now(), '\x00', ARRAY['\xaa'::bytea], 'pending');
INSERT INTO user_decryption_requests (decryption_id, user_address, public_key, extra_data, created_at, otlp_context, ct_handles, signature, allowed_contracts, status) VALUES
    ('\xd2', '\x01', '\x02', '\x', now(), '\x00', ARRAY['\xaa'::bytea], '\x07', '{}', 'under_process');
-- 20260421092426: an in-flight unified request that has its material.
INSERT INTO user_decryption_requests (decryption_id, sns_ct_materials, user_address, public_key, extra_data, created_at, otlp_context, signature, allowed_contracts) VALUES
    ('\xf1', $MATERIAL, '\x01', '\x02', '\x', now(), '\x00', '\x07', '{}');
-- 20260407130028: a prep keygen request, whose epoch lives in extra_data.
INSERT INTO prep_keygen_requests (prep_keygen_id, params_type, created_at, otlp_context, existing_key_id, status) VALUES
    ('\x51', 'Test', now(), '\x00', $ZERO_KEY, 'completed');
SQL
# 20260804000000: the new enum value cannot be used in the transaction that
# adds it, so it is inserted separately.
psql_do -q <<'SQL'
INSERT INTO keygen_responses (key_id, key_digests, signature, created_at, otlp_context) VALUES
    ('\x32', ARRAY[ROW('CompressedKeySet', '\x03')::key_digest], '\x05', now(), '\x00');
SQL

refusal 20260828000000 "in-flight HTTP-originated requests"
expect_failure "anything other than --force is refused" "usage:" "$MIGRATE" rollback 20260805100000 --yes
if output=$("$MIGRATE" rollback 20260805100000 --force 2>&1); then
    [[ $output == *"force=true"* ]] && pass "forced rollback to 20260805100000" || fail "forced rollback did not report force=true: $output"
else
    fail "forced rollback to 20260805100000: $output"
fi
check_schema 20260805100000 "schema matches 20260805100000"
check "only the in-flight HTTP-originated requests were deleted" \
    "$(ids public_decryption_requests decryption_id)/$(ids user_decryption_requests decryption_id)" \
    "01,02,03,05,06,07,d1/11,12,15,16,d2,f1"

refusal 20260805100000 "target an existing key"
expect_success "forced rollback to 20260804000000" "$MIGRATE" rollback 20260804000000 --force
check "only the in-flight keygen targeting an existing key is failed" \
    "$(psql_do -c "SELECT string_agg(encode(key_id, 'hex') || '=' || status, ',' ORDER BY key_id) FROM keygen_requests")" \
    "b2=failed,b4=pending,b6=completed"

refusal 20260804000000 "CompressedKeySet"
expect_success "forced rollback to 20260713000000" "$MIGRATE" rollback 20260713000000 --force
check "only the response with a compressed digest was deleted" \
    "$(psql_do -c "SELECT string_agg(encode(key_id, 'hex') || '=' || key_digests::text, ',' ORDER BY key_id) FROM keygen_responses")" \
    '31={"(Server,\"\\\\x01\")","(Public,\"\\\\x02\")"}'

refusal 20260713000000 "handles only"
expect_success "forced rollback to 20260708000000" "$MIGRATE" rollback 20260708000000 --force
check "only the in-flight handle-only requests were deleted" \
    "$(ids public_decryption_requests decryption_id)/$(ids user_decryption_requests decryption_id)" \
    "01,02,03,05,06,07/11,12,15,16,f1"

refusal 20260421092426 "in-flight rows in user_decryption_requests are unified"
expect_success "forced rollback to 20260407130028" "$MIGRATE" rollback 20260407130028 --force
check "only the in-flight unified request was deleted" "$(ids user_decryption_requests decryption_id)" "11,12,15,16"

refusal 20260407130028 "epoch only inside extra_data"
expect_success "forced rollback to 20260226161117" "$MIGRATE" rollback 20260226161117 --force
check "the prep keygen requests were deleted" "$(count prep_keygen_requests)" 0

echo "== Rollback across the status / under_process change"
expect_success "rollback to 20251113084850" "$MIGRATE" rollback 20251113084850
check_schema 20251113084850 "schema matches 20251113084850"
check "only pending work stays unlocked" \
    "$(psql_do -c "SELECT string_agg(encode(decryption_id, 'hex') || '=' || under_process, ',' ORDER BY decryption_id) FROM public_decryption_requests")" \
    "01=false,02=true,03=true,05=true,06=true,07=true"
check "a sent response stays locked" \
    "$(psql_do -c "SELECT string_agg(encode(decryption_id, 'hex') || '=' || under_process, ',' ORDER BY decryption_id) FROM public_decryption_responses")" \
    "02=true,05=true"

refusal 20250826111757 "previous columns do not describe"
expect_success "forced rollback to the baseline" "$MIGRATE" rollback "$FIRST" --force
check_schema "$FIRST" "schema matches the baseline"
check "the repurposed key generation rows were deleted" "$(count keygen_requests)/$(count crsgen_requests)" "0/0"
check "decryption requests survive down to the baseline" "$(count public_decryption_requests)/$(count user_decryption_requests)" "6/4"

echo "== Upgrade again after the full rollback"
if apply_all 2>"$WORK/reapply_error"; then
    check_schema "$LAST" "re-applied every migration, schema matches $LAST"
    check "decryption requests survive the round trip" "$(count public_decryption_requests)/$(count user_decryption_requests)" "6/4"
else
    fail "re-applying the migrations failed: $(cat "$WORK/reapply_error")"
fi

# -------------------------------------------------------------------------
# 4. Guards
# -------------------------------------------------------------------------

echo "== Guards"
reset_db
apply_all

[[ $("$MIGRATE" info 2>&1 | tail -1) == *"in_sync=true"* ]] && pass "info reports in sync" || fail "info does not report in sync"

[[ $("$MIGRATE" rollback "$LAST" 2>&1) == *"status=noop"* ]] && pass "rollback to the current version is a no-op" || fail "rollback to the current version is not a no-op"

expect_failure "target 0 is refused" "target version 0 is not allowed" "$MIGRATE" rollback 0
expect_failure "unknown target is refused" "not a migration known to this image" "$MIGRATE" rollback 20990101000000

# The database is ahead of the image: the image lacks the newest migration.
behind="$WORK/behind"
mkdir "$behind"
cp "$ROLLBACK_MIGRATIONS"/* "$behind/"
rm "$behind/$LAST"_*
expect_failure "image behind the database: info is refused" "out of sync: the database has migrations this image does not know" \
    env MIGRATIONS_DIR="$behind" "$MIGRATE" info
expect_failure "image behind the database: rollback is refused" "out of sync: the database has migrations this image does not know" \
    env MIGRATIONS_DIR="$behind" "$MIGRATE" rollback "$FIRST"

# Same version, different content.
edited="$WORK/edited"
mkdir "$edited"
cp "$ROLLBACK_MIGRATIONS"/* "$edited/"
echo "-- edited" >>"$(ls "$edited/$LAST"_*.up.sql)"
expect_failure "edited migration: rollback is refused" "out of sync: the migrations applied to the database differ" \
    env MIGRATIONS_DIR="$edited" "$MIGRATE" rollback "$FIRST"

# A migration without a down file.
no_down="$WORK/no_down"
mkdir "$no_down"
cp "$ROLLBACK_MIGRATIONS"/* "$no_down/"
rm "$no_down/$LAST"_*.down.sql
expect_failure "missing down file: rollback is refused" "missing down file" \
    env MIGRATIONS_DIR="$no_down" "$MIGRATE" rollback "$FIRST"

check "no refused run changed the database" "$(db_version)" "$LAST"

# The database is behind the image: an older release, newer image. Allowed.
reset_db
apply_up_to "$V014_TARGET"
[[ $("$MIGRATE" info 2>&1 | tail -1) == *"current=$V014_TARGET"*"in_sync=true"* ]] \
    && pass "an image newer than the database is in sync" || fail "an image newer than the database is not reported in sync"
expect_failure "a target that is not applied is refused" "is not applied to this database" "$MIGRATE" rollback "$LAST"

echo
if [[ $FAILURES -eq 0 ]]; then
    echo "All checks passed."
else
    echo "$FAILURES check(s) failed."
    exit 1
fi
