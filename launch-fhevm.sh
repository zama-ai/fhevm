#!/bin/bash
npx hardhat clean

PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)
PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)
IS_COPROCESSOR=$(grep IS_COPROCESSOR .env | cut -d '"' -f 2)

npx hardhat task:computeGatewayAddress --private-key "$PRIVATE_KEY_GATEWAY_DEPLOYER"
npx hardhat task:computeACLAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeTFHEExecutorAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeKMSVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeInputVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeFHEPaymentAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"

# [ADD DOCKER-COMPOSE COMMAND HERE] // Geth node, Gateway service, KMS service etc should be launched here, using previously precomputed addresses

if [ "$IS_COPROCESSOR" = "true" ]; then
    cp lib/InputVerifier.sol.coprocessor lib/InputVerifier.sol
else
    cp lib/InputVerifier.sol.native lib/InputVerifier.sol
fi
npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployFHEPayment --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"

npx hardhat task:addSigners --num-signers $NUM_KMS_SIGNERS --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --useAddress true

npx hardhat task:launchFhevm --skip-get-coin true