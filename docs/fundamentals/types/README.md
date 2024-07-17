# Use encrypted types
This document explains how to implement and manage encrypted integer types in smart contracts using `TFHE` library.

## Introduction
The `TFHE` library provides encrypted integer types and a type system that is checked both at compile time and at run time.

Encrypted integers function similarly to Solidity's integer types. However, features like "revert on overflow" are not supported, as this would expose certain information about the encrypted value. Therefore, arithmetic on `e(u)int`` types is [unchecked](https://docs.soliditylang.org/en/latest/control-structures.html#checked-or-unchecked-arithmetic), which means overflow will wrap around.

{% hint style="info" %}
Encrypted integers with overflow checking will be available soon in the `TFHE` library. They will allow reversal in case of an overflow, but will reveal some information about the operands.
{% endhint %}

In fhEVM, encrypted integers are implemented as FHE ciphertexts. The `TFHE` library abstracts this, presenting ciphertext handles to smart contract developers. The `e(u)int` types act as **wrappers** over these handles.

## List of encrypted types

The following encrypted data types are defined:

| type        | supported       |
| ----------- | --------------- |
| `ebool`     | yes             |
| `euint4`    | yes             |
| `euint8`    | yes             |
| `euint16`   | yes             |
| `euint32`   | yes             |
| `euint64`   | yes             |
| `euint128`  | no, coming soon |
| `euint256`  | no, coming soon |
| `eaddress`  | yes             |
| `ebytes64`  | no, coming soon |
| `ebytes128` | no, coming soon |
| `ebytes256` | yes             |
| `eint8`     | no, coming soon |
| `eint16`    | no, coming soon |
| `eint32`    | no, coming soon |
| `eint64`    | no, coming soon |
| `eint128`   | no, coming soon |
| `eint256`   | no, coming soon |

Higher-precision integers are supported in the `TFHE-rs` library and can be added as needed to `fhEVM`.

## Casting

You can cast types with `asEuint`/`asEbool` methods:

```solidity
euint64 value64 = TFHE.asEuint64(7262);
euint32 value32 = TFHE.asEuint32(value64);
ebool valueBool = TFHE.asEbool(value32);
```

## Contracting state variables
When using encrypted types for state variables, you cannot use the `immutable` or `constant` keywords. This is because the compiler attempts to resolve the value of T`FHE.asEuintXX(yy)` during compilation, which is not feasible because `asEuintXX()` calls a precompiled contract. 

To handle this, do not declare your encrypted state variables as `immutabl`e or `constant`. Instead, use the following methods to set your variables:


```solidity
euint64 private totalSupply = TFHE.asEuint64(0);
```

```solidity
euint64 private totalSupply;
constructor() {
  totalSupply = TFHE.asEuint64(0);
}
```
