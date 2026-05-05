#!/usr/bin/env bash
set -euo pipefail

# Exports the FHETest ABI from the forge build output to abi-v2.ts.
# Verifies that FHETest is deployed at the expected address and that the
# on-chain bytecode matches the locally compiled version.
#
# Usage: ./scripts/export-abi.sh [rpc-url]

RPC_URL="${1:-http://localhost:8545}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/../contracts" && pwd)"
ABI_FILE="${SCRIPT_DIR}/../test/fheTest/abi-v2.ts"
ARTIFACT="${CONTRACTS_DIR}/out/FHETest.sol/FHETest.json"
FHETEST_ADDRESS="0x61a8e950161daCCA06b0b623Efa2b7237BaBD4a0"

# Ensure forge has compiled
cd "${CONTRACTS_DIR}"
forge build --silent

if [ ! -f "${ARTIFACT}" ]; then
  echo "Error: ${ARTIFACT} not found. Run 'forge build' first."
  exit 1
fi

# Verify contract is deployed
ON_CHAIN_CODE=$(cast code "${FHETEST_ADDRESS}" --rpc-url "${RPC_URL}")
if [ "${ON_CHAIN_CODE}" = "0x" ] || [ -z "${ON_CHAIN_CODE}" ]; then
  echo "Error: No contract deployed at ${FHETEST_ADDRESS}"
  echo "Run: forge script script/DeployFHETest.s.sol --rpc-url ${RPC_URL} --broadcast"
  exit 1
fi

# Verify on-chain bytecode matches locally compiled version
LOCAL_DEPLOYED_CODE=$(node -e "
  const a = JSON.parse(require('fs').readFileSync('${ARTIFACT}', 'utf8'));
  console.log(a.deployedBytecode.object);
")

if [ "${ON_CHAIN_CODE}" != "${LOCAL_DEPLOYED_CODE}" ]; then
  echo "Error: On-chain bytecode at ${FHETEST_ADDRESS} does not match the locally compiled FHETest."
  echo "The contract needs to be redeployed."
  echo "Run: forge script script/DeployFHETest.s.sol --rpc-url ${RPC_URL} --broadcast"
  exit 1
fi

echo "  FHETest verified at: ${FHETEST_ADDRESS}"

# Extract ABI, write to abi-v2.ts
node -e "
const fs = require('fs');
const artifact = JSON.parse(fs.readFileSync('${ARTIFACT}', 'utf8'));
const abi = JSON.stringify(artifact.abi, null, 2);

const ts = \`export const FHETestAddresses = {
  localhostFhevm: '${FHETEST_ADDRESS}',
};

export const FHETestABI = \${abi} as const;
\`;

fs.writeFileSync('${ABI_FILE}', ts);
"

echo "Exported ABI to: ${ABI_FILE}"
