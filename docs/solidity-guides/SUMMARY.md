# Table of contents

- [Overview](README.md)

## Getting Started

- [What is FHEVM Solidity](getting-started/overview.md)
- [Set up Hardhat](getting-started/quick-start-tutorial/setup.md)
- [Quick Start Tutorial](getting-started/quick-start-tutorial/README.md)
  - [1. Set up Hardhat](getting-started/quick-start-tutorial/setup.md)
  - [2. Write a simple contract](getting-started/quick-start-tutorial/write_a_simple_contract.md)
  - [3. Turn it into FHEVM](getting-started/quick-start-tutorial/turn_it_into_fhevm.md)
  - [4. Test the FHEVM contract](getting-started/quick-start-tutorial/test_the_fhevm_contract.md)

## Smart Contract

- [Configuration](configure.md)
  - [Contract addresses](contract_addresses.md)
- [Supported types](types.md)
- [Operations on encrypted types](operations/README.md)
  - [AsEbool, asEuintXX, and asEaddress operations](operations/asEXXoperators.md)
  - [Generate random numbers](operations/random.md)
- [Encrypted Inputs](inputs.md)
- [Access Control List](acl/README.md)
  - [ACL examples](acl/acl_examples.md)
  - [Reorgs handling](acl/reorgs_handling.md)
- [Logics](<README (1).md>)
  - [Branching](logics/conditions.md)
  - [Dealing with branches and conditions](logics/loop.md)
  - [Error handling](logics/error_handling.md)
- [Decryption](decryption/oracle.md)

## Development Guide

- [Hardhat Plugin](hardhat/README.md)
  - [Setup Hardhat](getting-started/quick-start-tutorial/setup.md)
  - [Write FHEVM Tests in Hardhat](hardhat/write_test.md)
  - [Run FHEVM Tests in Hardhat](hardhat/run_test.md)
- [Foundry](foundry.md)
- [HCU](hcu.md)
- [Migrate to v0.7](migration.md)
