#!/bin/bash
#
# Default value
DEFAULT_NETWORK="localCoprocessorL2"

# Get the first argument ($1) if provided, otherwise use DEFAULT_VALUE
# The ${1:-default_value} syntax means "use $1 if it exists, otherwise use default_value"
NETWORK=${1:-$DEFAULT_NETWORK}
npm i
npx hardhat clean
npx hardhat compile:specific --contract kmsContracts

mkdir -p addressesL2

PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)


npx hardhat task:deployDecryptionManager --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK
npx hardhat task:deployHttpz --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK
npx hardhat task:deployZkPoKManager --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network $NETWORK

npx hardhat task:addSignersL2 --num-signers "$NUM_KMS_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network $NETWORK
