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
| **encrypted value ID** | Identity of one persistent encrypted value: recomputed from (domain, encrypted value account authority, encrypted value label). It is the PDA seed of the encrypted value account and is never stored. | `value_key` | — |
| **encrypted value account** | The canonical PDA account of one encrypted value ID. Holds the current handle, the subject list, and the MMR of sealed history. `EncryptedValue` in `zama_solana_acl` — the crate spells the struct without the `Account` suffix. Never shortened by dropping the adjective: an account holding a value describes every SPL token account, and what distinguishes this one is that the value it holds is *encrypted* and carries its own ACL state. | lineage account, value account | ACL storage entry |
| **domain** | First component of an encrypted value ID: the app-level scope a value belongs to, such as a confidential token mint. It is not a signer — it names which app's namespace the value lives in, and the same authority under two domains addresses two different values. Typed as `zama_fhe::Domain` in the SDK, so it cannot be swapped with the authority it scopes. | `acl_domain_key` | ACL contract instance |
| **encrypted value account authority** | Second component of an encrypted value ID: the account that controls the value. It must sign every `fhe_execute` output that creates the value, updates its handle, or replaces its subject list, and it must also sign out-of-band subject-list mutation (`allow_subjects` / `remove_subject`) — decrypt subjects are not co-admins (INVARIANT #11; fhevm-internal#1862 #13). Enforcement is by address, not by comparing a stored field: the signer must equal the authority declared in the execution (`assert_output_acl_metadata`), and the account written to must be the PDA rederived from that declared triple (`EncryptedValuePdaMismatch`) — so on update the signer is necessarily the stored authority, because the address encodes it. For a token balance it is the token account itself (rotated via confidential-token's `allow_token_account_subjects` / `remove_token_account_subject` CPI wrappers). Not called just "account": that named the attachment and hid the authority. The SDK spells the execution-wide one `ExecutionEncryptedValueAccountAuthority`, the same key an output declares — the `Execution` prefix marks the scope, not a second concept. | `app_account`, `EncryptedValue.account`, `app_authority`, `ExecutionAppAuthority` | the contract holding the value |
| **encrypted value label** | Third component of an encrypted value ID: the app-chosen discriminator naming *which* encrypted value of the authority this is — `balance`, `total_supply`, `burned_amount`. Bare "label" says only that it is 32 bytes, which is true of all three components. The struct field stays `label` where the enclosing type already says `EncryptedValue`; it carries the full name wherever it stands alone. An app naming a *specific* encrypted value substitutes that value for the generic word rather than stacking both: `encrypted_balance_label()`, not `balance_encrypted_value_label()`. The label's own bytes are PDA seeds and never change when its name does. | SDK `namespace` | the storage slot's name |
| **subject** | A pubkey in an encrypted value account's subject list: it may decrypt that value and use it in compute. Membership is flat for those rights — there are no role flags among subjects — but subjects are not ACL admins: extending or shrinking the list requires the encrypted value account authority (`allow_subjects` / `remove_subject`), not `has_subject`. `make_handle_public` remains subject-gated (INVARIANT #11b). | — | ACL `account` address |
| **compute subject** | The subject an execution runs *as*, and a subject in the sense above — not a second meaning of the word. It is the signer that must be an allowed subject on every persistent input the execution reads (a verified input pins it to the attested contract instead), the key the per-slot HCU meter is charged to, and part of the handle-derivation preimage. Not called "caller": that names only the invocation and hides the ACL check, which is the half that matters. Nor is it necessarily a program — `fhe_execute` enforces no CPI (see the pinning rule in `fhe_execute/preflight.rs`). | — | `msg.sender` of the dapp |

## Execution

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **execution** | One atomic `fhe_execute` invocation: its steps, dictionary, and outputs, validated as a whole before any state is touched. `zama_fhe::FheExecution` in the SDK. Not called a batch: the steps are not independent items grouped for efficiency, each reads what the one before it produced. | batch, frame, plan | one FHEVMExecutor transaction |
| **walk** | The in-order traversal of an execution's steps that actually computes: it runs after preflight has validated the whole execution, and it is the only phase that touches state. | — | — |
| **step** | One position in an execution's walk. | — | — |
| **op** | The FHE operation kind a step performs (add, mul, select…). | — | FHE library op |
| **dictionary** | The interned list of 32-byte values inside an execution's wire data; steps reference entries by index. Deliberately an untyped `Vec<[u8;32]>`: entries are interned across roles (handles, cleartexts, pubkeys). | pool | — |
| **fhe_execute** | The host instruction that runs one execution. | `fhe_eval` | `FHEVMExecutor` ops |
| **preflight** | The whole-execution validation pass (indexes, accounts, types, costs) that runs before the walk. | — | — |
| **transient** | Usable only inside the carrying execution; leaves no persistent grant. | — | `allowTransient` |
| **builder identity** | SDK-only. [`FheExecution::build`] gives each invocation a fresh invariant lifetime `'id`, and every value that builder returns carries it, so feeding one builder's value to another does not compile (`compile_fail` doctest on [`FheExecution::build`]). The field is a `PhantomData`: nothing on the wire, nothing read at runtime. It replaces a runtime scope tag, which SBF cannot have because on-chain code cannot have writable statics. Spelled `'id` after the same idiom in GhostCell, where the pattern is also called a brand. | runtime scope tag, `'brand` | — |
| **stored value** | Wire name of an operand read from an encrypted value account, and of an output written to one: `FheExecuteOperand::StoredValue`, `FheExecuteOutput::StoredValue`. Names what the slot is, not why it was admitted. Also the SDK type `zama_fhe::StoredValue<T>`, which — unlike a builder's transient values — belongs to no builder. | `AllowedPersistent` | ACL `persistAllowed` entry |
| **earlier step** | Wire name of an operand that reads what an earlier step of the same execution produced: `FheExecuteOperand::EarlierStep`. The index must point backwards, which is what "earlier" pins — a forward reference is rejected. It says nothing about lifetime: a step whose output was persisted is readable this way too. | `AllowedLocal` | `allowTransient` value |
| **persistent** | Outlives the execution: written to an encrypted value account with its own subject list. | durable | ACL `persistAllowed` |
| **create / created-public** | An execution output binding a new persistent value; created-public seals it publicly decryptable at creation. | birth, born-public | — |
| **update** | Replacing a persistent value's handle. The declared `previous_state` must echo the stored handle and subject list exactly, which pins *what is being replaced* so an indexer can follow the chain from instruction data alone. It does not pin the new audience: an update may hand the value a different subject list at the same time. | supersede, rotation | — |
| **HCU** | Homomorphic compute unit: the metering unit of FHE work. | — | HCU |

## Entry and exit trust

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **input proof** | The ZK bundle a user submits alongside an external encrypted input. | — | `inputProof` (`FHE.fromExternal`) |
| **attestation** | The coprocessor-committee-signed statement that a submitted input verified (names the calling app and host chain id). | — | `CiphertextVerification` |
| **certificate** | A KMS-threshold-signed statement, e.g. the public-decrypt result binding cleartext to handle and context. | — | KMS signature bundle |
| **proof** | A cryptographic proof and nothing else (MMR inclusion proof, ZK proof). Never used for signed statements — those are attestations or certificates. | — | — |
| **KmsContext** | The on-chain account naming one KMS committee's signer set and threshold; certificates bind to the context that issued them. | — | gateway KMS context |
| **MMR** | The append-only Merkle mountain range inside an encrypted value account sealing its handle history; supports inclusion proofs for replaced or public handles. | — | — |

## Off-chain

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **decoded op records** | The typed structs the listener decodes from an execution's instruction data, one per step, fed to the coprocessor. They are not on-chain events. | `Fhe*Event` structs | `FheAdd`… events |
| **reconstruction** | The listener's re-derivation of output handles from raw transaction bytes, using the program's own derivation functions. | — | — |
| **proof service** | The off-chain service that ingests confirmed blocks and serves MMR inclusion proofs. A failure can stall a decrypt, but the KMS re-verifies every proof, so the service cannot authorize one. | — | — |
| **crank** | A call anyone may make that advances work an earlier transaction could not finish — the demo's address-lookup-table lifecycle is driven this way, since deactivation has to wait out a cooldown. Retrying is the point: a crank that throws leaves the work for the next one rather than recording it as done. | — | — |

## Banned names

- **"lookup table"** for anything that is not a Solana address lookup table
  (ALT). The collision with the native ALT program is guaranteed confusion.
- Any synonym in a "Replaces" cell above.
