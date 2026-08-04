#!/usr/bin/env bash
# check-zama-host-idl.sh — rebuild SBF artifacts and verify host IDL/ABI goldens.
#
# Usage (from solana/):
#   bash scripts/check-zama-host-idl.sh
#
# When: before Mollusk runtime tests; what CI runs for IDL/ABI sync checks.
# Writes: target/deploy only (does not update goldens; see sync-zama-host-idl.sh).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT"
NO_DNA=1 anchor build --ignore-keys

python3 scripts/check_solana_abi.py --root "$ROOT"

# When solana-proof-service is present (stacked / vertical branches), keep its
# hand-decoded host instruction catalog partitioned against the vendored IDL.
python3 scripts/check_proof_store_idl.py --repo-root "$ROOT/.."

# Runtime Mollusk tests load ignored SBF artifacts from target/deploy, and the build above already
# produced them on the default feature set. There used to be a second `anchor build -p
# confidential_token -- --features poc` here to overwrite that artifact with a PoC-enabled one; the
# `poc` feature is gone, so Mollusk now runs against the same artifact that ships. Neither program
# has an alternate verification path.

# Event-version constants are runtime u8s stamped on protocol events. The ABI
# golden manifest (check_solana_abi.py above) pins both programs' versions from
# their constants.rs; the host-listener's decoded op records use
# zama_host::EVENT_VERSION directly, so no separate listener constant can drift.
