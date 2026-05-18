#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FHEVM_DIR="$(cd "$SCRIPT_DIR/../../../../test-suite/fhevm" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/../../contracts" && pwd)"
PORT=8545
FORCE=false

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Ensure the local FHEVM stack is running.

By default, if anvil is already listening on port ${PORT}, the script assumes
the local stack is already running and exits without restarting or redeploying.

Options:
  --force, -f     Force a full down/up restart and redeploy FHETest.
  --help, -h      Print this help message and exit.
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --force|-f)
            FORCE=true
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

port_is_listening() {
    lsof -nP -iTCP:"$PORT" -sTCP:LISTEN >/dev/null 2>&1
}

anvil_is_listening() {
    lsof -nP -iTCP:"$PORT" -sTCP:LISTEN 2>/dev/null | awk 'NR > 1 { print $1 }' | grep -Eq '^anvil$'
}

print_port_conflict_and_exit() {
    echo "" >&2
    echo "========================================" >&2
    echo "Port ${PORT} is already in use:" >&2
    echo "" >&2
    lsof -nP -iTCP:"$PORT" -sTCP:LISTEN >&2
    echo "" >&2
    echo "Stop the process listening on ${PORT} before starting the local stack." >&2
    echo "========================================" >&2
    echo "" >&2
    exit 1
}

if [[ "$FORCE" = false ]]; then
    if anvil_is_listening; then
        echo "anvil is already listening on port ${PORT}; assuming local stack is running."
        echo "Use --force to restart and redeploy FHETest."
        exit 0
    fi

    if port_is_listening; then
        print_port_conflict_and_exit
    fi
fi

# Make sure the fhevm-cli is ready to go
cd "$FHEVM_DIR"
bun install

if [[ "$FORCE" = true ]]; then
    "$FHEVM_DIR/fhevm-cli" down

    if port_is_listening; then
        print_port_conflict_and_exit
    fi
fi

# Start
"$FHEVM_DIR/fhevm-cli" up

# Deploy FHETest.sol
cd "$CONTRACTS_DIR"

forge clean

./scripts/fhetest-deploy.sh --chain localstack
