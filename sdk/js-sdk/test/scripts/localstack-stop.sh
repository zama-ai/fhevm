#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FHEVM_DIR="$(cd "$SCRIPT_DIR/../../../../test-suite/fhevm" && pwd)"

# Shutdown fhevm
${FHEVM_DIR}/fhevm-cli down
