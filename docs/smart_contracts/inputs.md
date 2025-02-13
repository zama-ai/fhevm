# Encrypted Inputs

This document introduces the concept of encrypted inputs in the fhEVM, explaining their role, structure, validation process, and how developers can integrate them into smart contracts and applications.

{% hint style="info" %}
Understanding how encryption, decryption and reencryption works is a prerequisite before implementation, see [Encryption, Decryption, Re-encryption, and Computation](d_re_ecrypt_compute.md)
{% endhint %}

Encrypted inputs are a core feature of fhEVM, enabling users to push encrypted data onto the blockchain while ensuring data confidentiality and integrity.

## What are encrypted inputs?

Encrypted inputs are data values submitted by users in ciphertext form. These inputs allow sensitive information to remain confidential while still being processed by smart contracts. They are accompanied by **Zero-Knowledge Proofs of Knowledge (ZKPoKs)** to ensure the validity of the encrypted data without revealing the plaintext.

### Key characteristics of encrypted inputs:

1. **Confidentiality**: Data is encrypted using the public FHE key, ensuring that only authorized parties can decrypt or process the values.
2. **Validation via ZKPoKs**: Each encrypted input is accompanied by a proof verifying that the user knows the plaintext value of the ciphertext, preventing replay attacks or misuse.
3. **Efficient packing**: All inputs for a transaction are packed into a single ciphertext in a user-defined order, optimizing the size and generation of the zero-knowledge proof.

## Parameters in encrypted functions

When a function in a smart contract is called, it may accept two types of parameters for encrypted inputs:

1. **`einput`**: Refers to the index of the encrypted parameter, representing a specific encrypted input handle.
2. **`bytes`**: Contains the ciphertext and the associated zero-knowledge proof used for validation.

Here’s an example of a Solidity function accepting multiple encrypted parameters:

```solidity
function myExample(
  address account,
  uint id,
  bool isAllowed,
  einput param1,
  einput param2,
  einput param3,
  bytes calldata inputProof
) public {
  // Function logic here
}
```

In this example, `param1`, `param2`, and `param3` are encrypted inputs, while `inputProof` contains the corresponding ZKPoK to validate their authenticity.

## Client-Side implementation

To interact with such a function, developers can use the [fhevmjs](https://github.com/zama-ai/fhevmjs) library to create and manage encrypted inputs. Below is an example implementation:

```javascript
import { createInstances } from "../instance";
import { getSigners, initSigners } from "../signers";

await initSigners(); // Initialize signers
const signers = await getSigners();

const instance = await createInstances(this.signers);
// Create encrypted inputs
const input = instance.createEncryptedInput(contractAddress, userAddress);
const inputs = input.add64(64).addBool(true).add8(4).encrypt(); // Encrypt the parameters

// Call the smart contract function with encrypted inputs
contract.myExample(
  "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80", // Account address
  32, // Plaintext parameter
  true, // Plaintext boolean parameter
  inputs.handles[0], // Handle for the first parameter
  inputs.handles[1], // Handle for the second parameter
  inputs.handles[2], // Handle for the third parameter
  inputs.inputProof, // Proof to validate all encrypted inputs
);
```

In this example:

- **`add64`, `addBool`, and `add8`**: Specify the types and values of inputs to encrypt.
- **`encrypt`**: Generates the encrypted inputs and the zero-knowledge proof.

## Validating encrypted inputs

Smart contracts process encrypted inputs by verifying them against the associated zero-knowledge proof. This is done using the `TFHE.asEuintXX`, `TFHE.asEbool`, or `TFHE.asEaddress` functions, which validate the input and convert it into the appropriate encrypted type.

### Example validation that goes along the client-Side implementation

This example demonstrates a function that performs multiple encrypted operations, such as updating a user's encrypted balance and toggling an encrypted boolean flag:

```solidity
  function myExample(
    einput encryptedAmount,
    einput encryptedToggle,
    bytes calldata inputProof
  ) public {
    // Validate and convert the encrypted inputs
    euint64 amount = TFHE.asEuint64(encryptedAmount, inputProof);
    ebool toggleFlag = TFHE.asEbool(encryptedToggle, inputProof);

    // Update the user's encrypted balance
    balances[msg.sender] = TFHE.add(balances[msg.sender], amount);

    // Toggle the user's encrypted flag
    userFlags[msg.sender] = TFHE.not(toggleFlag);
  }

  // Function to retrieve a user's encrypted balance
  function getEncryptedBalance() public view returns (euint64) {
    return balances[msg.sender];
  }

  // Function to retrieve a user's encrypted flag
  function getEncryptedFlag() public view returns (ebool) {
    return userFlags[msg.sender];
  }
}
```

### Example validation in the `encryptedERC20.sol` smart contract

Here’s an example of a smart contract function that verifies an encrypted input before proceeding:

```solidity
function transfer(
  address to,
  einput encryptedAmount,
  bytes calldata inputProof
) public {
  // Verify the provided encrypted amount and convert it into an encrypted uint64
  euint64 amount = TFHE.asEuint64(encryptedAmount, inputProof);

  // Function logic here, such as transferring funds
  ...
}
```

### How validation works

1. **Input verification**:\
   The `TFHE.asEuintXX` function ensures that the input is a valid ciphertext with a corresponding ZKPoK.
2. **Type conversion**:\
   The function transforms the `einput` into the appropriate encrypted type (`euintXX`, `ebool`, etc.) for further operations within the contract.

## Best Practices

- **Input packing**: Minimize the size and complexity of zero-knowledge proofs by packing all encrypted inputs into a single ciphertext.
- **Frontend encryption**: Always encrypt inputs using the FHE public key on the client side to ensure data confidentiality.
- **Proof management**: Ensure that the correct zero-knowledge proof is associated with each encrypted input to avoid validation errors.

Encrypted inputs and their validation form the backbone of secure and private interactions in the fhEVM. By leveraging these tools, developers can create robust, privacy-preserving smart contracts without compromising functionality or scalability.
