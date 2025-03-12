#!/bin/bash

NETWORK=${1:-"staging"}
npx hardhat clean
npx hardhat compile:specific --contract contracts/emptyProxy

mkdir -p addresses

PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)
NUM_COPROCESSOR_SIGNERS=$(grep NUM_COPROCESSOR_SIGNERS .env | cut -d '"' -f 2)
DECRYPTION_MANAGER_ADDRESS=$(grep DECRYPTION_MANAGER_ADDRESS .env | cut -d '"' -f 2)
ZKPOK_MANAGER_ADDRESS=$(grep ZKPOK_MANAGER_ADDRESS .env | cut -d '"' -f 2)

npx hardhat task:deployEmptyUUPSProxies --use-coprocessor-address true --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"

npx hardhat compile
npx hardhat compile:specific --contract decryptionOracle

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --decryption-manager-address "$DECRYPTION_MANAGER_ADDRESS" --network "$NETWORK"
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --zkpok-manager-address "$ZKPOK_MANAGER_ADDRESS"  --network "$NETWORK"
npx hardhat task:deployFHEGasLimit --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"
npx hardhat task:deployDecryptionOracle --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network "$NETWORK"

npx hardhat task:addSigners --num-signers "$NUM_KMS_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network "$NETWORK"
npx hardhat task:addInputSigners --num-signers "$NUM_COPROCESSOR_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network "$NETWORK"
