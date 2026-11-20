#!/usr/bin/env bash
#
# End-to-end test of the rollback tool against a throwaway Postgres.
#
# Requirements: Docker, psql, and sqlx-cli 0.7.2 on the PATH
#   cargo install sqlx-cli --version 0.7.2 --locked --no-default-features --features rustls,postgres
#
# What it proves:
#   1. Round trip. Every migration is applied with the coprocessor's own files
#      (coprocessor/fhevm-engine/db-migration/migrations, the way the
#      db-migration image applies them), then every migration above the
#      baseline is reverted one by one with migrate.sh. After each revert the
#      schema must be identical to the one recorded before that migration.
#   2. Full rollback to the baseline with data, on a database stamped with a
#      v0.15 stack version, in a single command: the stamp must become the
#      requested release. Followed by a re-upgrade.
#   3. The down that refuses to run on data it cannot convert stops the
#      rollback, changes nothing, and deletes that data only with --force.
#   4. Guards. The tool refuses to run when the image and the database are out
#      of sync, when the target is below the baseline, and reports where it
#      stopped when a down fails.
#
# Usage: tests/roundtrip.sh

set -euo pipefail

HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
COMPONENT_DIR=$(dirname "$HERE")
REPO_ROOT=$(dirname "$COMPONENT_DIR")
COPROCESSOR_MIGRATIONS="$REPO_ROOT/coprocessor/fhevm-engine/db-migration/migrations"
ROLLBACK_MIGRATIONS="$COMPONENT_DIR/migrations"
MIGRATE="$COMPONENT_DIR/scripts/migrate.sh"

BASELINE=20260704130000 # must match BASELINE_VERSION in scripts/migrate.sh

CONTAINER=coprocessor-db-rollback-test
PORT=${PORT:-55440}
export DATABASE_URL="postgres://postgres:postgres@localhost:$PORT/coprocessor"
export MAX_ATTEMPTS=5

WORK=$(mktemp -d)
FAILURES=0

cleanup() {
    docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
    rm -rf "${WORK:?}"
}
trap cleanup EXIT

pass() { echo "  ok    $1"; }
fail() { echo "  FAIL  $1"; FAILURES=$((FAILURES + 1)); }

psql_do() {
    docker exec -i "$CONTAINER" psql -U postgres -d coprocessor -v ON_ERROR_STOP=1 -At "$@"
}

snapshot() {
    psql_do -f - <"$HERE/schema_snapshot.sql" >"$1"
}

# Recreates the database, so each scenario starts from nothing.
reset_db() {
    docker exec "$CONTAINER" psql -U postgres -q -c 'DROP DATABASE IF EXISTS coprocessor WITH (FORCE)' -c 'CREATE DATABASE coprocessor' >/dev/null 2>&1
}

# Applies the coprocessor's own migrations up to a version, as the db-migration
# image does.
apply_up_to() {
    sqlx migrate run --source "$COPROCESSOR_MIGRATIONS" --target-version "$1" >/dev/null
}

apply_all() {
    sqlx migrate run --source "$COPROCESSOR_MIGRATIONS" >/dev/null
}

db_version() {
    psql_do -c 'SELECT coalesce(max(version), 0) FROM _sqlx_migrations'
}

# Makes the database look like one that runs v0.15: the stamp a cutover writes.
# The migration itself only seeds 'v0.14'.
stamp_live() {
    psql_do -q -c "UPDATE versioning SET stack_version = 'v0.15.1', updated_at = NOW() WHERE singleton"
}

stack_version() {
    psql_do -c 'SELECT stack_version FROM versioning WHERE singleton'
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
command -v psql >/dev/null || { echo "psql is required: migrate.sh reads versioning.stack_version with it" >&2; exit 1; }

echo "== The rollback copies are identical to the coprocessor's migrations"
for file in "$COPROCESSOR_MIGRATIONS"/*.sql; do
    name=$(basename "$file" .sql)
    if cmp -s "$file" "$ROLLBACK_MIGRATIONS/$name.up.sql"; then
        pass "$name"
    else
        fail "$name: missing from or different in $ROLLBACK_MIGRATIONS"
    fi
done

VERSIONS=()
for file in "$COPROCESSOR_MIGRATIONS"/*.sql; do
    name=$(basename "$file")
    VERSIONS+=("${name%%_*}")
done
LAST=${VERSIONS[${#VERSIONS[@]} - 1]}

# The versions above the baseline, ascending: the ones with a real down.
ABOVE=()
for version in "${VERSIONS[@]}"; do
    (( version > BASELINE )) && ABOVE+=("$version")
done

docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
docker run -d --name "$CONTAINER" -e POSTGRES_PASSWORD=postgres -p "$PORT:5432" postgres:17-alpine >/dev/null
until docker exec "$CONTAINER" pg_isready -U postgres >/dev/null 2>&1; do sleep 1; done
sleep 1

# -------------------------------------------------------------------------
# 1. Round trip, one migration at a time
# -------------------------------------------------------------------------

echo "== Round trip: schema after each down equals the schema before its up"
reset_db
apply_up_to "$BASELINE"
snapshot "$WORK/up_$BASELINE"
for version in "${ABOVE[@]}"; do
    apply_up_to "$version"
    snapshot "$WORK/up_$version"
done
stamp_live

for ((i = ${#ABOVE[@]} - 1; i >= 0; i--)); do
    reverted=${ABOVE[i]}
    if (( i > 0 )); then target=${ABOVE[i - 1]}; else target=$BASELINE; fi
    if ! output=$("$MIGRATE" rollback "$target" --stack-version v0.15.1 2>&1); then
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
# 2. Full rollback to the baseline with data, then upgrade again
# -------------------------------------------------------------------------

echo "== Full rollback to the baseline with data, in one command"
reset_db
apply_all
psql_do -q <<'SQL'
INSERT INTO upgrade_state (stack_role, state, status, host_chain_id) VALUES
    ('BCS', 'LIVE', 'completed', 11155111),
    ('GCS', 'LIVE', 'completed', 11155111);
INSERT INTO chain_recovery_halts (source, stack_role, chain_id, block_number, reason) VALUES
    ('host', 'BCS', 11155111, 42, 'test');
SQL
stamp_live

if output=$("$MIGRATE" rollback "$BASELINE" --stack-version 0.14.8 2>&1); then
    [[ $output == *"stack_version=v0.15.1->0.14.8"* && $output == *"status=success"*"stack_version=0.14.8"* ]] \
        && pass "rollback $LAST -> $BASELINE, stamped 0.14.8" || fail "rollback $LAST -> $BASELINE: stamp not reported: $output"
else
    fail "rollback $LAST -> $BASELINE: $output"
fi
[[ $(stack_version) == 0.14.8 ]] && pass "versioning.stack_version is 0.14.8: a v0.14.8 binary is not retired" || fail "versioning.stack_version is $(stack_version)"

snapshot "$WORK/full_down"
if diff -u "$WORK/up_$BASELINE" "$WORK/full_down" >"$WORK/diff_full"; then
    pass "schema matches the baseline"
else
    fail "schema differs from the baseline"
    sed 's/^/        /' "$WORK/diff_full"
fi

[[ $(db_version) == "$BASELINE" ]] && pass "migration history is at $BASELINE" || fail "migration history is at $(db_version)"

[[ $(psql_do -c "SELECT string_agg(stack_role, ',' ORDER BY stack_role) FROM upgrade_state") == BCS,GCS ]] \
    && pass "one upgrade_state row per stack role survived" || fail "upgrade_state rows: $(psql_do -c 'SELECT count(*) FROM upgrade_state')"

echo "== Upgrade again after the rollback"
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
# 3. Downs that refuse on data, and --force
# -------------------------------------------------------------------------

echo "== A down that cannot revert stops the rollback and changes nothing"
# Two upgrade_state rows for one stack role: legal since 20260725120000,
# illegal before it.
reset_db
apply_all
psql_do -q <<'SQL'
INSERT INTO upgrade_state (stack_role, state, status, host_chain_id) VALUES
    ('BCS', 'LIVE', 'completed', 11155111),
    ('BCS', 'LIVE', 'completed', 80002),
    ('GCS', 'LIVE', 'completed', 11155111);
SQL
stamp_live
expect_failure "per-chain upgrade_state rows: reports where it stopped" "current=20260725120000 target=$BASELINE" "$MIGRATE" rollback "$BASELINE" --stack-version 0.14.8
[[ $(stack_version) == v0.15.1 ]] && pass "the stamp is untouched after a failed rollback" || fail "the stamp changed after a failed rollback: $(stack_version)"
snapshot "$WORK/stopped"
if diff -u "$WORK/up_20260725120000" "$WORK/stopped" >"$WORK/diff_stopped"; then
    pass "schema is exactly the one of the version it stopped at"
else
    fail "schema differs from the version it stopped at"
    sed 's/^/        /' "$WORK/diff_stopped"
fi
[[ $(psql_do -c 'SELECT count(*) FROM upgrade_state') == 3 ]] && pass "no row was deleted" || fail "rows were deleted"

echo "== Forced rollback across 20260725120000"
expect_failure "anything other than --force is refused" "usage:" "$MIGRATE" rollback "$BASELINE" --yes
# Stop right below 20260725120000 first: host_chain_id still exists there, so the
# rows the forced down kept can be checked. 20260708120000 drops the column.
if output=$("$MIGRATE" rollback 20260721120000 --force --stack-version 0.14.8 2>&1); then
    [[ $output == *"force=true"* ]] && pass "forced rollback to 20260721120000" || fail "forced rollback did not report force=true: $output"
else
    fail "forced rollback to 20260721120000: $output"
fi
snapshot "$WORK/forced"
if diff -u "$WORK/up_20260721120000" "$WORK/forced" >"$WORK/diff_forced"; then
    pass "schema matches 20260721120000"
else
    fail "schema differs from 20260721120000"
    sed 's/^/        /' "$WORK/diff_forced"
fi
[[ $(psql_do -c "SELECT string_agg(stack_role || ':' || host_chain_id, ',' ORDER BY stack_role) FROM upgrade_state") == BCS:80002,GCS:11155111 ]] \
    && pass "only the row with the lowest host_chain_id survived per stack role" || fail "unexpected upgrade_state rows: $(psql_do -c 'SELECT stack_role, host_chain_id FROM upgrade_state')"
if output=$("$MIGRATE" rollback "$BASELINE" --stack-version 0.14.8 2>&1); then
    pass "the rest of the rollback to $BASELINE needs no --force"
else
    fail "rollback 20260721120000 -> $BASELINE: $output"
fi
[[ $(psql_do -c "SELECT string_agg(stack_role, ',' ORDER BY stack_role) FROM upgrade_state") == BCS,GCS ]] \
    && pass "one upgrade_state row per stack role at the baseline" || fail "unexpected upgrade_state rows at the baseline"

# -------------------------------------------------------------------------
# 4. Guards
# -------------------------------------------------------------------------

echo "== Guards"
reset_db
apply_all
stamp_live

[[ $("$MIGRATE" info 2>&1 | tail -1) == *"in_sync=true stack_version=v0.15.1"* ]] && pass "info reports in sync and the stamp" || fail "info does not report in sync and the stamp: $("$MIGRATE" info 2>&1 | tail -1)"

expect_failure "stamped database: rollback without --stack-version is refused" "pass --stack-version" "$MIGRATE" rollback "$BASELINE"

# The stamp lookup fails (psql breaks on the existence query): the rollback must
# stop before any down, not treat the database as unstamped.
broken="$WORK/broken-psql"
mkdir -p "$broken"
real_psql=$(command -v psql)
cat >"$broken/psql" <<STUB
#!/usr/bin/env bash
for arg in "\$@"; do
    case "\$arg" in *to_regclass*) echo "simulated psql failure" >&2; exit 1 ;; esac
done
exec "$real_psql" "\$@"
STUB
chmod +x "$broken/psql"
expect_failure "failed stamp lookup: rollback is refused" "could not read versioning.stack_version" \
    env PATH="$broken:$PATH" "$MIGRATE" rollback "$BASELINE" --stack-version 0.14.8
[[ $(db_version) == "$LAST" ]] && pass "no down ran after the failed stamp lookup" || fail "a down ran after the failed stamp lookup: version $(db_version)"
expect_failure "failed stamp lookup: info is refused" "could not read versioning.stack_version" \
    env PATH="$broken:$PATH" "$MIGRATE" info
expect_failure "malformed stack version is refused" "stack version must look like" "$MIGRATE" rollback "$BASELINE" --stack-version latest

[[ $("$MIGRATE" rollback "$LAST" 2>&1) == *"status=noop"* ]] && pass "rollback to the current version is a no-op" || fail "rollback to the current version is not a no-op"

expect_failure "target 0 is refused" "target version 0 is not allowed" "$MIGRATE" rollback 0
expect_failure "target below the baseline is refused" "below the baseline" "$MIGRATE" rollback 20260603120000
expect_failure "unknown target is refused" "not a migration known to this image" "$MIGRATE" rollback 20990101000000

# The database is ahead of the image: the image lacks the newest migration.
behind="$WORK/behind"
mkdir "$behind"
cp "$ROLLBACK_MIGRATIONS"/* "$behind/"
rm "${behind:?}/${LAST:?}"_*
expect_failure "image behind the database: info is refused" "out of sync: the database has migrations this image does not know" \
    env MIGRATIONS_DIR="$behind" "$MIGRATE" info
expect_failure "image behind the database: rollback is refused" "out of sync: the database has migrations this image does not know" \
    env MIGRATIONS_DIR="$behind" "$MIGRATE" rollback "$BASELINE"

# Same version, different content.
edited="$WORK/edited"
mkdir "$edited"
cp "$ROLLBACK_MIGRATIONS"/* "$edited/"
echo "-- edited" >>"$(ls "$edited/$LAST"_*.up.sql)"
expect_failure "edited migration: rollback is refused" "out of sync: the migrations applied to the database differ" \
    env MIGRATIONS_DIR="$edited" "$MIGRATE" rollback "$BASELINE"

# A migration without a down file.
no_down="$WORK/no_down"
mkdir "$no_down"
cp "$ROLLBACK_MIGRATIONS"/* "$no_down/"
rm "${no_down:?}/${LAST:?}"_*.down.sql
expect_failure "missing down file: rollback is refused" "missing down file" \
    env MIGRATIONS_DIR="$no_down" "$MIGRATE" rollback "$BASELINE"

[[ $(db_version) == "$LAST" ]] && pass "no refused run changed the database" || fail "a refused run changed the database: version $(db_version)"

echo
if [[ $FAILURES -eq 0 ]]; then
    echo "All checks passed."
else
    echo "$FAILURES check(s) failed."
    exit 1
fi
