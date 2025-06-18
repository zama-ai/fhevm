# **Key management system (KMS)**

The KMS is a decentralized threshold-MPC-based service that manages the FHE key lifecycle and cryptographic security.

### **Key functions**:

- **Threshold decryption**: Uses Multi-Party Computation (MPC) to securely decrypt ciphertexts without exposing the
  private key to any single entity.
- **Key distribution**: Maintains the global FHE keys, which include:
  - **Public key**: Used for encrypting data (accessible to the frontend and smart contracts).
  - **Private key**: Stored securely in the KMS and used for decryption.
  - **Evaluation key**: Used by the coprocessor to perform FHE computations.

The KMS ensures robust cryptographic security, preventing single points of failure and maintaining public verifiability.

## **FHE keys and their locations**

1. **Public Key**:
   - **Location**: Exposed via frontend SDK.
   - **Role**: Encrypts user inputs before any interaction with the blockchain.
2. **Private Key**:
   - **Location**: Secured in the Key Management System (KMS) using threshold MPC.
   - **Role**: Used to decrypt data when necessary — such as to reveal plaintext to users or smart contracts.
3. **Evaluation Key**:
   - **Location**: Hosted on coprocessors.
   - **Role**: Usage: Enables encrypted computation without decrypting any data.
