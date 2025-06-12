#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

cd $SCRIPTPATH/../external/fhevm/test-suite/fhevm && bash ./fhevm-cli deploy
