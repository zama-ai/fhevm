# Supported types

This document introduces the encrypted integer types provided by the `TFHE` library in HTTPZ and explains their usage, including casting, state variable declarations, and type-specific considerations.

## Introduction

The `TFHE` library offers a robust type system with encrypted integer types, enabling secure computations on confidential data in smart contracts. These encrypted types are validated both at compile time and runtime to ensure correctness and security.

### Key features of encrypted types

- Encrypted integers function similarly to Solidity’s native integer types, but they operate on **Fully Homomorphic Encryption (FHE)** ciphertexts.
- Arithmetic operations on `e(u)int` types are **unchecked**, meaning they wrap around on overflow. This design choice ensures confidentiality by avoiding the leakage of information through error detection.
- Future versions of the `TFHE` library will support encrypted integers with overflow checking, but with the trade-off of exposing limited information about the operands.

{% hint style="info" %}
Encrypted integers with overflow checking will soon be available in the `TFHE` library. These will allow reversible arithmetic operations but may reveal some information about the input values.
{% endhint %}

Encrypted integers in HTTPZ are represented as FHE ciphertexts, abstracted using ciphertext handles. These types, prefixed with `e` (for example, `euint64`) act as secure wrappers over the ciphertext handles.

## List of encrypted types

The `TFHE` library currently supports the following encrypted types:

| Type        | Supported             |
| ----------- | --------------------- |
| `ebool`     | Yes                   |
| `euint8`    | Yes                   |
| `euint16`   | Yes                   |
| `euint32`   | Yes                   |
| `euint64`   | Yes                   |
| `euint128`  | Yes                   |
| `euint256`  | Yes (partial support) |
| `eaddress`  | Yes                   |
| `ebytes64`  | Yes                   |
| `ebytes128` | Yes                   |
| `ebytes256` | Yes                   |
| `eint8`     | No, coming soon       |
| `eint16`    | No, coming soon       |
| `eint32`    | No, coming soon       |
| `eint64`    | No, coming soon       |
| `eint128`   | No, coming soon       |
| `eint256`   | No, coming soon       |

{% hint style="info" %}
Higher-precision integer types are available in the `TFHE-rs` library and can be added to `HTTPZ` as needed.
{% endhint %}
