# Oracle

This section explains how to handle decryption in fhevm. Decryption allows plaintext data to be accessed when required for contract logic or user presentation, ensuring confidentiality is maintained throughout the process.

Decryption is essential in two primary cases:

1. **Smart contract logic**: A contract requires plaintext values for computations or decision-making.
2. **User interaction**: Plaintext data needs to be revealed to all users, such as revealing the decision of the vote.

## Overview

Decryption in FHEVM is an asynchronous process that involves the Relayer and Key Management System (KMS). Here’s an example of how to safely request decryption in a contract.

### Example: asynchronous decryption in a contract

```solidity
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract TestAsyncDecrypt is SepoliaConfig {
  ebool xBool;
  bool public yBool;
  bool isDecryptionPending;
  uint256 latestRequestId;

  constructor() {
    xBool = FHE.asEbool(true);
    FHE.allowThis(xBool);
  }

  function requestBool() public {
    require(!isDecryptionPending, "Decryption is in progress");
    bytes32[] memory cts = new bytes32[](1);
    cts[0] = FHE.toBytes32(xBool);
    uint256 latestRequestId = FHE.requestDecryption(cts, this.myCustomCallback.selector);

    /// @dev This prevents sending multiple requests before the first callback was sent.
    isDecryptionPending = true;
  }

  function myCustomCallback(uint256 requestId, bytes memory cleartexts, bytes memory decryptionProof) public returns (bool) {
    /// @dev This check is used to verify that the request id is the expected one.
    require(requestId == latestRequestId, "Invalid requestId");
    FHE.checkSignatures(requestId, cleartexts, decryptionProof);

    (bool decryptedInput) = abi.decode(cleartexts, (bool));
    yBool = decryptedInput;
    isDecryptionPending = false;
    return yBool;
  }
}
```

## Decryption in depth

This document provides a detailed guide on implementing decryption in your smart contracts using the `DecryptionOracle` in fhevm. It covers the setup, usage of the `FHE.requestDecryption` function, and testing with Hardhat.

## `DecryptionOracle` setup

The `DecryptionOracle` is pre-deployed on the FHEVM testnet. It uses a default relayer account specified in the `.env` file.

Anyone can fulfill decryption requests but it is essential to add signature verification (and to include a logic to invalidate the replay of decryption requests). The role of the `DecryptionOracle` contract is to independently verify the KMS signature during execution. This ensures that the relayers cannot manipulate or send fraudulent decryption results, even if compromised.

There are two functions to consider: `requestDecryption` and `checkSignatures`.

### `FHE.requestDecryption` function

You can call the function `FHE.requestDecryption` as such:

```solidity
function requestDecryption(
  bytes32[] calldata ctsHandles,
  bytes4 callbackSelector
) external payable returns (uint256 requestId);
```

#### Function arguments

The first argument, `ctsHandles`, should be an array of ciphertexts handles which could be of different types, i.e `uint256` values coming from unwrapping handles of type either `ebool`, `euint8`, `euint16`, `euint32`, `euint64` or `eaddress`.&#x20;

`ctsHandles` is the array of ciphertexts that are requested to be decrypted. The relayer will send the corresponding ciphertexts to the KMS for decryption before fulfilling the request.

`callbackSelector` is the function selector of the callback function, which will be called once the relayer fulfils the decryption request.

```solidity
function [callbackName](uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) external;
```

`cleartexts` is the bytes array corresponding to the ABI encoding of all requested decrypted values. Each of these decrypted values' type should be a native Solidity type corresponding to the original ciphertext type, following this table of conventions:

| Ciphertext type | Decrypted type |
| --------------- | -------------- |
| ebool           | bool           |
| euint8          | uint8          |
| euint16         | uint16         |
| euint32         | uint32         |
| euint64         | uint64         |
| euint128        | uint128        |
| euint256        | uint256        |
| eaddress        | address        |

Here `callbackName` is a custom name given by the developer to the callback function, `requestID` will be the request id of the decryption (could be commented if not needed in the logic, but must be present) and `cleartexts` is an ABI encoded byte array of the results of the decryption of the `ct` array values, i.e their number should be the size of the `ct` array. `decryptionProof` is a byte array containing the KMS signatures and extra data.

`msgValue` is the value in native tokens to be sent to the calling contract during fulfillment, i.e when the callback will be called with the results of decryption.

{% hint style="warning" %}
Notice that the callback should always verify the signatures and implement a replay protection mechanism (see below).
{% endhint %}

### `FHE.checkSignatures` function

You can call the function `FHE.checkSignatures` as such:

```solidity
function checkSignatures(uint256 requestId, bytes memory cleartexts, bytes[] memory signatures);
```

#### Function arguments

- `requestID`, is the value that was returned in the `requestDecryption` function.
- `cleartexts`, is an ABI encoding of the decrypted values associated to the handles (using `abi.encode`). This can contain one or multiple values, depending on the number of handles requested in the `requestDecryption` function. Each of these values' type must match the type of the corresponding handle.
- `decryptionProof`, is a byte array containing the KMS signatures and extra data.

This function reverts if the signatures are invalid.
