#!/usr/bin/env bash
#
# ⚠️  WARNING: keep this a dependency-free bash script — do NOT port it to fhevm-npm. With
# --include-fhevm-npm it deletes fhevm-npm's own node_modules — the recovery tool for a broken tree
# (FHEVM_NPM_RULES 6.2.x) — so it must run when node tooling cannot. `make distclean` calls it
# (--dry-run, then --force).
#
# Deletes every node_modules under the sdk workspace, so the next `npm install` rebuilds the tree
# from scratch. fhevm-npm/ is spared by default: it is the orchestrator the uninstall flow itself
# runs (forge-dependency cleaning, and any fhevm-npm command afterwards), so it stays runnable.
#
# Usage: ./scripts/clean-node-modules.sh [--dry-run] [--force] [--include-package-lock]
#                                        [--include-fhevm-npm] [--help]
#
#   --dry-run                list what would go, delete nothing. Wins over --force.
#   --force                  skip the confirmation prompt. Required when stdin is not a terminal.
#   --include-package-lock   delete package-lock.json too, so npm re-resolves from the manifests
#                            instead of replaying recorded versions. Lockfiles are tracked in git.
#   --include-fhevm-npm      also delete fhevm-npm's node_modules (and, with the flag above, its
#                            lockfile) — the FHEVM_NPM_RULES 6.2.x broken-tree recovery.
#   -h, --help               print this header and exit.
#
# Example:
#   ./scripts/clean-node-modules.sh --dry-run     # see what it would remove
#   ./scripts/clean-node-modules.sh               # remove node_modules, after confirming
#
set -euo pipefail

# ------------------------------------------------------------------------------
# Scope. Resolved from this file, so the target never depends on the caller's
# cwd, and no working npm is needed to find it.
# ------------------------------------------------------------------------------

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname -- "$SCRIPT_DIR")"

# Subtrees not touched, relative to $ROOT. js-sdk sits under sdk/ but belongs to the OUTER fhevm
# workspace, so an install here would not bring its trees back. fhevm-npm is the orchestrator and
# stays runnable; --include-fhevm-npm lifts that one (broken-tree recovery, rules 6.2.x).
EXCLUDED_DIRS=("js-sdk" "fhevm-npm")

# ------------------------------------------------------------------------------
# Arguments.
# ------------------------------------------------------------------------------

USAGE="usage: ./scripts/clean-node-modules.sh [--dry-run] [--force] [--include-package-lock] [--include-fhevm-npm] [--help]"

DRY_RUN="false"
FORCE="false"
INCLUDE_LOCK="false"

for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN="true" ;;
        --force) FORCE="true" ;;
        --include-package-lock) INCLUDE_LOCK="true" ;;
        --include-fhevm-npm)
            for i in "${!EXCLUDED_DIRS[@]}"; do
                [ "${EXCLUDED_DIRS[$i]}" = "fhevm-npm" ] && unset 'EXCLUDED_DIRS[i]'
            done
            ;;
        -h | --help)
            sed -n '2,/^set -euo/p' "${BASH_SOURCE[0]}" | sed 's|^# \{0,1\}||; $d'
            exit 0
            ;;
        *)
            echo "Error: unknown argument '$arg'." >&2
            echo "$USAGE" >&2
            exit 1
            ;;
    esac
done

# ------------------------------------------------------------------------------
# Collect. Every node_modules, plus every lockfile when --include-package-lock.
# ------------------------------------------------------------------------------

# True when $1 is inside one of EXCLUDED_DIRS. Filtered after the search rather
# than pruned during it: `-prune` already stops at every node_modules, so nothing
# walks into a big tree either way.
is_excluded() {
    excluded_candidate="$1"
    for excluded_dir in ${EXCLUDED_DIRS[@]+"${EXCLUDED_DIRS[@]}"}; do
        case "$excluded_candidate" in
            "$ROOT/$excluded_dir" | "$ROOT/$excluded_dir"/*) return 0 ;;
        esac
    done
    return 1
}

TARGETS=()

while IFS= read -r path; do
    [ -n "$path" ] || continue
    is_excluded "$path" && continue
    TARGETS+=("$path")
done < <(find "$ROOT" -type d -name node_modules -prune -print | sort)

if [ "$INCLUDE_LOCK" = "true" ]; then
    while IFS= read -r path; do
        [ -n "$path" ] || continue
        is_excluded "$path" && continue
        TARGETS+=("$path")
    done < <(find "$ROOT" -type f -name package-lock.json -not -path '*/node_modules/*' | sort)
fi

if [ "${#TARGETS[@]}" -eq 0 ]; then
    echo "Nothing to remove under $ROOT."
    exit 0
fi

# ------------------------------------------------------------------------------
# Preview. What is about to go, and how big.
# ------------------------------------------------------------------------------

echo "The following will be removed from $ROOT:"
echo ""
for path in "${TARGETS[@]}"; do
    SIZE="$(du -sh "$path" 2> /dev/null | cut -f1 | tr -d '[:space:]')"
    printf '   %8s  %s\n' "${SIZE:-?}" "${path#"$ROOT"/}"
done
echo ""
echo "   ${#TARGETS[@]} item(s)."

if [ "${#EXCLUDED_DIRS[@]}" -gt 0 ]; then
    echo "   Excluded: ${EXCLUDED_DIRS[*]}"
fi

# ------------------------------------------------------------------------------
# Confirm. --dry-run stops here; otherwise ask, unless --force.
# ------------------------------------------------------------------------------

if [ "$DRY_RUN" = "true" ]; then
    echo ""
    echo "   Dry run — nothing was removed."
    exit 0
fi

if [ "$FORCE" != "true" ]; then
    if [ ! -t 0 ]; then
        echo "" >&2
        echo "Error: stdin is not a terminal, so there is nobody to confirm." >&2
        echo "       Re-run with --force to delete without asking." >&2
        exit 1
    fi
    echo ""
    printf 'Remove them? [y/N] '
    read -r REPLY
    case "$REPLY" in
        [yY] | [yY][eE][sS]) ;;
        *)
            echo "Aborted. Nothing was removed."
            exit 1
            ;;
    esac
fi

# ------------------------------------------------------------------------------
# Remove.
# ------------------------------------------------------------------------------

for path in "${TARGETS[@]}"; do
    rm -rf "$path"
done

echo "   🧹 removed ${#TARGETS[@]} item(s)."
if [ "$INCLUDE_LOCK" = "true" ]; then
    echo "   Next \`npm install\` re-resolves from the manifests — review the lockfile diff."
else
    echo "   Next \`npm install\` replays the existing package-lock.json."
fi
