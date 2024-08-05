# Zama's TKMS

The Key Management System (TKMS) is a self-contained service for performing sensitive cryptographic operations, including for a native fhEVM or a co-processor. It offers:

- **FHE key generation**: Generate a fresh FHE keypair; the secret key is stored securely inside the KMS and the public key is made available for download. This generation also includes bootstrapping keys with a secret PRF seed for randomness generation.
- **FHE decryption**: Decrypt a ciphertext encrypted under an FHE key known by the KMS and return the plaintext.
- **FHE reencryption**: Decrypt a ciphertext encrypted under an FHE key known by the KMS and return the plaintext encrypted under a client supplied public key.
- **Public material download**: Return URIs and signed fingerprints of the public material.
- **CRS generation**: Generate a fresh CRS, and make it available for download.

One KMS instance can support multiple applications at the same time. This is implemented via per application or per application type smart contracts running in the KMS. These smart contracts are customizable to for instance implement application specific authorization logic (e.g. ACLs).

## Gateway

The KMS system is facilitated through a gateway service which is designed _not_ to be required to be trusted, thus a malicious Gateway Service will _not_ be able to compromise correctness or privacy of the system, but at most be able to block requests and responses between the fhEVM and the KMS. However, this can be prevented by simply deploying multiple Gateways Services.

Furthermore we observe that it is possible to implement payment to a Gateway service through the KMS blockchain, thus incentivizing such a service to be honest and reliable.

The Gateway Service consists of two different Connectors in order to decouple a specific fhEVM from a specific KMS. This will make it simpler to roll new blockchain protocols on either the fhEVM or KMS side without requiring modifications to the Gateway, but instead only require the writing of new Connectors.