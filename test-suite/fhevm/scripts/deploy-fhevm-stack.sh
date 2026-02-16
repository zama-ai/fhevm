#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
LEGACY_DEPLOY_SCRIPT="${SCRIPT_DIR}/deploy-fhevm-stack.legacy.sh"
BUN_CLI="${SCRIPT_DIR}/bun/cli.ts"
CLI_IMPL="${FHEVM_CLI_IMPL:-bun}"

if [[ "$CLI_IMPL" == "legacy" ]]; then
  exec "$LEGACY_DEPLOY_SCRIPT" "$@"
fi

if ! command -v bun >/dev/null 2>&1; then
  echo "[WARN] bun runtime not found, falling back to legacy deploy script." >&2
  exec "$LEGACY_DEPLOY_SCRIPT" "$@"
fi

exec bun "$BUN_CLI" deploy "$@"
