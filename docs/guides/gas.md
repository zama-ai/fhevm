# Gas Estimation in fhEVM

This guide helps you understand and estimate gas costs for Fully Homomorphic Encryption (FHE) operations in your smart contracts.

## Overview

When working with encrypted data in fhEVM, operations consume more gas compared to regular smart contract operations. This is because FHE operations require complex mathematical computations to maintain data privacy and security.

Below you'll find detailed gas cost estimates for common FHE operations across different encrypted data types. Use these as a reference when designing and optimizing your confidential smart contracts.

> **Note**: Gas costs are approximate and may vary slightly based on network conditions and contract complexity.

## ebool

| Function name    | Gas    |
| ---------------- | ------ |
| `and`/`or`/`xor` | 26,000 |
| `not`            | 30,000 |

## euint4

| function name          | Gas     |
| ---------------------- | ------- |
| `add`/`sub`            | 65,000  |
| `add`/`sub` (scalar)   | 65,000  |
| `mul`                  | 150,000 |
| `mul` (scalar)         | 88,000  |
| `div` (scalar)         | 139,000 |
| `rem` (scalar)         | 286,000 |
| `and`/`or`/`xor`       | 32,000  |
| `shr`/`shl`            | 116,000 |
| `shr`/`shl` (scalar)   | 35,000  |
| `rotr`/`rotl`          | 116,000 |
| `rotr`/`rotl` (scalar) | 35,000  |
| `eq`/`ne`              | 51,000  |
| `ge`/`gt`/`le`/`lt`    | 70,000  |
| `min`/`max`            | 121,000 |
| `min`/`max` (scalar)   | 121,000 |
| `neg`                  | 60,000  |
| `not`                  | 33,000  |
| `select`               | 45,000  |

## euint8

| Function name          | Gas     |
| ---------------------- | ------- |
| `add`/`sub`            | 94,000  |
| `add`/`sub` (scalar)   | 94,000  |
| `mul`                  | 197,000 |
| `mul` (scalar)         | 159,000 |
| `div` (scalar)         | 238,000 |
| `rem` (scalar)         | 460,000 |
| `and`/`or`/`xor`       | 34,000  |
| `shr`/`shl`            | 133,000 |
| `shr`/`shl` (scalar)   | 35,000  |
| `rotr`/`rotl`          | 133,000 |
| `rotr`/`rotl` (scalar) | 35,000  |
| `eq`/`ne`              | 53,000  |
| `ge`/`gt`/`le`/`lt`    | 82,000  |
| `min`/`max`            | 128,000 |
| `min`/`max` (scalar)   | 128,000 |
| `neg`                  | 95,000  |
| `not`                  | 34,000  |
| `select`               | 47,000  |
| `randEuint8()`         | 100,000 |

## euint16

| function name          | euint16 |
| ---------------------- | ------- |
| `add`/`sub`            | 133,000 |
| `add`/`sub` (scalar)   | 133,000 |
| `mul`                  | 262,000 |
| `mul` (scalar)         | 208,000 |
| `div` (scalar)         | 314,000 |
| `rem` (scalar)         | 622,000 |
| `and`/`or`/`xor`       | 34,000  |
| `shr`/`shl`            | 153,000 |
| `shr`/`shl` (scalar)   | 35,000  |
| `rotr`/`rotl`          | 153,000 |
| `rotr`/`rotl` (scalar) | 35,000  |
| `eq`/`ne`              | 54,000  |
| `ge`/`gt`/`le`/`lt`    | 105,000 |
| `min`/`max`            | 153,000 |
| `min`/`max` (scalar)   | 150,000 |
| `neg`                  | 131,000 |
| `not`                  | 35,000  |
| `select`               | 47,000  |
| `randEuint16()`        | 100,000 |

## euint32

| Function name          | Gas fee |
| ---------------------- | ------- |
| `add`/`sub`            | 162,000 |
| `add`/`sub` (scalar)   | 162,000 |
| `mul`                  | 359,000 |
| `mul` (scalar)         | 264,000 |
| `div` (scalar)         | 398,000 |
| `rem` (scalar)         | 805,000 |
| `and`/`or`/`xor`       | 35,000  |
| `shr`/`shl`            | 183,000 |
| `shr`/`shl` (scalar)   | 35,000  |
| `rotr`/`rotl`          | 183,000 |
| `rotr`/`rotl` (scalar) | 35,000  |
| `eq`/`ne`              | 82,000  |
| `ge`/`gt`/`le`/`lt`    | 128,000 |
| `min`/`max`            | 183,000 |
| `min`/`max` (scalar)   | 164,000 |
| `neg`                  | 160,000 |
| `not`                  | 36,000  |
| `select`               | 50,000  |
| `randEuint32()`        | 100,000 |

## euint64

| Function name          | Gas fee   |
| ---------------------- | --------- |
| `add`/`sub`            | 188,000   |
| `add`/`sub` (scalar)   | 188,000   |
| `mul`                  | 641,000   |
| `mul` (scalar)         | 356,000   |
| `div` (scalar)         | 584,000   |
| `rem` (scalar)         | 1,095,000 |
| `and`/`or`/`xor`       | 38,000    |
| `shr`/`shl`            | 227,000   |
| `shr`/`shl` (scalar)   | 38,000    |
| `rotr`/`rotl`          | 227,000   |
| `rotr`/`rotl` (scalar) | 38,000    |
| `eq`/`ne`              | 86,000    |
| `ge`/`gt`/`le`/`lt`    | 156,000   |
| `min`/`max`            | 210,000   |
| `min`/`max` (scalar)   | 192,000   |
| `neg`                  | 199,000   |
| `not`                  | 37,000    |
| `select`               | 53,000    |
| `randEuint64()`        | 100,000   |

## eaddress

| Function name | Gas fee |
| ------------- | ------- |
| `eq`/`ne`     | 90,000  |

## Gas limit

The current devnet has a gas limit of **10,000,000**. Here's what you need to know:

- If you send a transaction that exceeds this limit:
  - The transaction will fail to execute
  - Your wallet will be unable to emit new transactions
  - You'll need to send a new transaction with the same nonce but correct gas limit

### Fixing Failed Transactions in MetaMask

To resolve a failed transaction due to gas limits:

1. Open MetaMask and go to Settings
2. Navigate to Advanced Settings
3. Enable "Customize transaction nonce"
4. When resending the transaction:
   - Use the same nonce as the failed transaction
   - Set an appropriate gas limit under 10M
   - Adjust other parameters as needed

This allows you to "replace" the failed transaction with a valid one using the correct gas parameters.
