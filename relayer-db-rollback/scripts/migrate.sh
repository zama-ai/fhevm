#!/bin/sh
#
# Rolls the relayer database back to an earlier migration version.
#
#   migrate.sh check                      validate the migration files (no database)
#   migrate.sh info                       validate everything, print the database state
#   migrate.sh rollback <target-version>  revert every migration above <target-version>
#   migrate.sh rollback <target-version> --force
#                                         same, and let the down files that refuse to
#                                         run delete the data that blocks them
#
# This tool never applies migrations: that stays with relayer-migrate.
#
# It is all or nothing. Before touching the database it requires that
#   1. every migration in this image has a down file,
#   2. every migration applied to the database exists in this image,
#   3. every applied migration has the same content as the copy in this image.
# If any of these does not hold it stops and changes nothing.
#
# --force does not relax any of these checks. It only reaches the down files
# that would otherwise stop on data they cannot convert, through the
# `relayer_db_rollback.force` session setting, and makes them delete that data
# instead. It is meant for test environments.
#
# Environment:
#   DATABASE_URL    postgres connection string (required for info and rollback)
#   MAX_ATTEMPTS    connection attempts, 2 seconds apart (default: 20)
#   MIGRATIONS_DIR  migration files (default: ../migrations next to this script)

set -eu

# =========================================================================
# Configuration
# =========================================================================

SCRIPT_DIR=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
MIGRATIONS_DIR=${MIGRATIONS_DIR:-$(dirname "$SCRIPT_DIR")/migrations}
DATABASE_URL=${DATABASE_URL:-}
MAX_ATTEMPTS=${MAX_ATTEMPTS:-20}

ACTION=${1:-}
TARGET_VERSION=${2:-}
FORCE=false

# =========================================================================
# Helpers
# =========================================================================

usage() {
    echo "usage: $0 check | info | rollback <target-version> [--force]" >&2
}

# Print a failure line and stop. $1 is the action, $2 the reason.
fail() {
    echo "event=rollback_result action=$1 status=failed error=\"$2\"" >&2
    exit 1
}

# Collapse a file to one line, without the characters that break key="value".
compact() {
    awk '
        {
            gsub(/["\\]/, "")
            printf "%s%s", separator, $0
            separator = " "
        }
        END { print "" }
    ' "$1"
}

# Versions sqlx reports as applied, one per line, ascending.
installed_versions() {
    awk -F '[/ ]' '$2 == "installed" { print $1 }' "$1" | sort -n
}

# Highest applied version, or 0 when nothing is applied.
current_version() {
    installed_versions "$1" | awk 'END { print NR ? $1 : "0" }'
}

# =========================================================================
# Check 1: every migration has a down file
# =========================================================================
#
# `sqlx migrate revert` silently skips an applied migration that has no down
# file and still reports success. That must never happen here, so the whole
# directory is validated, not only the range being reverted.

check_migration_files() {
    action=$1
    problems=$(mktemp)
    migration_count=0

    if [ ! -d "$MIGRATIONS_DIR" ]; then
        rm -f "$problems"
        fail "$action" "migrations directory not found: $MIGRATIONS_DIR"
    fi

    for file in "$MIGRATIONS_DIR"/*; do
        [ -e "$file" ] || continue
        name=${file##*/}

        case "$name" in
            *.up.sql)
                base=${name%.up.sql}
                migration_count=$((migration_count + 1))
                down="$MIGRATIONS_DIR/$base.down.sql"
                if [ ! -f "$down" ]; then
                    echo "missing down file for $name" >>"$problems"
                elif ! grep -v -e '^[[:space:]]*--' -e '^[[:space:]]*$' "$down" >/dev/null; then
                    echo "down file has no statement: $base.down.sql" >>"$problems"
                fi
                ;;
            *.down.sql)
                base=${name%.down.sql}
                if [ ! -f "$MIGRATIONS_DIR/$base.up.sql" ]; then
                    echo "down file without a migration: $name" >>"$problems"
                fi
                ;;
            *)
                echo "unexpected file (expected <version>_<name>.up.sql or .down.sql): $name" >>"$problems"
                continue
                ;;
        esac

        version=${base%%_*}
        case "$version" in
            '' | *[!0-9]*)
                echo "file name does not start with a numeric version: $name" >>"$problems"
                ;;
        esac
    done

    if [ "$migration_count" -eq 0 ]; then
        echo "no migration found in $MIGRATIONS_DIR" >>"$problems"
    fi

    # Two migrations with the same version would make the target ambiguous.
    for file in "$MIGRATIONS_DIR"/*.up.sql; do
        [ -e "$file" ] || continue
        name=${file##*/}
        echo "${name%%_*}"
    done | sort | uniq -d | while read -r duplicate; do
        echo "version used by more than one migration: $duplicate" >>"$problems"
    done

    if [ -s "$problems" ]; then
        error=$(compact "$problems")
        rm -f "$problems"
        fail "$action" "migration files are incomplete, refusing to run: $error"
    fi

    rm -f "$problems"
    MIGRATION_COUNT=$migration_count
}

# =========================================================================
# Database access
# =========================================================================

# Runs `sqlx migrate info`, retrying while the database is not reachable.
# On failure the reason is left in $command_output (sqlx prints its errors on
# stdout, so both streams are captured).
load_migration_info() {
    output_file=$1
    attempt=1

    while [ "$attempt" -le "$MAX_ATTEMPTS" ]; do
        if sqlx migrate info --no-dotenv \
            --source "$MIGRATIONS_DIR" >"$output_file" 2>&1; then
            return 0
        fi

        cp "$output_file" "$command_output"

        if [ "$attempt" -lt "$MAX_ATTEMPTS" ]; then
            sleep 2
        fi

        attempt=$((attempt + 1))
    done

    return 1
}

# =========================================================================
# Check 2 and 3: the image and the database describe the same history
# =========================================================================

check_in_sync() {
    action=$1
    info_file=$2

    # Check 3: same version, different content. The copy in this image is not
    # the migration that was applied, so its down file cannot be trusted.
    if grep 'different checksum' "$info_file" >/dev/null; then
        mismatched=$(awk -F '[/ ]' '/different checksum/ { printf "%s%s", separator, $1; separator = "," }' "$info_file")
        fail "$action" "out of sync: the migrations applied to the database differ from the copies in this image (versions: $mismatched). Copy them again from the fhevm release the database was migrated with."
    fi

    # Check 2: the database is ahead of this image. A dry run performs sqlx's
    # own validation of the applied history without changing anything.
    if ! sqlx migrate revert --no-dotenv --dry-run \
        --source "$MIGRATIONS_DIR" \
        --target-version "$(current_version "$info_file")" >"$command_output" 2>&1; then
        error=$(compact "$command_output")
        case "$error" in
            *'previously applied but is missing'*)
                fail "$action" "out of sync: the database has migrations this image does not know, so it cannot roll them back. Copy the missing migrations from the fhevm release the database was migrated with, write their down files and rebuild. sqlx said: $error"
                ;;
            *)
                fail "$action" "could not validate the applied migrations: $error"
                ;;
        esac
    fi
}

# =========================================================================
# Argument validation
# =========================================================================

case "$ACTION" in
    check)
        [ "$#" -eq 1 ] || { usage; exit 1; }
        ;;
    info)
        [ "$#" -eq 1 ] || { usage; exit 1; }
        ;;
    rollback)
        case "$#" in
            2) ;;
            3)
                [ "$3" = --force ] || { usage; exit 1; }
                FORCE=true
                ;;
            *)
                usage
                exit 1
                ;;
        esac
        case "$TARGET_VERSION" in
            '' | *[!0-9]*)
                fail rollback "target version must contain only digits"
                ;;
        esac
        if [ "$TARGET_VERSION" -eq 0 ]; then
            fail rollback "target version 0 is not allowed: it would drop the whole schema"
        fi
        ;;
    *)
        usage
        exit 1
        ;;
esac

# =========================================================================
# check
# =========================================================================

check_migration_files "$ACTION"

if [ "$ACTION" = check ]; then
    echo "event=rollback_result action=check status=success migrations=$MIGRATION_COUNT"
    exit 0
fi

# =========================================================================
# Prerequisites for the database actions
# =========================================================================

if ! command -v sqlx >/dev/null 2>&1; then
    fail "$ACTION" "sqlx-cli is not installed"
fi

if [ -z "$DATABASE_URL" ]; then
    fail "$ACTION" "DATABASE_URL is required"
fi
export DATABASE_URL

case "$MAX_ATTEMPTS" in
    '' | *[!0-9]* | 0)
        fail "$ACTION" "MAX_ATTEMPTS must be a positive integer"
        ;;
esac

before_info=$(mktemp)
after_info=$(mktemp)
command_output=$(mktemp)
trap 'rm -f "$before_info" "$after_info" "$command_output"' EXIT HUP INT TERM

if ! load_migration_info "$before_info"; then
    fail "$ACTION" "could not read the migration state after $MAX_ATTEMPTS attempts: $(compact "$command_output")"
fi

check_in_sync "$ACTION" "$before_info"

CURRENT_VERSION=$(current_version "$before_info")

# =========================================================================
# info
# =========================================================================

if [ "$ACTION" = info ]; then
    cat "$before_info"
    applied=$(installed_versions "$before_info" | awk 'END { print NR }')
    echo "event=rollback_result action=info status=success current=$CURRENT_VERSION applied=$applied known=$MIGRATION_COUNT in_sync=true"
    exit 0
fi

# =========================================================================
# rollback
# =========================================================================

if ! ls "$MIGRATIONS_DIR/${TARGET_VERSION}"_*.up.sql >/dev/null 2>&1; then
    fail rollback "target version $TARGET_VERSION is not a migration known to this image"
fi

if ! installed_versions "$before_info" | grep -x "$TARGET_VERSION" >/dev/null; then
    fail rollback "target version $TARGET_VERSION is not applied to this database (current version is $CURRENT_VERSION)"
fi

# Applied versions above the target, newest first: the order they are reverted in.
PLAN=$(installed_versions "$before_info" | awk -v target="$TARGET_VERSION" '$1 + 0 > target + 0' | sort -rn | tr '\n' ',' | sed 's/,$//')

if [ -z "$PLAN" ]; then
    echo "event=rollback_result action=rollback status=noop current=$CURRENT_VERSION target=$TARGET_VERSION"
    exit 0
fi

echo "event=rollback_plan action=rollback current=$CURRENT_VERSION target=$TARGET_VERSION force=$FORCE revert=$PLAN"

# The down files read this session setting; see the header of each one for what
# it makes them do. It is set for the revert only, never for the checks above.
if [ "$FORCE" = true ]; then
    PGOPTIONS="${PGOPTIONS:+$PGOPTIONS }-c relayer_db_rollback.force=on"
    export PGOPTIONS
fi

if ! sqlx migrate revert --no-dotenv \
    --source "$MIGRATIONS_DIR" \
    --target-version "$TARGET_VERSION" >"$command_output" 2>&1; then
    error=$(compact "$command_output")

    # Each down runs in its own transaction, so a failure can leave the
    # database between the two versions. Say exactly where it stopped.
    stopped_at=unknown
    if load_migration_info "$after_info"; then
        stopped_at=$(current_version "$after_info")
    fi

    echo "event=rollback_result action=rollback status=failed previous=$CURRENT_VERSION current=$stopped_at target=$TARGET_VERSION force=$FORCE error=\"$error\"" >&2
    exit 1
fi

cat "$command_output"

if ! load_migration_info "$after_info"; then
    fail rollback "the revert ran but the resulting state could not be read after $MAX_ATTEMPTS attempts: $(compact "$command_output")"
fi

FINAL_VERSION=$(current_version "$after_info")

# sqlx reported success. Do not take its word for it: confirm the database is
# exactly at the target.
if [ "$FINAL_VERSION" != "$TARGET_VERSION" ]; then
    echo "event=rollback_result action=rollback status=failed previous=$CURRENT_VERSION current=$FINAL_VERSION target=$TARGET_VERSION error=\"sqlx reported success but the database is not at the target version\"" >&2
    exit 1
fi

echo "event=rollback_result action=rollback status=success previous=$CURRENT_VERSION current=$FINAL_VERSION target=$TARGET_VERSION force=$FORCE reverted=$PLAN"
