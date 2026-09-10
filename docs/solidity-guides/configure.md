# Configuration

This document explains how to enable encrypted computations in your smart contract by setting up the `fhevm` environment. Learn how to integrate essential libraries, configure encryption, and add secure computation logic to your contracts.

## Core configuration setup

To utilize encrypted computations in Solidity contracts, you must configure the **FHE library**. The `fhevm` package simplifies this process with prebuilt configuration contracts, allowing you to focus on developing your contract's logic without handling the underlying cryptographic setup.

This library and its associated contracts provide a standardized way to configure and interact with Zama's FHEVM (Fully Homomorphic Encryption Virtual Machine) infrastructure on different Ethereum networks. It supplies the necessary contract addresses for Zama's FHEVM components (`ACL`, `FHEVMExecutor`, `KMSVerifier`), enabling seamless integration for Solidity contracts that require FHEVM support. The `InputVerifier` is not part of the inherited config — it is resolved at runtime via `FHEVMExecutor.getInputVerifierAddress()`.

## Key components configured automatically

1. **FHE library**: Sets up encryption parameters and cryptographic keys.
2. **Network-specific settings**: Adapts to local testing, testnets (Sepolia, Polygon Amoy) or mainnet deployments (Ethereum, Polygon).

By inheriting these configuration contracts, you ensure seamless initialization and functionality across environments.

## ZamaConfig.sol

The `ZamaConfig` library exposes functions to retrieve FHEVM configuration structs and contract addresses for every network where the Zama protocol is deployed:

| Network          | Chain id   | Environment | Selector                        |
| ---------------- | ---------- | ----------- | ------------------------------- |
| Ethereum mainnet | `1`        | mainnet     | `getEthereumCoprocessorConfig`  |
| Polygon          | `137`      | mainnet     | `getPolygonCoprocessorConfig`   |
| Sepolia          | `11155111` | testnet     | `getEthereumCoprocessorConfig`  |
| Polygon Amoy     | `80002`    | testnet     | `getPolygonCoprocessorConfig`   |
| Hardhat / Anvil  | `31337`    | local       | any selector                    |

`getCoprocessorConfig()` picks the right entry from `block.chainid` for all of them and reverts with `ZamaProtocolUnsupported` on any other chain. Under the hood, the library encapsulates the network-specific addresses of Zama's FHEVM infrastructure into a single struct (`CoprocessorConfig`).

The library also exposes `getConfidentialProtocolId()`: `1` on mainnet chains (Ethereum, Polygon), `10001` on testnet chains (Sepolia, Amoy), `type(uint256).max` locally. Ethereum and Polygon are different host chains talking to the **same** gateway and KMS, so a mainnet contract deployed on both chains shares one protocol environment.

## Configuration contracts

Three abstract contracts wrap the library so that a user contract only has to inherit from one of them. Their constructor calls `FHE.setCoprocessor` with the addresses for the chain the contract is being deployed on, which removes manual address management and the risk of misconfiguration. Each also exposes `confidentialProtocolId()`.

| Contract                | Chains accepted at deployment                    | Use it when                                          |
| ----------------------- | ------------------------------------------------ | ---------------------------------------------------- |
| `ZamaEthereumConfig`    | Ethereum mainnet, Sepolia, local                 | The contract only ever lives on Ethereum.            |
| `ZamaPolygonConfig`     | Polygon, Polygon Amoy, local                     | The contract only ever lives on Polygon.             |
| `ZamaMultiChainConfig`  | all of the above                                 | One bytecode deployed on several host chains.        |

Deploying on a chain the selected contract does not accept reverts in the constructor with `ZamaProtocolUnsupported`.

**Example**

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ZamaEthereumConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract MyERC20 is ZamaEthereumConfig {
  constructor() {
    // Additional initialization logic if needed
  }
}
```

The same contract, deployable on Ethereum and Polygon from a single artifact:

```solidity
import { ZamaMultiChainConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract MyERC20 is ZamaMultiChainConfig {
  constructor() {}
}
```

{% hint style="info" %}
Handles are chain-specific. A contract deployed on both chains holds independent encrypted state on each; to move a value across chains, use the [confidential bridge](bridge.md).
{% endhint %}

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

By leveraging a prebuilt configuration contract like `ZamaEthereumConfig`, `ZamaPolygonConfig` or `ZamaMultiChainConfig` from `ZamaConfig.sol`, you can efficiently set up your smart contract for encrypted computations. These tools abstract the complexity of cryptographic initialization, allowing you to focus on building secure, confidential smart contracts.
