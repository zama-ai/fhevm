# Homomorphic Complexity Units ("HCU") in FHEVM

This guide explains how to use Fully Homomorphic Encryption (FHE) operations in your smart contracts on FHEVM. Understanding HCU is critical for designing efficient confidential smart contracts.

## Overview

FHE operations in FHEVM are computationally intensive compared to standard Ethereum operations, as they require complex mathematical computations to maintain privacy and security. To manage computational load and prevent potential denial-of-service attacks, FHEVM implements a metering system called **Homomorphic Complexity Units ("HCU")**.

To represent this complexity, we introduced the **Homomorphic Complexity Unit ("HCU")**. In Solidity, each FHE operation consumes a set amount of HCU based on the operational computational complexity for hardware computation. Since FHE transactions are symbolic, this helps preventing resource exhaustion outside of the blockchain.

To do so, there is a contract named `HCULimit`, which monitors HCU consumption for each transaction and enforces two key limits:

- **Sequential homomorphic operations depth limit per transaction**: Controls HCU usage for operations that must be processed in order.
- **Global homomorphic operations complexity per transaction**: Controls HCU usage for operations that can be processed in parallel.

If either limit is exceeded, the transaction will revert.

## Measuring HCU

To monitor HCU during development, you can use the following helper function: **`getTxHCUFromTxReceipt`**:

You can import it as such: `import { getTxHCUFromTxReceipt } from "../coprocessorUtils";`
It allows to extract either the total HCU consumption or the maximum depth HCU consumption from a Solidity transaction receipt.

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

| Function name          | HCU (scalar) | HCU (non-scalar) |
| ---------------------- | ------------ | ---------------- |
| `add`                  | 84,000       | 87,000           |
| `sub`                  | 83,000       | 84,000           |
| `mul`                  | 117,000      | 146,000          |
| `div`                  | 203,000      | -                |
| `rem`                  | 387,000      | -                |
| `and`                  | 28,000       | 29,000           |
| `or`                   | 28,000       | 28,000           |
| `xor`                  | 29,000       | 29,000           |
| `shr`                  | 28,000       | 88,000           |
| `shl`                  | 29,000       | 86,000           |
| `rotr`                 | 29,000       | 86,000           |
| `rotl`                 | 29,000       | 87,000           |
| `eq`                   | 52,000       | 49,000           |
| `ne`                   | 49,000       | 52,000           |
| `ge`                   | 60,000       | 55,000           |
| `gt`                   | 53,000       | 56,000           |
| `le`                   | 53,000       | 54,000           |
| `lt`                   | 51,000       | 56,000           |
| `min`                  | 86,000       | 111,000          |
| `max`                  | 81,000       | 111,000          |
| `neg`                  | -            | 72,000           |
| `not`                  | -            | 8,000            |
| `select`               | -            | 43,000           |
| `randEuint8()`         | -            | 100,000          |

#### **16-bit Encrypted integers (`euint16`)**

| Function name          | HCU (scalar) | HCU (non-scalar) |
| ---------------------- | ------------ | ---------------- |
| `add`                  | 87,000       | 87,000           |
| `sub`                  | 86,000       | 88,000           |
| `mul`                  | 176,000      | 207,000          |
| `div`                  | 283,000      | -                |
| `rem`                  | 513,000      | -                |
| `and`                  | 29,000       | 29,000           |
| `or`                   | 29,000       | 29,000           |
| `xor`                  | 29,000       | 29,000           |
| `shr`                  | 29,000       | 118,000          |
| `shl`                  | 29,000       | 118,000          |
| `rotr`                 | 30,000       | 117,000          |
| `rotl`                 | 29,000       | 117,000          |
| `eq`                   | 52,000       | 78,000           |
| `ne`                   | 51,000       | 82,000           |
| `ge`                   | 60,000       | 80,000           |
| `gt`                   | 53,000       | 83,000           |
| `le`                   | 54,000       | 80,000           |
| `lt`                   | 53,000       | 80,000           |
| `min`                  | 86,000       | 141,000          |
| `max`                  | 83,000       | 140,000          |
| `neg`                  | -            | 89,000           |
| `not`                  | -            | 15,000           |
| `select`               | -            | 44,000           |
| `randEuint16()`        | -            | 100,000          |

#### **32-bit Encrypted Integers (`euint32`)**

| Function name          | HCU (scalar) | HCU (non-scalar) |
| ---------------------- | ------------ | ---------------- |
| `add`                  | 87,000       | 121,000          |
| `sub`                  | 87,000       | 120,000          |
| `mul`                  | 244,000      | 313,000          |
| `div`                  | 397,000      | -                |
| `rem`                  | 714,000      | -                |
| `and`                  | 29,000       | 30,000           |
| `or`                   | 30,000       | 31,000           |
| `xor`                  | 30,000       | 30,000           |
| `shr`                  | 30,000       | 150,000          |
| `shl`                  | 30,000       | 150,000          |
| `rotr`                 | 30,000       | 149,000          |
| `rotl`                 | 30,000       | 150,000          |
| `eq`                   | 81,000       | 82,000           |
| `ne`                   | 80,000       | 84,000           |
| `ge`                   | 81,000       | 111,000          |
| `gt`                   | 82,000       | 111,000          |
| `le`                   | 80,000       | 113,000          |
| `lt`                   | 80,000       | 111,000          |
| `min`                  | 113,000      | 177,000          |
| `max`                  | 112,000      | 174,000          |
| `neg`                  | -            | 116,000          |
| `not`                  | -            | 28,000           |
| `select`               | -            | 45,000           |
| `randEuint32()`        | -            | 100,000          |

#### **64-bit Encrypted integers (`euint64`)**

| Function name          | HCU (scalar) | HCU (non-scalar) |
| ---------------------- | ------------ | ---------------- |
| `add`                  | 128,000      | 156,000          |
| `sub`                  | 129,000      | 159,000          |
| `mul`                  | 346,000      | 571,000          |
| `div`                  | 651,000      | -                |
| `rem`                  | 1,111,000    | -                |
| `and`                  | 33,000       | 33,000           |
| `or`                   | 32,000       | 33,000           |
| `xor`                  | 33,000       | 32,000           |
| `shr`                  | 34,000       | 203,000          |
| `shl`                  | 33,000       | 203,000          |
| `rotr`                 | 34,000       | 206,000          |
| `rotl`                 | 34,000       | 203,000          |
| `eq`                   | 83,000       | 116,000          |
| `ne`                   | 84,000       | 111,000          |
| `ge`                   | 112,000      | 146,000          |
| `gt`                   | 113,000      | 141,000          |
| `le`                   | 113,000      | 146,000          |
| `lt`                   | 113,000      | 142,000          |
| `min`                  | 149,000      | 210,000          |
| `max`                  | 147,000      | 211,000          |
| `neg`                  | -            | 150,000          |
| `not`                  | -            | 84,000           |
| `select`               | -            | 52,000           |
| `randEuint64()`        | -            | 100,000          |

#### **128-bit Encrypted integers (`euint128`)**

| Function name          | HCU (scalar) | HCU (non-scalar) |
| ---------------------- | ------------ | ---------------- |
| `add`                  | 159,000      | 249,000          |
| `sub`                  | 159,000      | 244,000          |
| `mul`                  | 646,000      | 1,671,000        |
| `div`                  | 1,290,000    | -                |
| `rem`                  | 1,900,000    | -                |
| `and`                  | 33,000       | 34,000           |
| `or`                   | 34,000       | 35,000           |
| `xor`                  | 35,000       | 35,000           |
| `shr`                  | 33,000       | 254,000          |
| `shl`                  | 33,000       | 251,000          |
| `rotr`                 | 34,000       | 261,000          |
| `rotl`                 | 33,000       | 264,000          |
| `eq`                   | 115,000      | 117,000          |
| `ne`                   | 115,000      | 116,000          |
| `ge`                   | 144,000      | 206,000          |
| `gt`                   | 144,000      | 206,000          |
| `le`                   | 143,000      | 204,000          |
| `lt`                   | 143,000      | 204,000          |
| `min`                  | 180,000      | 280,000          |
| `max`                  | 181,000      | 274,000          |
| `neg`                  | -            | 241,000          |
| `not`                  | -            | 109,000          |
| `select`               | -            | 51,000           |
| `randEuint128()`       | -            | 100,000          |

#### **256-bit Encrypted integers (`euint256`)**

| Function name          | HCU (scalar) | HCU (non-scalar) |
| ---------------------- | ------------ | ---------------- |
| `and`                  | 37,000       | 38,000           |
| `or`                   | 37,000       | 37,000           |
| `xor`                  | 37,000       | 37,000           |
| `shr`                  | 37,000       | 359,000          |
| `shl`                  | 37,000       | 359,000          |
| `rotr`                 | 37,000       | 367,000          |
| `rotl`                 | 37,000       | 367,000          |
| `eq`                   | 117,000      | 151,000          |
| `ne`                   | 117,000      | 149,000          |
| `neg`                  | -            | 269,000          |
| `not`                  | -            | 216,000          |
| `select`               | -            | 71,000           |
| `randEuint256()`       | -            | 100,000          |


#### Encrypted addresses (`euint160`)**

When using `eaddress` (internally represented as `euint160`), the HCU costs for equality and inequality checks are as follows:

| Function name | HCU (scalar) | HCU (non-scalar) |
| ------------- | ------------ | ---------------- |
| `eq`          | 115,000      | 125,000          |
| `ne`          | 115,000      | 124,000          |


## Additional Operations

| Function name               | HCU             |
| --------------------------- | --------------- |
| `cast`                      | 200             |
| `trivialEncrypt`            | 100-800         |
| `randBounded`               | 100,000         |
| `rand`                      | 100,000         |
| `select`                    | 43,000-71,000   |
