# Glossary

This file is normative. Code, docs, IDL, tests, and commit messages use these
terms and no synonyms. Each entry gives the definition first; where a term
replaces an older name, the old name is listed so reviewers can grep for
stragglers. The EVM column names the concept's equivalent in the EVM fhevm
stack where one exists.

## Core objects

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **handle** | 32-byte opaque identifier of one ciphertext, derived on-chain during execution. The high bit of its embedded chain id marks the chain type (Solana = set). | — | `bytes32` handle |
| **encrypted value ID** | Identity of one persistent encrypted value: recomputed from (domain, account, label). It is the PDA seed of the encrypted value account and is never stored. | `value_key` | — |
| **encrypted value account** | The canonical PDA account of one encrypted value ID. Holds the current handle, the subject list, and the MMR of sealed history. `EncryptedValueAccount` in `zama_solana_acl`. Never shortened by dropping the adjective: an account holding a value describes every SPL token account, and what distinguishes this one is that the value it holds is *encrypted* and carries its own ACL state. | lineage account, value account | ACL storage entry |
| **domain** | First component of an encrypted value ID: the ACL domain a value belongs to. Typed as `zama_fhe::Domain` in the SDK, so it cannot be swapped with the account it scopes. | `acl_domain_key` | — |
| **account** (ID component) | Second component of an encrypted value ID: the app-level account the value is attached to. | `app_account` | — |
| **label** | Third component of an encrypted value ID: the app-chosen discriminator. | `encrypted_value_label`, SDK `namespace` | — |
| **subject** | A pubkey granted access on an encrypted value account. | — | ACL `account` address |
| **compute subject** | The account that is metered for HCU and named as the caller identity of an execution. | — | `msg.sender` of the dapp |

## Execution

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **execution** | One atomic `fhe_execute` invocation: its steps, dictionary, and outputs, validated as a whole before any state is touched. `zama_fhe::FheExecution` in the SDK. Not called a batch: the steps are not independent items grouped for efficiency, each reads what the one before it produced. | batch, frame, plan | one FHEVMExecutor transaction |
| **step** | One position in an execution's walk. | — | — |
| **op** | The FHE operation kind a step performs (add, mul, select…). | — | FHE library op |
| **dictionary** | The interned list of 32-byte values inside an execution's wire data; steps reference entries by index. Deliberately an untyped `Vec<[u8;32]>`: entries are interned across roles (handles, cleartexts, pubkeys). | pool | — |
| **fhe_execute** | The host instruction that runs one execution. | `fhe_eval` | `FHEVMExecutor` ops |
| **preflight** | The whole-execution validation pass (indexes, accounts, types, costs) that runs before the walk. | — | — |
| **transient** | Usable only inside the carrying execution; leaves no persistent grant. | — | `allowTransient` |
| **stored value** | Wire name of an operand read from an encrypted value account, and of an output written to one: `FheExecuteOperand::StoredValue`, `FheExecuteOutput::StoredValue`. Names what the slot is, not why it was admitted. Also the SDK type `zama_fhe::StoredValue<T>`, which — unlike a builder's transient values — belongs to no builder. | `AllowedPersistent` | ACL `persistAllowed` entry |
| **earlier step** | Wire name of an operand that reads the output of an earlier step of the same execution: `FheExecuteOperand::EarlierStep`. The matching output is `FheExecuteOutput::Transient`. | `AllowedLocal` | `allowTransient` value |
| **persistent** | Outlives the execution: written to an encrypted value account with its own subject list. | durable | ACL `persistAllowed` |
| **create / created-public** | An execution output binding a new persistent value; created-public seals it publicly decryptable at creation. | birth, born-public | — |
| **update** | Replacing a persistent value's handle, echoing the exact previous handle and subject list. | supersede, rotation | — |
| **HCU** | Homomorphic compute unit: the metering unit of FHE work. | — | HCU |

## Entry and exit trust

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **input proof** | The ZK bundle a user submits alongside an external encrypted input. | — | `inputProof` (`FHE.fromExternal`) |
| **attestation** | The coprocessor-committee-signed statement that a submitted input verified (names the calling app and host chain id). | — | `CiphertextVerification` |
| **certificate** | A KMS-threshold-signed statement, e.g. the public-decrypt result binding cleartext to handle and context. | — | KMS signature bundle |
| **proof** | A cryptographic proof and nothing else (MMR inclusion proof, ZK proof). Never used for signed statements — those are attestations or certificates. | — | — |
| **KmsContext** | The on-chain account naming one KMS committee's signer set and threshold; certificates bind to the context that issued them. | — | gateway KMS context |
| **MMR** | The append-only Merkle mountain range inside an encrypted value account sealing its handle history; supports inclusion proofs for superseded or public handles. | — | — |

## Off-chain

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **decoded op records** | The typed structs the listener decodes from an execution's instruction data, one per step, fed to the coprocessor. They are not on-chain events. | `Fhe*Event` structs | `FheAdd`… events |
| **reconstruction** | The listener's re-derivation of output handles from raw transaction bytes, using the program's own derivation functions. | — | — |
| **proof service** | The off-chain service that ingests confirmed blocks and serves MMR inclusion proofs. A failure can stall a decrypt, but the KMS re-verifies every proof, so the service cannot authorize one. | — | — |

## Banned names

- **"lookup table"** for anything that is not a Solana address lookup table
  (ALT). The collision with the native ALT program is guaranteed confusion.
- Any synonym in a "Replaces" cell above.
