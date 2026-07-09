#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FHEVM_DIR="$(cd "$SCRIPT_DIR/../../../../test-suite/fhevm" && pwd)"

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Shut down the local FHEVM stack.

Options:
  --fhevm-dir <path>    Directory containing the fhevm-cli / test-suite/fhevm
                        checkout to use. Default: ${FHEVM_DIR}.
  --help, -h            Print this help message and exit.
EOF
}

require_arg_value() {
    if [[ $# -lt 2 || -z "${2:-}" || "${2:-}" == -* ]]; then
        echo "Error: ${1} requires a value." >&2
        exit 1
    fi
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --fhevm-dir)
            require_arg_value "$1" "${2:-}"
            FHEVM_DIR="$2"
            shift 2
            ;;
        --fhevm-dir=*)
            FHEVM_DIR="${1#--fhevm-dir=}"
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

if [[ ! -d "$FHEVM_DIR" ]]; then
    echo "Error: --fhevm-dir '$FHEVM_DIR' is not a directory." >&2
    exit 1
fi
FHEVM_DIR="$(cd "$FHEVM_DIR" && pwd)"

# Shutdown fhevm
"${FHEVM_DIR}/fhevm-cli" down
