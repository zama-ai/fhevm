# Configuration

This document explains how to enable encrypted computations in your smart contract by setting up the `fhevm` environment. Learn how to integrate essential libraries, configure encryption, and add secure computation logic to your contracts.

## Core configuration setup

To utilize encrypted computations in Solidity contracts, you must configure the **FHE library** and **Oracle addresses**. The `fhevm` package simplifies this process with prebuilt configuration contracts, allowing you to focus on developing your contract’s logic without handling the underlying cryptographic setup.

This library and its associated contracts provide a standardized way to configure and interact with Zama's FHEVM (Fully Homomorphic Encryption Virtual Machine) infrastructure on different Ethereum networks. It supplies the necessary contract addresses for Zama's FHEVM components (`ACL`, `FHEVMExecutor`, `KMSVerifier`, `InputVerifier`) and the decryption oracle, enabling seamless integration for Solidity contracts that require FHEVM support.

## Key components configured automatically

1. **FHE library**: Sets up encryption parameters and cryptographic keys.
2. **Oracle**: Manages secure cryptographic operations such as public decryption.
3. **Network-specific settings**: Adapts to local testing, testnets (Sepolia for example), or mainnet deployment.

By inheriting these configuration contracts, you ensure seamless initialization and functionality across environments.

## ZamaConfig.sol

The `ZamaConfig` library exposes functions to retrieve FHEVM configuration structs and oracle addresses for supported networks (currently only the Sepolia testnet).

Under the hood, this library encapsulates the network-specific addresses of Zama's FHEVM infrastructure into a single struct (`FHEVMConfigStruct`). 

## SepoliaConfig

The `SepoliaConfig` contract is designed to be inherited by a user contract. The constructor automatically sets up the FHEVM coprocessor and decryption oracle using the configuration provided by the library for the respective network. When a contract inherits from `SepoliaConfig`, the constructor calls `FHE.setCoprocessor` and `FHE.setDecryptionOracle` with the appropriate addresses. This ensures that the inheriting contract is automatically wired to the correct FHEVM contracts and oracle for the target network, abstracting away manual address management and reducing the risk of misconfiguration.

**Example: using Sepolia configuration**

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract MyERC20 is SepoliaConfig {
  constructor() {
    // Additional initialization logic if needed
  }
}
```


## Using `isInitialized`

The `isInitialized` utility function checks whether an encrypted variable has been properly initialized, preventing unexpected behavior due to uninitialized values.

**Function signature**

```solidity
function isInitialized(T v) internal pure returns (bool)
```

**Purpose**

- Ensures encrypted variables are initialized before use.
- Prevents potential logic errors in contract execution.

**Example: Initialization Check for Encrypted Counter**

```solidity
require(FHE.isInitialized(counter), "Counter not initialized!");
```

## Summary

By leveraging prebuilt a configuration contract like `SepoliaConfig` in `ZamaConfig.sol`, you can efficiently set up your smart contract for encrypted computations. These tools abstract the complexity of cryptographic initialization, allowing you to focus on building secure, confidential smart contracts.
