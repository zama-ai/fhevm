#!/usr/bin/env bash
# Regenerate fixed PDA addresses and bumps using the programs' Rust helpers.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# Each crate writes its own section of the shared fixture; keep the writes sequential.
for program in zama-host confidential-token confidential-batcher demo-vault dep-chain encrypted-counter; do
  ZAMA_UPDATE_PDA_VECTORS=1 cargo test -p "$program" pda_golden -- --nocapture
  cargo test -p "$program" pda_golden
done
