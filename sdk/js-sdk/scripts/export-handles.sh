#!/usr/bin/env bash
set -euo pipefail

# Exports FHETest handles from the local chain to handles.localhostFhevm.json.
# Reads committed on-chain state — run after InitFHETest.s.sol transactions are mined.
#
# Usage: ./scripts/export-handles.sh [rpc-url]

RPC_URL="${1:-http://localhost:8545}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/../contracts" && pwd)"
HANDLES_JSON="${SCRIPT_DIR}/../test/fheTest/handles.localhostFhevm.json"

# Export handles via forge script
cd "${CONTRACTS_DIR}"
forge script scripts/ExportFHETestHandles.s.sol --rpc-url "${RPC_URL}"

# Post-process: convert handles from object with numeric keys to array
# (forge's vm.serializeString with numeric keys produces an object, not an array)
node -e "
  const fs = require('fs');
  const data = JSON.parse(fs.readFileSync('${HANDLES_JSON}', 'utf8'));
  if (data.handles && !Array.isArray(data.handles)) {
    data.handles = Object.keys(data.handles)
      .sort((a, b) => Number(a) - Number(b))
      .map(k => data.handles[k]);
  }
  fs.writeFileSync('${HANDLES_JSON}', JSON.stringify(data, null, 2) + '\n');
"

echo "Exported handles to: ${HANDLES_JSON}"
