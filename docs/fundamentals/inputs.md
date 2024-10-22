# Encrypted inputs

This document introduces the concept of encrypted inputs in the fhEVM, explaining how they are used, structured, and validated within smart contracts.

Inputs are a cornerstone of fhEVM: they allow users to push encrypted data onto the blockchain.

To prevent any attacks, a user must provide proof of knowledge of the plaintext value underlying the ciphertext. This proof ensures that a ciphertext cannot be reused once stored on the blockchain.

All inputs are packed into a single ciphertext in a user-defined order, thereby minimizing the size and time required to create a zero-knowledge proof.

When a function is called, there are two types of parameters:

- `einput`: Represents the index of the encrypted parameter.
- `bytes`: Contains the actual ciphertext and the associated zero-knowledge proof.

For example, if a function requires 3 encrypted parameters, it could be written as follows:

```solidity
function myExample(
  address account
  einput param1,
  uint id,
  einput param2,
  einput param3,
  bool isAllowed,
  bytes calldata inputProof
) {}
```

### Client-side implementation

On client side, you can interact with the the previous function using [fhevmjs](https://github.com/zama-ai/fhevmjs). Here's an example:

```javascript
const instance = await createInstance({
  kmsContractAddress: "0x208De73316E44722e16f6dDFF40881A3e4F86104",
  aclContractAddress: "0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c",
  networkUrl: "https://devnet.zama.ai/",
  gatewayUrl: "https://gateway.zama.ai/",
});

const input = instance.createEncryptedInput(contractAddress, userAddress);
const inputs = input.add64(64).addBool(true).add8(4).encrypt(); // Encrypt the three parameters

contract.myExample(
  "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
  inputs.handles[0],
  32,
  inputs.handles[1],
  inputs.handles[2],
  true,
  inputs.inputProof,
);
```

### Validate input

A contract can use an encrypted parameter by calling `TFHE.asEuintXX(param, proof)` (or `TFHE.asEbool` or `TFHE.asEaddress`). This function will transform the input as a valid encrypted type:

```solidity
function transfer(address to, einput encryptedAmount, bytes calldata inputProof) public {
  // Verify the provided encrypted amount
  euint64 amount = TFHE.asEuint64(encryptedAmount, inputProof);
  ...
}
```
