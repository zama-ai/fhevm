#!/bin/bash
FHEVM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if [ ! -d "${FHEVM_DIR}/.github" ]; then
  echo "Error: invalid FHEVM repo root directory." >&2
  exit 1
fi

npm run build && ./codegen.mjs lib --overloads ./overloads/host-contracts.json --config ./codegen.host-contracts.config.json --verbose
