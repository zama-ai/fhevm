#!/bin/bash
FHEVM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if [ ! -d "${FHEVM_DIR}/.github" ]; then
  echo "Error: invalid FHEVM repo root directory." >&2
  exit 1
fi

npm run build && ./codegen.mjs lib --overloads ./overloads/e2e.json --config ./codegen.e2e.config.json --verbose 
