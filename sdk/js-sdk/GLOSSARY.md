# Zama glossary (draft)

> **Editing rules:** Do not change already-defined terms — improve only their definitions. Definitions taken from the Zama Litepaper must not be modified.

## Naming decisions

The following terms had multiple names in use across the codebase, docs, whitepaper, and SDK. The canonical names have been decided.

| Canonical name                                                 | Other names (deprecated)                                                               | Decision                                                                                                                                         |
| -------------------------------------------------------------- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| **FHE encryption key**                                         | FHEVM public key, TFHE public key, Zama public key, global FHE key, GlobalFhePkeParams | Decided — SDK uses `fetchFheEncryptionKeyBytes()`                                                                                                |
| **encrypted value** (prose) / `EncryptedValue` (code)          | handle, fhevmHandle, fheHandle, FhevmHandle                                            | Decided — `EncryptedValue` is the primary public type name. `Handle` is a secondary alias for FHE.sol familiarity. "Encrypted value" in prose.   |
| **transport key pair** / `TransportKeyPair` (code)             | user decryption key pair, client decryption key pair, kms key pair, FhevmDecryptionKey | Decided — SDK uses `TransportKeyPair`. Generate with `generateTransportKeyPair()`.                                                               |
| **TKMS private key**                                           | KMS private key                                                                        | Decided — SDK uses `tkmsPrivateKey` internally in `-p` files                                                                                     |
| **FHE gas**                                                    | fheGas, fhe-gas, HCU (homomorphic complexity unit)                                     | Pending                                                                                                                                          |
| **encrypted types**                                            | core encrypted types, internal encrypted types                                         | Decided — Solidity: `ebool`, `euint8`, etc. SDK public types: `Ebool`, `Euint8`, etc. (aliases of `EncryptedValue<T>`)                           |
| **clear value** (prose) / `ClearValue` (code)                  | DecryptedFhevmHandle, decrypted handle                                                 | Decided — SDK uses `ClearValue<T>` with typed aliases `ClearBool`, `ClearUint8`, etc.                                                            |
| **signed decryption permit** / `SignedDecryptionPermit` (code) | SignedPermit                                                                           | Decided — SDK uses `SignedSelfDecryptionPermit` (self) and `SignedDelegatedDecryptionPermit` (delegation). Created via `signDecryptionPermit()`. |
| **external encrypted value** / `ExternalEncryptedValue` (code) | ExternalFhevmHandle, inputHandle                                                       | Decided — SDK uses `ExternalEncryptedValue<T>` with typed aliases `ExternalEbool`, `ExternalEuint8`, etc. `InputHandle` is a secondary alias.    |

---

## 1. High-level concepts

Terms for non-technical audiences.

**FHEVM**
An extension of the Ethereum Virtual Machine that enables smart contracts to compute directly on encrypted data using Fully Homomorphic Encryption (FHE). It allows developers to build confidential applications while preserving the composability and programmability of the EVM.

**Confidential smart contracts**
Smart contracts that process encrypted inputs and encrypted state, ensuring that sensitive data (such as balances, bids, or votes) remains private while still allowing computation and verification on-chain.

**The Zama Confidential Blockchain Protocol** (or simply **the Zama Protocol**)
The decentralized infrastructure and software stack developed by Zama that enables confidential smart contracts. It includes components such as coprocessors, the Gateway, the Key Management Service (KMS), and cryptographic libraries that together support FHEVM execution.

---

## 2. Protocol architecture

Terms describing the components of the Zama Protocol. Definitions marked _(Litepaper)_ are taken from [the Zama Protocol Litepaper](https://docs.zama.org/protocol/zama-protocol-litepaper) and must not be modified.

**Host Chains** _(Litepaper)_
The L1s and L2s that are supported by the Zama Protocol, and on which developers deploy their confidential dapps.

**FHEVM Library** _(Litepaper)_
The library that developers use to create confidential smart contracts.

**FHEVM Executor** _(Litepaper)_
The contract that is called by dapps to execute FHE operations on the Host Chain. Each time a contract uses an FHE operation, the Executor automatically emits an event to notify Coprocessors to compute it.

**Access Control List (ACL)** _(Litepaper)_
A smart contract deployed on each Host Chain, which keeps tracks of who can decrypt what. The ACL is central to the operations of the Zama Protocol and is used both to verify a contract is allowed to compute on an encrypted value, and that an address is allowed to decrypt it. Each time a contract allows an address to use a ciphertext, an event is emitted and relayed by Coprocessors to the Gateway, enabling the aggregation of all Host Chain ACLs into a single Gateway ACL used by the KMS to authenticate decryption requests.

**$ZAMA token** _(Litepaper)_
The native token of the Zama Protocol, used for payment of the fees and staking.

**Gateway** _(Litepaper)_
A set of smart contracts used to orchestrate the Zama Protocol, and allow users to request verification of their encrypted inputs, decryption of ciphertexts and bridging of encrypted assets between Host Chains. Each of these operations is a transaction to the Gateway contracts, and requires paying a small fee in $ZAMA tokens. While the Gateway contracts could be deployed on any L1 or L2, we opted to run a dedicated Arbitrum rollup for the Zama Protocol, ensuring maximal performance and cost efficiency. Note that our rollup serves only the Zama Protocol and doesn't allow third party contracts to be deployed on it.

**Coprocessors** _(Litepaper)_
A set of nodes responsible for 1. verifying encrypted inputs from users, 2. running the actual FHE computations and storing the resulting ciphertexts, 3. relaying ACL events to the Gateway. The Zama Protocol uses multiple coprocessors, which each commit their results to the Gateway, which in turns runs a majority consensus. All tasks performed by the coprocessors are publicly verifiable. Coprocessors can be vertically and horizontally scaled based on throughput requirements of the various confidential dapps.

**Key Management Service (KMS)** _(Litepaper)_
A set of nodes running various Multi-Party Computation (MPC) protocols for key generation, CRS generation and threshold decryption. The KMS ensures that no single party can ever access the decryption keys. KMS nodes are orchestrated by the Gateway, ensuring all operations are publicly visible. Furthermore, all KMS nodes must run the MPC software inside AWS Nitro Enclaves, making it harder for operators to leak their shares while providing some level of integrity on the MPC computation. Eventually, our goal will be to use ZK-MPC to enable verifiability without hardware assumptions.

**Operators** _(Litepaper)_
A set of entities that run the Zama Protocol nodes. This includes Coprocessors and KMS nodes.

**Relayer**
A service that facilitates communication between applications and the Gateway. It helps users submit encrypted inputs and request decryptions without interacting directly with the protocol infrastructure.
_Source: fhevm-whitepaper, Javascript_

---

## 3. Protocol internals

Terms describing components, mechanisms, and data structures within the Zama Protocol.

### Infrastructure

**Coprocessor**
An off-chain execution node responsible for performing FHE computations triggered by symbolic operations emitted by smart contracts. Coprocessors also verify encrypted inputs and store ciphertexts associated with encrypted values.
_Source: fhevm-whitepaper, InputVerifier.sol_

**KMS node**
A node participating in the distributed Key Management Service that holds a share of the secret key and executes MPC protocols for key generation and decryption.
_Source: fhevm-whitepaper_

**KMS core**
The core cryptographic engine within a KMS node that holds the key share and executes MPC protocols. It is isolated from network communication (handled by the KMS connector) and runs inside an AWS Nitro Enclave for integrity and confidentiality.

**KMS connector**
The component within a KMS node that handles communication with the Gateway. It receives decryption requests forwarded by the Gateway, processes them using the node's key share, and returns signcrypted shares.

**MPC operator**
An entity that runs one or more Zama Protocol nodes (coprocessors and/or KMS nodes). Operators are responsible for maintaining uptime and security of their nodes. Synonym for **Operator** in the Litepaper definition.

**MPC threshold**
The minimum number of KMS nodes that must participate to complete a threshold operation (key generation, decryption). For example, a threshold of 9 out of 13 means any 9 KMS nodes can collectively produce a valid decryption, but no subset of 8 or fewer can. In the SDK, this is exposed as `kmsSignerThreshold` on `KmsVerifierContractData`.

### Smart contracts

**KMS Verifier**
A smart contract deployed on each Host Chain (`KMSVerifier.sol`) that stores the list of authorized KMS signer addresses and the threshold required to validate a decryption response. During public decryption, the SDK uses this contract to verify that the response was signed by a sufficient quorum of KMS nodes. The contract also provides the EIP-712 domain and the Gateway chain ID.
_Source: `KMSVerifier.sol`, SDK type: `KmsVerifierContractData`_

**Input Verifier**
A smart contract deployed on each Host Chain (`InputVerifier.sol`) that verifies encrypted inputs from users. It checks ZK proofs and coprocessor signatures to ensure that ciphertexts were correctly generated.
_Source: `InputVerifier.sol`, SDK type: `InputVerifierContractData`_

### Signers

**KMS signer**
A KMS participant that contributes cryptographic signatures or decryption shares as part of the threshold decryption process. A quorum (for example 9 out of 13 nodes) must cooperate to complete operations.
_Source: [`KMSVerifier.sol`](https://github.com/zama-ai/fhevm/blob/58aebb099b61b81ae33fdfb4258ff79e6f5ca0e8/host-contracts/contracts/KMSVerifier.sol#L240)_

**Coprocessor signer**
A wallet address that signs the coprocessor's result during the inputProof verification mechanism. The coprocessor signer produces an EIP-712 signature attesting that it verified and processed the user's encrypted input correctly.
_Source: [`InputVerifier.sol`](https://github.com/zama-ai/fhevm/blob/58aebb099b61b81ae33fdfb4258ff79e6f5ca0e8/host-contracts/contracts/InputVerifier.sol#L335)_

### Execution model

**Symbolic execution** (also: symbolic FHE execution)
The execution model used by FHEVM smart contracts where encrypted operations are represented symbolically using encrypted values (handles). The EVM emits events describing the operations, and coprocessors later perform the actual FHE computations on ciphertexts.

**FHE gas** (also: fheGas, HCU — homomorphic complexity unit)
A resource accounting mechanism that limits the amount of FHE computation requested by a transaction. It ensures that symbolic FHE operations emitted on-chain remain within the processing capacity of coprocessors.
_Source: HCULimit.sol_

---

## 4. Keys and cryptographic material

**KMS key** (also: master secret key)
The master secret key of the FHEVM protocol, used to decrypt all ciphertexts. This key is never held by a single entity — it is split into shares distributed across KMS nodes via Multi-Party Computation (MPC). The protocol's security relies on the assumption that no quorum of KMS operators colludes. Not to be confused with the user decryption key.

**FHE encryption key** (also: FHEVM public key, TFHE public key, Zama public key, global FHE public key)
The public key used across the Zama Protocol to encrypt all confidential inputs and contract state. This shared key enables composability between users and smart contracts operating on encrypted data. In the SDK, fetched via `fetchFheEncryptionKeyBytes()`.
_Source: fhevm-whitepaper_

**CRS** (Common Reference String)
A piece of cryptographic data necessary for the security of zero-knowledge proofs. The CRS is generated in advance via a ceremony by/for the KMS and shared between all clients and the server. A CRS can be reused for multiple encryptions with the same parameters. In the SDK, the CRS is fetched alongside the FHE encryption key via `fetchFheEncryptionKeyBytes()`.
_Source: [TFHE-rs docs](https://docs.zama.org/tfhe-rs/fhe-computation/advanced-features/zk-pok)_

**E2E transport key pair** (also: user decryption key pair, client decryption key pair, kms key pair)
A classical asymmetric key pair generated by a user (or client application) to receive decrypted values securely. The KMS encrypts decryption shares under the user's public key so that only the user can reconstruct the plaintext. In the SDK, this is the `TransportKeyPair` type, created via `generateTransportKeyPair()`, serialized via `serializeTransportKeyPair()`, and restored via `parseTransportKeyPair()`.

**TKMS private key** (also: KMS private key)
The user's private key used to decrypt KMS signcrypted shares during decryption. It is the private half of the E2E transport key pair. This key secures the communication channel between the KMS and the entity requesting decryption — it is a communication key, not the master decryption key of the protocol. The entity requesting decryption may be the end user themselves or a delegate (e.g., a bank decrypting an encrypted value on behalf of a user). In the SDK, this key is hidden inside the opaque `TransportKeyPair` object and is never directly accessible to application code.

---

## 5. Encryption and decryption

**Encrypted value** (also: handle, fhevmHandle, fheHandle)
A deterministic identifier (`bytes32`) representing an encrypted value in the FHEVM system. Encrypted values (called "handles" in FHE.sol and the FHEVM whitepaper) are used inside smart contracts instead of actual ciphertexts. Each one references exactly one ciphertext stored and processed by coprocessors. In the SDK, the primary public type is `EncryptedValue<T>`, with `Handle<T>` as a secondary alias. In developer-facing prose, prefer "encrypted value" over "handle". Subtypes: `ComputedEncryptedValue` (verified, on-chain result of FHE operations) and `ExternalEncryptedValue` (unverified input from `encrypt()`).
_Source: fhevm-whitepaper, Solidity, SDK_

**ZKPoK** (Zero-Knowledge Proof of Knowledge, also: ZKProof)
A cryptographic proof included in inputProof that demonstrates knowledge of the plaintext corresponding to a ciphertext and proves that the ciphertext was correctly generated.
_Source: fhevm-whitepaper_

**ZKProof** (SDK term)
A zero-knowledge proof generated by the SDK during encryption. It proves that the user correctly encrypted their plaintext values using the global FHE public key, without revealing what those values are. The proof is verified by coprocessors as part of the inputProof mechanism. In the SDK, this is the opaque `ZkProof` type returned by `generateZkProof()`.
_Source: SDK, TFHE WASM_

**inputProof**
A proof provided by users when submitting encrypted inputs to the protocol. It proves that the ciphertext is well-formed and that the sender knows the underlying plaintext.
_Source: [Solidity](https://github.com/zama-ai/fhevm/blob/58aebb099b61b81ae33fdfb4258ff79e6f5ca0e8/host-contracts/contracts/InputVerifier.sol#L242)_

**Private decryption** (SDK: `decrypt()`)
A decryption mechanism where the plaintext result is returned only to the requesting user. The decrypted value is re-encrypted under the user's E2E transport public key so that only that user can reconstruct it locally. Requires a signed decryption permit (`SignedSelfDecryptionPermit` or `SignedDelegatedDecryptionPermit`). Returns `ClearValue[]`.
_Source: SDK (not in fhevm-whitepaper, not in Solidity)_

**Public decryption** (SDK: `publicDecrypt()`)
A decryption operation whose result becomes publicly available and can be returned on-chain. This is typically used when the result of confidential computation must be revealed to everyone. Returns a `PublicDecryptionProof` with `orderedClearValues`.
_Source: SDK (not in fhevm-whitepaper, not in Solidity)_

**Decryption proof** (also: public decryption proof)
The KMS public decryption proof. It includes the KMS signatures, associated metadata, and the context needed for verification.

**KMS signcrypted shares** (SDK: `KmsSigncryptedShares`)
In the private decryption flow, the signed and encrypted shares sent by each KMS node as a response to a decryption request. Each share is encrypted under the user's E2E transport public key (so only the user can read it) and signed by the KMS node (so the user can verify it's authentic). The SDK fetches these via `fetchKmsSignedcryptedShares()` and reconstructs the plaintext locally via `decryptKmsSignedcryptedShares()` using the TKMS WASM module. The high-level `decrypt()` function wraps both steps.

**ExtraData**
A `bytes` field included in EIP-712 permits and Relayer requests. It serves as an opaque context parameter that binds a decryption request to a specific KMS signer set. In standard operations, use `"0x00"`. The extraData is included in the EIP-712 message that the user signs, the Relayer request payload, and the KMS verification. In the SDK, extraData is auto-fetched — developers don't need to provide it manually.
_Source: SDK types `KmsUserDecryptEIP712Message.extraData`, `PublicDecryptParameters.extraData`_

---

## 6. Encrypted data types

**Encrypted types** (also: core encrypted types, internal encrypted types)
Encrypted value types used inside smart contracts, such as `ebool`, `euint8`, `euint16`, etc. These represent encrypted values and are used during confidential computation. In the SDK, the corresponding TypeScript types are `Ebool`, `Euint8`, `Euint16`, etc. (aliases of `EncryptedValue<T>`).
_Source: [`encrypted-types`](https://www.npmjs.com/package/encrypted-types)_

**External encrypted types** (`externalExxx`)
Types used when encrypted values are provided as inputs from outside the blockchain, such as `externalEuint32`. These values must be verified with an inputProof before being converted into internal encrypted types using `FHE.fromExternal()`. In the SDK, the corresponding TypeScript types are `ExternalEbool`, `ExternalEuint8`, etc. (aliases of `ExternalEncryptedValue<T>`).
_Source: [`encrypted-types`](https://www.npmjs.com/package/encrypted-types)_

**FheType**
A JavaScript/Solidity type abstraction representing encrypted data types such as `ebool`, `euintX`, and external encrypted inputs. In the SDK, `FheType` is a union type: `"ebool" | "euint8" | "euint16" | "euint32" | "euint64" | "euint128" | "euint256" | "eaddress"`.
_Source: [`FheType.sol`](https://github.com/zama-ai/fhevm/blob/58aebb099b61b81ae33fdfb4258ff79e6f5ca0e8/host-contracts/contracts/shared/FheType.sol#L4)_

**ClearValue** (SDK type)
The result of decrypting an encrypted value. Pairs the original `EncryptedValue` with its decrypted plaintext, the `fheType`, and the JavaScript value type name. Type-specific aliases: `ClearBool`, `ClearUint8`, `ClearUint16`, `ClearUint32`, `ClearUint64`, `ClearUint128`, `ClearUint256`, `ClearAddress`.
_Source: SDK_

---

## 7. Zama Protocol internals

Terms specific to the Zama Protocol.

**TFHE worker**
A Web Worker (in browsers) or `worker_thread` (in Node.js) that runs the TFHE WASM module for multi-threaded encryption operations. The SDK spawns a pool of TFHE workers controlled by the `numberOfThreads` runtime config parameter. Workers share memory via `SharedArrayBuffer` (in browsers, this requires COOP/COEP headers).
_Source: `startWorkers.v1.5.3.js`, SDK runtime config_

**SNS worker**
_To be defined_

---

## 8. General cryptography terms

Commonly used cryptographic terms. Not defined by Zama, but included here for reference.

**FHE (Fully Homomorphic Encryption)**
A cryptographic method allowing arbitrary, complex, and unlimited computations (both addition and multiplication) on encrypted data (ciphertext) without decrypting it first. The result, when decrypted, matches the output of operations performed on plaintext. This ensures data remains secure during processing, enabling privacy-preserving cloud computing.

**Homomorphic encryption**
A form of encryption that allows computations to be performed on encrypted data without first having to decrypt it. The resulting computations are left in an encrypted form which, when decrypted, result in an output that is identical to that of the operations performed on the unencrypted data.

**Ciphertext**
The unreadable, scrambled output produced when plaintext is encrypted using a cryptographic algorithm and a key. In the Zama Protocol context: the encrypted representation of a plaintext value produced using the FHE encryption key. Ciphertexts are stored off-chain by coprocessors and referenced by encrypted values (handles). There is a deterministic 1:1 mapping between an encrypted value and a ciphertext.

**Plaintext**
Unencrypted, human-readable data that serves as the original input for an encryption algorithm or the final output of a decryption process.

**Cleartext**
Unencrypted, human-readable data that is stored or transmitted without any cryptographic protection. Unlike plaintext (which may be intended for encryption), cleartext is generally not intended to be encrypted.

**Trivial encryption**
In general cryptography: an encryption scheme that offers no real security. In the FHEVM context: a special encryption operation that produces a valid ciphertext without requiring a zero-knowledge proof. Typically used for values that do not require secrecy (for example, constants).

**Public key** _(in homomorphic encryption)_
Used by the data owner to encrypt raw data before sending it to a server for computation.

**Secret key** _(in homomorphic encryption)_
Kept securely by the data owner to decrypt the final results returned by the server.

**Evaluation key** _(in homomorphic encryption)_
A public key, often derived from the secret key, that allows the server to perform homomorphic operations (addition/multiplication) on ciphertexts without needing to decrypt them.

---

## 9. General computing terms

**Handle** _(general computing)_
In computer programming, a handle is an abstract reference to a resource that is used when application software references blocks of memory or objects that are managed by another system like a database or an operating system. In the FHEVM context, "handle" is the FHE.sol / whitepaper term for what the SDK calls an `EncryptedValue` — see **Encrypted value** in section 5.
Source: [Wikipedia](<https://en.wikipedia.org/wiki/Handle_(computing)>)
