#!/bin/bash

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
RESET='\033[0m'

DEFAULT_GREP="test user input uint64"
DEFAULT_NETWORK="staging"
VERBOSE=false
NO_COMPILE=false

show_help() {
  echo -e "${BLUE}============================================================${RESET}"
  echo -e "${YELLOW}Fhevm Testing Script${RESET}"
  echo -e "${BLUE}============================================================${RESET}"
  echo -e "${YELLOW}Usage:${RESET} ./run-tests.sh [options] [test-grep-text]"
  echo -e ""
  echo -e "${YELLOW}Options:${RESET}"
  echo -e "  -h, --help          Show this help message"
  echo -e "  -g, --grep PATTERN  Specify test grep pattern (default: ${DEFAULT_GREP})"
  echo -e "  -n, --network NAME  Specify network (default: \$NETWORK, then \$HARDHAT_NETWORK, then ${DEFAULT_NETWORK})"
  echo -e "  --chain NAME        Run against a chain using its prefixed env vars (e.g. BNB_ACL_CONTRACT_ADDRESS):"
  echo -e "                      eth, polygon, bnb, hoodi, eth-mainnet, polygon-mainnet, bnb-mainnet."
  echo -e "                      Fails if a required var is missing; one run per container (e2e.lock)"
  echo -e "  -v, --verbose       Enable verbose output"
  echo -e "  --no-hardhat-compile        Skip Hardhat compilation step"
  echo -e ""
  echo -e "${YELLOW}Examples:${RESET}"
  echo -e "  ./run-tests.sh --chain bnb -g \"test user decrypt\" (runs on BNB testnet with the BNB_* vars)"
  echo -e "  ./run-tests.sh                         (uses default grep: \"${DEFAULT_GREP}\")"
  echo -e "  ./run-tests.sh -g \"test user input uint64\" (runs tests matching that text)"
  echo -e "  ./run-tests.sh -n staging -g \"my test\"  (runs on staging network)"
  echo -e "  ./run-tests.sh \"my test\"              (positional argument still works)"
  echo -e "${BLUE}============================================================${RESET}"
}

# Parse options
PARAMS=""
GREP_PARAM=""
HARDHAT_PARALLEL=""
CHAIN=""
NETWORK_PARAM=""
while (( "$#" )); do
  case "$1" in
    --parallel)
      HARDHAT_PARALLEL="--parallel"
      shift
      ;;
    -h|--help)
      show_help
      exit 0
      ;;
    -g|--grep)
      if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
        GREP_PARAM=$2
        shift 2
      else
        echo -e "${RED}Error: Argument for $1 is missing${RESET}" >&2
        exit 1
      fi
      ;;
    -n|--network)
      if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
        NETWORK=$2
        NETWORK_PARAM=$2
        shift 2
      else
        echo -e "${RED}Error: Argument for $1 is missing${RESET}" >&2
        exit 1
      fi
      ;;
    --chain)
      if [ -n "$2" ] && [ ${2:0:1} != "-" ]; then
        CHAIN=$2
        shift 2
      else
        echo -e "${RED}Error: Argument for $1 is missing${RESET}" >&2
        exit 1
      fi
      ;;
    -v|--verbose)
      VERBOSE=true
      shift
      ;;
    --no-hardhat-compile)
      NO_COMPILE=true
      shift
      ;;
    -*)
      # Reject unknown options instead of treating them as the grep text, so a typo (or an image
      # that predates an option) fails loudly instead of running the wrong tests.
      echo -e "${RED}Error: Unknown option $1 (see --help)${RESET}" >&2
      exit 1
      ;;
    *)
      PARAMS="$PARAMS $1"
      shift
      ;;
  esac
done

if [ -n "$CHAIN" ] && [ "$NO_COMPILE" = true ]; then
  # --chain regenerates the coprocessor config, which must be compiled.
  echo -e "${RED}Error: --chain cannot be combined with --no-hardhat-compile${RESET}" >&2
  exit 1
fi

eval set -- "$PARAMS"
# Priority: explicit grep parameter > positional argument > default
GREP_TEXT=${GREP_PARAM:-${1:-"$DEFAULT_GREP"}}
NETWORK=${NETWORK:-${HARDHAT_NETWORK:-"$DEFAULT_NETWORK"}}
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR" || {
  echo -e "${RED}Failed to navigate to script directory${RESET}" >&2
  exit 1
}

# One --chain run per container: runs share the generated contracts/E2ECoprocessorConfigLocal.sol,
# artifacts/ and cache/, so a concurrent run on another chain could deploy the wrong addresses.
# Only --chain runs take the lock (runs without --chain keep their current behaviour).
# The lock file holds the owner's PID; a lock whose process is gone (e.g. killed run) is stale.
LOCK_FILE="$SCRIPT_DIR/e2e.lock"

lock_owner_pid() {
  sed -n 's/^pid=\([0-9]*\).*/\1/p' "$LOCK_FILE" 2>/dev/null
}

release_lock() {
  if [ "$(lock_owner_pid)" = "$$" ]; then
    rm -f "$LOCK_FILE"
  fi
}

# A run whose bash was killed can leave hardhat running; don't treat its lock as stale then.
hardhat_test_running() {
  command -v pgrep >/dev/null 2>&1 && pgrep -f '[h]ardhat test' >/dev/null 2>&1
}

acquire_lock() {
  local info="pid=$$ chain=$CHAIN grep=\"$GREP_TEXT\" started=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  local tmp="$LOCK_FILE.$$"
  local owner content
  printf '%s\n' "$info" >"$tmp" || return 1
  for _ in 1 2 3; do
    # Hard-linking a fully written file is atomic and fails if the lock already exists.
    if ln "$tmp" "$LOCK_FILE" 2>/dev/null; then
      rm -f "$tmp"
      trap release_lock EXIT
      return 0
    fi
    content=$(cat "$LOCK_FILE" 2>/dev/null) || continue # released in the meantime: retry
    owner=$(lock_owner_pid)
    if [ -z "$owner" ] || kill -0 "$owner" 2>/dev/null || hardhat_test_running; then
      rm -f "$tmp"
      echo -e "${RED}Error: another e2e run is in progress in this container${RESET}" >&2
      echo -e "  ${content:-(lock file $LOCK_FILE has no owner details)}" >&2
      if [ -n "$owner" ] && ! kill -0 "$owner" 2>/dev/null; then
        echo -e "  (that run's shell has exited, but a 'hardhat test' process is still running)" >&2
      fi
      echo -e "Wait for it to finish, or stop it, and try again." >&2
      return 1
    fi
    # Stale: remove it only if it still holds the same dead owner.
    if [ "$(cat "$LOCK_FILE" 2>/dev/null)" = "$content" ]; then
      echo -e "${YELLOW}Removing stale $LOCK_FILE ($content)${RESET}" >&2
      rm -f "$LOCK_FILE"
    fi
  done
  rm -f "$tmp"
  echo -e "${RED}Error: could not acquire $LOCK_FILE${RESET}" >&2
  return 1
}

if [ -n "$CHAIN" ]; then
  acquire_lock || exit 1
  # Validates the chain's prefixed env vars and prints the unprefixed exports (see the script).
  if ! CHAIN_ENV=$(npx ts-node --transpile-only scripts/resolve-chain-env.ts --chain "$CHAIN" \
    ${NETWORK_PARAM:+--network "$NETWORK_PARAM"}); then
    echo -e "${RED}Error: invalid environment for --chain $CHAIN${RESET}" >&2
    exit 1
  fi
  # Only eval the expected `export NAME='value'` / `unset NAME` lines.
  if printf '%s\n' "$CHAIN_ENV" | grep -qvE "^(export [A-Z0-9_]+='.*'|unset [A-Z0-9_]+)$"; then
    echo -e "${RED}Error: unexpected output from resolve-chain-env.ts:${RESET}" >&2
    printf '%s\n' "$CHAIN_ENV" | grep -vE "^(export [A-Z0-9_]+='.*'|unset [A-Z0-9_]+)$" >&2
    exit 1
  fi
  eval "$CHAIN_ENV"
fi

# Display configuration
echo -e "${BLUE}============================================================${RESET}"
echo -e "${GREEN}Test Configuration:${RESET}"
echo -e "  Test filter: ${YELLOW}\"$GREP_TEXT\"${RESET}"
if [ -n "$CHAIN" ]; then
  echo -e "  Chain:       ${YELLOW}$CHAIN${RESET}"
fi
echo -e "  Network:     ${YELLOW}$NETWORK${RESET}"
if [ "$VERBOSE" = true ]; then
  echo -e "  Verbose:     ${YELLOW}Enabled${RESET}"
fi
if [ "$NO_COMPILE" = true ]; then
  echo -e "  Compile:     ${YELLOW}Disabled${RESET}"
fi
echo -e "${BLUE}============================================================${RESET}"

trap cleanup SIGINT SIGTERM

# Regenerate the coprocessor config from env vars before compilation (opt-in via
# E2E_COPROCESSOR_CONFIG_FROM_ENV=true; a no-op otherwise)
COPROCESSOR_CONFIG_SOL="contracts/E2ECoprocessorConfigLocal.sol"
COPROCESSOR_CONFIG_BEFORE=$(cksum "$COPROCESSOR_CONFIG_SOL")
GEN_CONFIG_OPTS=""
if [ "$NO_COMPILE" = true ]; then
  GEN_CONFIG_OPTS="--no-compile"
fi
if ! npx ts-node --transpile-only scripts/generate-coprocessor-config.ts ${GEN_CONFIG_OPTS}; then
  echo -e "${RED}Error: failed to generate the coprocessor config${RESET}" >&2
  exit 1
fi
# The custom `test` task in hardhat.config.ts narrows the compile sources, so `hardhat test`
# does not recompile contracts/. Compile explicitly when the config file changed.
if [ "$(cksum "$COPROCESSOR_CONFIG_SOL")" != "$COPROCESSOR_CONFIG_BEFORE" ]; then
  echo -e "${GREEN}Coprocessor config changed, compiling contracts...${RESET}"
  if ! npx hardhat compile; then
    echo -e "${RED}Error: failed to compile the contracts${RESET}" >&2
    exit 1
  fi
fi

echo -e "\n${GREEN}Running tests...${RESET}"

HARDHAT_OPTS="${HARDHAT_PARALLEL} "
if [ "$VERBOSE" = true ]; then
  HARDHAT_OPTS+=" --verbose "
fi
if [ "$NO_COMPILE" = true ]; then
  HARDHAT_OPTS+=" --no-compile "
fi

echo hardhat test ${HARDHAT_OPTS} --grep "$GREP_TEXT" --network "$NETWORK"

# Run the tests
if npx hardhat test ${HARDHAT_OPTS} --grep "$GREP_TEXT" --network "$NETWORK"; then
  echo -e "\n${GREEN}✓ Tests completed successfully!${RESET}"
else
  echo -e "\n${RED}✗ Tests failed!${RESET}"
  exit 1
fi
