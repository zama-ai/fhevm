# Supported types

This document introduces the encrypted integer types provided by the `FHE` library in FHEVM and explains their usage, including casting, state variable declarations, and type-specific considerations.

## Introduction

The `FHE` library offers a robust type system with encrypted integer types, enabling secure computations on confidential data in smart contracts. These encrypted types are validated both at compile time and runtime to ensure correctness and security.

### Key features of encrypted types

- Encrypted integers function similarly to Solidity’s native integer types, but they operate on **Fully Homomorphic Encryption (FHE)** ciphertexts.
- Arithmetic operations on `e(u)int` types are **unchecked**, meaning they wrap around on overflow. This design choice ensures confidentiality by avoiding the leakage of information through error detection.
- There is no checked arithmetic: detecting an overflow would reveal information about the operands. When an overflow must be handled, test for it with a comparison and neutralise it with `FHE.select` (see [Operator semantics](operations/semantics.md#arithmetic)).

Encrypted integers in FHEVM are represented as FHE ciphertexts, abstracted using ciphertext handles. These types, prefixed with `e` (for example, `euint64`) act as secure wrappers over the ciphertext handles.

## List of encrypted types

The `FHE` library currently supports the following encrypted types:

| Type     | Bit Length | Supported Operators                                                                                                                | Aliases (with supported operators) |
| -------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| Ebool    | 2          | and, or, xor, eq, ne, not, select, rand                                                                                            |                                    |
| Euint8   | 8          | add, sub, mul, div, rem, mulDiv, sum, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, isIn, min, max, neg, not, select, rand, randBounded |                                    |
| Euint16  | 16         | add, sub, mul, div, rem, mulDiv, sum, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, isIn, min, max, neg, not, select, rand, randBounded |                                    |
| Euint32  | 32         | add, sub, mul, div, rem, mulDiv, sum, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, isIn, min, max, neg, not, select, rand, randBounded |                                    |
| Euint64  | 64         | add, sub, mul, div, rem, mulDiv, sum, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, isIn, min, max, neg, not, select, rand, randBounded |                                    |
| Euint128 | 128        | add, sub, mul, div, rem, sum, and, or, xor, shl, shr, rotl, rotr, eq, ne, ge, gt, le, lt, isIn, min, max, neg, not, select, rand, randBounded |                                    |
| Euint160 | 160        | eq, ne, isIn, select                                                                                                               | Eaddress — `eaddress` is an alias for `euint160`, used for encrypted Ethereum addresses |
| Euint256 | 256        | and, or, xor, shl, shr, rotl, rotr, eq, ne, isIn, neg, not, select, rand, randBounded                                              |                                    |

{% hint style="info" %}  
Division (`div`), remainder (`rem`) and the divisor of `mulDiv` are only supported with a plaintext (non-encrypted) right-hand side: there is no overload taking an encrypted divisor, so such code does not compile. A plaintext divisor equal to zero reverts on-chain with `DivisionByZero`.
{% endhint %}

{% hint style="info" %}
Higher-precision integer types are available in the `TFHE-rs` library and can be added to `fhevm` as needed.
{% endhint %}

The exact behaviour of each operator (wrapping, shifts, casts, size limits) is specified in [Operator semantics](operations/semantics.md).
