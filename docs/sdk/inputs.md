# Encrypted inputs

The library provides a set of functions to encrypt integers of various sizes (8, 16, and 32 bits) using the blockchain's public key. These encrypted integers can then be securely used as parameters for smart contract methods within the blockchain ecosystem.

## FhevmInstance.encrypt8

### Parameters

- `value: number` (required): A number between 0 and 255.

### Returns

- `Uint8Array`

### Example

```javascript
const instance = await createInstance({ chainId, publicKey });
const encryptedParam = instance.encrypt8(14);
```

## FhevmInstance.encrypt16

### Parameters

- `value: number` (required): A number between 0 and 65535.

### Returns

- `Uint8Array`

### Example

```javascript
const instance = await createInstance({ chainId, publicKey });
const encryptedParam = instance.encrypt16(1234);
```

## FhevmInstance.encrypt32

### Parameters

- `value: number` (required): A number between 0 and 4294967295.

### Returns

- `Uint8Array`

### Example

```javascript
const instance = await createInstance({ chainId, publicKey });
const encryptedParam = instance.encrypt32(94839304);
```
