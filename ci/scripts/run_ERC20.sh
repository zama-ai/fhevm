#!/bin/bash

# This script execute a python script within a ready-to-use docker image with all the required python modules. 
# The script takes two arguments:
#   1. The private key of the main account which has already funds.
#   2. (Optional) The node address (default: http://host.docker.internal:8545)
#
# Example usage: ./run_ERC20.sh <private_key> http://host.docker.internal:8545

if [ "$#" -lt 2 ]; then
    echo "Please give the private key of the main account and optionnaly the node @:port"
    echo "Example: `basename "$0"` CB99CAA34343 http://host.docker.internal:8545"
    exit
fi

PRIVATE_KEY=$1
NODE_ADDRESS=${2:-http://host.docker.internal:8545}

# Create the keys directory if it doesn't exist
mkdir -p keys

echo "Exported private key: $PRIVATE_KEY"

# Run the Python script with the exported private key as an argument
docker compose -f ci/docker-compose.yml run app python ci/tests/ERC20.py $PRIVATE_KEY $NODE_ADDRESS
