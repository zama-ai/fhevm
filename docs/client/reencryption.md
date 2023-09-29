# Reencryption

The library provides a convenient function that generates a JSON object based on the EIP-712 standard. This JSON object includes a public key and is specifically designed to facilitate data reencryption in a smart contract environment.

By utilizing this JSON object and having it signed by the user, a secure process of reencrypting data becomes possible within a smart contract. The signed JSON includes the necessary information, including the public key, which allows for seamless reencryption of the data.

## FhevmInstance.generateToken

### Parameters

- `options` (required):
  - `verifyingContract: string` (required): The address of the contract
  - `name: string` (optional): The name used in the EIP712
  - `version: string` (optional): The version used in the EIP712

### Returns

- `FhevmToken`

```typescript
{
  keypair: {
    publicKey: Uint8Array;
    privateKey: Uint8Array;
  }
  token: EIP712;
}
```

### Example

```javascript
const instance = await createInstance({ chainId, publicKey });
const encryptedParam = instance.generateToken({
  name: "Authentication",
  verifyingContract: "0x1c786b8ca49D932AFaDCEc00827352B503edf16c",
});
const params = [userAddress, JSON.stringify(generatedToken.token)];
const sign = await window.ethereum.request({
  method: "eth_signTypedData_v4",
  params,
});
```

## FhevmInstance.decrypt

### Parameters

- `contractAddress: string` (required): address of the contract
- `ciphertext: Uint8Array | string` (required): a ciphertext, as a binary or hex string, to decrypt

### Returns

- `number`

### Example

```javascript
const instance = await createInstance({ chainId, publicKey });
const token = instance.generateToken({
  name: 'Authentication',
  verifyingContract: '0x1c786b8ca49D932AFaDCEc00827352B503edf16c',
});
...
const response = await contract.balanceOf(token.publicKey, sign);
instance.decrypt('0x1c786b8ca49D932AFaDCEc00827352B503edf16c', response)

```

## FhevmInstance.setTokenSignature

This method allows you to store the signature of a public key for a specific contract, in order to retrieve it later. The signature is also serialized in `serializeKeypairs`.

### Parameters

- `contractAddress: string` (required): address of the contract
- `signature: string` (required): the signature of the EIP-712 token

### Example

```javascript
const instance = await createInstance({ chainId, publicKey });
const generatedToken = instance.generateToken({
  name: "Authentication",
  verifyingContract: "0x1c786b8ca49D932AFaDCEc00827352B503edf16c",
});

// Ask for user to sign the token
const params = [userAddress, JSON.stringify(generatedToken.token)];
const sign = await window.ethereum.request({
  method: "eth_signTypedData_v4",
  params,
});
// Store signature
instance.setTokenSignature(contractAddress, sign);
```

## FhevmInstance.hasKeypair

This method returns true if contract has a keypair and a signature.

### Parameters

- `contractAddress: string` (required): address of the contract

### Returns

- `boolean`

### Example

```javascript
const contractAddress = '0x1c786b8ca49D932AFaDCEc00827352B503edf16c';
const instance = await createInstance({ chainId, publicKey });
const generatedToken = instance.generateToken({
  name: 'Authentication',
  verifyingContract: ,
});

// Ask for user to sign the token
const params = [userAddress, JSON.stringify(generatedToken.token)];
const sign = await window.ethereum.request({ method: 'eth_signTypedData_v4', params });

console.log(instance.hasKeypair(contractAddress)); // false

// Store signature
instance.setTokenSignature(contractAddress, sign);

console.log(instance.hasKeypair(contractAddress)); // true
```

## FhevmInstance.getTokenSignature

This method returns the saved public key and signature for a specific contract. If the contract has no keypair or no signature, this method returns `null`.

### Parameters

- `contractAddress: string` (required): address of the contract

### Returns

- `TokenSignature` or `null`

```typescript
{ publicKey: Uint8Array; signature: string; } | null
```

### Example

```javascript
const contractAddress = "0x1c786b8ca49D932AFaDCEc00827352B503edf16c";
const instance = await createInstance({ chainId, publicKey });
const generatedToken = instance.generateToken({
  name: "Authentication",
  verifyingContract: contractAddress,
});

// Ask for user to sign the token
const params = [userAddress, JSON.stringify(generatedToken.token)];
const sign = await window.ethereum.request({
  method: "eth_signTypedData_v4",
  params,
});
// Store signature
instance.setTokenSignature(contractAddress, sign);

// Now, since the signature is stored, we can fetch the public key and signature later
const { publicKey, signature } = instance.getTokenSignature();
console.log(publicKey); // Uint8Array(32) [192, 108, 9, ...]
console.log(signature); // '0x6214e232b2dae4d8d2c99837dd1af004e1b...'

const response = await contract.balanceOf(publicKey, signature);
instance.decrypt(contractAddress, response);
```

## FhevmInstance.serializeKeypairs

This method is useful if you want to store contract keypairs in the user LocalStorage.

### Returns

- `ExportedContractKeypairs`:

```typescript
{
  [contractAddress: string]: {
    publicKey: string;
    privateKey: string;
    signature: string;
  }
}
```

### Example

```javascript
const keypairs = instance.serializeKeypairs();
console.log(keypairs);
// {
//    '0x1c786b8ca49D932AFaDCEc00827352B503edf16c': {
//      signature: '0x6214e232b2dae4d8d2c99837dd1af0...',
//      publicKey: '7b2352b10cb4e379fc89094c445acb8b2161ec23a3694c309e01e797ab2bae22',
//      privateKey: '764d194c6c686164fa5eb3c53ef3f7f5b90985723f19e865baf0961dd28991eb',
//    }
// }
```
