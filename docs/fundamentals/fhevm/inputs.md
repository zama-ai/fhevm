# Inputs

When we talk about inputs, we refer to encrypted data the users send to the fhEVM-native blockchain. Data is in the form of FHE ciphertexts. An example would be the amount to be transferred when calling an ERC20 transfer function.

## ZKPoK
It is important that confidential data sent by users cannot be seen by anyone. Without measures, there are multiple ways that could happen, for example:
 * anyone decrypting the ciphertext
 * anyone doing arbitrary computations via the ciphertext (e.g. adding 0 to it), producing a new ciphertext that itself is decrypted (including malicious actors using ciphertexts of other users)
 * using the ciphertext in a malicious contract that leads to decryption

Furthermore, if users are allowed to send arbitrary ciphertexts (including malformed ones or maliciously-crafted ones), that could lead to revealing data about the FHE secret key.

Therefore, we employ zero-knowledge proofs of knowledge (ZKPoK) of input FHE ciphertexts that guarantee:

* ciphertext is well-formed (i.e. encryption has been done correctly)
* the user knows the plaintext value
* the input ciphertext can only be used in a particular smart contract

The ZKPoK is verified by validator nodes when the input byte array is passed to an `TFHE.asEuintXX()` function to convert from a ciphertext to a handle that can be used in smart contracts for FHE operations.

## Compact Input Lists

To greatly reduce the size of FHE ciphertexts inputs, we utilize a feature called compact lists. It allows us to pack multiple values efficiently. It is useful when there is only one input and even more so when the are multiple inputs in a call to a smart contract.

We define the `einput` type that refers to a particular ciphertext in the list. The list itself is serialized and passed as a byte array. For example, `inputA` and `inputB` refer to ciphertexts in the list and the serialized list is `inputProof`:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";

contract Adder {
  euint32 result;

  function add(einput inputA, einput inputB, bytes calldata inputProof) public {
    euint32 a = TFHE.asEuint32(inputA, inputProof);
    euint32 b = TFHE.asEuint32(inputB, inputProof);
    result = TFHE.add(a, b);
    TFHE.allow(result, address(this));
  }
}
```

Note that `inputProof` also contains the ZKPoK.
