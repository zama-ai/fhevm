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
| **encrypted value account** | The canonical PDA account of one persistent encrypted value, at `["encrypted-value", program, encrypted value account authority, scope, encrypted value label]` — four fixed-width seeds after the tag, no intermediate hash, so the identity is the seed list itself and is never stored or derived elsewhere. Holds the current handle and the MMR peaks of sealed history; it stores nothing about who may decrypt. `EncryptedValue` in `zama_solana_acl` — the crate spells the struct without the `Account` suffix. Never shortened by dropping the adjective: an account holding a value describes every SPL token account, and what distinguishes this one is that the value it holds is *encrypted* and carries its own ACL state. | lineage account, value account, encrypted value ID (the former derived seed) | ACL storage entry |
| **application** | The pair `(program, scope)`: the program a value belongs to and the scope that program declared within itself. `AppScope` in code. It is the identity HCU metering charges, the deny list names, and a permit scopes to. Trustworthy only as the pair: the host verifies `program` on every write (below), so another program cannot forge that half, and `scope` means what `program` says it means. | domain, `acl_domain_key`, compute subject (as the metered identity) | ACL contract instance / `msg.sender` of the dapp |
| **program** | First seed of an encrypted value account after the tag: the application program the value belongs to. Verified, never declared: on every write the encrypted value account authority must be a PDA of `program`, proven by the declared seeds (`assert_authority_is_program_pda`, `EncryptedValueAuthorityNotProgramPda`). | — | the contract holding the value |
| **scope** | Third seed: the program-declared partition within `program` — the mint for the token program, a single constant for a program with one namespace. Meaningful only as half of the application pair; never an authority. Also the unit a permit narrows to (`allowedScopes`, at most seven `(program, scope)` pairs; empty is permissive). | domain, `domainKey`, `allowedAclDomainKeys` | ACL contract instance |
| **encrypted value account authority** | Second seed: the account that controls the value. It must sign every `fhe_execute` output that creates the value or updates its handle, and `make_handle_public`; it is a PDA of `program`, so only that program can sign for it. For a token balance it is the token account itself. Reading a value into a computation is admitted by this signature and nothing else — there is no separate compute identity. Enforcement is by address, not by comparing a stored field: the account written to must be the PDA rederived from the declared seeds (`EncryptedValuePdaMismatch`), so on update the signer is necessarily the stored authority, because the address encodes it. The SDK spells the execution-wide one `ExecutionEncryptedValueAccountAuthority`, the same key an output declares — the `Execution` prefix marks the scope, not a second concept. | `app_account`, `EncryptedValue.account`, `app_authority`, `ExecutionAppAuthority`, app context, compute signer | the contract holding the value |
| **encrypted value label** | Fourth seed: the app-chosen discriminator naming *which* encrypted value of the authority this is — `balance`, `total_supply`, `burned_amount`. Bare "label" says only that it is 32 bytes, which is true of every seed. The struct field stays `label` where the enclosing type already says `EncryptedValue`; it carries the full name wherever it stands alone. An app naming a *specific* encrypted value substitutes that value for the generic word rather than stacking both: `encrypted_balance_label()`, not `balance_encrypted_value_label()`. | SDK `namespace` | the storage slot's name |
| **encrypted value ID** | SDK-only: `zama_fhe::EncryptedValueId`, the four identity seeds of an encrypted value account plus the address they derive, built once (`EncryptedValueId::new(app, authority, label)`) so the derivation syscall is paid where the id is built. It is the seeds themselves, not a hash of them: nothing derives a key from the components, and the account stores them in the clear. | `value_key`, the former sha256-derived seed | — |
| **allow** | One decrypt permission: `key` may decrypt `handle`. Allows are declared on the write that installs the handle (`PersistentOutput::allow`, in list order) and sealed as leaves; the account keeps no list of them, and nothing adds or removes one afterwards. A key so allowed is a **viewer**: it may decrypt, and nothing else — it is not an admin and not a compute identity. | subject, subject list, `allow_subjects`, `remove_subject`, `persistAllowed` entry | `FHE.allow`, `isAllowed` |
| **allowed key** | The viewer a user-decrypt entry names: the permit's user by default, or the delegator on a delegated entry. Bound into the allow leaf the connector verifies. | subject | the `account` of `isAllowed` |

## Confidential token burn lifecycle

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **pending burn** | The one unsettled burn allowed for a confidential token account. Its `PendingBurn` PDA is derived from `(mint, token account)`. A second burn is rejected until the pending burn is settled. Parallel burns may use separate app-owned token accounts; multiple pending burns per token account are deferred. | burn lane | — |
| **redeem** | Settle a pending burn by verifying its public-decryption certificate and proof, transferring the certified underlying amount from the wrapper vault, and closing `PendingBurn`. | claim burn | withdraw |
| **cancel** | Settle a pending burn without an underlying payout by restoring the encrypted balance and encrypted total supply and closing `PendingBurn`. Only the token account owner can cancel, and the burned handle must still be current. This term does not describe `BatchStatus::Canceled`. | recover | — |
| **settled** | Terminal description for a pending burn that was either redeemed or cancelled. It is an adjective describing the burn lifecycle, not an instruction name or the batcher's `BatchStatus::Settled`. | finalized, recovered | — |

## Confidential batcher lifecycle

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **`BatchStatus::Settled`** | The batch payout was received and participant claims are open. It does not describe pending-burn settlement. | — | — |
| **`BatchStatus::Canceled`** | The KMS-certified batch total was zero, so there is no payout to claim and no burned value to restore. It is distinct from cancelling a pending burn. | — | — |
| **refunding** | `BatchStatus::Refunding`: the dispatched pending burn was cancelled and existing participants may retrieve their recorded joins. | — | — |

## Execution

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **execution** | One atomic `fhe_execute` invocation: its steps, dictionary, and outputs, validated as a whole before any state is touched. `zama_fhe::FheExecution` in the SDK. Not called a batch: the steps are not independent items grouped for efficiency, each reads what the one before it produced. | batch, frame, plan | one FHEVMExecutor transaction |
| **walk** | The in-order traversal of an execution's steps that actually computes: it runs after preflight has validated the whole execution, and it is the only phase that touches state. | — | — |
| **step** | One position in an execution's walk. | — | — |
| **op** | The FHE operation kind a step performs (add, mul, select…). | — | FHE library op |
| **dictionary** | The interned list of 32-byte values inside an execution's wire data; steps reference entries by index. Deliberately an untyped `Vec<[u8;32]>`: entries are interned across roles (handles, cleartexts, pubkeys, PDA seeds). | pool | — |
| **fhe_execute** | The host instruction that runs one execution. | `fhe_eval`, eval | `FHEVMExecutor` ops |
| **preflight** | The whole-execution validation pass (indexes, accounts, types, costs) that runs before the walk. | — | — |
| **transient** | Usable only inside the carrying execution; leaves no persistent grant. | — | `allowTransient` |
| **builder identity** | SDK-only. [`FheExecution::build`] gives each invocation a fresh invariant lifetime `'id`, and every value that builder returns carries it, so feeding one builder's value to another does not compile (`compile_fail` doctest on [`FheExecution::build`]). The field is a `PhantomData`: nothing on the wire, nothing read at runtime. It replaces a runtime scope tag, which SBF cannot have because on-chain code cannot have writable statics. Spelled `'id` after the same idiom in GhostCell, where the pattern is also called a brand. | runtime scope tag, `'brand` | — |
| **stored value** | Wire name of an operand read from an encrypted value account, and of an output written to one: `FheExecuteOperand::StoredValue`, `FheExecuteOutput::StoredValue`. Names what the slot is, not why it was admitted. Also the SDK type `zama_fhe::StoredValue<T>`, which — unlike a builder's transient values — belongs to no builder. | `AllowedPersistent` | ACL `persistAllowed` entry |
| **earlier step** | Wire name of an operand that reads what an earlier step of the same execution produced: `FheExecuteOperand::EarlierStep`. The index must point backwards, which is what "earlier" pins — a forward reference is rejected. It says nothing about lifetime: a step whose output was persisted is readable this way too. | `AllowedLocal` | `allowTransient` value |
| **persistent** | Outlives the execution: written to an encrypted value account, with its allows sealed on the write. | durable | ACL `persistAllowed` |
| **create / created-public** | An execution output binding a new persistent value; created-public seals it publicly decryptable at creation. | birth, born-public | — |
| **update** | Replacing a persistent value's handle. The declared `previous_handle` must echo the stored handle exactly, which pins *what is being replaced* so an indexer can follow the chain from instruction data alone. The new handle carries its own allows; the old handle's allows stay sealed and keep authorizing it. | supersede, rotation | — |
| **rand nonce** | The host's `RandNonce` singleton (`["rand-nonce"]`), a counter every execution with a rand step must pass and advance; its value is bound into every rand seed, so two executions can never derive the same seed. | persistent-write anchor | `counterRand` |
| **HCU** | Homomorphic compute unit: the metering unit of FHE work. | — | HCU |

## Entry and exit trust

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **input proof** | The ZK bundle a user submits alongside an external encrypted input. | — | `inputProof` (`FHE.fromExternal`) |
| **attestation** | The coprocessor-committee-signed statement that a submitted input verified (names the calling program and host chain id). | — | `CiphertextVerification` |
| **certificate** | A KMS-threshold-signed statement, e.g. the public-decrypt result binding cleartext to handle and context. | — | KMS signature bundle |
| **proof** | A cryptographic proof and nothing else (MMR inclusion proof, ZK proof). Never used for signed statements — those are attestations or certificates. | — | — |
| **KmsContext** | The on-chain account naming one KMS committee's signer set and threshold; certificates bind to the context that issued them. | — | gateway KMS context |
| **MMR** | The append-only Merkle mountain range inside an encrypted value account sealing every allow and every public sealing as a leaf; the account holds only the peaks. Every decrypt is authorized by an inclusion proof against them. | — | — |
| **leaf** | One sealed fact in an encrypted value account's MMR: an allow (`HistoricalAccessLeaf{encrypted_value_account, leaf_index, handle, key}`) or a public sealing (`PublicDecryptLeaf{encrypted_value_account, leaf_index, handle}`), keccak-committed with a version prefix. Leaves are appended in write order: the allows of the installed handle in list order, then its public leaf. | — | — |
| **leaf record** | The coprocessors' record of every leaf the host sealed, kept by the host listener next to the compute rows it was derived with and served over `POST /v1/solana/leaf-proofs` behind an API key. A source of proofs, never of decisions: the connector verifies each proof against the peaks it read on chain. | proof service, `solana-proof-service` | — |
| **disclosed value kind** | The token field identity carried by `DisclosedValueKind`. It binds a disclosure to the expected encrypted value account authority, encrypted value label, and canonical account address; it is not caller-supplied descriptive metadata. | — | — |

## Off-chain

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **decoded op records** | The typed structs the listener decodes from an execution's instruction data, one per step, fed to the coprocessor. They are not on-chain events. | `Fhe*Event` structs | `FheAdd`… events |
| **reconstruction** | The listener's re-derivation of output handles and leaves from raw transaction bytes, using the program's own derivation functions. | — | — |
| **crank** | A call anyone may make that advances work an earlier transaction could not finish — the demo's address-lookup-table lifecycle is driven this way, since deactivation has to wait out a cooldown. Retrying is the point: a crank that throws leaves the work for the next one rather than recording it as done. | — | — |

## Banned names

- **"lookup table"** for anything that is not a Solana address lookup table
  (ALT). The collision with the native ALT program is guaranteed confusion.
- **"subject"**, **"compute subject"**, **"compute signer"**: an encrypted
  value account keeps no list of who may decrypt it, and reading a value is
  admitted by its authority's signature. Say *allow* / *viewer* / *allowed
  key*, or *encrypted value account authority*.
- **"domain"** in the ACL sense (`acl_domain_key`, "mint domain"): say
  *application* or *scope*. EIP-712 signing domains and hash-domain separation
  keep the word.
- **"proof service"**: the leaf record lives in the host listener.
- **`value_key`**, or any "derived" encrypted value id: the identity is the
  seed list; nothing hashes it into a key.
- Any synonym in a "Replaces" cell above.
