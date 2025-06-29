# asEbool, asEuintXX, and asEaddress operations

This documentation covers the `asEbool`, `asEuintXX`, and `asEaddress` operations provided by the FHE library for working with encrypted data in the FHEVM. These operations are essential for converting between plaintext and encrypted types, as well as handling encrypted inputs.

The operations can be categorized into three main use cases:

1. **Trivial encryption**: Converting plaintext values to encrypted types
2. **Type casting**: Converting between different encrypted types
3. **Input handling**: Processing encrypted inputs with proofs

## 1. Trivial encryption

Trivial encryption simply put is a plain text in a format of a ciphertext.

### Overview

Trivial encryption is the process of converting plaintext values into encrypted types (ciphertexts) compatible with FHE operators. Although the data is in ciphertext format, it remains publicly visible on-chain, making it useful for operations between public and private values.

This type of casting involves converting plaintext (unencrypted) values into their encrypted equivalents, such as:

- `bool` → `ebool`
- `uint` → `euintXX`
- `address` → `eaddress`

> **Note**: When doing trivial encryption, the data is made compatible with FHE operations but remains publicly visible on-chain unless explicitly encrypted.

#### **Example**

```solidity
euint64 value64 = FHE.asEuint64(7262);  // Trivial encrypt a uint64
ebool valueBool = FHE.asEbool(true);   // Trivial encrypt a boolean
```

## 2. Casting between encrypted types

This type of casting is used to reinterpret or convert one encrypted type into another. For example:

- `euint32` → `euint64`

Casting between encrypted types is often required when working with operations that demand specific sizes or precisions.

> **Important**: When casting between encrypted types:
>
> - Casting from smaller types to larger types (e.g. `euint32` → `euint64`) preserves all information
> - Casting from larger types to smaller types (e.g. `euint64` → `euint32`) will truncate and lose information

The table below summarizes the available casting functions:

| From type | To type  | Function        |
| --------- | -------- | --------------- |
| `euintX`  | `euintX` | `FHE.asEuintXX` |
| `ebool`   | `euintX` | `FHE.asEuintXX` |
| `euintX`  | `ebool`  | `FHE.asEboolXX` |

{% hint style="info" %} 
Casting between encrypted types is efficient and often necessary when handling data with differing precision requirements. 
{% endhint %}

### **Workflow for encrypted types**

```solidity
// Casting between encrypted types
euint32 value32 = FHE.asEuint32(value64); // Cast to euint32
ebool valueBool = FHE.asEbool(value32);   // Cast to ebool
```

## 3. Encrypted input

### Overview

Encrypted input casting is the process of interpreting a handle (ciphertext reference) and its proof as a specific encrypted type. This ensures the validity of the input before it is used in computations.

Encrypted inputs is in depth explained in the following section: [encrypted inputs](./inputs.md)

#### Example

```solidity
euint64 encryptedValue = FHE.asEuint64(einputHandle, inputProof); // einputHandle as of type externalEuint64
```

#### Details

Encrypted input casting validates:

1.  The input handle references a valid ciphertext.
2.  The accompanying proof matches the expected type.

For more information, see the [Encrypetd inputs documentation](./inputs.md)

## Overall operation summary

| Casting Type             | Function               | Input Type                        | Output Type |
| ------------------------ | ---------------------- | --------------------------------- | ----------- |
| Trivial encryption       | `FHE.asEuintXX(x)`     | `uintX`                           | `euintX`    |
|                          | `FHE.asEbool(x)`       | `bool`                            | `ebool`     |
|                          | `FHE.asEaddress(x)`    | `address`                         | `eaddress`  |
| Conversion between types | `FHE.asEuintXX(x)`     | `euintXX`/`ebool`                 | `euintYY`   |
|                          | `FHE.asEbool(x)`       | `euintXX`                         | `ebool`     |
| Encrypted input          | `FHE.asEuintXX(x, y)`  | `externalEuintXX`, `bytes` proof  | `euintX`    |
|                          | `FHE.asEbool(x, y)`    | `externalEbool`,`bytes` proof     | `ebool`     |
|                          | `FHE.asEaddress(x, y)` | `externalEaddress`, `bytes` proof | `eaddress`  |
