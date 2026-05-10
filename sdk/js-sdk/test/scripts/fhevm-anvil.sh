#!/usr/bin/env bash
set -euo pipefail

# shellcheck source=_anvil-lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/_anvil-lib.sh"

anvil_setup_dirs
anvil_setup_vars
anvil_check_deps
anvil_check_scripts
anvil_setup_cleanup

# ------------------------------------------------------------------------------
# Fail if something is already listening
# ------------------------------------------------------------------------------

if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
    echo "Error: something is already listening on $RPC_URL. Stop it or use another PORT/RPC_URL." >&2
    exit 1
fi

anvil_start_and_wait
anvil_deploy_cleartext

echo
echo "================================================================================"
echo "🎯  Foundry profile: ${FOUNDRY_PROFILE}"
echo "================================================================================"

echo
echo "✅ FHEVM Cleartext stack deployed and initialized on ${RPC_URL} (chain-id: $(cast chain-id --rpc-url "$RPC_URL"))."
echo "anvil is running as PID ${ANVIL_PID}. Press Ctrl-C to stop."
wait "$ANVIL_PID"
