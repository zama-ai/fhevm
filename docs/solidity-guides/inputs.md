# Encrypted Inputs

This document introduces the concept of encrypted inputs in the fhevm, explaining their role, structure, validation process, and how developers can integrate them into smart contracts and applications.

{% hint style="info" %} Understanding how encryption, public, and user decryption works is a prerequisite before implementation, see [Encryption, Decryption, and Computation](../protocol/d_re_ecrypt_compute.md) {% endhint %}

Encrypted inputs are a core feature of fhevm, enabling users to push encrypted data onto the blockchain while ensuring data confidentiality and integrity.

## What are encrypted inputs?

Encrypted inputs are data values submitted by users in ciphertext form. These inputs allow sensitive information to remain confidential while still being processed by smart contracts. They are accompanied by **Zero-Knowledge Proofs of Knowledge (ZKPoKs)** to ensure the validity of the encrypted data without revealing the plaintext.

### Key characteristics of encrypted inputs:

1. **Confidentiality**: Data is encrypted using the public FHE key, ensuring that only authorized parties can decrypt or process the values.
2. **Validation via ZKPoKs**: Each encrypted input is accompanied by a proof verifying that the user knows the plaintext value of the ciphertext, preventing replay attacks or misuse.
3. **Efficient packing**: All inputs for a transaction are packed into a single ciphertext in a user-defined order, optimizing the size and generation of the zero-knowledge proof.

## Parameters in encrypted functions

When a function in a smart contract is called, it may accept two types of parameters for encrypted inputs:

1. **`externalEbool`, `externalEaddress`,`externalEuintXX`**: Refers to the index of the encrypted parameter, representing a specific encrypted input handle.
2. **`bytes`**: Contains the ciphertext and the associated zero-knowledge proof used for validation.

Here’s an example of a Solidity function accepting multiple encrypted parameters:

```solidity
function exampleFunction(
  externalEbool param1,
  externalEuint64 param2,
  externalEuint8 param3,
  bytes calldata inputProof
) public {
  // Function logic here
}
```

In this example, `param1`, `param2`, and `param3` are encrypted inputs for `ebool`, `euint64`, and `euint8` while `inputProof` contains the corresponding ZKPoK to validate their authenticity.

### Input Generation using Hardhat

In the below example, we use Alice's address to create the encrypted inputs and submits the transaction.

```typescript
import * as hre from "hardhat";
import { fhevm } from "hardhat";

const input = hre.fhevm.createEncryptedInput(contract.address, signers.alice.address);
input.add64(transferAmount);
const encryptedInput = await input.encrypt();

tx = await myContract
  .connect(signers.alice)
  [
    "exampleFunction(bytes32,bytes32,bytes32,bytes)"
  ](signers.bob.address, encryptedInput.handles[0], encryptedInput.handles[1], encryptedInput.handles[2], encryptedTransferAmount.inputProof);

await tx.wait();
```

## Validating encrypted inputs

Smart contracts process encrypted inputs by verifying them against the associated zero-knowledge proof. This is done using the `FHE.asEuintXX`, `FHE.asEbool`, or `FHE.asEaddress` functions, which validate the input and convert it into the appropriate encrypted type.

### Example validation

This example demonstrates a function that performs multiple encrypted operations, such as updating a user's encrypted balance and toggling an encrypted boolean flag:

```solidity
function myExample(externalEuint64 encryptedAmount, externalEbool encryptedToggle, bytes calldata inputProof) public {
  // Validate and convert the encrypted inputs
  euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);
  ebool toggleFlag = FHE.fromExternal(encryptedToggle, inputProof);

  // Update the user's encrypted balance
  balances[msg.sender] = FHE.add(balances[msg.sender], amount);

  // Toggle the user's encrypted flag
  userFlags[msg.sender] = FHE.not(toggleFlag);
}

// Function to retrieve a user's encrypted balance
function getEncryptedBalance() public view returns (euint64) {
  return balances[msg.sender];
}

// Function to retrieve a user's encrypted flag
function getEncryptedFlag() public view returns (ebool) {
  return userFlags[msg.sender];
}
```

### Example validation in the `ConfidentialERC20.sol` smart contract

Here’s an example of a smart contract function that verifies an encrypted input before proceeding:

```solidity
function transfer(
  address to,
  externalEuint64 encryptedAmount,
  bytes calldata inputProof
) public {
  // Verify the provided encrypted amount and convert it into an encrypted uint64
  euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);

  // Function logic here, such as transferring funds
  ...
}
```

### How validation works

1. **Input verification**:\
   The `FHE.fromExternal` function ensures that the input is a valid ciphertext with a corresponding ZKPoK.
2. **Type conversion**:\
   The function transforms `externalEbool`, `externalEaddress`, `externalEuintXX` into the appropriate encrypted type (`ebool`, `eaddress`, `euintXX`) for further operations within the contract.

## Best Practices

- **Input packing**: Minimize the size and complexity of zero-knowledge proofs by packing all encrypted inputs into a single ciphertext.
- **Frontend encryption**: Always encrypt inputs using the FHE public key on the client side to ensure data confidentiality.
- **Proof management**: Ensure that the correct zero-knowledge proof is associated with each encrypted input to avoid validation errors.

Encrypted inputs and their validation form the backbone of secure and private interactions in the fhevm. By leveraging these tools, developers can create robust, privacy-preserving smart contracts without compromising functionality or scalability.
