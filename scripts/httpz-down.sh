#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

# TODO: move to use httpz-test-suite
cd $SCRIPTPATH/../external/httpz-test-suite/httpz && bash ./httpz-cli clean
