#!/usr/bin/env bash
# =============================================================================
# anvil-local-v3.sh — fixed-address local cleartext v13 deployment, no compiler
#
# Modelled on forge-fhevm's deploy-local.sh, adapted to this package.
#
# The idea borrowed from there: a proxy does not have to be CREATEd to exist at
# a fixed address. Write its runtime code with anvil_setCode, write the three
# storage slots that make it a valid, freshly-initialised ERC-1967 proxy, then
# send ONE real upgradeToAndCall so the actual initializer runs on chain.
#
#   anvil_setCode(proxy, ERC1967 runtime)
#   anvil_setStorageAt(proxy, ERC1967_IMPL_SLOT,  empty implementation)
#   anvil_setStorageAt(proxy, INITIALIZABLE_SLOT, 1)
#   anvil_setStorageAt(proxy, OWNABLE_SLOT,       deployer)      # ACL only
#   ... then ONE ACLOwner.upgrade(ops) that materializes all nine at once.
#
# ACL's owner slot is written with the ACLOwner address, not the deployer. That
# is precisely the state a finished Ownable2Step transfer leaves behind, so the
# transferOwnership/acceptACLOwnership pair is unnecessary, and it is what makes
# a single atomic upgrade authorized for every proxy.
#
# Why that is better than the CREATE-ordered approaches (anvil.sh,
# anvil-local.sh, anvil-local-v2.sh): the addresses come from setCode, not from
# CREATE(deployer, nonce). So there is no nonce sequence to preserve, no
# "deployer must be at nonce 0" precondition, and the implementations can be
# deployed in any order. That is what makes it both faster and far less brittle.
#
# Why the slot values are exactly these:
#   - INITIALIZABLE_SLOT must be 1, not merely non-zero. Every initializer is
#     guarded by `onlyFromEmptyProxy`, which reverts unless
#     `_getInitializedVersion() == 1`, and each is a `reinitializer(N)` with
#     N > 1 (checked: 2..5 across the nine contracts).
#   - OWNABLE_SLOT on the ACL proxy only. EmptyUUPSProxyACL._authorizeUpgrade is
#     `onlyOwner`, so the ACL proxy needs an owner before it can be upgraded.
#     Every other proxy is `onlyACLOwner`, which reads ACL.owner() instead.
#
# No Solidity compilation at all. Blobs, addresses and the whole bootstrap are
# read out of the generated files under pkg/forge/src/_internal/. The one blob
# we ship as creation-only but need as runtime (ERC1967Proxy) is obtained by
# deploying one throwaway proxy and reading its code back — its implementation
# lives in storage, not in an immutable, so the runtime is address-independent
# (verified: 135 bytes, no address inlined).
#
# See --help for flags.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PACKAGE_ROOT"

# shellcheck source=scripts/anvil-lib.sh
source "$SCRIPT_DIR/anvil-lib.sh"

# -----------------------------------------------------------------------------
# Constants
# -----------------------------------------------------------------------------

PORT=8545
VERBOSE=0
CHECK=1
KEEP_RUNNING=1
ANVIL_PID=""

# -----------------------------------------------------------------------------
# CLI
# -----------------------------------------------------------------------------
print_usage() {
    cat <<'EOF'
Usage: ./scripts/anvil-local-v3.sh [options]

Start anvil and deploy the cleartext v13 stack onto it with no Solidity
compilation, using anvil_setCode/anvil_setStorageAt plus one real
upgradeToAndCall per proxy.

Options:
  --port <port>     anvil port (default: 8545)
  --no-check        skip the post-deploy smoke check
  --exit            deploy, report, then stop anvil instead of staying up
  -v, --verbose     print every step
  -h, --help        show this help

Notes:
  - Canonical stack only: addresses are compiled into the shipped blobs, so the
    deployer is the package mnemonic at its generated account index.
  - Unlike the CREATE-ordered scripts, the deployer's starting nonce is
    irrelevant here; proxies are placed with setCode.
EOF
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --port) require_arg_value "$1" "${2:-}"; PORT="$2"; shift 2 ;;
            --port=*) PORT="${1#--port=}"; shift ;;
            --no-check) CHECK=0; shift ;;
            --exit) KEEP_RUNNING=0; shift ;;
            -v | --verbose) VERBOSE=1; shift ;;
            -h | --help) print_usage; exit 0 ;;
            *) echo "Error: unknown option: $1" >&2; print_usage >&2; exit 1 ;;
        esac
    done
}

# -----------------------------------------------------------------------------
# Readers for the generated Solidity
# -----------------------------------------------------------------------------

resolve_deployer() {
    PRIVATE_KEY="$(cast wallet private-key --mnemonic "$ANVIL_MNEMONIC" --mnemonic-index "$ACCOUNT_INDEX")"
    local derived
    derived="$(cast wallet address --private-key "$PRIVATE_KEY")"
    [[ "${derived,,}" == "${DEPLOYER_ADDRESS,,}" ]] || {
        echo "Error: derived deployer $derived != $DEPLOYER_ADDRESS from the generated addresses." >&2
        exit 1
    }
    log "deployer $derived (mnemonic index $ACCOUNT_INDEX)"

    # Raw code deploys go through a DIFFERENT account, and that is not cosmetic. The canonical
    # addresses are exactly CREATE(deployer, 0..11), so a raw deploy from the deployer at a low nonce
    # lands on one of them — and the setCode that follows then overwrites the implementation we just
    # deployed. Measured: EmptyUUPSProxy landed at nonce 1, which is the ACL address, and every
    # subsequent proxy delegated into the overwritten slot and reverted with no data.
    #
    # The implementations are permissionless and nothing refers to their addresses, so deploying them
    # from an unrelated account removes the collision by construction instead of by nonce arithmetic.
    FACTORY_INDEX=$(( ACCOUNT_INDEX == 0 ? 1 : 0 ))
    CREATE_PRIVATE_KEY="$(cast wallet private-key --mnemonic "$ANVIL_MNEMONIC" --mnemonic-index "$FACTORY_INDEX")"
    log "factory  $(cast wallet address --private-key "$CREATE_PRIVATE_KEY") (mnemonic index $FACTORY_INDEX)"
}

# Place a valid, freshly-initialised ERC-1967 proxy at a fixed address.
materialize_proxy() {
    local target="$1" empty_impl="$2" owner="${3:-}"
    anvil_rpc anvil_setCode "$target" "$PROXY_RUNTIME_CODE"
    anvil_rpc anvil_setStorageAt "$target" "$ERC1967_IMPL_SLOT" "$(pad_word "$empty_impl")"
    anvil_rpc anvil_setStorageAt "$target" "$INITIALIZABLE_SLOT" "$ONE_WORD"
    if [[ -n "$owner" ]]; then
        anvil_rpc anvil_setStorageAt "$target" "$OWNABLE_SLOT" "$(pad_word "$owner")"
    fi
}

# We ship ERC1967Proxy as creation code but need its runtime for setCode. Deploy one
# and read it back: the implementation lives in storage, so the runtime is identical
# for every proxy regardless of address or target.
capture_proxy_runtime() {
    local throwaway args
    # Reuses the shared EmptyUUPSProxy as the throwaway's initial target; only the proxy's own runtime
    # bytes are wanted, and they are the same whatever it points at.
    args="$(cast abi-encode "constructor(address,bytes)" "$EMPTY_IMPL" "$(cast calldata 'initialize()')")"
    throwaway="$(deploy_contract "0x$(read_blob ERC1967_PROXY_CREATION_CODE)${args#0x}" "throwaway ERC1967Proxy")"
    PROXY_RUNTIME_CODE="$(cast code "$throwaway")"
    [[ "${#PROXY_RUNTIME_CODE}" -gt 2 ]] || { echo "Error: throwaway proxy has no runtime code." >&2; exit 1; }
    echo "   ✅ ERC1967 runtime captured ($(( (${#PROXY_RUNTIME_CODE} - 2) / 2 )) bytes)"
}

# Place all nine proxies. No transactions here at all — setCode and setStorageAt are RPC calls, so this
# whole phase is free. ACL's owner slot gets the ACLOwner address rather than the deployer: that is
# exactly the state a completed Ownable2Step transfer would leave (pendingOwner stays zero), and it is
# what authorizes the single upgrade below. It also means no transferOwnership/acceptACLOwnership pair.
materialize_all_proxies() {
    materialize_proxy "$ACL_ADDRESS" "$EMPTY_ACL_IMPL" "$ACL_OWNER"
    materialize_proxy "$FHEVM_EXECUTOR_ADDRESS" "$EMPTY_IMPL"
    materialize_proxy "$KMS_VERIFIER_ADDRESS" "$EMPTY_IMPL"
    materialize_proxy "$INPUT_VERIFIER_ADDRESS" "$EMPTY_IMPL"
    materialize_proxy "$HCU_LIMIT_ADDRESS" "$EMPTY_IMPL"
    materialize_proxy "$PROTOCOL_CONFIG_ADDRESS" "$EMPTY_IMPL"
    materialize_proxy "$KMS_GENERATION_ADDRESS" "$EMPTY_IMPL"
    materialize_proxy "$CLEARTEXT_ARITHMETIC_ADDRESS" "$EMPTY_IMPL"
    materialize_proxy "$CLEARTEXT_DB_ADDRESS" "$EMPTY_IMPL"
    echo "   ✅ 9 proxies placed at their canonical addresses (0 transactions)"
}

# PauserSet has no constructor and no proxy, so its runtime blob is complete as shipped.
install_pauser_set() {
    anvil_rpc anvil_setCode "$PAUSER_SET_ADDRESS" "0x$(read_blob PAUSER_SET_RUNTIME_CODE)"
    local code
    code="$(cast code "$PAUSER_SET_ADDRESS")"
    [[ "${#code}" -gt 2 ]] || { echo "Error: anvil_setCode left no code at PauserSet." >&2; exit 1; }
    # `pausers` is the first (and only) storage variable in PauserSet, so slot 0. Written directly
    # because addPauser is onlyACLOwner: once ACL's owner is the ACLOwner contract, the deployer cannot
    # call it, and routing through ACLOwner.execute would cost an extra transaction for one bool.
    anvil_rpc anvil_setStorageAt "$PAUSER_SET_ADDRESS" "$(cast index address "$ACL_OWNER" 0)" "$ONE_WORD"
    echo "   ✅ PauserSet installed at $PAUSER_SET_ADDRESS, ACLOwner registered as pauser"
}

# -----------------------------------------------------------------------------
main() {
    parse_args "$@"
    require_tools anvil cast
    require_generated_files "$BYTECODE_SOL" "$ADDRESSES_SOL" "$BOOTSTRAP_SOL"
    load_generated_addresses_and_bootstrap

    RPC_URL="http://127.0.0.1:${PORT}"
    export ETH_RPC_URL="$RPC_URL"

    trap cleanup EXIT INT TERM
    echo "⛓  anvil on ${RPC_URL} (traces off during deploy)"
    start_anvil
    wait_for_node
    resolve_deployer

    echo ""
    echo "🚀 deploying the cleartext v13 stack (setCode + one atomic ACLOwner.upgrade, no solc)"
    deploy_empty_acl_implementation
    deploy_empty_implementation
    deploy_real_implementations
    deploy_acl_owner
    echo "   ✅ 2 empty + 9 real implementations + ACLOwner deployed"
    capture_proxy_runtime
    build_initializer_calldata
    materialize_all_proxies
    install_pauser_set
    upgrade_stack_atomically

    if [[ "$CHECK" == "1" ]]; then
        echo ""
        echo "🔎 checking the deployed stack"
        smoke_check || exit 1
    fi

    report_stack

    if [[ "$KEEP_RUNNING" == "1" ]]; then
        echo ""
        enable_anvil_traces
        echo "   anvil is still running on ${RPC_URL} — press Ctrl-C to stop."
        wait "$ANVIL_PID"
    fi
}

main "$@"
