#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

# load utils
source "$SCRIPT_DIR/fhevm-lib.sh"

# ==============================================================================

# Defaults (lowest priority — overridden by env vars or CLI flags below).
fhevm_mnemonic_default="test test test test test test test future home engine virtual motion"
fhevm_mnemonic_path="m/44'/60'/0'/0/0"
chain_default="localhost"
abi_v2_file="$SCRIPT_DIR/../../test/fheTest/fhe-test-addresses-v2.json"

# ---- CLI parsing ----
mnemonic_cli=""
chain_cli=""
dry_run=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --mnemonic)
            mnemonic_cli="${2:?--mnemonic requires a value}"
            shift 2
            ;;
        --chain)
            chain_cli="${2:?--chain requires a value}"
            shift 2
            ;;
        --dry-run)
            dry_run=true
            shift
            ;;
        -h|--help)
            cat <<EOF
Usage: fhetest-deploy.sh [options]

Options:
  --mnemonic <phrase>   BIP-39 mnemonic for the deployer key.
                        Precedence: --mnemonic flag > \$MNEMONIC env > built-in test mnemonic.
  --chain <name>        FHEVM chain (localstack | localhost | devnet).
                        Precedence: --chain flag > \$CHAIN env > $chain_default
  --dry-run             Resolve and print all addresses + config, then exit
                        before any state-changing call (no funding, no
                        forge create, no cast send).
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

# Resolve: CLI flag > env var > default.
# IMPORTANT: $CHAIN must be read here BEFORE the `unset CHAIN` block below
# (the unset is needed so forge doesn't pick CHAIN up from our shell when
# running its own scripts later).
fhevm_mnemonic="${mnemonic_cli:-${MNEMONIC:-$fhevm_mnemonic_default}}"

chain="${chain_cli:-${CHAIN:-$chain_default}}"
fhevm_assert_chain "$chain"

# only chain="localhost|localstack" for the moment
case "$chain" in
    localhost|localstack) ;;
    *)
        echo "❌ fhetest-deploy.sh only supports chains 'localhost|localstack'; got '$chain'" >&2
        exit 1
        ;;
esac

# Derive RPC URL from the resolved chain via fhevm-lib.
rpc_url="$(fhevm_rpc_url "$chain")"

assert_is_anvil ${rpc_url}

# ==============================================================================
# Unset CHAIN environment variable (forge)
# ==============================================================================

[[ -n "${CHAIN:-}" ]] && echo "Warning: unsetting CHAIN=$CHAIN" >&2; unset CHAIN

# ==============================================================================

acl_addr="$(fhevm_chain_address ${chain} acl)"
kms_verifier_addr="$(fhevm_chain_address ${chain} kmsVerifier)"

# If this call fails, the ACL proxy isn't deployed (or hasn't been upgraded
# past the empty UUPS impl) — that's the FHEVM host stack's job, not ours.
if ! coprocessor_addr="$(cast call "$acl_addr" "getFHEVMExecutorAddress()(address)" --rpc-url "$rpc_url" 2>/dev/null)" \
        || [[ -z "$coprocessor_addr" || "$coprocessor_addr" == "0x0000000000000000000000000000000000000000" ]]; then
    boxed_error "Failed to read FHEVMExecutor address from ACL at $acl_addr.
  The FHEVM host contracts don't appear to be deployed (or aren't fully wired) on $rpc_url.

  🎃 Run './fhevm-deploy.sh' first, then re-run './fhetest-deploy.sh'."
    exit 1
fi

echo "rpc-url:       $rpc_url"
echo "ACL:           $acl_addr"
echo "KMSVerifier:   $kms_verifier_addr"
echo "Coprocessor:   $coprocessor_addr"

# ==============================================================================

private_key=$(cast wallet private-key --mnemonic "${fhevm_mnemonic}" --mnemonic-derivation-path "${fhevm_mnemonic_path}")
deployer_addr=$(cast wallet address --private-key "${private_key}")

echo "deployer:      $deployer_addr"

if [[ "$dry_run" == true ]]; then
    # Predict the FHETest CREATE address: keccak256(rlp(deployer, nonce))[12:].
    # `cast compute-address` reads the current on-chain nonce when --nonce isn't given.
    future_fhe_test_addr=$(cast compute-address "$deployer_addr" --rpc-url "$rpc_url" | awk '{print $NF}')

    echo
    echo "🟡 --dry-run set; skipping:"
    echo "    - anvil_setBalance(deployer, 10000 ETH)"
    echo "    - forge create src/FHETest.sol:FHETest"
    echo "    - cast send setCoprocessorConfig"
    echo "    - cast send initFheTest(true)"
    echo
    echo "🔮 FHETest (predicted): $future_fhe_test_addr"
    exit 0
fi

# ==============================================================================

if is_anvil ${rpc_url}; then
    set_anvil_balance ${deployer_addr} 10000 ${rpc_url}
fi

# ==============================================================================

fhe_test_addr="$(forge create src/FHETest.sol:FHETest \
    --rpc-url "$rpc_url" --private-key "${private_key}" --broadcast --json | jq -r '.deployedTo')"

# Wire FHETest to the host stack via setCoprocessorConfig(CoprocessorConfig).
# Struct passed as a flat tuple: (ACLAddress, CoprocessorAddress, KMSVerifierAddress).
cast send "$fhe_test_addr" \
    "setCoprocessorConfig((address,address,address))" \
    "(${acl_addr},${coprocessor_addr},${kms_verifier_addr})" \
    --rpc-url "$rpc_url" --private-key "${private_key}" >/dev/null

echo "✅ setCoprocessorConfig"

cast send "$fhe_test_addr" \
    "initFheTest(bool)" \
    "true" \
    --rpc-url "$rpc_url" --private-key "${private_key}" >/dev/null

echo "✅ initFheTest"
echo "✅ FheTest:   ${fhe_test_addr}"

# ==============================================================================
#
# ---- Validate abi-v2.ts addresses ----
#
# ==============================================================================

assert_fhetest_address_in_abi_v2 "$chain" "$fhe_test_addr" "$abi_v2_file"

echo "✅ abi-v2.ts FHETestAddresses match deployed address."
