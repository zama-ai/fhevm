#!/bin/bash
# Default value
DEFAULT_NETWORK="localCoprocessorL1"

# Get the first argument ($1) if provided, otherwise use DEFAULT_VALUE
# The ${1:-default_value} syntax means "use $1 if it exists, otherwise use default_value"
NETWORK=${1:-$DEFAULT_NETWORK}

npm i
npx hardhat clean
npx hardhat compile:specific --contract contracts/emptyProxy

mkdir -p addresses

PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)
NUM_COPROCESSOR_SIGNERS=$(grep NUM_COPROCESSOR_SIGNERS .env | cut -d '"' -f 2)

npx hardhat task:deployEmptyUUPSProxies --use-coprocessor-address true --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK

npx hardhat compile
npx hardhat compile:specific --contract decryptionOracle

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK
npx hardhat task:deployFHEGasLimit --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK
npx hardhat task:deployDecryptionOracle --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK

npx hardhat task:addSigners --num-signers "$NUM_KMS_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network $NETWORK
npx hardhat task:addInputSigners --num-signers "$NUM_COPROCESSOR_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network $NETWORK