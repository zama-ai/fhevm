#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/lib-common.sh"

FHEVM_DIR="$(cd "$SCRIPT_DIR/../../../../test-suite/fhevm" && pwd)"
CONTRACTS_DIR="$(cd "$SCRIPT_DIR/../../contracts" && pwd)"
PORT=8545
RPC_URL="http://127.0.0.1:$PORT"
FORCE=false
DRY_RUN=false
PROFILE=""
PROFILES_DIR="$FHEVM_DIR/profiles"
CHAIN="localstack"
VALID_CHAINS=(localstack localstack_v11 localstack_v12 localstack_v13 localstack_v14)
CHAINS_DIR="$SCRIPT_DIR/../chains"
CHAIN_DEFAULTS_FILE="$CHAINS_DIR/chain-defaults.json"

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Ensure the local FHEVM stack is running.

By default, if anvil is already listening on port ${PORT}, the script assumes
the local stack is already running and exits without restarting or redeploying.

Options:
  --fhevm-cli-profile <name>
                        Profile filename in ${PROFILES_DIR} (e.g., v0.11.0-mainnet.json).
                        If omitted, fhevm-cli starts without a profile lock file.
  --chain, -c <name>    Chain to pass to fhetest-deploy.sh. One of:
                        ${VALID_CHAINS[*]}.
                        Default: ${CHAIN}.
  --fhevm-dir <path>    Directory containing the fhevm-cli / test-suite/fhevm
                        checkout to use. Default: ${FHEVM_DIR}.
  --force, -f           Force a full down/up restart and redeploy FHETest.
  --dry-run, -n         Print the resolved configuration and the commands that
                        would be executed, then exit without doing anything.
  --help, -h            Print this help message and exit.

Example:
  ./localstack-restart.sh --chain localstack_v11 --fhevm-cli-profile v0.11.0-mainnet.json
  ./localstack-restart.sh --chain localstack_v12 --fhevm-cli-profile v0.12.0-testnet.json
  ./localstack-restart.sh --chain localstack_v13 --fhevm-cli-profile v0.13.0.json
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
        --fhevm-dir)
            require_arg_value "$1" "${2:-}"
            FHEVM_DIR="$2"
            shift 2
            ;;
        --fhevm-dir=*)
            FHEVM_DIR="${1#--fhevm-dir=}"
            shift
            ;;
        --force|-f)
            FORCE=true
            shift
            ;;
        --dry-run|-n)
            DRY_RUN=true
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

if [[ ! -d "$FHEVM_DIR" ]]; then
    echo "Error: --fhevm-dir '$FHEVM_DIR' is not a directory." >&2
    exit 1
fi
FHEVM_DIR="$(cd "$FHEVM_DIR" && pwd)"
PROFILES_DIR="$FHEVM_DIR/profiles"

if [[ -n "$PROFILE" ]]; then
    PROFILE_PATH="$PROFILES_DIR/$PROFILE"
    if [[ ! -f "$PROFILE_PATH" ]]; then
        echo "Error: profile '$PROFILE' not found at $PROFILE_PATH." >&2
        if [[ -d "$PROFILES_DIR" ]]; then
            echo "Available profiles:" >&2
            (cd "$PROFILES_DIR" && ls -1 *.json 2>/dev/null) | sed 's/^/  /' >&2
        fi
        exit 1
    fi
fi

port_is_listening() {
    lsof -nP -iTCP:"$PORT" -sTCP:LISTEN >/dev/null 2>&1
}

detect_rpc_client() {
    cast client --rpc-url "$RPC_URL" 2>/dev/null | tr '[:upper:]' '[:lower:]'
}

# Resolve a path to its canonical form (removes ".." and "." segments).
# Falls back to printing the input unchanged if neither the path nor its
# parent directory exists. Portable across macOS and Linux — does not
# rely on GNU `readlink -f` or `realpath`.
to_clean_path() {
    local path="$1"
    if [[ -d "$path" ]]; then
        (cd "$path" && pwd)
        return
    fi
    local dir base
    dir="$(dirname "$path")"
    base="$(basename "$path")"
    if [[ -d "$dir" ]]; then
        printf '%s/%s\n' "$(cd "$dir" && pwd)" "$base"
    else
        printf '%s\n' "$path"
    fi
}

anvil_is_listening() {
    [[ "$(detect_rpc_client)" == *anvil* ]]
}

verify_fhetest_deploy_or_exit() {
    local fhetest_address
    if ! fhetest_address="$(resolve_fhetest_address "$CHAIN" "$CHAIN_DEFAULTS_FILE")"; then
        exit 1
    fi

    local code
    if ! code="$(cast code "$fhetest_address" --rpc-url "$RPC_URL" 2>/dev/null)"; then
        echo "Error: failed to read FHETest bytecode at $fhetest_address on $RPC_URL." >&2
        exit 1
    fi
    if [[ -z "$code" || "$code" == "0x" ]]; then
        echo "Error: FHETest is not deployed at $fhetest_address on $RPC_URL." >&2
        echo "Use --force to restart and redeploy FHETest." >&2
        exit 1
    fi

    local contract_name
    if ! contract_name="$(cast call "$fhetest_address" "CONTRACT_NAME()(string)" --rpc-url "$RPC_URL" 2>/dev/null)"; then
        echo "Error: deployed contract at $fhetest_address does not expose FHETest CONTRACT_NAME()." >&2
        echo "Use --force to restart and redeploy FHETest." >&2
        exit 1
    fi
    if [[ "$contract_name" != "FHETestv2" && "$contract_name" != '"FHETestv2"' ]]; then
        echo "Error: unexpected FHETest CONTRACT_NAME() at $fhetest_address: $contract_name" >&2
        echo "Use --force to restart and redeploy FHETest." >&2
        exit 1
    fi

    echo "FHETest is deployed at $fhetest_address."
}

print_dry_run() {
    local chain_ts_file
    chain_ts_file="$(to_clean_path "$CHAINS_DIR/${CHAIN}.ts")"
    local chain_defaults_display
    chain_defaults_display="$(to_clean_path "$CHAIN_DEFAULTS_FILE")"
    local profile_display="(none — fhevm-cli will run without --lock-file)"
    if [[ -n "$PROFILE" ]]; then
        profile_display="$PROFILE_PATH"
    fi

    local chain_ts_status="exists"
    [[ -f "$chain_ts_file" ]] || chain_ts_status="MISSING"

    local chain_defaults_status="exists"
    [[ -f "$CHAIN_DEFAULTS_FILE" ]] || chain_defaults_status="MISSING"

    echo ""
    echo "🟡 --dry-run set; no commands will be executed."
    echo ""
    echo "Resolved configuration:"
    echo "  chain:               $CHAIN"
    echo "  chain TS file:       $chain_ts_file  ($chain_ts_status)"
    echo "  chain-defaults.json: $chain_defaults_display  ($chain_defaults_status)"
    echo "  profile:             $profile_display"
    echo "  force restart:       $FORCE"
    echo "  rpc port:            $PORT"
    echo "  fhevm-cli dir:       $FHEVM_DIR"
    echo "  contracts dir:       $CONTRACTS_DIR"
    echo ""
    echo "Current state:"
    if anvil_is_listening; then
        echo "  anvil on $PORT:        listening"
    elif port_is_listening; then
        echo "  anvil on $PORT:        NO — port held by a non-anvil process"
    else
        echo "  anvil on $PORT:        not listening"
    fi
    echo ""
    echo "Planned actions:"
    if [[ "$FORCE" = false ]] && anvil_is_listening; then
        echo "  → verify FHETest deploy, then exit 0 if healthy (use --force to redeploy)"
        return
    fi
    if [[ "$FORCE" = false ]] && port_is_listening; then
        echo "  → exit 1 — port $PORT is held by a non-anvil process"
        return
    fi
    echo "  cd $FHEVM_DIR"
    echo "  bun install"
    if [[ "$FORCE" = true ]]; then
        echo "  $FHEVM_DIR/fhevm-cli down"
    fi
    if [[ -n "$PROFILE" ]]; then
        echo "  $FHEVM_DIR/fhevm-cli up --lock-file profiles/$PROFILE"
    else
        echo "  $FHEVM_DIR/fhevm-cli up"
    fi
    echo "  cd $CONTRACTS_DIR"
    echo "  forge clean"
    echo "  ./scripts/fhetest-deploy.sh --chain $CHAIN"
    echo "  verify FHETest deploy"
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

if [[ "$DRY_RUN" = true ]]; then
    print_dry_run
    exit 0
fi

if [[ "$FORCE" = false ]]; then
    if anvil_is_listening; then
        echo "anvil is already listening on port ${PORT}; checking FHETest deploy."
        verify_fhetest_deploy_or_exit
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
if [[ -n "$PROFILE" ]]; then
    "$FHEVM_DIR/fhevm-cli" up --lock-file "profiles/$PROFILE"
else
    "$FHEVM_DIR/fhevm-cli" up
fi

# Deploy FHETest.sol
cd "$CONTRACTS_DIR"

forge clean

./scripts/fhetest-deploy.sh --chain "${CHAIN}"

verify_fhetest_deploy_or_exit
