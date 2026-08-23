#!/usr/bin/env bash
#
# RULES.md rule 6 gate: every file vendored into pkg/src/contracts must be byte-for-byte identical to
# its counterpart in host-contracts at the commit declared in pkg/package.json -> fhevm.vendoredFrom.
#
# Usage: ./scripts/check-vendored-sources.sh [--verbose]
#   --verbose   also list files that exist upstream but are not vendored here
#
# Subset semantics: cleartext may vendor only part of host-contracts, so this checks every file present
# HERE against upstream; an upstream-only file is reported as informational, never a failure — adopting
# one is a decision, not a side effect.
#
# On the 0.13 line the two sets happen to be equal (21 files either side), so the subset path is not
# exercised today. It will be: contracts/bridge/ exists on main and not on any 0.11/0.12/0.13 tag, so a
# 0.14 sync is where an upstream-only file first shows up and has to be judged rather than copied.
#
# Exits non-zero on drift, on a missing declaration, or if the upstream extraction produced nothing.
# That last check matters: an empty extraction makes `diff -r` report no differences, which looks
# exactly like success.
set -euo pipefail

PACKAGE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

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

if [ ! -f pkg/package.json ]; then
    echo "Error: $PACKAGE_ROOT/pkg/package.json not found." >&2
    exit 1
fi

read_declared() {
    node -p "const v=require('./pkg/package.json').fhevm?.vendoredFrom; if(!v?.$1) { console.error('Error: pkg/package.json has no fhevm.vendoredFrom.$1 (RULES.md rule 7).'); process.exit(1); } v.$1"
}

TAG="$(read_declared tag)"
COMMIT="$(read_declared commit)"
FROM="$(read_declared from)"
TO="$(read_declared to)"

# `vendoredFrom.to` is relative to the PUBLISHED package (pkg/), not to the harness root we run from.
VENDORED_DIR="pkg/$TO"
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
    echo "   ⏭  commit $COMMIT not present in $REPO_ROOT — fetch the fhevm history to verify."
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
    elif ! cmp -s "$VENDORED_DIR/$rel" "$UPSTREAM_DIR/$rel"; then
        echo "   ❌ $rel — differs from upstream"
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