# Understanding `fhevmjs`

The `fhevmjs` library is a powerful tool for integrating Fully Homomorphic Encryption (FHE) into Ethereum-based applications. This guide will walk you through the initialization and usage of `fhevmjs`, focusing on the key components and abstractions used in a typical setup.

## Key Components

### Initialization

The `fhevmjs` library must be initialized before use. This is done using the `initFhevm` function, which loads necessary resources such as WebAssembly modules.

```javascript
import { initFhevm } from 'fhevmjs/bundle';

export const init = initFhevm;
```

### Creating an Instance

An instance of `fhevmjs` is created using the `createInstance` function. This instance is configured with various parameters, including network details and contract addresses.

```javascript
import { createInstance, FhevmInstance } from 'fhevmjs/bundle';

export const createFhevmInstance = async () => {
  const instance = await createInstance({
    network: window.ethereum,
    aclContractAddress: import.meta.env.VITE_ACL_ADDRESS,
    kmsContractAddress: import.meta.env.VITE_KMS_ADDRESS,
    gatewayUrl: import.meta.env.VITE_GATEWAY_URL,
  });
  return instance;
};
```

## Understanding Encryption

Encryption is a critical part of using `fhevmjs`, allowing you to securely handle data on the blockchain. The `useEncrypt` hook simplifies this process by managing the encryption state and logic.

### Using the `useEncrypt` Hook

The `useEncrypt` hook provides a straightforward way to encrypt data for use in smart contracts. It manages the encryption process and returns the encrypted data.

```javascript
import { useEncrypt } from '@/hooks/useEncrypt';

const { encryptAmount, isEncrypting, encryptedAmount, resetEncrypt } = useEncrypt();

const handleEncrypt = async () => {
  await encryptAmount('0xContractAddress', '0xUserAddress', 100n);
  console.log('Encrypted Amount:', encryptedAmount);
};
```

### Hook Details

- **`encryptAmount`**: Initiates the encryption process for a given amount, contract address, and user address.
- **`isEncrypting`**: Indicates whether the encryption process is currently running.
- **`encryptedAmount`**: Contains the result of the encryption process.
- **`resetEncrypt`**: Resets the encryption state.

## Understanding Decryption

Decryption allows you to retrieve the original data from its encrypted form. The `useDecryptValue` hook facilitates this by handling the decryption logic and state.

### Using the `useDecryptValue` Hook

The `useDecryptValue` hook provides a mechanism to decrypt data, returning the original value.

```javascript
import { useDecryptValue } from '@/hooks/useDecryptValue';

const { decryptedValue, isDecrypting, decrypt, error } = useDecryptValue({ signer });

const handleDecrypt = async () => {
  await decrypt(123456789n, '0xContractAddress');
  console.log('Decrypted Value:', decryptedValue);
};
```

### Hook Details

- **`decrypt`**: Initiates the decryption process for a given handle and contract address.
- **`isDecrypting`**: Indicates whether the decryption process is currently running.
- **`decryptedValue`**: Contains the result of the decryption process.
- **`error`**: Captures any errors that occur during decryption.

## Using `FhevmProvider`

The `FhevmProvider` component abstracts the initialization and instance management of `fhevmjs`, making it easier to integrate into React applications.

In your main application file, such as `App.tsx`, wrap your components with the `FhevmProvider` to ensure `fhevmjs` is properly initialized and available throughout your app.

```ts
function App() {
  return (
    <FhevmProvider>
      <YourAppComponents />
    </FhevmProvider>
  );
}
```

## Conclusion

The `fhevmjs` library provides a robust framework for integrating FHE into Ethereum applications. By following this guide, you can effectively initialize and manage `fhevmjs` instances, handle keypairs, and integrate the library into your React applications using the `FhevmProvider`. Additionally, the `useEncrypt` and `useDecryptValue` hooks simplify the encryption and decryption processes, making it easier to work with encrypted data in your applications.
