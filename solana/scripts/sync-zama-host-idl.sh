#!/usr/bin/env bash
# sync-zama-host-idl.sh — rebuild programs and write IDL/ABI golden snapshots.
#
# Usage (from solana/):
#   bash scripts/sync-zama-host-idl.sh
#
# When: after an intentional IDL or ABI change to any of the four programs. This is
# the only way a committed IDL should ever change — they are build output, and every
# one of them is compared with a fresh build by check-zama-host-idl.sh in CI.
# Writes: checked-in IDL/ABI goldens (via check_solana_abi.py --write).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT"

# The goldens this writes are compared byte for byte against a fresh `anchor build` in CI, so
# minting them under a different Anchor than CI runs produces a diff that looks like a real
# IDL change and fails the next unrelated PR. Keep in lockstep with solana-tests.yml.
EXPECTED_SOLANA="${EXPECTED_SOLANA:-4.1.2}"
. "$(dirname "${BASH_SOURCE[0]}")/lib/require-pinned-toolchain.sh"

NO_DNA=1 anchor build --ignore-keys
# Writes all four vendored IDLs, including the two demo-only programs whose copies live
# with the dapp that consumes their generated clients. The list they come from is
# check_solana_abi.py's, the same one its check reads, so a copy that nothing compares
# cannot exist.
python3 scripts/check_solana_abi.py --root "$ROOT" --write
echo "Synced Solana IDLs and ABI golden manifest"
