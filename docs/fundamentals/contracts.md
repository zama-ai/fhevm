# Contracts standard library

This document provides guidance on how to use the [fhEVM Contracts standard library](https://github.com/zama-ai/fhevm-contracts/tree/main).

fhEVM contracts is a Solidity library for secure smart-contract development using fhEVM and TFHE.

## Installation

```bash
# Using npm
npm install fhevm-contracts

# Using Yarn
yarn add fhevm-contracts

# Using pnpm
pnpm add fhevm-contracts
```

## Example

> To write Solidity contracts that use `TFHE` and/or `Gateway`, it is required to set different contract addresses. This repo (`fhevm`) exports config files that can be inherited to simplify the process.

### Using the mock network (for testing)

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

### Using Sepolia

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

## Available contracts

Template contracts are available [here](https://github.com/zama-ai/fhevm-contracts/tree/main).
Currently, templates include governance-related and token-related contracts.

### Token

- [EncryptedERC20](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/EncryptedERC20.sol)
- [EncryptedERC20Mintable](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/extensions/EncryptedERC20Mintable.sol)
- [EncryptedERC20WithErrors](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/extensions/EncryptedERC20WithErrors.sol)
- [EncryptedERC20WithErrorsMintable](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/extensions/EncryptedERC20WithErrorsMintable.sol)

### Governance

- [Comp](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/governance/Comp.sol)
- [GovernorAlphaZama](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/governance/GovernorAlphaZama.sol)

### Utils

- [EncryptedErrors](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/utils/EncryptedErrors.sol)
