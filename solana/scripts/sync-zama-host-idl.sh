#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT"
NO_DNA=1 anchor build --ignore-keys
python3 scripts/check_solana_abi.py --root "$ROOT" --write
echo "Synced Solana IDLs and ABI golden manifest"
