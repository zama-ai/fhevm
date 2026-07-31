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
# The demo dapp owns the IDLs of the two demo-only programs (they are not part of the
# listener-ingest ABI manifest above; the SDK codegen renders their clients into the dapp —
# see sdk/js-sdk/scripts/build/codegen-solana-confidential-token.mjs).
cp target/idl/confidential_batcher.json target/idl/demo_vault.json "$ROOT/demo-dapp/idl/"
echo "Synced Solana IDLs and ABI golden manifest"
