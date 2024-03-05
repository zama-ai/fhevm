# Reencryption

## How it works

When user needs to access encrypted data, there are two solution for the smart contract developer:

- Return a decryption of this value. In this case, the becomes public and the confidentiality is compromised.
- Return a reencryption of this value. In this case, the user provides a public key, the value will be reencrypted from the blockchain's FHE public key to this public key.

A value reencryption is accessible through a view function. However, it's important to note that there's a potential vulnerability where `msg.sender` can be spoofed, and you cannot guarantee that the `msg.sender` truly represents the user.

## Using EIP-712

To prevent this issue, the smart contract can use the [EIP-712 standard](https://eips.ethereum.org/EIPS/eip-712). It describes how data is structured, hashed, and signed. By utilizing this data format and having it signed by the user, a secure process of reencrypting data becomes possible within a smart contract. The signed structure includes the necessary information, including the public key, which allows for seamless reencryption of the data.

1. The user provides the signature of the EIP-712 object and the public key
2. The contract verify the signature by generating the EIP-712 with the provided public key. The signature must match the `msg.sender`
3. If the signature match, the contract can return the reencryption of the value with the provided public key

`fhevmjs` provides a convenient function that generates this structured object and store user's signatures.

## Generate your typed data structure

You can generate the data to sign:

```javascript
const instance = getInstance();
const reencryption = instance.generatePublicKey("0x1c786b8ca49D932AFaDCEc00827352B503edf16c");
const params = [userAddress, JSON.stringify(reencryption.eip712)];
const sign = await window.ethereum.request({
  method: "eth_signTypedData_v4",
  params,
});

const encryptedBalance = await contract.balanceOf(reencryption.publicKey, sign);
```

## Decrypt a reencrypted value

When generating the public key using `generatePublicKey`, the corresponding private key is kept by the fhEVM instance and linked to the specified contract. To decrypt a value using the user's private key, you only need to provide the contract address and the encrypted value.

```javascript
const encryptedBalance = await contract.balanceOf(reencryption.publicKey, sign);
const balance = instance.decrypt("0x1c786b8ca49D932AFaDCEc00827352B503edf16c", encryptedBalance);
```

## Reuse previously signed public key

Upon the user's signing of the public key, it is advisable to keep the signature for potential use in other reencryption requests. The library offers the `setPublicKeySignature` method to link this signature with the contract's public key.

```javascript
const instance = getInstance();
const reencryption = instance.generatePublicKey("0x1c786b8ca49D932AFaDCEc00827352B503edf16c");
const params = [userAddress, JSON.stringify(reencryption.eip712)];
const sign = await window.ethereum.request({
  method: "eth_signTypedData_v4",
  params,
});

instance.setPublicKeySignature("0x1c786b8ca49D932AFaDCEc00827352B503edf16c", sign);
```

When the signature is saved, you can reuse it with `getPublicKey()`.

```javascript
const reencryption = instance.getPublicKey("0x1c786b8ca49D932AFaDCEc00827352B503edf16c");

const encryptedBalance = await contract.balanceOf(reencryption.publicKey, reencryption.signature);
```

You can merge these two functions to create one that returns both the public key and the signature. To check if a signed key pair already exists for a specific contract, you can use the `hasKeypair()` method provided by the instance.

```javascript
const getPublicKey = async (contractAddress) => {
  const instance = getInstance();
  if (!instance.hasKeypair(contractAddress)) {
    const reencryption = instance.generatePublicKey("0x1c786b8ca49D932AFaDCEc00827352B503edf16c");
    const params = [userAddress, JSON.stringify(reencryption.eip712)];
    const sign = await window.ethereum.request({
      method: "eth_signTypedData_v4",
      params,
    });
    instance.setPublicKeySignature(contractAddress, sign);
  }
  return instance.getPublicKey(contractAddress);
};

const reencryption = await getPublicKey(contractAddress);
const encryptedBalance = await contract.balanceOf(reencryption.publicKey, reencryption.signature);
```

Note: only signed keypairs are returned by `hasKeypair()` and `getPublicKey`.
