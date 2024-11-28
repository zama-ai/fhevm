# Glossary

- _Coprocessor_: An off-chain component in fhEVM-native that does the actual FHE computation.

- _Executor_: A component that runs alongside the fhEVM-native blockchain node/validator and does the FHE computation. The node/validator and the Executor communicate over a network connection.

- _FheLib_: A precompiled contract on fhEVM-native that is available on nodes/validators. Exposes functions such as reading FHE ciphertexts from the on-chain storage in fhEVM-native, etc. At the time of writing, it exists at address **0x000000000000000000000000000000000000005d**.

- _fhEVM-coprocessor_: An fhEVM configuration where an off-chain Coprocessor component does the actual FHE computation. FHE ciphertexts are stored in an off-chain database local to the Coprocessor and in an off-chain public Data Availablility (DA) layer. No modifications the validator software of the existing chain is required (except for the full-node running for the Coprocessor).

- _fhEVM-native_: An fhEVM configuration where each validator is paired with an Executor. FHE ciphertexts are stored on-chain. fhEVM-native requires modifications to the validator software of an existing chain.

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

- _fhEVM ISC_: Smart contract which contains all the custom logic needed to validate whether an operation such as decryption, is permitted on a given fhEVM chain. Specifically this involves inclusion proofs of an ACL. Note there is _one_ ISC for _each_ fhEVM.

- _fhEVM ASC_: Smart contract to which transactions from the gateway (connector) are submitted to. This contract contains all logic required to work with _any_ fhEVM blockchain. It handles any fhEVM chain-specific logic (such as ACL validation) by calling the ISC associated with the given fhEVM chain.
