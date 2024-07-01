#!/bin/bash

make init-ethermint-node
# Run fhEVM + full KMS components 
make run-full
# Deploy ACL, Gateway ..., please wait until the end before testing!!!
make prepare-e2e-test
# This test will fail (first event catch is buggy - we are on it)
make run-async-test
