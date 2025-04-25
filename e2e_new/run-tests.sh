#!/bin/bash

# Colors for messages
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RESET='\033[0m'

# Help message function
show_help() {
  echo -e "${YELLOW}Usage:${RESET} ./run-tests.sh [test-grep-text]"
  echo -e "Runs Hardhat tests with an optional grep filter."
  echo -e ""
  echo -e "${YELLOW}Examples:${RESET}"
  echo -e "  ./run-tests.sh                         (uses default grep: \"test user input uint64\")"
  echo -e "  ./run-tests.sh \"decryptionOracle test\"  (runs tests matching that text)"
  echo -e ""
  echo -e "${YELLOW}Options:${RESET}"
  echo -e "  -h, --help     Show this help message"
}

# Handle help option
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
  show_help
  exit 0
fi

# Get grep argument or fallback to default
GREP_TEXT=${1:-"test user input uint64"}
NETWORK=${2:-"staging"}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR" || exit 1

echo -e "${GREEN}Running input proof tests with grep: \"$GREP_TEXT\"...${RESET}"
RUST_RELAYER=true npx hardhat test --grep "$GREP_TEXT" --network "$NETWORK"
