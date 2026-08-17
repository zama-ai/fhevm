#!/usr/bin/env bash

set -euo pipefail

# Run the fheTest suites (test/fheTest/ethers and test/fheTest/viem) against a
# REMOTE chain: testnet, devnet, or sepolia.
#
# Unlike localstack-run-tests.sh / localcleartext-run-tests.sh, this does NOT
# start any local stack — it just points vitest at a remote chain via CHAIN.
#
# Requires ZAMA_FHEVM_API_KEY (and optionally MNEMONIC / RPC_URL) in test/.env
# or the environment; the fheTest setup fails fast if the API key is missing.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

LIB="ethers,viem"
CHAIN=""
FAST=false
VALID_CHAINS=(testnet devnet sepolia)

usage() {
    cat <<EOF
Usage: $(basename "$0") --chain <testnet|devnet|sepolia> [OPTIONS]

Run the fheTest ethers/viem suites against a remote chain (no local stack).

Options:
  --chain <name>       Required. One of: ${VALID_CHAINS[*]}.
  --ethlib <libs>      ethers | viem | ethers,viem   (default: ethers,viem)
  --fast               Exclude slow suites (**/**.slow.test.ts).
  -h, --help           Show this help.

Environment (from test/.env or the shell):
  ZAMA_FHEVM_API_KEY   Required.
  MNEMONIC, RPC_URL    Optional (fall back to the chain definition defaults).

Examples:
  $(basename "$0") --chain testnet
  $(basename "$0") --chain devnet --ethlib viem --fast
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --chain)
            shift
            [[ $# -gt 0 ]] || { echo "Error: --chain requires an argument." >&2; exit 1; }
            CHAIN="$1"
            shift
            ;;
        --chain=*)
            CHAIN="${1#--chain=}"
            shift
            ;;
        --ethlib)
            shift
            [[ $# -gt 0 ]] || { echo "Error: --ethlib requires an argument." >&2; exit 1; }
            LIB="$1"
            shift
            ;;
        --ethlib=*)
            LIB="${1#--ethlib=}"
            shift
            ;;
        --fast)
            FAST=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Error: unknown option '$1'." >&2
            usage
            exit 1
            ;;
    esac
done

# Validate --chain
if [[ -z "$CHAIN" ]]; then
    echo "Error: --chain is required (one of: ${VALID_CHAINS[*]})." >&2
    exit 1
fi
CHAIN_OK=false
for c in "${VALID_CHAINS[@]}"; do
    [[ "$c" == "$CHAIN" ]] && CHAIN_OK=true
done
if [[ "$CHAIN_OK" != true ]]; then
    echo "Error: invalid --chain '$CHAIN'. Expected one of: ${VALID_CHAINS[*]}." >&2
    exit 1
fi

# Validate --ethlib
RUN_ETHERS=0
RUN_VIEM=0
case "$LIB" in
    ethers) RUN_ETHERS=1 ;;
    viem) RUN_VIEM=1 ;;
    ethers,viem | viem,ethers) RUN_ETHERS=1; RUN_VIEM=1 ;;
    *)
        echo "Error: --ethlib must be 'ethers', 'viem', or 'ethers,viem' (got: '$LIB')." >&2
        exit 1
        ;;
esac

cd "$ROOT_DIR"

VITEST_CONFIG="test/fheTest/vitest.config.ts"

run_suite() {
    local dir="$1"
    echo "▶ fheTest ${dir} on CHAIN=${CHAIN} (fast=${FAST})"
    if [[ "$FAST" == true ]]; then
        CHAIN="$CHAIN" npx vitest run --config "$VITEST_CONFIG" --exclude '**/**.slow.test.ts' "$dir"
    else
        CHAIN="$CHAIN" npx vitest run --config "$VITEST_CONFIG" "$dir"
    fi
}

# NOTE the trailing slash: vitest positionals are substring path filters, so
# "test/fheTest/viem" would ALSO match "test/fheTest/viem-cleartext/**". The
# trailing "/" pins the match to the exact suite directory.
[[ "$RUN_VIEM" -eq 1 ]] && run_suite "test/fheTest/viem/"
[[ "$RUN_ETHERS" -eq 1 ]] && run_suite "test/fheTest/ethers/"

echo "✅ Done (CHAIN=${CHAIN}, ethlib=${LIB})."
