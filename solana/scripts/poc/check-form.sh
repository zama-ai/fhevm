#!/usr/bin/env bash
# Deterministic "form" gate for the Solana PoC — part of the L0 oracle.
#
# It enforces NO-REGRESSION from the feature/solana baseline; it does NOT require
# the target state. Target-state checks (test-shim removal, keccak handles, etc.)
# are per-item oracles, because the baseline branch intentionally still violates
# them — they are the work #1494 exists to do. The baseline branch, unchanged,
# must pass this gate with zero failures.
#
# Backpressure encoded here (the parts that can be a grep, not a judgement):
#   1. Known oversized files are grandfathered but may not GROW; no NEW file may
#      exceed the cap. Pressure is always toward splitting, never toward sprawl.
#   2. No shortcut / glue / over-defensive markers introduced in changed Rust.
#   3. No moving the goalposts: changes may not edit the tests or oracle scripts.
#   4. No silent new dependencies.
# The judgement parts ("is this slop / does it mirror EVM semantics") live in the
# adversarial verifier, not here.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/../.." # -> solana/
BASE="${FORM_BASE:-origin/feature/solana}"
CAP="${FORM_CAP:-500}"
ALLOW="scripts/poc/form-allow.txt"
fail=0
violate() {
	echo "FORM FAIL: $1"
	fail=1
}

# 1. File-size: allowlisted files may not grow; everything else must stay <= CAP.
#    (bash 3.2-safe: no associative arrays — macOS ships bash 3.2, CI is bash 4+.)
allow_base() { awk -v p="$1" '$1==p {print $2}' "$ALLOW" 2>/dev/null; }
while IFS= read -r f; do
	lines=$(wc -l <"$f")
	base=$(allow_base "$f")
	if [[ -n "$base" ]]; then
		((lines > base)) && violate "$f grew to $lines (baseline $base); split toward <= $CAP, do not grow"
	else
		((lines > CAP)) && violate "$f is $lines lines (> $CAP); split it (allowlist additions need explicit sign-off)"
	fi
done < <(find programs crates -name '*.rs' -not -path '*/target/*' | sort)

# 2. Changed-file hygiene (diff vs baseline): no shortcut/glue markers in new code.
#    (rust paths have no spaces, so unquoted word-splitting of $changed is safe.)
changed=$(git diff --name-only "$BASE" -- 'programs/*' 'crates/*' 2>/dev/null | grep '\.rs$' || true)
if [[ -n "$changed" ]]; then
	if git diff "$BASE" -- $changed | grep -E '^\+' | grep -vE '^\+\+\+' |
		grep -qE 'TODO|FIXME|HACK|XXX|dbg!\(|\.unwrap\(\)|panic!\(|#\[allow\('; then
		violate "shortcut/glue/over-defensive marker added in changed Rust (TODO/HACK/unwrap/panic/dbg/#[allow])"
	fi
fi

# 3. No moving the goalposts: do not MODIFY or DELETE the acceptance tests or the
#    oracle scripts (adding new tests is fine, so additions are not flagged).
if git diff --name-only --diff-filter=MD "$BASE" | grep -qE '^solana/(runtime-tests/|scripts/poc/(check-form|run-oracle))'; then
	violate "changeset modifies/deletes tests or oracle scripts — fix the code, not the goalposts"
fi

# 4. No silent new dependencies — except crates pre-approved (signed off) in dep-allow.txt.
DEP_ALLOW="scripts/poc/dep-allow.txt"
dep_allowed() { grep -qE "^$1( |\$)" "$DEP_ALLOW" 2>/dev/null; }
added_deps=$(git diff "$BASE" -- '*/Cargo.toml' 'Cargo.toml' | grep -E '^\+' | grep -vE '^\+\+\+' |
	grep -E '^\+[a-zA-Z0-9_-]+ *= *.*(version|git|path|"[0-9])' | sed -E 's/^\+([a-zA-Z0-9_-]+).*/\1/' | sort -u || true)
if [[ -n "$added_deps" ]]; then
	while IFS= read -r dep; do
		[[ -z "$dep" ]] && continue
		dep_allowed "$dep" || violate "new dependency '$dep' is not pre-approved in $DEP_ALLOW — needs explicit sign-off"
	done <<<"$added_deps"
fi

((fail)) && {
	echo "check-form: FAIL"
	exit 1
}
echo "check-form: OK"
