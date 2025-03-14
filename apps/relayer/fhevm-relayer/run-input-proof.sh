#!/bin/bash

# Colors for messages
GREEN='\033[0;32m'
RESET='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/hardhat/contracts" || exit 1

echo -e "${GREEN}Running input proof tests...${RESET}"
npx hardhat compile
npx hardhat compile:specific --contract decryptionOracle
RUST_RELAYER=true npx hardhat test --grep "test user input uint64" --network staging