#!/bin/bash

# This script exports an Ethereum private key from an evmos node and uses it to run a Python script.
# The script takes two arguments:
#   1. The name of the key to export (e.g., mykey1 or mykey2)
#   2. The path to the evmos directory
#   3. (Optional) The node address (default: http://host.docker.internal:8545)
#
# Example usage: ./run_ERC20_ci_test.sh.sh mykey1 ../evmos

if [ "$#" -lt 2 ]; then
    echo "Please give the key name (e.g., mykey1 or mykey2) and the path to the evmos directory and optionnaly the node @:port"
    echo "Example: `basename "$0"` mykey1 ../evmos http://host.docker.internal:8545"
    exit
fi

key=$1
PATH_TO_EVMOS=$2
NODE_ADDRESS=${3:-http://host.docker.internal:8545}

# Create the keys directory if it doesn't exist
mkdir -p keys

# Export the private key from the evmos node
PRIVATE_KEY=$(docker compose -f $PATH_TO_EVMOS/docker-compose/docker-compose.validator.yml exec validator evmosd --home /root/.evmosd keys unsafe-export-eth-key $key --keyring-backend test)

echo "Exported private key: $PRIVATE_KEY"

# Run the Python script with the exported private key as an argument
docker compose -f ci/docker-compose.yml run app python ci/tests/ERC20.py $PRIVATE_KEY $NODE_ADDRESS
