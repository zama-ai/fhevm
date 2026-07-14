#!/usr/bin/env bash
# update-cost-snapshots.sh — regenerate Mollusk CU cost snapshot JSON under a
# pinned Solana/Anchor toolchain.
#
# Usage (from solana/):
#   bash scripts/update-cost-snapshots.sh
#   bash scripts/update-cost-snapshots.sh --no-clean
#
# When: after an intentional compute-unit / account-shape / ix-data change that
# should update the committed baselines in runtime-tests/cost-snapshots/.
# Writes: runtime-tests/cost-snapshots/{host,token}_mollusk.json
#
# Requires Solana CLI and Anchor versions matching CI
# (.github/workflows/solana-tests.yml). Override EXPECTED_SOLANA only for
# experiments; do not commit baselines minted under a divergent toolchain.
#
# Clears existing snapshot JSON before regenerating so orphaned profiles
# (renamed/deleted tests) cannot linger.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

print_help() {
  # Emit the leading `#` comment block; stop before the first non-comment line.
  awk 'NR == 1 { next } /^#/ { sub(/^# ?/, ""); print; next } { exit }' "$0"
}

CLEAN=1
for arg in "$@"; do
  case "$arg" in
    --no-clean) CLEAN=0 ;;
    -h|--help)
      print_help
      exit 0
      ;;
    *)
      echo "error: unknown argument: $arg" >&2
      echo "usage: bash scripts/update-cost-snapshots.sh [--no-clean]" >&2
      exit 1
      ;;
  esac
done

# Keep in lockstep with .github/workflows/solana-tests.yml SOLANA_VERSION.
EXPECTED_SOLANA="${EXPECTED_SOLANA:-2.1.0}"
EXPECTED_ANCHOR="$(awk -F'"' '/^anchor_version/ { print $2; exit }' Anchor.toml)"
if [[ -z "$EXPECTED_ANCHOR" ]]; then
  echo "error: could not read anchor_version from Anchor.toml" >&2
  exit 1
fi

require_cmd() {
  command -v "$1" >/dev/null || {
    echo "error: missing required command: $1" >&2
    exit 1
  }
}

require_cmd solana
require_cmd anchor
require_cmd cargo

solana_ver="$(solana --version)"
anchor_ver="$(anchor --version)"

# Require an exact version token after `solana-cli` (space or end), so e.g.
# 2.1.05 cannot satisfy EXPECTED_SOLANA=2.1.0.
case "$solana_ver" in
  "solana-cli ${EXPECTED_SOLANA}"|"solana-cli ${EXPECTED_SOLANA} "*) ;;
  *)
    echo "error: need Solana CLI ${EXPECTED_SOLANA} (got: ${solana_ver})" >&2
    echo "       match CI: .github/workflows/solana-tests.yml SOLANA_VERSION" >&2
    exit 1
    ;;
esac
if [[ "$anchor_ver" != "anchor-cli ${EXPECTED_ANCHOR}" ]]; then
  echo "error: need Anchor ${EXPECTED_ANCHOR} (got: ${anchor_ver})" >&2
  echo "       match CI / Anchor.toml anchor_version" >&2
  exit 1
fi

echo "toolchain ok: ${solana_ver}; ${anchor_ver}"

if [[ "$CLEAN" -eq 1 ]]; then
  echo "cargo clean (use --no-clean to skip after a trusted rebuild)"
  cargo clean
else
  echo "skipping cargo clean (--no-clean)"
fi

bash scripts/check-zama-host-idl.sh

SNAPSHOT_DIR="$ROOT/runtime-tests/cost-snapshots"
echo "clearing ${SNAPSHOT_DIR}/*.json so the regen output is exactly the live profiles"
rm -f "${SNAPSHOT_DIR}"/*.json

echo "updating cost snapshots..."
ZAMA_UPDATE_COST_SNAPSHOT=1 \
  cargo test -p zama-solana-runtime-tests cost_snapshot_ -- --nocapture

echo "updated: runtime-tests/cost-snapshots/host_mollusk.json"
echo "updated: runtime-tests/cost-snapshots/token_mollusk.json"
echo "review the JSON diff and commit it with the intentional CU change"
