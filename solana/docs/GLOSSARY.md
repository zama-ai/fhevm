# Glossary

Last synced: 2026-09-11.

This file is normative. Code, docs, IDL, tests, and commit messages use these
terms and no synonyms. Each entry gives the definition first; where a term
replaces an older name, the old name is listed so reviewers can grep for
stragglers. The EVM column names the concept's equivalent in the EVM fhevm
stack where one exists.

## Core objects

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **handle** | 32-byte opaque identifier of one ciphertext, derived on-chain during execution. The high bit of its embedded chain id marks the chain type (Solana = set). | — | `bytes32` handle |
| **encrypted store** | Host-owned `EncryptedStore` PDA at `["encrypted-state", program, authority, scope]`. Holds up to 32 keyed current handles and one shared decryption-history MMR. Creating it validates the authority as a PDA of the program; subsequent access validates canonical identity and the required signer. | encrypted value account, `EncryptedValue`, lineage account | application storage plus ACL history |
| **application** | The pair `(program, scope)`: the program a value belongs to and the scope that program declared within itself. `AppScope` in code. It is the identity HCU metering charges, the deny list names, and a permit scopes to. Trustworthy only as the pair: the host verifies `program` on every write (below), so another program cannot forge that half, and `scope` means what `program` says it means. | domain, `acl_domain_key`, compute subject (as the metered identity) | ACL contract instance / `msg.sender` of the dapp |
| **program** | The application program in a Store identity. Its control of the authority PDA is proved when the Store is created. | — | application contract |
| **scope** | The program-declared instance partition, paired with `program`: a confidential mint for token state, a batch for contribution state. A permit can restrict `(program, scope)` pairs. Scope is not a signing credential. | domain, `domainKey`, `allowedAclDomainKeys` | application instance |
| **authority** | The PDA controlling an encrypted store. Its signature authorizes Store reads for computation, Store outputs and current-slot publication. Token state uses the token-account PDA; contribution state uses each participant’s JoinRecord PDA. | encrypted value account authority, `ExecutionEncryptedValueAccountAuthority` | application contract |
| **slot key** | Application-defined 32-byte key locating a current handle inside Store; never a PDA seed. Examples: balance, total supply, burned amount, contribution. | encrypted value label | storage slot |
| **StoreId** | Builder identity containing `(program, authority, scope)` and its canonical Store address. A slot is separately identified by its key. | `EncryptedValueId` | — |
| **allow** | A historical decrypt permission binding Store, handle and allowed key. Fresh Store outputs may append allows without occupying a slot. Later permission changes for history-only handles are deferred to #2007. An allow does not itself admit compute in the current Solana model. | subject, subject list, `allow_subjects`, `remove_subject`, `persistAllowed` entry | `FHE.allow`, with re-sharing deferred on Solana |
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
| **execution** | One `fhe_execute` invocation containing an ordered list of steps, a dictionary, Store effects and selected returned results. Steps may consume external inputs or earlier results; they need not depend on the immediately preceding step. Multiple executions may share one transaction. `zama_fhe::FheExecution` in the SDK. | batch, frame, plan | a sequence of FHEVMExecutor calls |
| **walk** | The in-order evaluation of an execution's steps after preflight. It checks operand types, permissions, origins and HCU, and records produced results in the transient store; Store effects follow the steps. | — | — |
| **step** | One position in an execution's walk. | — | — |
| **op** | The FHE operation kind a step performs (add, mul, select…). | — | FHE library op |
| **dictionary** | The interned list of 32-byte values inside an execution's wire data; steps reference entries by index. Deliberately an untyped `Vec<[u8;32]>`: entries are interned across roles (handles, cleartexts, pubkeys, PDA seeds). | pool | — |
| **fhe_execute** | The host instruction that runs one execution. | `fhe_eval`, eval | `FHEVMExecutor` ops |
| **preflight** | Validates execution indexes, account identities, signers and effects before the walk. Operand types, origins and HCU are checked during the walk. | — | — |
| **transient** | A result recorded in the transaction’s transient store. Its producing Store may use it across calls; another Store needs an explicit grant. `EarlierStep` references a current-call occurrence. Returning bytes grants no permission. | — | local FHE result |
| **builder identity** | SDK-only. [`FheExecution::build`] gives each invocation a fresh invariant lifetime `'id`, and every value that builder returns carries it, so feeding one builder's value to another does not compile (`compile_fail` doctest on [`FheExecution::build`]). The field is a `PhantomData`: nothing on the wire, nothing read at runtime. It replaces a runtime scope tag, which SBF cannot have because on-chain code cannot have writable statics. Spelled `'id` after the same idiom in GhostCell, where the pattern is also called a brand. | runtime scope tag, `'brand` | — |
| **Store slot** | `FheExecuteOperand::StoreSlot` reads a key and checks the expected handle; `FheExecuteEffect` optionally writes a slot and appends permissions after all steps. Builder `Store.get`/`Store.set` describe these operations; `FheHandle<T>` carries a validated typed handle. | `StoredValue`, `AllowedPersistent` | application storage |
| **earlier step** | Wire name of an operand that reads what an earlier step of the same execution produced: `FheExecuteOperand::EarlierStep`. The index must point backwards, which is what "earlier" pins — a forward reference is rejected. It says nothing about lifetime: a step whose output was persisted is readable this way too. | `AllowedLocal` | `allowTransient` value |
| **persistent** | Store or decrypt history that outlives the execution. A historical result can remain decryptable without a current slot. | durable | persistent ACL permission |
| **create / created-public** | Create allocates empty encrypted store separately from computation. A fresh execution result can be made public with `make_public`, with or without a slot write. | birth, born-public | — |
| **update** | Replace a Store slot using an expected previous handle and previous shared leaf count. Failure reverts the transaction. Previously sealed permissions remain valid. | supersede, rotation | storage update |
| **rand nonce** | The host's `RandNonce` singleton (`["rand-nonce"]`), a counter every execution with a rand step must pass and advance; its value is bound into every rand seed, so two executions can never derive the same seed. | persistent-write anchor | `counterRand` |
| **HCU** | Homomorphic compute unit: the metering unit of FHE work. | — | HCU |

| **result grant** | Transaction-local compute permission for an exact produced handle and consumer Store. Held in shared host-owned transient store; the consumer authority must sign consumption, not grant creation. Never a decrypt leaf. | — | `allowTransient` composition |
| **transient store** (transaction journal) | Host-owned PDA `["transient", payer]`, opened once by a top-level payer signature and closed by the exact final top-level instruction. All FHE calls share its result occurrences, grants and HCU. Payer funds/refunds only; Store authorities control use. Failed finalization rolls back all writes. | — | transient ACL storage |
| **returned result** | An explicitly selected `(step_index, output_index)` in `FheExecuteArgs.returned_results`. At most 32 entries in requested order, including duplicates; current operators require output index zero. Empty selection returns no handles. `build_returning` selects one typed result. | — | function return value |

## Entry and exit trust

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **input proof** | The ZK bundle a user submits alongside an external encrypted input. | — | `inputProof` (`FHE.fromExternal`) |
| **attestation** | The coprocessor-committee-signed statement that a submitted input verified (names the calling program and host chain id). | — | `CiphertextVerification` |
| **certificate** | A KMS-threshold-signed statement, e.g. the public-decrypt result binding cleartext to handle and context. | — | KMS signature bundle |
| **proof** | A cryptographic proof and nothing else (MMR inclusion proof, ZK proof). Never used for signed statements — those are attestations or certificates. | — | — |
| **KmsContext** | The on-chain account naming one KMS committee's signer set and threshold; certificates bind to the context that issued them. | — | gateway KMS context |
| **MMR** | Append-only Merkle mountain range shared by all decrypt permissions under a Store. Only peaks and leaf count are stored on-chain; proofs establish exact historical leaves. | — | — |
| **leaf** | Historical-access commitment `(store, leaf_index, handle, allowed_key)` or public commitment `(store, leaf_index, handle)`, each domain-separated. Fresh Store outputs append allows in declaration order, then a public leaf if requested. | — | — |
| **leaf record** | The coprocessors' record of every leaf the host sealed, kept by the host listener next to the compute rows it was derived with and served over `POST /v1/solana/leaf-proofs` behind an API key. A source of proofs, never of decisions: the connector verifies each proof against the peaks it read on chain. | proof service, `solana-proof-service` | — |
| **disclosed value kind** | `DisclosedValueKind` selects the current token slot when requesting publication. The generic `HandleDisclosedEvent` certifies Store, handle and cleartext, not the slot kind; original operation events identify the result. | — | generic amount disclosure |

## Off-chain

| Term | Definition | Replaces | EVM equivalent |
|---|---|---|---|
| **decoded op records** | The typed structs the listener decodes from an execution's instruction data, one per step, fed to the coprocessor. They are not on-chain events. | `Fhe*Event` structs | `FheAdd`… events |
| **reconstruction** | The listener's re-derivation of output handles and leaves from raw transaction bytes, using the program's own derivation functions. | — | — |
| **crank** | A call anyone may make that advances work an earlier transaction could not finish — the demo's address-lookup-table lifecycle is driven this way, since deactivation has to wait out a cooldown. Retrying is the point: a crank that throws leaves the work for the next one rather than recording it as done. | — | — |

## Banned names

- **"lookup table"** for anything that is not a Solana address lookup table
  (ALT). The collision with the native ALT program is guaranteed confusion.
- **"subject"**, **"compute subject"**, **"compute signer"**: an encrypted store keeps no list of who may decrypt it, and reading a value is
  admitted by its authority's signature. Say *allow* / *viewer* / *allowed
  key*, or *Store authority*.
- **"domain"** in the ACL sense (`acl_domain_key`, "mint domain"): say
  *application* or *scope*. EIP-712 signing domains and hash-domain separation
  keep the word.
- **"standalone proof service"** for a separate trusted component: the listener serves proofs
  from its leaf record, and consumers verify them against their own chain snapshot.
- **`value_key`**, or the retired `EncryptedValue` account: Store identity is `(program, authority, scope)`;
  a slot key selects a handle within that Store.
- Any synonym in a "Replaces" cell above.
