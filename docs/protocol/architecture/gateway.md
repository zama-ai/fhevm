# Gateway

The Gateway acts as the communication hub between the blockchain, the coprocessor, the KMS, and user-facing
applications.

### **Key functions**:

- **API for developers**: Exposes endpoints to submit encrypted inputs, request decryption, and manage user decryption.
- **Proof validation**: Forwards ZKPoKs to the Coprocessor for verification.
- **Off-chain coordination**: Handles smart contract and user decryption workflows in a verifiable and secure manner.

The Gateway abstracts complex cryptographic flows, simplifying developer integration.
