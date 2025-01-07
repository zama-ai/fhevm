#!/bin/bash
npx hardhat clean
npx hardhat compile:specific --contract contracts/emptyProxy

mkdir -p addresses

PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)

npx hardhat task:deployEmptyUUPSProxies --use-coprocessor-address true --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia

npx hardhat compile
npx hardhat compile:specific --contract decryptionOracle

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployFHEGasLimit --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployDecryptionOracle --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia

npx hardhat task:addSigners --num-signers "$NUM_KMS_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network sepolia

echo "Waiting 2 minutes before contract verification... Please wait..."
sleep 120 # makes sure that contracts bytescode propagates on Etherscan, otherwise contracts verification might fail in next step
npx hardhat task:verifyACL --network sepolia
npx hardhat task:verifyTFHEExecutor --network sepolia
npx hardhat task:verifyKMSVerifier --network sepolia
npx hardhat task:verifyInputVerifier --network sepolia
npx hardhat task:verifyFHEGasLimit --network sepolia
npx hardhat task:verifyDecryptionOracle --network sepolia