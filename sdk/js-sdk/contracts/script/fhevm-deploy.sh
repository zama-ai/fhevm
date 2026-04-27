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

# load utils (fhevm_assert_chain, fhevm_rpc_url, is_anvil, ...)
source "$SCRIPT_DIR/fhevm-lib.sh"

# ==============================================================================
# CLI options
# ==============================================================================

chain_default="localhost"
chain_cli=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --chain)
            chain_cli="${2:?--chain requires a value}"
            shift 2
            ;;
        -h|--help)
            cat <<EOF
Usage: fhevm-deploy.sh [options]

Options:
  --chain <name>        FHEVM chain (mainnet | testnet | devnet | localhost | localhostFhevm) [default: localhost].
                        Precedence: --chain flag > \$CHAIN env > $chain_default
  -h, --help            Show this help.
EOF
            exit 0
            ;;
        *)
            echo "Error: unknown option: $1" >&2
            exit 1
            ;;
    esac
done

# Resolve chain. Read $CHAIN BEFORE unsetting it below (the unset is for
# downstream forge invocations that would otherwise pick CHAIN up from env).
chain="${chain_cli:-${CHAIN:-$chain_default}}"
fhevm_assert_chain "$chain"

# only chain="localhost" for the moment
case "$chain" in
    localhost) ;;
    *)
        echo "❌ fhevm-deploy.sh only supports chain=localhost; got '$chain'" >&2
        exit 1
        ;;
esac

# Derive RPC URL from the resolved chain via fhevm-lib.
rpc_url="$(fhevm_rpc_url "$chain")"

# Fail-fast: anvil must already be running at $rpc_url. is_anvil returns
# non-zero both for "RPC unreachable" and "wrong client running" — both are
# blocking for this script (it relies on anvil_setBalance and a fresh chain).
if ! is_anvil "$rpc_url"; then
    echo "❌ anvil is not running at $rpc_url (or another client is on that port)" >&2
    echo "   Start anvil first, e.g.:  anvil --port 8545" >&2
    exit 1
fi

# ==============================================================================
# Unset CHAIN environment variable
# ==============================================================================

[[ -n "${CHAIN:-}" ]] && echo "Warning: unsetting CHAIN=$CHAIN" >&2; unset CHAIN

# ==============================================================================

fhevm_mnemonic="test test test test test test test future home engine virtual motion"
fhevm_host_contracts_mnemonic="adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"

deployer_mnemonic=${fhevm_host_contracts_mnemonic}
deployer_mnemonic_index=5
empty_uups_mnemonic=${fhevm_host_contracts_mnemonic}
empty_uups_mnemonic_index=100

chain_id_gateway=654321

num_kms_nodes=4
num_coprocessors=4

kms_threshold=1
coprocessor_threshold=1

hcu_cap_per_block=281474976710655
max_hcu_depth_per_tx=5000000
max_hcu_per_tx=20000000

pausers_mnemonic=${fhevm_host_contracts_mnemonic}
pausers_mnemonic_index=2
num_pausers=2

# Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))`.
input_verification_address=0x6189F6c0c3E40B4a3c72ec86262295D78d845297

# Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))`.
decryption_address=0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721

default_anvil_balance=10000

# ==============================================================================

# kms_nodes_mnemonic=${fhevm_host_contracts_mnemonic}
# kms_nodes_mnemonic_path="m/44'/60'/0'/0/"
# kms_nodes_mnemonic_index=7

# coprocessors_mnemonic=${fhevm_host_contracts_mnemonic}
# coprocessors_mnemonic_path="m/44'/60'/0'/0/"
# coprocessors_mnemonic_index=11

kms_nodes_mnemonic=${fhevm_mnemonic}
kms_nodes_mnemonic_path="m/44'/60'/0'/3/"
kms_nodes_mnemonic_index=0

coprocessors_mnemonic=${fhevm_mnemonic}
coprocessors_mnemonic_path="m/44'/60'/0'/2/"
coprocessors_mnemonic_index=0

# ==============================================================================

declare -a FORGE_ENV=(
    "DEPLOYER_MNEMONIC=${deployer_mnemonic}"
    "DEPLOYER_MNEMONIC_INDEX=${deployer_mnemonic_index}"
    "EMPTY_UUPS_MNEMONIC=${empty_uups_mnemonic}"
    "EMPTY_UUPS_MNEMONIC_INDEX=${empty_uups_mnemonic_index}"
    "CHAIN_ID_GATEWAY=${chain_id_gateway}"
    "NUM_KMS_NODES=${num_kms_nodes}"
    "KMS_NODES_MNEMONIC=${kms_nodes_mnemonic}"
    "KMS_NODES_MNEMONIC_PATH=${kms_nodes_mnemonic_path}"
    "KMS_NODES_MNEMONIC_INDEX=${kms_nodes_mnemonic_index}"
    "NUM_COPROCESSORS=${num_coprocessors}"
    "COPROCESSORS_MNEMONIC=${coprocessors_mnemonic}"
    "COPROCESSORS_MNEMONIC_PATH=${coprocessors_mnemonic_path}"
    "COPROCESSORS_MNEMONIC_INDEX=${coprocessors_mnemonic_index}"
    "PUBLIC_DECRYPTION_THRESHOLD=${kms_threshold}"
    "COPROCESSOR_THRESHOLD=${coprocessor_threshold}"
    "HCU_CAP_PER_BLOCK=${hcu_cap_per_block}"
    "MAX_HCU_DEPTH_PER_TX=${max_hcu_depth_per_tx}"
    "MAX_HCU_PER_TX=${max_hcu_per_tx}"
    "NUM_PAUSERS=${num_pausers}"
    "PAUSERS_MNEMONIC=${pausers_mnemonic}"
    "PAUSERS_MNEMONIC_INDEX=${pausers_mnemonic_index}"
    "DECRYPTION_ADDRESS=${decryption_address}"
    "INPUT_VERIFICATION_ADDRESS=${input_verification_address}"
)

# ==============================================================================
#
# ---- Generate FHEVM host contracts addresses ----
#
# ==============================================================================

env "${FORGE_ENV[@]}" forge script script/DeployCleartextFHEVMHost.s.sol:WriteFHEVMHostAddressesDotSol 

# ==============================================================================
#
# ---- Fund deployers ----
#
# ==============================================================================

forge_json() {
    local target="$1"
    env "${FORGE_ENV[@]}" forge script "$target" --rpc-url "${rpc_url}" --non-interactive 2>&1 | awk '
      /JSON_RESULT_START/ { capture=1; next }
      /JSON_RESULT_END/   { capture=0; exit }
      capture
    '
}

signers_json="$(forge_json script/DeployCleartextFHEVMHost.s.sol:PrintFhevmSigners)"

deployer_address="$(jq -r '.deployer.address' <<<"$signers_json")"
empty_uups_deployer_address="$(jq -r '.emptyUupsDeployer.address' <<<"$signers_json")"

echo "deployer:           $deployer_address"
echo "emptyUupsDeployer:  $empty_uups_deployer_address"

# Funds the deployer and emptyUupsDeployer addresses on anvil via
# anvil_setBalance. Skips emptyUupsDeployer when it's the zero address
# (single-key flow → resolveDeployersAsJson emits 0x000…000).
#
# Usage: fund_anvil_deployers <deployer_address> <empty_uups_deployer_address> <amount_eth>
# Thin wrapper over fhevm-lib's `set_anvil_balance`. The lib helper already
# silently skips the zero-address case (emptyUupsDeployer when not configured).
fund_anvil_deployers() {
    local deployer_addr="$1"
    local empty_uups_addr="$2"

    set_anvil_balance "$deployer_addr"   "$default_anvil_balance" "$rpc_url"
    set_anvil_balance "$empty_uups_addr" "$default_anvil_balance" "$rpc_url"
}

# Verifies the deployer and emptyUupsDeployer balances on $rpc_url match the
# expected amount in ETH. Skips emptyUupsDeployer when it's the zero address.
# Returns non-zero on any mismatch.
#
# Usage: verify_anvil_balances <deployer_address> <empty_uups_address> <expected_eth>
# Thin wrapper over fhevm-lib's `verify_balance`. Both calls return non-zero
# on mismatch; `set -euo pipefail` propagates the failure to the caller.
verify_anvil_balances() {
    local deployer_addr="$1"
    local empty_uups_addr="$2"
    local expected_eth="$3"

    verify_balance "$deployer_addr"   "$expected_eth" "$rpc_url" "deployer"
    verify_balance "$empty_uups_addr" "$expected_eth" "$rpc_url" "emptyUupsDeployer"
}

if is_anvil "$rpc_url"; then
    fund_anvil_deployers "$deployer_address" "$empty_uups_deployer_address"
    verify_anvil_balances "$deployer_address" "$empty_uups_deployer_address" "$default_anvil_balance"
else
    echo "ℹ️  Not on anvil (or RPC unreachable); skipping anvil_setBalance funding."
fi

# ==============================================================================
#
# ---- Deploy ----
#
# ==============================================================================

echo
echo "🚚  Deploying Cleartext FHEVM Host Constracts ..."

env "${FORGE_ENV[@]}" forge script \
    script/DeployCleartextFHEVMHost.s.sol:Deploy \
    --non-interactive \
    --rpc-url "${rpc_url}" \
    --broadcast

# ==============================================================================
#
# ---- Verify ----
#
# ==============================================================================

echo
echo "🥬  Verifying Cleartext FHEVM Host Constracts ..."

env "${FORGE_ENV[@]}" forge script \
    script/DeployCleartextFHEVMHost.s.sol:Verify \
    --non-interactive \
    --rpc-url "${rpc_url}"

echo "✅  ok"
