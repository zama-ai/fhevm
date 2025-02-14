#!/bin/bash
npx hardhat clean

npx hardhat compile

DEPLOYER_PRIVATE_KEY=$(grep DEPLOYER_PRIVATE_KEY .env | cut -d '"' -f 2)

# Deploy contracts
npx hardhat task:deployContracts --deployer-private-key "$DEPLOYER_PRIVATE_KEY"