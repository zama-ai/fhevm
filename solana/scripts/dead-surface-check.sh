#!/usr/bin/env bash
# Dead-surface and glossary checks for the Solana workstream (fhevm-internal#1859 §7 tooling).
#
# Checks, each mechanically re-runnable so "nothing on this list can regenerate":
#   1. Error variants nobody references: a variant declared in a program/crate error enum with
#      zero `::Variant` references in production code is dead surface.
#   2. `#[event]` structs never emitted: an Anchor event that nothing hands to an emission cannot be
#      observed and only bloats the IDL. Constructing one is not enough. Both emission paths in the
#      tree count — the `emit!`/`emit_cpi!` macros the app programs use, and zama-host's shared
#      `emit_event_cpi` helper.
#   3. Rejected glossary aliases: vocabulary the normative GLOSSARY.md replaced must not reappear
#      in the Solana workstream sources. Every allowed survival is an explicit, reasoned exception.
#   4. Retrofit sentinels without a justification at the site (§8 taxonomy): a constant stuffed
#      into an EVM-shaped field is allowed only with a written reason.
#   5. Reused-value retrofits whose written justification must stay in place (§8 taxonomy).
#   6. Exported symbols with no audience: a `pub fn` / `export function` with no production caller
#      outside its own file must either name its audience in a `Public API surface:` line or go.
#      One that nothing references at all — not a test, not its own module — is simply dead.
#   7. Every tree swept above is a CI trigger for this script: a check that does not run on a change
#      to the tree it protects is the same hole as a check that does not fire.
#
# References are counted against a PRODUCTION INDEX (see `build_index`): test files and
# `#[cfg(test)]` / `describe(` regions are dropped, and comments are stripped, so neither a
# fixture nor a doc comment can vouch for a symbol nothing calls.
#
# Exits non-zero if any check finds something, printing every finding.
#
# Self-test: `dead-surface-check.sh --self-test` re-runs each check against fixtures that are
# supposed to fail it, and fails if a check stays silent. That is what keeps the greps honest —
# every one of these checks has been silently vacuous at least once.

set -euo pipefail
# Resolved before the cd, because after it a relative $0 no longer names this file: invoked from
# `solana/` as `scripts/dead-surface-check.sh`, the cd lands at the repo root where that path does
# not exist. The self-test re-runs this script per fixture, and bash's "no such file" is exit 127 —
# which `expect_fires` reads as "the check did not report", so a cwd typo looked like 25 vacuous
# checks. Every self-invocation below uses this absolute path.
SELF="$(cd "$(dirname "$0")" && pwd)/$(basename "$0")"
cd "$(dirname "$0")/../.."

fail=0
SELF_TEST=0
[ "${1:-}" = "--self-test" ] && SELF_TEST=1

# The self-test asserts one fixture per check, and each assertion runs a fresh child. Those
# children used to re-run all seven checks over the whole tree, so twenty single-check
# assertions cost twenty full sweeps. A child may be told which check its fixture violates.
# Unset — every real invocation, including CI's — runs all of them.
run_check() { [ -z "${DEAD_SURFACE_ONLY_CHECK:-}" ] || [ "${DEAD_SURFACE_ONLY_CHECK}" = "$1" ]; }

# Everything the Solana workstream owns. GLOSSARY.md is excluded from the alias sweep because its
# "Replaces" column intentionally quotes the old names, and DESIGN_DECISIONS.md because it is a
# historical record: a decision written under the old vocabulary keeps it, so the alias patterns
# would fire on every entry. Decisions there are only ever appended, but their prose does get
# reworded, and that is not something this sweep can police — a rename inside those bodies is
# review's job. It has already gone wrong once: a blanket frame -> batch pass overwrote "CPI frame".
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

# Shell and workflow files the workstream owns. `fhe_eval` survived in a solana-tests.yml comment
# and `supersede` in full-vertical.sh precisely because the sweep never looked here.
SCRIPT_ROOTS=(
  solana/scripts
  solana/docs
  .github/workflows/solana-tests.yml
  .github/workflows/solana-e2e.yml
)

# Two scopes, because three of the retired words are ordinary technical English outside the FHE
# core. `plan` names a docker-compose stack plan all over test-suite/fhevm, `pool` names a Postgres
# connection pool all over the listener and the proof service, and `namespace` names a Kubernetes
# namespace in the workflows. Sweeping those trees for those words would produce either noise or an
# exception list so long it stops meaning anything, so they are swept in CORE only — the sources
# that speak the Solana FHE vocabulary natively, where the word can only be the retired sense.
# Every other alias is swept across everything the workstream owns.
CORE_ROOTS=(
  solana/programs
  solana/crates
  solana/runtime-tests
  solana/scripts
  solana/docs
  sdk/js-sdk/src/solana
  solana/demo-dapp/src
)

# A third, narrower scope for one word: "batch". An fhe_execute invocation is an `execution` (see
# GLOSSARY), but the confidential batcher's `Batch` account really is a batch — a settlement round
# of independent deposits — and the listener batches RPC calls and DB writes. Sweeping the word
# across everything would flag those legitimate uses forever, so it is swept exactly where "batch"
# can only mean the retired name for one execution: the host, the SDK, the token program, and the
# live client. The batcher's own program, its mollusk test, the vault dapp and the vault docs speak
# the settlement sense and are outside this list on purpose.
FHE_ROOTS=(
  solana/crates
  solana/programs/zama-host
  solana/programs/confidential-token
  solana/scripts/e2e/live-client
  solana/runtime-tests/tests/host_mollusk.rs
  solana/runtime-tests/tests/token_mollusk.rs
  solana/runtime-tests/tests/operator_conformance.rs
  solana/runtime-tests/tests/operator_mollusk_conformance.rs
  solana/runtime-tests/tests/execution_contracts.rs
)
ALL_ROOTS=("${RUST_ROOTS[@]}" "${TS_ROOTS[@]}" "${SCRIPT_ROOTS[@]}")

# A root that no longer exists makes every sweep over it silently narrower, which is the failure
# this script exists to prevent. `batch_contracts.rs` sat in FHE_ROOTS after being renamed to
# `execution_contracts.rs` and the FHE sweep quietly stopped reading it: grep's error went to
# /dev/null and the `|| true` swallowed the status. Named roots are asserted instead.
for root in "${ALL_ROOTS[@]}" "${CORE_ROOTS[@]}" "${FHE_ROOTS[@]}" kms-connector/crates; do
  [ -e "$root" ] || { echo "dead-surface-check: swept root does not exist: $root" >&2; exit 2; }
done

# The scan surfaces of checks 4 and 5, declared here rather than inside the checks that read them
# first. Check 7 audits every one of these paths against the CI trigger filter, so a list that only
# exists while its own check runs makes check 7 depend on which other checks ran — under
# `DEAD_SURFACE_ONLY_CHECK=7` the arrays were unbound and check 7 audited nothing.
#
# The listener's `src/` is mostly the EVM ingest path, which zero-fills EVM fields for reasons that
# have nothing to do with Solana (a synthesized `TrivialEncrypt` log, for one). Only the Solana
# adapter files there are ours to hold to the §8 rule.
SENTINEL_ROOTS=(
  solana/programs
  solana/crates
  solana/scripts/e2e/live-client
  solana-proof-service
  sdk/js-sdk/src/solana
  solana/demo-dapp/src
  test-suite/fhevm
  coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  coprocessor/fhevm-engine/host-listener/src/solana_reconstruct.rs
  coprocessor/fhevm-engine/host-listener/src/solana_grpc_listener.rs
  coprocessor/fhevm-engine/host-listener/src/solana_grpc_source.rs
)
# Files expected to still carry at least one retrofit sentinel. This is a STALENESS guard, not the
# scan surface: check 4 finds retrofits anywhere in the owned trees, and this list fails loudly when
# a file that used to carry one no longer does, so the list cannot rot into fake coverage.
SENTINEL_FILES=(
  coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  relayer/src/core/event.rs
)
# Check 5's pinned justification prose, as `file|phrase`.
RETROFIT_JUSTIFICATIONS=(
  "sdk/js-sdk/src/solana/actions/userDecrypt.ts|reuse the derived"
  "relayer/src/gateway/arbitrum/transaction_calldata.rs|no Solana meaning"
)
# The self-test cannot violate check 5 by planting a file — it would have to delete prose from a real
# one — so it adds an entry through the environment instead. Same code path, both arms.
[ -n "${DEAD_SURFACE_EXTRA_RETROFIT:-}" ] && \
  RETROFIT_JUSTIFICATIONS+=("${DEAD_SURFACE_EXTRA_RETROFIT}")

# Same assertion the swept roots get above, for the same reason: these four listener files are named
# individually and read behind `2>/dev/null || true`, so a rename or a typo in any of them would
# silently narrow checks 4 and 5 instead of failing. `solana_adapter.rs` alone contributes most of
# check 4's hits, so its disappearance would look exactly like a clean run.
for root in "${SENTINEL_ROOTS[@]}"; do
  [ -e "$root" ] || { echo "dead-surface-check: sentinel root does not exist: $root" >&2; exit 2; }
done

# `target/` holds generated crates (mime_guess ships a word list containing half the dictionary);
# node_modules and build outputs are other people's vocabulary.
EXCLUDES=(
  --exclude-dir=node_modules
  --exclude-dir=dist
  --exclude-dir=_esm
  --exclude-dir=_cjs
  --exclude-dir=_types
  --exclude-dir=target
)

# ---------------------------------------------------------------------------
# Production index: `path:line:code` for every owned Rust/TS source, with test files, test
# regions, and comments removed. Reference counting reads this, never the raw tree.
# ---------------------------------------------------------------------------
# `--self-test` fixtures have to sit inside the swept trees, because that is the only place the
# sweeps look — a scratch directory outside the repo would never be read. `solana/crates` is in
# RUST_ROOTS, CORE_ROOTS and FHE_ROOTS at once, so one file there is reachable from every scope.
SELFTEST_FIXTURES=(
  solana/crates/zama-fhe/.dead-surface-selftest.md
  solana/crates/zama-fhe/src/dead_surface_selftest.rs
  solana/crates/zama-fhe/src/dead_surface_selftest_caller.rs
  solana/programs/zama-host/src/dead_surface_selftest.rs
  solana/demo-dapp/src/deadSurfaceSelftest.ts
)
# A fixture left behind by an interrupted `--self-test` is indistinguishable from a real violation,
# so every later run reports a bogus finding — observed once as a phantom `app_account` hit printed
# alongside "self-test clean". The child runs spawned by the self-test export
# DEAD_SURFACE_SELFTEST_ACTIVE, so this only trips on a genuinely stray file.
if [ -z "${DEAD_SURFACE_SELFTEST_ACTIVE:-}" ]; then
  for fixture in "${SELFTEST_FIXTURES[@]}"; do
    if [ -e "$fixture" ]; then
      echo "dead-surface-check: leftover self-test fixture ${fixture} — delete it and rerun" >&2
      exit 2
    fi
  done
fi

INDEX=$(mktemp)
ALIAS_LABELS=$(mktemp)
trap 'rm -f "$INDEX" "$ALIAS_LABELS"' EXIT

build_index() {
  local files
  files=$(grep -rl '' --include='*.rs' --include='*.ts' --include='*.tsx' "${EXCLUDES[@]}" \
    "${RUST_ROOTS[@]}" "${TS_ROOTS[@]}" kms-connector/crates 2>/dev/null | sort -u)
  local file
  for file in $files; do
    case "$file" in
      # Whole-file test surfaces: a fixture is not a caller.
      */tests/*|*/tests.rs|*.test.ts|*.test.tsx|*/test_utils/*|*/testing/*) continue ;;
    esac
    awk -v path="$file" '
      BEGIN {
        quote = sprintf("%c", 39)
        char_literal = quote "(\\\\.|[^" quote "\\\\])" quote
      }
      # Skip the item a `#[cfg(test)]` attaches to — not the rest of the file. This used to `exit` at
      # the first one, on the assumption that a test region is always the trailing `mod tests`. It is
      # not: a file can carry a `#[cfg(test)] use ...;` near the top, and then everything below it
      # vanished from the index. `solana_reconstruct.rs` has one at line 19 of 1216, so 98% of the
      # listener file that actually calls `zama_host::decode::decode_event_cpi` was invisible, and the
      # export was duly reported as having no production caller. Ten of the 80 files holding a
      # `#[cfg(test)]` were losing more than a quarter of their indexed production lines. Under-counting
      # references only ever invents findings, so the failure mode was a gate loud enough to get
      # switched off.
      # `#[test]` / `#[tokio::test]` / `#[rstest]` are matched too, for the same reason: a test written
      # at file scope rather than inside a `#[cfg(test)] mod tests` was indexed as production, so a
      # symbol only its assertions touched counted as having a production caller. That direction hides
      # findings rather than inventing them, which is the worse of the two.
      /^[[:space:]]*#\[cfg\(test\)\]/ || /^[[:space:]]*#\[(([a-z_]+::)?test|rstest)\]/ {
        skip_depth = 0
        skip_started = 0
        while ((getline skip_line) > 0) {
          # Braces inside a comment, a string, a char literal or a raw string are not structure. A
          # `}` in a comment used to end the skip early and index test code as production; an unclosed
          # `{` in a string ran the skip to EOF and dropped every production line below it. Both are
          # the same bug this rewrite exists to fix, one level down, so strip the quotable forms before
          # counting rather than counting what happens to be on the line.
          code = skip_line
          sub(/\/\/.*$/, "", code)
          gsub(/r#*"[^"]*"#*/, "STR", code)
          gsub(/"(\\.|[^"\\])*"/, "STR", code)
          # A char literal, matched through a dynamic regex so the awk program needs no literal
          # apostrophe — it lives inside a single-quoted shell string.
          gsub(char_literal, "CH", code)
          opens = gsub(/\{/, "&", code)
          closes = gsub(/\}/, "&", code)
          if (opens > 0) skip_started = 1
          skip_depth += opens - closes
          # A braced item (`mod tests { ... }`) ends when its braces balance; an unbraced one
          # (`use foo::bar;`) ends at its semicolon. Further attribute lines carry neither and are
          # simply consumed on the way to the item they decorate.
          if (skip_started) {
            if (skip_depth <= 0) break
          } else if (code ~ /;[[:space:]]*$/) {
            break
          }
        }
        next
      }
      # TS keeps the hard stop: `describe(` blocks live in `*.test.ts`, which is dropped whole above,
      # so a describe reached here is a test region in a file that has no production code below it.
      /^[[:space:]]*describe\(/ { exit }
      {
        line = $0
        sub(/^[[:space:]]*\/\/.*$/, "", line)       # whole-line // comment
        sub(/^[[:space:]]*\*.*$/, "", line)         # continuation of a /* */ block
        sub(/^[[:space:]]*\/\*.*$/, "", line)       # /* opener
        gsub(/[^:]\/\/[^\/].*$/, "", line)          # trailing comment, sparing https://
        if (line ~ /[^[:space:]]/) print path ":" NR ":" line
      }
    ' "$file"
  done
}
# Checks 1, 2 and 6 are the only ones that read the index, so a child running just one of the
# others does not pay for building it.
if run_check 1 || run_check 2 || run_check 6; then
  build_index > "$INDEX"
fi

# Counts production references to a boundary-anchored pattern, ignoring the declaring file.
index_refs() {
  local pattern="$1" declaring_file="$2"
  (grep -E "$pattern" "$INDEX" || true) | (grep -v -F "${declaring_file}:" || true) | wc -l | tr -d ' '
}

if run_check 1; then
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
      hits=$(index_refs "::${variant}([^A-Za-z0-9_]|$)" "$file")
      if [ "$hits" -eq 0 ]; then
        echo "DEAD ERROR VARIANT: ${variant} (declared in ${file}, zero production references)"
        fail=1
      fi
    done
  done
fi

if run_check 2; then
  echo "== 2. #[event] structs never emitted =="
  event_names=$(grep -rn --include='*.rs' -A2 '#\[event\]' solana/programs \
    | grep -oE 'pub struct [A-Za-z0-9]+' | awk '{print $3}' | sort -u)
  for event in $event_names; do
    # Emission is the only liveness proof. Construction alone is NOT: a struct built into a local and
    # never handed to an emission is exactly the dead shape this check exists to catch, and the old
    # `emits == 0 && constructions == 0` condition let it pass.
    #
    # Two emission paths exist in-tree. The macro path names the event on the macro line (`emit!(Name {`
    # / `emit_cpi!(Name {`) and is what the app programs use. ZamaHost instead hands every event to the
    # shared `emit_event_cpi` helper, where the struct literal sits a line or two below the call, so a
    # construction counts when that call appears just above it.
    #
    # That window is deliberately tight, and it constrains how an emission may be written: hoisting the
    # literal into a local (`let e = Name { .. }; emit_event_cpi(&auth, &e)?;`) puts the construction out
    # of range and this check reports the event as never emitted. That is the intended trade. Every
    # emission in zama-host reads the same way as a result, and the alternative — a window wide enough
    # for any phrasing — is a window wide enough to vouch for a second event that merely happens to be
    # constructed nearby.
    emits=$( (grep -E 'emit_cpi!|emit!' "$INDEX" || true) | grep -cE "\b${event}\b" || true)
    if [ "$emits" -eq 0 ]; then
      manual=0
      for constructing_file in $( (grep -E "\b${event} \{" "$INDEX" || true) | cut -d: -f1 | sort -u); do
        if (grep -A4 'emit_event_cpi(' "$constructing_file" || true) | grep -qE "\b${event}\b"; then
          manual=1
        fi
      done
      if [ "$manual" -eq 0 ]; then
        echo "NEVER-EMITTED EVENT: ${event} (nothing emits it: no emit!/emit_cpi!, and no emit_event_cpi call takes it)"
        fail=1
      fi
    fi
  done
fi

if run_check 3; then
  echo "== 3. rejected glossary aliases =="
  # SCAFFOLDING, WITH AN EXPIRY. This check exists to stop a retired name being written again while
  # the renames of fhevm-internal#1859 are still fresh in a long-lived branch — it is not a permanent
  # rule the way checks 1, 2 and 6 are. Delete it once the vocabulary has been on `feature/solana`
  # for a release and the old names are gone from the base branch; GLOSSARY.md stays normative either
  # way. Every entry added here also adds an exception list to maintain, which is the cost that makes
  # the expiry worth honouring rather than letting the check ossify.
  #
  # Each entry is a retired alias from GLOSSARY.md's "Replaces" column plus, where the word also has
  # a legitimate unrelated meaning in these trees, the narrow exception that keeps it. An exception
  # is a documented sentence here, never a blanket skip of a file or a directory.
  #
  # `label` -> reported name; `scope` -> fhe|core|all; `exceptions` -> an -viE pattern applied to the
  # hits; the rest are grep args.
  check_alias() {
    local label="$1" scope="$2" exceptions="$3"; shift 3
    # Every entry records its label so the self-test can prove it has a fixture. Without this the
    # fixture list and the entry list drift, which is how five entries (including the newest) went
    # unexercised.
    printf '%s\n' "$label" >> "$ALIAS_LABELS"
    local hits
    local -a roots
    case "$scope" in
      fhe)  roots=("${FHE_ROOTS[@]}") ;;
      core) roots=("${CORE_ROOTS[@]}") ;;
      all)  roots=("${ALL_ROOTS[@]}") ;;
      # ALL plus the connector crates and the connector-auth fixtures, for a word whose only live
      # occurrences are there. The fixture tree is in no other root: the generator lives under
      # kms-worker/tests but the vector constants and the committed JSON do not.
      kms)  roots=("${ALL_ROOTS[@]}" kms-connector/crates solana/test-fixtures) ;;
      *) echo "check_alias: unknown scope '${scope}' for '${label}'" >&2; exit 2 ;;
    esac
    # `.tsx` and `.json` are swept too: the dapp's React surface is .tsx, and several entries below
    # claim the retired spelling is gone from "the IDL", which is .json. Without them those claims
    # were unenforced.
    hits=$( (grep -rniE --include='*.rs' --include='*.ts' --include='*.tsx' --include='*.md' \
      --include='*.py' --include='*.sh' --include='*.yml' --include='*.json' \
      "${EXCLUDES[@]}" "$@" "${roots[@]}" 2>/dev/null || true) )
    hits=$(echo "$hits" \
      | (grep -v 'solana/docs/GLOSSARY.md' || true) \
      | (grep -v 'solana/docs/DESIGN_DECISIONS.md' || true) \
      | (grep -v 'dead-surface-check.sh' || true) )
    if [ -n "$exceptions" ]; then
      # Applied to each hit's CONTENT, never to the `path:line:` prefix. Matching the whole record
      # let an exception that happened to look like a path blanket-exempt a whole file — the blanket
      # skip the contract above forbids, and what made the `app_account` entry silently inert.
      hits=$(printf '%s\n' "$hits" | while IFS= read -r record; do
        [ -n "$record" ] || continue
        content=${record#*:}
        content=${content#*:}
        printf '%s\n' "$content" | grep -qiE "$exceptions" || printf '%s\n' "$record"
      done)
    fi
    if [ -n "$hits" ]; then
      echo "REJECTED ALIAS (${label}):"
      echo "$hits"
      fail=1
    fi
  }

  # `b"FHE_eval*"` are the frozen handle-derivation domain separators: those bytes are hashed into
  # every handle ever minted, so the tag keeps the old spelling forever and the `computed_eval_*`
  # helpers that produce it keep matching names on purpose. Only the instruction identifier was
  # renamed. The English verb "evaluate" is NOT swept — FHE evaluation is what the coprocessor
  # actually does, and that is a recorded decision, not an oversight.
  check_alias 'fhe_eval — renamed to fhe_execute' all \
    'b"FHE_eval' -E '\bfhe_eval\b|\bFheEval\b'
  # Bare `born`, not just `born[-_ ]public`: the narrow pattern could not match the spellings that
  # actually survived — `*born* public` (asterisks between the two words), "are born with", "born in
  # `initialize_mint`". Prose is where retired vocabulary hides, because a rename sweep looks for
  # identifiers. The English word has no legitimate use in these trees now that all five sites say
  # "created" instead, so the whole family is banned and `birthday` stays exempted.
  check_alias 'born / birth — renamed to created-public / create' all \
    'birthday' -iE '\bborn\b|\bbirth\b|_birth\b|birth_'
  # `value_key` as an identifier was renamed to encrypted-value-ID vocabulary. The signed/wire
  # spellings stay: the sha256 tag string "...value-key-v1" (preimage bytes), the v3 JSON key
  # `aclValueKey`, and the connector's matching `acl_value_key` field.
  check_alias 'value_key identifier — renamed to encrypted_value_id' all \
    '' -E '\bvalue_key\b' --exclude-dir=utils
  # The encrypted-value ID components are domain / encrypted_value_account_authority /
  # encrypted_value_label. `acl_domain_key` is NOT swept: it is the normative field name of the signed
  # Solana permit (`allowed_acl_domain_keys` in the user-decryption specification) and of the v3 wire,
  # so the permit crate and the connector keep it deliberately.
  #
  # The `UserDecryptionDelegation` witness in kms-connector used to be excluded by name: it holds the
  # key a delegation is scoped over, and that was argued to be a different object from the ID
  # component. It is not — the delegation PDA's third seed IS the encrypted value account authority,
  # which is why one delegation covers that authority's values in every domain. The mirror is renamed
  # and the exclusion is gone, so nothing here is exempt.
  #
  # Scope is `kms` because `app_account` has no occurrence anywhere else: under plain `all` this entry
  # could not reach the word it documents, so it passed vacuously and its exception masked nothing.
  #
  # `app_authority` / `ExecutionAppAuthority` is banned for the same reason. It was the SDK's public
  # name for this *same key* — `lower.rs` compares an output's declared authority against it directly,
  # so the two spellings named one concept — and it is now `encrypted_value_account_authority` /
  # `ExecutionEncryptedValueAccountAuthority`. That rename touched 102 lines of public SDK surface, so
  # the guard matters: nothing else would stop the short name coming back on the next builder change.
  check_alias 'app_account / app_authority — renamed to encrypted_value_account_authority' kms \
    '' \
    -E '\bapp_account\b|\bapp_accounts\b|\bapp_account_authority\b|\bauthorized_app_accounts\b|\bappAccount\b|\bapp_authority\b|\bappAuthority\b|\bExecutionAppAuthority\b'
  # The same object under a third name, and the one the identifier sweep above could not see: prose
  # and constants called the authority a request may not supply an "app context", including the
  # reserved wildcard sentinel that stands in the authority's seed position. No spelling of it is
  # legitimate — the delegation row is keyed by the authority, and where the phrase meant the
  # authority *and* the domain, both are glossary terms with their own names.
  check_alias 'app context — say encrypted value account authority' kms \
    '' \
    -iE '(^|[^[:alnum:]_])app[ _-]context'
  # There was a `check_alias 'encrypted_value_label — renamed to label'` here, banning the long name.
  # That decision was reversed: bare "label" says only that the component is 32 bytes, which is true of
  # all three, so the long name is now the canonical one and the guard would reject correct code. The
  # reverse cannot be guarded by grep — "label" is a legitimate word for log labels and chart axes —
  # so this one is carried by review and by the GLOSSARY row instead.
  # The SDK's `namespace` became `label`. Swept in CORE only: `namespace` means a Kubernetes
  # namespace in the workflows and a TypeScript namespace in the test-suite tooling.
  check_alias 'SDK namespace — renamed to label' core '' -E '\bnamespace\b|\bnamespaceKey\b'
  # batch <- frame, plan. "frame" survives only in its Solana runtime sense (the CPI/instruction
  # stack frame), and as the proper name of the retired `execute_frame` RFC-024 prototype where the
  # sentence says so. A batch of steps is never a frame.
  # The last group is the demo dapp's architecture walkthrough, where a "frame" is a presentation
  # panel — the film sense of the word, with its own CSS classes. Surfaced only once `.tsx` joined the
  # sweep. It is the same kind of unrelated meaning as the stack frame above, not a walk of steps.
  check_alias 'frame — a walk of steps is an execution' all \
    'stack frame|instruction frame|enclosing frame|execution frame|cpi frame|frame it belongs to|older `execute_frame`|framework|heap frame|heap-frame|ArchitectureFrame|architecture-frame|frame-(heading|label|statement|separator)|frameNumber|\bframed\b|frames\.(map|length)|frame\.(diagram|title)' \
    -E '\bframes?\b|execute_frame|_frame\b|frame_|Frame[A-Z]'
  # One fhe_execute invocation is an `execution`, never a batch: its steps are dependent, each reading
  # what the one before it produced, which is the opposite of what a batch means. Swept in FHE scope
  # only — see FHE_ROOTS for why the batcher and the listener keep the word.
  check_alias 'batch — one fhe_execute invocation is an execution' fhe \
    'deliberately not a batch' -iE '\bbatch(es|ed|ing)?\b'
  # "plan" is CORE-only: test-suite/fhevm threads a docker-compose `plan: StackSpec` through every
  # generator. Inside the FHE core an fhe_execute batch is the only thing a "plan" could be.
  check_alias 'plan — an fhe_execute batch is a batch' core \
    'transaction plan|claim plan|instruction plan|const plan|let plan|plan\.|plan =|plan\)|plan,|plan\?|we plan|plans to|plan and stop' \
    -iE '\bplans?\b'
  # dictionary <- pool. CORE-only: the listener and the proof service are full of Postgres connection
  # pools. Inside the FHE core the only collection that could be called a pool is the dictionary.
  check_alias 'pool — the interning structure is the dictionary' core \
    'db pool|database pool|connection|pgpool|pool is big enough|vault pool|deposit pool|pool\(\)|poolopt|pool slots|pool capacity|pool size|acquire|max_connections|admission' \
    -iE '\bpools?\b'
  # persistent <- durable. The proof store's durability vocabulary (a durably ingested checkpoint) is
  # a different axis from value persistence, and Solana's durable nonce is a protocol term.
  check_alias 'durable — a persistent value is persistent' all \
    'durable ingest|durable checkpoint|durable tip|durable history_start|durable nonce|durably ingest|observation durably|durably, keyed' \
    -iE '\bdurable\b|\bdurably\b'
  # update <- supersede, rotation.
  check_alias 'supersede — an updated handle is updated' all '' \
    -iE '\bsupersede|\bsuperseded\b|\bsupersedes\b'
  # `rotation` is matched in the update sense only, not as a bare word, and that narrowing is the
  # point rather than a concession: four other rotations are real and unrelated. Bit rotations
  # (`Rotl`/`Rotr`) are FHE ops. KMS context rotation and coprocessor signer-set rotation are
  # EVM-parity operations on their own state. An ACL *audience* rotation swaps which subjects may
  # read a value and is not a handle update at all. What is banned is calling the handle update
  # itself a rotation — "the balance rotates", "before any balance rotation".
  check_alias 'rotation — an updated handle is updated' all '' \
    -iE '(balance|handle|value|amount|output|receipt)s? rotat|rotat[a-z]* the (confidential )?(balance|handle|value|amount)|(balance|handle|value|amount) rotation'
  # encrypted value account <- lineage account. Scope is `kms`: every occurrence of the word lived in
  # the connector and in the generated auth vectors, so under plain `all` this entry passed vacuously.
  check_alias 'lineage — renamed to encrypted value account' kms '' -iE '\blineage\b'
  # The adjective matters: "value account" describes every SPL token account, so dropping
  # "encrypted" turns the one distinguishing fact — that this account holds an *encrypted* value's
  # handle, subject list and MMR — into a generic phrase. The struct is `EncryptedValue`; the
  # adjective is the part that must survive in prose.
  #
  # ERE has no negative lookbehind, so the correct phrase is excluded by the line-level exception
  # rather than by the pattern. The known limit: a line carrying both spellings is exempted by its
  # correct one. That is the same trade every other entry here makes, so the check's job is catching
  # new prose — where a single line saying it both ways is not a realistic way to reintroduce the
  # alias. A second limit, bigger than that one: these patterns match whole words, and `_` is a word
  # character to grep, so a retired word inside a compound identifier (`value_account`,
  # `max_op_batch`) is invisible here. Identifier renames are carried by review, not by this sweep.
  # (GLOSSARY.md is dropped by check_alias itself: its "Replaces" column has to keep the retired
  # spelling greppable, which is that column's entire purpose.)
  check_alias 'value account — say encrypted value account' all 'encrypted[ -]value[ -]account' \
    -iE '(^|[^-[:alnum:]_])value[ -]accounts?\b'
  # The operand/output variants were renamed to say what the slot is rather than why it was admitted:
  # `AllowedPersistent` -> `StoredValue`, and `AllowedLocal` -> `EarlierStep` (operand) or `Transient`
  # (output). Both spellings are gone from the wire names, the IDL, and the prose, so a bare match is
  # the rule. The English word "allowed" is untouched — subject membership really is an allow-list.
  check_alias 'AllowedPersistent / AllowedLocal — renamed to StoredValue / EarlierStep / Transient' all \
    '' -E '\bAllowedPersistent\b|\bAllowedLocal\b'
  # decoded op records <- `Fhe*Event` structs. The two compute events keep their names
  # (they are emitted); the nine per-op value types must not come back as events.
  check_alias 'Fhe*Event — the per-op value types are decoded op records' all \
    'FheExecuteRandomSeedsEvent' -E '\bFhe[A-Za-z0-9]*Event\b'
  # "lookup table" is banned for the interning dictionary. The Solana Address Lookup Table keeps its
  # name and is very often written as a bare "lookup table" ("the settle lookup table"), so a bare
  # match cannot be the rule. The old exception list went the other way and waved through any line
  # containing `batch`, `settle`, `slot`, or `addresses` — which is exactly the phrasing the check
  # exists to catch. The rule is now what it should always have been: a lookup table OF handles, keys,
  # or dictionary entries is the banned sense, whatever else the line says. An ALT holds addresses.
  alias_hits=$( (grep -rniE --include='*.rs' --include='*.ts' --include='*.md' "${EXCLUDES[@]}" \
    -E 'lookup[- ]?table of (handles|keys|entries|constants|operands|subjects)|(handle|key|dictionary|interned|intern|operand|constant)[- ]lookup[- ]?table|lookup[- ]?table \(the (dictionary|intern)' \
    "${RUST_ROOTS[@]}" "${TS_ROOTS[@]}" solana/docs 2>/dev/null || true) \
    | (grep -v 'solana/docs/GLOSSARY.md' || true) \
    | (grep -v 'solana/docs/DESIGN_DECISIONS.md' || true) )
  # Registered like a `check_alias` entry so the self-test's fixture-parity gate covers it too, even
  # though it is spelled out inline rather than going through the helper.
  printf '%s\n' 'lookup table — the interning structure is the dictionary' >> "$ALIAS_LABELS"
  if [ -n "$alias_hits" ]; then
    echo "REJECTED ALIAS (lookup table — the interning structure is the dictionary):"
    echo "$alias_hits"
    fail=1
  fi
fi

if run_check 4; then
  echo "== 4. retrofit sentinels without a justification at the site =="
  # fhevm-internal#1859 §8 taxonomy: where Solana meets EVM-shaped infrastructure, a constant
  # stuffed into a field with no Solana meaning is allowed ONLY with a written justification at the
  # site. The retrofit is discovered, not whitelisted: a sentinel counts when the same line also
  # names an EVM-shaped field, which is what makes it a retrofit rather than an ordinary zero. A
  # plain `[0u8; 32]` scratch buffer or an empty MMR peak is not a retrofit and is not swept.
  SENTINEL='Address::ZERO|FixedBytes::ZERO|\[0u8; ?(20|32)\]|0x0{40}|new Uint8Array\((20|32)\)|Pubkey::default\(\)'
  EVM_SHAPED_FIELD='\bcaller\b|contract_address|contractAddress|user_address|userAddress|owner_address|ownerAddress|allowed_contracts|allowedContracts|handle_contract_pairs|contract_addresses|delegator_address|delegate_address|transaction_hash|txHash|block_number'
  JUSTIFICATION='placeholder|not persisted|unused on the Solana|oblivious|ignores the EVM|reuses the EVM|discard|EVM-shaped|EVM-only|no Solana meaning|off-gateway|inert'
  # bash 3.2 (the macOS default) has no associative arrays, so covered files accumulate in a string.
  sentinel_seen=""
  retrofit_hits=$( (grep -rnE --include='*.rs' --include='*.ts' "${EXCLUDES[@]}" "$SENTINEL" \
    "${SENTINEL_ROOTS[@]}" "${SENTINEL_FILES[@]}" 2>/dev/null || true) \
    | (grep -E "$EVM_SHAPED_FIELD" || true) \
    | (grep -vE '(^|/)tests?/|\.test\.tsx?:|tests\.rs:' || true) )
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    file=${hit%%:*}
    rest=${hit#*:}
    line=${rest%%:*}
    # In-file test regions restate EVM shapes as fixtures; only production code is swept. (Dropping
    # whole test *files* is not enough — `hcu.rs` builds EVM-shaped fixtures in its own test module.)
    test_line=$( (grep -n -m1 -E '#\[cfg\(test\)\]|describe\(' "$file" || true) | cut -d: -f1)
    if [ -n "$test_line" ] && [ "$line" -ge "$test_line" ]; then continue; fi
    sentinel_seen="${sentinel_seen} ${file}"
    start=$((line > 12 ? line - 12 : 1))
    # Comment markers stripped and the window joined into one line: justifications are prose and
    # wrap, so a phrase straddles two comment lines with a `///` in between (the same trap the
    # tombstone check above hit).
    context=$(sed -n "${start},${line}p" "$file" \
      | sed -E -e 's#^[[:space:]]*//+!?[[:space:]]*##' -e 's#^[[:space:]]*\*[[:space:]]*##' \
      | tr '\n' ' ' | tr -s ' ')
    if ! echo "$context" | grep -qiE "$JUSTIFICATION"; then
      echo "UNJUSTIFIED RETROFIT SENTINEL: ${hit}"
      fail=1
    fi
  done <<< "$retrofit_hits"
  for file in "${SENTINEL_FILES[@]}"; do
    [ -f "$file" ] || { echo "MISSING SENTINEL FILE: ${file} (update SENTINEL_FILES)"; fail=1; continue; }
    if ! echo "$sentinel_seen" | grep -qF " ${file}"; then
      echo "STALE SENTINEL ENTRY: ${file} has no production retrofit left (drop it from SENTINEL_FILES)"
      fail=1
    fi
  done
fi

if run_check 5; then
  echo "== 5. reused-value retrofits whose justification must stay written =="
  # The other §8 shape: a field with no Solana meaning filled with a *reused* real value rather than
  # a zero. There is no constant to grep for, so the check pins the explanation — delete the prose
  # and this fails, because the prose is the only way a reader learns the field is inert.
  for entry in "${RETROFIT_JUSTIFICATIONS[@]}"; do
    file=${entry%%|*}
    phrase=${entry#*|}
    [ -f "$file" ] || { echo "MISSING RETROFIT FILE: ${file} (update RETROFIT_JUSTIFICATIONS)"; fail=1; continue; }
    if ! grep -qF "$phrase" "$file"; then
      echo "MISSING RETROFIT JUSTIFICATION: ${file} no longer says \"${phrase}\""
      fail=1
    fi
  done
fi

if run_check 6; then
  echo "== 6. exported symbols with no audience =="
  # #1859 asked for "exported, no non-test caller". Applied literally that flags 174 symbols, and
  # ~150 of them are Codama-generated client functions and the on-chain SDK's deliberately complete
  # op surface — a gate that loud gets disabled, and deleting a library's API because this repo
  # happens not to call it is the wrong fix. So the criterion is split in two, and both halves are
  # hard failures:
  #
  #   6a. Zero references ANYWHERE in the owned trees, tests included. Nothing names it; it is dead
  #       by any definition. This is what caught `signs_cpi_account` and `full_width`.
  #   6b. References only from tests, in a file that does not say who its audience is. A library
  #       module whose exports exist for app authors declares that once, in a
  #       `Public API surface:` line naming the audience. Then the exports are accounted for in
  #       writing instead of passing silently.
  #
  # Generated clients are exempt as a category: their surface is generated whole, so deleting from it
  # would only be undone by the next regeneration, and
  # regenerating them would undo any deletion.
  API_SURFACE_MARKER='Public API surface:'

  # Reference count across everything outside the declaring file, tests included.
  external_refs() {
    local name="$1" declaring_file="$2"
    (grep -rn --include='*.rs' --include='*.ts' --include='*.tsx' "${EXCLUDES[@]}" -E "\b${name}\b" \
      "${RUST_ROOTS[@]}" "${TS_ROOTS[@]}" kms-connector/crates 2>/dev/null || true) \
      | (grep -v -F "${declaring_file}:" || true) | wc -l | tr -d ' '
  }

  # References from inside the declaring file, minus the declaration itself. A `pub fn` that only its
  # own module calls is not dead code — it is over-exposed, and the fix is to drop `pub`, not to
  # delete the body. Separating the two verdicts is what makes the finding actionable.
  internal_refs() {
    local name="$1" file="$2" kind="$3"
    local declaration
    case "$kind" in
      'pub fn') declaration="pub fn ${name}" ;;
      *) declaration="function ${name}" ;;
    esac
    (grep -nE "\b${name}\b" "$file" || true) | (grep -v -F "$declaration" || true) | wc -l | tr -d ' '
  }

  # `same_file_counts` (fourth argument) reports an export as dead only when nothing anywhere uses it,
  # instead of requiring a caller outside its own file. That is the right rule for TypeScript and the
  # wrong one for Rust, because the remedy differs. A Rust `pub fn` used only inside its own file can
  # simply lose its `pub`, and a `#[cfg(test)] mod tests` in that same file still reaches it — so
  # demanding an outside caller is actionable. A TypeScript module has no such inside: a sibling
  # `foo.test.ts` can only import what `foo.ts` exports, so an over-exposed helper cannot be un-exported
  # while its unit test exists. Reporting those would leave 16 findings in the demo dapp whose only
  # available remedy is an audience line vouching for itself. So for TypeScript this check is a deadness
  # check and not an over-exposure one, and TypeScript over-exposure is deliberately unchecked.
  check_export() {
    local file="$1" kind="$2" name="$3" same_file_counts="${4:-}"
    case "$file" in
      */internal/generated/*) return ;;
    esac
    local production external internal
    production=$(index_refs "\b${name}\b" "$file")
    [ "$production" -gt 0 ] && return
    if [ -n "$same_file_counts" ]; then
      # Its own file, minus the declaration itself. A helper its module actually calls is alive.
      local own
      own=$(grep -cE "\b${name}\b" "$file" 2>/dev/null || echo 0)
      if [ "$own" -gt 1 ]; then return; fi
    fi
    external=$(external_refs "$name" "$file")
    if [ "$external" -eq 0 ]; then
      internal=$(internal_refs "$name" "$file" "$kind")
      if [ "$internal" -eq 0 ]; then
        echo "DEAD EXPORT: ${file}: ${kind} ${name} (zero references anywhere, tests and its own module included)"
        fail=1
        return
      fi
    fi
    if ! head -40 "$file" | grep -qF "$API_SURFACE_MARKER"; then
      echo "UNDECLARED EXPORT: ${file}: ${kind} ${name} (no production caller outside its file, and the file does not state an audience — add a '${API_SURFACE_MARKER} ...' line, drop the export, or remove it)"
      fail=1
    fi
  }

  # Anchor's `#[program]` handlers and `#[derive(Accounts)]` plumbing are pub-by-generation and the
  # runtime is what calls them, so a zero-reference count means nothing there. That exempts two file
  # shapes, not the program crates as a whole: the file holding the `#[program]` module, and the
  # `instructions/` tree holding the handler bodies with their account contexts. Everything else in a
  # program crate — state, events, errors, constants, the fhe helpers — is ordinary code no macro
  # generates and no runtime calls, so it is swept on the same terms as the SDK crates. Skipping the
  # crates wholesale is what let `encrypted_transfer_success_label` and
  # `encrypted_debit_candidate_label` outlive DD-019, which stopped creating the scratch PDAs they
  # named: nothing referenced either one, and no check was looking. This pass then renamed both of them
  # forward (§4) without noticing, which is the argument for the widened scope in one line.
  program_sweep_files=$(find solana/programs -name '*.rs' \
    -not -path '*/instructions/*' -not -path '*/target/*' -print0 \
    | xargs -0 grep -L '#\[program\]' 2>/dev/null || true)
  rust_pub_decls=$(grep -rnE --include='*.rs' "${EXCLUDES[@]}" '^\s*pub fn [a-z_][a-z_0-9]*' solana/crates \
    | grep -vE '(^|/)tests?/|tests\.rs:')
  if [ -n "$program_sweep_files" ]; then
    # -H because grep omits the filename for a single-file argument, and `${decl%%:*}` needs it.
    rust_pub_decls="${rust_pub_decls}
  $(echo "$program_sweep_files" | tr '\n' '\0' \
      | xargs -0 grep -HnE '^\s*pub fn [a-z_][a-z_0-9]*' \
      | grep -vE '(^|/)tests?/|tests\.rs:')"
  fi
  while IFS= read -r decl; do
    [ -n "$decl" ] || continue
    check_export "${decl%%:*}" "pub fn" \
      "$(echo "$decl" | grep -oE 'pub fn [a-z_][a-z_0-9]*' | awk '{print $3}')"
  done <<< "$rust_pub_decls"

  # TypeScript: exported functions in the SDK's Solana surface and the demo dapp. Re-export from an
  # index counts as a reference — the dead shape is a symbol no index and no caller names.
  ts_pub_decls=$(grep -rnE --include='*.ts' --include='*.tsx' "${EXCLUDES[@]}" \
    '^export (async )?function [A-Za-z_][A-Za-z_0-9]*' sdk/js-sdk/src/solana solana/demo-dapp/src \
    | grep -vE '\.test\.tsx?:')
  while IFS= read -r decl; do
    [ -n "$decl" ] || continue
    check_export "${decl%%:*}" "export function" \
      "$(echo "$decl" | grep -oE 'function [A-Za-z_][A-Za-z_0-9]*' | awk '{print $2}')" ts
  done <<< "$ts_pub_decls"

  # `export const f = () => …` is the same export as `export function f`, and in these two trees it is
  # the more common one — 81 declarations against 273. Matching only the `function` form left roughly a
  # quarter of the TypeScript surface unswept while the check reported on "exported symbols".
  ts_pub_arrows=$(grep -rnE --include='*.ts' --include='*.tsx' "${EXCLUDES[@]}" \
    '^export const [A-Za-z_][A-Za-z_0-9]* *(:[^=]*)?= *(async *)?(\(|function)' \
    sdk/js-sdk/src/solana solana/demo-dapp/src | grep -vE '\.test\.tsx?:')
  while IFS= read -r decl; do
    [ -n "$decl" ] || continue
    check_export "${decl%%:*}" "export const" \
      "$(echo "$decl" | grep -oE '^[^:]+:[0-9]+:export const [A-Za-z_][A-Za-z_0-9]*' \
         | grep -oE '[A-Za-z_][A-Za-z_0-9]*$')" ts
  done <<< "$ts_pub_arrows"
fi

if run_check 7; then
  echo "== 7. every swept root is a CI trigger for this script =="
  # The script only protects a tree if a change to that tree runs it. It used to run inside
  # build-and-test, gated on `solana`, so an edit to the proof service, the listener, the
  # kms-connector, the SDK's Solana surface, or the test suite could add a retired name with the sweep
  # never executing. The dedicated `dead-surface` job fixed that, and this check keeps the two lists in
  # step: every root swept below must be covered by a path in the job's paths-filter.
  TRIGGER_WORKFLOW='.github/workflows/solana-tests.yml'
  # The `dead-surface:` filter block: its own indented list, ending at the next filter name.
  trigger_paths=$(awk '
    /^            dead-surface:$/ { inside = 1; next }
    inside && /^            [a-z-]+:$/ { inside = 0 }
    inside && /^              - / { sub(/^              - /, ""); print }
  ' "$TRIGGER_WORKFLOW")

  root_is_triggered() {
    local root="$1" path
    while IFS= read -r path; do
      [ -n "$path" ] || continue
      path="${path%/\*\*}"
      case "$root/" in
        "$path"/*) return 0 ;;
      esac
      [ "$root" = "$path" ] && return 0
    done <<< "$trigger_paths"
    return 1
  }

  # SENTINEL_ROOTS, SENTINEL_FILES and the retrofit-justification files are included: checks 4 and 5
  # read them, so a path of theirs outside the trigger filter is the same hole as an untriggered sweep
  # root. Both `relayer/` files were exactly that until the filter above learned them.
  for root in "${ALL_ROOTS[@]}" "${FHE_ROOTS[@]}" "${SENTINEL_ROOTS[@]}" "${SENTINEL_FILES[@]}" \
    "${RETROFIT_JUSTIFICATIONS[@]%%|*}" kms-connector/crates ${DEAD_SURFACE_EXTRA_ROOT:-}; do
    if ! root_is_triggered "$root"; then
      echo "UNTRIGGERED ROOT: ${root} is swept by this script but no path in ${TRIGGER_WORKFLOW}'s dead-surface filter matches it — a change there would not run this check"
      fail=1
    fi
  done
fi

if [ "$SELF_TEST" -eq 1 ]; then
  echo "== self-test: every check must fire on a fixture that violates it =="
  # Children must not refuse to run on the fixtures this block deliberately plants.
  export DEAD_SURFACE_SELFTEST_ACTIVE=1
  trap 'rm -f "$INDEX" "$ALIAS_LABELS" "${SELFTEST_FIXTURES[@]}"' EXIT INT TERM
  self_test_fail=0
  covered_labels=""
  # Exit status alone is not evidence. Any unrelated finding anywhere in the tree also makes the
  # child exit 1, so a fixture the sweeps cannot see still reported "ok" — every one of these was
  # satisfiable by planting one real violation elsewhere. The expected report text is required too,
  # which ties each fixture to the specific check it is meant to exercise.
  # `only` is the check number the fixture violates, passed to the child so it runs that check
  # alone. Every assertion below sets it; a child that ran all seven would still pass, just
  # twenty times slower, which is the state this replaced.
  expect_fires() {
    local what="$1" expect="$2" label="$3" only="$4"; shift 4
    local out status
    out=$(DEAD_SURFACE_ONLY_CHECK="$only" "$@" </dev/null 2>&1) && status=0 || status=$?
    if [ "$status" -eq 0 ]; then
      echo "SELF-TEST HOLE: ${what} did not fire on a violating fixture"
      self_test_fail=1
    elif ! printf '%s\n' "$out" | grep -qF "$expect"; then
      echo "SELF-TEST HOLE: ${what} exited ${status} without reporting: ${expect}"
      self_test_fail=1
    else
      echo "  ok: ${what} fires"
      if [ -n "$label" ]; then
        covered_labels="${covered_labels}
${label}"
      fi
    fi
  }
  # Check 3 (aliases): each retired word must be caught, and the report must name that entry.
  # `solana/crates` is in ALL, CORE and FHE scope at once, so one fixture location serves every
  # entry — the previous split (solana/docs for most, solana/crates for the FHE-scoped one) is why
  # the core-scoped entries were easy to leave uncovered.
  fixture="solana/crates/zama-fhe/.dead-surface-selftest.md"
  while IFS='|' read -r word label; do
    [ -n "$word" ] || continue
    printf '%s\n' "$word" > "$fixture"
    expect_fires "alias sweep on \"${word}\"" "REJECTED ALIAS (${label})" "$label" 3 bash "$SELF"
    rm -f "$fixture"
  done <<'FIXTURES'
fhe_eval|fhe_eval — renamed to fhe_execute
born-public|born / birth — renamed to created-public / create
are born with|born / birth — renamed to created-public / create
value_key|value_key identifier — renamed to encrypted_value_id
app_account|app_account / app_authority — renamed to encrypted_value_account_authority
app_authority|app_account / app_authority — renamed to encrypted_value_account_authority
app context|app context — say encrypted value account authority
namespace|SDK namespace — renamed to label
a frame of steps|frame — a walk of steps is an execution
one fhe_execute batch|batch — one fhe_execute invocation is an execution
a plan of steps|plan — an fhe_execute batch is a batch
the handle pool|pool — the interning structure is the dictionary
durable value|durable — a persistent value is persistent
supersede|supersede — an updated handle is updated
handle rotation|rotation — an updated handle is updated
lineage|lineage — renamed to encrypted value account
the value account holds it|value account — say encrypted value account
AllowedPersistent|AllowedPersistent / AllowedLocal — renamed to StoredValue / EarlierStep / Transient
FheAddEvent|Fhe*Event — the per-op value types are decoded op records
the batch's lookup table of handles|lookup table — the interning structure is the dictionary
FIXTURES
  # Nothing kept the fixture list and the entry list in step, so five entries — including the one
  # added by the commit that introduced this self-test — were never exercised. Every entry now
  # registers its label as it runs, and an entry with no fixture is a hole.
  while IFS= read -r label; do
    [ -n "$label" ] || continue
    case "$covered_labels" in
      *"$label"*) ;;
      *)
        echo "SELF-TEST HOLE: alias entry '${label}' has no fixture in this block"
        self_test_fail=1
        ;;
    esac
  done < <(sort -u "$ALIAS_LABELS")
  # Check 4: an unjustified EVM-shaped zero-fill in a swept file.
  fixture="solana/crates/zama-fhe/src/dead_surface_selftest.rs"
  printf 'pub fn f() { let contract_address = [0u8; 20]; let _ = contract_address; }\n' > "$fixture"
  expect_fires "retrofit sentinel sweep" "UNJUSTIFIED RETROFIT SENTINEL" "" 4 bash "$SELF"
  rm -f "$fixture"
  # Check 6: an exported function nobody calls.
  printf 'pub fn dead_surface_selftest_never_called() {}\n' > "$fixture"
  expect_fires "uncalled-export sweep" "dead_surface_selftest_never_called" "" 6 bash "$SELF"
  rm -f "$fixture"
  # Check 6 again, in a program crate: the export sweep read only solana/crates until the program
  # crates were folded in, so a dead `pub fn` in a program's state or events module was unreachable by
  # construction. This fixture is what keeps that scope from narrowing back — a file no `mod` declares
  # never reaches rustc, and the sweep is grep-driven, so it needs no wiring to be seen.
  program_fixture="solana/programs/zama-host/src/dead_surface_selftest.rs"
  printf 'pub fn program_dead_surface_selftest_never_called() {}\n' > "$program_fixture"
  expect_fires "uncalled-export sweep (program crates)" \
    "program_dead_surface_selftest_never_called" "" 6 bash "$SELF"
  rm -f "$program_fixture"
  # Check 2: an `#[event]` struct nothing emits. Check 2 is the one this pass reworked — it learned
  # zama-host's shared emitter — and it was the only reworked check with no fixture, which is the
  # position every silently-vacuous check in this script started from.
  printf '#[event]\npub struct DeadSurfaceSelftestNeverEmittedEvent {\n    pub version: u8,\n}\n' \
    > "$program_fixture"
  expect_fires "never-emitted event sweep" \
    "DeadSurfaceSelftestNeverEmittedEvent" "" 2 bash "$SELF"
  rm -f "$program_fixture"
  # Check 6, TypeScript half: both fixtures above are Rust, so the TS declaration patterns had never
  # been shown to fire at all — and one of them (`export const f = () => …`, the more common form in
  # these two trees) was missing entirely while the check reported on "exported symbols".
  ts_fixture="solana/demo-dapp/src/deadSurfaceSelftest.ts"
  printf 'export const deadSurfaceSelftestNeverCalled = (): number => 1;\n' > "$ts_fixture"
  expect_fires "uncalled-export sweep (TypeScript arrow form)" \
    "deadSurfaceSelftestNeverCalled" "" 6 bash "$SELF"
  printf 'export function deadSurfaceSelftestNeverCalledFn(): number {\n  return 1;\n}\n' > "$ts_fixture"
  expect_fires "uncalled-export sweep (TypeScript function form)" \
    "deadSurfaceSelftestNeverCalledFn" "" 6 bash "$SELF"
  rm -f "$ts_fixture"
  # Check 1: an error variant nothing references. Two things had to be true for this fixture to
  # exercise the check rather than a neighbour: the file must carry `#[error_code]`, which is how
  # check 1 finds enums, and the variant must be indented four spaces and end in a comma, which is
  # the shape the variant-name grep matches. It also must not look like a tombstone — the sweep
  # exempts variants whose doc comment says the code is retained for ordinal stability.
  printf '#[error_code]\npub enum DeadSurfaceSelftestError {\n    DeadSurfaceSelftestUnreferenced,\n}\n' \
    > "$fixture"
  expect_fires "dead-error-variant sweep" \
    "DEAD ERROR VARIANT: DeadSurfaceSelftestUnreferenced" "" 1 bash "$SELF"
  rm -f "$fixture"
  # Check 5: the justification prose for a reused-value retrofit. The check pins phrases in real
  # files, so a fixture cannot violate it by planting a file — it would have to delete prose from
  # one. `DEAD_SURFACE_EXTRA_RETROFIT` adds an entry instead, which is the same code path: both
  # arms are exercised, the missing-file arm and the missing-phrase arm.
  expect_fires "retrofit-justification sweep (missing file)" \
    "MISSING RETROFIT FILE: solana/does-not-exist.ts" "" 5 \
    env "DEAD_SURFACE_EXTRA_RETROFIT=solana/does-not-exist.ts|any phrase" bash "$SELF"
  printf 'nothing here explains anything\n' > "$fixture"
  expect_fires "retrofit-justification sweep (missing phrase)" \
    "MISSING RETROFIT JUSTIFICATION: ${fixture} no longer says" "" 5 \
    env "DEAD_SURFACE_EXTRA_RETROFIT=${fixture}|a phrase this fixture does not contain" bash "$SELF"
  rm -f "$fixture"
  # Check 6b: an export whose only references come from tests, in a file that never says who its
  # audience is. Needs two fixtures — without a caller this is 6a (zero references anywhere), and
  # the two arms report differently, which is what the expected text distinguishes.
  caller_fixture="solana/crates/zama-fhe/src/dead_surface_selftest_caller.rs"
  printf 'pub fn dead_surface_selftest_test_only() {}\n' > "$fixture"
  printf '#[cfg(test)]\nmod tests {\n    #[test]\n    fn t() { super::super::dead_surface_selftest_test_only(); }\n}\n' \
    > "$caller_fixture"
  expect_fires "test-only-export sweep" \
    "UNDECLARED EXPORT: ${fixture}: pub fn dead_surface_selftest_test_only" "" 6 bash "$SELF"
  rm -f "$fixture" "$caller_fixture"
  # Check 7: a swept root outside every triggering path. Driven through the environment rather than
  # a fixture file, since the thing under test is the root list itself.
  expect_fires "untriggered-root sweep" \
    "UNTRIGGERED ROOT: coprocessor/fhevm-engine/tfhe-worker" "" 7 \
    env DEAD_SURFACE_EXTRA_ROOT=coprocessor/fhevm-engine/tfhe-worker bash "$SELF"
  if [ "$self_test_fail" -ne 0 ]; then
    echo "dead-surface-check: SELF-TEST FAILED (a check is vacuous)"
    exit 1
  fi
  echo "dead-surface-check: self-test clean"
fi

if [ "$fail" -ne 0 ]; then
  echo "dead-surface-check: FINDINGS ABOVE"
  exit 1
fi
echo "dead-surface-check: clean"
