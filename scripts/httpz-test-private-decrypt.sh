#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"


cd $SCRIPTPATH/.. && docker compose -f ./docker-compose.05.test.yaml up -d --wait && docker exec console-tests-e2e-debug ./run-tests.sh test reencrypt ebool
