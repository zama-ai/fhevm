#!/bin/bash
npx hardhat clean

mkdir -p addresses

PRIVATE_KEY_DECRYPTION_ORACLE_DEPLOYER=$(grep PRIVATE_KEY_DECRYPTION_ORACLE_DEPLOYER .env | cut -d '"' -f 2)
PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)

npx hardhat task:computeDecryptionOracleAddress --private-key "$PRIVATE_KEY_DECRYPTION_ORACLE_DEPLOYER"
npx hardhat task:computeACLAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeTFHEExecutorAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeKMSVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeInputVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true
npx hardhat task:computeFHEGasLimitAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"