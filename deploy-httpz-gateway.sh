#!/bin/bash
# Default value
DEFAULT_NETWORK="localHTTPZGateway"

# Get the first argument ($1) if provided, otherwise use DEFAULT_VALUE
# The ${1:-default_value} syntax means "use $1 if it exists, otherwise use default_value"
NETWORK=${1:-$DEFAULT_NETWORK}

npx hardhat clean

npx hardhat compile

# Deployer
DEPLOYER_PRIVATE_KEY=$(grep DEPLOYER_PRIVATE_KEY .env | cut -d '"' -f 2)

# Number of admins, KMS nodes, coprocessors and networks
NUM_ADMINS=$(grep NUM_ADMINS .env | cut -d '"' -f 2)
NUM_KMS_NODES=$(grep NUM_KMS_NODES .env | cut -d '"' -f 2)
NUM_COPROCESSORS=$(grep NUM_COPROCESSORS .env | cut -d '"' -f 2)
NUM_NETWORKS=$(grep NUM_NETWORKS .env | cut -d '"' -f 2)

echo "Deploy HTTPZ contract:"
# Deploy HTTPZ contract
npx hardhat task:deployHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" \
    --num-admins "$NUM_ADMINS" \
    --num-kms-nodes "$NUM_KMS_NODES" \
    --num-coprocessors "$NUM_COPROCESSORS" \
    --num-networks "$NUM_NETWORKS" \
    --network $NETWORK

# Important: the contract deployment order is currently important as some contracts depend on others

echo "Deploy ZKPoKManager contract:"
# Deploy ZKPoKManager contract
npx hardhat task:deployZkpokManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy KeyManager contract:"
# Deploy KeyManager contract
npx hardhat task:deployKeyManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy CiphertextStorage contract:"
# Deploy CiphertextStorage contract
npx hardhat task:deployCiphertextStorage --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy ACLManager contract:"
# Deploy ACLManager contract
npx hardhat task:deployAclManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy DecryptionManager contract:"
# Deploy DecryptionManager contract
npx hardhat task:deployDecryptionManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK