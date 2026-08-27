#!/usr/bin/env bash
#
# Asserts that `forge lint` is the ONLY Solidity linter in the sdk workspace, by failing if solhint
# has reappeared in any form.
#
# Usage: ./scripts/check-lint-policy.sh
#   Run from a package root (npm sets that as the CWD), e.g.:
#     "check:lint-policy": "\"$(npm prefix)/scripts/check-lint-policy.sh\""
#
# SHARED SCRIPT, and unlike the other shared gates it checks the WHOLE workspace rather than the
# calling package: the ban is a workspace-level policy, so whichever member runs it, a reappearance
# anywhere is reported. That means it is idempotent across members — v12 and v13 both running it in
# `build` is redundant, not conflicting.
#
# WHY BAN IT. Two Solidity linters means two rule sets disagreeing about the same file, and rules
# silenced in one tool staying loud in the other. The workspace already resolved the same question
# for formatting — `forge fmt` won, `prettier-plugin-solidity` was removed outright rather than
# merely configured off (see FORGE_FMT_MIGRATION_PLAN.md §3.4). Linting follows the same shape:
# `[lint] exclude_lints` in each foundry.toml is the single place where a Solidity rule is turned
# off, and it can only stay authoritative if nothing else is also linting.
#
# WHAT IT CHECKS, in the order that matters:
#
#   1. Manifests — any occurrence in a package.json under sdk/. Deliberately blunt: ANY mention
#      fails, not just a dependency key. That keeps the check trivial and unambiguous, and is why
#      this script is named `check-lint-policy` rather than naming the banned tool — an npm script
#      called "check:no-solhint" would trip its own gate. Prose about the ban belongs in
#      ARCHITECTURE.md (I14), never in a manifest.
#   2. Config files — .solhint.json, .solhintrc*, solhint.config.*, .solhintignore.
#   3. An installed binary under any node_modules/.bin, which is the surface that makes it runnable
#      even with no manifest entry (a hoisted transitive install).
#   4. `solhint-disable` directives in Solidity we own. An inert annotation for a linter that never
#      runs is worse than noise: it tells the next reader a rule is being enforced somewhere.
#
# SCOPE OF 4 — three exclusions, each load-bearing:
#
#   - `pkg/src/contracts/` is EXCLUDED. Vendored sources are upstream's, stored byte-identical, and
#     RULES.md rule 6 / invariant I0 forbid editing them. Upstream lints with solhint and its
#     directives come along with the source. Flagging them would make this gate permanently red with
#     no legal fix. This is the same carve-out `[lint] ignore` already makes in foundry.toml.
#   - node_modules/ and dependencies/ are EXCLUDED: installed tarball fixtures and forge deps.
#   - Only workspace MEMBERS are scanned (host-contracts-cleartext/v*). Everything else under sdk/
#     — js-sdk/contracts/src/v0.*, the host-contracts-cleartext-v* payload snapshots — is a copy of
#     upstream Solidity held for reference, not code this workspace lints.
#
# Exits non-zero listing every hit, with the file:line of each.
set -euo pipefail

SDK_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Assembled at runtime so this script does not match itself if it is ever scanned.
TOOL="sol""hint"

if [ ! -d "$SDK_ROOT/host-contracts-cleartext" ]; then
    echo "Error: $SDK_ROOT does not look like the sdk workspace root." >&2
    exit 1
fi

HITS=""
add_hits() { [ -n "$1" ] && HITS="$HITS$1"$'\n'; return 0; }

# 1. Manifests.
MANIFESTS="$(
    find "$SDK_ROOT" -name package.json -not -path '*/node_modules/*' -not -path '*/tarballs/*' \
        -print0 2> /dev/null | xargs -0 grep -n "$TOOL" 2> /dev/null || true
)"
add_hits "$MANIFESTS"

# 2. Config files. Matched by NAME, so an empty or commented-out config still counts.
CONFIGS="$(
    find "$SDK_ROOT" \( -name ".${TOOL}*" -o -name "${TOOL}.config.*" \) \
        -not -path '*/node_modules/*' -not -path '*/tarballs/*' 2> /dev/null || true
)"
add_hits "$CONFIGS"

# 3. An installed binary anywhere in the workspace — runnable even with no manifest entry.
BINARIES="$(
    find "$SDK_ROOT" -path "*/node_modules/.bin/$TOOL" 2> /dev/null || true
)"
add_hits "$BINARIES"

# 4. Directives in Solidity this workspace owns. See SCOPE OF 4 above for every exclusion.
DIRECTIVES="$(
    find "$SDK_ROOT"/host-contracts-cleartext/v* -name '*.sol' \
        -not -path '*/node_modules/*' \
        -not -path '*/dependencies/*' \
        -not -path '*/pkg/src/contracts/*' \
        -not -path '*/out/*' \
        -not -path '*/cache/*' \
        -not -path '*/broadcast/*' \
        -print0 2> /dev/null | xargs -0 grep -n "$TOOL" 2> /dev/null || true
)"
add_hits "$DIRECTIVES"

HITS="$(echo "$HITS" | sed '/^$/d')"

if [ -n "$HITS" ]; then
    COUNT="$(echo "$HITS" | wc -l | tr -d '[:space:]')"
    echo "   ❌ $TOOL is banned in the sdk workspace — $COUNT occurrence(s):" >&2
    echo "" >&2
    echo "$HITS" | sed "s|^$SDK_ROOT/|      |" >&2
    cat >&2 << EOF

\`forge lint\` is the only Solidity linter here, and each foundry.toml's \`[lint] exclude_lints\` is
the single place a Solidity rule may be turned off (ARCHITECTURE.md I14).

  - a dependency or npm script  -> remove it; run \`npm run forge:lint\` instead
  - a config file               -> delete it; move any rule you still want into exclude_lints
  - a \`$TOOL-disable\` comment -> delete the comment; if the finding is real, silence the
                                  equivalent forge rule in [lint] exclude_lints instead
  - prose about this ban        -> ARCHITECTURE.md, never a package.json

Vendored sources under pkg/src/contracts are exempt and are not scanned: they are upstream's, and
rule 6 forbids editing them.
EOF
    exit 1
fi

echo "   ✅ forge lint is the only Solidity linter (no $TOOL in the workspace)"
