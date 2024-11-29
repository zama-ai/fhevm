# **Encryption, Decryption, Re-encryption, and Computation**

This document introduces the core cryptographic operations in the fhEVM system, including how data is encrypted, decrypted, re-encrypted and computed upon while maintaining privacy.

The fhEVM system ensures end-to-end confidentiality by leveraging Fully Homomorphic Encryption (FHE). The encryption, decryption, re-encryption, and computation processes rely on a coordinated flow of information and cryptographic keys across the fhEVM components. This section details how these operations work and outlines the role of the FHE keys in enabling secure and private processing.

---

## **FHE Keys and Their Locations**

1. **Public Key**:

   - **Location**: Directly accessible from the frontend or the smart contract.
   - **Role**: Used to encrypt plaintext data before submission to the blockchain or during contract execution.

2. **Private Key**:

   - **Location**: Stored securely in the Key Management System (KMS).
   - **Role**: Used for decrypting ciphertexts when necessary, either to verify results or enable user-specific plaintext access.

3. **Evaluation Key**:
   - **Location**: Stored on the coprocessor.
   - **Role**: Enables operations on ciphertexts (e.g., addition, multiplication) without decrypting them.

<figure><img src="../.gitbook/assets/architecture.png" alt="FHE Keys Overview"><figcaption>High level overview of the fhEVM Architecture</figcaption></figure>

---

## **Workflow: Encryption, Decryption, and Processing**

### **1. Encryption**

Encryption is the starting point for any interaction with the fhEVM system, ensuring that data is protected before it is transmitted or processed.

- **How It Works**:

  1. The **frontend** or client application uses the **public key** to encrypt user-provided plaintext inputs.
  2. The encrypted data (ciphertext) is submitted to the blockchain as part of a transaction or stored for later computation.

- **Data Flow**:
  - **Source**: Frontend or smart contract.
  - **Destination**: Blockchain (for storage and symbolic execution) or coprocessor (for processing).

<figure style="text-align: center"><img src="../.gitbook/assets/encrypt.png" alt="re-encryption" width="600"></figure>

You can read about the implemention details in [our encryption guide](../fundamentals//first_step/inputs.md).

---

### **2. Computation**

Encrypted computations are performed using the **evaluation key** on the coprocessor.

- **How It Works**:

  1. The blockchain initiates symbolic execution, generating a set of operations to be performed on encrypted data.
  2. These operations are offloaded to the **coprocessor**, which uses the **evaluation key** to compute directly on the ciphertexts.
  3. The coprocessor returns updated ciphertexts to the blockchain for storage or further use.

- **Data Flow**:
  - **Source**: Blockchain smart contracts (via symbolic execution).
  - **Processing**: Coprocessor (using the evaluation key).
  - **Destination**: Blockchain (updated ciphertexts).

<figure style="text-align: center"><img src="../.gitbook/assets/computation.png" alt="computation"></figure>

---

### **3. Decryption**

Decryption is used when plaintext results are required for contract logic or for presentation to a user. After the decryption in preformed on the blockchain, the decrypted result is exposed to everyone who has access to the blockchain.

<figure style="text-align: center"><img src="../.gitbook/assets/decryption.png" alt="decryption"></figure>

- **How it works**:  
   Validators on the blockchain do not possess the private key needed for decryption. Instead, the **Key Management System (KMS)** securely holds the private key. If plaintext values are needed, the process is facilitated by a service called the **Gateway**, which provides two options:

  1.  **For Smart Contract Logic**:  
      The Gateway acts as an oracle service, listening for decryption request events emitted by the blockchain. Upon receiving such a request, the Gateway interacts with the KMS to decrypt the ciphertext and sends the plaintext back to the smart contract via a callback function.

  2.  **For dApps**:  
      If a dApp needs plaintext values, the Gateway enables re-encryption of the ciphertext. The KMS securely re-encrypts the ciphertext with the dApp's public key, ensuring that only the dApp can decrypt and access the plaintext.

- **Data flow**:
  - **Source**: Blockchain or dApp (ciphertext).
  - **Processing**: KMS performs decryption or re-encryption via the Gateway.
  - **Destination**: Plaintext is either sent to the smart contract or re-encrypted and delivered to the dApp.

<figure><img src="../.gitbook/assets/asyncDecrypt.png" alt="re-encryption"><figcaption>re-encryption</figcaption></figure>

You can read about the implemention details in [our decryption guide](../fundamentals//first_step/decrypt.md).

---

### **4. Re-encryption**

Re-encryption enables encrypted data to be securely shared or reused under a different encryption key without ever revealing the plaintext. This process is essential for scenarios where data needs to be accessed by another contract, dApp, or user while maintaining confidentiality.

#### How it works

Re-encryption is facilitated by the **Gateway** in collaboration with the **Key Management System (KMS)**:

1. The Gateway receives a re-encryption request, which includes details of the original ciphertext and the target public key.
2. The KMS securely decrypts the ciphertext using its private key and re-encrypts the data with the recipient's public key.
3. The re-encrypted ciphertext is then sent to the intended recipient.

---

#### Data flow

1. **source**:

   - The process starts with an original ciphertext retrieved from the blockchain or a dApp.

2. **processing**:

   - The Gateway forwards the re-encryption request to the KMS.
   - The KMS handles decryption and re-encryption using the appropriate keys.

3. **destination**:
   - The re-encrypted ciphertext is delivered to the target entity, such as a dApp, user, or another contract.

<figure style="text-align: center"><img src="../.gitbook/assets/reencryption.png" alt="re-encryption"><figcaption>re-encryption process</figcaption></figure>

---

#### Client-side implementation

Re-encryption is initiated on the client side via the **Gateway service** using the [`fhevmjs`](https://github.com/zama-ai/fhevmjs/) library. Here’s the general workflow:

1. **Retrieve the ciphertext**:

   - The dApp calls a view function (e.g., `balanceOf`) on the smart contract to get the handle of the ciphertext to be re-encrypted.

2. **Generate and sign a keypair**:

   - The dApp generates a keypair for the user.
   - The user signs the public key to ensure authenticity.

3. **Submit re-encryption request**:

   - The dApp calls the Gateway, providing the following information:
     - The ciphertext handle.
     - The user’s public key.
     - The user’s address.
     - The smart contract address.
     - The user’s signature.

4. **Decrypt the re-encrypted ciphertext**:
   - The dApp receives the re-encrypted ciphertext from the Gateway.
   - The dApp decrypts the ciphertext locally using the private key.

<figure><img src="../.gitbook/assets/reencryption.png" alt="re-encryption"><figcaption>re-encryption</figcaption></figure>

You can read [our guide explaining how to use it](../fundamentals/first_step/reencryption.md).

---

## **Tying It All Together**

The flow of information across the fhEVM components during these operations highlights how the system ensures privacy while maintaining usability:

| **Operation**     | **Keys Used**           | **Flow of Information**                                                                                                                                                         |
| ----------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Encryption**    | Public Key              | Frontend encrypts plaintext → Ciphertext submitted to blockchain or coprocessor.                                                                                                |
| **Computation**   | Evaluation Key          | Blockchain initiates computation → Coprocessor processes ciphertext using evaluation key → Updated ciphertext returned to blockchain.                                           |
| **Decryption**    | Private Key             | Blockchain or Gateway sends ciphertext to KMS → KMS decrypts using private key → Plaintext returned to authorized requester (e.g., frontend or specific user).                  |
| **Re-encryption** | Private and Target Keys | Blockchain or Gateway sends ciphertext to KMS → KMS re-encrypts using private key and target key → Updated ciphertext returned to blockchain, frontend, or other contract/user. |

This architecture ensures that sensitive data remains encrypted throughout its lifecycle, with decryption or re-encryption only occurring in controlled, secure environments. By separating key roles and processing responsibilities, fhEVM provides a scalable and robust framework for private smart contracts.
