#!/bin/bash

PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)

npx hardhat task:computePredeployAddress --private-key "$PRIVATE_KEY_GATEWAY_DEPLOYER"

PRIVATE_KEY_GATEWAY_RELAYER=$(grep PRIVATE_KEY_GATEWAY_RELAYER .env | cut -d '"' -f 2)

GATEWAY_CONTRACT_PREDEPLOY_ADDRESS=$(grep GATEWAY_CONTRACT_PREDEPLOY_ADDRESS gateway/.env.gateway | cut -d '=' -f2)

TFHE_EXECUTOR_CONTRACT_ADDRESS=$(grep TFHE_EXECUTOR_CONTRACT_ADDRESS lib/.env.exec | cut -d '=' -f2)

docker run -d -i -p 8545:8545 --rm --name fhevm \
  -e PRIVATE_KEY_ORACLE_RELAYER="$PRIVATE_KEY_ORACLE_RELAYER" \
  -e ORACLE_CONTRACT_PREDEPLOY_ADDRESS="$ORACLE_CONTRACT_PREDEPLOY_ADDRESS" \
  -e TFHE_EXECUTOR_CONTRACT_ADDRESS="$TFHE_EXECUTOR_CONTRACT_ADDRESS" \
  ghcr.io/zama-ai/ethermint-dev-node:v0.5.0-1
  
sleep 10

npx hardhat task:computeACLAddress
npx hardhat task:computeTFHEExecutorAddress
npx hardhat task:computeKMSVerifierAddress
npx hardhat task:deployACL
npx hardhat task:deployTFHEExecutor
npx hardhat task:deployKMSVerifier

npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

npx hardhat task:launchFhevm

docker attach fhevm
