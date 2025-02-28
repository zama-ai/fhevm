#!/bin/bash
# Default value
DEFAULT_NETWORK="localHTTPZGateway"

# Get the first argument ($1) if provided, otherwise use DEFAULT_VALUE
# The ${1:-default_value} syntax means "use $1 if it exists, otherwise use default_value"
NETWORK=${1:-$DEFAULT_NETWORK}

pnpm exec hardhat clean

pnpm exec hardhat compile

# Deployer
DEPLOYER_PRIVATE_KEY=$(grep DEPLOYER_PRIVATE_KEY .env | cut -d '"' -f 2)

# Protocol
PROTOCOL_NAME=$(grep PROTOCOL_NAME .env | cut -d '"' -f 2)
PROTOCOL_WEBSITE=$(grep PROTOCOL_WEBSITE .env | cut -d '"' -f 2)

# Admin
ADMIN_PRIVATE_KEY_1=$(grep ADMIN_PRIVATE_KEY_1 .env | cut -d '"' -f 2)
ADMIN_ADDRESS_1=$(grep ADMIN_ADDRESS_1 .env | cut -d '"' -f 2)

# KMS Threshold
KMS_THRESHOLD=$(grep KMS_THRESHOLD .env | cut -d '"' -f 2)

# KMS Nodes
# KMS node addresses are the KMS connector's addresses
# KMS Node 1
KMS_NODE_ADDRESS_1=$(grep KMS_NODE_ADDRESS_1 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_1=$(grep KMS_NODE_PUBLIC_KEY_1 .env | cut -d '"' -f 2)
KMS_NODE_DA_ADDRESS_1=$(grep KMS_NODE_DA_ADDRESS_1 .env | cut -d '"' -f 2)
KMS_NODE_TLS_CERTIFICATE_1=$(grep KMS_NODE_TLS_CERTIFICATE_1 .env | cut -d '"' -f 2)

# KMS Node 2
KMS_NODE_ADDRESS_2=$(grep KMS_NODE_ADDRESS_2 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_2=$(grep KMS_NODE_PUBLIC_KEY_2 .env | cut -d '"' -f 2)
KMS_NODE_DA_ADDRESS_2=$(grep KMS_NODE_DA_ADDRESS_2 .env | cut -d '"' -f 2)
KMS_NODE_TLS_CERTIFICATE_2=$(grep KMS_NODE_TLS_CERTIFICATE_2 .env | cut -d '"' -f 2)

# KMS Node 3
KMS_NODE_ADDRESS_3=$(grep KMS_NODE_ADDRESS_3 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_3=$(grep KMS_NODE_PUBLIC_KEY_3 .env | cut -d '"' -f 2)
KMS_NODE_DA_ADDRESS_3=$(grep KMS_NODE_DA_ADDRESS_3 .env | cut -d '"' -f 2)
KMS_NODE_TLS_CERTIFICATE_3=$(grep KMS_NODE_TLS_CERTIFICATE_3 .env | cut -d '"' -f 2)

# KMS Node 4
KMS_NODE_ADDRESS_4=$(grep KMS_NODE_ADDRESS_4 .env | cut -d '"' -f 2)
KMS_NODE_PUBLIC_KEY_4=$(grep KMS_NODE_PUBLIC_KEY_4 .env | cut -d '"' -f 2)
KMS_NODE_DA_ADDRESS_4=$(grep KMS_NODE_DA_ADDRESS_4 .env | cut -d '"' -f 2)
KMS_NODE_TLS_CERTIFICATE_4=$(grep KMS_NODE_TLS_CERTIFICATE_4 .env | cut -d '"' -f 2)

# Coprocessors
# Coprocessor addresses are the transaction senders' addresses
# Coprocessor 1
COPROCESSOR_ADDRESS_1=$(grep COPROCESSOR_ADDRESS_1 .env | cut -d '"' -f 2)
COPROCESSOR_PUBLIC_KEY_1=$(grep COPROCESSOR_PUBLIC_KEY_1 .env | cut -d '"' -f 2)
COPROCESSOR_DA_ADDRESS_1=$(grep COPROCESSOR_DA_ADDRESS_1 .env | cut -d '"' -f 2)

# Coprocessor 2
COPROCESSOR_ADDRESS_2=$(grep COPROCESSOR_ADDRESS_2 .env | cut -d '"' -f 2)
COPROCESSOR_PUBLIC_KEY_2=$(grep COPROCESSOR_PUBLIC_KEY_2 .env | cut -d '"' -f 2)
COPROCESSOR_DA_ADDRESS_2=$(grep COPROCESSOR_DA_ADDRESS_2 .env | cut -d '"' -f 2)

# Coprocessor 3
COPROCESSOR_ADDRESS_3=$(grep COPROCESSOR_ADDRESS_3 .env | cut -d '"' -f 2)
COPROCESSOR_PUBLIC_KEY_3=$(grep COPROCESSOR_PUBLIC_KEY_3 .env | cut -d '"' -f 2)
COPROCESSOR_DA_ADDRESS_3=$(grep COPROCESSOR_DA_ADDRESS_3 .env | cut -d '"' -f 2)

# Network
NETWORK_CHAIN_ID_1=$(grep NETWORK_CHAIN_ID_1 .env | cut -d '"' -f 2)
NETWORK_HTTPZ_EXECUTOR_1=$(grep NETWORK_HTTPZ_EXECUTOR_1 .env | cut -d '"' -f 2)
NETWORK_ACL_ADDRESS_1=$(grep NETWORK_ACL_ADDRESS_1 .env | cut -d '"' -f 2)
NETWORK_NAME_1=$(grep NETWORK_NAME_1 .env | cut -d '"' -f 2)
NETWORK_WEBSITE_1=$(grep NETWORK_WEBSITE_1 .env | cut -d '"' -f 2)

echo "Deploy HTTPZ contract:"
# Deploy HTTPZ contract
pnpm exec hardhat task:deployHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" \
    --admin-private-key "$ADMIN_PRIVATE_KEY_1" \
    --protocol-metadata "{\"website\":\"${PROTOCOL_WEBSITE}\",\"name\":\"${PROTOCOL_NAME}\"}" \
    --admin-addresses '["'$ADMIN_ADDRESS_1'"]' \
    --kms-threshold $KMS_THRESHOLD \
    --kms-nodes "[{\"connectorAddress\":\"${KMS_NODE_ADDRESS_1}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_1}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_1}\",\"daAddress\":\"${KMS_NODE_DA_ADDRESS_1}\", \"tlsCertificate\":\"${KMS_NODE_TLS_CERTIFICATE_1}\"},{\"connectorAddress\":\"${KMS_NODE_ADDRESS_2}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_2}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_2}\",\"daAddress\":\"${KMS_NODE_DA_ADDRESS_2}\", \"tlsCertificate\":\"${KMS_NODE_TLS_CERTIFICATE_2}\"},{\"connectorAddress\":\"${KMS_NODE_ADDRESS_3}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_3}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_3}\",\"daAddress\":\"${KMS_NODE_DA_ADDRESS_3}\", \"tlsCertificate\":\"${KMS_NODE_TLS_CERTIFICATE_3}\"},{\"connectorAddress\":\"${KMS_NODE_ADDRESS_4}\",\"identity\":\"${KMS_NODE_PUBLIC_KEY_4}\",\"ipAddress\":\"${KMS_NODE_IP_ADDRESS_4}\",\"daAddress\":\"${KMS_NODE_DA_ADDRESS_4}\", \"tlsCertificate\":\"${KMS_NODE_TLS_CERTIFICATE_4}\"}]" \
    --coprocessors "[{\"transactionSenderAddress\":\"${COPROCESSOR_ADDRESS_1}\",\"identity\":\"${COPROCESSOR_PUBLIC_KEY_1}\",\"daAddress\":\"${COPROCESSOR_DA_ADDRESS_1}\"},{\"transactionSenderAddress\":\"${COPROCESSOR_ADDRESS_2}\",\"identity\":\"${COPROCESSOR_PUBLIC_KEY_2}\",\"daAddress\":\"${COPROCESSOR_DA_ADDRESS_2}\"},{\"transactionSenderAddress\":\"${COPROCESSOR_ADDRESS_3}\",\"identity\":\"${COPROCESSOR_PUBLIC_KEY_3}\",\"daAddress\":\"${COPROCESSOR_DA_ADDRESS_3}\"}]" \
    --layer1-networks "[{\"chainId\":${NETWORK_CHAIN_ID_1},\"httpzExecutor\":\"${NETWORK_HTTPZ_EXECUTOR_1}\",\"aclAddress\":\"${NETWORK_ACL_ADDRESS_1}\",\"name\":\"${NETWORK_NAME_1}\",\"website\":\"${NETWORK_WEBSITE_1}\"}]" \
    --network $NETWORK

# Important: the contract deployment order is currently important as some contracts depend on others

echo "Deploy ZKPoKManager contract:"
# Deploy ZKPoKManager contract
pnpm exec hardhat task:deployZkpokManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy KeyManager contract:"
# Deploy KeyManager contract
pnpm exec hardhat task:deployKeyManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy CiphertextStorage contract:"
# Deploy CiphertextStorage contract
pnpm exec hardhat task:deployCiphertextStorage --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy ACLManager contract:"
# Deploy ACLManager contract
pnpm exec hardhat task:deployAclManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK

echo "Deploy DecryptionManager contract:"
# Deploy DecryptionManager contract
pnpm exec hardhat task:deployDecryptionManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network $NETWORK