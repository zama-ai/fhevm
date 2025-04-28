#!/bin/bash

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
RESET='\033[0m'

DEFAULT_GREP="test user input uint64"
DEFAULT_NETWORK="staging"
VERBOSE=false
RUST_RELAYER=true

show_help() {
  echo -e "${BLUE}============================================================${RESET}"
  echo -e "${YELLOW}Fhevm Testing Script${RESET}"
  echo -e "${BLUE}============================================================${RESET}"
  echo -e "${YELLOW}Usage:${RESET} ./run-tests.sh [options] [test-grep-text]"
  echo -e ""
  echo -e "${YELLOW}Options:${RESET}"
  echo -e "  -h, --help          Show this help message"
  echo -e "  -g, --grep PATTERN  Specify test grep pattern (default: ${DEFAULT_GREP})"
  echo -e "  -n, --network NAME  Specify network (default: ${DEFAULT_NETWORK})"
  echo -e "  -v, --verbose       Enable verbose output"
  echo -e "  -r, --no-relayer    Disable Rust relayer"
  echo -e ""
  echo -e "${YELLOW}Examples:${RESET}"
  echo -e "  ./run-tests.sh                         (uses default grep: \"${DEFAULT_GREP}\")"
  echo -e "  ./run-tests.sh -g \"test user input uint64\" (runs tests matching that text)"
  echo -e "  ./run-tests.sh -n staging -g \"my test\"  (runs on staging network)"
  echo -e "  ./run-tests.sh \"my test\"              (positional argument still works)"
  echo -e "${BLUE}============================================================${RESET}"
}

# Parse options
PARAMS=""
GREP_PARAM=""
while (( "$#" )); do
  case "$1" in
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
    -r|--no-relayer)
      RUST_RELAYER=false
      shift
      ;;
    *)
      PARAMS="$PARAMS $1"
      shift
      ;;
  esac
done

eval set -- "$PARAMS"
# Priority: explicit grep parameter > positional argument > default
GREP_TEXT=${GREP_PARAM:-${1:-"$DEFAULT_GREP"}}
NETWORK=${NETWORK:-"$DEFAULT_NETWORK"}
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR" || {
  echo -e "${RED}Failed to navigate to script directory${RESET}" >&2
  exit 1
}

# Display configuration
echo -e "${BLUE}============================================================${RESET}"
echo -e "${GREEN}Test Configuration:${RESET}"
echo -e "  Test filter: ${YELLOW}\"$GREP_TEXT\"${RESET}"
echo -e "  Network:     ${YELLOW}$NETWORK${RESET}"
echo -e "  Rust Relayer:${YELLOW}$([ "$RUST_RELAYER" = true ] && echo " Enabled" || echo " Disabled")${RESET}"
if [ "$VERBOSE" = true ]; then
  echo -e "  Verbose:     ${YELLOW}Enabled${RESET}"
fi
echo -e "${BLUE}============================================================${RESET}"

trap cleanup SIGINT SIGTERM

echo -e "\n${GREEN}Running tests...${RESET}"

HARDHAT_OPTS=""
if [ "$VERBOSE" = true ]; then
  HARDHAT_OPTS="--verbose"
fi

# Run the tests
if RUST_RELAYER=$RUST_RELAYER npx hardhat test $HARDHAT_OPTS --grep "$GREP_TEXT" --network "$NETWORK"; then
  echo -e "\n${GREEN}✓ Tests completed successfully!${RESET}"
else
  echo -e "\n${RED}✗ Tests failed!${RESET}"
  exit 1
fi