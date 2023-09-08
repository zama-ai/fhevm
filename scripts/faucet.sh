#!/bin/bash

# Alice
npm run fhevm:faucet
sleep 8 
npm run fhevm:faucet:bob
sleep 8
npm run fhevm:faucet:carol


