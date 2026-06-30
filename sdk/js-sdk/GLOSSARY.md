# Internal / External Zama Glossary

This glossary is organized **top-down by altitude** — from non-technical
concepts, through the protocol vocabulary the Litepaper defines, down to the
SDK's high-level client methods, low-level actions, modules, and WASM internals.

SDK-specific terms carry an **altitude tag** so you always know whether a term is
something an app developer reaches for daily or an internal mechanism:

| Tag | Layer | Example |
| --- | --- | --- |
| `[high]` | Client methods (decorators) | `client.encryptValues({...})` |
| `[low]` | Standalone actions | `encryptValues(fhevm, {...})` |
| `[module]` | Runtime modules | `runtime.relayer.fetchUserDecrypt()` |
| `[wasm]` | WASM bindings | TFHE / TKMS |

## 1. High-level concepts

Terms for non-technical audiences. No code, no types — just the ideas.

**FHEVM**
An extension of the Ethereum Virtual Machine that enables smart contracts to compute directly on encrypted data using Fully Homomorphic Encryption (FHE). It allows developers to build confidential applications while preserving the composability and programmability of the EVM.

**Confidential smart contracts**
Smart contracts that process encrypted inputs and encrypted state, ensuring that sensitive data (such as balances, bids, or votes) remains private while still allowing computation and verification on-chain.

**The Zama Confidential Blockchain Protocol** (or simply **the Zama Protocol**)
The decentralized infrastructure and software stack developed by Zama that enables confidential smart contracts. It includes components such as coprocessors, relayer, the Key Management Service (KMS), and cryptographic libraries that together support FHEVM execution.

---

## 2. Protocol architecture (Litepaper terms)

Terms describing the components of the Zama Protocol. Definitions marked *(Litepaper)* are taken from the Zama Protocol Litepaper and must not be modified.

**Host Chains** *(Litepaper)*
The L1s and L2s that are supported by the Zama Protocol, and on which developers deploy their confidential dapps.

**FHEVM Library** *(Litepaper)*
The library that developers use to create confidential smart contracts.

**FHEVM Executor** *(Litepaper)*
The contract that is called by dapps to execute FHE operations on the Host Chain. Each time a contract uses an FHE operation, the Executor automatically emits an event to notify Coprocessors to compute it.

**Access Control List (ACL)** *(Litepaper)*
A smart contract deployed on each Host Chain, which keeps tracks of who can decrypt what. The ACL is central to the operations of the Zama Protocol and is used both to verify a contract is allowed to compute on an encrypted value, and that an address is allowed to decrypt it. Each time a contract allows an address to use a ciphertext, an event is emitted and relayed by Coprocessors to the Gateway, enabling the aggregation of all Host Chain ACLs into a single Gateway ACL used by the KMS to authenticate decryption requests.

**$ZAMA token** *(Litepaper)*
The native token of the Zama Protocol, used for payment of the fees and staking.

**Gateway** *(Litepaper)*
A set of smart contracts used to orchestrate the Zama Protocol, and allow users to request verification of their encrypted inputs, decryption of ciphertexts and bridging of encrypted assets between Host Chains. Each of these operations is a transaction to the Gateway contracts, and requires paying a small fee in $ZAMA tokens. While the Gateway contracts could be deployed on any L1 or L2, we opted to run a dedicated Arbitrum rollup for the Zama Protocol, ensuring maximal performance and cost efficiency. Note that our rollup serves only the Zama Protocol and doesn't allow third party contracts to be deployed on it.

**Coprocessors** *(Litepaper)*
A set of nodes responsible for 1. verifying encrypted inputs from users, 2. running the actual FHE computations and storing the resulting ciphertexts, 3. relaying ACL events to the Gateway. The Zama Protocol uses multiple coprocessors, which each commit their results to the Gateway, which in turns runs a majority consensus. All tasks performed by the coprocessors are publicly verifiable. Coprocessors can be vertically and horizontally scaled based on throughput requirements of the various confidential dapps.

**Key Management Service (KMS)** *(Litepaper)*
A set of nodes running various Multi-Party Computation (MPC) protocols for key generation, CRS generation and threshold decryption. The KMS ensures that no single party can ever access the decryption keys. KMS nodes are orchestrated by the Gateway, ensuring all operations are publicly visible. Furthermore, all KMS nodes must run the MPC software inside AWS Nitro Enclaves, making it harder for operators to leak their shares while providing some level of integrity on the MPC computation. Eventually, our goal will be to use ZK-MPC to enable verifiability without hardware assumptions.

**Operators** *(Litepaper)*
A set of entities that run the Zama Protocol nodes. This includes Coprocessors and KMS nodes.

**Relayer**
A service that facilitates communication between applications and the rest of the Zama Protocol. It helps users submit encrypted inputs and request decryptions without interacting directly with the protocol infrastructure.
*Source: fhevm-whitepaper, Javascript*

---

## 3. SDK terms

### 3a. High-level client methods `[high]`

**Encryption** `[high]`
SDK term: `encryptValue()`, `encryptValues()`
Encrypts plaintext values client-side — `encryptValue()` for a single value, `encryptValues()` for a batch under one shared proof. Both return external encrypted values plus an `inputProof` to pass to a contract. The FHE public encryption key is fetched and cached on first use; internally orchestrates the proof machinery (§3b).
*Source: SDK*

**Decryption** `[high]`
SDK term: `decryptValue()`, `decryptValues()`, `decryptValuesFromPairs()`
Decrypts encrypted values privately to their owner — a single value, a batch, or from `(encryptedValue, contractAddress)` pairs. The plaintext is re-encrypted under the user's transport public key so that only that user can reconstruct it locally. Requires a signed decryption permit and a `TransportKeyPair`. Returns `ClearValue[]`.
*Source: SDK (not in fhevm-whitepaper, not in Solidity)Deprecated terms (do not use): reencrypt, reencryption, user decrypt*

**Public value decryption** `[high]`
SDK term: `decryptPublicValue()`, `decryptPublicValues()`, `decryptPublicValuesWithSignatures()`
Reads values a contract has made publicly decrypt-able — used when the result of confidential computation must be revealed to everyone. `decryptPublicValuesWithSignatures()` additionally returns a `DecryptionProof` (with `checkSignaturesArgs`) so a contract can verify on-chain that a handle decrypts to a specific clear value.
*Source: SDK (not in fhevm-whitepaper, not in Solidity)Deprecated terms (do not use): publicDecrypt (stale README/api-reference name)*

**Transport key pair** `[high]`
SDK term: `generateTransportKeyPair()`, `serializeTransportKeyPair()`, `parseTransportKeyPair()`; type `TransportKeyPair`
A classical asymmetric key pair generated by the user to receive decrypted values securely — the KMS encrypts decryption shares under its public key so only the user can reconstruct the plaintext. The opaque `TransportKeyPair` type never exposes its private key.
*Source: SDK type `TransportKeyPair`, `src/core/kms/TransportKeyPair-p.ts`Deprecated terms (do not use): E2eTransportKeypair, generateE2eTransportKeypair, serializeE2eTransportKeypair, parseE2eTransportKeypair, user decryption key pair, client decryption key pair, kms key pair, FhevmDecryptionKey*

**Decryption permit** `[high]`
SDK term: `signDecryptionPermit()`, `serializeSignedDecryptionPermit()`, `parseSignedDecryptionPermit()`; types `SignedSelfDecryptionPermit`, `SignedDelegatedDecryptionPermit`
An EIP-712 permit signed by the user that authorizes a decryption request. `signDecryptionPermit()` constructs and signs it in one step, producing `SignedSelfDecryptionPermit` (self-decryption) or `SignedDelegatedDecryptionPermit` (delegated, with an `onBehalfOf` field). The lower-level EIP-712 builders live in §3b.
*Source: SDKDeprecated terms (do not use): SignedPermit*

### 3b. High-level data types `[high]`

The everyday types an app developer passes and receives.

**Encrypted value** `[high]`
SDK term: `EncryptedValue<T>` (alias `Handle<T>`)
A deterministic identifier (`bytes32`) representing an encrypted value in the FHEVM system. Encrypted values (called "handles" in `FHE.sol` and the FHEVM whitepaper) are used inside smart contracts instead of actual ciphertexts; each references exactly one ciphertext stored and processed by coprocessors. Prefer "encrypted value" over "handle" in prose. Subtypes: `ComputedEncryptedValue` (verified, on-chain result of FHE operations) and `ExternalEncryptedValue` (unverified input from `encryptValue()`/`encryptValues()`; typed aliases `ExternalEbool`, `ExternalEuint8`, etc.; `InputHandle` is a secondary alias).
*Source: fhevm-whitepaper, Solidity, SDKDeprecated terms (do not use): fhevmHandle, fheHandle, FhevmHandle (for the value itself); ExternalFhevmHandle, inputHandle (for ExternalEncryptedValue)*

**Encrypted types** `[high]`
SDK term: `Ebool`, `Euint8`, `Euint16`, … `Euint256`, `Eaddress`
Encrypted value types used inside smart contracts (`ebool`, `euint8`, `euint16`, etc.). In the SDK, the corresponding TypeScript types are `Ebool`, `Euint8`, etc. (aliases of `EncryptedValue<T>`).
*Source: `encrypted-types`Deprecated terms (do not use): core encrypted types, internal encrypted types*

**External encrypted types** `[high]`
SDK term: `ExternalEbool`, `ExternalEuint8`, … `ExternalEaddress`
Types used when encrypted values are provided as inputs from outside the blockchain, such as `externalEuint32`. These must be verified with an inputProof before conversion into internal encrypted types via `FHE.fromExternal()`. In the SDK, the corresponding TypeScript types are aliases of `ExternalEncryptedValue<T>`.
*Source: `encrypted-types`*

**FheType** `[high]`
SDK term: `FheType`
A type abstraction representing encrypted data types. In the SDK, `FheType` is a union: `"ebool" | "euint8" | "euint16" | "euint32" | "euint64" | "euint128" | "euint256" | "eaddress"`.
*Source: `FheType.sol`*

**Typed value (encryption input)** `[high]`
SDK term: `TypedValueLike`, `TypedValue`
The input format for encryption — `{ type, value }` using Solidity type names (`"uint32"`, `"bool"`, `"address"`), not FHE names. `TypedValueLike` accepts flexible input; `TypedValue` is the validated/normalized form.
*Source: SDK*

**ClearValue** `[high]`
SDK term: `ClearValue` (prose: clear value); aliases `ClearBool`, `ClearUint8`, … `ClearUint256`, `ClearAddress`
The result of decrypting an encrypted value. Pairs the original `EncryptedValue` with its decrypted plaintext, the `fheType`, and the JavaScript value type name.
*Source: SDKDeprecated terms (do not use): DecryptedFhevmHandle, decrypted handle*

### 3c. Low-level standalone actions `[low]`

The same operations as 3a, imported from `@fhevm/sdk/actions/*` and taking the `fhevm` client as the first argument — plus the building blocks the high-level methods orchestrate internally.

**Proof machinery** `[low]`
SDK term: `generateZkProof()`, `fetchVerifiedInputProof()`, `ZkProof`, `VerifiedInputProof`, `inputProofgenerateZkProof()` produces the `ZkProof` (also: ZKPoK — Zero-Knowledge Proof of Knowledge, the fhevm-whitepaper term) of correct encryption — proving the user encrypted their plaintext under the FHE public key and knows the underlying value, without revealing it. `fetchVerifiedInputProof()` exchanges it for a `VerifiedInputProof` with coprocessor signatures (the `inputProof`) that proves the ciphertext is well-formed.
*Source: SDK, TFHE WASM, fhevm-whitepaper, Solidity*

**KMS share flow** `[low]`
SDK term: `fetchKmsSignedcryptedShares()`, `decryptKmsSignedcryptedShares()`, `KmsSigncryptedShares`
In the private decryption flow, the signed-and-encrypted shares each KMS node returns: each is encrypted under the user's transport public key (only the user can read it) and signed by the node (the user can verify authenticity). `fetchKmsSignedcryptedShares()` retrieves them; `decryptKmsSignedcryptedShares()` reconstructs the plaintext locally via the TKMS WASM. These are the two steps the high-level `decryptValue()` wraps. Note the naming: the *type* is `KmsSigncryptedShares` ("signcryption"), but the *functions* spell it `Signedcrypted` (extra "ed") — a code inconsistency worth fixing at the source.

**EIP712** `[low]`
SDK term: `createKmsUserDecryptEip712()`, `createKmsDelegatedUserDecryptEip712()`, `createKmsEip712Domain()`, `createCoprocessorEip712Domain()`, `verifyKmsUserDecryptEip712()`
Build the decrypt-permit typed data and domains without signing, and verify a signature. (Casing note: names mix `EIP712` and `Eip712` in code; `docs/api-reference.md` normalizes to `EIP712`, which does not match the real exports.)

**Decryption proof** `[low]`
SDK term: `PublicDecryptionProof`
The KMS public decryption proof, returned by `decryptPublicValuesWithSignatures()`. Includes the KMS signatures, associated metadata, and the context needed for on-chain verification.

**ExtraData** `[low]`
SDK term: `extraData` (e.g. `KmsUserDecryptEip712Message.extraData`, `PublicDecryptParameters.extraData`)
A `bytes` field included in EIP-712 permits and Relayer requests. It serves as an opaque context parameter that binds a decryption request to a specific KMS signer set. In standard operations, use `"0x00"`. In the SDK, extraData is auto-fetched — developers don't need to provide it manually.

---

## 4. Keys and cryptographic material

Protocol-level cryptographic key material. (The SDK's opaque key types —
`TransportKeyPair`, `TkmsPrivateKey` — live in §3 since they are part of the SDK
surface.)

**KMS key**
The master secret key of the FHEVM protocol, used to decrypt all ciphertexts. This key is never held by a single entity — it is split into shares distributed across KMS nodes via Multi-Party Computation (MPC). The protocol's security relies on the assumption that no quorum of KMS operators colludes. Not to be confused with the transport key pair.

**FHE encryption key**
The public key used across the Zama Protocol to encrypt all confidential inputs and contract state. This shared key enables composability between users and smart contracts operating on encrypted data. In the SDK, fetched via `fetchFheEncryptionKeyBytes()` (see §3, "Public key fetch").
*Source: fhevm-whitepaperDeprecated terms (do not use): FHEVM public key, TFHE public key, Zama public key, global FHE key, GlobalFhePkeParams*

**CRS** (Common Reference String)
A piece of cryptographic data necessary for the security of zero-knowledge proofs. The CRS is generated in advance via a ceremony by/for the KMS and shared between all clients and the server. A CRS can be reused for multiple encryptions with the same parameters. In the SDK, the CRS is fetched alongside the FHE encryption key via `fetchFheEncryptionKeyBytes()`.
*Source: TFHE-rs docs*

**TKMS ML KEM Key pair** `[wasm]`
SDK term: `TransportKeyPair`
This is an ML KEM 512 Key pair. 
ML-KEM = Module Lattice Key Encapsulation Mechanism. The user's private key used to decrypt KMS signcrypted shares during decryption — the private half of the transport key pair. It secures the communication channel between the KMS and the entity requesting decryption (the user themselves or a delegate, e.g. a bank decrypting on behalf of a user); it is a communication key, not the protocol's master decryption key. Hidden inside the opaque `TransportKeyPair` object and never accessible to application code.
*Deprecated terms (do not use): KMS private key*

---

## 5. Protocol internals

Lower-level protocol components, mechanisms, and on-chain data structures within
the Zama Protocol. These are concepts of the protocol itself, not the SDK API
(the SDK actions that *read* them live in §3).

### Infrastructure

**KMS node**
A node participating in the distributed Key Management Service that holds a share of the secret key and executes MPC protocols for key generation and decryption.
*Source: fhevm-whitepaper*

**KMS core**
The core cryptographic engine within a KMS node that holds the key share and executes MPC protocols. It is isolated from network communication (handled by the KMS connector) and runs inside an AWS Nitro Enclave for integrity and confidentiality.

**KMS connector**
The component within a KMS node that handles communication with the Gateway. It receives decryption requests forwarded by the Gateway, processes them using the node's key share, and returns signcrypted shares.

**MPC threshold**
The minimum number of KMS nodes that must participate to complete a threshold operation (key generation, decryption). For example, a threshold of 9 out of 13 means any 9 KMS nodes can collectively produce a valid decryption, but no subset of 8 or fewer can. In the SDK, this is exposed as `kmsSignerThreshold` on `KmsVerifierContractData`.

### Smart contracts

**KMS Verifier**
A smart contract deployed on each Host Chain (`KMSVerifier.sol`) that stores the list of authorized KMS signer addresses and the threshold required to validate a decryption response. During public decryption, the SDK uses this contract to verify that the response was signed by a sufficient quorum of KMS nodes. The contract also provides the EIP-712 domain and the Gateway chain ID. In the SDK, read via `readKmsVerifierContractData()` (`[low]`).
*Source: `KMSVerifier.sol`, SDK type: `KmsVerifierContractData`*

**Input Verifier**
A smart contract deployed on each Host Chain (`InputVerifier.sol`) that verifies encrypted inputs from users. It checks ZK proofs and coprocessor signatures to ensure that ciphertexts were correctly generated. In the SDK, read via `readInputVerifierContractData()` (`[low]`).
*Source: `InputVerifier.sol`, SDK type: `InputVerifierContractData`*

### Signers

**KMS signer**
A KMS participant that contributes cryptographic signatures or decryption shares as part of the threshold decryption process. A quorum (for example 9 out of 13 nodes) must cooperate to complete operations.
*Source: `KMSVerifier.sol`*

**Coprocessor signer**
A wallet address that signs the coprocessor's result during the inputProof verification mechanism. The coprocessor signer produces an EIP-712 signature attesting that it verified and processed the user's encrypted input correctly.
*Source: `InputVerifier.sol`*

### Execution model

**Symbolic execution**
The execution model used by FHEVM smart contracts where encrypted operations are represented symbolically using encrypted values (handles). The EVM emits events describing the operations, and coprocessors later perform the actual FHE computations on ciphertexts.

**FHE gas**
A resource accounting mechanism that limits the amount of FHE computation requested by a transaction. It ensures that symbolic FHE operations emitted on-chain remain within the processing capacity of coprocessors.
*Source: HCULimit.solDeprecated terms (do not use): fheGas, fhe-gas, HCU (homomorphic complexity unit)*

---

## 6. General cryptography terms

Commonly used cryptographic terms.
Not defined by Zama, but included here for reference.

**FHE (Fully Homomorphic Encryption)**
A cryptographic method allowing arbitrary, complex, and unlimited computations (both addition and multiplication) on encrypted data (ciphertext) without decrypting it first. The result, when decrypted, matches the output of operations performed on plaintext. This ensures data remains secure during processing, enabling privacy-preserving cloud computing.

**Ciphertext**
The unreadable, scrambled output produced when plaintext is encrypted using a cryptographic algorithm and a key. In the Zama Protocol context: the encrypted representation of a plaintext value produced using the public FHE encryption key. Ciphertexts are stored off-chain by coprocessors and referenced by handles, which can be considered as unique pointers to the ciphertexts. There is a deterministic 1:1 mapping between an encrypted value and a ciphertext.

**Plaintext**
Unencrypted, human-readable data that serves as the original input for an encryption algorithm or the final output of a decryption process.

**Cleartext**
Unencrypted, human-readable data that is stored or transmitted without any cryptographic protection. Unlike plaintext (which may be intended for encryption), cleartext is generally not intended to be encrypted.

**Trivial encryption**
In general cryptography: an encryption scheme that offers no security. In the FHEVM context: a special encryption operation that produces a valid ciphertext without requiring a zero-knowledge proof, and without protecting the plaintext. Typically used for values that do not require secrecy (for example, constants).

**Public key** *(in homomorphic encryption)*
Used by the data owner to encrypt data before sending it to a server for computation.

**Secret key** *(in homomorphic encryption)*
Kept securely by the data owner to decrypt the final results returned by the server.

**Evaluation key** *(in homomorphic encryption)*
A public key, often derived from the secret key, that allows the server to perform homomorphic operations (e.g. addition/multiplication) on ciphertexts without needing to decrypt them.

---

## 7. General computing terms

**Handle** *(general computing)*
In computer programming, a handle is an abstract reference to a resource that is used when application software references blocks of memory or objects that are managed by another system like a database or an operating system. In the FHEVM context, "handle" is the `FHE.sol` / whitepaper term for what the SDK calls an `EncryptedValue` — see **Encrypted value** in §3 (3c).
*Source: Wikipedia*
