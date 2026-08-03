#!/usr/bin/env bash
# Dead-surface and glossary checks for the Solana workstream (fhevm-internal#1859 §7 tooling).
#
# Checks, each mechanically re-runnable so "nothing on this list can regenerate":
#   1. Error variants nobody references: a variant declared in a program/crate error enum with
#      zero `::Variant` references anywhere outside its declaring file is dead surface.
#   2. `#[event]` structs never emitted: an Anchor event that is never constructed cannot be
#      observed and only bloats the IDL.
#   3. Rejected glossary aliases: vocabulary the normative GLOSSARY.md replaced must not
#      reappear in the Solana workstream sources.
#   4. Retrofit zero-fills without a justification at the site (§8 taxonomy).
#   5. Reused-value retrofits whose written justification must stay in place (§8 taxonomy).
#
# Exits non-zero if any check finds something, printing every finding.

set -euo pipefail
cd "$(dirname "$0")/../.."

fail=0

# Everything the Solana workstream owns. GLOSSARY.md is excluded from the alias sweep because
# its "Replaces" column intentionally quotes the old names.
RUST_ROOTS=(
  solana/programs
  solana/crates
  solana/runtime-tests
  solana/scripts/e2e/live-client
  solana-proof-service
  coprocessor/fhevm-engine/host-listener/src
)

# TypeScript the Solana workstream owns. The demo dapp holds the vault module (moved out of the
# SDK) and test-suite/fhevm holds the seeder and the scenarios; both were unswept while the vault
# lived under sdk/js-sdk/src/solana. Installed graphs and build outputs are excluded — a vendored
# dependency's vocabulary is not ours to police.
TS_ROOTS=(
  sdk/js-sdk/src/solana
  solana/demo-dapp/src
  solana/demo-dapp/demoServerPlugin.ts
  test-suite/fhevm
)
EXCLUDES=(
  --exclude-dir=node_modules
  --exclude-dir=dist
  --exclude-dir=_esm
  --exclude-dir=_cjs
  --exclude-dir=_types
)

echo "== 1. error variants with zero references =="
error_files=$(grep -rln '#\[error_code\]' solana/programs solana/crates --include='*.rs')
for file in $error_files; do
  # Variant names: capitalized identifiers directly followed by `,` at enum-body indentation.
  variants=$(grep -oE '^    ([A-Z][A-Za-z0-9]+),' "$file" | sed -E 's/^    ([A-Za-z0-9]+),/\1/')
  for variant in $variants; do
    # Deliberate tombstones keep their positional error code; their doc comment says so.
    context=$(grep -B3 -E "^    ${variant},\$" "$file" || true)
    if echo "$context" | grep -qiE 'ordinals stay stable|code stability|stability\)|retired|legacy \(unused'; then
      continue
    fi
    # Boundary-anchored, not a substring match: `::InvalidKmsContext` must not be kept alive by
    # `::InvalidKmsContextId`, and pairs like that exist today (`InvalidInputHandle` sits inside
    # four longer variants, so the substring form counted 8 references where 2 are real).
    hits=$( (grep -rn --include='*.rs' --include='*.ts' "${EXCLUDES[@]}" \
      -E "::${variant}([^A-Za-z0-9_]|$)" \
      "${RUST_ROOTS[@]}" "${TS_ROOTS[@]}" kms-connector/crates 2>/dev/null || true) \
      | (grep -v -F "$file" || true) | wc -l | tr -d ' ')
    if [ "$hits" -eq 0 ]; then
      echo "DEAD ERROR VARIANT: ${variant} (declared in ${file}, zero references elsewhere)"
      fail=1
    fi
  done
done

echo "== 2. #[event] structs never emitted =="
event_names=$(grep -rn --include='*.rs' -A2 '#\[event\]' solana/programs \
  | grep -oE 'pub struct [A-Za-z0-9]+' | awk '{print $3}' | sort -u)
for event in $event_names; do
  # Boundary-anchored for the same reason as check 1: a longer event name starting with this one
  # would otherwise vouch for it.
  emits=$( (grep -rn --include='*.rs' -E "emit_cpi!|emit!" solana/programs || true) \
    | grep -cE "\b${event}\b" || true)
  constructions=$( (grep -rn --include='*.rs' -E "\b${event} \{" solana/programs || true) \
    | (grep -v -E "pub struct ${event} \{" || true) | wc -l | tr -d ' ')
  if [ "$emits" -eq 0 ] && [ "$constructions" -eq 0 ]; then
    echo "NEVER-EMITTED EVENT: ${event}"
    fail=1
  fi
done

echo "== 3. rejected glossary aliases =="
# token -> extra grep args; every hit outside the allowed exceptions is a violation.
check_alias() {
  local label="$1"; shift
  local hits
  hits=$( (grep -rn --include='*.rs' --include='*.ts' --include='*.md' --include='*.py' --include='*.sh' \
    "${EXCLUDES[@]}" "$@" "${RUST_ROOTS[@]}" "${TS_ROOTS[@]}" solana/docs solana/scripts 2>/dev/null || true) \
    | (grep -v 'solana/docs/GLOSSARY.md' || true) \
    | (grep -v 'solana/docs/DESIGN_DECISIONS.md' || true) \
    | (grep -v 'dead-surface-check.sh' || true) )
  if [ -n "$hits" ]; then
    echo "REJECTED ALIAS (${label}):"
    echo "$hits"
    fail=1
  fi
}

check_alias 'fhe_eval — renamed to fhe_execute' -E '\bfhe_eval\b|\bFheEval\b'
check_alias 'born-public — renamed to created-public' -iE 'born[-_ ]public'
# `value_key` as an identifier was renamed to encrypted-value-ID vocabulary. The signed/wire
# spellings stay: the sha256 tag string "...value-key-v1" (preimage bytes), the v3 JSON key
# `aclValueKey`, and the connector's matching `acl_value_key` field.
check_alias 'value_key identifier — renamed to encrypted_value_id' \
  -E '\bvalue_key\b' --exclude-dir=utils
# "lookup table" is banned for the interning dictionary; the Solana Address Lookup Table keeps
# its proper name (matched case-insensitively here, so allow the capitalized ALT spelling and
# `addressLookupTable`/`address lookup table` phrasing).
alias_hits=$( (grep -rn --include='*.rs' --include='*.ts' --include='*.md' "${EXCLUDES[@]}" \
  -iE 'lookup[- ]table' "${RUST_ROOTS[@]}" "${TS_ROOTS[@]}" solana/docs 2>/dev/null || true) \
  | (grep -viE 'address[- ]?lookup[- ]?table|ALT' || true) \
  | (grep -viE 'settle|batch|v0|authority|packet|addresses|slot' || true) \
  | (grep -v 'solana/docs/GLOSSARY.md' || true) )
if [ -n "$alias_hits" ]; then
  echo "REJECTED ALIAS (lookup table — the interning structure is the dictionary):"
  echo "$alias_hits"
  fail=1
fi

echo "== 4. retrofit zero-fills without a justification at the site =="
# fhevm-internal#1859 §8 taxonomy: where Solana meets EVM-shaped infrastructure, a constant
# stuffed into a field with no Solana meaning is allowed ONLY with a written justification at the
# site. These are the files that zero-fill such a field today; each hit must have a reason within
# the twelve lines above it (a doc comment on the enclosing constructor counts), so a new
# zero-fill cannot land silently.
#
# Only files that actually zero-fill belong here. Two entries used to sit in this list asserting
# nothing: relayer's `transaction_calldata.rs` and the SDK's `userDecrypt.ts` carry a
# *reused-value* retrofit, not a zero-fill, so this check never matched a line in either. They
# moved to check 5, which pins their written justification instead.
SENTINEL_FILES=(
  coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  kms-connector/crates/gw-listener/src/core/publish.rs
  relayer/src/core/event.rs
  sdk/js-sdk/src/solana/deSigncrypt.ts
)
JUSTIFICATION='placeholder|not persisted|unused on the Solana|oblivious|ignores the EVM|reuses the EVM|discard|EVM-shaped columns|EVM-only'
for file in "${SENTINEL_FILES[@]}"; do
  [ -f "$file" ] || { echo "MISSING SENTINEL FILE: ${file} (update SENTINEL_FILES)"; fail=1; continue; }
  # Test modules re-state EVM shapes as fixtures; only production code is swept.
  test_line=$( (grep -n -m1 -E '#\[cfg\(test\)\]|describe\(' "$file" || true) | cut -d: -f1)
  sentinels=$( (grep -nE 'Address::ZERO|FixedBytes::ZERO|\[0u8; ?(20|32)\]|0x0{40}' "$file" || true) )
  production_sentinels=0
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    line=${hit%%:*}
    if [ -n "$test_line" ] && [ "$line" -ge "$test_line" ]; then continue; fi
    production_sentinels=$((production_sentinels + 1))
    start=$((line > 12 ? line - 12 : 1))
    # Comment markers stripped and the window joined into one line: justifications are prose and
    # wrap, so a phrase straddles two comment lines with a `///` in between (the same trap the
    # tombstone check above hit).
    context=$(sed -n "${start},${line}p" "$file" \
      | sed -E -e 's#^[[:space:]]*//+!?[[:space:]]*##' -e 's#^[[:space:]]*\*[[:space:]]*##' \
      | tr '\n' ' ' | tr -s ' ')
    if ! echo "$context" | grep -qiE "$JUSTIFICATION"; then
      echo "UNJUSTIFIED RETROFIT SENTINEL: ${file}:${line}: ${hit#*:}"
      fail=1
    fi
  done <<< "$sentinels"
  # A listed file with no zero-fill left is a stale entry that reads as coverage. Fail so the
  # list cannot rot into a set of files nobody is checking.
  if [ "$production_sentinels" -eq 0 ]; then
    echo "STALE SENTINEL ENTRY: ${file} has no production zero-fill left (drop it from SENTINEL_FILES)"
    fail=1
  fi
done

echo "== 5. reused-value retrofits whose justification must stay written =="
# The other §8 shape: a field with no Solana meaning filled with a *reused* real value rather than
# a zero. There is no constant to grep for, so the check pins the explanation — delete the prose
# and this fails, because the prose is the only way a reader learns the field is inert.
RETROFIT_JUSTIFICATIONS=(
  "sdk/js-sdk/src/solana/actions/userDecrypt.ts|reuse the derived"
  "relayer/src/gateway/arbitrum/transaction_calldata.rs|no Solana meaning"
)
for entry in "${RETROFIT_JUSTIFICATIONS[@]}"; do
  file=${entry%%|*}
  phrase=${entry#*|}
  [ -f "$file" ] || { echo "MISSING RETROFIT FILE: ${file} (update RETROFIT_JUSTIFICATIONS)"; fail=1; continue; }
  if ! grep -qF "$phrase" "$file"; then
    echo "MISSING RETROFIT JUSTIFICATION: ${file} no longer says \"${phrase}\""
    fail=1
  fi
done

if [ "$fail" -ne 0 ]; then
  echo "dead-surface-check: FINDINGS ABOVE"
  exit 1
fi
echo "dead-surface-check: clean"
