#!/bin/bash
pnpm exec hardhat clean

pnpm exec hardhat compile

DEPLOYER_ADDRESS=$(grep DEPLOYER_ADDRESS .env | cut -d '"' -f 2)
DEPLOYER_PRIVATE_KEY=$(grep DEPLOYER_PRIVATE_KEY .env | cut -d '"' -f 2)

# Deploy HTTPZ contract
pnpm exec hardhat task:deployHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network staging

# Deploy ZKPoKManager contract
pnpm exec hardhat task:deployZkpokManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network staging

# Coprocessor
COPROCESSOR_ADDRESS=$(grep COPROCESSOR_ADDRESS .env | cut -d '"' -f 2)
COPROCESSOR_PUBLIC_KEY=$(grep COPROCESSOR_PUBLIC_KEY .env | cut -d '"' -f 2)

# Network
NETWORK_CHAIN_ID_1=$(grep NETWORK_CHAIN_ID_1 .env | cut -d '"' -f 2)

# Initialize HTTPZ contract - register KMS nodes and coprocessor (transaction-sender services) and netwrok chainID
## for simplicity admin is the contract deployer/owner - should be different in real scenarios
pnpm exec hardhat task:initHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" \
    --admin-private-key "$DEPLOYER_PRIVATE_KEY" \
    --protocol-metadata "{\"website\":\"test\",\"name\":\"test\"}" \
    --admin-addresses "[\"${DEPLOYER_ADDRESS}\"]" \
    --kms-threshold 0 \
    --kms-nodes "[{\"connectorAddress\":\"0x1234567890abcdef1234567890abcdef12345678\",\"identity\":\"0x1234567890abcdef1234567890abcdef12345678\",\"ipAddress\":\"\",\"signedNodes\": [],\"daAddress\": \"0x1234567890abcdef1234567890abcdef12345678\"}]" \
    --coprocessors "[{\"connectorAddress\":\"${COPROCESSOR_ADDRESS}\",\"identity\":\"${COPROCESSOR_PUBLIC_KEY}\",\"daAddress\": \"0x1234567890abcdef1234567890abcdef12345678\"}]" \
    --layer1-network "{\"chainId\":${NETWORK_CHAIN_ID_1},\"httpzLibrary\":\"0x1234567890abcdef1234567890abcdef12345678\",\"acl\":\"0x1234567890abcdef1234567890abcdef12345678\",\"name\":\"\",\"website\":\"\"}" \
    --network staging
