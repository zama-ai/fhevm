#!/usr/bin/env bash

make run-kms
sleep 4
make init-db
# Deploy ACL, Gateway ..., please wait until the end before testing!!!
make prepare-e2e-test
# This test will fail (first event catch is buggy - we are on it)
make run-async-test
