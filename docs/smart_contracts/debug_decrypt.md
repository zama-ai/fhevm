# Debugging with `debug.decrypt[XX]`

This guide explains how to use the `debug.decrypt[XX]` functions for debugging encrypted data in mocked environments during development with HTTPZ.

{% hint style="warning" %}
The `debug.decrypt[XX]` functions should not be used in production as they rely on private keys.
{% endhint %}

## Overview

The `debug.decrypt[XX]` functions allow you to decrypt encrypted handles into plaintext values. This feature is useful for debugging encrypted operations such as transfers, balance checks, and other computations involving FHE-encrypted data.

### Key points

- **Environment**: The `debug.decrypt[XX]` functions work **only in mocked environments** (e.g., `hardhat` network).
- **Production limitation**: In production, decryption is performed asynchronously via the Gateway and requires an authorized onchain request.
- **Encrypted types**: The `debug.decrypt[XX]` functions supports various encrypted types, including integers, booleans, and `ebytesXX`.
- **Bypass ACL authorization**: The `debug.decrypt[XX]` functions allow decryption without ACL authorization, useful for verifying encrypted operations during development and testing.

## Supported functions

### Integer decryption

Decrypts encrypted integers of different bit-widths (`euint8`, `euint16`, ..., `euint256`).

| Function Name | Returns  | Encrypted Type |
| ------------- | -------- | -------------- |
| `decrypt8`    | `bigint` | `euint8`       |
| `decrypt16`   | `bigint` | `euint16`      |
| `decrypt32`   | `bigint` | `euint32`      |
| `decrypt64`   | `bigint` | `euint64`      |
| `decrypt128`  | `bigint` | `euint128`     |
| `decrypt256`  | `bigint` | `euint256`     |

### Boolean decryption

Decrypts encrypted booleans (`ebool`).

| Function Name | Returns   | Encrypted Type |
| ------------- | --------- | -------------- |
| `decryptBool` | `boolean` | `ebool`        |

### Byte array decryption

Decrypts encrypted byte arrays of various sizes (`ebytesXX`).

| Function Name      | Returns  | Encrypted Type |
| ------------------ | -------- | -------------- |
| `decryptEbytes64`  | `string` | `ebytes64`     |
| `decryptEbytes128` | `string` | `ebytes128`    |
| `decryptEbytes256` | `string` | `ebytes256`    |

### Address decryption

Decrypts encrypted addresses.

| Function Name    | Returns  | Encrypted Type |
| ---------------- | -------- | -------------- |
| `decryptAddress` | `string` | `eaddress`     |

## Function usage

### Example: decrypting encrypted values

```typescript
import { debug } from "../utils";

// Decrypt a 64-bit encrypted integer
const handle64: bigint = await this.erc20.balanceOf(this.signers.alice);
const plaintextValue: bigint = await debug.decrypt64(handle64);
console.log("Decrypted Balance:", plaintextValue);
```

{% hint style="info" %}
To utilize the debug functions, import the [utils.ts](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/test/utils.ts) file.
{% endhint %}

For a more complete example, refer to the [ConfidentialERC20 test file](https://github.com/zama-ai/fhevm-hardhat-template/blob/f9505a67db31c988f49b6f4210df47ca3ce97841/test/confidentialERC20/ConfidentialERC20.ts#L181-L205).

### Example: decrypting byte arrays

```typescript
// Decrypt a 128-byte encrypted value
const ebytes128Handle: bigint = ...; // Get handle for the encrypted bytes
const decryptedBytes: string = await debug.decryptEbytes128(ebytes128Handle);
console.log("Decrypted Bytes:", decryptedBytes);
```

## **How it works**

### Verifying types

Each decryption function includes a **type verification step** to ensure the provided handle matches the expected encrypted type. If the type is mismatched, an error is thrown.

```typescript
function verifyType(handle: bigint, expectedType: number) {
  const typeCt = handle >> 8n;
  if (Number(typeCt % 256n) !== expectedType) {
    throw "Wrong encrypted type for the handle";
  }
}
```

### Environment checks

{% hint style="danger" %}
The functions only work in the `hardhat` network. Attempting to use them in a production environment will result in an error.
{% endhint %}

```typescript
if (network.name !== "hardhat") {
  throw Error("This function can only be called in mocked mode");
}
```

## **Best practices**

- **Use only for debugging**: These functions require access to private keys and are meant exclusively for local testing and debugging.
- **Production decryption**: For production, always use the asynchronous Gateway-based decryption.
