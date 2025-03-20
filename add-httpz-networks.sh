#!/bin/bash
# Default value
DEFAULT_NETWORK="localHTTPZGateway"

# Get the first argument ($1) if provided, otherwise use DEFAULT_VALUE
# The ${1:-default_value} syntax means "use $1 if it exists, otherwise use default_value"
NETWORK=${1:-$DEFAULT_NETWORK}

npx hardhat clean

npx hardhat compile

# Add L1 networks to HTTPZ contract
npx hardhat task:addNetworksToHttpz --network $NETWORK
