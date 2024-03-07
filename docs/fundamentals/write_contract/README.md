# Write contract

## Usage

Our library compiles seamlessly with the traditional Solidity compiler and is generally compatible with traditional Solidity tools. However, it's important to note that the execution is designed to function exclusively on a fhEVM. As a result, this library is not intended for deployment on a classic EVM, such as Goerli or Ganache.

## Installation

To get started with fhEVM Solidity Library, you need to install it as a dependency in your JavaScript project. You can do this using npm (Node Package Manager) or Yarn. Open your terminal and navigate to your project's directory, then run one of the following commands:

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

1/ For quick prototyping of a specific feature, use the [Zama version of the Remix IDE](remix.md). This will let you quickly deploy a contract on the devnet via Metamask, and interact easily with it through the Remix UI. Otherwise, for a bigger project, you should use our custom [`fhevm-hardhat-template` repository](https://github.com/zama-ai/fhevm-hardhat-template). Hardhat is a popular development environment for Solidity developers and will let you test and deploy your contracts to the fhEVM using TypeScript.

2/ A good first step is to start with an unencrypted version of the contract you want to implement, as you would usually do on a regular EVM chain. It is easier to reason first on cleartext variables, before thinking on how to add confidentiality.

3/ When you're ready, you can start to add confidentiality by using the `TFHE` solidity library. Typically, this would involve converting some `uintX` types to `euintX`, as well as following all the detailed advices that we gave in the [pitfalls to avoid and best practises](../../guides/pitfalls.md) section of the documentation. For inspiration, you can take a look at the examples inside the [`fhevm` repository](https://github.com/zama-ai/fhevm/tree/main/examples). If you're using the Hardhat template, read the advices that we gave in the [Hardhat section](hardhat.md).
