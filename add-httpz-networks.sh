#!/bin/bash

npx hardhat clean

npx hardhat compile

# Add L1 networks to HTTPZ contract
echo "Register networks to HTTPZ contract"
npx hardhat task:addNetworksToHttpz

echo "Networks registration done !"