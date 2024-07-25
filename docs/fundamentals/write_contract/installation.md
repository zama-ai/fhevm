This document guides you to install fhEVM Solidity library in your project and explains the basic workflow of writing confidential smart contract.

## Introduction

fhEVM compiles seamlessly with the traditional Solidity compiler and is generally compatible with traditional Solidity tools. However, it is designed to function exclusively on an fhEVM. Therefore, this library is not intended for deployment on a classic EVM, such as Goerli or Ganache.

## Installation

To get started with fhEVM Solidity library, you need to install it as a dependency in your JavaScript project. You can do this using npm (Node Package Manager) or Yarn.

Open your terminal and navigate to your project's directory, then run one of the following commands:

```bash
# Using npm
npm install fhevm

# Using Yarn
yarn add fhevm

# Using pnpm
pnpm add fhevm
```

This will download and install the fhEVM Solidity Library and its dependencies into your project.

## Typical workflow for writing confidential smart contracts

1. Use our custom [`fhevm-hardhat-template` repository](https://github.com/zama-ai/fhevm-hardhat-template). Hardhat is a popular development environment for Solidity developers and lets you test and deploy your contracts to the fhEVM using TypeScript.

2. Start with an unencrypted version of the contract you want to implement, as you would usually do on a regular EVM chain. It is easier to reason first on cleartext variables before adding confidentiality.

3. When you're ready, add confidentiality by using the `TFHE` Solidity library. Typically, this involves converting some `uintX` types to `euintX`. Follow the detailed advices provided in the [pitfalls to avoid and best practises](../../guides/pitfalls.md) section of the documentation. For inspiration, refer to the examples inside the [`fhevm` repository](https://github.com/zama-ai/fhevm/tree/main/examples). If you're using the Hardhat template, read the advices in the [Hardhat section](hardhat.md).
