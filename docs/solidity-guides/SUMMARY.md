# Table of contents

- [Overview](README.md)

## Getting Started

- [What is FHEVM Solidity](getting-started/overview.md)
- [Set up Hardhat](getting-started/quick-start-tutorial/setup.md)
- [Set up Foundry](foundry/setup.md)
- [Quick start tutorial (Hardhat)](getting-started/quick-start-tutorial/README.md)
  - [1. Set up Hardhat](getting-started/quick-start-tutorial/setup.md)
  - [2. Write a simple contract](getting-started/quick-start-tutorial/write_a_simple_contract.md)
  - [3. Turn it into FHEVM](getting-started/quick-start-tutorial/turn_it_into_fhevm.md)
  - [4. Test the FHEVM contract](getting-started/quick-start-tutorial/test_the_fhevm_contract.md)

## Smart Contract

- [Configuration](configure.md)
  - [Contract addresses](contract_addresses.md)
- [Supported types](types.md)
- [Handles](handles.md)
- [Operations on encrypted types](operations/README.md)
  - [Casting and trivial encryption](operations/casting.md)
  - [Generate random numbers](operations/random.md)
- [Encrypted inputs](inputs.md)
- [Access Control List](acl/README.md)
  - [ACL examples](acl/acl_examples.md)
  - [User decryption delegation](acl/delegation.md)
  - [Reorgs handling](acl/reorgs_handling.md)
- [Logics](logics/README.md)
  - [Branching](logics/conditions.md)
  - [Dealing with branches and conditions](logics/loop.md)
  - [Error handling](logics/error_handling.md)
- [Public Decryption](decryption/oracle.md)
- [FHEVM API reference](functions.md)

## Development Guide

- [Hardhat plugin](hardhat/README.md)
  - [Setup Hardhat](getting-started/quick-start-tutorial/setup.md)
  - [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template)
  - [Write FHEVM tests in Hardhat](hardhat/write_test.md)
  - [Deploy contracts and run tests](hardhat/run_test.md)
  - [Write FHEVM-enabled Hardhat Tasks](hardhat/write_task.md)
- [Foundry](foundry/README.md)
  - [Setup Foundry](foundry/setup.md)
  - [Foundry template](https://github.com/zama-ai/fhevm-foundry-template)
  - [Write FHEVM tests in Foundry](foundry/write_test.md)
  - [Deploy contracts](foundry/deploy.md)
  - [forge-fhevm API reference](foundry/api.md)
- [HCU](hcu.md)
- [How to Transform Your Smart Contract into a FHEVM Smart Contract?](transform_smart_contract_with_fhevm.md)