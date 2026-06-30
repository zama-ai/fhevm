#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

LIB="ethers,viem"
SKIP_START=false
PROFILE=""
CHAIN="localstack"
VALID_CHAINS=(localstack localstack_v11 localstack_v12 localstack_v13 localstack_v14)

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Run FHEVM tests against the localstack.

Options:
  --ethlib ethers          Run only the ethers test suite.
  --ethlib viem            Run only the viem test suite.
  --ethlib ethers,viem     Run both suites (default).
  --fhevm-cli-profile <name>
                           Profile filename forwarded to localstack-restart.sh
                           (e.g., v0.11.0-mainnet.json). If omitted, the stack
                           starts without a profile lock file.
  --chain, -c <name>       Chain forwarded to localstack-restart.sh and used as
                           the CHAIN env var when invoking vitest. One of:
                           ${VALID_CHAINS[*]}.
                           Default: ${CHAIN}.
  --skip-start             Skip localstack-restart.sh before and localstack-stop.sh after running tests.
  --help                   Print this help message and exit.
EOF
}

is_valid_chain() {
    local candidate="$1"
    local c
    for c in "${VALID_CHAINS[@]}"; do
        if [[ "$c" == "$candidate" ]]; then
            return 0
        fi
    done
    return 1
}

require_arg_value() {
    if [[ $# -lt 2 || -z "${2:-}" || "${2:-}" == -* ]]; then
        echo "Error: ${1} requires a value." >&2
        exit 1
    fi
}

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
        --fhevm-cli-profile)
            require_arg_value "$1" "${2:-}"
            PROFILE="$2"
            shift 2
            ;;
        --fhevm-cli-profile=*)
            PROFILE="${1#--fhevm-cli-profile=}"
            shift
            ;;
        --chain|-c)
            require_arg_value "$1" "${2:-}"
            CHAIN="$2"
            shift 2
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

if ! is_valid_chain "$CHAIN"; then
    echo "Error: invalid --chain '$CHAIN'. Expected one of: ${VALID_CHAINS[*]}." >&2
    exit 1
fi

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
  RESTART_ARGS=(--chain "$CHAIN")
  if [[ -n "$PROFILE" ]]; then
    RESTART_ARGS+=(--fhevm-cli-profile "$PROFILE")
  fi
  RESTART_ARGS+=(--force)
  "$SCRIPT_DIR/localstack-restart.sh" "${RESTART_ARGS[@]}"
fi

export CHAIN

# First check configuration
if ! npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem/clientBase.chain.test.ts; then
  echo ""
  echo "❌ ERROR: update test/chains/${CHAIN}.ts config file ❌"
  echo ""
  exit 1
fi

[[ "$RUN_VIEM"   -eq 1 ]] && npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem
[[ "$RUN_ETHERS" -eq 1 ]] && npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/ethers

if [ "$SKIP_START" = false ]; then
  if ! "$SCRIPT_DIR/localstack-stop.sh"; then
    echo "Warning: localstack-stop.sh failed; ignoring teardown error because tests already completed." >&2
  fi
fi
