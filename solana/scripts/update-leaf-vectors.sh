#!/usr/bin/env bash
# update-leaf-vectors.sh — regenerate the normative ACL leaf vectors.
#
# Usage (from solana/):
#   bash scripts/update-leaf-vectors.sh
#
# When: after an intentional change to how leaves are hashed — a prefix, the hash function,
# the leaf fields, the MMR node rule — or after adding a vector. Never to make a failing test
# pass: the coprocessor's stored leaves and every proof the KMS verifies depend on these bytes,
# so a diff here is a protocol change.
#
# Writes: test-fixtures/leaves/leaves_v1.json
#
# Needs no Solana toolchain: the ACL crate is pure Rust. The generator is the runner
# (crates/zama-solana-acl/tests/leaf_vectors.rs) — there is no separate binary to drift.
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
      echo "usage: bash scripts/update-leaf-vectors.sh" >&2
      exit 1
      ;;
  esac
done

command -v cargo >/dev/null || {
  echo "error: missing required command: cargo" >&2
  exit 1
}

OUT="$ROOT/test-fixtures/leaves/leaves_v1.json"

echo "regenerating leaf vectors..."
ZAMA_UPDATE_LEAF_VECTORS=1 \
  cargo test -p zama-solana-acl --test leaf_vectors committed_vectors_match_the_generator -- --nocapture

echo "checking the regenerated file against the suite..."
cargo test -p zama-solana-acl --test leaf_vectors

echo "updated: ${OUT#"$ROOT/"}"
echo "review the JSON diff and commit it with the intentional protocol change"
