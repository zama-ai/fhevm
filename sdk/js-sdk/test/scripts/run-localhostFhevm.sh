#!/usr/bin/env bash


set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR"

NO_RESTART=false
for arg in "$@"; do
  case "$arg" in
    --no-restart) NO_RESTART=true ;;
    --help)
      echo "Usage: $(basename "$0") [--no-restart]"
      echo ""
      echo "Options:"
      echo "  --no-restart  Skip fhevm:restart before and fhevm:stop after running tests"
      echo "  --help        Show this help message"
      exit 0
      ;;
  esac
done

if [ "$NO_RESTART" = false ]; then
  npm run fhevm:restart
fi

export CHAIN=localhostFhevm

# First check configuiration
if ! npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem/clientBase.chain.test.ts; then
  echo ""
  echo "❌ ERROR: update test/fheTest/chains/localhostFhevm.ts config file ❌"
  echo ""
  exit 1
fi

npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem
npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/ethers 

if [ "$NO_RESTART" = false ]; then
  npm run fhevm:stop
fi
