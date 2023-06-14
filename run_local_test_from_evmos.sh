#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Please give the key name mykey1 or mykey2"
    exit
fi

key=$1
mkdir -p res/ct
mkdir -p keys

PRIVATE_KEY=$(docker compose -f ../../docker-compose/docker-compose.local.yml exec evmosnodelocal evmosd --home /root/.evmosd keys unsafe-export-eth-key $key --keyring-backend test)
echo "Get address from private key: $PRIVATE_KEY"
docker compose -f docker-compose.yml run app python demo_test_high_level_fhe_tool.py $PRIVATE_KEY
