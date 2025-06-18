# Supported types

This document introduces the encrypted integer types provided by the `FHE` library in FHEVM and explains their usage,
including casting, state variable declarations, and type-specific considerations.

## Introduction

The `FHE` library offers a robust type system with encrypted integer types, enabling secure computations on confidential
data in smart contracts. These encrypted types are validated both at compile time and runtime to ensure correctness and
security.

### Key features of encrypted types

- Encrypted integers function similarly to Solidity’s native integer types, but they operate on **Fully Homomorphic
  Encryption (FHE)** ciphertexts.
- Arithmetic operations on `e(u)int` types are **unchecked**, meaning they wrap around on overflow. This design choice
  ensures confidentiality by avoiding the leakage of information through error detection.
- Future versions of the `FHE` library will support encrypted integers with overflow checking, but with the trade-off of
  exposing limited information about the operands.

{% hint style="info" %} Encrypted integers with overflow checking will soon be available in the `FHE` library. These
will allow reversible arithmetic operations but may reveal some information about the input values. {% endhint %}

Encrypted integers in FHEVM are represented as FHE ciphertexts, abstracted using ciphertext handles. These types,
prefixed with `e` (for example, `euint64`) act as secure wrappers over the ciphertext handles.

## List of encrypted types

The `FHE` library currently supports the following encrypted types:

| Type     | Bit Length | Supported Operators                                                                                                                | Aliases (with supported operators) |
| -------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| Ebool    | 2          | and, or, xor, eq, ne, not, select, rand                                                                                            |                                    |
| Euint8   | 8          | add, sub, mul, div, rem, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, min, max, neg, not, select, rand, randBounded |                                    |
| Euint16  | 16         | add, sub, mul, div, rem, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, min, max, neg, not, select, rand, randBounded |                                    |
| Euint32  | 32         | add, sub, mul, div, rem, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, min, max, neg, not, select, rand, randBounded |                                    |
| Euint64  | 64         | add, sub, mul, div, rem, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, min, max, neg, not, select, rand, randBounded |                                    |
| Euint128 | 128        | add, sub, mul, div, rem, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, min, max, neg, not, select, rand, randBounded |                                    |
| Euint160 | 160        |                                                                                                                                    | Eaddress (eq, ne, select)          |
| Euint256 | 256        | and, or, xor, shl, shr, rotl, rotr, eq, ne, neg, not, select, rand, randBounded                                                    |                                    |

> **Note:**  
> Division (`div`) and remainder (`rem`) operations are only supported when the right-hand side (`rhs`) operand is a
> plaintext (non-encrypted) value. Attempting to use an encrypted value as `rhs` will result in a panic. This
> restriction ensures correct and secure computation within the current framework.

{% hint style="info" %} Higher-precision integer types are available in the `TFHE-rs` library and can be added to
`fhevm` as needed. {% endhint %}
