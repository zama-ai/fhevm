# Glossary

- _Executor_: A component that runs alongside the fhEVM blockchain node/validator and does the FHE computation. The node/validator and the Executor communicate over a network connection.

- _FheLib_: A precompiled contract that is available on nodes/validators. Exposes a number of functions, e.g. getting ciphertexts, verifying inputs, etc. At the time of writing, it exists at address **0x000000000000000000000000000000000000005d**.

- _fhevmjs_: A JavaScript library that allows dApps to interact with the fhEVM.

- _handle_: A handle refers to (or is a pointer to) a ciphertext in the fhEVM. A handle uniquely refers to a single ciphertext from the user's perspective.

- _KMS_: Key Management Service. Used for managing secret FHE key material.

- _Symbolic Execution_: Onchain execution where inputs to FHE operations are symbolic values (also called handles) that refer to ciphertexts. We check constraints on these handles, but ignore their actual values.

- _TFHE_: An Fully Homomorphic Encryption scheme used in fhEVM and TKMS.

- _TKMS_: Threshold Key Management Service. Uses threshold cryptography and multi-party computation. See _KMS_.

- _ZKPoK_: Zero-knowledge proof of knowledge of an input FHE ciphertext.

## Smart Contracts

### fhEVM

- _ACL Smart Contract_: Smart contract deployed on the fhEVM blockchain to manage access control of ciphertexts. dApp contracts use this to persists their own access rights and to delegate access to other contracts.

- _Gateway Smart Contract_: Smart contract deployed on the fhEVM blockchain that is used by a dApp smart contract to request a decrypt. This emits an event that triggers the gateway.

- _KMS Smart Contract_: Smart contract running on the fhEVM blockchain that is used by a dApp contract to verify decryption results from the TKMS. To that end, it contains the identity of the TKMS and is used to verify its signatures.

### TKMS

- _fhEVM ASC_: Smart contract to which transaction from the gateway (connector) are submitted to. This contract contains all customization logic required to work with the specific fhEVM blockchain.
