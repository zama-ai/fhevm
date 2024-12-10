# Gas estimation in fhEVM

This guide explains how to estimate gas costs for Fully Homomorphic Encryption (FHE) operations in your smart contracts on Zama's fhEVM. Understanding gas consumption is critical for designing efficient confidential smart contracts.

## Overview

FHE operations in fhEVM are computationally intensive, resulting in higher gas costs compared to standard Ethereum operations. This is due to the complex mathematical operations required to ensure privacy and security.

### Types of gas in fhEVM

1. **Native Gas**:
   - Represents gas used for standard Ethereum operations.
   - On fhEVM, native gas consumption is approximately 20% higher than in mocked environments.
2. **FHE Gas**:
   - Represents gas consumed by FHE-specific computations.
   - FHE gas is consistent across both mocked and real fhEVM environments.

> **Note**: Gas values provided are approximate and may vary based on network conditions, implementation details, and contract complexity.

---

## Measuring gas consumption

To monitor gas usage during development, use the following tools:

1. **`getFHEGasFromTxReceipt`**:

   - Extracts FHE gas consumption from a transaction receipt.
   - Works in only in mocked fhEVM environments.

2. **`.gasUsed`**:
   - Returns the native gas used during a transaction.
   - In mocked mode, this value underestimates real native gas usage by ~20%.
   - Works in both mocked and real fhEVM environments.

### Example: gas measurement

The following code demonstrates how to measure both FHE gas and native gas during a transaction:

```typescript
const transaction = await tx.wait();
expect(transaction?.status).to.eq(1);

if (network.name === "hardhat") {
  const FHEGasConsumed = getFHEGasFromTxReceipt(transaction);
  console.log("FHE Gas Consumed:", FHEGasConsumed);
}

console.log("Native Gas Consumed:", transaction.gasUsed);
```

## Gas limit

The current devnet has a gas limit of **10,000,000**. Here's what you need to know:

- If you send a transaction that exceeds this limit:
  - The transaction will fail to execute
  - Your wallet will be unable to emit new transactions
  - You'll need to send a new transaction with the same nonce but correct gas limit

## Gas costs for common operations

### Boolean Operations (`ebool`)

| Function Name    | Gas Cost |
| ---------------- | -------- |
| `and`/`or`/`xor` | 26,000   |
| `not`            | 30,000   |

---

### Unsigned integer operations

Gas costs increase with the bit-width of the encrypted integer type. Below are the detailed costs for various operations on encrypted types.

#### **4-bit Encrypted Integers (`euint4`)**

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

#### **8-bit Encrypted integers (`euint8`)**

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

#### **16-bit Encrypted integers (`euint16`)**

| Function name          | Gas     |
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

#### **32-bit Encrypted Integers (`euint32`)**

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

#### **64-bit Encrypted integers (`euint64`)**

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

#### **128-bit Encrypted integers (`euint128`)**

| Function name          | Gas fee   |
| ---------------------- | --------- |
| `add`/`sub`            | 218,000   |
| `add`/`sub` (scalar)   | 218,000   |
| `mul`                  | 1,145,000 |
| `mul` (scalar)         | 480,000   |
| `div` (scalar)         | 857,000   |
| `rem` (scalar)         | 1,499,000 |
| `and`/`or`/`xor`       | 41,000    |
| `shr`/`shl`            | 282,000   |
| `shr`/`shl` (scalar)   | 41,000    |
| `rotr`/`rotl`          | 282,000   |
| `rotr`/`rotl` (scalar) | 41,000    |
| `eq`/`ne`              | 88,000    |
| `ge`/`gt`/`le`/`lt`    | 190,000   |
| `min`/`max`            | 241,000   |
| `min`/`max` (scalar)   | 225,000   |
| `neg`                  | 248,000   |
| `not`                  | 38,000    |
| `select`               | 70,000    |

#### **256-bit Encrypted integers (`euint256`)**

| function name          | Gas fee   |
| ---------------------- | --------- |
| `add`/`sub`            | 253,000   |
| `add`/`sub` (scalar)   | 253,000   |
| `mul`                  | 2,045,000 |
| `mul` (scalar)         | 647,000   |
| `div` (scalar)         | 1,258,000 |
| `rem` (scalar)         | 2,052,000 |
| `and`/`or`/`xor`       | 44,000    |
| `shr`/`shl`            | 350,000   |
| `shr`/`shl` (scalar)   | 44,000    |
| `rotr`/`rotl`          | 350,000   |
| `rotr`/`rotl` (scalar) | 44,000    |
| `eq`/`ne`              | 100,000   |
| `ge`/`gt`/`le`/`lt`    | 231,000   |
| `min`/`max`            | 277,000   |
| `min`/`max` (scalar)   | 264,000   |
| `neg`                  | 309,000   |
| `not`                  | 39,000    |
| `select`               | 90,000    |

### eAddress

| Function name | Gas fee |
| ------------- | ------- |
| `eq`/`ne`     | 90,000  |

## Additional Operations

| Function name               | Gas fee         |
| --------------------------- | --------------- |
| `cast`                      | 200             |
| `trivialEncrypt` (basic)    | 100-800         |
| `trivialEncrypt` (extended) | 1,600-6,400     |
| `randBounded`               | 100,000         |
| `ifThenElse`                | 43,000-300,000  |
| `rand`                      | 100,000-400,000 |

## Fixing Failed Transactions in MetaMask

To resolve a failed transaction due to gas limits:

1. Open MetaMask and go to Settings
2. Navigate to Advanced Settings
3. Enable "Customize transaction nonce"
4. When resending the transaction:
   - Use the same nonce as the failed transaction
   - Set an appropriate gas limit under 10M
   - Adjust other parameters as needed

This allows you to "replace" the failed transaction with a valid one using the correct gas parameters.
