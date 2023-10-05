# Getting Started

Welcome to the documentation for fhEVM Solidity Library! This comprehensive guide provides developers with detailed information on the library's functions, parameters, and usage examples. Explore how to leverage TFHE's powerful capabilities for computing over encrypted data within Solidity smart contracts, enabling secure computations and encrypted data manipulation. Unlock a new level of privacy and confidentiality in your blockchain applications with fhEVM Solidity Library.

## Usage

Our library TFHE requires Solidity version **0.8.19** specifically, as we rely on features exclusive to this version and do not currently provide support for versions beyond it.

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
