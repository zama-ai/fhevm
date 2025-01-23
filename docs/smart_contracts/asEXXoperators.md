# asEbool, asEuintXX, asEaddress and asEbytesXX operations

This documentation covers the `asEbool`, `asEuintXX`, `asEaddress` and `asEbytesXX` operations provided by the TFHE library for working with encrypted data in the fhEVM. These operations are essential for converting between plaintext and encrypted types, as well as handling encrypted inputs.

The operations can be categorized into three main use cases:

1. **Trivial encryption**: Converting plaintext values to encrypted types
2. **Type casting**: Converting between different encrypted types
3. **Input handling**: Processing encrypted inputs with proofs

## 1. Trivial encryption

Trivial encryption simply put is a plain text in a format of a ciphertext.

### Overview

Trivial encryption is the process of converting plaintext values into encrypted types (ciphertexts) compatible with TFHE operators. Although the data is in ciphertext format, it remains publicly visible on-chain, making it useful for operations between public and private values.

This type of casting involves converting plaintext (unencrypted) values into their encrypted equivalents, such as:

- `bool` → `ebool`
- `uint` → `euintXX`
- `bytes` → `ebytesXX`
- `address` → `eaddress`

> **Note**: When doing trivial encryption, the data is made compatible with FHE operations but remains publicly visible on-chain unless explicitly encrypted.

#### **Example**

```solidity
euint64 value64 = TFHE.asEuint64(7262);  // Trivial encrypt a uint64
ebool valueBool = TFHE.asEbool(true);   // Trivial encrypt a boolean
```

### Trivial encryption of `ebytesXX` types

The `TFHE.padToBytesXX` functions facilitate this trivial encryption process for byte arrays, ensuring compatibility with `ebytesXX` types. These functions:

- Pad the provided byte array to the appropriate length (`64`, `128`, or `256` bytes).
- Prevent runtime errors caused by improperly sized input data.
- Work seamlessly with `TFHE.asEbytesXX` for trivial encryption.

> **Important**: Trivial encryption does NOT provide any privacy guarantees. The input data remains fully visible on the blockchain. Only use trivial encryption when working with public values that need to interact with actual encrypted data.

#### Workflow

1. **Pad Input Data**:
   Use the `padToBytesXX` functions to ensure your byte array matches the size requirements.
2. **Encrypt the Padded Data**:
   Use `TFHE.asEbytesXX` to encrypt the padded byte array into the corresponding encrypted type.
3. **Grant Access**:
   Use `TFHE.allowThis` and `TFHE.allow`optionally, if you want to persist allowance for those variables for later use.

### Example: Trivial Encryption with `ebytesXX`

Below is an example demonstrating how to encrypt and manage `ebytes64`, `ebytes128`, and `ebytes256` types:

```solidity
function trivialEncrypt() public {
  // Encrypt a 64-byte array
  ebytes64 yBytes64 = TFHE.asEbytes64(
    TFHE.padToBytes64(
      hex"19d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc5"
    )
  );
  TFHE.allowThis(yBytes64);
  TFHE.allow(yBytes64, msg.sender);

  // Encrypt a 128-byte array
  ebytes128 yBytes128 = TFHE.asEbytes128(
    TFHE.padToBytes128(
      hex"13e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
    )
  );
  TFHE.allowThis(yBytes128);
  TFHE.allow(yBytes128, msg.sender);

  // Encrypt a 256-byte array
  ebytes256 yBytes256 = TFHE.asEbytes256(
    TFHE.padToBytes256(
      hex"d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc513e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
    )
  );
  TFHE.allowThis(yBytes256);
  TFHE.allow(yBytes256, msg.sender);
}
```

## 2. Casting between encrypted types

This type of casting is used to reinterpret or convert one encrypted type into another. For example:

- `euint32` → `euint64`
- `ebytes128` → `ebytes256`

Casting between encrypted types is often required when working with operations that demand specific sizes or precisions.

> **Important**: When casting between encrypted types:
>
> - Casting from smaller types to larger types (e.g. `euint32` → `euint64`) preserves all information
> - Casting from larger types to smaller types (e.g. `euint64` → `euint32`) will truncate and lose information

The table below summarizes the available casting functions:

| From type | To type  | Function         |
| --------- | -------- | ---------------- |
| `euintX`  | `euintX` | `TFHE.asEuintXX` |
| `ebool`   | `euintX` | `TFHE.asEuintXX` |
| `euintX`  | `ebool`  | `TFHE.asEboolXX` |

{% hint style="info" %}
Casting between encrypted types is efficient and often necessary when handling data with differing precision requirements.
{% endhint %}

### **Workflow for encrypted types**

```solidity
// Casting between encrypted types
euint32 value32 = TFHE.asEuint32(value64); // Cast to euint32
ebool valueBool = TFHE.asEbool(value32);   // Cast to ebool
```

## 3. Encrypted input

### Overview

Encrypted input casting is the process of interpreting a handle (ciphertext reference) and its proof as a specific encrypted type. This ensures the validity of the input before it is used in computations.

Encrypted inputs is in depth explained in the following section: [encrypted inputs](./inputs.md)

#### Example

```solidity
euint64 encryptedValue = TFHE.asEuint64(einputHandle, inputProof); // Interpret einputHandle as euint64
```

#### Details

Encrypted input casting validates:

1.  The input handle references a valid ciphertext.
2.  The accompanying proof matches the expected type.

For more information, see the [Encrypetd inputs documentation](./inputs.md)

## Overall operation summary

| Casting Type             | Function                | Input Type              | Output Type |
| ------------------------ | ----------------------- | ----------------------- | ----------- |
| Trivial encryption       | `TFHE.asEuintXX(x)`     | `uintX`                 | `euintX`    |
|                          | `TFHE.asEbool(x)`       | `bool`                  | `ebool`     |
|                          | `TFHE.asEbytesXX(x)`    | `bytesXX`               | `ebytesXX`  |
|                          | `TFHE.asEaddress(x)`    | `address`               | `eaddress`  |
| Conversion between types | `TFHE.asEuintXX(x)`     | `euintXX`/`ebool`       | `euintYY`   |
|                          | `TFHE.asEbool(x)`       | `euintXX`               | `ebool`     |
| Encrypted input          | `TFHE.asEuintXX(x, y)`  | `einput`, `bytes` proof | `euintX`    |
|                          | `TFHE.asEbool(x, y)`    | `einput`,`bytes` proof  | `ebool`     |
|                          | `TFHE.asEbytesXX(x, y)` | `einput`,`bytes` proof  | `ebytesXX`  |
|                          | `TFHE.asEaddress(x, y)` | `einput`, `bytes` proof | `eaddress`  |
