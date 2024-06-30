# Encrypted inputs

Inputs are a cornerstone of fhEVM: they allow users to push encrypted data onto the blockchain. To prevent any attacks, a user must provide proof of knowledge of the plaintext value underlying the ciphertext. This prevents the reuse of a ciphertext already stored on the blockchain.

All inputs are packed into a single ciphertext in a user-defined order, thereby minimizing the size and time required to create a zero-knowledge proof.
When a function is called, there are two types of parameters: `einput`, which is the index of the parameter, and `bytes`, which contains the actual ciphertext and zero-knowledge proof.

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

### Client Side

On client side for the previous function, using [fhevmjs](https://github.com/zama-ai/fhevmjs), the code will be:

```javascript
const instance = await createInstance({ networkUrl: "http://localhost:8545" });

const input = instance.createEncryptedInput(contractAddress, userAddress);
const { inputs, data } = input.add64(64).addBool(true).add8(4).encrypt(); // Encrypt the three parameters

contract.myExample("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80", inputs[0], 32, inputs[1], inputs[2], data);
```

### Validate input

A contract can use an encrypted parameter by calling `TFHE.asEuintXX(param, proof)` (or `TFHE.asEbool` or `TFHE.asEaddress`). This function will transform the input as a valid encrypted type.

```solidity
function transfer(address to, einput encryptedAmount, bytes calldata inputProof) public {
  // Verify the provided encrypted amount
  euint64 amount = TFHE.asEuint64(encryptedAmount, inputProof);
  ...
}
```
