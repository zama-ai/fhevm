#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FHEVM_DIR="$(cd "$SCRIPT_DIR/../../../test-suite/fhevm" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/../contracts" && pwd)"

echo $SCRIPT_DIR
echo $FHEVM_DIR

# Stop
${FHEVM_DIR}/fhevm-cli down

# Start
${FHEVM_DIR}/fhevm-cli up

# Deploy FHETest.sol
cd ${CONTRACTS_DIR}
forge clean
forge script script/DeployFHETest.s.sol --rpc-url http://localhost:8545 --broadcast

# Init FHETest.sol (broadcast init transactions)
cd ${CONTRACTS_DIR}
forge script script/InitFHETest.s.sol --rpc-url http://localhost:8545 --broadcast

# Export handles JSON (reads committed on-chain state after init transactions are mined)
${SCRIPT_DIR}/export-handles.sh

# Verify all exported handles are allowed for decryption on the ACL contract
${SCRIPT_DIR}/check-handles-acl.sh