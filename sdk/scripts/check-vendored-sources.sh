#!/usr/bin/env bash
#
# RULES.md rule 6 gate: every file vendored into a package's payload must be identical to
# `forge fmt` of its counterpart upstream, at the commit the package itself declares.
#
# NOT a raw byte compare. Vendored sources are STORED forge-formatted, so the gate normalises the
# upstream side before comparing. The reason: upstream formats with prettier-plugin-solidity, this
# workspace formats with forge, and the two cannot be reconciled by configuration — 20 prettier
# configs and 13 forge configs were measured, neither converges (FORGE_FMT_MIGRATION_PLAN.md §1.1).
# Normalising upstream lets the whole tree be forge-formatted while still detecting real change.
#
# What this still catches: renamed identifiers, changed types, changed licences, inserted blank
# lines, added or removed code — anything `forge fmt` does not erase. What it no longer catches: a
# purely cosmetic upstream reflow. That is the deliberate trade (plan §3.5).
#
# Only the UPSTREAM side is normalised. The vendored file is compared exactly as stored, so if
# anyone reformats it by other means the gate fails rather than silently absorbing the difference.
#
# Because the comparison depends on `forge fmt` output, the forge version matters: run
# check-forge-version.sh first. A forge upgrade changes what this gate expects.
#
# Usage: ./scripts/check-vendored-sources.sh [--verbose]
#   --verbose   also list files that exist upstream but are not vendored here
#
# Run from the package root (npm sets that as the CWD), e.g. in package.json:
#   "check:vendored": "\"$(npm prefix)/scripts/check-vendored-sources.sh\""
#
# SHARED SCRIPT. Lives at sdk/scripts/ and is used by every workspace member that vendors sources.
# It is fully data-driven and knows nothing about any particular package: the upstream repository,
# tag, commit, source path and destination path all come from the payload manifest's
# `fhevm.vendoredFrom` (RULES.md rule 7). Keep it that way — anything package-specific belongs in
# that package's own scripts/.
#
# `sdk/scripts/` is NOT a workspace member and needs no package.json: shell scripts are invoked by
# path, so they need neither node module resolution nor bin symlinks. `$(npm prefix)` resolves to the
# workspace root from any member at any depth.
#
# PAYLOAD_DIR (env, default "pkg") is the harness-relative directory holding the PUBLISHED manifest.
# It is a parameter rather than a constant because the split is a convention, not a rule: the
# cleartext generations use ./pkg, js-sdk uses ./src.
#
# Subset semantics: a package may vendor only part of upstream, so this checks every file present
# HERE against upstream; an upstream-only file is reported as informational, never a failure —
# adopting one is a decision, not a side effect.
#
# Exits non-zero on drift, on a missing declaration, or if the upstream extraction produced nothing.
# That last check matters: an empty extraction makes a file-by-file compare report no differences,
# which looks exactly like success.
set -euo pipefail

PACKAGE_ROOT="$PWD"
PAYLOAD_DIR="${PAYLOAD_DIR:-pkg}"

VERBOSE=0
while [ $# -gt 0 ]; do
    case "$1" in
        --verbose)
            VERBOSE=1
            shift
            ;;
        -h | --help)
            # Print the whole header block rather than a hardcoded line range, which silently
            # truncates the help text whenever the header is edited.
            awk 'NR > 1 && /^#/ { sub(/^# ?/, ""); print; next } NR > 1 { exit }' "${BASH_SOURCE[0]}"
            exit 0
            ;;
        *)
            echo "Error: unknown argument '$1'. Try --help." >&2
            exit 1
            ;;
    esac
done

cd "$PACKAGE_ROOT"

# The comparison normalises upstream through `forge fmt`, so forge is a hard dependency now.
if ! command -v forge > /dev/null 2>&1; then
    echo "Error: forge is not installed or not on PATH — this gate normalises upstream with forge fmt." >&2
    exit 1
fi

PAYLOAD_MANIFEST="$PAYLOAD_DIR/package.json"
if [ ! -f "$PAYLOAD_MANIFEST" ]; then
    echo "Error: $PACKAGE_ROOT/$PAYLOAD_MANIFEST not found (override with PAYLOAD_DIR=...)." >&2
    exit 1
fi

read_declared() {
    node -p "const v=require('./$PAYLOAD_MANIFEST').fhevm?.vendoredFrom; if(!v?.$1) { console.error('Error: $PAYLOAD_MANIFEST has no fhevm.vendoredFrom.$1 (RULES.md rule 7).'); process.exit(1); } v.$1"
}

TAG="$(read_declared tag)"
COMMIT="$(read_declared commit)"
FROM="$(read_declared from)"
TO="$(read_declared to)"

# `vendoredFrom.to` is relative to the PUBLISHED package, not to the harness root we run from.
VENDORED_DIR="$PAYLOAD_DIR/$TO"
if [ ! -d "$VENDORED_DIR" ]; then
    echo "Error: $VENDORED_DIR not found — fhevm.vendoredFrom.to ('$TO') is payload-relative." >&2
    exit 1
fi

echo "🔎 rule 6: $TO must match $FROM at $TAG (${COMMIT:0:12})"

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [ -z "$REPO_ROOT" ]; then
    echo "   ⏭  not inside a git repo — cannot verify (this is expected for a standalone checkout)."
    exit 0
fi

if ! git -C "$REPO_ROOT" cat-file -e "${COMMIT}^{commit}" 2>/dev/null; then
    echo "   ⏭  commit $COMMIT not present in $REPO_ROOT — fetch the history to verify."
    exit 0
fi

UPSTREAM="$(mktemp -d)"
cleanup() { rm -rf "$UPSTREAM"; }
trap cleanup EXIT

# `git archive` resolves pathspecs relative to the CWD, so run it from the repo root explicitly.
git -C "$REPO_ROOT" archive "$COMMIT" "$FROM" | tar -x -C "$UPSTREAM"

UPSTREAM_DIR="$UPSTREAM/$FROM"
UPSTREAM_COUNT="$(find "$UPSTREAM_DIR" -name '*.sol' 2>/dev/null | wc -l | tr -d ' ')"
if [ "$UPSTREAM_COUNT" -eq 0 ]; then
    echo "   ❌ extraction produced no .sol files — the check would have passed vacuously." >&2
    exit 1
fi

VENDORED_COUNT=0
DRIFT=0
while IFS= read -r rel; do
    VENDORED_COUNT=$((VENDORED_COUNT + 1))
    if [ ! -f "$UPSTREAM_DIR/$rel" ]; then
        echo "   ❌ $rel — vendored here but absent upstream at $TAG"
        DRIFT=$((DRIFT + 1))
    elif ! forge fmt --raw - < "$UPSTREAM_DIR/$rel" 2> /dev/null | cmp -s - "$VENDORED_DIR/$rel"; then
        echo "   ❌ $rel — differs from forge fmt(upstream)"
        DRIFT=$((DRIFT + 1))
    fi
done < <(cd "$VENDORED_DIR" && find . -name '*.sol' | sed 's|^\./||' | sort)

if [ "$VERBOSE" -eq 1 ]; then
    while IFS= read -r rel; do
        [ -f "$VENDORED_DIR/$rel" ] || echo "   ℹ  $rel — upstream only, not vendored (adopting it is a decision)"
    done < <(cd "$UPSTREAM_DIR" && find . -name '*.sol' | sed 's|^\./||' | sort)
fi

if [ "$VENDORED_COUNT" -eq 0 ]; then
    echo "   ❌ no vendored .sol files found under $VENDORED_DIR — the check would have passed vacuously." >&2
    exit 1
fi

if [ "$DRIFT" -ne 0 ]; then
    echo ""
    echo "$DRIFT of $VENDORED_COUNT vendored files drifted from $TAG."
    echo "Either re-sync them from upstream, or update fhevm.vendoredFrom to the commit you actually"
    echo "vendored (RULES.md rules 6 and 7). Do not edit vendored sources in place."
    exit 1
fi

echo "   ✅ $VENDORED_COUNT vendored files identical to upstream ($UPSTREAM_COUNT upstream files scanned)"
