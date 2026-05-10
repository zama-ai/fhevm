#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
#
# Flow:
#   1. If Anvil is already running on RPC_URL, reuse it and run tests directly.
#   2. Otherwise start a fresh Anvil instance.
#   3. Deploy the cleartext FHEVM stack.
#   4. Run only test/fheTest/cleartext-{ethers|viem}
#   5. Tear down Anvil only if this script started it.
#
# ------------------------------------------------------------------------------

ETH_LIBRARY="${ETH_LIBRARY:-}"
if [[ "$ETH_LIBRARY" != "ethers" && "$ETH_LIBRARY" != "viem" ]]; then
    echo "Error: ETH_LIBRARY must be set to 'ethers' or 'viem' (got: '${ETH_LIBRARY}')." >&2
    exit 1
fi

# shellcheck source=_anvil-lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/_anvil-lib.sh"

anvil_setup_dirs
anvil_setup_vars
anvil_check_deps
anvil_check_scripts

REUSE_EXISTING_ANVIL=0
anvil_setup_cleanup

# ------------------------------------------------------------------------------
# Reuse existing Anvil or start a fresh one
# ------------------------------------------------------------------------------

if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
    REUSE_EXISTING_ANVIL=1
    echo "♻️  Reusing existing Anvil on $RPC_URL."
    echo "⏭️  Skipping Anvil startup and deployment."
else
    anvil_start_and_wait
    anvil_deploy_cleartext
fi

# ------------------------------------------------------------------------------
# Run cleartext tests
# ------------------------------------------------------------------------------

echo "🧪 Running cleartext ${ETH_LIBRARY} tests..."
(
    TEST_TARGET="test/fheTest/cleartext-${ETH_LIBRARY}"

    cd "$JS_SDK_DIR"
    CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts "$TEST_TARGET"
    #CHAIN=localhost npx vitest run --config test/fheTest/vitest-manual-packing.config.ts "$TEST_TARGET"
)

echo
echo "================================================================================"
echo "🎯  Foundry profile: ${FOUNDRY_PROFILE}"
echo "================================================================================"

echo
echo "✅ cleartext-${ETH_LIBRARY} tests passed."
