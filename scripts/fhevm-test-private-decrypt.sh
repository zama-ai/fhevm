#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"


cd $SCRIPTPATH/.. && docker compose -f ./docker-compose.05.test.yaml -p console up -d --wait && docker exec console-tests-e2e-debug /bin/bash -c 'RPC_URL=http://fhevm-host-node:8545 ./run-tests.sh "test reencrypt ebool" "staging"'
