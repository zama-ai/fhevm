#!/bin/bash

npx hardhat clean
npx hardhat compile:specific --contract contracts/emptyProxy

mkdir -p addresses

npx hardhat task:deployEmptyUUPSProxies --use-coprocessor-address true

npx hardhat compile
npx hardhat compile:specific --contract decryptionOracle

npx hardhat task:deployACL
npx hardhat task:deployTFHEExecutor
npx hardhat task:deployKMSVerifier --use-address true
npx hardhat task:deployInputVerifier --use-address true
npx hardhat task:deployFHEGasLimit
npx hardhat task:deployDecryptionOracle