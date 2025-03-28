#!/bin/bash

npx hardhat clean

npx hardhat compile:specific --contract "contracts/emptyProxy"

npx hardhat task:deployEmptyUUPSProxies

# The deployEmptyUUPSProxies task may have updated the contracts' addresses in `addresses/*.sol`.
# Thus, we must re-compile the contracts with these new addresses, otherwise the old ones will be
# used.
npx hardhat compile:specific --contract "contracts"

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
