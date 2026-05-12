#!/usr/bin/env bash
#
# For nonces 0..20, compute the CREATE address deployed from DEPLOYER and
# check whether its lowercase form appears (case-sensitive) in FILE.

set -euo pipefail

DEPLOYER="0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4"
#FILE="/Users/alex/po/host-contracts-0.10.0/node_modules/@fhevm/host-contracts/artifacts/contracts/FHEVMExecutor.sol/FHEVMExecutor.json"
#FILE="/Users/alex/src/me/zama-ai/fhevm/host-contracts/artifacts/contracts/FHEVMExecutor.sol/FHEVMExecutor.json"
FILE="/Users/alex/src/me/zama-ai/fhevm/host-contracts/artifacts/contracts/ACL.sol/ACL.json"

if [ ! -f "$FILE" ]; then
  echo "File not found: $FILE" >&2
  exit 1
fi

for i in $(seq 0 20); do
  addr=$(cast compute-address "$DEPLOYER" --nonce "$i" | grep -oE '0x[0-9a-fA-F]{40}')
  lower=$(echo "$addr" | tr '[:upper:]' '[:lower:]')
  stripped="${lower#0x}"
  if grep -q "$stripped" "$FILE"; then
    printf 'nonce=%-2d  %s  FOUND\n' "$i" "$stripped"
  else
    printf 'nonce=%-2d  %s  not found\n' "$i" "$stripped"
  fi
done
