# fhevm-contracts

This guide explains how to use the [fhEVM Contracts standard library](https://github.com/zama-ai/fhevm-contracts/tree/main). This library provides secure, extensible, and pre-tested Solidity templates designed for developing smart contracts on fhEVM using the TFHE library.

## Overview

The **fhEVM Contracts standard library** streamlines the development of confidential smart contracts by providing templates and utilities for tokens, governance, and error management. These contracts have been rigorously tested by ZAMA's engineers and are designed to accelerate development while enhancing security.

## Installation

Install the library using your preferred package manager:

```bash
# Using npm
npm install fhevm-contracts

# Using Yarn
yarn add fhevm-contracts

# Using pnpm
pnpm add fhevm-contracts
```

## Example

### Local testing with the mock network

When testing your contracts locally, you can use the `MockZamaFHEVMConfig` which provides a mock configuration for local development and testing. This allows you to test your contracts without needing to connect to a real network:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { MockZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
import { EncryptedERC20 } from "fhevm-contracts/contracts/token/ERC20/EncryptedERC20.sol";

contract MyERC20 is MockZamaFHEVMConfig, EncryptedERC20 {
  constructor() EncryptedERC20("MyToken", "MYTOKEN") {
    _unsafeMint(1000000, msg.sender);
  }
}
```

### Deploying to Ethereum Sepolia

When deploying to Sepolia, you can use the `SepoliaZamaFHEVMConfig` which provides the correct configuration for the Sepolia testnet:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
import { EncryptedERC20 } from "fhevm-contracts/contracts/token/ERC20/EncryptedERC20.sol";

contract MyERC20 is SepoliaZamaFHEVMConfig, EncryptedERC20 {
  constructor() EncryptedERC20("MyToken", "MYTOKEN") {
    _unsafeMint(1000000, msg.sender);
  }
}
```

## Best practices for contract inheritance

When inheriting from configuration contracts, the order of inheritance is critical. Since constructors are evaluated from left to right in Solidity, you must inherit the configuration contract first to ensure proper initialization.

✅ **Correct Order**:

```
contract MyERC20 is SepoliaZamaFHEVMConfig, EncryptedERC20 { ... }
```

❌ **Wrong order**:

```
contract MyERC20 is EncryptedERC20, SepoliaZamaFHEVMConfig { ... }
```

## Available contracts

For a list of all available contracts see the page [See all tutorials](../tutorials/see-all-tutorials.md)
