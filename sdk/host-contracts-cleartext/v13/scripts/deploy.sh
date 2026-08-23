#!/usr/bin/env bash
#
# Deploy a cleartext FHEVM stack with Foundry, from source, and verify it.
#
# Usage: ./scripts/deploy.sh [--rpc-url URL] [--mnemonic "..."] [--account-index N]
#                            [--private-key KEY] [--no-verify]
#
#   --rpc-url URL      node to deploy to (default: http://127.0.0.1:8545)
#   --mnemonic "..."   deployer mnemonic (default: the package's test mnemonic)
#   --account-index N  account within that mnemonic (default: 5)
#   --private-key KEY  deployer key, instead of --mnemonic/--account-index
#   --no-verify        skip the post-deploy verification
#
# The default mnemonic and account index 5 are what place the stack on the addresses
# library-solidity/config/ZamaConfig.sol hardcodes for chain 31337, so a dApp compiled against the FHE
# library's local config can find it. Change either and the whole stack moves somewhere else — which is
# fine for a real network, but not for local development.
#
# Three steps, in this order, because the addresses have to exist before the code that references them can
# be compiled:
#
#   1. ComputeAddresses.s.sol   reads the deployer's live nonce and writes the address set its CREATE
#                               sequence will produce, as a Solidity config
#   2. forge build              compiles pkg/src against that config, so every contract carries the real
#                               addresses of the stack it is about to be part of
#   3. FhevmDeployScript.s.sol  deploys it, checking each CREATE lands where step 1 predicted
#   4. VerifyFhevmDeploy.s.sol  reads the stack back and checks it against this package
#
# Nothing is ever patched: the bytecode is compiled for exactly one address set and deployed to exactly
# that set. Nothing else may send from the deployer between steps 1 and 3 — every address derives from the
# nonce read in step 1.
set -euo pipefail

PACKAGE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

RPC_URL="http://127.0.0.1:8545"
MNEMONIC='adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer'
ACCOUNT_INDEX=5
PRIVATE_KEY=""
VERIFY=1

while [ $# -gt 0 ]; do
    case "$1" in
        --rpc-url)
            [ $# -ge 2 ] || { echo "Error: --rpc-url requires a value." >&2; exit 1; }
            RPC_URL="$2"; shift 2 ;;
        --rpc-url=*) RPC_URL="${1#--rpc-url=}"; shift ;;
        --mnemonic)
            [ $# -ge 2 ] || { echo "Error: --mnemonic requires a value." >&2; exit 1; }
            MNEMONIC="$2"; shift 2 ;;
        --mnemonic=*) MNEMONIC="${1#--mnemonic=}"; shift ;;
        --account-index)
            [ $# -ge 2 ] || { echo "Error: --account-index requires a value." >&2; exit 1; }
            ACCOUNT_INDEX="$2"; shift 2 ;;
        --account-index=*) ACCOUNT_INDEX="${1#--account-index=}"; shift ;;
        --private-key)
            [ $# -ge 2 ] || { echo "Error: --private-key requires a value." >&2; exit 1; }
            PRIVATE_KEY="$2"; shift 2 ;;
        --private-key=*) PRIVATE_KEY="${1#--private-key=}"; shift ;;
        --no-verify) VERIFY=0; shift ;;
        -h | --help) sed -n '2,30p' "${BASH_SOURCE[0]}"; exit 0 ;;
        *) echo "Error: unknown argument '$1'. Try --help." >&2; exit 1 ;;
    esac
done

cd "$PACKAGE_ROOT"

command -v forge >/dev/null 2>&1 || { echo "Error: forge not on PATH (install foundry)." >&2; exit 1; }
command -v cast >/dev/null 2>&1 || { echo "Error: cast not on PATH (install foundry)." >&2; exit 1; }

# The forge scripts take a raw key (DEPLOYER_PRIVATE_KEY), so the mnemonic convenience lives here rather
# than in Solidity — one less thing for the scripts to resolve two different ways.
if [ -z "$PRIVATE_KEY" ]; then
    PRIVATE_KEY="$(cast wallet private-key --mnemonic "$MNEMONIC" --mnemonic-index "$ACCOUNT_INDEX")"
fi
DEPLOYER="$(cast wallet address --private-key "$PRIVATE_KEY")"

# The generated config, and the build that uses it. Kept out of the project's own out/ so a real-address
# build never becomes an input to `npm run build:templates`, which reads artifacts from out/. Kept out of
# internal/placeholders/ because that is the committed marker config the templates and the shipped
# pkg/forge bytecode are built from — overwriting it would silently corrupt both.
CONFIG_DIR="internal/.deploy-config"
BUILD_OUT="$CONFIG_DIR/out"
# Must match remappings.txt and FHEVM_CONFIG_REMAPPING_PREFIX in internal/constants.ts.
CONFIG_PREFIX="fhevm-config-0.13.0/"

export DEPLOYER_PRIVATE_KEY="$PRIVATE_KEY"

START_NONCE="$(cast nonce "$DEPLOYER" --rpc-url "$RPC_URL")"

# ACLOwner is created at start nonce + 12, right after PauserSet at +11 (see the nonce layout in
# pkg/forge/src/../script/ComputeAddresses.s.sol). Computed here rather than read back off the chain: an
# "expected" value fetched from the thing under test always matches, which is not a check.
EXPECTED_ACL_OWNER="$(cast compute-address "$DEPLOYER" --nonce "$((START_NONCE + 12))" | awk '{print $NF}')"

echo "🔗 rpc      $RPC_URL"
echo "👤 deployer $DEPLOYER (nonce $START_NONCE)"
echo ""

# ---------------------------------------------------------------------------
echo "==> 1/3 computing addresses"
rm -f "$CONFIG_DIR/addresses.sol"
# Built against the committed placeholder config: this script imports no address constants, so the
# default remapping keeps step 1 independent of its own output.
forge script pkg/forge/script/ComputeAddresses.s.sol:ComputeAddresses \
    --rpc-url "$RPC_URL" --out "$BUILD_OUT"

# `forge script` can report success for a run that reverted, and a missing file here would send step 2
# into a stale build. Check the artifact itself rather than the exit code.
[ -f "$CONFIG_DIR/addresses.sol" ] || { echo "Error: step 1 wrote no $CONFIG_DIR/addresses.sol." >&2; exit 1; }

# ---------------------------------------------------------------------------
# From here on the contracts must see the generated config. FOUNDRY_REMAPPINGS overrides just this one
# prefix and leaves the rest (openzeppelin, forge-std) to be discovered as usual, so remappings.txt is
# never edited and there is no restore-on-failure to get wrong.
export FOUNDRY_REMAPPINGS="${CONFIG_PREFIX}=${CONFIG_DIR}/"

echo ""
echo "==> 2/3 compiling the stack against those addresses"
forge build --out "$BUILD_OUT" --skip test

# Prove the config took effect rather than trusting it: no placeholder marker may survive. Without this a
# silently-ignored remapping would deploy markers as if they were real addresses.
ACL_MARKER="$(sed -n 's/.*ACL_ADDRESS = address(0x\([0-9a-fA-F]*\)).*/\1/p' internal/placeholders/addresses.sol)"
if [ -n "$ACL_MARKER" ] && grep -qi "$ACL_MARKER" "$BUILD_OUT/ACL.sol/ACL.json"; then
    echo "Error: placeholder marker 0x$ACL_MARKER survived the build — the remapping did not take." >&2
    exit 1
fi

echo ""
echo "==> 3/3 deploying"
forge script pkg/forge/script/FhevmDeployScript.s.sol:FhevmDeployScript \
    --rpc-url "$RPC_URL" --out "$BUILD_OUT" --broadcast

if [ "$VERIFY" -eq 1 ]; then
    echo ""
    echo "==> verifying"
    # Reads the chain back and checks it against this package. Separate from the deploy on purpose: the
    # deploy's own requires compare what it just did against the same constants it used, so they cannot
    # catch a stack built from a stale config. Reverts non-zero if anything is wrong.
    ACL_OWNER_ADDRESS="$EXPECTED_ACL_OWNER" \
    UPGRADE_ADMIN_ADDRESS="$DEPLOYER" \
        forge script pkg/forge/script/VerifyFhevmDeploy.s.sol:VerifyFhevmDeploy \
            --rpc-url "$RPC_URL" --out "$BUILD_OUT"
fi

echo ""
echo "✅ done"
