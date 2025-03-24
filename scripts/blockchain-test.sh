#!/bin/bash

# Run smart contracts
cd external/fhevm-devops/coprocessor/work_dir/fhevm 
npm install
mkdir -p fhevmTemp && cp -L -r node_modules/fhevm-core-contracts/. fhevmTemp/
SEPOLIA_RPC_URL="" npx hardhat compile:specific --contract fhevmTemp
SEPOLIA_RPC_URL="" npx hardhat test --grep 'should transfer' --network localCoprocessor
