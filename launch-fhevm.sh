#!/bin/bash

PRIVATE_KEY_ORACLE_DEPLOYER=$(grep PRIVATE_KEY_ORACLE_DEPLOYER .env | cut -d '"' -f 2)

npx hardhat task:computePredeployAddress --private-key "$PRIVATE_KEY_ORACLE_DEPLOYER"

PRIVATE_KEY_ORACLE_RELAYER=$(grep PRIVATE_KEY_ORACLE_RELAYER .env | cut -d '"' -f 2)

ORACLE_CONTRACT_PREDEPLOY_ADDRESS=$(grep ORACLE_CONTRACT_PREDEPLOY_ADDRESS oracle/.env.oracle | cut -d '=' -f2)

docker run -d -i -p 8545:8545 --rm --name fhevm \
  -e PRIVATE_KEY_ORACLE_RELAYER="$PRIVATE_KEY_ORACLE_RELAYER" \
  -e ORACLE_CONTRACT_PREDEPLOY_ADDRESS="$ORACLE_CONTRACT_PREDEPLOY_ADDRESS" \
  ghcr.io/zama-ai/ethermint-dev-node:v0.3.0-5-async
  
sleep 10

npx hardhat compile:specific --contract lib

npx hardhat compile:specific --contract oracle

npx hardhat task:launchFhevm

docker attach fhevm