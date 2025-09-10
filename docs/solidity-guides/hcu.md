# Homomorphic Complexity Units ("HCU") in FHEVM

This guide explains how to use Fully Homomorphic Encryption (FHE) operations in your smart contracts on FHEVM. Understanding HCU is critical for designing efficient confidential smart contracts.

## Overview

FHE operations in FHEVM are computationally intensive compared to standard Ethereum operations, as they require complex mathematical computations to maintain privacy and security. To manage computational load and prevent potential denial-of-service attacks, FHEVM implements a metering system called **Homomorphic Complexity Units ("HCU")**.

To represent this complexity, we introduced the **Homomorphic Complexity Unit ("HCU")**. In Solidity, each FHE operation consumes a set amount of HCU based on the operational computational complexity for hardware computation. Since FHE transactions are symbolic, this helps preventing resource exhaustion outside of the blockchain.

To do so, there is a contract named `HCULimit`, which monitors HCU consumption for each transaction and enforces two key limits:

- **Sequential homomorphic operations depth limit per transaction**: Controls HCU usage for operations that must be processed in order.
- **Global homomorphic operations complexity per transaction**: Controls HCU usage for operations that can be processed in parallel.

If either limit is exceeded, the transaction will revert.

## HCU limit

The current devnet has an HCU limit of **20,000,000** per transaction and an HCU depth limit of **5,000,000** per transaction. If either HCU limit is exceeded, the transaction will revert.

To resolve this, you must do one of the following:

- Refactor your code to reduce the number of FHE operations in your transaction.
- Split your FHE operations across multiple independent transactions.

## HCU costs for common operations

### Boolean operations (`ebool`)

| Function name  | HCU (scalar) | HCU (non-scalar) |
| -------------- | ------------ | ---------------- |
| `and`          | 22,000       | 25,000           |
| `or`           | 22,000       | 24,000           |
| `xor`          | 2,000        | 22,000           |
| `not`          |      -       | 2                |
| `select`       |      -       | 55,000           |
| `randEbool`    |      -       | 19,000           | 

---

### Unsigned integer operations

HCU increase with the bit-width of the encrypted integer type. Below are the detailed costs for various operations on encrypted types.

#### **8-bit Encrypted integers (`euint8`)**

| Function name  | HCU (scalar) | HCU (non-scalar) |
| -------------- | ------------ | ---------------- |
| `add`          | 84,000       | 88,000           |
| `sub`          | 84,000       | 91,000           |
| `mul`          | 122,000      | 150,000          |
| `div`          | 210,000      | -                |
| `rem`          | 440,000      | -                |
| `and`          | 31,000       | 31,000           |
| `or`           | 30,000       | 30,000           |
| `xor`          | 31,000       | 31,000           |
| `shr`          | 32,000       | 91,000           |
| `shl`          | 32,000       | 92,000           |
| `rotr`         | 31,000       | 93,000           |
| `rotl`         | 31,000       | 91,000           |
| `eq`           | 55,000       | 55,000           |
| `ne`           | 55,000       | 55,000           |
| `ge`           | 52,000       | 63,000           |
| `gt`           | 52,000       | 59,000           |
| `le`           | 58,000       | 58,000           |
| `lt`           | 52,000       | 59,000           |
| `min`          | 84,000       | 119,000          |
| `max`          | 89,000       | 121,000          |
| `neg`          | -            | 79,000           |
| `not`          | -            | 9                |
| `select`       | -            | 55,000           |
| `randEuint8`   | -            | 23,000           |

#### **16-bit Encrypted integers (`euint16`)**

| Function name   | HCU (scalar) | HCU (non-scalar) |
| --------------- | ------------ | ---------------- |
| `add`           | 93,000       | 93,000           |
| `sub`           | 93,000       | 93,000           |
| `mul`           | 193,000      | 222,000          |
| `div`           | 302,000      | -                |
| `rem`           | 580,000      | -                |
| `and`           | 31,000       | 31,000           |
| `or`            | 30,000       | 31,000           |
| `xor`           | 31,000       | 31,000           |
| `shr`           | 32,000       | 123,000          |
| `shl`           | 32,000       | 125,000          |
| `rotr`          | 31,000       | 125,000          |
| `rotl`          | 31,000       | 125,000          |
| `eq`            | 55,000       | 83,000           |
| `ne`            | 55,000       | 83,000           |
| `ge`            | 55,000       | 84,000           |
| `gt`            | 55,000       | 84,000           |
| `le`            | 58,000       | 83,000           |
| `lt`            | 58,000       | 84,000           |
| `min`           | 88,000       | 146,000          |
| `max`           | 89,000       | 145,000          |
| `neg`           | -            | 93,000           |
| `not`           | -            | 16               |
| `select`        | -            | 55,000           |
| `randEuint16`   | -            | 23,000           |

#### **32-bit Encrypted Integers (`euint32`)**

| Function name   | HCU (scalar) | HCU (non-scalar) |
| --------------- | ------------ | ---------------- |
| `add`           | 95,000       | 125,000          |
| `sub`           | 95,000       | 125,000          |
| `mul`           | 265,000      | 328,000          |
| `div`           | 438,000      | -                |
| `rem`           | 792,000      | -                |
| `and`           | 32,000       | 32,000           |
| `or`            | 32,000       | 32,000           |
| `xor`           | 32,000       | 32,000           |
| `shr`           | 32,000       | 163,000          |
| `shl`           | 32,000       | 162,000          |
| `rotr`          | 32,000       | 160,000          |
| `rotl`          | 32,000       | 163,000          |
| `eq`            | 82,000       | 86,000           |
| `ne`            | 83,000       | 85,000           |
| `ge`            | 84,000       | 118,000          |
| `gt`            | 84,000       | 118,000          |
| `le`            | 84,000       | 117,000          |
| `lt`            | 83,000       | 117,000          |
| `min`           | 117,000      | 182,000          |
| `max`           | 117,000      | 180,000          |
| `neg`           | -            | 131,000          |
| `not`           | -            | 32               |
| `select`        | -            | 55,000           |
| `randEuint32`   | -            | 24,000           |

#### **64-bit Encrypted integers (`euint64`)**

| Function name   | HCU (scalar) | HCU (non-scalar) |
| --------------- | ------------ | ---------------- |
| `add`           | 133,000      | 162,000          |
| `sub`           | 133,000      | 162,000          |
| `mul`           | 365,000      | 596,000          |
| `div`           | 715,000      | -                |
| `rem`           | 1,153,000    | -                |
| `and`           | 34,000       | 34,000           |
| `or`            | 34,000       | 34,000           |
| `xor`           | 34,000       | 34,000           |
| `shr`           | 34,000       | 209,000          |
| `shl`           | 34,000       | 208,000          |
| `rotr`          | 34,000       | 209,000          |
| `rotl`          | 34,000       | 209,000          |
| `eq`            | 83,000       | 120,000          |
| `ne`            | 84,000       | 118,000          |
| `ge`            | 116,000      | 152,000          |
| `gt`            | 117,000      | 152,000          |
| `le`            | 119,000      | 149,000          |
| `lt`            | 118,000      | 146,000          |
| `min`           | 150,000      | 219,000          |
| `max`           | 149,000      | 218,000          |
| `neg`           | -            | 131,000          |
| `not`           | -            | 63               |
| `select`        | -            | 55,000           |
| `randEuint64`   | -            | 24,000           |

#### **128-bit Encrypted integers (`euint128`)**

| Function name    | HCU (scalar) | HCU (non-scalar) |
| ---------------- | ------------ | ---------------- |
| `add`            | 172,000      | 259,000          |
| `sub`            | 172,000      | 260,000          |
| `mul`            | 696,000      | 1,686,000        |
| `div`            | 1,225,000    | -                |
| `rem`            | 1,943,000    | -                |
| `and`            | 37,000       | 37,000           |
| `or`             | 37,000       | 37,000           |
| `xor`            | 37,000       | 37,000           |
| `shr`            | 37,000       | 272,000          |
| `shl`            | 37,000       | 272,000          |
| `rotr`           | 37,000       | 283,000          |
| `rotl`           | 37,000       | 278,000          |
| `eq`             | 117,000      | 122,000          |
| `ne`             | 117,000      | 122,000          |
| `ge`             | 149,000      | 210,000          |
| `gt`             | 150,000      | 218,000          |
| `le`             | 150,000      | 218,000          |
| `lt`             | 149,000      | 215,000          |
| `min`            | 186,000      | 289,000          |
| `max`            | 180,000      | 290,000          |
| `neg`            | -            | 168,000          |
| `not`            | -            | 130              |
| `select`         | -            | 57,000           |
| `randEuint128`   | -            | 25,000           |

#### **256-bit Encrypted integers (`euint256`)**

| Function name    | HCU (scalar) | HCU (non-scalar) |
| ---------------- | ------------ | ---------------- |
| `and`            | 38,000       | 38,000           |
| `or`             | 38,000       | 38,000           |
| `xor`            | 39,000       | 39,000           |
| `shr`            | 38,000       | 369,000          |
| `shl`            | 39,000       | 378,000          |
| `rotr`           | 40,000       | 375,000          |
| `rotl`           | 38,000       | 378,000          |
| `eq`             | 118,000      | 152,000          |
| `ne`             | 117,000      | 150,000          |
| `neg`            | -            | 269,000          |
| `not`            | -            | 130              |
| `select`         | -            | 108,000          |
| `randEuint256`   | -            | 30,000           |

#### **Encrypted addresses (`euint160`)**

When using `eaddress` (internally represented as `euint160`), the HCU costs for equality and inequality checks and select are as follows:

| Function name | HCU (scalar) | HCU (non-scalar) |
| ------------- | ------------ | ---------------- |
| `eq`          | 115,000      | 125,000          |
| `ne`          | 115,000      | 124,000          |
| `select`      | -            | 83,000           |

## Additional Operations

| Function name    | HCU           |
| ---------------- | ------------- |
| `cast`           | 32            |
| `trivialEncrypt` | 32            |
| `randBounded`    | 23,000-30,000 |
