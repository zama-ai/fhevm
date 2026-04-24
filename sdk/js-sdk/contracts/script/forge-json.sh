#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
RPC_URL="${RPC_URL:-http://localhost:8545}"

cd "$CONTRACTS_DIR"

forge script "$@" --rpc-url "$RPC_URL" 2>&1 | awk '
  /JSON_RESULT_START/ { capture=1; next }
  /JSON_RESULT_END/   { capture=0; exit }
  capture
'
