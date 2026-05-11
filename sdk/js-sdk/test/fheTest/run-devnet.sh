#!/usr/bin/env bash
#
# run-devnet.sh
#
# Runs all ethers/*.test.ts and viem/*.test.ts with CHAIN=devnet.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR"

CHAIN=devnet npx vitest run \
  --config test/fheTest/vitest.config.ts \
  --exclude '**/**.slow.test.ts' \
  test/fheTest/ethers \
  test/fheTest/viem
