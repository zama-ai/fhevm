#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/../contracts" && pwd)"

RPC_URL="${RPC_URL:-http://localhost:8545}"

cd "$CONTRACTS_DIR"

forge clean

#
# Step 1: Deploy (skips if already deployed, funds if needed, verifies config)
#
echo "Running deploy script..."
forge script script/DeployFHETest.s.sol \
  --rpc-url "$RPC_URL" \
  --broadcast

echo ""

#
# Step 2: Run tests against deployed contract
#
echo "Running tests..."
forge test \
  --rpc-url "$RPC_URL" \
  -vv

# forge script script/DeployFHETest.s.sol --rpc-url http://localhost:8545
# forge test test/FHETest.t.sol --rpc-url http://localhost:8545