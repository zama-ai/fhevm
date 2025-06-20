# User decryption

This document explains how to perform user decryption. User decryption required when you want a user to access their
private data without it being exposed to the blockchain.

User decryption in FHEVM enables the secure sharing or reuse of encrypted data under a new public key without exposing
the plaintext. This feature is essential for scenarios where encrypted data must be transferred between contracts,
dApps, or users while maintaining its confidentiality.

{% hint style="info" %} Before implementing user decryption ensure you are familiar with the foundational concepts of
encryption, decryption, and computation. Refer to
[Encryption, Decryption, and Computation](../protocol/d_re_ecrypt_compute.md). {% endhint %}

## When to use user decryption

User decryption is particularly useful for **allowing individual users to securely access and decrypt their private
data**, such as balances or counters, while maintaining data confidentiality.

## Overview

The user decryption process involves retrieving ciphertext from the blockchain and performing user-decryption on the
client-side. In other words we take the data that has been encrypted by the KMS, decrypt it and encrypt it with the
users private key, so only he can access the information.

This ensures that the data remains encrypted under the blockchain’s FHE key but can be securely shared with a user by
re-encrypting it under the user’s NaCl public key.

User decryption is facilitated by the **Relayer** and the **Key Management System (KMS)**. The workflow consists of the
following:

1. Retrieving the ciphertext from the blockchain using a contract’s view function.
2. Re-encrypting the ciphertext client-side with the user’s public key, ensuring only the user can decrypt it.

## Step 1: retrieve the ciphertext

To retrieve the ciphertext that needs to be decrypted, you can implement a view function in your smart contract. Below
is an example implementation:

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

Here, `balanceOf` allows retrieval of the user’s encrypted balance stored on the blockchain.

## Step 2: decrypt the ciphertext

User decryption is performed client-side using the `@fhevm/sdk` library. [Refer to the guide](../../frontend/webapp.md)
to learn how to include `@fhevm/sdk` in your project.
