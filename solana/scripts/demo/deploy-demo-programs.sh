#!/usr/bin/env bash
# Demo build and local key fixtures; deployment uses the packaged CLI entry point.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SOLANA="$ROOT/solana"
VALIDATOR_RPC="${SOLANA_RPC_URL:?missing validator RPC URL (the lifecycle sets SOLANA_RPC_URL)}"
DEPLOYER_KEYPAIR="${SOLANA_DEPLOYER_KEYPAIR:-$HOME/.config/solana/id.json}"

echo "==> [demo-deploy] build + deploy confidential_token, demo_vault, confidential_batcher"
# The validator loaded these programs at genesis (src/solana/validator.ts genesisDeployedPrograms)
# with the deployer wallet as upgrade authority, so this is a bytecode check on a fresh stack and an
# in-place upgrade after an edit.
bash "$SOLANA/scripts/build-programs.sh" preview-env confidential_token demo_vault confidential_batcher
SOLANA_RPC_URL="$VALIDATOR_RPC" \
SOLANA_DEPLOYER_KEYPAIR="$DEPLOYER_KEYPAIR" \
SOLANA_ARTIFACTS_DIR="$SOLANA/target/deploy" \
ADDRESSES_DIR="$SOLANA/target/deploy" \
bun run "$ROOT/solana/deploy/src/cli.ts" demos "${1:-deploy}"
