#!/usr/bin/env bash

make run-kms
sleep 4
make init-db
# Deploy ACL, Gateway ..., please wait until the end before testing!!!
make prepare-e2e-test
# Run one test
make run-async-test
