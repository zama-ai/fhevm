# Turn it into FHEVM

In this tutorial, you'll learn how to take a basic Solidity smart contract and progressively upgrade it to support Fully Homomorphic Encryption using the FHEVM library by Zama.

Starting with the plain `Counter.sol` contract that you build from the ["Write a simple contract" tutorial](write_a_simple_contract.md), and step-by-step, you’ll learn how to:

- Replace standard types with encrypted equivalents
- Integrate zero-knowledge proof validation
- Enable encrypted on-chain computation
- Grant permissions for secure off-chain decryption

By the end, you'll have a fully functional smart contract that supports FHE computation.

## Initiate the contract

{% stepper %} {% step %}

## Create the `FHECounter.sol` file

Navigate to your project’s `contracts` directory:

```sh
cd <your-project-root-directory>/contracts
```

From there, create a new file named `FHECounter.sol`, and copy the following Solidity code into it:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title A simple counter contract
contract Counter {
  uint32 private _count;

  /// @notice Returns the current count
  function getCount() external view returns (uint32) {
    return _count;
  }

  /// @notice Increments the counter by a specific value
  function increment(uint32 value) external {
    _count += value;
  }

  /// @notice Decrements the counter by a specific value
  function decrement(uint32 value) external {
    require(_count >= value, "Counter: cannot decrement below zero");
    _count -= value;
  }
}
```

This is a plain `Counter` contract that we’ll use as the starting point for adding FHEVM functionality. We will modify this contract step-by-step to progressively integrate FHEVM capabilities. 
{% endstep %}

{% step %}

## Turn `Counter` into `FHECounter`

To begin integrating FHEVM features into your contract, we first need to import the required FHEVM libraries.

#### Replace the current header

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
```

#### With this updated header:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { FHE, euint32, externalEuint32 } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
```

This imports:

- **FHE** — the core library to work with FHEVM encrypted types
- **euint32** and **externalEuint32** — encrypted uint32 types used in FHEVM
- **SepoliaConfig** — provides the FHEVM configuration for the Sepolia network.\
  Inheriting from it enables your contract to use the FHE library

#### Replace the current contract declaration:

```solidity
/// @title A simple counter contract
contract Counter {
```

#### With the updated declaration :

```solidity
/// @title A simple FHE counter contract
contract FHECounter is SepoliaConfig {
```

This change:

- Renames the contract to `FHECounter`
- Inherits from `SepoliaConfig` to enable FHEVM support

{% hint style="warning" %}
This contract must inherit from the `SepoliaConfig` abstract contract; otherwise, it will not be able to execute any FHEVM-related functionality on Sepolia or Hardhat.
{% endhint %}

From your project's root directory, run:

```sh
npx hardhat compile
```

Great! Your smart contract is now compiled and ready to use **FHEVM features.**

{% endstep %} {% endstepper %}

## Apply FHE functions and types

{% stepper %} {% step %}

## Comment out the `increment()` and `decrement()` Functions

Before we move forward, let’s comment out the `increment()` and `decrement()` functions in `FHECounter`. We'll replace them later with updated versions that support FHE-encrypted operations.

```solidity
 /// @notice Increments the counter by a specific value
// function increment(uint32 value) external {
//     _count += value;
// }

/// @notice Decrements the counter by a specific value
// function decrement(uint32 value) external {
//     require(_count >= value, "Counter: cannot decrement below zero");
//     _count -= value;
// }
```

{% endstep %}

{% step %}

## Replace `uint32` with the FHEVM `euint32` Type

We’ll now switch from the standard Solidity `uint32` type to the encrypted FHEVM type `euint32`.

This enables private, homomorphic computation on encrypted integers.

#### Replace

```solidity
uint32 _counter;
```

and

```solidity
function getCount() external view returns (uint32) {
```

#### With :

```solidity
euint32 _counter;
```

and

```solidity
function getCount() external view returns (euint32) {
```

{% endstep %}

{% step %}

## Replace `increment(uint32 value)` with the FHEVM version `increment(externalEuint32 value)`

To support encrypted input, we will update the increment function to accept a value encrypted off-chain.

Instead of using a `uint32`, the new version will accept an `externalEuint32`, which is an encrypted integer produced off-chain and sent to the smart contract.

To ensure the validity of this encrypted value, we also include a second argument:`inputProof`, a bytes array containing a Zero-Knowledge Proof of Knowledge (ZKPoK) that proves two things:

1. The `externalEuint32` was encrypted off-chain by the function caller (`msg.sender`)
2. The `externalEuint32` is bound to the contract (`address(this)`) and can only be processed by it.

#### Replace

```solidity
 /// @notice Increments the counter by a specific value
// function increment(uint32 value) external {
//     _count += value;
// }
```

#### With :

```solidity
/// @notice Increments the counter by a specific value
function increment(externalEuint32 inputEuint32, bytes calldata inputProof) external {
  //     _count += value;
}
```

{% endstep %}

{% step %}

## Convert `externalEuint32` to `euint32`

You cannot directly use `externalEuint32` in FHE operations. To manipulate it with the FHEVM library, you first need to convert it into the native FHE type `euint32`.

This conversion is done using:

```solidity
FHE.fromExternal(inputEuint32, inputProof);
```

This method verifies the zero-knowledge proof and returns a usable encrypted value within the contract.

#### Replace

```solidity
/// @notice Increments the counter by a specific value
function increment(externalEuint32 inputEuint32, bytes calldata inputProof) external {
  //     _count += value;
}
```

#### With :

```solidity
/// @notice Increments the counter by a specific value
function increment(externalEuint32 inputEuint32, bytes calldata inputProof) external {
  euint32 evalue = FHE.fromExternal(inputEuint32, inputProof);
  //     _count += value;
}
```

{% endstep %}

{% step %}

## Convert `_count += value` into its FHEVM equivalent

To perform the update `_count += value` in a Fully Homomorphic way, we use the `FHE.add()` operator. This function allows us to compute the FHE sum of 2 encrypted integers.

#### Replace

```solidity
/// @notice Increments the counter by a specific value
function increment(externalEuint32 inputEuint32, bytes calldata inputProof) external {
  euint32 evalue = FHE.fromExternal(inputEuint32, inputProof);
  //     _count += value;
}
```

#### With :

```solidity
/// @notice Increments the counter by a specific value
function increment(externalEuint32 inputEuint32, bytes calldata inputProof) external {
  euint32 evalue = FHE.fromExternal(inputEuint32, inputProof);
  _count = FHE.add(_count, evalue);
}
```

{% hint style="info" %}
This FHE operation allows the smart contract to process encrypted values without ever decrypting them — a core feature of FHEVM that enables on-chain privacy. 
{% endhint %}

{% endstep %} 
{% endstepper %}

## Grant FHE Permissions

{% hint style="warning" %}
This step is critical! You must grant FHE permissions to both the contract and the caller to ensure the encrypted `_count` value can be decrypted off-chain by the caller. Without these 2 permissions, the caller will not be able to compute the clear result. 
{% endhint %}

To grant FHE permission we will call the `FHE.allow()` function.

#### Replace

```solidity
/// @notice Increments the counter by a specific value
function increment(externalEuint32 inputEuint32, bytes calldata inputProof) external {
  euint32 evalue = FHE.fromExternal(inputEuint32, inputProof);
  _count = FHE.add(_count, evalue);
}
```

#### With :

```solidity
/// @notice Increments the counter by a specific value
function increment(externalEuint32 inputEuint32, bytes calldata inputProof) external {
  euint32 evalue = FHE.fromExternal(inputEuint32, inputProof);
  _count = FHE.add(_count, evalue);

  FHE.allowThis(_count);
  FHE.allow(_count, msg.sender);
}
```

{% hint style="info" %}
We grant **two** FHE permissions here — not just one. In the next part of the tutorial, you'll learn why **both** are necessary.
{% endhint %}

Congratulations! Your smart contract is now fully **FHEVM-compatible**.

Now you should have the following files in your project:

- [`contracts/FHECounter.sol`](https://app.gitbook.com/s/UTmYJ1UQyasGNx2K8Aqd/smart-contract-examples/use-case-examples/fhe-counter#fhecounter.sol) — your Solidity smart FHEVM contract
- [`test/FHECounter.ts`](https://app.gitbook.com/s/UTmYJ1UQyasGNx2K8Aqd/smart-contract-examples/use-case-examples/fhe-counter#fhecounter.ts) — your FHEVM Hardhat test suite written in TypeScript

In the [next tutorial](test_fhevm_contract.md), we’ll move on to the **TypeScript integration**, where you’ll learn how to interact with your newly upgraded FHEVM contract in a test suite.
