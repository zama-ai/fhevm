#!/usr/bin/env bash
set -euo pipefail

# Checks that all handles in handles.localhostFhevm.json are allowed for decryption
# on the ACL contract.
#
# Usage: ./scripts/check-handles-acl.sh [rpc-url]

RPC_URL="${1:-http://localhost:8545}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
HANDLES_JSON="${SCRIPT_DIR}/../test/fheTest/handles.localhostFhevm.json"

ACL_ADDRESS=$(node -e "const d = require('${HANDLES_JSON}'); console.log(d.aclAddress)")
HANDLES=$(node -e "const d = require('${HANDLES_JSON}'); d.handles.forEach(h => console.log(h.fheType + ' ' + h.bytes32Hex))")

echo "ACL address: ${ACL_ADDRESS}"
echo "RPC URL:     ${RPC_URL}"
echo ""

FAILED=0

while IFS=' ' read -r fheType handle; do
  result=$(cast call "${ACL_ADDRESS}" "isAllowedForDecryption(bytes32)(bool)" "${handle}" --rpc-url "${RPC_URL}")

  if [ "${result}" = "true" ]; then
    echo "  ✓ ${fheType}: ${handle}... isAllowedForDecryption=true"
  else
    echo "  ✗ ${fheType}: ${handle}... isAllowedForDecryption=FALSE"
    FAILED=1
  fi
done <<< "${HANDLES}"

echo ""
if [ "${FAILED}" -eq 1 ]; then
  echo "FAIL: Some handles are not allowed for decryption."
  exit 1
else
  echo "OK: All handles are allowed for decryption."
fi
