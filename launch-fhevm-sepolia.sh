#!/bin/bash
# This script should be launched after precomputing the addresses via `precompute-addresses.sh`, 
# and preferably after setting up the different services - KMS, Geth node, Gateway

./launch-fhevm.sh sepolia

echo "Waiting 2 minutes before contract verification... Please wait..."
sleep 120 # makes sure that contracts bytescode propagates on Etherscan, otherwise contracts verification might fail in next step
npx hardhat task:verifyContracts --network sepolia