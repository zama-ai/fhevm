#!/bin/bash
# Default value
DEFAULT_NETWORK="localHTTPZGateway"

# Get the first argument ($1) if provided, otherwise use DEFAULT_VALUE
# The ${1:-default_value} syntax means "use $1 if it exists, otherwise use default_value"
NETWORK=${1:-$DEFAULT_NETWORK}

npx hardhat clean

npx hardhat compile

# Deployer
DEPLOYER_ADDRESS=$(grep DEPLOYER_ADDRESS .env | cut -d '"' -f 2)
DEPLOYER_PRIVATE_KEY=$(grep DEPLOYER_PRIVATE_KEY .env | cut -d '"' -f 2)

# Coprocessor
# Coprocessor address is the transaction sender's address
COPROCESSOR_ADDRESS=$(grep COPROCESSOR_ADDRESS .env | cut -d '"' -f 2)

# Network
NETWORK_CHAIN_ID_1=$(grep NETWORK_CHAIN_ID_1 .env | cut -d '"' -f 2)

# Dummy address
# This is used for inputting dummy addresses or any bytes type when deploying the HTTPZ contract as passing
# empty strings raises an error
DUMMY_HEX_BYTES="0x1234567890abcdef1234567890abcdef12345678"
# Also, we need to add a dummy KMS node, else deploying the contract will fail because the threshold will be too low
DUMMY_KMS_NODE="[{\"connectorAddress\":\"${DUMMY_HEX_BYTES}\",\"identity\":\"${DUMMY_HEX_BYTES}\",\"ipAddress\":\"\",\"daAddress\": \"\", \"tlsCertificate\":\"${DUMMY_HEX_BYTES}\"}]"

echo "Deploy HTTPZ contract:"
# Deploy HTTPZ contract - register KMS nodes and coprocessor (transaction-sender services) and network chainID
# for simplicity admin is the contract deployer/owner - should be different in real scenarios
npx hardhat task:deployHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" \
    --admin-private-key "$DEPLOYER_PRIVATE_KEY" \
    --protocol-metadata "{\"website\":\"test\",\"name\":\"test\"}" \
    --admin-addresses "[\"${DEPLOYER_ADDRESS}\"]" \
    --kms-threshold 0 \
    --kms-nodes "${DUMMY_KMS_NODE}" \
    --coprocessors "[{\"transactionSenderAddress\":\"${COPROCESSOR_ADDRESS}\",\"identity\":\"${DUMMY_HEX_BYTES}\",\"daAddress\": \"\"}]" \
    --layer1-networks "[{\"chainId\":${NETWORK_CHAIN_ID_1},\"httpzExecutor\":\"${DUMMY_HEX_BYTES}\",\"aclAddress\":\"${DUMMY_HEX_BYTES}\",\"name\":\"\",\"website\":\"\"}]" \
    --network $NETWORK

echo "Deploy ZKPoKManager contract:"
# Deploy ZKPoKManager contract
npx hardhat task:deployZkpokManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

