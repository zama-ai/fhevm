#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

LIB="ethers,viem"
SKIP_START=false
CHAIN="localstack"
VALID_CHAINS=(localstack localstack_v14)

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Run FHEVM tests against the localstack.

Options:
  --ethlib ethers          Run only the ethers test suite.
  --ethlib viem            Run only the viem test suite.
  --ethlib ethers,viem     Run both suites (default).
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

format_duration() {
    local total="$1"
    local m=$(( total / 60 ))
    local s=$(( total % 60 ))
    if [[ "$m" -gt 0 ]]; then
        printf '%dm%02ds' "$m" "$s"
    else
        printf '%ds' "$s"
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

SETUP_DURATION=0
TESTS_DURATION=0
STOP_DURATION=0

SETUP_START=$SECONDS
if [ "$SKIP_START" = false ]; then
  "$SCRIPT_DIR/localstack-restart.sh" --chain "$CHAIN" --force
fi
SETUP_DURATION=$(( SECONDS - SETUP_START ))

export CHAIN

TESTS_START=$SECONDS

# First check configuration
if ! npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem/clientBase.chain.test.ts; then
  echo ""
  echo "❌ ERROR: update test/chains/${CHAIN}.ts config file ❌"
  echo ""
  exit 1
fi

[[ "$RUN_VIEM"   -eq 1 ]] && npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/viem
[[ "$RUN_ETHERS" -eq 1 ]] && npx vitest run --config test/fheTest/vitest.config.ts test/fheTest/ethers

TESTS_DURATION=$(( SECONDS - TESTS_START ))

STOP_START=$SECONDS
if [ "$SKIP_START" = false ]; then
  STOP_RESULT=0
  "$SCRIPT_DIR/localstack-stop.sh" || STOP_RESULT=$?
  if [[ "$STOP_RESULT" -ne 0 ]]; then
    echo "Warning: localstack-stop.sh failed; ignoring teardown error because tests already completed." >&2
  fi
fi
STOP_DURATION=$(( SECONDS - STOP_START ))

echo ""
echo "Timing summary:"
printf '  %-24s %s\n' "Setup ${CHAIN}:" "$(format_duration "$SETUP_DURATION")"
printf '  %-24s %s\n' "Run tests:" "$(format_duration "$TESTS_DURATION")"
printf '  %-24s %s\n' "Stop ${CHAIN}:" "$(format_duration "$STOP_DURATION")"
