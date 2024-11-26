#!/bin/bash
# This script should be launched after precomputing the addresses via `precompute-addresses.sh`, and preferably after setting up the different services - KMS, Geth node, Gateway
npm i
npx hardhat clean


rm -rf fhevmTemp/
mkdir -p fhevmTemp
cp -L -r node_modules/fhevm-core-contracts/ fhevmTemp/
npx hardhat compile:specific --contract fhevmTemp
npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

