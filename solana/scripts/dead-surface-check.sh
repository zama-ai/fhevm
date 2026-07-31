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
    hits=$( (grep -rn --include='*.rs' --include='*.ts' -F "::${variant}" "${RUST_ROOTS[@]}" sdk/js-sdk/src/solana kms-connector/crates 2>/dev/null || true) \
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
  emits=$( (grep -rn --include='*.rs' -E "emit_cpi!|emit!" solana/programs || true) \
    | grep -cF "$event" || true)
  constructions=$( (grep -rn --include='*.rs' -F "${event} {" solana/programs || true) \
    | (grep -v "pub struct ${event} {" || true) | wc -l | tr -d ' ')
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
    "$@" "${RUST_ROOTS[@]}" solana/docs sdk/js-sdk/src/solana solana/scripts 2>/dev/null || true) \
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
alias_hits=$( (grep -rn --include='*.rs' --include='*.ts' --include='*.md' -iE 'lookup[- ]table' \
  "${RUST_ROOTS[@]}" solana/docs sdk/js-sdk/src/solana 2>/dev/null || true) \
  | (grep -viE 'address[- ]?lookup[- ]?table|ALT' || true) \
  | (grep -viE 'settle|batch|v0|authority|packet|addresses|slot' || true) \
  | (grep -v 'solana/docs/GLOSSARY.md' || true) )
if [ -n "$alias_hits" ]; then
  echo "REJECTED ALIAS (lookup table — the interning structure is the dictionary):"
  echo "$alias_hits"
  fail=1
fi

if [ "$fail" -ne 0 ]; then
  echo "dead-surface-check: FINDINGS ABOVE"
  exit 1
fi
echo "dead-surface-check: clean"
