#!/bin/bash

npx hardhat clean

npx hardhat compile

npx hardhat task:deployEmptyUUPSProxies

echo "Deploy HTTPZ contract:"
# Deploy HTTPZ contract
npx hardhat task:deployHttpz

echo "Deploy ZKPoKManager contract:"
# Deploy ZKPoKManager contract
npx hardhat task:deployZkpokManager

echo "Deploy KeyManager contract:"
# Deploy KeyManager contract
npx hardhat task:deployKeyManager

echo "Deploy CiphertextManager contract:"
# Deploy CiphertextManager contract
npx hardhat task:deployCiphertextManager

echo "Deploy ACLManager contract:"
# Deploy ACLManager contract
npx hardhat task:deployAclManager

echo "Deploy DecryptionManager contract:"
# Deploy DecryptionManager contract
npx hardhat task:deployDecryptionManager
