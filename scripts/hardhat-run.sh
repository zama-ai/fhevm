#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

cd $SCRIPTPATH/../external/fhevm-relayer/hardhat/contracts && npm i &&cp .env.example .env && npx hardhat node --hostname 127.0.0.1 --port 8746
