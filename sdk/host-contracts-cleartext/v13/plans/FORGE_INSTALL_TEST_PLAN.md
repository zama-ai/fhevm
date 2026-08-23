# Plan — test forge install

## Goal

Must test `forge install`

## The test

1. one dApp foundry project named dummy-dapp
2. one First Library foundry named forge-fhevm-dummy
3. one Second Library : the host-contracts-cleartext repo

`dummy-dapp` has `forge-fhevm-dummy` in its dependencies
`forge-fhevm-dummy` has `host-contracts-cleartext` in its dependencies

let's pretend that `forge-fhevm-dummy` also has OZ contracts are deps
let's pretend that `dummy-dapp` also has OZ contracts are deps

# How to run the test

- i want a standalone folder.
  with a install.sh script that will construct the 3 dummy repos autonomously to simulate the future forge install situation

- the test should be fully autonomous
- delete the generated folders when finished
- ideally we should be able to put it in a CI env

# Use of GitHub

- to perform a real test, we need a test github repo
- the test installer should use `gh` with the necessary permissions to run the test
- `forge-fhevm-dummy` should be renamed as `libA`
- `host-contracts-cleartext` should be renamed as `libB`
