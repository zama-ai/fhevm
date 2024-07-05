# Contracts standard library

## Getting Started

### Installation

```bash
# Using npm
npm install fhevm-contracts

# Using Yarn
yarn add fhevm-contracts

# Using pnpm
pnpm add fhevm-contracts
```

## A Simple Example

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import "fhevm-contracts/contracts/token/ERC20/EncryptedERC20.sol";

contract MyERC20 is EncryptedERC20 {
  constructor() EncryptedERC20("MyToken", "MYTOKEN") {
    _mint(1000000, msg.sender);
  }
}
```

## Available contracts

- [EncryptedERC20](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/EncryptedERC20.sol)
- [DAO](https://github.com/zama-ai/fhevm-contracts/tree/main/contracts/DAO)
- [EncryptedErrors](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/utils/EncryptedErrors.sol)
