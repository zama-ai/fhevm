# 1. TFHE Library API Documentation

This document provides an overview of the functions available in the `TFHE` Solidity library. The TFHE library provides functionality for working with encrypted types and performing operations on them. It implements fully homomorphic encryption (FHE) operations in Solidity.

## 1.1. Overview

The `TFHE` Solidity library provides essential functionality for working with encrypted data types and performing fully homomorphic encryption (FHE) operations in smart contracts. It is designed to streamline the developer experience while maintaining flexibility and performance.

### 1.1.1. **Core Functionality**

- **Homomorphic Operations**: Enables arithmetic, bitwise, and comparison operations on encrypted values.
- **Ciphertext-Plaintext Interoperability**: Supports operations that mix encrypted and plaintext operands, provided the plaintext operand's size does not exceed the encrypted operand's size.
  - Example: `add(uint8 a, euint8 b)` is valid, but `add(uint32 a, euint16 b)` is not.
  - Ciphertext-plaintext operations are generally faster and consume less gas than ciphertext-ciphertext operations.
- **Implicit Upcasting**: Automatically adjusts operand types when necessary to ensure compatibility during operations on encrypted data.

### 1.1.2. **Key Features**

- **Flexibility**: Handles a wide range of encrypted data types, including booleans, integers, addresses, and byte arrays.
- **Performance Optimization**: Prioritizes efficient computation by supporting optimized operator versions for mixed plaintext and ciphertext inputs.
- **Ease of Use**: Offers consistent APIs across all supported data types, enabling a smooth developer experience.

The library ensures that all operations on encrypted data follow the constraints of FHE while abstracting complexity, allowing developers to focus on building privacy-preserving smart contracts.

## 1.2. Table of Contents

- [1. TFHE Library API Documentation](#1-tfhe-library-api-documentation)
  - [1.1. Overview](#11-overview)
    - [1.1.1. **Core Functionality**](#111-core-functionality)
    - [1.1.2. **Key Features**](#112-key-features)
  - [1.2. Table of Contents](#12-table-of-contents)
- [2. Types](#2-types)
  - [2.1. Encrypted Data Types](#21-encrypted-data-types)
    - [2.1.1. Boolean](#211-boolean)
    - [2.1.2. Unsigned Integers](#212-unsigned-integers)
    - [2.1.3. Addresses \& Bytes](#213-addresses--bytes)
    - [2.1.4. Special Types](#214-special-types)
  - [2.2. Casting Types](#22-casting-types)
    - [2.2.1. `asEuint`](#221-aseuint)
      - [2.2.1.1. Examples](#2211-examples)
    - [2.2.2. `asEbool`](#222-asebool)
- [3. Core Functions](#3-core-functions)
  - [3.1. Configuration](#31-configuration)
  - [3.2. Initialization Checks](#32-initialization-checks)
  - [3.3. Arithmetic operations](#33-arithmetic-operations)
    - [3.3.1. Arithmetic operations (`add`, `sub`, `mul`, `div`, `rem`)](#331-arithmetic-operations-add-sub-mul-div-rem)
      - [3.3.1.1. Examples](#3311-examples)
    - [3.3.2. Min/Max Operations - `min`, `max`](#332-minmax-operations---min-max)
      - [3.3.2.1. Examples](#3321-examples)
    - [3.3.3. Unary operators (`neg`, `not`)](#333-unary-operators-neg-not)
  - [3.4. Bitwise operations](#34-bitwise-operations)
    - [3.4.1. Bitwise operations (`AND`, `OR`, `XOR`)](#341-bitwise-operations-and-or-xor)
      - [3.4.1.1. Examples](#3411-examples)
    - [3.4.2. Bit shift operations (`<<`, `>>`)](#342-bit-shift-operations--)
      - [3.4.2.1. Examples](#3421-examples)
    - [3.4.3. Rotate operations](#343-rotate-operations)
      - [3.4.3.1. Examples](#3431-examples)
  - [3.5. Comparison operation (`eq`, `ne`, `ge`, `gt`, `le`, `lt`)](#35-comparison-operation-eq-ne-ge-gt-le-lt)
    - [3.5.1. Examples](#351-examples)
  - [3.6. Multiplexer operator (`select`)](#36-multiplexer-operator-select)
    - [3.6.1. Example](#361-example)
  - [3.7. Generating random encrypted integers](#37-generating-random-encrypted-integers)
    - [3.7.1. Example](#371-example)
- [4. Access control functions](#4-access-control-functions)
  - [4.1. Permission management](#41-permission-management)
    - [4.1.1. Functions](#411-functions)
      - [4.1.1.1. Descriptions](#4111-descriptions)
    - [4.1.2. Access control list (ACL) overview](#412-access-control-list-acl-overview)
      - [4.1.2.1. Example: granting access](#4121-example-granting-access)
  - [4.2. Permission checks](#42-permission-checks)
    - [4.2.1. Functions](#421-functions)
    - [4.2.2. Descriptions](#422-descriptions)
    - [4.2.3. Verifying Permissions](#423-verifying-permissions)
      - [4.2.3.1. Example: permission verification](#4231-example-permission-verification)
- [5. Storage Management](#5-storage-management)
  - [5.1. **Function**](#51-function)
  - [5.2. Description](#52-description)
  - [5.3. Example](#53-example)
- [6. Additional Notes](#6-additional-notes)

# 2. Types

## 2.1. Encrypted Data Types

### 2.1.1. Boolean

- `ebool`: Encrypted boolean value

### 2.1.2. Unsigned Integers

- `euint4`: Encrypted 4-bit unsigned integer
- `euint8`: Encrypted 8-bit unsigned integer
- `euint16`: Encrypted 16-bit unsigned integer
- `euint32`: Encrypted 32-bit unsigned integer
- `euint64`: Encrypted 64-bit unsigned integer
- `euint128`: Encrypted 128-bit unsigned integer
- `euint256`: Encrypted 256-bit unsigned integer

### 2.1.3. Addresses & Bytes

- `eaddress`: Encrypted Ethereum address
- `ebytes64`: Encrypted 64-byte value
- `ebytes128`: Encrypted 128-byte value
- `ebytes256`: Encrypted 256-byte value

### 2.1.4. Special Types

- `einput`: Input type for encrypted operations (bytes32)

## 2.2. Casting Types

- **Casting between encrypted types**: `TFHE.asEbool` converts encrypted integers to encrypted booleans
- **Casting to encrypted types**: `TFHE.asEuintX` converts plaintext values to encrypted types
- **Casting to encrypted addresses**: `TFHE.asEaddress` converts plaintext addresses to encrypted addresses
- **Casting to encrypted bytes**: `TFHE.asEbytesX` converts plaintext bytes to encrypted bytes

### 2.2.1. `asEuint`

The `asEuint` functions serve three purposes:

1. verify ciphertext bytes and return a valid handle to the calling smart contract;
2. cast a `euintX` typed ciphertext to a `euintY` typed ciphertext, where `X != Y`;
3. trivially encrypt a plaintext value.

The first case is used to process encrypted inputs, e.g. user-provided ciphertexts. Those are generally included in a transaction payload.

The second case is self-explanatory. When `X > Y`, the most significant bits are dropped. When `X < Y`, the ciphertext is padded to the left with trivial encryptions of `0`.

The third case is used to "encrypt" a public value so that it can be used as a ciphertext.
Note that what we call a trivial encryption is **not** secure in any sense.
When trivially encrypting a plaintext value, this value is still visible in the ciphertext bytes.
More information about trivial encryption can be found [here](https://www.zama.ai/post/tfhe-deep-dive-part-1).

#### 2.2.1.1. Examples

```solidity
// first case
function asEuint8(bytes memory ciphertext) internal view returns (euint8)
// second case
function asEuint16(euint8 ciphertext) internal view returns (euint16)
// third case
function asEuint16(uint16 value) internal view returns (euint16)
```

### 2.2.2. `asEbool`

The `asEbool` functions behave similarly to the `asEuint` functions, but for encrypted boolean values.

# 3. Core Functions

## 3.1. Configuration

```solidity
function setFHEVM(FHEVMConfig.FHEVMConfigStruct memory fhevmConfig) internal
```

Sets the FHEVM configuration for encrypted operations.

## 3.2. Initialization Checks

```solidity
function isInitialized(T v) internal pure returns (bool)
```

Returns true if the encrypted value is initialized, false otherwise.
Supported for all encrypted types (T can be ebool, euint*, eaddress, ebytes*).

## 3.3. Arithmetic operations

Available for euint\* types:

```solidity
function add(T a, T b) internal returns (T)
function sub(T a, T b) internal returns (T)
function mul(T a, T b) internal returns (T)
```

- Arithmetic: `TFHE.add`, `TFHE.sub`, `TFHE.mul`, `TFHE.min`, `TFHE.max`, `TFHE.neg`, `TFHE.div`, `TFHE.rem`
  - Note: `div` and `rem` operations are supported only with plaintext divisors

### 3.3.1. Arithmetic operations (`add`, `sub`, `mul`, `div`, `rem`)

Performs the operation homomorphically.

Note that division/remainder only support plaintext divisors.

#### 3.3.1.1. Examples

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

### 3.3.2. Min/Max Operations - `min`, `max`

Available for euint\* types:

```solidity
function min(T a, T b) internal returns (T)
function max(T a, T b) internal returns (T)
```

Returns the minimum (resp. maximum) of the two given values.

#### 3.3.2.1. Examples

```solidity
// min(a, b)
function min(euint32 a, euint16 b) internal view returns (euint32)

// max(a, b)
function max(uint32 a, euint8 b) internal view returns (euint32)
```

### 3.3.3. Unary operators (`neg`, `not`)

There are two unary operators: `neg` (`-`) and `not` (`!`).
Note that since we work with unsigned integers, the result of negation is interpreted as the modular opposite.
The `not` operator returns the value obtained after flipping all the bits of the operand.

> **_NOTE:_** More information about the behavior of these operators can be found at the [TFHE-rs docs](https://docs.zama.ai/tfhe-rs/getting-started/operations#arithmetic-operations.).

## 3.4. Bitwise operations

- Bitwise: `TFHE.and`, `TFHE.or`, `TFHE.xor`, `TFHE.not`, `TFHE.shl`, `TFHE.shr`, `TFHE.rotl`, `TFHE.rotr`

### 3.4.1. Bitwise operations (`AND`, `OR`, `XOR`)

Unlike other binary operations, bitwise operations do not natively accept a mix of ciphertext and plaintext inputs.
To ease developer experience, the `TFHE` library adds function overloads for these operations.
Such overloads implicitely do a trivial encryption before actually calling the operation function, as shown in the examples below.

Available for euint\* types:

```solidity
function and(T a, T b) internal returns (T)
function or(T a, T b) internal returns (T)
function xor(T a, T b) internal returns (T)
```

#### 3.4.1.1. Examples

```solidity
// a & b
function and(euint8 a, euint8 b) internal view returns (euint8)

// implicit trivial encryption of `b` before calling the operator
function and(euint8 a, uint16 b) internal view returns (euint16)
```

### 3.4.2. Bit shift operations (`<<`, `>>`)

Shifts the bits of the base two representation of `a` by `b` positions.

#### 3.4.2.1. Examples

```solidity
// a << b
function shl(euint16 a, euint8 b) internal view returns (euint16)
// a >> b
function shr(euint32 a, euint16 b) internal view returns (euint32)
```

### 3.4.3. Rotate operations

Rotates the bits of the base two representation of `a` by `b` positions.

#### 3.4.3.1. Examples

```solidity
function rotl(euint16 a, euint8 b) internal view returns (euint16)
function rotr(euint32 a, euint16 b) internal view returns (euint32)
```

## 3.5. Comparison operation (`eq`, `ne`, `ge`, `gt`, `le`, `lt`)

> **Note** that in the case of ciphertext-plaintext operations, since our backend only accepts plaintext right operands, calling the operation with a plaintext left operand will actually invert the operand order and call the _opposite_ comparison.

The result of comparison operations is an encrypted boolean (`ebool`). In the backend, the boolean is represented by an encrypted unsinged integer of bit width 8, but this is abstracted away by the Solidity library.

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

### 3.5.1. Examples

```solidity
// a == b
function eq(euint32 a, euint16 b) internal view returns (ebool)

// actually returns `lt(b, a)`
function gt(uint32 a, euint16 b) internal view returns (ebool)

// actually returns `gt(a, b)`
function gt(euint16 a, uint32 b) internal view returns (ebool)
```

## 3.6. Multiplexer operator (`select`)

```solidity
function select(ebool control, T a, T b) internal returns (T)
```

If control is true, returns a, otherwise returns b.
Available for ebool, eaddress, and ebytes\* types.

This operator takes three inputs. The first input `b` is of type `ebool` and the two others of type `euintX`.
If `b` is an encryption of `true`, the first integer parameter is returned. Otherwise, the second integer parameter is returned.

### 3.6.1. Example

```solidity
// if (b == true) return val1 else return val2
function select(ebool b, euint8 val1, euint8 val2) internal view returns (euint8) {
  return TFHE.select(b, val1, val2);
}
```

## 3.7. Generating random encrypted integers

Random encrypted integers can be generated fully on-chain.

That can only be done during transactions and not on an `eth_call` RPC method,
because PRNG state needs to be mutated on-chain during generation.

### 3.7.1. Example

```solidity
// Generate a random encrypted unsigned integer `r`.
euint32 r = TFHE.randEuint32();
```

# 4. Access control functions

The `TFHE` library provides a robust set of access control functions for managing permissions on encrypted values. These functions ensure that encrypted data can only be accessed or manipulated by authorized accounts or contracts.

## 4.1. Permission management

### 4.1.1. Functions

```solidity
function allow(T value, address account) internal
function allowThis(T value) internal
function allowTransient(T value, address account) internal
```

#### 4.1.1.1. Descriptions

- **`allow`**: Grants **permanent access** to a specific address. Permissions are stored persistently in a dedicated ACL contract.
- **`allowThis`**: Grants the **current contract** access to an encrypted value.
- **`allowTransient`**: Grants **temporary access** to a specific address for the duration of the transaction. Permissions are stored in transient storage for reduced gas costs.

### 4.1.2. Access control list (ACL) overview

The `allow` and `allowTransient` functions enable fine-grained control over who can access, decrypt, and reencrypt encrypted values. Temporary permissions (`allowTransient`) are ideal for minimizing gas usage in scenarios where access is needed only within a single transaction.

#### 4.1.2.1. Example: granting access

```solidity
// Store an encrypted value.
euint32 r = TFHE.asEuint32(94);

// Grant permanent access to the current contract.
TFHE.allowThis(r);

// Grant permanent access to the caller.
TFHE.allow(r, msg.sender);

// Grant temporary access to an external account.
TFHE.allowTransient(r, 0x1234567890abcdef1234567890abcdef12345678);
```

## 4.2. Permission checks

### 4.2.1. Functions

```solidity
function isAllowed(T value, address account) internal view returns (bool)
function isSenderAllowed(T value) internal view returns (bool)
```

### 4.2.2. Descriptions

- **`isAllowed`**: Checks whether a specific address has permission to access a ciphertext.
- **`isSenderAllowed`**: Similar to `isAllowed`, but automatically checks permissions for the `msg.sender`.

> **Note**: Both functions return `true` if the ciphertext is authorized for the specified address, regardless of whether the allowance is stored in the ACL contract or in transient storage.

### 4.2.3. Verifying Permissions

These functions help ensure that only authorized accounts or contracts can access encrypted values.

#### 4.2.3.1. Example: permission verification

```solidity
// Store an encrypted value.
euint32 r = TFHE.asEuint32(94);

// Verify if the current contract is allowed to access the value.
bool isContractAllowed = TFHE.isAllowed(r, address(this)); // returns true

// Verify if the caller has access to the value.
bool isCallerAllowed = TFHE.isSenderAllowed(r); // depends on msg.sender
```

# 5. Storage Management

## 5.1. **Function**

```solidity
function cleanTransientStorage() internal
```

## 5.2. Description

- **`cleanTransientStorage`**: Removes all temporary permissions from transient storage. Use this function at the end of a transaction to ensure no residual permissions remain.

## 5.3. Example

```solidity
// Clean up transient storage at the end of a function.
function finalize() public {
  // Perform operations...

  // Clean up transient storage.
  TFHE.cleanTransientStorage();
}
```

# 6. Additional Notes

1. **Underlying Implementation**:  
   All encrypted operations and access control functionalities are performed through the underlying `Impl` library.

2. **Uninitialized Values**:  
   Uninitialized encrypted values are treated as `0` (for integers) or `false` (for booleans) in computations.

3. **Implicit Casting**:  
   Type conversion between encrypted integers of different bit widths is supported through implicit casting, allowing seamless operations without additional developer intervention.
