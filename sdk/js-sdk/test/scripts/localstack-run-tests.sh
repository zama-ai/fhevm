#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Run FHEVM tests against the localstack.

Options:
  --ethlib ethers          Run only the ethers test suite.
  --ethlib viem            Run only the viem test suite.
  --ethlib ethers,viem     Run both suites (default).
  --skip-start             Skip localstack-restart.sh before and localstack-stop.sh after running tests.
  --help                   Print this help message and exit.
EOF
}

LIB="ethers,viem"
SKIP_START=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --ethlib)
            shift
            if [[ $# -eq 0 ]]; then
                echo "Error: --ethlib requires an argument." >&2
                exit 1
            fi
            LIB="$1"
            shift
            ;;
        --ethlib=*)
            LIB="${1#--ethlib=}"
            shift
            ;;
        --skip-start)
            SKIP_START=true
            shift
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            echo "Error: unknown option '$1'. Use --help for usage." >&2
            exit 1
            ;;
    esac
done

RUN_ETHERS=0
RUN_VIEM=0
case "$LIB" in
    ethers)       RUN_ETHERS=1 ;;
    viem)         RUN_VIEM=1 ;;
    ethers,viem)  RUN_ETHERS=1; RUN_VIEM=1 ;;
    viem,ethers)  RUN_ETHERS=1; RUN_VIEM=1 ;;
    *)
        echo "Error: --ethlib must be 'ethers', 'viem', or 'ethers,viem' (got: '${LIB}')." >&2
        exit 1
        ;;
esac

cd "$ROOT_DIR"

if [ "$SKIP_START" = false ]; then
  $SCRIPT_DIR/localstack-restart.sh
fi

export CHAIN=localstack

# First check configuration
if ! npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem/clientBase.chain.test.ts; then
  echo ""
  echo "❌ ERROR: update test/fheTest/chains/localstack.ts config file ❌"
  echo ""
  exit 1
fi

[[ "$RUN_VIEM"   -eq 1 ]] && npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem
[[ "$RUN_ETHERS" -eq 1 ]] && npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/ethers

if [ "$SKIP_START" = false ]; then
  $SCRIPT_DIR/localstack-stop.sh
fi
