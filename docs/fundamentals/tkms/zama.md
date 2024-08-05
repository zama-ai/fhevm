# Zama's TKMS

The Key Management System (TKMS) is a self-contained service for performing sensitive cryptographic operations, including for a native fhEVM or a co-processor. It offers:

- **FHE key generation**: Generate a fresh FHE keypair; the secret key is stored securely inside the KMS and the public key is made available for download. This generation also includes bootstrapping keys with a secret PRF seed for randomness generation.
- **FHE decryption**: Decrypt a ciphertext encrypted under an FHE key known by the KMS and return the plaintext.
- **FHE reencryption**: Decrypt a ciphertext encrypted under an FHE key known by the KMS and return the plaintext encrypted under a client supplied public key.
- **Public material download**: Return URIs and signed fingerprints of the public material.
- **CRS generation**: Generate a fresh CRS, and make it available for download.

One KMS instance can support multiple applications at the same time. This is implemented via per application or per application type smart contracts running in the KMS. These smart contracts are customizable to for instance implement application specific authorization logic (e.g. ACLs).

