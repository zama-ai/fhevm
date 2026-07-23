#!/usr/bin/env bash
# deploy-demo-programs.sh — build + deploy the two confidential-vault demo programs (#1760).
#
# Zero-logic glue over `anchor build` + `solana program deploy`, mirroring
# setup-solana-side.sh's zama_host/confidential_token deploy exactly (explicit -k, --program-id from
# committed keypairs; --use-rpc). These two programs are deployed by NOTHING in the e2e vertical;
# the demo is their only deployer. Run AFTER clean-e2e.sh has the validator + stack up.
#
# Committed program keypairs (low-value, local-only) live under scripts/e2e/test-keypairs/ next to
# the other PoC program keys and pin each program id to its `declare_id!` (see that dir's README).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SOLANA="$ROOT/solana"
VALIDATOR_RPC="http://127.0.0.1:8899"
DEPLOYER_KEYPAIR="${SOLANA_DEPLOYER_KEYPAIR:-$HOME/.config/solana/id.json}"

echo "==> [demo-deploy] build + deploy demo_vault, confidential_batcher"
mkdir -p "$SOLANA/target/deploy"
# Seed the committed program keypairs so the built program ids match each declare_id! (-n keeps any
# pre-existing local key, exactly as setup-solana-side.sh does for zama_host/confidential_token).
for p in demo_vault confidential_batcher; do
  cp -n "$SOLANA/scripts/e2e/test-keypairs/$p-keypair.json" "$SOLANA/target/deploy/$p-keypair.json" 2>/dev/null || true
done

# Per-crate anchor build (--ignore-keys: keep the committed keypairs, do not regenerate ids).
( cd "$SOLANA" \
    && anchor build --ignore-keys --no-idl -p demo_vault \
    && anchor build --ignore-keys --no-idl -p confidential_batcher ) \
  || { echo "[demo-deploy] anchor build failed" >&2; exit 1; }

# --use-rpc: deploy over RPC 8899 (the TPU ports are not published). `solana program deploy`
# upgrades in place when the program already exists (deployer is the upgrade authority), so a
# re-run against an already-seeded stack redeploys idempotently rather than erroring.
for p in demo_vault confidential_batcher; do
  solana program deploy -u "$VALIDATOR_RPC" -k "$DEPLOYER_KEYPAIR" --use-rpc \
    --program-id "$SOLANA/target/deploy/$p-keypair.json" "$SOLANA/target/deploy/$p.so" >/dev/null
  echo "    $p=$(solana address -k "$SOLANA/target/deploy/$p-keypair.json") deployed"
done
