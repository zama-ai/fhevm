#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FHEVM_DIR="$(cd "$SCRIPT_DIR/../../../../test-suite/fhevm" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/../../contracts" && pwd)"

# Make sure the fhevm-cli is ready to go
cd ${FHEVM_DIR}
bun install 

# Stop
${FHEVM_DIR}/fhevm-cli down

# fail fast if port 8545 is already in use
if lsof -nP -iTCP:8545 -sTCP:LISTEN >/dev/null 2>&1; then
    echo "" >&2
    echo "========================================" >&2
    echo "❌ Port 8545 is already in use:" >&2
    echo "" >&2
    lsof -nP -iTCP:8545 -sTCP:LISTEN >&2
    echo "" >&2
    echo "❌ Stop the process listening on 8545 before restarting the local stack." >&2
    echo "========================================" >&2
    echo "" >&2
    exit 1
fi

# Start
${FHEVM_DIR}/fhevm-cli up

# Deploy FHETest.sol
cd ${CONTRACTS_DIR}

forge clean

./scripts/fhetest-deploy.sh --chain localstack
