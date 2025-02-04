#!/bin/bash
npx hardhat compile:specific --contract kmsContracts

mkdir -p addressesL2

PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)


npx hardhat task:deployDecryptionManager --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network localCoprocessorL2

npx hardhat task:addSignersL2 --num-signers "$NUM_KMS_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network localCoprocessorL2