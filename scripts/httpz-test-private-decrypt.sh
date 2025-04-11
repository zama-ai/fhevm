#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

# TODO: implement
# cd $SCRIPTPATH/../apps/relayer/fhevm-relayer && RELAYER_URL=http://127.0.0.1:3005 make run-test-private-decrypt-hardhat
# cd $SCRIPTPATH/../apps/relayer/fhevm-relayer && RELAYER_URL=http://127.0.0.1:3000 make run-test-private-decrypt-hardhat
cd $SCRIPTPATH/../apps/relayer/fhevm-relayer && RELAYER_URL=http://127.0.0.1:4324 make run-test-private-decrypt-hardhat
