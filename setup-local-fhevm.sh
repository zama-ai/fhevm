#!/bin/bash

# Assumes the following:
# 1. A local and **fresh** fhEVM node is already running.
# 2. All test addresses are funded (e.g. via the fund_test_addresses.sh script).

PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)
npx hardhat task:computePredeployAddress --private-key "$PRIVATE_KEY_GATEWAY_DEPLOYER"

npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

npx hardhat task:deployACL
npx hardhat task:deployTFHEExecutor
npx hardhat task:computeKmsVerifierAddress
npx hardhat task:deployKmsVerifier

npx hardhat task:launchFhevm --skip-get-coin true
