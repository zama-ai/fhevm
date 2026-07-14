#!/usr/bin/env bash
# sync-zama-host-idl.sh — rebuild programs and write IDL/ABI golden snapshots.
#
# Usage (from solana/):
#   bash scripts/sync-zama-host-idl.sh
#
# When: after an intentional host/token IDL or ABI change that should update
# the vendored listener snapshot and ABI manifest.
# Writes: checked-in IDL/ABI goldens (via check_solana_abi.py --write).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT"
NO_DNA=1 anchor build --ignore-keys
python3 scripts/check_solana_abi.py --root "$ROOT" --write
echo "Synced Solana IDLs and ABI golden manifest"
