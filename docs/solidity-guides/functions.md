# Smart contracts - FHEVM API

This document provides an overview of the functions available in the `FHE` Solidity library. The FHE library provides functionality for working with encrypted types and performing operations on them. It implements fully homomorphic encryption (FHE) operations in Solidity.

## Overview

The `FHE` Solidity library provides essential functionality for working with encrypted data types and performing fully homomorphic encryption (FHE) operations in smart contracts. It is designed to streamline the developer experience while maintaining flexibility and performance.

### **Core Functionality**

- **Homomorphic Operations**: Enables arithmetic, bitwise, and comparison operations on encrypted values.
- **Ciphertext-Plaintext Interoperability**: Supports operations that mix encrypted and plaintext operands, provided the plaintext operand's size does not exceed the encrypted operand's size.
  - Example: `add(uint8 a, euint8 b)` is valid, but `add(uint32 a, euint16 b)` is not.
  - Ciphertext-plaintext operations are generally faster and consume less gas than ciphertext-ciphertext operations.
- **Implicit Upcasting**: Automatically adjusts operand types when necessary to ensure compatibility during operations on encrypted data.

### **Key Features**

- **Flexibility**: Handles a wide range of encrypted data types, including booleans, integers, addresses, and byte arrays.
- **Performance Optimization**: Prioritizes efficient computation by supporting optimized operator versions for mixed plaintext and ciphertext inputs.
- **Ease of Use**: Offers consistent APIs across all supported data types, enabling a smooth developer experience.

The library ensures that all operations on encrypted data follow the constraints of FHE while abstracting complexity, allowing developers to focus on building privacy-preserving smart contracts.

## Types

### Encrypted Data Types

#### Boolean

- `ebool`: Encrypted boolean value

#### Unsigned Integers

- `euint8`: Encrypted 8-bit unsigned integer
- `euint16`: Encrypted 16-bit unsigned integer
- `euint32`: Encrypted 32-bit unsigned integer
- `euint64`: Encrypted 64-bit unsigned integer
- `euint128`: Encrypted 128-bit unsigned integer
- `euint256`: Encrypted 256-bit unsigned integer

#### Addresses

- `eaddress`: Encrypted Ethereum address

#### Special Types

- `externalEbool`: Input type for encrypted boolean value
- `externalEuint8`: Input type for encrypted 8-bit unsigned integer value
- `externalEuint16`: Input type for encrypted 16-bit unsigned integer value
- `externalEuint32`: Input type for encrypted 32-bit unsigned integer value
- `externalEuint64`: Input type for encrypted 64-bit unsigned integer value
- `externalEuint128`: Input type for encrypted 128-bit unsigned integer value
- `externalEuint256`: Input type for encrypted 256-bit unsigned integer value
- `externalEaddress`: Input type for encrypted Ethereum address

### Casting Types

- **Casting between encrypted types**: `FHE.asEbool` converts encrypted integers to encrypted booleans
- **Casting to encrypted types**: `FHE.asEuintX` converts plaintext values to encrypted types
- **Casting to encrypted addresses**: `FHE.asEaddress` converts plaintext addresses to encrypted addresses

#### `asEuint`

The `asEuint` functions serve three purposes:

- verify ciphertext bytes and return a valid handle to the calling smart contract;
- cast a `euintX` typed ciphertext to a `euintY` typed ciphertext, where `X != Y`;
- trivially encrypt a plaintext value.

The first case is used to process encrypted inputs, e.g. user-provided ciphertexts. Those are generally included in a transaction payload.

The second case is self-explanatory. When `X > Y`, the most significant bits are dropped. When `X < Y`, the ciphertext is padded to the left with trivial encryptions of `0`.

The third case is used to "encrypt" a public value so that it can be used as a ciphertext. Note that what we call a trivial encryption is **not** secure in any sense. When trivially encrypting a plaintext value, this value is still visible in the ciphertext bytes. More information about trivial encryption can be found [here](https://www.zama.ai/post/tfhe-deep-dive-part-1).

**Examples**

```solidity
// first case
function asEuint8(bytes memory ciphertext) internal view returns (euint8)
// second case
function asEuint16(euint8 ciphertext) internal view returns (euint16)
// third case
function asEuint16(uint16 value) internal view returns (euint16)
```

#### &#x20;`asEbool`

The `asEbool` functions behave similarly to the `asEuint` functions, but for encrypted boolean values.

## Core Functions

### Configuration

```solidity
function setCoprocessor(CoprocessorConfig memory coprocessorConfig) internal
```

Sets the FHEVM coprocessor configuration for encrypted operations. The `CoprocessorConfig` struct contains the addresses of the ACL, Coprocessor (FHEVMExecutor), and KMSVerifier contracts. In most cases, you do not need to call this directly — inherit from `ZamaEthereumConfig` instead, which calls this automatically based on the current chain ID.

### Initialization Checks

```solidity
function isInitialized(T v) internal pure returns (bool)
```

Returns true if the encrypted value is initialized, false otherwise. Supported for all encrypted types (T can be ebool, euintX, eaddress).

### Arithmetic operations

Available for euint\* types:

```solidity
function add(T a, T b) internal returns (T)
function sub(T a, T b) internal returns (T)
function mul(T a, T b) internal returns (T)
```

- Arithmetic: `FHE.add`, `FHE.sub`, `FHE.mul`, `FHE.min`, `FHE.max`, `FHE.neg`, `FHE.div`, `FHE.rem`
  - Note: `div` and `rem` operations are supported only with plaintext divisors

> :warning: Functions with FHE operations cannot be marked as `view` since FHE operations cost gas to execute since they always involve a state-change. For instance, you cannot compute and return the encrypted sum of two encrypted values in a view function.

#### Arithmetic operations (`add`, `sub`, `mul`, `div`, `rem`)

Performs the operation homomorphically.

Note that division/remainder only support plaintext divisors.

**Examples**

```solidity
// a + b
function add(euint8 a, euint8 b) internal view returns (euint8)
function add(euint8 a, euint16 b) internal view returns (euint16)
function add(uint32 a, euint32 b) internal view returns (euint32)

// a / b
function div(euint8 a, uint8 b) internal pure returns (euint8)
function div(euint16 a, uint16 b) internal pure returns (euint16)
function div(euint32 a, uint32 b) internal pure returns (euint32)
```

#### Min/Max Operations - `min`, `max`

Available for euint\* types:

```solidity
function min(T a, T b) internal returns (T)
function max(T a, T b) internal returns (T)
```

Returns the minimum (resp. maximum) of the two given values.

**Examples**

```solidity
// min(a, b)
function min(euint32 a, euint16 b) internal view returns (euint32)

// max(a, b)
function max(uint32 a, euint8 b) internal view returns (euint32)
```

#### Unary operators (`neg`, `not`)

There are two unary operators: `neg` (`-`) and `not` (`!`). Note that since we work with unsigned integers, the result of negation is interpreted as the modular opposite. The `not` operator returns the value obtained after flipping all the bits of the operand.

{% hint style="info" %}
More information about the behaviour of these operators can be found at the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/fhe-computation/operations/arithmetic-operations).
{% endhint %}

### Bitwise operations

- Bitwise: `FHE.and`, `FHE.or`, `FHE.xor`, `FHE.not`, `FHE.shl`, `FHE.shr`, `FHE.rotl`, `FHE.rotr`

#### Bitwise operations (`AND`, `OR`, `XOR`)

Unlike other binary operations, bitwise operations do not natively accept a mix of ciphertext and plaintext inputs. To ease developer experience, the `FHE` library adds function overloads for these operations. Such overloads implicitly do a trivial encryption before actually calling the operation function, as shown in the examples below.

Available for euint\* types:

```solidity
function and(T a, T b) internal returns (T)
function or(T a, T b) internal returns (T)
function xor(T a, T b) internal returns (T)
```

**Examples**

```solidity
// a & b
function and(euint8 a, euint8 b) internal view returns (euint8)

// implicit trivial encryption of `b` before calling the operator
function and(euint8 a, uint16 b) internal view returns (euint16)
```

#### Bit shift operations (`<<`, `>>`)

Shifts the bits of the base two representation of `a` by `b` positions.

**Examples**

```solidity
// a << b
function shl(euint16 a, euint8 b) internal view returns (euint16)
// a >> b
function shr(euint32 a, euint16 b) internal view returns (euint32)
```

#### Rotate operations

Rotates the bits of the base two representation of `a` by `b` positions.

&#x20;**Examples**

```solidity
function rotl(euint16 a, euint8 b) internal view returns (euint16)
function rotr(euint32 a, euint16 b) internal view returns (euint32)
```

### Comparison operation (`eq`, `ne`, `ge`, `gt`, `le`, `lt`)

{% hint style="info" %}
**Note** that in the case of ciphertext-plaintext operations, since our backend only accepts plaintext right operands, calling the operation with a plaintext left operand will actually invert the operand order and call the _opposite_ comparison.
{% endhint %}

The result of comparison operations is an encrypted boolean (`ebool`). In the backend, the boolean is represented by an encrypted unsigned integer of bit width 8, but this is abstracted away by the Solidity library.

Available for all encrypted types:

```solidity
function eq(T a, T b) internal returns (ebool)
function ne(T a, T b) internal returns (ebool)
```

Additional comparisons for euint\* types:

```solidity
function ge(T a, T b) internal returns (ebool)
function gt(T a, T b) internal returns (ebool)
function le(T a, T b) internal returns (ebool)
function lt(T a, T b) internal returns (ebool)
```

#### Examples

```solidity
// a == b
function eq(euint32 a, euint16 b) internal view returns (ebool)

// actually returns `lt(b, a)`
function gt(uint32 a, euint16 b) internal view returns (ebool)

// actually returns `gt(a, b)`
function gt(euint16 a, uint32 b) internal view returns (ebool)
```

### Multiplexer operator (`select`)

```solidity
function select(ebool control, T a, T b) internal returns (T)
```

If control is true, returns a, otherwise returns b. Available for all encrypted types (ebool, euintX, eaddress).

This operator takes three inputs. The first input `b` is of type `ebool` and the two others of type `euintX`. If `b` is an encryption of `true`, the first integer parameter is returned. Otherwise, the second integer parameter is returned.

#### Example

```solidity
// if (b == true) return val1 else return val2
function select(ebool b, euint8 val1, euint8 val2) internal view returns (euint8) {
  return FHE.select(b, val1, val2);
}
```

### Generating random encrypted integers

Random encrypted integers can be generated fully on-chain.

That can only be done during transactions and not on an `eth_call` RPC method, because PRNG state needs to be mutated on-chain during generation.

#### Example

```solidity
// Generate a random encrypted unsigned integer `r`.
euint32 r = FHE.randEuint32();
```

## Access control functions

The `FHE` library provides a robust set of access control functions for managing permissions on encrypted values. These functions ensure that encrypted data can only be accessed or manipulated by authorized accounts or contracts.

### Permission management

#### Functions

```solidity
function allow(T value, address account) internal
function allowThis(T value) internal
function allowTransient(T value, address account) internal
```

**Descriptions**

- **`allow`**: Grants **permanent access** to a specific address. Permissions are stored persistently in a dedicated ACL contract.
- **`allowThis`**: Grants the **current contract** access to an encrypted value.
- **`allowTransient`**: Grants **temporary access** to a specific address for the duration of the transaction. Permissions are stored in transient storage for reduced gas costs.

#### Access control list (ACL) overview

The `allow` and `allowTransient` functions enable fine-grained control over who can access and decrypt encrypted values. Temporary permissions (`allowTransient`) are ideal for minimizing gas usage in scenarios where access is needed only within a single transaction.

**Example: granting access**

```solidity
// Store an encrypted value.
euint32 r = FHE.asEuint32(94);

// Grant permanent access to the current contract.
FHE.allowThis(r);

// Grant permanent access to the caller.
FHE.allow(r, msg.sender);

// Grant temporary access to an external account.
FHE.allowTransient(r, 0x1234567890abcdef1234567890abcdef12345678);
```

### Permission checks

#### Functions

```solidity
function isAllowed(T value, address account) internal view returns (bool)
function isSenderAllowed(T value) internal view returns (bool)
```

#### Descriptions

- **`isAllowed`**: Checks whether a specific address has permission to access a ciphertext.
- **`isSenderAllowed`**: Similar to `isAllowed`, but automatically checks permissions for the `msg.sender`.

{% hint style="info" %}
Both functions return `true` if the ciphertext is authorized for the specified address, regardless of whether the allowance is stored in the ACL contract or in transient storage.
{% endhint %}

#### Verifying Permissions

These functions help ensure that only authorized accounts or contracts can access encrypted values.

&#x20;**Example: permission verification**

```solidity
// Store an encrypted value.
euint32 r = FHE.asEuint32(94);

// Verify if the current contract is allowed to access the value.
bool isContractAllowed = FHE.isAllowed(r, address(this)); // returns true

// Verify if the caller has access to the value.
bool isCallerAllowed = FHE.isSenderAllowed(r); // depends on msg.sender
```

## Storage Management

### **Function**

```solidity
function cleanTransientStorage() internal
```

### Description

- **`cleanTransientStorage`**: Removes all temporary permissions from transient storage. Use this function at the end of a transaction to ensure no residual permissions remain.

### Example

```solidity
// Clean up transient storage at the end of a function.
function finalize() public {
  // Perform operations...

  // Clean up transient storage.
  FHE.cleanTransientStorage();
}
```

## Public decryption functions

These functions support the three-step public decryption workflow. For a complete tutorial, see [Public decryption](decryption/oracle.md).

### Make publicly decryptable

```solidity
function makePubliclyDecryptable(T value) internal returns (T)
```

Marks an encrypted value as publicly decryptable. Once called, any entity can request the off-chain decryption of this value via the relayer SDK. Supported for all encrypted types (T can be ebool, euintX, eaddress). The calling contract must have ACL permission to access the handle.

### Check if publicly decryptable

```solidity
function isPubliclyDecryptable(T value) internal view returns (bool)
```

Returns true if the encrypted value has been marked as publicly decryptable. Supported for all encrypted types.

### Verify decryption signatures

```solidity
function checkSignatures(
    bytes32[] memory handlesList,
    bytes memory abiEncodedCleartexts,
    bytes memory decryptionProof
) internal
```

Verifies that the cleartext values submitted on-chain match the authentic decryption results from the KMS. Reverts if:

- The `decryptionProof` is empty or has invalid length
- The number of valid signatures is below the KMS signers threshold
- Any signature is from a non-registered KMS signer

Emits a `PublicDecryptionVerified(handlesList, abiEncodedCleartexts)` event on success.

{% hint style="warning" %}
The order of handles in `handlesList` must match the order used when calling `publicDecrypt` off-chain. A proof computed for `[handleA, handleB]` is different from a proof computed for `[handleB, handleA]`.
{% endhint %}

### Validate decryption result (view)

```solidity
function isPublicDecryptionResultValid(
    bytes32[] memory handlesList,
    bytes memory abiEncodedCleartexts,
    bytes memory decryptionProof
) internal view returns (bool)
```

A `view` variant of `checkSignatures`. Returns `true` if the KMS signatures are valid, `false` otherwise (or reverts on malformed input). Unlike `checkSignatures`, this function does not emit events or cache results.

{% hint style="info" %}
Prefer `checkSignatures` over this function in most cases. `checkSignatures` is optimized for gas via signature caching, emits a `PublicDecryptionVerified` event for indexers, and is the standard approach for on-chain verification. Use `isPublicDecryptionResultValid` only when you need a read-only validation check (for example, in off-chain simulations).

Neither function provides replay protection on its own — emitting an event does not prevent the same `(handles, cleartexts, proof)` triple from being submitted twice. The callback that consumes the cleartexts must implement its own replay/state guard (see [Public Decryption](decryption/oracle.md)).
{% endhint %}

### Convert to bytes32

```solidity
function toBytes32(T value) internal pure returns (bytes32)
```

Converts an encrypted type handle to its underlying `bytes32` representation. Supported for all encrypted types (ebool, euintX, eaddress). This is required when building the `handlesList` array for `checkSignatures`.

**Example**

```solidity
bytes32[] memory handles = new bytes32[](2);
handles[0] = FHE.toBytes32(encryptedFoo);
handles[1] = FHE.toBytes32(encryptedBar);
```

## User decryption delegation

These functions transfer the rights of the `(delegator, contractAddress)` user-decryption pair to a new pair `(delegate, contractAddress)` for the same handles.

When called from a contract, **the calling contract is the delegator** (`msg.sender` to the ACL is `address(this)`). EOAs that want to delegate their own rights must call `IACL.delegateForUserDecryption` on the ACL contract directly. See [User decryption delegation](acl/delegation.md) for the full guide.

### Delegate user decryption

```solidity
function delegateUserDecryption(address delegate, address contractAddress, uint64 expirationDate) internal
function delegateUserDecryptionWithoutExpiration(address delegate, address contractAddress) internal
```

Delegates the caller contract's user decryption rights to `delegate` for ciphertexts associated with `contractAddress`. The delegation can have an expiration date or be indefinite.

The ACL enforces the following invariants — all must hold or the call reverts:

- `contractAddress != address(this)` (reverts with `IACL-SenderCannotBeContractAddress`).
- `delegate != address(this)` (reverts with `IACL-SenderCannotBeDelegate`).
- `delegate != contractAddress` (reverts with `IACL-DelegateCannotBeContractAddress`).
- `expirationDate > block.timestamp` (reverts with `IACL-ExpirationDateInThePast`).
- At most one delegate-or-revoke per block for a given `(address(this), delegate, contractAddress)` tuple.

### Batch delegate user decryption

```solidity
function delegateUserDecryptions(
    address delegate,
    address[] memory contractAddresses,
    uint64 expirationDate
) internal

function delegateUserDecryptionsWithoutExpiration(
    address delegate,
    address[] memory contractAddresses
) internal
```

Delegates user decryption rights across multiple contracts in a single call.

### Revoke user decryption delegation

```solidity
function revokeUserDecryptionDelegation(address delegate, address contractAddress) internal
function revokeUserDecryptionDelegations(address delegate, address[] memory contractAddresses) internal
```

Revokes previously granted decryption delegation for one or more contracts.

### Query delegation status

```solidity
function isDelegatedForUserDecryption(
    address delegator,
    address delegate,
    address contractAddress,
    bytes32 handle
) internal view returns (bool)

function getDelegatedUserDecryptionExpirationDate(
    address delegator,
    address delegate,
    address contractAddress
) internal view returns (uint64)

function isUserDecryptable(bytes32 handle, address user, address contractAddress) internal view returns (bool)
```

- **`isDelegatedForUserDecryption`**: Checks if `delegate` has active decryption delegation from `delegator` for a specific handle and contract.
- **`getDelegatedUserDecryptionExpirationDate`**: Returns the expiration timestamp of a delegation. Returns `0` if no delegation exists.
- **`isUserDecryptable`**: Checks if a handle can be decrypted by `user` in the context of `contractAddress`. Returns `true` only if both the user and the contract have persistent ACL permission on the handle.

## Account deny list

```solidity
function isAccountDenied(address account) internal view returns (bool)
```

Returns whether the given account is on the deny list. Denied accounts cannot interact with encrypted values.

## Additional notes

- **Underlying implementation**:\
  All encrypted operations and access control functionalities are performed through the underlying `Impl` library.
- **Uninitialized values**:\
  Uninitialized encrypted values are treated as `0` (for integers) or `false` (for booleans) in computations.
- **Implicit casting**:\
  Type conversion between encrypted integers of different bit widths is supported through implicit casting, allowing seamless operations without additional developer intervention.
