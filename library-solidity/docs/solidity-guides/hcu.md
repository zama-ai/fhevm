# Homomorphic Complexity Units ("HCU") in fhevm

This guide explains how to use Fully Homomorphic Encryption (FHE) operations in your smart contracts on fhevm. Understanding HCU is critical for designing efficient confidential smart contracts.

## Overview

FHE operations in fhevm are computationally intensive compared to standard Ethereum operations, as they require complex mathematical computations to maintain privacy and security. To manage computational load and prevent potential denial-of-service attacks, fhevm implements a metering system called **Homomorphic Complexity Units ("HCU")**.

Each FHE operation consumes a specific amount of HCU based on its computational complexity. The `HCULimit` contract monitors HCU consumption for each transaction and enforces two key limits:

- **Sequential homomorphic operations depth limit per transaction**: Controls HCU usage for operations that must be processed in order.
- **Global homomorphic operations complexity per transaction**: Controls HCU usage for operations that can be processed in parallel.

If either limit is exceeded, the transaction will revert, ensuring network stability and preventing resource exhaustion.

## Measuring HCU

To monitor HCU during development, you can use the following tool: **`getTxHCUFromTxReceipt`**:

You can import it as such: `import { getTxHCUFromTxReceipt } from "../coprocessorUtils";`

It allows to extract either the total HCU consumption or the maximum depth HCU consumption from a transaction receipt.

### Example

The following code demonstrates how to obtain information about HCU from the logs.

```typescript
import { getTxHCUFromTxReceipt } from "../coprocessorUtils";

const tx = await this.erc20["transfer(address,bytes32,bytes)"](
  this.signers.bob.address,
  encryptedTransferAmount.handles[0],
  encryptedTransferAmount.inputProof,
);
const receipt = await tx.wait();
expect(receipt?.status).to.eq(1);

const {
  globalTxHCU: globalTxHCU,
  maxTxHCUDepth: maxTxHCUDepth,
  HCUDepthPerHandle: hcuDepthPerHandle,
} = getTxHCUFromTxReceipt(tx);

console.log("Total Transaction HCU:", globalTxHCU);
console.log("Maximum transaction HCU depth:", maxTxHCUDepth);
console.log(hcuDepthPerHandle);
```

The output from the code above will look similar to the following:

```
Total Transaction HCU: 586200
Maximum transaction HCU depth: 397000
{
  '0xbd7130bfcc326fda4bb2d1369a8f2aa53f8f537a66ff0000000000007a690000': 156000,
  '0xf1a829fc1ef14cec26872d6671ed40e6e46b923a94ff0000000000007a690500': 600,
  '0xf44b392aec4240d38d0fa468a9334139784f3e1ac7ff0000000000007a690500': 209000,
  '0xf6f5c565fda71893b08bba9b8395d0877a5beb3847ff0000000000007a690500': 397000,
  '0x1f442f6150dae1ca5dd32e1e34c11210f9c37a68edff0000000000007a690500': 397000
}
```

- The first two lines show the total HCU consumed by the transaction and the maximum HCU depth.
- The object lists HCU usage per handle, where each key is a handle identifier and each value is the HCU depth for that handle.

## HCU limit

The current devnet has an HCU limit of **20,000,000** per transaction and an HCU depth limit of **5,000,000** per transaction. If either HCU limit is exceeded, the transaction will revert.

To resolve this, you must do one of the following:

- Refactor your code to reduce the number of FHE operations in your transaction.
- Split your FHE operations across multiple independent transactions.

## HCU costs for common operations

### Boolean operations (`ebool`)

| Function Name    | HCU    |
| ---------------- | ------ |
| `and`/`or`/`xor` | 26,000 |
| `not`            | 30,000 |

---

### Unsigned integer operations

HCU increase with the bit-width of the encrypted integer type. Below are the detailed costs for various operations on encrypted types.

#### **8-bit Encrypted integers (`euint8`)**

| Function name          | HCU     |
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

| Function name          | HCU     |
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

| Function name          | HCU     |
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

| Function name          | HCU       |
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

| Function name          | HCU       |
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

| function name          | HCU     |
| ---------------------- | ------- |
| `and`/`or`/`xor`       | 44,000  |
| `shr`/`shl`            | 350,000 |
| `shr`/`shl` (scalar)   | 44,000  |
| `rotr`/`rotl`          | 350,000 |
| `rotr`/`rotl` (scalar) | 44,000  |
| `eq`/`ne`              | 100,000 |
| `ge`/`gt`/`le`/`lt`    | 231,000 |
| `min`/`max`            | 277,000 |
| `min`/`max` (scalar)   | 264,000 |
| `neg`                  | 309,000 |
| `not`                  | 39,000  |
| `select`               | 90,000  |

### eAddress

| Function name | HCU    |
| ------------- | ------ |
| `eq`/`ne`     | 90,000 |

## Additional Operations

| Function name               | HCU             |
| --------------------------- | --------------- |
| `cast`                      | 200             |
| `trivialEncrypt` (basic)    | 100-800         |
| `trivialEncrypt` (extended) | 1,600-6,400     |
| `randBounded`               | 100,000         |
| `select`                    | 43,000-300,000  |
| `rand`                      | 100,000-400,000 |
