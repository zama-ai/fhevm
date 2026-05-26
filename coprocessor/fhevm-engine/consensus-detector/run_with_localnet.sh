#!/bin/bash
source ./../.env-test

echo "DATABASE_URL=$DATABASE_URL"
echo "GATEWAY_WS_URL=$GATEWAY_WS_URL"
echo "GATEWAY_CONFIG_ADDRESS=$GATEWAY_CONFIG_ADDRESS"

cargo run --release -- \
--database-url=${DATABASE_URL} \
--database-pool-size=4 \
--gw-url=${GATEWAY_WS_URL} \
--gateway-config-address=${GATEWAY_CONFIG_ADDRESS} \
--commitment-poll-interval=5s \
--commitment-timeout=60s \
--poll-interval-secs=30
