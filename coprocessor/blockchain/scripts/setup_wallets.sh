#!/bin/sh

# TODO: better handle paths, maybe add an env var to specify the smart contracts folder

set -e

#############################
#         Wallets           #
#############################

# TODO: Create multiple accounts for the different connectors

echo ""
echo "+++++++++++++++++++++++"
echo "Starting wallets setups"
echo "+++++++++++++++++++++++"
echo ""

ulimit unlimited

export KEYRING_PASSWORD="1234567890"
export VALIDATOR_NODE_ENDPOINT="${VALIDATOR_NODE_ENDPOINT:-tcp://localhost:26657}"
export NODE="$VALIDATOR_NODE_ENDPOINT"
export WASMD_NODE="$VALIDATOR_NODE_ENDPOINT"

tail -n 1 /app/secrets/validator_stderr.log > /app/secrets/validator.mnemonic

echo "Using wasmd node: ${WASMD_NODE}"

# Add Connector account
export PUB_KEY_KMS_CONN='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A/ZoCPf+L7Uxf3snWT+RU5+ivCmT8XR+NFpuhjm5cTP2"}'
export PUB_KEY_KMS_GATEWAY='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AqAodaWg+3JUxIz6CeH0hKN8rxUzuBgQ67SR0KemoDnp"}'

# Add accounts
# When addings wallets to keyring we are prompted twice for the keyring password.
# In other settings we are usually only prompted once for it.
echo "Adding validator wallet keys"
(cat /app/secrets/validator.mnemonic; echo $KEYRING_PASSWORD; echo $KEYRING_PASSWORD) | wasmd keys add validator --recover
sleep 1
echo "Adding connector wallet keys"
(echo $KEYRING_PASSWORD; echo $KEYRING_PASSWORD) | wasmd keys add connector --pubkey "$PUB_KEY_KMS_CONN"
sleep 1
echo "Adding gateway wallet keys"
(echo $KEYRING_PASSWORD; echo $KEYRING_PASSWORD) | wasmd keys add gateway --pubkey "$PUB_KEY_KMS_GATEWAY"
sleep 1

# Get addresses
CONN_ADDRESS=$(echo $KEYRING_PASSWORD | wasmd keys show connector --output json |jq -r '.address')
GATEWAY_ADDRESS=$(echo $KEYRING_PASSWORD | wasmd keys show gateway --output json |jq -r '.address')
VALIDATOR_ADDRESS=$(echo $KEYRING_PASSWORD | wasmd keys show validator --output json |jq -r '.address')

# TODO: Have one account per connector instead of a shared one
# TODO: Add to the faucet account too
# TODO: Ideally we would fund these accounts through the faucet and not with a multi-send from the validator

# Send tokens to connector and gateway accounts
echo "Sending tokens from validator to connector and gateway accounts"
# The validator has 1000000000ucosm (setup_NODE="$NODE" wasmd.sh)
NODE="$NODE" echo $KEYRING_PASSWORD | wasmd tx bank multi-send "$VALIDATOR_ADDRESS" "$CONN_ADDRESS" "$GATEWAY_ADDRESS" "450000000ucosm" -y --chain-id testing 

echo ""
echo "+++++++++++++++++++++++"
echo "Wallet setup successful"
echo "+++++++++++++++++++++++"
echo ""
