#!/usr/bin/env bash
#
# End-to-end test of the rollback tool against a throwaway Postgres.
#
# Requirements: Docker, and sqlx-cli 0.8.6 on the PATH
#   cargo install sqlx-cli --version 0.8.6 --locked --no-default-features --features rustls,postgres
#
# What it proves:
#   1. Round trip. Every migration is applied with the relayer's own files
#      (relayer/relayer-migrate/migrations, the way relayer-migrate applies
#      them), then reverted one by one with migrate.sh. After each revert the
#      schema must be identical to the one recorded before that migration.
#   2. Full rollback with data, in a single command, followed by a re-upgrade.
#   3. Guards. The tool refuses to run when the image and the database are out
#      of sync, and reports where it stopped when a down fails.
#
# Usage: tests/roundtrip.sh

set -euo pipefail

HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
COMPONENT_DIR=$(dirname "$HERE")
REPO_ROOT=$(dirname "$COMPONENT_DIR")
RELAYER_MIGRATIONS="$REPO_ROOT/relayer/relayer-migrate/migrations"
ROLLBACK_MIGRATIONS="$COMPONENT_DIR/migrations"
MIGRATE="$COMPONENT_DIR/scripts/migrate.sh"

CONTAINER=relayer-db-rollback-test
PORT=${PORT:-55439}
export DATABASE_URL="postgres://postgres:postgres@localhost:$PORT/relayer"
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
    docker exec -i "$CONTAINER" psql -U postgres -d relayer -v ON_ERROR_STOP=1 -At "$@"
}

snapshot() {
    psql_do -f - <"$HERE/schema_snapshot.sql" >"$1"
}

# Recreates the database, so each scenario starts from nothing.
reset_db() {
    docker exec "$CONTAINER" psql -U postgres -q -c 'DROP DATABASE IF EXISTS relayer WITH (FORCE)' -c 'CREATE DATABASE relayer' >/dev/null 2>&1
}

# Applies the relayer's own migrations up to a version, as relayer-migrate does.
apply_up_to() {
    sqlx migrate run --no-dotenv --source "$RELAYER_MIGRATIONS" --target-version "$1" >/dev/null
}

apply_all() {
    sqlx migrate run --no-dotenv --source "$RELAYER_MIGRATIONS" >/dev/null
}

db_version() {
    psql_do -c 'SELECT coalesce(max(version), 0) FROM _sqlx_migrations'
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

# -------------------------------------------------------------------------
# Setup
# -------------------------------------------------------------------------

command -v sqlx >/dev/null || { echo "sqlx-cli is required, see the header of this script" >&2; exit 1; }

echo "== The rollback copies are identical to the relayer's migrations"
for file in "$RELAYER_MIGRATIONS"/*.sql; do
    name=$(basename "$file" .sql)
    if cmp -s "$file" "$ROLLBACK_MIGRATIONS/$name.up.sql"; then
        pass "$name"
    else
        fail "$name: missing from or different in $ROLLBACK_MIGRATIONS"
    fi
done

VERSIONS=()
for file in "$RELAYER_MIGRATIONS"/*.sql; do
    name=$(basename "$file")
    VERSIONS+=("${name%%_*}")
done
FIRST=${VERSIONS[0]}
LAST=${VERSIONS[${#VERSIONS[@]} - 1]}

docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
docker run -d --name "$CONTAINER" -e POSTGRES_PASSWORD=postgres -p "$PORT:5432" postgres:17-alpine >/dev/null
until docker exec "$CONTAINER" pg_isready -U postgres >/dev/null 2>&1; do sleep 1; done
sleep 1

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
    snapshot "$WORK/down_$target"
    if diff -u "$WORK/up_$target" "$WORK/down_$target" >"$WORK/diff_$target"; then
        pass "revert $reverted -> schema matches $target"
    else
        fail "revert $reverted -> schema differs from $target"
        sed 's/^/        /' "$WORK/diff_$target"
    fi
done

# -------------------------------------------------------------------------
# 2. Full rollback with data, then upgrade again
# -------------------------------------------------------------------------

echo "== Full rollback with data, in one command"
reset_db
apply_all
psql_do -q <<'SQL'
INSERT INTO input_proof_req (ext_job_id, int_job_id, req, req_status) VALUES
    (gen_random_uuid(), '\x01', '{}', 'queued'),
    (gen_random_uuid(), '\x02', '{}', 'processing'),
    (gen_random_uuid(), '\x03', '{}', 'completed'),
    (gen_random_uuid(), '\x04', '{}', 'failure');
INSERT INTO public_decrypt_req (ext_job_id, int_job_id, req, req_status) VALUES
    (gen_random_uuid(), '\x11', '{}', 'queued'),
    (gen_random_uuid(), '\x12', '{}', 'completed');
INSERT INTO user_decrypt_req (ext_job_id, int_job_id, req, req_status, req_type) VALUES
    (gen_random_uuid(), '\x21', '{}', 'completed', 'user_decrypt'),
    (gen_random_uuid(), '\x22', '{}', 'queued', 'delegated_user_decrypt');
INSERT INTO gateway_block_number_store (instance_id, last_block_number, last_block_hash) VALUES (0, 100, '0xabc');
SQL

if output=$("$MIGRATE" rollback "$FIRST" 2>&1); then
    pass "rollback $LAST -> $FIRST"
else
    fail "rollback $LAST -> $FIRST: $output"
fi

snapshot "$WORK/full_down"
if diff -u "$WORK/up_$FIRST" "$WORK/full_down" >"$WORK/diff_full"; then
    pass "schema matches the baseline"
else
    fail "schema differs from the baseline"
    sed 's/^/        /' "$WORK/diff_full"
fi

[[ $(db_version) == "$FIRST" ]] && pass "migration history is at $FIRST" || fail "migration history is at $(db_version)"

[[ $(psql_do -c 'SELECT count(*) FROM input_proof_req') == 4 ]] && pass "no row was deleted" || fail "rows were deleted"

in_flight=$(psql_do -c "SELECT count(*) FROM input_proof_req WHERE req_status NOT IN ('completed', 'failure', 'timed_out')")
[[ $in_flight == 0 ]] && pass "in-flight input proof requests were failed" || fail "$in_flight input proof requests are still in flight"

[[ $(psql_do -c 'SELECT last_block_number FROM gateway_block_number_store') == 100 ]] \
    && pass "gateway_block_number_store is untouched" || fail "gateway_block_number_store changed"

echo "== Upgrade again after the rollback"
# 20260204120000 stamps existing user decrypt rows as 'legacy' and
# 20260519152353 refuses to run while such rows exist. That is how the relayer's
# migrations behave on any database with old rows; in production the expiry cron
# clears them. Clear them here to reach the end of the upgrade.
psql_do -q -c 'DELETE FROM user_decrypt_req'
if apply_all 2>"$WORK/reapply_error"; then
    snapshot "$WORK/reapplied"
    if diff -u "$WORK/up_$LAST" "$WORK/reapplied" >"$WORK/diff_reapplied"; then
        pass "re-applied every migration, schema matches $LAST"
    else
        fail "schema after re-applying differs from $LAST"
        sed 's/^/        /' "$WORK/diff_reapplied"
    fi
else
    fail "re-applying the migrations failed: $(cat "$WORK/reapply_error")"
fi

# -------------------------------------------------------------------------
# 3. Rollback to a release, with the data a running relayer leaves behind
# -------------------------------------------------------------------------

RELEASE_TARGET=20260519152353 # last migration of v0.14.x
EPOCH_ONLY_TARGET=20260825120000 # reverts the epoch sequence and nothing below it

# What a rollback above 20260204120000 must leave untouched in the request tables.
stable_rows() {
    psql_do -c "
        SELECT 'user', ext_job_id, encode(int_job_id, 'hex'), req_status, req_type::text, resolved_threshold::text FROM user_decrypt_req
        UNION ALL
        SELECT 'public', ext_job_id, encode(int_job_id, 'hex'), req_status, '', '' FROM public_decrypt_req
        UNION ALL
        SELECT 'input', ext_job_id, encode(int_job_id, 'hex'), req_status, '', '' FROM input_proof_req
        ORDER BY 1, 2"
}

echo "== Rollback to a release with live data"
reset_db
apply_all
psql_do -q <<'SQL'
SELECT setval('dispatcher_epoch_seq', 41) \gset
INSERT INTO input_proof_req (ext_job_id, int_job_id, req, req_status, owner_epoch) VALUES
    (gen_random_uuid(), '\x01', '{}', 'queued', 41),
    (gen_random_uuid(), '\x02', '{}', 'completed', 40),
    (gen_random_uuid(), '\x02', '{}', 'failure', 0);
INSERT INTO public_decrypt_req (ext_job_id, int_job_id, req, req_status, owner_epoch) VALUES
    (gen_random_uuid(), '\x11', '{}', 'tx_in_flight', 41),
    (gen_random_uuid(), '\x12', '{}', 'timed_out', 7),
    (gen_random_uuid(), '\x12', '{}', 'queued', 41);
INSERT INTO user_decrypt_req (ext_job_id, int_job_id, req, req_status, req_type, resolved_threshold, owner_epoch) VALUES
    (gen_random_uuid(), '\x21', '{}', 'completed', 'user_decrypt', 3, 12),
    (gen_random_uuid(), '\x22', '{}', 'queued', 'delegated_user_decrypt', NULL, 41);
INSERT INTO user_decrypt_share (gw_reference_id, share_index, share, kms_signature, extra_data) VALUES ('\xaa', 0, 's', 'sig', 'x');
INSERT INTO gateway_chain_cursor (id, last_block_number, owner_epoch) VALUES (TRUE, 500, 41);
SQL
stable_rows >"$WORK/rows_before"

if output=$("$MIGRATE" rollback "$EPOCH_ONLY_TARGET" 2>&1); then
    stamped=$(psql_do -c "
        SELECT (SELECT count(*) FROM user_decrypt_req WHERE owner_epoch <> 0)
             + (SELECT count(*) FROM public_decrypt_req WHERE owner_epoch <> 0)
             + (SELECT count(*) FROM input_proof_req WHERE owner_epoch <> 0)
             + (SELECT count(*) FROM gateway_chain_cursor WHERE owner_epoch <> 0)")
    [[ $stamped == 0 ]] && pass "reverting the epoch sequence resets every owner_epoch" || fail "$stamped rows keep an owner_epoch no sequence can reach"
else
    fail "rollback to $EPOCH_ONLY_TARGET: $output"
fi

if output=$("$MIGRATE" rollback "$RELEASE_TARGET" 2>&1); then
    pass "rollback to $RELEASE_TARGET"
else
    fail "rollback to $RELEASE_TARGET: $output"
fi

snapshot "$WORK/release_down"
if diff -u "$WORK/up_$RELEASE_TARGET" "$WORK/release_down" >"$WORK/diff_release"; then
    pass "schema matches $RELEASE_TARGET"
else
    fail "schema differs from $RELEASE_TARGET"
    sed 's/^/        /' "$WORK/diff_release"
fi

stable_rows >"$WORK/rows_after"
if diff -u "$WORK/rows_before" "$WORK/rows_after" >"$WORK/diff_rows"; then
    pass "request rows are unchanged"
else
    fail "request rows changed"
    sed 's/^/        /' "$WORK/diff_rows"
fi
[[ $(psql_do -c 'SELECT count(*) FROM user_decrypt_share') == 1 ]] && pass "shares are unchanged" || fail "shares changed"

if apply_all 2>"$WORK/reapply_error"; then
    snapshot "$WORK/release_reapplied"
    if diff -u "$WORK/up_$LAST" "$WORK/release_reapplied" >"$WORK/diff_release_reapplied"; then
        pass "upgraded again, schema matches $LAST"
    else
        fail "schema after upgrading again differs from $LAST"
        sed 's/^/        /' "$WORK/diff_release_reapplied"
    fi
else
    fail "upgrading again failed: $(cat "$WORK/reapply_error")"
fi

echo "== A down that cannot revert stops the rollback and changes nothing"
# Two rows sharing an int_job_id: legal since 20260113011557, illegal before it.
reset_db
apply_all
psql_do -q <<'SQL'
INSERT INTO user_decrypt_req (ext_job_id, int_job_id, req, req_status, req_type) VALUES
    (gen_random_uuid(), '\x41', '{}', 'failure', 'user_decrypt'),
    (gen_random_uuid(), '\x41', '{}', 'failure', 'user_decrypt');
SQL
expect_failure "duplicated int_job_id: reports where it stopped" "current=20260113011557 target=$FIRST" "$MIGRATE" rollback "$FIRST"
snapshot "$WORK/stopped"
if diff -u "$WORK/up_20260113011557" "$WORK/stopped" >"$WORK/diff_stopped"; then
    pass "schema is exactly the one of the version it stopped at"
else
    fail "schema differs from the version it stopped at"
    sed 's/^/        /' "$WORK/diff_stopped"
fi
[[ $(psql_do -c 'SELECT count(*) FROM user_decrypt_req') == 2 ]] && pass "no row was deleted" || fail "rows were deleted"

reset_db
apply_all

# -------------------------------------------------------------------------
# 4. Guards
# -------------------------------------------------------------------------

echo "== Guards"
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

[[ $(db_version) == "$LAST" ]] && pass "no refused run changed the database" || fail "a refused run changed the database: version $(db_version)"

# A down that fails half way: rows using 'unified' block 20260519152353.
psql_do -q -c "INSERT INTO user_decrypt_req (ext_job_id, int_job_id, req, req_type) VALUES (gen_random_uuid(), '\x31', '{}', 'unified')"
expect_failure "failing down: reports where it stopped" "current=20260519152353 target=$FIRST" "$MIGRATE" rollback "$FIRST"
[[ $(db_version) == 20260519152353 ]] && pass "the failing down left its own migration applied" || fail "unexpected version after a failing down: $(db_version)"
snapshot "$WORK/unified_stopped"
if diff -u "$WORK/up_20260519152353" "$WORK/unified_stopped" >"$WORK/diff_unified_stopped"; then
    pass "the failing down changed nothing in the schema"
else
    fail "the failing down changed the schema"
    sed 's/^/        /' "$WORK/diff_unified_stopped"
fi

echo "== Forced rollback across 20260519152353"
# Still at 20260519152353, with the 'unified' row that blocked the rollback above.
psql_do -q <<'SQL'
UPDATE user_decrypt_req SET gw_reference_id = '\xb1' WHERE req_type = 'unified';
INSERT INTO user_decrypt_req (ext_job_id, int_job_id, req, req_type, gw_reference_id) VALUES
    (gen_random_uuid(), '\x32', '{}', 'user_decrypt', '\xb2');
INSERT INTO user_decrypt_share (gw_reference_id, share_index, share, kms_signature, extra_data) VALUES
    ('\xb1', 0, 's', 'sig', 'x'),
    ('\xb2', 0, 's', 'sig', 'x');
SQL
expect_failure "anything other than --force is refused" "usage:" "$MIGRATE" rollback 20260416120000 --yes
if output=$("$MIGRATE" rollback 20260416120000 --force 2>&1); then
    [[ $output == *"force=true"* ]] && pass "forced rollback to 20260416120000" || fail "forced rollback did not report force=true: $output"
else
    fail "forced rollback to 20260416120000: $output"
fi
snapshot "$WORK/forced"
if diff -u "$WORK/up_20260416120000" "$WORK/forced" >"$WORK/diff_forced"; then
    pass "schema matches 20260416120000"
else
    fail "schema differs from 20260416120000"
    sed 's/^/        /' "$WORK/diff_forced"
fi
[[ $(psql_do -c "SELECT string_agg(req_type::text, ',') FROM user_decrypt_req") == user_decrypt ]] \
    && pass "only the 'unified' request was deleted" || fail "unexpected requests left: $(psql_do -c 'SELECT req_type FROM user_decrypt_req')"
[[ $(psql_do -c "SELECT string_agg(encode(gw_reference_id, 'hex'), ',') FROM user_decrypt_share") == b2 ]] \
    && pass "only the shares of the deleted request were deleted" || fail "unexpected shares left"

echo
if [[ $FAILURES -eq 0 ]]; then
    echo "All checks passed."
else
    echo "$FAILURES check(s) failed."
    exit 1
fi
