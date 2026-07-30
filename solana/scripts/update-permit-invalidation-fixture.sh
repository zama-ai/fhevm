#!/usr/bin/env bash
# update-permit-invalidation-fixture.sh — regenerate the PermitInvalidation account fixture.
#
# Usage (from solana/):
#   bash scripts/update-permit-invalidation-fixture.sh
#
# When: after an intentional change to the on-chain PermitInvalidation record — a field
# added, removed, renamed, retyped or reordered, or the address seeds changed. Never to
# make a failing test pass: the fixture is the byte layout the KMS Connector decodes by
# hand out of an account snapshot, and a diff here is a change every off-chain reader has
# to be told about.
#
# Writes: test-fixtures/permit/permit_invalidation_account_v1.json
#
# Needs an existing SBF artifact: the fixture is captured from a real revocation run
# through Mollusk, so target/deploy/zama_host.so must already be built with the `poc`
# feature. This script does not rebuild it — rebuilding under an unpinned local toolchain
# also moves the committed compute-unit snapshots.
#
# After regenerating, review the diff and run the target without the gate, so the
# committed file is checked rather than rewritten.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
cd "$ROOT"

print_help() {
  awk 'NR == 1 { next } /^#/ { sub(/^# ?/, ""); print; next } { exit }' "$SCRIPT_PATH"
}

for arg in "$@"; do
  case "$arg" in
    -h|--help)
      print_help
      exit 0
      ;;
    *)
      echo "error: unknown argument: $arg" >&2
      echo "usage: bash scripts/update-permit-invalidation-fixture.sh" >&2
      exit 1
      ;;
  esac
done

command -v cargo >/dev/null || {
  echo "error: missing required command: cargo" >&2
  exit 1
}

ARTIFACT="$ROOT/target/deploy/zama_host.so"
OUT="$ROOT/test-fixtures/permit/permit_invalidation_account_v1.json"

[ -f "$ARTIFACT" ] || {
  echo "error: missing ${ARTIFACT#"$ROOT/"}; build it with the \`poc\` feature first" >&2
  exit 1
}

echo "regenerating the permit-invalidation account fixture..."
ZAMA_UPDATE_PERMIT_INVALIDATION_FIXTURE=1 \
  cargo test -p zama-solana-runtime-tests --test permit_invalidation_mollusk \
  the_committed_fixture_is_what_the_program_writes -- --nocapture

echo "checking the regenerated file against the suite..."
cargo test -p zama-solana-runtime-tests --test permit_invalidation_mollusk

echo "updated: ${OUT#"$ROOT/"}"
echo "review the JSON diff and commit it with the intentional layout change"
