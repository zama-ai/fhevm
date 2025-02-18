#!/bin/bash
pnpm exec hardhat clean

pnpm exec hardhat compile

DEPLOYER_PRIVATE_KEY=$(grep DEPLOYER_PRIVATE_KEY .env | cut -d '"' -f 2)

# Deploy HTTPZ contract
pnpm exec hardhat task:deployHttpz --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network localGatewayL2

# Deploy ZKPoKManager contract
pnpm exec hardhat task:deployZkpokManager --deployer-private-key "$DEPLOYER_PRIVATE_KEY" --network localGatewayL2
