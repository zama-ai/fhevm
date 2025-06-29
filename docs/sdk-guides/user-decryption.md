# User decryption

This document explains how to perform user decryption. 
User decryption is required when you want a user to access their private data without it being exposed to the blockchain.

User decryption in FHEVM enables the secure sharing or reuse of encrypted data under a new public key without exposing the plaintext. 

This feature is essential for scenarios where encrypted data must be transferred between contracts, dApps, or users while maintaining its confidentiality.

{% hint style="info" %} 
Before implementing user decryption ensure you are familiar with the foundational concepts of encryption, decryption, and computation. Refer to [Encryption, Decryption, and Computation](../protocol/d_re_ecrypt_compute.md). 
{% endhint %}

## When to use user decryption

User decryption is particularly useful for **allowing individual users to securely access and decrypt their private data**, such as balances or counters, while maintaining data confidentiality.

## Overview

The user decryption process involves retrieving ciphertext from the blockchain and performing user-decryption on the client-side. In other words we take the data that has been encrypted by the KMS, decrypt it and encrypt it with the users private key, so only he can access the information.

This ensures that the data remains encrypted under the blockchain’s FHE key but can be securely shared with a user by re-encrypting it under the user’s NaCl public key.

User decryption is facilitated by the **Relayer** and the **Key Management System (KMS)**. The workflow consists of the following:

1. Retrieving the ciphertext from the blockchain using a contract’s view function.
2. Re-encrypting the ciphertext client-side with the user’s public key, ensuring only the user can decrypt it.

## Step 1: retrieve the ciphertext

To retrieve the ciphertext that needs to be decrypted, you can implement a view function in your smart contract. Below is an example implementation:

```solidity
import "@fhevm/solidity/lib/FHE.sol";

contract ConfidentialERC20 {
  ...
  function balanceOf(account address) public view returns (euint64) {
    return balances[msg.sender];
  }
  ...
}
```

Here, `balanceOf` allows retrieval of the user’s encrypted balance handle stored on the blockchain.
Doing this will return the ciphertext handle, an identifier for the underlying ciphertext.


{% hint style="warning" %}
For the user to be able to user decrypt (also called re-encrypt) the ciphertext value the access control (ACL) needs to be set properly using the `FHE.allow(ciphertext, address)` function in the solidity contract holding the ciphertext.
For more details on the topic please refer to [the ACL documentation](../solidity-guides/acl/README.md).
{% endhint %}

## Step 2: decrypt the ciphertext

Using that ciphertext handle user decryption is performed client-side using the `@zama-fhe/relayer-sdk` library.
The user needs to have created an instance object prior to that (for more context see [the relayer-sdk setup page](./initialization.md)).

```ts
// instance: [`FhevmInstance`] from `zama-fhe/relayer-sdk`
// signer: [`Signer`] from ethers (could a [`Wallet`])
// ciphertextHandle: [`string`]
// contractAddress: [`string`]

const keypair = instance.generateKeypair();
const handleContractPairs = [
  {
    handle: ciphertextHandle,
    contractAddress: contractAddress,
  },
];
const startTimeStamp = Math.floor(Date.now() / 1000).toString();
const durationDays = '10'; // String for consistency
const contractAddresses = [contractAddress];

const eip712 = instance.createEIP712(
  keypair.publicKey, 
  contractAddresses, 
  startTimeStamp, 
  durationDays
);

const signature = await signer.signTypedData(
  eip712.domain,
  {
    UserDecryptRequestVerification: eip712.types.UserDecryptRequestVerification,
  },
  eip712.message,
);

const result = await instance.userDecrypt(
  handleContractPairs,
  keypair.privateKey,
  keypair.publicKey,
  signature.replace('0x', ''),
  contractAddresses,
  signer.address,
  startTimeStamp,
  durationDays,
);

const decryptedValue = result[ciphertextHandle];
```
