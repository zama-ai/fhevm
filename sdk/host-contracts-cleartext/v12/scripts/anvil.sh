#!/usr/bin/env bash
#
# Start a fresh anvil node, deploy a full cleartext v12 stack onto it with scripts/deploy.sh, and keep the
# node running so you can point tests, scripts or a dApp at it. Ctrl-C stops anvil.
#
# Usage: ./scripts/anvil.sh [--port PORT] [--mnemonic "..."] [--account-index N] [--no-deploy]
#
#   --port PORT        anvil port (default: 8545)
#   --mnemonic "..."   mnemonic to fund/derive from (default: the package's test mnemonic)
#   --account-index N  deployer account within that mnemonic (default: 5)
#   --no-deploy        start the node only, deploy nothing
#
# The deploy and its verification both live in scripts/deploy.sh — this script only starts a node and
# hands over. Everything the stack is checked against is checked there, by
# pkg/forge/script/VerifyFhevmDeploy.s.sol reading the chain back.
#
# The node runs with stock settings: no --code-size-limit. Every contract must fit the 24576 B EIP-170
# cap so the stack deploys on any chain (RULES.md rule 12). Largest today: CleartextFHEVMExecutor at
# 22,994 B. If a deploy ever fails on code size, shrink the contract — do not raise the node's limit.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

PORT=8545
ANVIL_MNEMONIC='adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer'
ACCOUNT_INDEX=5
DEPLOY=1

while [ $# -gt 0 ]; do
    case "$1" in
        --port)
            [ $# -ge 2 ] || { echo "Error: --port requires a value." >&2; exit 1; }
            PORT="$2"
            shift 2
            ;;
        --port=*)
            PORT="${1#--port=}"
            shift
            ;;
        --mnemonic)
            [ $# -ge 2 ] || { echo "Error: --mnemonic requires a value." >&2; exit 1; }
            ANVIL_MNEMONIC="$2"
            shift 2
            ;;
        --mnemonic=*)
            ANVIL_MNEMONIC="${1#--mnemonic=}"
            shift
            ;;
        --account-index)
            [ $# -ge 2 ] || { echo "Error: --account-index requires a value." >&2; exit 1; }
            ACCOUNT_INDEX="$2"
            shift 2
            ;;
        --account-index=*)
            ACCOUNT_INDEX="${1#--account-index=}"
            shift
            ;;
        --no-deploy)
            DEPLOY=0
            shift
            ;;
        -h | --help)
            sed -n '2,19p' "${BASH_SOURCE[0]}"
            exit 0
            ;;
        *)
            echo "Error: unknown argument '$1'. Try --help." >&2
            exit 1
            ;;
    esac
done

cd "$PACKAGE_ROOT"

# shellcheck source=scripts/anvil-lib.sh
source "$SCRIPT_DIR/anvil-lib.sh"

require_tools anvil cast

RPC_URL="http://127.0.0.1:${PORT}"

ANVIL_PID=""
trap cleanup EXIT INT TERM

echo "⛓  anvil on ${RPC_URL}"
start_anvil

wait_for_node

# The three addresses a dApp expects when it compiles against the FHE library's local config
# (library-solidity/config/ZamaConfig.sol -> _getLocalConfig). Checked here rather than in deploy.sh
# because deploy.sh serves any network, while this script exists to produce the *local* stack: it is the
# default mnemonic at account index 5, with a start nonce of 0, that lands on these. Pass a different
# --mnemonic or --account-index and the whole stack moves — VerifyFhevmDeploy will still pass, because it
# checks against the addresses the build was compiled for, but no ZamaConfig-compiled dApp will find it.
ZAMA_LOCAL_ACL=0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D
ZAMA_LOCAL_COPROCESSOR=0xe3a9105a3a932253A70F126eb1E3b589C643dD24
ZAMA_LOCAL_KMS_VERIFIER=0x901F8942346f7AB3a01F6D7613119Bca447Bb030

verify_zama_local_config() {
    local failed=0
    echo ""
    echo "🔎 checking the stack sits where ZamaConfig.sol _getLocalConfig() says"
    for entry in \
        "ACLAddress:$ZAMA_LOCAL_ACL" \
        "CoprocessorAddress:$ZAMA_LOCAL_COPROCESSOR" \
        "KMSVerifierAddress:$ZAMA_LOCAL_KMS_VERIFIER"; do
        local name="${entry%%:*}"
        local address="${entry##*:}"
        local code
        code="$(cast code "$address" --rpc-url "$RPC_URL" 2>/dev/null || echo 0x)"
        if [ "${#code}" -le 2 ]; then
            echo "   ❌ $name $address — no code deployed"
            failed=1
        else
            echo "   ✅ $name $address"
        fi
    done

    if [ "$failed" -ne 0 ]; then
        echo ""
        echo "The stack deployed and verified, but not at the addresses ZamaConfig.sol hardcodes for chain"
        echo "31337, so a dApp compiled against the FHE library's local config would call empty addresses."
        echo "Expected with a non-default --mnemonic/--account-index; otherwise the deploy order or the"
        echo "deployer's start nonce has changed."
        return 1
    fi
}

if [ "$DEPLOY" -eq 1 ]; then
    echo "🚀 deploying the cleartext v12 stack"
    ./scripts/deploy.sh \
        --rpc-url "$RPC_URL" \
        --mnemonic "$ANVIL_MNEMONIC" \
        --account-index "$ACCOUNT_INDEX"
    verify_zama_local_config || exit 1
fi

echo ""
enable_anvil_traces
echo "   anvil is still running on ${RPC_URL} — press Ctrl-C to stop."
wait "$ANVIL_PID"
