#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

# TODO: add verification that hardhat node is running
cd $SCRIPTPATH/../apps/relayer/fhevm-relayer && make run-test-decrypt
