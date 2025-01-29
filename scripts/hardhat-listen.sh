#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

# TODO: add verification that hardhat node is running
cd $SCRIPTPATH/../external/fhevm-relayer/hardhat/contracts && sh launch-fhevm-local-coprocessor.sh && cd ../.. && cargo run
