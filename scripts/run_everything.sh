#!/usr/bin/env bash

set -e

# Trap any error and print a custom message
trap 'echo "Error: A command failed. Exiting."' ERR

make run-kms
sleep 4
make init-db
# Deploy ACL, Gateway ..., please wait until the end before testing!!!
make prepare-e2e-test
# Run one test
make run-async-test

