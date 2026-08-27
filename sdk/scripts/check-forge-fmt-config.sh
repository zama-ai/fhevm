#!/usr/bin/env bash
#
# Asserts that this package's `forge fmt` settings come from the shared sdk/foundry.base.toml, and
# that nothing local silently overrides them.
#
# Usage: ./scripts/check-fmt-config.sh
#   Run from the package root (npm sets that as the CWD), e.g.:
#     "check:fmt-config": "\"$(npm prefix)/scripts/check-fmt-config.sh\""
#
# Two failure modes it catches, both silent otherwise:
#
#   1. A package whose foundry.toml has no `extends` — it would format with forge's defaults, which
#      happen to match today, so the drift would only appear when a shared value is changed.
#   2. A package that sets a [fmt] key locally — `extends` MERGES and local wins, so a stray key
#      quietly opts that package out of the shared style with no error anywhere.
#
# It compares EFFECTIVE values (`forge config`) against the shared file rather than parsing the
# local foundry.toml, so it catches an override however it was introduced — local key, profile,
# or FOUNDRY_FMT_* in the environment.
#
# `ignore` is exempt: it is per-package by nature (each package ignores its own fixture paths) and
# is deliberately not set in the shared file.
#
# Why this matters more than style consistency: vendored sources under pkg/src/contracts are STORED
# forge-formatted and the rule 6 gate compares them against `forge fmt`(upstream). A package that
# silently formats differently would fail that gate for reasons that look unrelated.
set -euo pipefail

PACKAGE_ROOT="$PWD"
SHARED="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/foundry.base.toml"

if [ ! -f "$SHARED" ]; then
    echo "Error: shared config not found at $SHARED." >&2
    exit 1
fi

if [ ! -f "$PACKAGE_ROOT/foundry.toml" ]; then
    echo "Error: $PACKAGE_ROOT/foundry.toml not found." >&2
    exit 1
fi

if ! grep -qE '^[[:space:]]*extends[[:space:]]*=' "$PACKAGE_ROOT/foundry.toml"; then
    cat >&2 << EOF
Error: $PACKAGE_ROOT/foundry.toml declares no \`extends\`.

Add this to its [profile.default] so it inherits the shared forge fmt settings:

    extends = "\$(realpath --relative-to . "$SHARED" 2> /dev/null || echo ../../foundry.base.toml)"
EOF
    exit 1
fi

# Effective [fmt] values for this package, as forge actually resolves them.
EFFECTIVE="$(forge config 2> /dev/null | sed -n '/^\[fmt\]/,/^\[/p')"

DRIFT=0
while IFS= read -r line; do
    key="${line%%=*}"
    key="$(echo "$key" | tr -d '[:space:]')"
    [ -n "$key" ] || continue
    [ "$key" = "ignore" ] && continue

    want="$(echo "${line#*=}" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
    got="$(echo "$EFFECTIVE" | grep -E "^[[:space:]]*$key[[:space:]]*=" | head -1 | sed 's/^[^=]*=[[:space:]]*//;s/[[:space:]]*$//')"

    if [ -z "$got" ]; then
        echo "   ❌ $key — shared file sets it, forge reports nothing"
        DRIFT=$((DRIFT + 1))
    elif [ "$got" != "$want" ]; then
        echo "   ❌ $key — shared says $want, this package resolves to $got"
        DRIFT=$((DRIFT + 1))
    fi
done < <(sed -n '/^\[fmt\]/,/^\[/p' "$SHARED" | grep -E '^[a-z_]+[[:space:]]*=')

if [ "$DRIFT" -ne 0 ]; then
    echo ""
    echo "$DRIFT forge fmt setting(s) deviate from $SHARED."
    echo "Remove the local override, or — if the deviation is deliberate — state why in foundry.toml"
    echo "and remember that vendored sources are stored forge-formatted against the shared style."
    exit 1
fi

echo "   ✅ forge fmt settings match $(basename "$SHARED")"
