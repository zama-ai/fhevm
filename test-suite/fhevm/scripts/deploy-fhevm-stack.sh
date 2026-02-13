#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
FHEVM_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
LEGACY_DEPLOY_SCRIPT="${SCRIPT_DIR}/deploy-fhevm-stack.legacy.sh"
BUN_CLI="${SCRIPT_DIR}/bun/cli.ts"
CLI_IMPL="${FHEVM_CLI_IMPL:-bun}"

if [[ "$CLI_IMPL" == "legacy" ]]; then
  exec "$LEGACY_DEPLOY_SCRIPT" "$@"
fi

if ! command -v bun >/dev/null 2>&1; then
  echo "[ERROR] bun runtime is required for deploy-fhevm-stack.sh. Install bun or run with FHEVM_CLI_IMPL=legacy." >&2
  exit 1
fi

exec bun "$BUN_CLI" deploy "$@"
