#!/usr/bin/env bash
# =============================================================================
# fhevm-anvil-fund.sh — Start a local anvil node and run FundAddresses.s.sol.
#
#   1. Launch anvil in the background on the chosen port.
#   2. Wait for the JSON-RPC endpoint to become ready.
#   3. Invoke `forge script FundAddresses.s.sol --broadcast` against it.
#   4. Keep anvil running in the foreground; Ctrl-C tears it down cleanly.
#
# FundAddresses.s.sol resolves the deployer and emptyUupsDeployer from env
# (DEPLOYER_PRIVATE_KEY / EMPTY_UUPS_DEPLOYER_PRIVATE_KEY) or — if unset —
# derives them from FHEVM_HOST_CONTRACTS_MNEMONIC as defined in Constants.sol.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

# ==============================================================================
# Unset CHAIN environment variable
# ==============================================================================

[[ -n "${CHAIN:-}" ]] && echo "Warning: unsetting CHAIN=$CHAIN" >&2; unset CHAIN

# ==============================================================================

DEPLOYER_MNEMONIC="adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"
DEPLOYER_MNEMONIC_DERIVATION_PREFIX="m/44'/60'/0'/0/"
DEPLOYER_MNEMONIC_DERIVATION_INDEX="5"
UUPS_DEPLOYER_MNEMONIC_DERIVATION_INDEX="4"

DEPLOYER_PRIVATE_KEY="$(cast wallet private-key --mnemonic "$DEPLOYER_MNEMONIC" --mnemonic-derivation-path "${DEPLOYER_MNEMONIC_DERIVATION_PREFIX}${DEPLOYER_MNEMONIC_DERIVATION_INDEX}")"
DEPLOYER_ADDRESS="$(cast wallet address --mnemonic "$DEPLOYER_MNEMONIC" --mnemonic-derivation-path "${DEPLOYER_MNEMONIC_DERIVATION_PREFIX}${DEPLOYER_MNEMONIC_DERIVATION_INDEX}")"

UUPS_DEPLOYER_PRIVATE_KEY="$(cast wallet private-key --mnemonic "$DEPLOYER_MNEMONIC" --mnemonic-derivation-path "${DEPLOYER_MNEMONIC_DERIVATION_PREFIX}${UUPS_DEPLOYER_MNEMONIC_DERIVATION_INDEX}")"
UUPS_DEPLOYER_ADDRESS="$(cast wallet address --mnemonic "$DEPLOYER_MNEMONIC" --mnemonic-derivation-path "${DEPLOYER_MNEMONIC_DERIVATION_PREFIX}${UUPS_DEPLOYER_MNEMONIC_DERIVATION_INDEX}")"

# ==============================================================================

# Load the test-user mnemonic from sdk/js-sdk/test/.env so the same identity
# used by the js-sdk test suite signs `initFheTest` here.
FHE_TEST_ENV_FILE="$SCRIPT_DIR/../../test/.env"
if [[ ! -f "$FHE_TEST_ENV_FILE" ]]; then
    echo "Error: $FHE_TEST_ENV_FILE not found — expected MNEMONIC there for FHE_TEST_USER." >&2
    exit 1
fi
# shellcheck disable=SC1090
source "$FHE_TEST_ENV_FILE"
if [[ -z "${MNEMONIC:-}" ]]; then
    echo "Error: MNEMONIC not defined in $FHE_TEST_ENV_FILE." >&2
    exit 1
fi

FHE_TEST_USER_PRIVATE_KEY="$(cast wallet private-key --mnemonic "test test test test test test test future home engine virtual motion" --mnemonic-derivation-path "m/44'/60'/0'/0/0")"
FHE_TEST_USER_ADDRESS="$(cast wallet address --mnemonic "test test test test test test test future home engine virtual motion" --mnemonic-derivation-path "m/44'/60'/0'/0/0")"

# ==============================================================================

RPC_URL="http://127.0.0.1:8545"

if ! cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
    echo "Error: nothing is listening on $RPC_URL. Start Anvil or use another --rpc-url." >&2
    exit 1
fi

# ==============================================================================
#
# ---- Fund deployer addresses via anvil_setBalance ----
#
# ==============================================================================

DEPLOYER_FUND_WEI="$(cast to-hex "$(cast to-wei 20000 ether)")" # 20_000 ETH

echo "🍟 Funding ${DEPLOYER_ADDRESS} with 20_000 ETH"
cast rpc anvil_setBalance "$DEPLOYER_ADDRESS" "$DEPLOYER_FUND_WEI" --rpc-url "$RPC_URL" >/dev/null

echo "🍟 Funding ${FHE_TEST_USER_ADDRESS} with 20_000 ETH"
cast rpc anvil_setBalance "$FHE_TEST_USER_ADDRESS" "$DEPLOYER_FUND_WEI" --rpc-url "$RPC_URL" >/dev/null

echo
echo "💰 Balances after funding:"
printf "  %-20s %s : %s ETH\n" "deployer"          "$DEPLOYER_ADDRESS"        "$(cast from-wei "$(cast balance "$DEPLOYER_ADDRESS"      --rpc-url "$RPC_URL")")"
printf "  %-20s %s : %s ETH\n" "deployer"          "$FHE_TEST_USER_ADDRESS"   "$(cast from-wei "$(cast balance "$FHE_TEST_USER_ADDRESS"      --rpc-url "$RPC_URL")")"

# ==============================================================================
#
# ---- Deploy the FHEVM host contracts ----
#
# ==============================================================================

echo
echo "🏗️  Running DeployFHEVMHost.s.sol against ${RPC_URL}..."
forge script script/DeployFHEVMHost.s.sol:DeployFHEVMHost \
    --rpc-url "$RPC_URL" \
    --broadcast \
    --non-interactive

# ==============================================================================
#
# ---- Install FhevmCheats on the live anvil ----
#
# `DeployFHEVMHost.s.sol` installs FhevmCheats via `vm.etch(...)` which is a
# simulation-only cheatcode — it never broadcasts, so the live anvil has no
# code at FHEVM_CHEATS_ADDRESS after `--broadcast` finishes. Plant the runtime
# bytecode directly via `anvil_setCode`, then populate storage by calling
# `setAll(...)` as a real tx.
# ==============================================================================

if ! command -v jq >/dev/null 2>&1; then
    echo "Error: jq not found in PATH (needed to read forge artifacts)." >&2
    exit 1
fi

FHEVM_CHEATS_ADDRESS="0xC71923396eE5fFc886cb769aC7841b8d8d94DD50"

CHEATS_ARTIFACT="out/FhevmCheats.sol/FhevmCheats.json"
if [[ ! -f "$CHEATS_ARTIFACT" ]]; then
    echo "Error: $CHEATS_ARTIFACT not found — did forge build run?" >&2
    exit 1
fi
CHEATS_RUNTIME="$(jq -r '.deployedBytecode.object // .deployedBytecode' "$CHEATS_ARTIFACT")"

echo
echo "🧩 Etching FhevmCheats runtime at ${FHEVM_CHEATS_ADDRESS}..."
cast rpc anvil_setCode "$FHEVM_CHEATS_ADDRESS" "$CHEATS_RUNTIME" --rpc-url "$RPC_URL" >/dev/null

# Host addresses are baked into src/host-contracts/addresses/FHEVMHostAddresses.sol
# as `address constant <name> = address(0x...)`.
extract_fhevm_addr() {
    local name="$1"
    local src="src/host-contracts/addresses/FHEVMHostAddresses.sol"
    sed -nE "s/^[[:space:]]*address[[:space:]]+constant[[:space:]]+${name}[[:space:]]*=[[:space:]]*address\(([^)]+)\).*/\1/p" "$src"
}
ACL_ADD="$(extract_fhevm_addr aclAdd)"
FHEVM_EXECUTOR_ADD="$(extract_fhevm_addr fhevmExecutorAdd)"
KMS_VERIFIER_ADD="$(extract_fhevm_addr kmsVerifierAdd)"
INPUT_VERIFIER_ADD="$(extract_fhevm_addr inputVerifierAdd)"
HCU_LIMIT_ADD="$(extract_fhevm_addr hcuLimitAdd)"
PAUSER_SET_ADD="$(extract_fhevm_addr pauserSetAdd)"

# FHETest is deployed at deployer nonce 19 by DeployFHEVMHost. Pull the actual
# address from the forge broadcast log rather than recomputing, so we stay
# correct even if the deploy sequence changes.
CHAIN_ID_DEC="$(($(cast chain-id --rpc-url "$RPC_URL")))"
BROADCAST_LOG="broadcast/DeployFHEVMHost.s.sol/${CHAIN_ID_DEC}/run-latest.json"
if [[ ! -f "$BROADCAST_LOG" ]]; then
    echo "Error: $BROADCAST_LOG not found — forge broadcast didn't write a log for chain ${CHAIN_ID_DEC}." >&2
    exit 1
fi
FHE_TEST_ADD="$(jq -r '[.transactions[] | select(.contractName == "FHETest") | .contractAddress] | last // ""' "$BROADCAST_LOG")"
if [[ -z "$FHE_TEST_ADD" || "$FHE_TEST_ADD" == "null" ]]; then
    echo "Error: FHETest contract address not found in $BROADCAST_LOG." >&2
    exit 1
fi

echo "🧩 Populating FhevmCheats.setAll(...)"
cast send "$FHEVM_CHEATS_ADDRESS" \
    "setAll((address,address,address,address,address,address),address)" \
    "(${ACL_ADD},${FHEVM_EXECUTOR_ADD},${KMS_VERIFIER_ADD},${INPUT_VERIFIER_ADD},${HCU_LIMIT_ADD},${PAUSER_SET_ADD})" \
    "$FHE_TEST_ADD" \
    --private-key "$DEPLOYER_PRIVATE_KEY" --rpc-url "$RPC_URL" >/dev/null

# ==============================================================================
#
# ---- Retrieve FHETest address from FhevmCheats and initialize it ----
#
# ==============================================================================

echo
echo "🔎 Reading FHETest address from FhevmCheats at ${FHEVM_CHEATS_ADDRESS}..."
FHE_TEST_ADDRESS="$(cast call "$FHEVM_CHEATS_ADDRESS" "fheTest()(address)" --rpc-url "$RPC_URL")"
if [[ -z "$FHE_TEST_ADDRESS" || "$FHE_TEST_ADDRESS" == "0x0000000000000000000000000000000000000000" ]]; then
    echo "Error: FhevmCheats.fheTest() returned the zero address — did DeployFHEVMHost succeed?" >&2
    exit 1
fi
echo "  fheTest: $FHE_TEST_ADDRESS"

echo
echo "🚀 Calling FHETest(${FHE_TEST_ADDRESS}).initFheTest(true)..."
cast send "$FHE_TEST_ADDRESS" "initFheTest(bool)" true \
    --private-key "$DEPLOYER_PRIVATE_KEY" \
    --rpc-url "$RPC_URL" >/dev/null
echo "✅ initFheTest complete. ${DEPLOYER_ADDRESS}"

echo
echo "🚀 Calling FHETest(${FHE_TEST_ADDRESS}).initFheTest(true)..."
cast send "$FHE_TEST_ADDRESS" "initFheTest(bool)" true \
    --private-key "$FHE_TEST_USER_PRIVATE_KEY" \
    --rpc-url "$RPC_URL" >/dev/null
echo "✅ initFheTest complete. ${FHE_TEST_USER_ADDRESS}"

echo
echo "✅ FHEVM stack deployed and initialized on ${RPC_URL} (chain-id: $(cast chain-id --rpc-url "$RPC_URL"))."
