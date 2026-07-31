#!/usr/bin/env bash
# update-permit-vectors.sh — regenerate the normative permit vectors.
#
# Usage (from solana/):
#   bash scripts/update-permit-vectors.sh
#
# When: after an intentional change to the permit canon — the typed form, the canonical
# text, the envelope, the fingerprint — or after adding a vector class. Never to make a
# failing test pass: the vectors are the specification's own fixtures, and a diff here
# is a protocol change that four other implementations will consume.
#
# Writes: test-fixtures/permit/permit_v1.json
#
# Needs no Solana toolchain: the permit crate is pure Rust. The generator is the runner
# (crates/zama-solana-permit/tests/vectors.rs) — there is no separate binary to drift.
#
# After regenerating, review the diff and run the full suite without the gate, so the
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
      echo "usage: bash scripts/update-permit-vectors.sh" >&2
      exit 1
      ;;
  esac
done

command -v cargo >/dev/null || {
  echo "error: missing required command: cargo" >&2
  exit 1
}

OUT="$ROOT/test-fixtures/permit/permit_v1.json"

echo "regenerating permit vectors..."
ZAMA_UPDATE_PERMIT_VECTORS=1 \
  cargo test -p zama-solana-permit --test vectors committed_vectors_match_the_generator -- --nocapture

echo "checking the regenerated file against the suite..."
cargo test -p zama-solana-permit --test vectors

echo "updated: ${OUT#"$ROOT/"}"
echo "review the JSON diff and commit it with the intentional canon change"
