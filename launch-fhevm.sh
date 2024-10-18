#!/bin/bash
# This script should be launched after precomputing the addresses via `precompute-addresses.sh`, 
# and preferably after setting up the different services - KMS, Geth node, Gateway

# Check if the network argument is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <network>"
  exit 1
fi

NETWORK=$1

npx hardhat clean

PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)

npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"
npx hardhat task:deployFHEPayment --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"

npx hardhat task:addSigners --num-signers "$NUM_KMS_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network "$NETWORK"

npx hardhat task:launchFhevm --skip-get-coin true --use-address true --network "$NETWORK"

