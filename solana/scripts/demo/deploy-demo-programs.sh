#!/usr/bin/env bash
# Demo build and local key fixtures; deployment uses the packaged CLI entry point.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SOLANA="$ROOT/solana"
VALIDATOR_RPC="http://127.0.0.1:8899"
DEPLOYER_KEYPAIR="${SOLANA_DEPLOYER_KEYPAIR:-$HOME/.config/solana/id.json}"

echo "==> [demo-deploy] build + deploy demo_vault, confidential_batcher"
mkdir -p "$SOLANA/target/deploy"
# Seed the committed program keypairs so the built program ids match each declare_id!. Always
# overwrite target artifacts left by older branches, as src/solana/validator.ts does for host programs.
for p in demo_vault confidential_batcher; do
  cp -f "$SOLANA/scripts/e2e/test-keypairs/$p-keypair.json" "$SOLANA/target/deploy/$p-keypair.json"
done

bash "$SOLANA/scripts/build-programs.sh" localnet confidential_token demo_vault confidential_batcher
if [[ -z "${SOLANA_DEPLOY_DATABASE_URL:-}" ]]; then
  SOLANA_DEPLOY_DATABASE_URL=$(cd "$ROOT/test-suite/fhevm" && bun -e 'import { readCoprocessorDatabaseUrl } from "./src/solana/deploy"; process.stdout.write(await readCoprocessorDatabaseUrl());')
fi
SOLANA_DEPLOY_DATABASE_URL="$SOLANA_DEPLOY_DATABASE_URL" \
SOLANA_RPC_URL="$VALIDATOR_RPC" \
SOLANA_DEPLOYER_KEYPAIR="$DEPLOYER_KEYPAIR" \
SOLANA_ARTIFACTS_DIR="$SOLANA/target/deploy" \
SOLANA_PROGRAM_PROFILE=localnet \
ADDRESSES_DIR="$SOLANA/target/deploy" \
bun run "$ROOT/test-suite/fhevm/src/solana/host-deploy/cli.ts" demos "${1:-deploy}"
