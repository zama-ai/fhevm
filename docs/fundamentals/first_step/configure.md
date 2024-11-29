# Configuring Your Smart Contract for Encrypted Computations

This document explains how to enable encrypted computations in your smart contract by setting up the `fhEVM` environment. Learn how to integrate essential libraries, configure encryption, and add secure computation logic to your contracts.

---

## Core configuration setup

To utilize encrypted computations in Solidity contracts, you must configure the **TFHE library** and **Gateway addresses**. The `fhevm` package simplifies this process with prebuilt configuration contracts, allowing you to focus on developing your contract’s logic without handling the underlying cryptographic setup.

### Key components configured automatically:

1. **TFHE Library**: Sets up encryption parameters and cryptographic keys.
2. **Gateway**: Manages secure cryptographic operations, including reencryption and decryption.
3. **Network-Specific Settings**: Adapts to local testing, testnets (e.g., Sepolia), or mainnet deployment.

By inheriting these configuration contracts, you ensure seamless initialization and functionality across environments.

---

### ZamaFHEVMConfig.sol

This configuration contract initializes the **FHEVM environment** with required encryption parameters.

#### Import based on your environment:

```solidity
// For Mock testnet
import { MockZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";

// For Ethereum Sepolia
import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";

// For Ethereum Mainnet (when ready)
import { EthereumZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
```

#### Purpose:

- Sets encryption parameters such as cryptographic keys and supported ciphertext types.
- Ensures proper initialization of the FHEVM environment.

#### Example: using mock configuration

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { MockZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";

contract MyERC20 is MockZamaFHEVMConfig {
  constructor() {
    // Additional initialization logic if needed
  }
}
```

---

### ZamaGatewayConfig.sol

To perform decryption or reencryption, your contract must interact with the **Gateway**, which acts as a secure bridge between the blockchain, coprocessor, and Key Management System (KMS).

#### Import based on your environment:

```solidity
// For Mock testnet
import { MockZamaGatewayConfig } from "fhevm/config/ZamaGatewayConfig.sol";

// For Ethereum Sepolia
import { SepoliaZamaGatewayConfig } from "fhevm/config/ZamaGatewayConfig.sol";

// For Ethereum Mainnet (when ready)
import { EthereumZamaGatewayConfig } from "fhevm/config/ZamaGatewayConfig.sol";
```

#### Purpose:

- Configures the Gateway for secure cryptographic operations.
- Facilitates reencryption and decryption requests.

#### Example: Configuring the gateway with mock settings

```solidity
import "fhevm/lib/TFHE.sol";
import { MockZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
import { MockZamaGatewayConfig } from "fhevm/config/ZamaGatewayConfig.sol";
import "fhevm/gateway/GatewayCaller.sol";

contract Test is MockZamaFHEVMConfig, MockZamaGatewayConfig, GatewayCaller {
  constructor() {
    // Gateway and FHEVM environment initialized automatically
  }
}
```

---

### Using `isInitialized`

The `isInitialized` utility function checks whether an encrypted variable has been properly initialized, preventing unexpected behavior due to uninitialized values.

#### Function Signature:

```solidity
function isInitialized(T v) internal pure returns (bool)
```

#### Purpose:

- Ensures encrypted variables are initialized before use.
- Prevents potential logic errors in contract execution.

#### Example: Initialization Check for Encrypted Counter

```solidity
require(TFHE.isInitialized(counter), "Counter not initialized!");
```

---

## Summary

By leveraging prebuilt configuration contracts like `ZamaFHEVMConfig.sol` and `ZamaGatewayConfig.sol`, you can efficiently set up your smart contract for encrypted computations. These tools abstract the complexity of cryptographic initialization, allowing you to focus on building secure, confidential smart contracts.
