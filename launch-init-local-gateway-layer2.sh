#!/bin/bash
pnpm exec hardhat clean

pnpm exec hardhat compile

DEPLOYER_PRIVATE_KEY=$(grep DEPLOYER_PRIVATE_KEY .env | cut -d '"' -f 2)

# Deploy HTTPZ contract
pnpm exec hardhat task:deployHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network localGatewayL2

# Deploy ZKPoKManager contract
pnpm exec hardhat task:deployZkpokManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network localGatewayL2

# Protocol
PROTOCOL_NAME=$(grep PROTOCOL_NAME .env | cut -d '"' -f 2)
PROTOCOL_WEBSITE=$(grep PROTOCOL_WEBSITE .env | cut -d '"' -f 2)

# Admin
ADMIN_PRIVATE_KEY_1=$(grep ADMIN_PRIVATE_KEY_1 .env | cut -d '"' -f 2)
ADMIN_ADDRESS_1=$(grep ADMIN_ADDRESS_1 .env | cut -d '"' -f 2)

# KMS Threshold
KMS_THRESHOLD=$(grep KMS_THRESHOLD .env | cut -d '"' -f 2)

# KMS Node 1
KMS_NODE_ADDRESS_1=$(grep KMS_NODE_ADDRESS_1 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_1=$(grep KMS_NODE_PUBLIC_KEY_1 .env | cut -d '"' -f 2)

# KMS Node 2
KMS_NODE_ADDRESS_2=$(grep KMS_NODE_ADDRESS_2 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_2=$(grep KMS_NODE_PUBLIC_KEY_2 .env | cut -d '"' -f 2)

# KMS Node 3
KMS_NODE_ADDRESS_3=$(grep KMS_NODE_ADDRESS_3 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_3=$(grep KMS_NODE_PUBLIC_KEY_3 .env | cut -d '"' -f 2)

# KMS Node 4
KMS_NODE_ADDRESS_4=$(grep KMS_NODE_ADDRESS_4 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_4=$(grep KMS_NODE_PUBLIC_KEY_4 .env | cut -d '"' -f 2)

# Coprocessor 1
COPROCESSOR_ADDRESS_1=$(grep COPROCESSOR_ADDRESS_1 .env | cut -d '"' -f 2)
COPROCESSOR_PUBLIC_KEY_1=$(grep COPROCESSOR_PUBLIC_KEY_1 .env | cut -d '"' -f 2)

# Coprocessor 2
COPROCESSOR_ADDRESS_2=$(grep COPROCESSOR_ADDRESS_2 .env | cut -d '"' -f 2)
COPROCESSOR_PUBLIC_KEY_2=$(grep COPROCESSOR_PUBLIC_KEY_2 .env | cut -d '"' -f 2)

# Coprocessor 3
COPROCESSOR_ADDRESS_3=$(grep COPROCESSOR_ADDRESS_3 .env | cut -d '"' -f 2)
COPROCESSOR_PUBLIC_KEY_3=$(grep COPROCESSOR_PUBLIC_KEY_3 .env | cut -d '"' -f 2)

# Network
NETWORK_CHAIN_ID_1=$(grep NETWORK_CHAIN_ID_1 .env | cut -d '"' -f 2)
NETWORK_HTTPZ_LIBRARY_1=$(grep NETWORK_HTTPZ_LIBRARY_1 .env | cut -d '"' -f 2)
NETWORK_ACL_1=$(grep NETWORK_ACL_1 .env | cut -d '"' -f 2)
NETWORK_NAME_1=$(grep NETWORK_NAME_1 .env | cut -d '"' -f 2)
NETWORK_WEBSITE_1=$(grep NETWORK_WEBSITE_1 .env | cut -d '"' -f 2)

# Initialize HTTPZ contract
pnpm exec hardhat task:initHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" \
    --admin-private-key "$ADMIN_PRIVATE_KEY_1" \
    --protocol-metadata "{\"website\":\"${PROTOCOL_WEBSITE}\",\"name\":\"${PROTOCOL_NAME}\"}" \
    --admin-addresses '["'$ADMIN_ADDRESS_1'"]' \
    --kms-threshold $KMS_THRESHOLD \
    --kms-nodes "[{\"connectorAddress\":\"${KMS_NODE_ADDRESS_1}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_1}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_1}\"},{\"connectorAddress\":\"${KMS_NODE_ADDRESS_2}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_2}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_2}\"},{\"connectorAddress\":\"${KMS_NODE_ADDRESS_3}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_3}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_3}\"},{\"connectorAddress\":\"${KMS_NODE_ADDRESS_4}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_4}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_4}\"}]" \
    --coprocessors "[{\"connectorAddress\":\"${COPROCESSOR_ADDRESS_1}\",\"identity\":\"${COPROCESSOR_PUBLIC_KEY_1}\"},{\"connectorAddress\":\"${COPROCESSOR_ADDRESS_2}\",\"identity\":\"${COPROCESSOR_PUBLIC_KEY_2}\"},{\"connectorAddress\":\"${COPROCESSOR_ADDRESS_3}\",\"identity\":\"${COPROCESSOR_PUBLIC_KEY_3}\"}]" \
    --layer1-network "{\"chainId\":${NETWORK_CHAIN_ID_1},\"httpzLibrary\":\"${NETWORK_HTTPZ_LIBRARY_1}\",\"acl\":\"${NETWORK_ACL_1}\",\"name\":\"${NETWORK_NAME_1}\",\"website\":\"${NETWORK_WEBSITE_1}\"}" \
    --network localGatewayL2
