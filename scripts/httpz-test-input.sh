#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"


# TODO: validate response
# curl -X POST http://127.0.0.1:4324/input-proof \
# curl -X POST http://127.0.0.1:3000/input-proof \
#   -H "Content-Type: application/json" \
#   -d '{"contractChainId": "12345", "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1", "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80", "ciphertextWithZkpok": "abcdef"}'


cd $SCRIPTPATH/../apps/relayer/fhevm-relayer && RELAYER_URL=http://localhost:4324 make run-test-input-proof-hardhat
# cd $SCRIPTPATH/../apps/relayer/fhevm-relayer && RELAYER_URL=http://127.0.0.1:3005 make run-test-input-proof-hardhat
# cd $SCRIPTPATH/../apps/relayer/fhevm-relayer && RELAYER_URL=http://127.0.0.1:3000 make run-test-input-proof-hardhat
# private-decrypt
# TODO
# ERC-20
# TODO
# public-decrypt
# TODO
