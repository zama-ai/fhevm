# Solana design decisions

Last synced: 2026-09-17.

Each numbered entry records one decision the Solana port relies on and why it was taken. Entries
are appended, never renumbered. A decision that a later one replaces moves to
[`DESIGN_HISTORY.md`](DESIGN_HISTORY.md) with its original text, so this file holds only decisions
the code follows today. A live entry states only the current rule: when a later decision replaces
part of it, the entry is rewritten and the replaced wording moves to the "Replaced parts of live
decisions" section of `DESIGN_HISTORY.md`.

Read DD-049 first for the account, permission and disclosure model and DD-050 for transaction
composition; most earlier entries are written against them. Vocabulary follows
[`GLOSSARY.md`](GLOSSARY.md). The EVM mapping is [`EVM_PARITY.md`](EVM_PARITY.md); deferred
requirements are in [`FUTURE_DESIGN.md`](FUTURE_DESIGN.md).

Status values:

```text
adopted       the code relies on this design and tests preserve it
product-open  the direction is clear; production API, governance or encoding details need a final product decision
```

Each entry has the same four parts: Context, Decision, Rationale, Consequences. Some later entries
are written as one narrative instead.

## Index

| Decision                                                                                                                                  | Status                                   | Title                                                                                                                           |
| ----------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| DD-001                                                                                                                                    | replaced by DD-032, then DD-049          | Store Handles In ACL Records, Not PDA Seeds, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                          |
| [DD-002](#dd-002-keep-app-store-and-host-acl-store-separate)                                                                              | adopted                                  | Keep App Store And Host ACL Store Separate                                                                                      |
| [DD-003](#dd-003-treat-events-as-indexing-hints-not-authorization)                                                                        | adopted                                  | Treat Events As Indexing Hints, Not Authorization                                                                               |
| [DD-004](#dd-004-account-metas-and-witness-layouts-are-abi)                                                                               | adopted                                  | Account Metas And Witness Layouts Are ABI                                                                                       |
| DD-005                                                                                                                                    | replaced by DD-032, then DD-049          | Public Decrypt Is A Post-Creation Release, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                            |
| DD-006                                                                                                                                    | replaced by DD-031                       | Material Commitment Is Separate From ACL Authorization, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                               |
| [DD-007](#dd-007-external-inputs-verify-against-an-on-chain-secp256k1-coprocessor-attestation-verify-not-bind)                            | adopted                                  | External Inputs Verify Against An On-Chain secp256k1 Coprocessor Attestation (verify, not bind)                                 |
| [DD-008](#dd-008-model-transient-allow-as-explicit-solana-evidence)                                                                       | adopted                                  | Model Transient Allow As Explicit Solana Evidence                                                                               |
| DD-009                                                                                                                                    | replaced by removed; fhevm-internal#1692 | Operator Transfer Model Removed, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                                      |
| DD-010                                                                                                                                    | replaced by DD-040                       | Token Disclosure Paths Are Label-Scoped, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                              |
| DD-011                                                                                                                                    | replaced by DD-042 composition           | Transfer-And-Call Removed In Favor Of App-Driven CPI Composition, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                     |
| [DD-012](#dd-012-solana-user-decrypt-reuses-the-gateway-stack)                                                                            | adopted                                  | Solana User Decrypt Reuses The Gateway Stack                                                                                    |
| [DD-013](#dd-013-prefer-fail-closed-chain-boundaries)                                                                                     | adopted                                  | Prefer Fail-Closed Chain Boundaries                                                                                             |
| [DD-014](#dd-014-host-handle-creation-has-no-local-test-relaxation)                                                                       | adopted                                  | Host Handle Creation Has No Local Test Relaxation                                                                               |
| [DD-015](#dd-015-handle-creation-keeps-per-block-entropy)                                                                                 | adopted                                  | Handle Creation Keeps Per-Block Entropy                                                                                         |
| [DD-016](#dd-016-confidential-balances-use-the-immediate-available-balance-profile)                                                       | product-open                             | Confidential Balances Use The Immediate-Available-Balance Profile                                                               |
| DD-017                                                                                                                                    | replaced by DD-023                       | Role-Aware `fhe_execute` And Per-Op Bind Instructions (replaced), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                     |
| DD-018                                                                                                                                    | replaced by DD-011                       | Transfer-And-Call Refund Prepare/Finalize (replaced), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                 |
| DD-019                                                                                                                                    | replaced by DD-049                       | Confidential Transfer Persists Only Final Balance And Transferred-Amount ACL Records, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md) |
| [DD-020](#dd-020-verifierset-removed--canonical-kms-context-singleton)                                                                    | adopted                                  | VerifierSet Removed → Canonical KMS Context Singleton                                                                           |
| [DD-021](#dd-021-on-chain-secp256k1-kms-public-decrypt-cert-verification)                                                                 | adopted                                  | On-Chain secp256k1 KMS Public-Decrypt Cert Verification                                                                         |
| DD-022                                                                                                                                    | replaced by DD-040, DD-045               | Witness PDAs Created Before The secp Consume (replaced), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                              |
| [DD-023](#dd-023-fhe_execute-composed-executor--typed-fheexecutionbuilder-dsl)                                                            | adopted                                  | `fhe_execute` Composed Executor + Typed `FheExecutionBuilder` DSL                                                               |
| [DD-024](#dd-024-eager-ciphertext-material-preparation-coprocessor-side)                                                                  | adopted                                  | Eager Ciphertext-Material Preparation (coprocessor side)                                                                        |
| [DD-025](#dd-025-where-the-release-gate-sits)                                                                                             | adopted                                  | Where The Release Gate Sits                                                                                                     |
| [DD-026](#dd-026-input-and-identity-encoding-is-bytes32-user-decrypt-is-typed)                                                            | adopted                                  | Input And Identity Encoding Is bytes32, User Decrypt Is Typed                                                                   |
| [DD-027](#dd-027-chain-aware-v2-user-decrypt-validation)                                                                                  | adopted                                  | Chain-Aware V2 User-Decrypt Validation                                                                                          |
| [DD-028](#dd-028-what-the-port-does-not-do)                                                                                               | adopted                                  | What The Port Does Not Do                                                                                                       |
| [DD-029](#dd-029-drift_revert--on-chain-reorg-disambiguation)                                                                             | adopted                                  | `drift_revert` ≠ On-Chain Reorg (disambiguation)                                                                                |
| [DD-030](#dd-030-keep-verifyproofrequestsolana-not-a-v2-rename)                                                                           | adopted                                  | Keep `verifyProofRequestSolana`, Not A V2 Rename                                                                                |
| [DD-031](#dd-031-materiality-moves-to-the-gateways-ciphertextcommits-dd-006-revision)                                                     | adopted                                  | Materiality Moves To The Gateway's `CiphertextCommits` (DD-006 revision)                                                        |
| DD-032                                                                                                                                    | replaced by DD-049                       | `EncryptedValue` + MMR Replaces Keyed-Nonce `AclRecord` (RFC-024), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                    |
| [DD-033](#dd-033-no-acl-lifecycle-events--self-describing-args--instruction-replay-indexing)                                              | adopted                                  | No ACL-Lifecycle Events — Self-Describing Args + Instruction-Replay Indexing                                                    |
| DD-034                                                                                                                                    | replaced by DD-069                       | Eager Compute Scheduling For Solana (Q11 Option A), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                  |
| DD-035                                                                                                                                    | replaced by DD-048                       | Standalone Untrusted Solana MMR Proof Service, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                        |
| DD-036                                                                                                                                    | replaced by DD-045                       | Burn-Redemption Consume Authorizes By MMR Public-Decrypt Proof, Not Live Handle, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)      |
| DD-037                                                                                                                                    | replaced by DD-038                       | `fhe_execute` Events — `emit_cpi!`-Only, No `emit!` Log Fallback (DD-033 addendum), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)   |
| DD-038                                                                                                                                    | replaced by removed; fhevm-internal#2079 | One Host-Owned Born-Public Lifecycle Batch Replaces Per-Operation Events, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)             |
| DD-039                                                                                                                                    | replaced by DD-047                       | HCU Block Cap Meters The Signed `compute_subject`, Not A Separate Authority, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)          |
| [DD-040](#dd-040-app-public-decrypt-is-a-stateless-pull-oracle-verifier-not-a-request-lifecycle)                                          | adopted                                  | App Public-Decrypt Is A Stateless Pull-Oracle Verifier, Not A Request Lifecycle                                                 |
| [DD-041](#dd-041-coprocessor-input-trust-is-a-registered-n-of-m-signer-set-in-hostconfig)                                                 | adopted                                  | Coprocessor Input Trust Is A Registered n-of-m Signer Set In `HostConfig`                                                       |
| [DD-042](#dd-042-confidential-vaults-are-a-batcher-gateway-in-front-of-a-public-share-mint-vault)                                         | adopted                                  | Confidential Vaults Are A Batcher-Gateway In Front Of A Public Share-Mint Vault                                                 |
| [DD-043](#dd-043-two-derivation-regimes--content-addressed-deterministic-handles-persistent-write-anchored-rand-seeds-context_id-deleted) | adopted                                  | Two Derivation Regimes — Content-Addressed Deterministic Handles, Persistent-Write-Anchored Rand Seeds (`context_id` deleted)   |
| [DD-044](#dd-044-every-event-goes-through-the-event-cpi-or-is-not-emitted-at-all)                                                         | adopted                                  | Every Event Goes Through The Event CPI, Or Is Not Emitted At All                                                                |
| [DD-045](#dd-045-keep-burn-settlement-sequential-and-keep-wrapper-policy-separate-from-host-governance)                                   | adopted                                  | Keep Burn Settlement Sequential and Keep Wrapper Policy Separate From Host Governance                                           |
| [DD-046](#dd-046-the-program-heap-is-fixed-at-32-kb--no-custom-allocator)                                                                 | adopted                                  | The Program Heap Is Fixed At 32 KB — No Custom Allocator                                                                        |
| [DD-047](#dd-047-the-application-is-program-scope--program-verified-scope-owned-by-program)                                               | adopted                                  | The Application Is `(program, scope)` — Program Verified, Scope Owned By Program                                                |
| [DD-048](#dd-048-allows-are-sealed-on-the-write-the-deny-list-names-applications-one-connector-path)                                      | adopted                                  | Allows Are Sealed On The Write; The Deny List Names Applications; One Connector Path                                            |
| [DD-049](#dd-049-shared-encrypted-store-and-transaction-local-result-grants)                                                              | adopted                                  | Shared Encrypted Store And Transaction-Local Result Grants                                                                      |
| [DD-050](#dd-050-transient-storage-shared-across-the-transaction)                                                                         | adopted                                  | Transient Storage Shared Across The Transaction                                                                                 |
| [DD-051](#dd-051-a-zama-is-one-host-program-id)                                                                                           | adopted; see the note under its status   | A Zama Is One Host Program ID                                                                                                   |
| [DD-052](#dd-052-a-solana-chain-id-is-type-byte-0x01-plus-a-published-cluster-tag)                                                        | adopted                                  | A Solana chain id is type byte `0x01` plus a published cluster tag                                                              |
| [DD-053](#dd-053-a-program-id-is-environment-config-not-a-cargo-feature)                                                                  | adopted                                  | A program id is environment config, not a cargo feature                                                                        |
| [DD-054](#dd-054-the-programs-stay-on-anchor-v1)                                                                                          | adopted                                  | The programs stay on Anchor v1                                                                                                 |
| [DD-055](#dd-055-the-ledger-is-the-work-log-not-a-pda-queue)                                                                              | adopted                                  | The ledger is the work log, not a PDA queue                                                                                    |
| [DD-056](#dd-056-an-execution-describes-itself-the-listener-re-derives-handles-only-as-a-check)                                           | adopted                                  | An execution describes itself; the listener re-derives handles only as a check                                                 |
| DD-057                                                                                                                                    | withdrawn before merge                   | Rand nonce keyed on the application; the nonce stays global (DD-043)                                                            |
| [DD-058](#dd-058-pausers-stop-one-area-at-a-time-only-the-admin-resumes)                                                                  | adopted                                  | Pausers stop one area at a time; only the admin resumes                                                                        |
| [DD-059](#dd-059-the-listener-catches-up-from-an-archive-when-the-stream-cannot-replay)                                                   | adopted                                  | The listener catches up from an archive when the stream cannot replay                                                          |
| [DD-060](#dd-060-a-public-decrypt-names-its-stores-beside-the-kms-routing)                                                                | adopted                                  | A public decrypt names its stores beside the KMS routing                                                                       |
| [DD-061](#dd-061-a-delegation-is-keyed-by-application-and-expires-on-unix-time)                                                           | adopted                                  | A delegation is keyed by application and expires on Unix time                                                                  |
| [DD-062](#dd-062-the-listener-reads-one-transaction-per-message)                                                                          | adopted                                  | The listener reads one transaction per message                                                                                 |
| [DD-063](#dd-063-a-leaf-proof-reads-its-path-by-position)                                                                                 | adopted                                  | A leaf proof reads its path by position                                                                                        |
| [DD-064](#dd-064-leaf-proofs-are-served-apart-from-ingestion)                                                                             | adopted                                  | Leaf proofs are served apart from ingestion                                                                                    |
| [DD-065](#dd-065-a-public-decryption-is-accepted-on-chain-by-its-certificate-alone)                                                       | adopted                                  | A public decryption is accepted on-chain by its certificate alone                                                              |
| [DD-066](#dd-066-the-leaf-record-has-its-own-indexer-and-database)                                                                        | adopted                                  | The leaf record has its own indexer and database                                                                               |
| [DD-067](#dd-067-a-merkle-proof-request-is-signed-by-a-kms-contexts-tx-sender)                                                             | adopted                                  | A Merkle proof request is signed by a KMS context's tx-sender                                                                  |
| [DD-068](#dd-068-the-merkle-indexer-checks-its-record-against-the-chain-and-quarantines-a-store-that-disagrees)                            | adopted                                  | The Merkle indexer checks its record against the chain and quarantines a store that disagrees                                  |
| [DD-069](#dd-069-only-an-output-its-transaction-stores-is-computed-and-recorded-as-a-block-producer)                                      | adopted                                  | Only an output its transaction stores is computed and recorded as a block producer                                             |
| [DD-070](#dd-070-solana-is-read-at-finalized-commitment)                                                                                  | adopted                                  | Solana is read at finalized commitment                                                                                         |
| [DD-071](#dd-071-the-listener-records-each-solana-block-as-a-finalized-host-block-numbered-by-height)                                     | adopted                                  | The listener records each Solana block as a finalized host block, numbered by height                                           |
| [DD-072](#dd-072-one-source-for-everything-that-can-change)                                                                               | adopted                                  | One source for everything that can change                                                                                      |

## DD-002: Keep App Store And Host ACL Store Separate

Status: adopted

Context:

The confidential token program owns token semantics. The host program owns FHEVM authorization
semantics. Mixing those responsibilities would make it unclear which program is authoritative for
decrypt or compute permission.

Decision:

`confidential-token` stores token-local pointers such as current balance handles and emits
app-local indexing events. `zama-host` stores canonical ACL, delegation, and transient
authorization state. Ciphertext material is not host state (DD-031).

Rationale:

The host boundary gives KMS a chain-native source of truth that is independent from token-specific
business logic. Token state can answer "is this the current balance?", while host state answers
"may this authority compute with this handle, and may this key decrypt it?"

Consequences:

KMS does not parse token state to authorize decrypts. Apps discover current handles from Store
slots and historical ones from their own records; the KMS connector verifies the host-owned Store
and its leaf proofs.

## DD-003: Treat Events As Indexing Hints, Not Authorization

Status: adopted

Context:

EVM logs and contract state share one execution model. Solana log delivery is provider-dependent,
plain `emit!` logs can be truncated, and Anchor `emit_cpi!` adds nested CPI frames.

Decision:

Events are discovery and indexing signals. Production authorization must be rebuilt from
policy-approved transaction/account data and verified against the host's Store, delegation and KMS
context accounts.

Rationale:

Decrypt authorization cannot depend on whether a provider preserved a log line. It also cannot
require every production path to spend a self-CPI frame solely for observability.

Consequences:

The port keeps Anchor CPI events for tests and local listener compatibility, but production event
transport should use a Yellowstone/Geyser transaction and account stream with explicit commitment,
reconnect, replay, and account-witness verification policy.

The local side stack builds the listener from source (`test-suite/fhevm/src/solana/deploy.ts`). The
shared host-listener image also packages the Solana binaries. There is no fallback to the deleted
RPC listener.

## DD-004: Account Metas And Witness Layouts Are ABI

Status: adopted

Context:

KMS verification depends on exact account shape. Accepting arbitrary extra accounts, malformed
unused slots, executable placeholders, or ambiguous optional accounts can create witness confusion.

Decision:

Instruction account lists, dynamic remaining accounts, optional accounts and witness layouts are
treated as ABI. Consumers reject trailing metas, malformed unused fixed slots, invalid bumps, wrong
lengths, and stale or unsupported witness layouts.

Rationale:

The same account tuple must mean the same thing to the Solana program, listener, KMS verifier, and
tests. A loose ABI would let one layer accept evidence that another layer did not intend.

Consequences:

Negative tests are part of the contract. Changes to account layout must update program checks,
KMS witness decoders, fixture encoders, listener expectations, and docs together.

## DD-007: External Inputs Verify Against An On-Chain secp256k1 Coprocessor Attestation (verify, not bind)

Status: adopted

Adopted in the June 2026 reconciliation. It replaces the earlier verifier-signed-intent design, and
the verify-only refinement below replaces the earlier "and-bind" shape that created ACL state.

Context:

The port needs a production-shaped encrypted input path. The earlier design bound inputs through a
bespoke native Ed25519 "input verifier set" signing a `SolanaInputBindIntent`
([DESIGN_HISTORY](DESIGN_HISTORY.md#dd-007-replaced-in-part-by-dd-023)). That set was a Solana-only
trust root divorced from the EVM coprocessor trust model.

Decision:

Inputs enter compute through the `FheExecuteOperand::VerifiedInput` operand consumed inside `fhe_execute`
— the Solana `FHE.fromExternal` analog. The operand carries the coprocessor's EIP-712
`CiphertextVerification` attestation; the shared `verify_input_attestation` verifier
(`zama_host::instructions::input_verification`) re-verifies it **in-execution** by recovering the EVM
coprocessor signer via `secp256k1_recover` and threshold-checking it against the configured signer
set, and asserts the attested `contract_chain_id` equals the host chain id (EVM's
`contractChainId == block.chainid`). On success the input is usable within that execution only.
Verification creates no allow, mirroring EVM `FHEVMExecutor.verifyInput` (verify, not allow).

**Binding model (the `contractAddress` analog).** EVM binds an attestation to a `contractAddress`.
On Solana the host requires `attestation.contract_address` to equal the application `program`, which
it proves through the output authority PDA (DD-047). `user_address` is not EVM `msg.sender`; the host
does not interpret it, so apps must check the attested `user_address` themselves. An observer who
sees another user's verified input can replay it if the app skips that check.
Confidential-token checks the attested user equals the token account owner. Per-state-account
(per-mint) scoping is
deliberate and finer-grained than EVM's per-contract binding.

**Derived outputs are NOT tainted by the input attestation.** Once verified, the input is an ordinary
operand; any _persistent_ ACL on an input-derived handle is the app's separate, explicit choice at
output-binding time — exactly EVM parity, where the input gets a transient allow and persistent output
ACLs are the contract's decision. There is no output-taint from the input.

The gateway side is the bytes32 input path of the chain-agnostic address RFC (zama-ai/tech-spec#419):
`InputVerification.verifyProofRequestSolana(contractChainId, bytes32 contractAddress,
bytes32 userAddress, ciphertextWithZKProof, extraData)` + `event VerifyProofRequestSolana`, which
shares the zkProofId counter and consensus state with the EVM path and stores the request in a
parallel `solanaZkProofInputs` mapping for bytes32 EIP-712 response validation. (Here `extraData`
is the coprocessor cert's EIP-712 `CiphertextVerification` extraData — NOT the `0x03` user-decrypt
auth blob; see DD-026.)

Reusing the coprocessor attestation makes Solana input trust identical to EVM input trust — one
trust root, recovered and threshold-checked on-chain — instead of a parallel verifier-set subsystem
that could drift. Consuming it as an in-execution operand (rather than a standalone verify + persistent
receipt) restores EVM parity (verify ≠ allow) and removes a persistent ACL account per input — one of
the "3 ACLs" that inflated per-tx cost — so it is also a cost win.

Consequences:

- Inputs are the `FheExecuteOperand::VerifiedInput` operand of `fhe_execute`. There is no standalone
  verify instruction, receipt event or output-taint binding, so derived outputs are unconstrained by
  the input.
- The "caller is the attested contract" gate is enforced at input-consumption time
  (`attestation.contract_address == program`, DD-047).
- `fhe_execute` invokes the shared verifier `zama_host::eip712::verify_coprocessor_input` (via
  `instructions::input_verification::verify_input_attestation`) in-execution.

Open for debate / follow-up: the input proof / ZKPoK / transciphering behind the attestation is still
a harness shortcut; real ZKPoK + transciphering is production work.

## DD-008: Model Transient Allow As Explicit Solana Evidence

Status: adopted

Context:

EVM transient allowance uses transaction-local storage. Solana has no hidden transaction-local map
that a later instruction can read.

Decision:

Temporary permission is explicit Solana state. A result is recorded in the transaction's transient
store (DD-050); its producing Store may use it across calls, and another Store needs an explicit
result grant whose consumption its authority signs. A verified external input is usable within its
execution only (DD-007). A value that must outlive the transaction is written to a Store slot
(DD-049). EVM `allowTransient(handle, account)` maps to the result grant.

Rationale:

Solana has no hidden transaction-local map a later instruction can read; temporary permission must be
explicit. Keeping intermediates in the transaction's transient store, closed at the end of the
transaction, leaves no rent behind and prevents a temporary compute grant from silently becoming
persistent ACL or decrypt authority.

Consequences:

A Store output derived from transient inputs still passes its authority check and declares its own
allows; nothing is public unless the output says so.

## DD-012: Solana User Decrypt Reuses The Gateway Stack

Status: adopted

Adopted in the June 2026 reconciliation. It reverses the earlier decision that a native Solana
KMS flow must not reuse EVM routing.

Context:

The earlier decision (below) deliberately kept a Solana-native KMS request/response model (`native-v0`)
out of the EVM Gateway routing, on the theory that the data models were too different to share. That
produced a large, separately-maintained native-v0 admission/store/response subsystem in the connector.

Decision:

Treat Solana as a **gateway-compatible host chain** and route its decrypt flows through the unified
Gateway V2 path (the Unified EIP-712 Decryption Request RFC) rather than a parallel native stack:

- **User-decrypt** flows through the unified Gateway V2 path, through the Gateway's
  `solanaUserDecryptionRequest` entry, which types the handles, validity, transport key and
  `extraData` and carries the Solana permit fields in a versioned blob, rather than smuggling
  Solana auth through `extraData`. `extraData` is only the KMS routing (v0, v1 or v2, DD-060). A chain-aware validator
  branches on `contracts_chain_id` (see DD-027) so EVM stays strict and Solana is relaxed. The
  bytes32 handle surface still admits both EVM and Solana. (See DD-026 for the typed-vs-extraData
  boundary.)
- **Public-decrypt** certificates are verified **on-chain** via secp256k1: `zama_host` recovers EVM
  KMS signers from the cert and threshold-checks them, mirroring the EVM `KMSVerifier`
  (`verifyDecryptionEIP712KMSSignatures`). See DD-021.
- Solana is registered as a host chain (bytes32 ACL = the `zama_host` program id, type-byte `0x01` chain id;
  added to the relayer `host_chains`).

One decrypt trust model and one routing path is far less surface to keep in sync than a parallel
native subsystem. The coprocessor/KMS already verify EIP-712 over bytes32; making Solana speak the
same shapes lets the existing Gateway/coprocessor/KMS pods serve Solana with chain-aware validation
instead of a second pipeline.

What worked:

The full vertical (input → eval → compute → user-decrypt → public-decrypt/disclose) runs against the
real gateway/coprocessor/KMS/relayer side-stack reusing the shared coprocessor Postgres.

What didn't / had to change:

The reconciliation initially relaxed EVM input validation unconditionally to admit Solana over V2,
which weakened EVM (CI caught empty-contracts / wrong-sig being accepted). Fixed with the chain-aware
cross-field validator (DD-027).

Replaced design (stub): the earlier decision kept a Solana-native KMS request/response subsystem
(`native-v0`) out of EVM Gateway routing, on the theory the data models were too different to share.
Reversed to reuse one decrypt trust model and routing path. The branch has no native-v0 tables, no
typed-column detour and no connector subsystem that reads them.

Open for debate: is unifying on the EVM/Gateway stack the right long-term call, or does a second
non-EVM chain eventually justify a native path? The KMS-connector decrypt is exercised in the harness,
not full production KMS wiring (DD-028).

## DD-013: Prefer Fail-Closed Chain Boundaries

Status: adopted

Context:

Solana handles and witnesses are not EVM contract calls. Accidentally applying EVM ACL checks to a
Solana handle would create a false sense of authorization.

Decision:

A host chain's kind is its chain id's type byte, and each kind carries only its own settings. A
request against a Solana chain is authorized only by the Solana verifier, and a request whose kind
does not match its chain's is refused.

Rationale:

An unsupported chain path should be an explicit integration gap, not a permissive fallback.

Consequences:

Tests should cover both positive Solana witness acceptance and negative cases where EVM-shaped
checks are unavailable or inappropriate.

## DD-014: Host Handle Creation Has No Local Test Relaxation

Status: adopted

Context:

Earlier local tests used admin-toggled `HostConfig` flags and a zero creation-entropy fallback when the
Mollusk slot-hash sysvar was empty. That made test setup diverge from the deployed handle-creation path.

Decision:

The host `poc` feature, its admin controls, and the zero-entropy fallback are removed. Handle creation
always reads the previous bank hash and fails with `PreviousBankHashUnavailable` if the runtime does
not provide one. Mollusk and LiteSVM tests seed `Clock` and `SlotHashes` as a validator would. The real
input path remains the in-execution secp256k1 attestation verify (DD-007).

Rationale:

One handle-creation path is easier to reason about and tests the same entropy requirement as deployment.

Consequences:

Missing prior-bank entropy fails closed on every chain. The confidential-token demo retains its own
compile-gated receiver helpers; those do not alter host verification or handle derivation.

## DD-015: Handle Creation Keeps Per-Block Entropy

Status: adopted

Resolved in the June 2026 reconciliation; it was product-open before.

Context:

Computed handles are `bytes32`. ~88 bits are metadata (version, chain id, FHE type, computed marker),
leaving ~168 bits of keccak digest → roughly 2^84 birthday collision resistance. 2^84 is feasible to
grind offline for an extreme adversary. Computed handles therefore mix per-block entropy into the
digest (`previous_bank_hash` + `clock.unix_timestamp` on Solana). EVM does the identical thing via
`blockhash(block.number - 1)` (and `block.timestamp`) in `FHEVMExecutor._binaryOp` /
`_ternaryOp` / `_mulDivOp` / `_naryOp`. Persistent outputs derive **the same base handle** as transient
outputs — no per-output binding (see "Preimage" below).

Decision:

Keep the per-block-entropy-seeded derivation. The alternative — widening `bytes32` → `bytes` (full
hash) to remove the collision concern without entropy — was rejected. Persistent outputs derive the
plain base handle; do not mix a per-output sequence or a per-account ID into the handle.

Per-block entropy denies an offline adversary the ability to grind a target collision: the block hash
isn't known until the block exists, so the 2^84 search cannot be done ahead of time. This is exactly
why EVM mixes `blockhash(block.number-1)`. The `bytes32 → bytes` alternative was rejected because it
roughly triples SSTORE/account-write cost and has no migration path for already-deployed handles.

What this means plainly (state it in the debate):

Handles are **block-bound and therefore reorg-unstable on EVERY chain** (EVM and Solana alike): a
resubmitted or reorged transaction over the same inputs yields a _different_ handle. This is reconciled
by the listener's reorg handling on EVM (block-status machine, DD-025). Solana ingests only
finalized blocks, which do not reorg (DD-070).

Consequences:

Handle byte layout remains stable; handle creation is not idempotent across slots/blocks. The
`PreviousBankHashUnavailable` fail-closed surface remains as designed; handle derivation never falls
back to zero entropy (DD-014).

Preimage:

The preimage is DD-043's: deterministic handles carry neither `context_id` nor `op_index`, and
`op_index` enters only the rand seed. Operand-bearing preimages also carry DD-050's
transaction-origin mask, which the journal fixes before it records the result. A persistent output's
handle is byte-identical to the transient handle of the same computation: no per-output sequence and
no per-slot, per-caller or per-account value enters it. EVM's `FHEVMExecutor` binds none of these
either for binary, ternary, trivial, unary and cast operations; its only counter, `counterRand`,
feeds the rand seed.

Two distinct ciphertext materials cannot share a handle this way. Material is fully determined by
`(op / plaintext / rand-seed, operands, fhe_type)`, all of which are in the preimage, and per-block
entropy keeps the birthday search from being ground ahead of time. An identical recomputation yields
the identical handle, as on EVM. Every result occurrence is recorded, including identical
recomputations. Equal handles refer to the same encrypted computation, while Store identity and
explicit grants decide who may use it. Store slot writes use the initial snapshot and ordered
effects, not a duplicate-handle rejection. Random outputs keep their nonce-derived seed. The
canonical preimage helpers are in `state/mod.rs`.

## DD-016: Confidential Balances Use The Immediate-Available-Balance Profile

Status: product-open

Context:

Two Solana token profiles were weighed: a staged inbound-credit profile
(recommended default for public-receivable tokens, where the recipient applies pending funds under
their own transaction timing) and an immediate available-balance profile (EVM-style, where the sender
updates the recipient's balance directly). The latter lets a sender force an update of the recipient's
balance slot, which can invalidate a transaction the recipient already built against the prior
handle.

Decision:

The port uses the immediate available-balance profile: `confidential_transfer` credits the
recipient's balance slot inside the sender's transaction, with no recipient participation in the
base transfer.

Rationale:

It is the closest analog to ERC7984 `_update` and keeps the confidential-balance logic explicit and
EVM-parity-checkable. The stale-transaction and forced-inbound-write hazard is accepted for this
release.

Consequences:

This is an explicitly accepted tradeoff, not the recommended production default. A production
public-receivable token should evaluate the staged inbound-credit profile (pending → available under
recipient timing) or otherwise predeclare/lock the recipient's next balance transition so the
inbound-write surface is bounded.

## DD-020: VerifierSet Removed → Canonical KMS Context Singleton

Status: adopted

Context:

Witnesses and decrypt trust need an anchor. A Solana-only `VerifierSet` subsystem
(`create_verifier_set` / `disable_verifier_set` / `migrate_verifier_set`) would be a second trust
root beside the EVM KMS context, with its own lifecycle.

Options considered:

- (A) A VerifierSet subsystem with its migration lifecycle.
- (B) Collapse trust to a single on-chain KMS context keyed by `kms_context_id`. **Chosen.**

Decision:

Decrypt trust anchors to one `KmsContext` account per `kms_context_id`, which `define_kms_context`
creates (`zama_host::kms_context_address(context_id)`, seed `[KMS_CONTEXT_SEED, context_id]` with a
32-byte id). A certificate is verified against the live context it names, and `destroy_kms_context`
is the revocation lever (DD-040).

Why / what worked:

Single source of truth, less divergence between a Solana-only set and the EVM KMS context. Invariant-
tested.

Open for debate:

Context rotation governance (who may `define` and `destroy`, and the rotation choreography) is not
yet designed for production (fhevm-internal#1634).

## DD-021: On-Chain secp256k1 KMS Public-Decrypt Cert Verification

Status: adopted

Context:

Public-decrypt release needs the KMS threshold certificate verified somewhere. An Ed25519 cert checked
against a Solana-only verifier set would be a second trust model beside the EVM KMS one.

Decision:

`zama_host::eip712::verify_kms_public_decrypt` recovers secp256k1 EVM signers from the cert
(`recover_evm_address`), requires a **distinct-signer threshold** (`verify_threshold`) against the
signer set and threshold of the live `KmsContext` that `extract_kms_context_id(extra_data, current)`
names, and **rejects high-s (malleable) signatures** (`signature[32..64] > SECP256K1_HALF_ORDER`).
`verify_public_decrypt` returns that context id, so a caller can demand the current one (DD-040).
`extract_kms_context_id`
mirrors the EVM `KMSVerifier`: empty / version-0 `extra_data` selects the current context,
versions 1 and 2 carry a big-endian context id in `extra_data[1..33]`.

Why / what worked:

Mirrors the EVM `KMSVerifier` so the same threshold cert verifies on both sides. Adversarial cases
(wrong threshold / wrong signer set / context mismatch — the "L4-b/c/d" harness rejections) are rejected
live.

Open for debate:

The harness exercises the KMS connector decrypt, not full production KMS-connector wiring (DD-028).

## DD-023: `fhe_execute` Composed Executor + Typed `FheExecutionBuilder` DSL

Status: adopted

Context:

The first Solana ACL design sketched one batched `execute_frame` entry point. The host needs one batched execution with
instruction-local transients that apps compose through CPI.

Decision:

`fhe_execute` runs one execution of ordered steps (binary, ternary, unary, trivial, random, sum, isIn
and mulDiv operations). External encrypted inputs are not a step type: they enter as the
`FheExecuteOperand::VerifiedInput` operand of a step and are verified in-execution (DD-007).
Intermediate results are transient and later steps consume them through `EarlierStep`; only results
the execution writes to a Store slot or declares allows for outlive it (DD-049). The app-facing
`zama-fhe` crate (`solana/crates/zama-fhe`) exposes the typed `FheExecutionBuilder`, returning
`Encrypted<T>` for intermediates and hiding raw producer and account indices, with a `cpi`-feature
account resolver.

Transient intermediates keep rent proportional to product state: a plain transfer writes two balance
slots and no intermediate, while each Store output keeps its authority check (DD-049).

Open for debate:

The step cap `MAX_FHE_EXECUTION_STEPS` is derived from measured instruction-data and compute-unit budgets
on the interned wire format (fhevm-internal#1853 W8; see the constant's doc in
`programs/zama-host/src/constants.rs`). There is no per-operation replay event and no
created-public batch (DD-038, in DESIGN_HISTORY.md).

## DD-024: Eager Ciphertext-Material Preparation (coprocessor side)

Status: adopted

Context:

Ciphertext/SnS preparation is expensive but does not authorize plaintext release. The KMS separately
validates the Store and the leaf proof before decrypting.

Decision:

Instruction reconstruction emits concrete material requests at handle creation and Store
update. The listener inserts those handles directly into `pbs_computations`. Later allows reuse
already-prepared material. No account-fetch queue, witness
store, retry state machine, or coprocessor-owned ACL decision remains.

Why / what worked:

This removes a duplicate Solana RPC read. Prepared ciphertext material is not authorization and
cannot cause plaintext release.

Open for debate:

None on the coprocessor side. KMS commitment and authorization semantics are documented separately.

## DD-025: Where The Release Gate Sits

Status: adopted

Eager materialization; live KMS authorization at release time.

Context:

The previous Solana ingestion inserted computations dormant and activated them per finalized allow.
That did not compose with transient eval intermediates: a `confidential_burn`'s burned-amount handle
depends on transient sub-handles that are never individually allowed, so a per-handle allow gate could
not activate the graph that produced the released handle.

Separately, the EVM reorg substrate already implements the recommended shape: a block-status machine
(`pending → finalized / orphaned` in the `host_chain_blocks_valid` table) plus ancestor catch-up in
`cmd/block_history.rs`. The Solana listener (`bin/solana_host_listener.rs`) reconstructs from a
Yellowstone stream at `finalized` and records each block in `host_chain_blocks_valid` as finalized
when it ingests it (DD-071). A finalized block is never orphaned (DD-070), so the reorg path of that
substrate does not run for Solana.

Options considered:

- (A) Eager-materialize and gate decrypt release on a separate finality check. Superseded by
  DD-070: every reader is at `finalized`, so the gate would check nothing.
- (B) Keep the two-step dormant model + add transitive subgraph activation via a recursive CTE
  (activate the whole producing subgraph when the released handle is allowed). Rejected: the
  dormant/activate model and transient eval intermediates were designed separately and do not
  compose.
- (C) Slot-level finality gate. Superseded by DD-070, as (A).

The accepted design is eager materialization from the listener's finalized ingest with no separate
gate: the KMS connector revalidates authorization at the plaintext-release boundary, reading the
chain at `finalized` (DD-070). KMS remains the only plaintext-release boundary.

Open for debate:

None.

## DD-026: Input And Identity Encoding Is bytes32, User Decrypt Is Typed

Status: adopted

Context:

The unified bytes32 input path must encode non-EVM (Solana) dapp/user identities. Separately, a Solana
_user-decrypt_ request must carry ed25519 auth (user identity, nonce, allowed scopes). These
are two different surfaces.

Decision:

**Input path (identities are bytes32; NO `0x03` blob):**

- Non-EVM bytes32 input via `InputVerification.verifyProofRequestSolana` + event
  `VerifyProofRequestSolana` (dapp/user are 32-byte host addresses; shares zkProofId + consensus with
  the EVM path; request stored in `solanaZkProofInputs` for bytes32 EIP-712 response validation).
- Which `u64` is a Solana host chain id is DD-052. Relayer `is_solana_host_chain_id`
  matches type byte `0x01`.
- The input's `extraData` is the **coprocessor cert's EIP-712 `CiphertextVerification` extraData**, not
  the Solana user-decrypt blob. The input identity itself is a plain bytes32 host address (no
  version-byte blob).

**User-decrypt path (typed identity and auth fields):**

- The gateway's `solanaUserDecryptionRequest(ctHandles, requestValidity, publicKey, extraData,
  solanaRequest)` types the fields it budgets and charges, and carries the rest in the
  `solanaRequest` blob: `0x05 ‖ borsh{user_address, allowed_scopes, verifying_program_id,
  signature, entries}`. It emits `SolanaUserDecryptionRequest`. `SolanaUserDecryptRequest::assemble`
  (`zama-solana-request`) joins the two parts into the typed request every authorizer reads, so no
  fact travels twice. One claim per handle names its owner and Store (DD-048, DD-049), and
  `extraData` is only the KMS routing (DD-060). The
  connector fetches leaf proofs itself, so neither client nor relayer can substitute proof data.
- The js-sdk builds the blob and the relayer submits the call. The KMS connector routes Solana
  requests by their event, joins the two parts and verifies the permit signature before using them.
- KMS-cert context: `extract_kms_context_id` (DD-021) handles `extra_data` versions 0, 1 and 2 (the
  public-decrypt cert) — a _different_ extraData from either path above.

A bytes32 identity plus a Solana chain id (DD-052) keeps one input ABI for EVM and non-EVM hosts. For user-decrypt,
typed gateway fields make the Solana identity and auth request self-describing, and `extraData` stays
the KMS routing field it is on EVM.

## DD-027: Chain-Aware V2 User-Decrypt Validation

Status: adopted

Context:

Admitting Solana over the unified V2 user-decrypt path (DD-012) required relaxing EVM input validation
(empty `contractAddresses`, 128-or-130-char signature).

Decision:

A **cross-field validator branches on `contracts_chain_id`** via `is_solana_host_chain_id` (type byte
`0x01`; the predicate’s meaning is DD-052): EVM-strict (non-empty contracts, exact EIP-712 130-hex
signature) vs Solana-relaxed (empty contracts allowed, 128-or-130-char signature). Per-field
validators stay permissive; strictness is enforced in the cross-field branch.

Why / what worked:

Branching on the chain type keeps EVM strictness intact while admitting Solana. The CI integration
test covers both chains.

Open for debate:

The Solana-relaxed signature acceptance (128 ed25519 vs 130) is the seam most likely to need tightening
once the input-identity encoding (DD-026) is frozen.

## DD-028: What The Port Does Not Do

Status: adopted

- **KMS connector decrypt** is exercised in the harness, **not** full production KMS-connector wiring.
- **Solana reorg unwind is not wired**: the listener records each block finalized at ingest
  (DD-071), and the `block_history.rs` reorg path never runs for it. A finalized block is never
  orphaned, so there is nothing to unwind (DD-070).
- **Single local validator** in the harness — real reorgs / finality lag are not exercised end-to-end.
- **Input proof / transciphering** behind the coprocessor attestation is a shortcut today; real ZKPoK +
  transciphering is production work (DD-007).
- Host handle creation always requires the previous bank hash; local runtime tests seed the real
  `Clock` and `SlotHashes` sysvars (DD-014).

## DD-029: `drift_revert` ≠ On-Chain Reorg (disambiguation)

Status: adopted

Context:

Two distinct "revert" notions were easy to conflate in the coprocessor.

Decision:

Store them apart, explicitly:

- **`drift_revert`** = COPROCESSOR consensus: two coprocessors disagree on a ciphertext's bitwise
  representation. It **fires even on a chain that never reorgs** (`fhevm_engine_common::drift_revert`;
  consumer `check_if_drift_revert_is_over` / `latest_signal_for_chain`).
- **On-chain reorg** = the host chain orphaning an ingested block; handled by the listener's block-status
  machine / `cmd/block_history.rs`.

The discriminator now in the code comments: "would it fire on a chain that never reorgs?" — yes ⇒
`drift_revert`; only on an orphaned block ⇒ reorg.

They have different triggers, owners, and remedies; conflating them muddles both reorg handling (DD-025)
and the consensus path.

## DD-030: Keep `verifyProofRequestSolana`, Not A V2 Rename

Status: adopted

An attempted rename was reverted.

Context:

There was a proposal to rename `verifyProofRequestSolana` → `verifyProofRequestV2` for a cleaner
"V2 = multi-chain" naming.

Options considered:

- (A) Rename now to `verifyProofRequestV2`.
- (B) Keep `verifyProofRequestSolana`; revisit V2 later as a deliberate multi-step change. **Chosen.**

Decision / why:

Keep `verifyProofRequestSolana`. The rename is an **ABI break** (fails contract upgrade-compat) and
**cascades across cross-repo binding consumers** — relayer/coprocessor consume gateway bindings as a
pinned rev, and the local-path workaround breaks `cargo fmt`. (A separate `InputVerificationV2Example`
contract exists in the examples tree; the production interface keeps the `Solana` name.)

What didn't work:

An attempted rename was reverted for the upgrade-compat + cross-repo binding reasons above.

Open for debate:

Revisit `verifyProofRequestV2` as a coordinated multi-step change when a 2nd non-EVM chain or an
EVM-migration lands.

## DD-031: Materiality Moves To The Gateway's `CiphertextCommits` (DD-006 revision)

Status: adopted

Revises DD-006.

Context:

DD-006 put materiality (does ciphertext material exist, is it bound to the right key, is it ready for
KMS release) into host-owned `HandleMaterialCommitment` accounts sealed onto the ACL record. The
`EncryptedValue` + MMR rewrite (DD-032) removes the per-handle ACL record this sealed onto.

Decision:

Delete the `HandleMaterialCommitment` subsystem (`commit_handle_material`, the material authority
config, and the sealed-commitment fields) entirely. Ciphertext material commitments belong to the
gateway's `CiphertextCommits`, where the coprocessor already registers Solana handles — not to host
ACL state.

Rationale (from the rewrite's commit message, verbatim intent): host ACL state answers "who may use or
decrypt this handle"; whether the ciphertext material itself is available and bound to the right key
is a gateway-side concern the coprocessor already tracks, so duplicating it on-chain on Solana added a
second source of truth for no benefit.

Consequences:

KMS public-decrypt admission checks no material commitment on chain: it relies on the gateway's
`CiphertextCommits` for materiality and on the Store MMR (DD-049) for authorization, and the KMS
connector carries no material witness.

## DD-033: No ACL-Lifecycle Events — Self-Describing Args + Instruction-Replay Indexing

Status: adopted

Context:

Store lifecycle changes could emit Anchor events (`emit!`/`emit_cpi!`) the way compute-step events
do, or stay event-free and let consumers decode instruction data instead.

Decision:

Store-changing paths (`fhe_execute` Store outputs and `make_store_handle_public`) emit no ACL
lifecycle Anchor events by design. The host listener reconstructs compute requests from finalized
Yellowstone transaction instructions, including inner CPI instructions, since confidential-token
and other app programs invoke the host via CPI. The Merkle indexer reconstructs the MMR leaves from
the same instructions (DD-066).
Store outputs carry the expected previous handle and leaf count, so every transaction is
independently interpretable off-chain and the Merkle indexer reconstructs leaves from instruction
data alone, in replay order, without reading account state first. Compute facts, including which
outputs are made public, are reconstructed from the execution; what the host decided (the result
handles, their block context and the random seeds) travels in its one `FheExecutedEvent` (DD-056).

Rationale:

`DESIGN_DECISIONS.md` (DD-004 context) already notes that plain `emit!` logs can be truncated and
Anchor `emit_cpi!` adds nested CPI frames; avoiding events for a lifecycle that must survive CPI and be
replayed byte-for-byte from the instruction stream (the leaf record is rebuilt from it alone,
DD-048) sidesteps both concerns for this
particular subsystem. No further code-comment rationale beyond this was found for the CPI-depth angle
specifically; `EVM_PARITY.md` separately notes `fhe_execute`'s own step batching is bounded partly to
limit CPI depth (DD-008), which is a related but distinct concern from why ACL lifecycle avoids events.

Consequences:

The coprocessor produces handle-only material requests and inserts them directly into
`pbs_computations`; it does not derive authorization from instruction names or maintain allow reasons.
The Solana host follower's decoder (`solana-host-follower/src/host.rs`) parses raw instruction data
(Anchor discriminators + borsh args) instead of dispatching on ACL events.

## DD-040: App Public-Decrypt Is A Stateless Pull-Oracle Verifier, Not A Request Lifecycle

Status: adopted

Context:

App-usable public decrypt on EVM is a relayer-paid callback into a passive contract, so the gateway
keeps a requestID registry to route the callback and to not deliver twice. Solana has no callbacks, so
a request-witness account would only simulate one. The idiomatic Solana shape for "consume an
off-chain-signed fact on-chain" is the pull oracle (Pyth pull, Switchboard on-demand): the consumer
brings the signed attestation in its own transaction, verifies it statelessly, and uses it in the same
instruction.

Decision:

`verify_public_decrypt` is a CPI-able, stateless host instruction. It verifies a KMS
`PublicDecryptVerification` secp256k1 threshold certificate and returns the certified `(handle,
cleartext, context_id)` through `set_return_data`: 96 bytes, `handle ++ cleartext ++ context_id`, well
under the 1024-byte limit. It creates, mutates and emits nothing, and takes no signer. Its two
accounts, `host_config` and `kms_context`, are read-only; it reads no Store and takes no Merkle proof
(DD-065). An app CPIs it, asserts that the returned handle equals the handle it pinned, then applies
its own state transition. Act-once and timeout live in the app's own state machine (a settled flag
and a deadline), which it needs anyway.

The verifier reads no live handle, so a handle the Store has since replaced still verifies. An app
that must release value only for the current handle checks its own state, as redemption does
(DD-045).

Any live context, not a current-only pin (fhevm-internal#1765):

The cert is verified against the `KmsContext` the certificate itself names in its signed `extra_data`
(EVM `_extractContextId` parity), for whatever context that is, as long as the context is still alive
(`destroyed == false`). The binding chain is: signed `extra_data` → context id → canonical PDA for
that id → that context's signer set. The verifier reads the committed id, derives its canonical PDA,
requires the supplied account to be exactly that PDA (with a matching stored id) and not destroyed,
then checks the threshold signature against that context's signers. A v0 / empty `extra_data` cert
commits no explicit id and so selects the current context; v1 / v2 `extra_data` carries the id.

This adopts EVM's rotation semantics. On EVM a request pinned to context N stays answerable by N's
signers after a rotation to N+1, until an operator explicitly calls `destroyKmsContext(N)` — a
deliberate dual-set grace window. We get the same outcome with less machinery: the verifier is
stateless and receives a complete threshold cert in one call, so there is no per-request pin to store;
the cert names its own context and acceptance is a pure read of an existing, non-destroyed account.
Rotation for hygiene keeps in-flight certs verifiable (no liveness hiccup); `destroy_kms_context(N)`
is the revocation lever — one flag flip that instantly invalidates every outstanding N-cert
everywhere. Rotation for compromise is therefore `define` + `destroy`.

The accepted footgun (as on EVM): valid-until-destroyed means a forgotten `destroy` leaves an old
signer set powerful indefinitely. EVM manages this by runbook and we do the same for now; a cheap
`max_context_lag` in `HostConfig` (accept only contexts within K of current) is the natural guard if we
ever want one — noted, not in scope.

### Ops runbook: KMS context rotation (fhevm-internal#1862 #15)

1. **Rotate for hygiene:** `define_kms_context(N+1, E)` (new signer set and its epoch `E` become
   current). In-flight certs that name live context `N` remain verifiable — do **not** treat rotation
   alone as revocation.
2. **Revoke old set:** after grace (or immediately on compromise), `destroy_kms_context(N)`. That
   flips `destroyed` and fails every outstanding `N`-named cert at `verify_public_decrypt`.
3. **Compromise path:** `define` the replacement, then `destroy` the compromised context in the same
   ops window. Forgotten destroy = old signers stay powerful indefinitely (same as EVM).
4. **Reshare keys:** when the KMS moves context `N` to a new epoch with the same signer set,
   `define_kms_epoch(N, E+1)` sets the epoch new public decrypts route to. Verification reads only
   the context, so certificates of `N` stay verifiable.
5. **Token policy:** confidential-token `disclose_secp` / `redeem_burned_amount` accept any
   non-destroyed context the cert names (default). Apps that need current-only can compare
   `return_data`'s context id to `host_config.current_kms_context_id` — not wired in the token today.

The verified context id is surfaced in `return_data` (32 bytes appended after `handle ++
cleartext`, so 96 bytes total) precisely so a calling program can pick its own policy: an
informational consumer accepts any live context, while a value-releasing instruction can compare the
returned id against `host_config.current_kms_context_id` and demand current-only. Confidential-token's
`disclose_secp` and `redeem_burned_amount` both take the default (accept any live context), matching
EVM.

The verifier never pauses, as EVM's `KMSVerifier` (DD-058). A certificate certifies a value the
KMS has already made public, so verifying it again reveals nothing new. Against a compromised KMS
context, the admin makes a new context current, then `destroy_kms_context` revokes every
certificate the old one signed, as EVM's owner-only `destroyKmsContext`, which also refuses the
active context.

Return data carries the cleartext: today's KMS cleartexts are ≤32 bytes; if larger types are ever
revealed, the fallback is a caller-provided scratch account.

Token consumers:

Confidential-token consumes the verifier in two instructions, and neither keeps a request account:

- `disclose_secp(handle, cleartext, signatures, extra_data)` publishes a certified handle and its
  cleartext in `HandleDisclosedEvent`, as ERC-7984 `discloseEncryptedAmount` does. It reads no token
  state. Disclosure is idempotent information release with no replay marker; an app that needs
  consume-once tracks it in its own state.
- `redeem_burned_amount(burned_handle, cleartext_amount, signatures, extra_data)` asserts that the
  certified handle equals the `burned_handle` pinned in `PendingBurn` and that the certified
  cleartext equals `cleartext_amount`, requires the burned Store's current handle to be that handle,
  then pays out and closes `PendingBurn` (DD-045, DD-065).

On the request side, a token owner or mint authority calls `make_token_account_handle_public` or
`make_total_supply_handle_public`. The wrapper validates one exact token state field, then signs the
host `make_store_handle_public` CPI as the Store authority. No request PDA, `kms_context_id` pin or
`expires_slot` exists.

Deny policy applies when the host seals an allow (DD-048). Redemption and cancellation seal no
allow, so a later policy change cannot trap a pending burn.

Act-once for redemption is the closeable `PendingBurn` PDA at `["pending-burn", mint, token_account]`.
Exactly one burn may be pending for a token account. Redeem pays underlying tokens and closes it;
cancel restores confidential balance and encrypted supply and closes it. A second settlement fails
because the account is gone, and a new burn cannot start until that close has committed.

## DD-041: Coprocessor Input Trust Is A Registered n-of-m Signer Set In `HostConfig`

Status: adopted

Input `CiphertextVerification` attestations are verified against a **registered coprocessor signer
set + configurable threshold**, matching EVM `InputVerifier`'s trust model. The n-of-m recovery
machinery (`eip712::verify_threshold`, distinct-signer counting + high-s rejection) is shared with the
KMS cert path.

The set lives **inline in `HostConfig`**, not in a dedicated PDA (the `KmsContext` shape was the other
option). `HostConfig` holds `coprocessor_signers: [[u8; 20]; MAX_COPROCESSOR_SIGNERS]` (cap 8),
`coprocessor_signer_count: u8` and `coprocessor_threshold: u8`. A fixed-capacity array keeps the
singleton's byte layout **pinned** (the account serializes to the same size regardless of how many
signers are active), and avoids threading a second account through `fhe_execute`, which is
byte-tight. The cap is 8: comfortably above realistic coprocessor-quorum sizes while bounding both
the account size (`HostConfig::SPACE` is 311) and the worst-case per-attestation recovery cost.
Rotation is admin-driven today via the admin-gated `set_coprocessor_signers` instruction (same
admin/pause-neutral pattern as the other `set_*` config setters); a gateway-sync authority would
drive it from the EVM `GatewayConfig` coprocessor registry in production.

Registration invariants (mirroring the KMS-context rules): non-empty set, within the cap,
`1 <= threshold <= len`, no duplicate signer (distinct-signer counting would otherwise silently raise
the effective quorum), no zero-address signer. Enforced identically by `initialize_host_config` and
`set_coprocessor_signers` via one shared validator. `InitializeHostConfigArgs` carries
`coprocessor_signers: Vec<[u8; 20]>` + `coprocessor_threshold`.

**Signatures carried equal the threshold, not the party count.** A verifier needs `t` valid distinct
signatures over the attestation; the coprocessor sends `t`, not `n`. This holds for **both** EIP-712
attestation families — coprocessor `CiphertextVerification` inputs and KMS `PublicDecryptVerification`
certs — so the carried EIP-712 signature payload scales with `t` (t x 65 bytes), independent of how
many signers are registered. A threshold-4 `confidential_transfer` transaction (4 x 65B sigs over the
real token account list) serializes to **989 bytes** as a legacy transaction, inside the 1232-byte
(`solana_packet::PACKET_DATA_SIZE`) single packet. Clients send version 1 transactions, which allow
4,096 bytes and 64 account keys.

Public-decrypt consume transactions carry the certificate and no Merkle proof (DD-065), so their size
grows only with the threshold. With the SDK's 65-byte v2 `extra_data`, `disclose_secp`,
`redeem_burned_amount` and `verify_public_decrypt` each fit one version 1 transaction at the host's
maximum of 16 KMS signatures (`demo-dapp/src/vault/actions/kmsCertificateSize.test.ts`).

Relates to DD-007 (input verification model).

## DD-042: Confidential Vaults Are A Batcher-Gateway In Front Of A Public Share-Mint Vault

Status: adopted

Confidential yield on Solana is built as a **confidential batcher in front of an ordinary public
vault**, not as a vault whose own accounting is encrypted. The batcher collects encrypted deposits,
sums them homomorphically, and reveals **only the batch total**; that one public number is deposited
into a standard share-mint vault (PDA authority, SPL share mint, share price = assets / shares,
yield as share-price appreciation — the shape Kamino/Jupiter/Meteora/LSTs all use). Per-user share
distribution is `encrypted(deposit) x public batch rate` — the share-price division happens once, on
the plaintext aggregate. Ciphertext-by-ciphertext division is never needed anywhere in the flow.

This mirrors the architecture Zama shipped on Ethereum (confidential batcher over a Morpho ERC-4626
vault) for the same reason it was chosen there: a natively confidential vault needs encrypted
division for share pricing, which FHE cannot do efficiently, while an aggregate-only reveal
preserves individual amount privacy at near-zero encrypted-math cost. Ported as _intent_, not
mechanics: where the EVM join is a token-side `transferAndCall` hook, Solana inverts control — the
batcher's `join` CPIs the confidential transfer itself (programs cannot react to incoming
transfers); where the EVM aggregate reveal is a gateway callback, ours is the existing pull-shaped
burn-redemption certificate (`confidential_burn` -> KMS-certified `redeem_burned_amount` against any
live cert-named context, DD-040 family; the request-witness lifecycle is dissolved by
fhevm-internal#1763) —
the burn certificate _is_ the aggregate decrypt, no separate reveal instruction.

The mechanics this relies on: the token returns the transferred handle and grants it to the
participant's contribution Store through the transient store (DD-049), so the batcher adds each
deposit into that Store in the same join transaction. Each batch gets its **own token
account**, so the burned/revealed total is exactly that batch's sum (the EVM code documents the
inter-batch dust leak this prevents). Lifecycle is Pending -> Dispatched -> Settled/Canceled, or
Refunding after a cancelled dispatch, with permissionless dispatch/settle/claim and an
exact-refund `quit` — no operator custody of principal.

Deliberate non-goals, carrying the EVM team's recorded lessons: **no participant-count gates**
(trivially defeated by one actor joining N times with encrypted zeros; a single-participant batch
reveals that participant's amount and we document it instead of gating it), **no protocol-level
noise injection**, **no action that branches on encrypted store** (push-only flow; reactive designs
are probeable), and **no reward/incentive machinery** (the EVM campaign's dominant operational pain;
demo yield is simulated by donating underlying to the vault). The demo vault is new, deliberately
minimal code whose instruction interface mirrors Jupiter Earn's and is isolated behind one CPI
module in the batcher — Solana has no adopted vault interface standard (no ERC-4626 equivalent), so
compatibility means matching the prevailing shape, not importing a program.

The only confidential-token addition the flow needs is `confidential_burn_from_value` (burn an
existing handle) — the burn-side analog of `confidential_transfer_from_value`.

Didactic companion: `CONFIDENTIAL_VAULTS.md`. Relates to DD-039 (HCU metering identity), DD-040
(pull-oracle public decrypt), DD-041 (packet envelope — batch transactions stay within the measured
fit table).

Deposit path implemented (fhevm-internal#1757): `programs/confidential-batcher` (evolved in place
from the `confidential-deposit-app` reference) with `initialize_batcher` / `open_batch` / `join` /
`quit` / `dispatch` / `settle` / `claim`. Refinements over the sketch above, all mechanism-level:
the per-batch authority PDA is one identity that owns both batch token accounts, is the batcher's
Store authority, and signs every token CPI via `invoke_signed`;
`join` moves the amount with the ATTESTED `confidential_transfer` arm (a wallet user's fresh
encryption is a fromExternal input; `confidential_transfer_from_value` remains the mechanism for
`quit` refunds and `claim` payouts, whose amounts are existing computed handles); each participant's
JoinRecord owns its contribution Store, and the token returns the transferred handle and grants it
to that Store through the transient store (DD-049); "next batch opens immediately" is a
permissionless `open_batch` gated only on the previous batch no longer being pending, rather than
being folded into `dispatch` (keeps each instruction inside one transaction envelope); and the rate
is fixed-point at `RATE_SCALE = 10^9` with both divisions rounding down, so the sum of claims can
never exceed the wrapped shares (the claim MulDiv's 128-bit intermediate and euint64 result are
bounded by `shares * RATE_SCALE`). Settle prices and wraps only the vault-minted share DELTA across
its deposit phase — never the share account's raw balance — because SPL destinations cannot refuse
incoming transfers: a preloaded share balance stays inert instead of inflating the rate past u64 and
bricking the batch (pinned by `mollusk_preloaded_shares_do_not_poison_the_rate`).

Known deposit-path limitation (open): a batch whose certified total floors to zero shares at the
vault's current price cannot settle — `demo_vault::deposit` rejects `ZeroShares`, settle reverts
atomically (retryable but never to success, since the demo vault's price only rises), and the batch
is stuck Dispatched with its deposits burned. An attacker holding ~all vault shares can brick
sub-price-P batches near-free by `harvest`-donating P (the donation accrues to their own shares);
the loss per batch is bounded below one share's worth. Behavior is pinned by
`mollusk_dust_total_settle_reverts_and_batch_stays_dispatched`. The intended fix is a
cancel-and-refund settle branch: wrap the redeemed underlying back into the batch's confidential
account and refund each user's encrypted deposit via `confidential_transfer_from_value` (quit's
mechanism) — not implemented in the deposit-path PR.

Redeem path implemented (fhevm-internal#1758), as an addendum to the deposit path above. **One
program serves both directions, with the direction on the `Batcher` config** — each config is a
Deposit or a Redeem instance, mirroring the EVM's two batcher deployments: a pending deposit batch
never blocks a redeem batch (each batcher serializes only its own batches), yet every account
layout and every instruction is shared. The vocabulary is direction-neutral — a JOIN confidential
mint (what users batch in: cUnderlying for deposits, cShares for redeems) and a PAYOUT confidential
mint (what claims pay: cShares for deposits, cUnderlying for redeems) — and the batch lifecycle's
only direction branch is settle's vault phase (`demo_vault::deposit` vs `demo_vault::withdraw`);
`initialize_batcher` additionally validates the mint wiring per direction at setup, and join, quit,
dispatch, claim, and open_batch are direction-free. The
alternative (a duplicated instruction set) was rejected because the two flows differ in exactly one
CPI: duplicating fourteen account structs to encode one branch would double the review surface for
zero clarity.

In both directions a claim is the exact proportional floor
`encrypted(joined) x payout_received / total_joined` in one MulDiv (fhevm-internal#1774 item 1).
Sum-of-claims <= payout holds: `sum(floor(j_i * P / T)) <= floor(sum(j_i) * P / T) = P`. The MulDiv intermediate
`joined * payout_received < 2^128` stays inside the coprocessor's widened MulDiv and the result is
at most `payout_received`, so it fits euint64; `total_joined > 0` because zero-total batches
cancel. The frozen `payout_rate` remains on the batch and in `BatchSettled`, but is informational
only and SATURATES at u64::MAX instead of failing settle (a redeem batch of few shares against a
large payout can legitimately exceed the u64 rate domain — a display number must not brick funds).

Settle's delta accounting is preserved as a security invariant on the redeem direction's
underlying-received phase: the payout is the batch payout account's SPL balance DELTA across the
vault CPI, never its raw balance, so preloaded tokens (which SPL destinations cannot refuse) stay
inert (pinned by `mollusk_redeem_preloaded_underlying_stays_inert` alongside the deposit-side
test). The dust-brick limitation above is deposit-only: the vault's share price never drops below
1:1 (floor rounding favors the vault; `harvest` only raises the price), so withdrawing any non-zero
share total always returns at least that many underlying units and `ZeroAssets` is unreachable from
a redeem batch (pinned by `mollusk_redeem_one_share_dust_settles_at_extreme_price`). Exit rules are
symmetric too: `quit` returns the exact encrypted share amount while pending; there is NO exit
between dispatch and settle in either direction — the deadline-cancel path stays out of demo scope
(fhevm-internal#1773). Operational assumption, both directions (fhevm-internal#1774 item 2): every
token/host CPI passes HCU accounts (`hcu_block_meter`, `hcu_trusted_app_record`) as hardcoded `None`.
`join`, `quit` and `claim` forward their remaining accounts as the execution's deny records, and
`cancel_dispatch` forwards them to the token's restore. `dispatch`, `open_batch` and `settle` pass
none, so they assume `grant_deny_list_enabled = false`. Every path assumes an unlimited
per-application block cap, which the deployer leaves in place; the per-transaction caps do bind.

## DD-043: Two Derivation Regimes — Content-Addressed Deterministic Handles, Persistent-Write-Anchored Rand Seeds (`context_id` deleted)

Status: adopted

Decision (fhevm-internal#1853 W3+W4). Handle derivation is unified on keccak (the recorded
2026-07-06 team position: EVM-side handle math is keccak, and both are same-price syscalls) and
split into exactly two regimes, mirroring `FHEVMExecutor`:

1. **Deterministic ops** (binary, ternary, unary, sum, is-in, mul-div, trivial-encrypt) are
   content-addressed: `H(domain, op, operand handles, fhe_type, program_id, chain_id,
previous_bank_hash, unix_timestamp)`. No `context_id`, no `compute_subject`, no `op_index` —
   an identical computation derives the identical handle, which is the same value by construction
   (EVM's exact behavior; a second party can only reproduce a result whose inputs it was
   independently authorized on, and the DAG is public in instruction data regardless).
2. **Rand / rand-bounded seeds** are compulsorily fresh. As amended by the Solana access control RFC:
   `H("FHE_eval_seed", rand_nonce, op_index, program, scope, host program id, chain_id,
previous_bank_hash, unix_timestamp)`. `rand_nonce` is the host's `RandNonce` singleton
   (`["rand-nonce"]`), which every execution with a rand step must pass and which the host
   advances (`FheExecuteRandNonceMissing` otherwise): a global counter, consumed once, never
   caller-supplied, so two executions in one slot cannot share a seed whatever they persist.
   `(program, scope)` is the execution's verified application (DD-047), so a seed is bound to the
   values it will land in. The host emits the resolved seeds through the event CPI
   (`FheExecutedEvent`, DD-056) so the listener needs no historical account read. The original
   design anchored freshness to the execution's persistent writes instead — every persistent
   output's live `(account, tag, handle, leaf_count)` in wire order — which forced a rand step to
   declare a persistent output; the nonce removes that requirement and the account-state
   dependence with it.

`context_id` is deleted from `FheExecuteArgs` (−32 B per batch), `EvalContextId` from the SDK, and
`transfer_eval_context` from the confidential token. Its two jobs are covered better: handle
domain separation was never needed for deterministic ops (content addressing), and rand freshness
was only caller-supplied _advice_ (two batches sharing a `context_id` in one slot derived identical
rand seeds), where the anchor is _enforced_.

Properties that must survive any refactor:

- The rand nonce is consumed exactly once per execution and advances monotonically; a reverted
  execution does not advance it or emit a usable seed.
- No seed-steering: the preimage is the host's own counter plus slot context plus the verified
  application; nothing in it is chosen by the caller.
- Duplicate accounts and second writes to one slot within an execution are rejected
  (`ExecutionAccountTable::new` and effect preflight), for the decode cache and the
  read-after-write rule. Seed freshness does not rely on it.

The nonce stays global (fhevm-internal#2081). Every execution with a rand step write-locks it, so
rand executions of all applications run one at a time; an execution without a rand step does not
take it. A nonce per application would remove that contention, at the cost of rent and a lazy
creation per application. Revisit it if rand executions become frequent enough to contend. The
preimage already binds `(program, scope)`, so that change would touch only the account.

## DD-044: Every Event Goes Through The Event CPI, Or Is Not Emitted At All

Status: adopted

Context:

A log can be truncated by whichever RPC provider a reader goes through, so an `emit!` event is a hint
rather than a delivery. That is fine for something a reader can reconstruct and not fine for
something it cannot. An IDL that declares an event the deployed build never sends also misleads
every reader that waits for it.

Decision:

There are two options for an event and no third. Either it is emitted unconditionally through the event
CPI, or it is not emitted at all and off-chain readers reconstruct it from instruction data over
Yellowstone, which is the normal path. `emit!` is not used anywhere in `zama-host`, and no cargo
feature turns events on or off.

Administration emits. An admin instruction changes a protocol-level setting that off-chain components
have to be able to query directly. The admin and config events are `HostConfigUpdatedEvent`,
`DenyScopeUpdatedEvent`, `HcuAppTrustUpdatedEvent`, `NewKmsContextEvent`, `NewKmsEpochEvent`,
`KmsContextDestroyedEvent` and `PauserUpdatedEvent`. Their instructions (`initialize_host_config`,
`define_kms_context`, `define_kms_epoch`, `destroy_kms_context`, `set_deny_scope`,
`set_hcu_app_trusted`, `set_admin`, the `HostAdmin` config setters, `pause`, `unpause` and
`set_pauser`) carry Anchor's `#[event_cpi]` accounts (`event_authority`, `program`).

`fhe_execute` emits one `FheExecutedEvent` per execution with what the host decided: the block
context, the random seeds and each step's result handle (DD-056). An indexer cannot recompute the
seeds and should not depend on its own copy of the handle derivation, so a block from any archive is
enough to ingest it. Its steps stay in instruction data.

Everything else is reconstructed from instruction data. That includes user-decryption delegation:
`delegate_for_user_decryption` takes a `delegator: Signer` and no admin, so it is a user action, not
administration. The KMS connector fetches and checks the delegation record on a delegated decrypt
(DD-048, DD-061; INVARIANTS #27).

Note that no in-tree component reads any of the six today; the only off-chain reader of host config
state reads the account, not an event (`solana-host-follower`'s `host_chain_id`). That is deliberate and is
not an argument against emitting them: the transport exists because the category calls for it, so that
a component which needs an admin change does not have to replay instruction data to find one. The test
is the category, not the current existence of a reader — otherwise the rule would flip every time
somebody wrote or deleted a reader.

`fhe_execute`'s event shares one emitter with the admin events (`event_cpi.rs`), instead of keeping
its own copy of the expansion.

Rationale:

Reliable delivery costs an account pair on the instruction and a self-CPI per emission. That is nothing
on an admin instruction, which runs when an operator changes configuration, and would be real weight on
one event per compute step — which is why the per-step shapes are still not emitted. Authorization never
rests on an event (DD-003). An execution
emits one event with what the host decided (DD-056); its steps stay in instruction data.

Anchor's `emit_cpi!` macro is not used, though the bytes it produces are. It reads a binding named
`ctx`, and most admin instructions emit through a shared `emit_config_updated` helper that has no
`ctx`; using the macro would mean copying the event literal into each of them. One
hand-written emitter takes the event authority as an argument and serves every call site.

The tag and the payload encoding come from anchor-lang, so they track upstream; only the assembly is
ours. What happens if the assembly itself drifts is worth stating precisely, because it is less than it
looks. A wrong tag fails the transaction: `dispatch` routes on the tag, and an unrouted instruction hits
the fallback. Past that, Anchor's generated `__event_dispatch` checks that the _first_ account is a
signer and is the canonical event authority — and nothing else. It never reads the event data, and it
ignores any account after the first. So an extra account or a changed payload encoding would not be
caught by the runtime at all. What catches those is two tests, and they are the reason the emitter
returns an `Instruction` as a value: `event_transport.rs`'s unit test asserts the built instruction's
program, account count, signer and writable flags, data length, and that its data is the bytes
`emit_cpi!` would send, and `host_mollusk.rs`'s
`sole_emitted_event` reads an event back out of the inner instructions and asserts one account, the
canonical authority, and every payload field. Those two cover `FheExecutedEvent` and
`NewKmsContextEvent`. Keep it that way: if they ever stop being covered, this becomes an unchecked copy
of an upstream wire format.

## DD-045: Keep Burn Settlement Sequential and Keep Wrapper Policy Separate From Host Governance

Status: adopted

Recorded in fhevm-internal#1862.

One confidential token account has one pending burn. `PendingBurn` is derived from `(mint,
token_account)`, not from a caller-selected identifier. A second burn is rejected before FHE work
until the first is settled by either `redeem_burned_amount` or `cancel_pending_burn`. Redeem pays the
certified underlying amount and closes the account. Cancel restores encrypted balance and encrypted
total supply, then closes it. The batcher exposes the same escape path: only the join mint's
`ConfidentialMint.authority` (the wrapper policy authority, not the Host upgrade authority) may
cancel a dispatch. Cancellation enters the terminal, refund-only `Refunding` state so participants
can quit but the batch cannot be reused.

Multiple pending burns for one token account were rejected for this release. They require identifiers,
ordering rules, and explicit protection against consuming the same burned state twice. No current
application needs that complexity: it can aggregate a three-way burn into one amount or use separate
app-owned token accounts. This is a deliberate deferral, not a claim that parallel settlement is
impossible.

`ConfidentialBurnEvent` carries no MMR `leaf_index`. Settlement binds the pending account to the
burned Store and handle and verifies the KMS certificate alone (DD-065), so no leaf index is an
authorization input.

The Host deny list applies only when the host seals an allow (DD-048). It does not block
redemption or cancellation. Applying it to settlement can trap funds after policy changes. Sealing
requires the Store authority; an allowed key is not an administrator.

The confidential mint authority is the wrapper policy authority. It can declare the allowed keys of
the encrypted total supply through token wrappers that sign the Host CPI as the total-supply
authority PDA. It is intentionally separate from the authority that upgrades Zama Host. Governance can later be
assigned to the mint authority without receiving Host upgrade power. Governance wiring and authority
rotation are not introduced by this decision.

The wrapper accepts classic Token and extension-free Token-2022 through `TokenInterface`. The
underlying mint's owner selects the token program, and mint/token-account ownership is revalidated on
every initialize, wrap, and redeem. Token-2022 mint
extensions fail closed; token accounts permit only `ImmutableOwner`. Frozen source or destination
accounts cannot cross the wrapper boundary. Confidential transfer and burn also reject a frozen
associated token account for each relevant owner (`from_ata`/`to_ata`/`owner_ata`). Uninitialized
at that address is treated as not frozen. Cancel
does not check freeze. Redeem destination ownership is not required: the confidential token-account
owner must sign, which is the theft check; `destination_usdc.owner == owner` was only a
no-unwrap-to-third-party policy and is not enforced.

What this freeze mirror does not reach (fhevm-internal#1981). Transfer checks both owners' canonical
underlying ATAs; burn checks the owner's ATA. Wrap and redeem check the SPL source and destination
accounts they move. These account-level checks do not establish a durable restriction on a holder:

- A holder who never held the underlying: funds arrive as a confidential transfer from a third party
  (an exchange paying out directly into the wrapped mint). No canonical ATA exists, absent reads as
  not frozen, and there is no holder ATA for the issuer to freeze. The shared underlying vault
  remains subject to its own freeze status.
- A holder who wrapped their entire balance: the canonical ATA is empty, and the issuer freezes an
  empty account. Classic Token and Token-2022 both allow closing a frozen account with a zero
  balance, so the holder can close it and recreate it unfrozen. The check rejects the frozen ATA
  while it exists, including when its balance is zero; closure removes that restriction.

The port retains these checks and their limitations. An issuer-authorized restriction on confidential
holders is one proposed response in fhevm-internal#1981, not an adopted design. Before launch, product,
compliance and the issuer must agree on the required behavior, authority authentication and rotation,
and treatment of pending burns. Neither the host admin nor the wrapper creator is automatically the
issuer. Host application-deny, pause and upgrade controls remain separate; these checks alone do not
establish Zama's compliance responsibilities.

Wrap credits use the same saturating `ge → select` pattern as burn debits (`tryIncrease` parity).
The clamp is unreachable while wrap stays 1:1 with an SPL `u64` vault.

A wrapper/mint pauser with governance unpause, and a mint-wide observer over every handle, remain
launch work. They are not in this release. Host pause and per-Store allows stay as they are.
Async delegated spend stays out of this program.

Supporting transfer fees, hooks, non-transferable tokens,
or Token-2022 confidential transfer requires a separate decision because each changes conservation or
transfer semantics.

Token-facing instructions pass a typed Host config account but do not redundantly derive its PDA at
the wrapper boundary. Every path that writes or computes immediately invokes a Host instruction that
enforces the canonical config and its pause flags (DD-058). This keeps the boundary aligned without
paying for a second PDA derivation or adding redundant IDL metadata.

Confidential accounts follow the associated token account model: canonical derivation, permissionless
create-for, and idempotent creation (`initialize_token_account`, like `CreateIdempotent`). An account
that already exists is left unchanged and its creator keeps the rent, so a client includes the
instruction without reading the account first. Because the owner does not sign, anyone can create the
next batch's token accounts before `open_batch` does, and the open still succeeds (pinned by
`mollusk_open_batch_accepts_precreated_batch_token_accounts`).

Disclosure publishes the certified handle and cleartext and reads no token state (DD-040). The
binding is checked when the handle is made public: `make_token_account_handle_public` names a
token-account state kind, and `make_total_supply_handle_public` is fixed to the total supply and
authorized by the mint authority. Both validate the mint scope, canonical Store and slot key, then
sign the host CPI as the Store authority. Scope-only validation was rejected
because two fields within the same mint would remain interchangeable.

## DD-046: The Program Heap Is Fixed At 32 KB — No Custom Allocator

Status: adopted

Recorded in fhevm-internal#1872.

No program in this repo installs a custom allocator. Every program keeps the default allocator
`solana-program-entrypoint` compiles in — a bump allocator over a fixed 32 KB region, never freed —
so `ComputeBudgetInstruction::RequestHeapFrame` is inert for us: the runtime grants the larger
frame and the allocator, whose length is a compile-time constant, cannot spend it. The heap is
also strictly per invocation: each top-level instruction and each CPI frame gets its own fresh
32 KB region (verified in `solana-program-runtime`'s `create_vm!`, which borrows a zeroed buffer
and installs a fresh bump allocator per invocation), so the app's heap and the host's heap in one
CPI are independent and neither can donate capacity to the other.

Why not ship an allocator:

1. The guild precedent (Pinocchio, 2026-06-25): a low-level win bought with permanent complexity
   is not worth it while the executor "doesn't do much compute at all" — stay on the framework
   default for now, revisit with a benchmark of the application that needs more heap.
2. The builder has typed limits for steps (`TooManySteps`), CPI instruction data
   (`ExceedsCpiInstructionDataLimit`) and its own requested heap
   (`ExceedsBuildHeapBudget`). Counting-allocator tests cover build, packet and invoke tables.
   These limits do not model live Store size or prevent the host from exhausting its separate
   heap; a runtime failure still rolls back the transaction. See INVARIANTS #54 and #61.
3. A Store output creates no account per result, so no per-result system CPI bounds an
   execution. The runtime snapshots show
   32-step dependent chains and 32 public outputs with eight viewers each reaching the step
   cap, and updates across Stores with 8, 32 and 55 MMR peaks reaching 16, 7 and 4 steps.
   These are shape limits; the allocator decision does not make
   a host heap failure acceptable for an application we intend to support. A failing application
   benchmark is grounds to reopen fhevm-internal#1872.

No feature lifts the SDK's on-chain step ceiling to the host's maximum: without an allocator, a
program doing so would keep the 32 KB allocator and land in exactly the silent abort the ceiling
exists to prevent.

Reopening condition: a benchmark showing a real application blocked by the measured shape
boundaries after the copy-reduction work (argument clone, decode-once, packet pre-sizing) landed.

## DD-047: The Application Is `(program, scope)` — Program Verified, Scope Owned By Program

Status: adopted

Recorded in the Solana access control RFC (zama-ai/tech-spec#448).

Context:

Everything the host had to attribute to "the app" — the HCU block meter and trust record, the
deny list, permit scoping, the rand seed — keyed on `compute_subject`, a signer the caller chose.
DD-039 closed the cheap rotation bypasses but left the honest statement that a caller with a
throwaway output account could still mint a fresh identity per execution, because nothing tied the
signer to a program. The EVM has no such problem: `msg.sender` _is_ the contract.

Decision:

The application identity is the pair `(program, scope)` carried by every encrypted store.
`program` is never taken on the caller's word: a Store's authority must be a PDA of `program`,
proven once when `create_encrypted_store` derives it from `authority_seeds`
(`EncryptedStoreAuthorityNotProgramPda`), and every later write needs that authority's signature.
Only `program` can sign for such an authority, so only `program` can write a value that claims it. `scope` is an
account of `program` that `program` names: the mint for the token program, the batch for the batcher, one of its
PDAs for a program with a single namespace. `create_encrypted_store` takes it as an account and requires `program`
to own it (`EncryptedStoreScopeNotProgramAccount`, added in zama-ai/fhevm#4120). A scope is therefore a real
address with one owner: two programs never share one, a program id is never one (the loader owns it), and the
wildcard delegation sentinel `0xff×32`, a delegation row's whole application, is never one. Both are seeds of the Store
(`["encrypted-state", program, authority, scope]`), so the identity is the address.

An execution runs as one application: every stored operand and output its default authority
controls must carry the same pair (`FheExecuteMixedScopes`). That pair is what the block meter
charges (`["hcu-block-meter", program, scope]`), the trust record names (`["hcu-trusted", program,
scope]`), the permit scopes to (`allowedScopes`), the rand seed binds, and the input attestation's
`contract_address` must equal (`program`). A value an additional signing authority admits (the
token writing a receipt into a batcher-owned value) keeps its own application and does not fold,
but the deny list is not scoped that way: a write is an allow in the value's own application, so
the execution passes the deny record of every application it touches (`["deny-scope", program,
scope]`), whoever signed for the value.
No caller-chosen compute subject exists in the host, the SDK, the token program or the deposit app;
reading a stored value into a computation is admitted by its authority's signature and nothing
else.

Rationale:

A PDA is the one thing on Solana that a program, and only that program, can sign for — the
`msg.sender` analog the port had been missing. Verifying the program through the authority costs one
`create_program_address` per output and buys an unforgeable identity with no registry: the program
is its own registry entry (the "Option B" DD-039 deferred). Letting the program name the scope
rather than deriving it keeps the host ignorant of app seed layouts while still letting the token
program meter per mint. Requiring an owned account instead of free bytes costs one owner comparison and gives the scope a
meaning a reader can check: a permit or a delegation names an account, not a number the program chose.
Every specimen program already had such an account at the call.

Consequences:

- Wallets cannot own Stores. A Store's authority is a program PDA, so the test suite drives two
  specimen programs (`encrypted-counter`, `dep-chain`) instead of a wallet-signed `fhe_execute`;
  the live operator matrix runs in the pure conformance layer (TESTING.md).
- The execution's application is known at build time: the SDK's `AppScope` is a builder input, and
  the deny record and meter an app must pass are derivable from it.
- FUTURE_DESIGN §2 (canonicalize the compute-authority-PDA convention) is resolved.
- **A self-declared identifier can carry consent but never coercion.** Since `program` declares
  its own `scope`, the pair works for every consumer where the _affected party_ signs the literal
  value it is agreeing to — permit `allowedScopes`, the admin-written trust record, Store identity
  — because nothing can be substituted for a value the party named. It cannot work for a consumer
  meant to restrain the program itself: a program with unbounded scopes has unbounded per-slot
  meters (INVARIANTS #41), and a `["deny-scope", program, scope]` record is escaped by declaring
  another scope. That is not an implementation weakness to tighten; whoever chooses the identifier
  cannot be bound by it. Rent is the only cost of a fresh scope, and rent is not a bound.
  Consequently the deny list holds against the case it is for — a legitimate application
  misbehaving, which cannot rotate away from the balances addressed under its own scope — and not
  against an attacker with no state to lose, for whom the levers are fees and the block cap. A
  lever that must bind a program regardless of its cooperation has to key on the one field the
  program cannot change, its program id (`["deny-program", program]`, or a program-level ceiling);
  neither is built, and EVM has no program-level ban either, so neither is a parity gap.
- **The owner check grounds the scope; it does not bind the program.** Because `scope` must be an
  account `program` owns, a program cannot forward a caller-supplied scope it does not own and so
  hand out trust or metering it was never granted, and a permit or a delegation names a real
  account. The check is unconditional and runs where a scope enters the host:
  `create_encrypted_store`, and `delegate_for_user_decryption` for a grant. Updates and metering
  read the stored scope (`preflight.rs` `fold_app`) and do not re-check it. It does not
  make the deny list or the meter binding: a hostile program can still create fresh accounts it
  owns, one per scope it wants.
- **A scope account must outlive its Stores.** Only a grant reads the scope account. If `program`
  closes or reassigns it, its Stores keep working and existing delegation rows keep authorizing
  until they expire or are revoked, but no new application-specific row can be granted for those
  Stores, only the wildcard row (DD-061).

## DD-048: Allows Are Sealed On The Write; The Deny List Names Applications; One Connector Path

Status: adopted

Recorded in the Solana access control RFC (zama-ai/tech-spec#448).

Context:

The account carried a mutable `subjects` list (capped at eight) beside the MMR, and a decrypt had
three paths: current membership, a historical leaf proof, a public leaf proof. The historical and
public proofs came from a standalone proof service, fetched by the client and embedded in the
signed request; the connector verified them, and the relayer passed them through. The deny list
named keys, and was consulted wherever a key joined the list.

Decision:

1. **Allows are sealed on the write.** A Store effect of `fhe_execute` (`FheExecuteEffect`) names
   the keys allowed on the result it records (`allow_indexes`). The host seals one
   `HistoricalAccessLeaf` per key in that order, then the `PublicDecryptLeaf` when the effect sets
   `make_public`. The Store keeps no allow list. There is no instruction to add or remove an allow
   afterwards — the next write declares the next handle's allows (the token program's
   `allow_balance_viewers` / `allow_total_supply_viewers` are exactly that: a re-write by the
   authority). A viewer is a viewer: it decrypts, and cannot grant, seal or write.
2. **One decrypt path.** A user decrypt proves the allow leaf; the current handle and a replaced
   one authorize the same way, with no separate path for the current one. A public decrypt proves
   the public leaf. Both proofs are fetched by the KMS connector from the coprocessors' leaf record
   (`POST /v1/solana/merkle-proofs`, signed by the KMS context's tx-sender, DD-067) and verified
   against the peaks the connector read on chain. The connector asks the coprocessors one after
   another in a random order: the next one as soon as an answer leaves a proof missing, or after
   `HEDGE_DELAY` (250 ms) without an answer. There is no retry inside an attempt. One stalled or
   unreachable coprocessor therefore delays a batch another one serves by at most that delay, and a
   coprocessor that serves the whole batch within that delay is the only one asked
   (fhevm-internal#2104).
   A request names only the Store and, for a delegated entry, the delegator as owner address; a
   client-supplied proof is rejected. A public decrypt names each handle's Store beside `extraData` (DD-060).
3. **Each coprocessor keeps the leaf record.** Its Merkle indexer recomputes the leaves from the
   finalized instruction stream into the Merkle proof service's own database (DD-066), and
   `solana_merkle_proof_server` serves them apart from ingestion (DD-064). There is no standalone
   proof service: the relayer passes no proofs through, and the SDK has no RPC evidence or
   proof-service client (DD-035 superseded).
4. **The deny list names applications.** `set_deny_scope` writes `DenyScopeRecord` at
   `["deny-scope", program, scope]`; it gates every allow the host would seal — each `fhe_execute`
   and `make_store_handle_public`, because sealing a public leaf is an allow. The deny list names no
   keys: an application is denied, or it is not.

Rationale:

A stored list was a second source of truth beside the MMR, needed a cap, needed admin
instructions, and made "current" a special case the connector had to read live. Sealing every
allow as a leaf leaves one authorization fact per (handle, key), permanent, proven the same way
whether the handle is current or replaced. Fetching proofs connector-side removes the client from
the trust path entirely — the client could never authorize anything, but it could carry stale or
malformed evidence into a signed request — and lets each coprocessor, which already follows the
host's instructions, keep the record instead of a standalone service. Denying an application
rather than a key matches what a host operator can actually judge (a program and its scope) and
what the EVM `blockAccount` denies in practice (a contract).

Consequences:

- Account layout: `121 + 64·slots + 32·peaks`, at most 4,217 bytes (`EncryptedStore::account_size`,
  INVARIANTS Part II).
- `fhe_execute` wire: `allow_indexes` per Store effect and no subject lists; the deny record an
  execution passes is its application's.
- The connector pipeline is one explicit sequence with one observation point
  (`kms-worker/src/core/solana/pipeline.rs`); the relayer pre-checks dead delegation rows
  advisorily and nothing else (INVARIANTS #50).
- Delegation records are consumed (INVARIANTS #27 closed).
- A handle with no allows and no public leaf is undecryptable by everyone, including its author;
  that is the author's choice, not a stranding.

## DD-049: Shared Encrypted Store And Transaction-Local Result Grants

Status: adopted

Adopted with the Solana access control RFC (fhevm PR #3883).

A host-owned `EncryptedStore` PDA uses `(program, authority, scope)` identity, with bounded
slot keys and one shared MMR. Store creation proves the program-owned authority; execution
requires its signature. Slot keys are not PDA seeds. Each batch participant's JoinRecord
is the authority of its contribution Store, scoped to the batch.

`Store` outputs independently choose a slot write, exact-handle private/public permission
leaves, and transient store grants. Transient grants authorize an exact produced handle for a consumer
Store whose authority must sign use. The transient store opens and closes in one transaction, with a
mandatory final top-level close and the recorded rent refund destination. It is not a decryption
permission or a restriction on what authorized computations can subsequently reveal.

Execution-level `returned_results` selects `(step_index, output_index)` pairs; current operations
have output index zero. At most 32 handles are returned in requested order, including duplicates;
empty selection returns none. Return bytes do not authorize use. Token transfer returns its
result and the batcher performs its own contribution update; there is no transferred-amount
register or token-owned accumulator API. Burn retains its result slot and PendingBurn lifecycle.

Decryption names each handle's Store beside the KMS routing (DD-060). The KMS connectors check
exact-handle MMR proofs, and no on-chain instruction takes one (DD-065). Current-slot publication
and fresh slotless permissions are supported. Adding new private/public permissions to a
history-only handle is deferred to fhevm-internal#2007. Generic disclosure verifies the KMS
certificate and emits the certified handle and cleartext; it reads no Store and no token-kind
label. Original token events establish provenance.

The input-attestation, threshold-KMS, program-upgrade and RPC trust
assumptions apply. Resource limits are shape-dependent; see runtime cost snapshots.

## DD-050: Transient Storage Shared Across The Transaction

Status: adopted

Implemented for fhevm-internal#2003, building on DD-049.

`TransientStore` is payer-derived and opened once at the top level, before any app calls. Every FHE invocation validates
that same transient store against the exact final top-level close. Payer identity controls funding/refund only. The bounded
zero-copy account holds 112 produced occurrences, 32 explicit grants and transaction HCU total; every occurrence
records its producing Store and depth. No resizing, initiating-Store credential or client session nonce is needed.

Each `fhe_execute` explicitly names its producing Store. That Store implicitly may use every result from its execution,
including unstored intermediates, across calls in this transaction. Foreign Stores need an explicit grant and must
sign consumption. Ordered arithmetic and ordered effects are separate; typed Rust expressions select effects with
`fhe.output(result, state.set(key)...)`. Executions keep initial slot snapshots, duplicate-write rejection and ordered
MMR cursors.

Production membership determines operand origin independently of the supplied witness. The 256-bit big-endian mask
enters operand-bearing handle preimages; bit 0 marks input position 0. The listener reconstructs membership per
transaction. HCU total and depth use the same journal, while each application's block meter receives only that call's
cost. Return data remains immediate CPI transport, independent of permissions and result storage.

A transaction opens one transient store, apps pass no result-scratch or result-authority accounts, the host meters each
execution once, and balances need no add-zero copy. Resource snapshots include lifecycle CU overhead and separate
whole-transaction packet checks. Granting decrypt permission on historical handles is fhevm-internal#2007.

## DD-051: A Zama Is One Host Program ID

Status: adopted

Adopted for identity. zama-host closes its accounts through `close_owned_accounts` (`admin-sweep` builds only) and the deployer's `host wipe`.

HostConfig is that program's singleton `PDA("host-config")`. Four public `zama-host` program IDs:

```text
mainnet        one program; Squads will be the upgrade authority
zama-devnet    one program on Solana devnet
zama-testnet   a second program on Solana devnet
preview-env    DPq5y89RDZPq9NcMh9X1NgjBWgYmSXg3QoipSBV3ZMzQ on Solana devnet
```

Localnet e2e loads the preview-env build at validator genesis under the same ids, with the
deployer wallet as upgrade authority. That is CI, not a fifth Zama, and it needs no program keypair.

A handle binds `(program_id, chain_id)` and every host PDA is derived under `program_id`.
`program_id` is the Zama, compiled as `crate::ID`. `chain_id` is the Solana cluster and lives in
HostConfig. The `u64` is DD-052. No PDA seed includes it. Two programs on Solana devnet therefore do not accept each
other's proofs. EncryptedStore addresses already seed the app program, authority and
scope under `crate::ID`. Putting HostConfig in those seeds would make a second config account a
second Zama under one program ID, which this decision rejects.

The program ID is compiled into the `.so` (`declare_id!`) from `solana/environments/<name>.json`
(DD-053): `preview-env` is `DPq5y89…`, and it is the one id the repository's clients carry, on
Solana devnet and on the test validator alike. Mainnet, zama-devnet and zama-testnet will each
need their own compiled id the same way, so shipping those Zamas is one build per environment
file, not a runtime switch, and CI does not generate the keypairs. Each deployed program will have
its own upgrade authority, so a preview-env workflow cannot replace zama-testnet's bytecode.
Whether the durable Zamas should instead share the mainnet id per cluster, as public Solana
programs do, is open (fhevm-internal#2055).

`zama-zws/gitops` does not deploy the preview-env program. GitOps provides the Kubernetes cluster,
the Solana RPC credentials, and the deployer keypair. The preview-env GitHub Actions workflows are
the only intended writers of `DPq5y89…`. Durable zama-devnet, zama-testnet and mainnet are later
GitOps environments on the other three IDs.

### Preview-env wipe

Only one preview namespace should use `DPq5y89…` at a time. Each namespace deploys its own Anvil
Gateway, and HostConfig stores that Gateway's chain id, InputVerification address, Decryption
address and coprocessor signers, so leftover HostConfig from the previous namespace cannot bind
the next one.

`destroy_kms_context` only sets `destroyed = true`, and no production instruction closes HostConfig
or EncryptedStores. `close_owned_accounts`, compiled only behind the `admin-sweep` feature that
`preview-env.json` enables, takes raw account addresses as remaining accounts, skips the ones
this program does not own, and returns the rent to the signer, who must be the program's upgrade
authority. The accounts are untyped because an old byte layout would fail to deserialize, and that
leftover is what the wipe must delete. Solana has no parent account: closing HostConfig leaves
EncryptedStores, KMS contexts and the rand nonce in place until the same instruction closes each
of them, so the deployer's `host wipe` closes everything `getProgramAccounts` lists and fails if
anything remains. `initialize_host_config` also creates the rand nonce, so both addresses must be
empty or the next init fails. The demo programs carry the same preview-only instruction, and the
preview recovery closes their accounts before it wipes the host.

`deploy-preview.sh` runs the preview recovery, then `host deploy --allow-upgrade`, which uploads this
`.so` when the bytecode differs and runs `initialize_host_config` and `define_kms_context` for this
Gateway; a plain `host deploy` refuses differing bytecode. `preview-env-destroy.yml` runs the same
recovery before it deletes the namespace and does not initialize. Before a reset, the recovery
upgrades each deployed program to the recovery image's build, so `close_owned_accounts` is present.
Pull-request CI runs on the test validator and does not touch the `DPq5y89…` on Solana devnet.
Durable GitOps environments will upgrade bytecode in place on their own program IDs.

Coprocessor `host_chains` uses `chain_id BIGINT PRIMARY KEY`. That collides only if one
coprocessor database indexes both zama-devnet and zama-testnet. Separate databases, one per Zama,
do not need a schema change. The Solana access control and Solana user decryption RFCs do not change.

## DD-052: A Solana chain id is type byte `0x01` plus a published cluster tag

Status: adopted

This entry is the encoding the tree uses. `SOLANA_POC_CHAIN_ID` is
`solana_host_chain_id(12345)` (`0x0100000000003039`), and `is_solana_host_chain_id`
matches high byte `0x01`.

`chain_id` names the Solana cluster (DD-051), not a Zama. Preview on Solana devnet uses the
solana-devnet row. Two Zamas on one cluster share the number and differ by `program_id`.

The chain id is a `u64` because handle bytes 22–29 (`handle[22..30]` in Rust) and coprocessor
`host_chains.chain_id BIGINT` already store that width. `host_chains` also checks
`chain_id >= 0`. Type byte `0x01` stays a positive `BIGINT`. Bit 63 is a negative `i64` and
would fail that check. EVM ids are unchanged (Sepolia `11155111`, Anvil `12345`): they sit
below `2^56`, so their type byte reads as `0x00`.

```text
bits 56..63  chain type    0x00 = EVM, 0x01 = Solana
bits  0..55  cluster tag
```

Public Solana rows take the first seven bytes of the cluster genesis hash (the raw 32-byte
hash, big-endian), not the CAIP-2 32-character base58 prefix. Localnet has no stable genesis:
`solana-test-validator --reset` mints a new one, so that row is a pinned sentinel. A type
byte of `0x01` is outside JavaScript `Number` (`2^53`), as bit 63 already was. Solana ids
travel as `bigint` or hex.

| Row            | Low 56 bits                                            | `u64`                |
| -------------- | ------------------------------------------------------ | -------------------- |
| solana-mainnet | genesis `5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d` | `0x0145296998a6f8e2` |
| solana-devnet  | genesis `EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG` | `0x01ce59db5080fc2c` |
| solana-testnet | genesis `4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY` | `0x013a132ece10305e` |
| localnet       | pinned `12345`, not a hash                             | `0x0100000000003039` |

This table is the assignment. The deploy workflow selects a row by its name, such as
`solana-devnet`, and passes that one integer to `initialize_host_config`, the listener and
the connector. Nothing invents a second integer.

`initialize_host_config` requires type byte `0x01` on the host `chain_id` and `0x00` on
`gateway_chain_id`.
HostConfig then holds the chosen row. The listener, connector and relayer must use that
same value. The listener reads `chain_id` from HostConfig rather than from its own config. A
deployment on a named public row may also compare RPC `getGenesisHash` with the hash above to
confirm it is on the intended cluster, without that comparison defining the id.

#1880 proposed this type byte and the genesis recipe. This entry accepts both and writes
the numbers down. It rejects deriving localnet from RPC at boot, and it rejects treating
“e2e targets any cluster” as part of the id.

This entry supersedes the chain-type marker in DD-026, the bit-63 detector in DD-027,
the chain-agnostic address RFC’s high-bit reservation as the long-term marker, and the open-product #1635
sentinel. DD-026 still owns bytes32 input identity and typed user-decrypt. DD-027 still
owns EVM-strict vs Solana-relaxed validation. Both keep calling `is_solana_host_chain_id`;
this entry is that predicate: high byte `0x01`.

## DD-053: A program id is environment config, not a cargo feature

Status: adopted

An Anchor program checks its own id at the entrypoint (`declare_id!`), so one `.so` serves one
program id. DD-051 makes each Zama one host program id. Together they mean one build per Zama.
This entry fixes how that id reaches the build.

The id is read at build time from `solana/environments/<name>.json`, one file per deployed
environment. A `build.rs` in each program (`crates/program-environment`) writes the `declare_id!`
line from the file named by `PROGRAM_ENVIRONMENT`. `.cargo/config.toml` pins that variable to
`preview-env` (`force = true`), so for cargo invoked in this workspace a shell export cannot change
what a build compiles; only the `--config` override that `scripts/build-programs.sh` passes does,
and such a build warns. Cargo reads config from the invocation directory, so a build of
`zama-host` as a path dependency from another workspace (the coprocessor images) is not pinned;
no off-chain code reads the compiled id, so those images' id is not load-bearing.
Plain `cargo` and `anchor build` therefore produce the shipped program. The deployer image and CI
build the same environment; `scripts/build-programs.sh` also enables the cargo features the file
lists per program (`features.zama_host`).

```text
solana/environments/preview-env.json   the one id set today: Solana devnet and the test validator; enables admin-sweep
```

Localnet is not an environment. A Solana program has one id on every cluster, and the test
validator loads the preview-env build at genesis (`solana-test-validator --upgradeable-program
<id> <so> <deployer>`), so the deployer finds the bytecode in place, only bootstraps, and upgrades
exactly as on devnet. The repository holds no keypair for the deployed programs; the generated
clients, the IDLs and `Anchor.toml` carry the preview-env ids. The earlier tree compiled a second,
localnet id set from committed keypairs, which made every client default to ids that existed only
on the test validator and would have forced a program-id parameter through the demo vault module
and the harness.

The file holds program ids and build features only. Chain id, RPC and keypairs are runtime
config and stay in Helm values and the environment's secret store (DD-052 for the chain id).

Why not the alternatives:

| Option | Why not |
|---|---|
| `#[cfg(feature = "<env>")]` per Zama (the tree before this entry) | The id lived in `lib.rs` × 4, `Anchor.toml` and the deployer. Any crate linking `zama-host` inherited whichever feature was on, so a listener built without the preview feature reconstructed handles under the localnet id. Each Zama added `#[cfg]` lines to four crates. |
| `anchor keys sync` | Rewrites source in CI, and `Anchor.toml` is keyed by cluster. zama-devnet, zama-testnet and preview share Solana devnet, so they collide. |
| One id on every cluster (Token-program style) | Adopted between localnet and preview-env. Across Zamas it means one Zama per cluster, and zama-devnet and zama-testnet both target Solana devnet, so they keep distinct ids (DD-051); fhevm-internal#2055 tracks whether that stays. |
| Zama identity in HostConfig, one program id | Rejected in DD-051: tenants under one program share handle space and PDA seeds. |

Off-chain code never needs the id at build time. The connector, relayer and SDK derive PDAs from
configured ids, and the listener hashes handles with the `--program-id` it follows (#4043). Adding a
Zama is a new JSON file, its entry in `SOLANA_ENVIRONMENTS` (`deploy/src/environment.ts`, a static
import because the deployer ships as one bundle) and deployer keypairs. No Rust change. A wrong id
in the file flows consistently into the `.so` and the deployer and stops at deploy time, where the
deployer refuses a program keypair whose pubkey differs from `declare_id!`.

## DD-054: The programs stay on Anchor v1

Status: adopted

Recorded in fhevm-internal#2094.

Every program is written with Anchor v1, pinned to a stable release in `Anchor.toml`, and builds
with the platform tools and SBPF target that release selects. New code uses Anchor's typed
accounts and constraints where they express the check; a check written by hand is a local choice
of that handler, not a step towards leaving the framework.

| Option | Why not |
|---|---|
| Hand-written Pinocchio | The guild precedent of 2026-06-25 (DD-046): permanent complexity for programs that do little compute. |
| Anchor v2 (`lang-v2` on `anchor-next`) | Alpha: not audited, not on crates.io, APIs break between commits. It is the planned successor. |
| Quasar | Beta, unaudited, no release. Not a production candidate. |

Pinocchio-level cost should come from the framework, and neither newer framework can be the code
we audit and ship.

Reopening condition: Anchor v2 published on crates.io with an audit. Measure a port against the
runtime cost snapshots first: `Account<T>` becomes a Pod layout and `EncryptedStore`'s `Vec`
fields become a `Slab`. A port after the external audit needs its own audit.

## DD-055: The ledger is the work log, not a PDA queue

Status: adopted

Recorded in fhevm-internal#2079.

The host listener rebuilds all coprocessor work from sealed Yellowstone blocks at `finalized`. The
alternative is a work queue on-chain: each `fhe_execute` writes its payload into a temporary PDA, the
coprocessor marks it computed, and the user closes it for a rent refund. Work would then stay
on-chain until handled, so a listener that missed it could find it again. It is rejected:

| Cost | Why it does not fit |
|---|---|
| Completion needs coprocessor transactions on Solana | They would need threshold signatures and fees. FHEVM has no such path: an EVM host never learns that a computation finished, and completion lives on the Gateway. Refunds would depend on coprocessor liveness. |
| Order is lost | The coprocessor needs the order of dependent operations, which a block gives. A global sequence counter is one account every FHE transaction writes, so all apps would execute one at a time. A counter per Store does not order handles that move between programs. |
| A stream is still needed | Low latency needs an account subscription, and `getProgramAccounts` scans are heavy and throttled. The listener would have two ways to discover work. |
| Rent and a second transaction per execution | A 1 KB payload locks about (128 + 1,024) × 6,960 lamports, roughly 0.008 SOL, until someone closes it. |

The ledger is already the durable log. The risk is a listener that falls behind until the
provider's replay window closes, and the answer is to see it early: the listener exports its lag and
reconnects (fhevm-internal#2079), and a self-describing execution makes archive replay possible
(fhevm-internal#2081), which the listener now uses to catch up past the window (DD-059).

## DD-056: An execution describes itself; the listener re-derives handles only as a check

Status: adopted

Recorded in fhevm-internal#2081. Revises DD-033 and DD-044.

Context:

`fhe_execute` used to emit only its random seeds, and only when a step was random. The listener
rebuilt every other result handle from the decoded step and a block context that no transaction
carries: the parent bank hash from the `SlotHashes` sysvar and the timestamp from `Clock`. It
streamed both accounts from Yellowstone next to the blocks and joined them per slot. So it learned
an execution's outputs in two ways, and one of them needed live state. A missed slot could be
rebuilt from an archive only by reading the bank hash out of later vote transactions and trusting
that `getBlock`'s `blockTime` equals `Clock`. Neither is a contract.

Decision:

Every `fhe_execute` emits one `FheExecutedEvent` through the event CPI, after its account writes:
the event version, the parent bank hash, the Unix timestamp, the result handle of each step in step
order, and the seeds of the random steps. The program id and the chain id are fixed per deployment
and stay out. The instruction carries what the caller asked for; the event carries what the host
decided.

The listener and the Merkle indexer both follow the host through `solana-host-follower`, which pairs
each host `fhe_execute` with the one `FheExecutedEvent` from the host program that follows it before
the next host `fhe_execute`. Only the host can sign its event authority, so an app cannot forge the
event inside the host's instruction trace. The listener stores the emitted handles: computation
rows, operands that name an earlier step and allowed handles all use them. The Merkle indexer
records the leaves with them too (DD-066). The listener then re-derives each handle from the decoded
step, the emitted context, the followed program id and the chain id, and compares. The listener
subscribes to no sysvar and joins nothing per slot, and the block time it records is the one
Yellowstone sends with the block.

What the listener does when something does not line up:

| Case | Response |
|---|---|
| A host `fhe_execute` without exactly one event of the current version, or an event whose results do not match the execution's steps | Fatal: the block is not applied and the checkpoint does not move. The listener and the program disagree on the wire format. |
| A step whose emitted handle does not re-derive | The block is applied. That step is held back: its computation row is inserted as a terminal error, so the tfhe-worker never computes it and ends its dependents as errors. Allowed handles keep the emitted handle, and so do the leaves the Merkle indexer records. After the commit the listener logs the slot, signature, execution, step and both handles, and counts `coprocessor_solana_host_listener_handle_check_failures_total`. |

A mismatch means our software is wrong, and the listener cannot tell which part. If the derivation
drifted, the ciphertext it would compute is still right. If it misdecoded the step, the ciphertext
is wrong, and stored under the chain's real handle it would decrypt to a wrong value and spread to
everything computed from it, with no visible failure. Holding the step back keeps the damage to
that handle and what is computed from it. Refusing the block instead would stall every app. The emitted handle wins
for the leaves because proofs must keep matching the peaks on chain.

The check finds our bugs, not a hostile provider: a provider that lies can forge the event and the
transaction consistently.

Repair is a replay of the affected slots with the fixed listener. The operator rewinds the listener
checkpoint to a slot `S` before the failure (`rewind_solana_listener_checkpoint.sql`) and reverts
the rows above that block's height `H` with the existing `revert_coprocessor_db_state.sql`, which
deletes the held rows and their errored dependents. Rows are numbered by height (DD-071). The revert
refuses a Solana chain whose checkpoint is not the block at `H`: from a checkpoint above `H`, the
listener would never re-ingest what it deleted. `revert_coprocessor_db_state.sh` runs both when
given `SOLANA_SLOT` and `SOLANA_BLOCK_HASH`. On restart the listener replays from `S` and inserts
the rows again as new work. The replay leaves the leaf record alone:
the Merkle indexer keeps it in its own database and skips its checkpoint block when the hash
matches (DD-066). So a replay repairs computation rows, not a bug
that recorded wrong leaves. A replay older than the provider's replay window comes from the archive
RPC (DD-059), so `S` must be in the archive's history, and nothing checks that before the rows are
deleted. The runbook is in the host-listener README.

An archive transaction prepares into the same host instructions as a streamed one
(`prepare_rpc_transaction`), so archive catch-up (DD-059) reuses one decoder. It requires inner
instructions and loaded addresses, which `getTransaction` returns (DD-062). A test rebuilds a slot
from `getBlock` and `getTransaction` output alone.

Rationale:

The seeds already had to travel in an event because an indexer cannot recompute them (DD-043). The
block context is the same kind of fact, and emitting it removes the live sysvar stream. The result
handles go in too, so that what the listener stores does not depend on its own copy of the
derivation; the derivation becomes a check that can fail without corrupting the leaf record.

Cost, from the committed cost snapshots: the event CPI adds 1,859 CU and 161 bytes of CPI data to a
three-step execution that used to emit nothing, and 3,981 CU and 1,089 bytes at the 32-step limit. A
confidential transfer costs 2,129 CU more. The event is built on the host heap, so two heap-bound
shapes run one step fewer: `attestation_per_step` 16 instead of 17 and `mature_updates_peaks_8` 15
instead of 16 (INVARIANTS #61). The event CPI also runs one level below `fhe_execute`, which used to
be true only of random executions. Under Solana's invoke stack limit of five (nine once SIMD-0268 is
active), the host must now be invoked at height four or less: the top-level program and at most two
programs between it and the host. The batcher's path (batcher, token, host, event) uses four.

Consequences:

The host emits no `FheExecuteRandomSeedsEvent`: `FheExecutedEvent` carries the seeds. The listener
needs no historical sysvar state from its provider, only blocks. A held step still gets its material
request, since the Store write is real on chain. The automated drift revert, which runs the same
revert SQL, fails on a Solana chain whose checkpoint is ahead, which is always the case when drift
is detected, so it never deletes rows the listener would not re-ingest. A failed revert signal stops
every coprocessor service on that database from starting, including those of EVM chains, until an
operator repairs by hand.

## DD-058: Pausers stop one area at a time; only the admin resumes

Status: adopted

Recorded in fhevm-internal#2088. Replaces the single admin-set `HostConfig.paused`.

Context: the host had one pause flag that only the admin could set or clear. Governance is moving
the admin behind a Squads vault with a time lock (fhevm-internal#1634), so an admin-only pause
would wait out that time lock too. On EVM, `ACL.pause()` accepts any member of `PauserSet`, while
`unpause()` is `onlyOwner`. The host ACL pause stops `allow`, `allowForDecryption`,
`allowTransient` and both delegation calls, and so execution, which needs `allowTransient`. The
gateway's `InputVerification` and `Decryption` contracts each pause on their own, and Solana requests
enter the gateway through the same paused calls (`verifyProofRequestSolana` and both decryption
requests), so the gateway pause already stops new input proofs and decryptions for Solana.

Decision: `HostConfig.paused` is `PauseFlags`, one flag per area.

| Flag | Stops | EVM counterpart |
|---|---|---|
| `execution` | `fhe_execute`, with the allows, transient grants and public releases it writes, so every token instruction that computes (account setup, transfer, viewer grants, wrap, burn, cancel) and the batcher's settle through its wrap | ACL pause |
| `verified_inputs` | `fhe_execute` steps that consume a `VerifiedInput` | None: `InputVerifier` cannot be paused; the gateway pause stops only new proofs |
| `acl_writes` | `create_encrypted_store`, `make_store_handle_public`, `delegate_for_user_decryption`, `revoke_delegation_for_user_decryption` | ACL pause |

`verified_inputs` acts when a signed input is used, not when it is requested. It is the lever
against compromised coprocessor signers, whose results the gateway pause cannot recall.
`verify_public_decrypt` never pauses, as EVM's `KMSVerifier` (fhevm-internal#2096). The gateway
pause stops an honest KMS from signing new certificates. Against a compromised KMS context, the
admin makes a new context current, then `destroy_kms_context` revokes every certificate the old one
signed, as EVM's owner-only `destroyKmsContext`, which also refuses the active context.

A pauser is a `PauserRecord` PDA `("pauser", key)`, which the admin creates, enables or disables
with `set_pauser`, as it does deny and HCU-trusted records. `pause` takes the pauser's signature and
its enabled record, and sets the flags it names. A wallet pauser must call `pause` at the top level,
by the rule `delegate_for_user_decryption` applies to a wallet delegator: a wallet's signature reaches
every program of the transaction it signed, while EVM's `msg.sender` check keeps a called contract
from pausing with a pauser's right. A PDA pauser, such as a Squads vault, pauses through CPI, as only
its own program can sign for it. `unpause` takes the admin and clears them. As on
EVM, the admin pauses only if it also holds a pauser record. Pausing an area already paused
changes nothing and emits nothing. A change emits `HostConfigUpdatedEvent`, whose `signer` names the
pauser or the admin. `set_pauser` emits `PauserUpdatedEvent`.

Programs act on KMS results only through `verify_public_decrypt`. The token reads no pause flag and
passes the config through to the host.

Admin setters are never paused. `revoke_permits` takes no config account, so it runs under every
flag, as EVM's `invalidateDecryptionSignaturesBefore` runs under the ACL pause.
`revoke_delegation_for_user_decryption` is gated on `acl_writes`, as EVM's
`revokeDelegationForUserDecryption` is `whenNotPaused`.

Accepted gap: no host flag stops user decryption. During a host pause, user decryption of values
already allowed continues, as it does on EVM while only the host ACL is paused; the gateway pause is
what stops it. While `acl_writes` is paused, a delegator cannot revoke a delegation until the admin
resumes ACL writes.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Pausers as a list in `HostConfig` | A fixed maximum and a realloc for every change. One record per pauser matches the deny and HCU-trusted records and keeps `HostConfig` fixed-size. |
| Keep one flag and add pausers | An operator could not stop compromised coprocessor inputs without also stopping every application's execution. |
| A flag that stops `verify_public_decrypt` | EVM's `KMSVerifier` has no pause. It would also strand redeems and disclosures that hold valid certificates; `destroy_kms_context` revokes a compromised context's certificates instead. |
| Let the admin pause without a record | EVM requires `PauserSet` membership for `pause()` even from the owner. Keeping that rule makes the pauser set the one list of who can pause. |

Consequences: `PauseFlags` is three bytes of `HostConfig` (`HostConfig::SPACE` 311). The host
listener decodes `HostConfig` with the program's type. The KMS connector reads no pause flag, so user
decryption pauses at the gateway alone, as on EVM. The pause errors are `ExecutionPaused`,
`VerifiedInputsPaused`, `AclWritesPaused`, `NotPauser` and `WalletPauseThroughCpi`.

## DD-059: The listener catches up from an archive when the stream cannot replay

Status: adopted

Recorded in fhevm-internal#2085.

A listener that was down longer than the provider's replay window, about a day on a hosted provider,
catches up from an archive RPC and then returns to the stream. When Yellowstone refuses the
checkpoint as outside its window, the listener lists the produced slots after it with `getBlocks`
and fetches each block's transactions with `getBlock` and `getTransaction` (DD-062), all at
`finalized`. Each fetched transaction is reduced to its host instructions as a streamed one is
(`prepare_rpc_transaction`). The block must extend the checkpoint as the stream's validator
requires: an unapplied checkpoint first and unchanged, then each block naming the last applied one
as its parent. It is applied through the same path as a streamed block. Catch-up stops at the slot
the archive had finalized when it started, so it ends however fast the chain moves. There the
listener subscribes again from its checkpoint, and the stream's own replay check takes over; if that
catch-up outlasted the window, the next pass is shorter.

The archive is `--archive-url`, which defaults to `--url` and may be another provider's. A provider
that cannot replay from a slot at all (`from_slot is not supported`) still stops the listener: after
catch-up, the stream could never take over. An archive behind the checkpoint, an archive missing
slots after it (a block whose parent is later than the checkpoint, or, for an unapplied checkpoint,
a later block descending from it), or a failed read, is retried like a dropped subscription.
Any other block that does not extend the checkpoint is a fork and stops the listener, as on the
stream. `archive_catch_up_active` is 1 during catch-up, and the existing lag
metrics show its progress.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Subscribe first and backfill alongside, as the EVM listener does | `eth_subscribe` cannot resume from a block, so EVM must backfill beside a live stream. Yellowstone resumes from a slot inside its window, so one ordered source at a time keeps a single checkpoint and a single ancestry rule. |
| Fetch only the slots holding host transactions, found with `getSignaturesForAddress` | The checkpoint needs every block's hash to check ancestry, and the address index adds a second completeness assumption. It is an optimization for later if catch-up volume matters. |
| Several gRPC endpoints with failover | EVM has none. With catch-up, a provider outage only delays processing, and switching provider is a configuration change and a restart (fhevm-internal#2085). |

Consequences:

Catch-up lists every block after the checkpoint. On mainnet a day is about 216,000 blocks, so it
takes hours and needs an archive provider whose rate limits allow it; it fetches eight blocks at a
time, each with up to eight `getTransaction` calls in flight. Its reads accept every transaction
version (`maxSupportedTransactionVersion: 1`), because the RPC refuses a whole block that holds a
later version than the request allows. Each transaction is fetched as JSON, which spells legacy, v0
and v1 messages alike. A slot rewound for repair (DD-056) need not be inside the replay window, only in the
archive's history.

## DD-060: A public decrypt names its stores beside the KMS routing

Status: adopted

Recorded in zama-ai/fhevm#4120 and zama-ai/fhevm#4357.

`extraData` is the KMS routing field on EVM, and the KMS signs it. A Store carried inside it would
become part of a signed field whose version space EVM owns, and every layer (relayer, Gateway,
connector, host verifier, SDK) would parse a Solana-only version.

The Gateway has a Solana entry, `solanaPublicDecryptionRequest(bytes32[] ctHandles,
bytes extraData, bytes32[] encryptedStores)`. It takes one Store per handle, in handle order, and
emits `SolanaPublicDecryptionRequest(decryptionId, ctHandles, extraData, encryptedStores)`. It is
named rather than overloaded, so the EVM `publicDecryptionRequest` keeps its generated binding
names. `extraData` holds only KMS routing (v0, v1 or v2) on both chains. Before it takes the fee,
the Gateway refuses handles that are not of one registered Solana host chain and a store count that
differs from the handle count. The relayer requires `encryptedStores` for Solana handles and
refuses it for EVM handles; the stores are part of the request's content hash. The
connector keeps them in `handle_encrypted_stores` and proves each handle's public leaf against its
own Store in one snapshot. The host verifier reads the context from v1 or v2 exactly as EVM
`KMSVerifier` does, and reads no Store (DD-065). The SDK sends v2, `0x02 ‖ contextId ‖ epochId`, with
the pair read from `HostConfig.current_kms_context_id` and `current_kms_epoch_id`, as the EVM SDK reads
`ProtocolConfig.getCurrentKmsContextAndEpoch()`. `define_kms_context` sets both; `define_kms_epoch`
moves the epoch of the active context when the KMS reshares its keys. Before the SDK verifies a
certificate, it requires the certificate's `extraData` to be exactly that v2 routing.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Carry the stores in a version-4 `extraData` | The KMS would still sign Solana account addresses inside a field whose versions EVM defines. The Store is not a KMS routing input. |
| Put the stores in an opaque blob, as `solanaRequest` does for user decryption | The user request needs one signed blob for its permit. Public decryption has no signature to bind, so named typed fields are easier to check at every layer. |

Consequences:

The KMS certificate does not commit to the Store; the KMS connectors check each handle's public leaf
against it. The Solana entry
shares the decryption counter and the fee with the EVM entry. It emits one event, so the relayer
looks for the Solana request event when the EVM one is absent from the receipt.

## DD-061: A delegation is keyed by application and expires on Unix time

Status: adopted

Recorded in zama-ai/fhevm#4120.

A user-decryption delegation lets a delegate request user decryption of the handles its delegator
is allowed on. EVM keys the grant by `(delegator, delegate, contractAddress)` and ends it at
`expirationDate > block.timestamp`.

Decision:

The host keys the record by `(delegator, delegate, program, scope)`, the application of the
encrypted stores it reaches (DD-047). The record lives at
`PDA("user-decryption-delegation", delegator, delegate, program, scope)`. The wildcard row carries
`0xff×32` as both `program` and `scope` and covers every application. A grant that sets the
sentinel in only one position is refused.

`delegate_for_user_decryption` takes the scope as an account and requires `program` to own it
(`DelegationScopeNotProgramAccount`), the rule `create_encrypted_store` applies to a store's scope.
A grant therefore names an application a store can have. The wildcard row skips the check, since no
account lives at the sentinel.

`expires_at` is a Unix second, exclusive, compared against the Clock sysvar. A revocation writes 0.
`delegation_counter` and `last_update_slot` keep EVM's `delegationCounter` and
`lastBlockDelegateOrRevoke`: a record changes at most once per slot. The KMS connector reads the
application row, the wildcard row and the Clock in one snapshot (INVARIANTS #27).

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Key by `(delegator, delegate, authority)` and expire at a slot, the earlier layout | The authority is one store's controlling PDA. For the confidential token that is one token account, so a delegate needed a grant per account and a new grant for every new account. EVM grants per contract. The application is also what HCU metering, the deny list and permit scopes key on, so a delegation keyed on it matches every other policy. A slot expiry drifts from wall-clock time by an amount the delegator cannot predict; EVM's expiry is a timestamp. |
| Accept the scope as free bytes | A grant could name a scope no store of `program` can carry, which authorizes nothing and looks like a real grant. Checking the owner costs one account and one comparison. |

Consequences:

- One grant covers every value of the application that allows the delegator: for the confidential
  token, one mint's balances, transfers and burns, historical handles included.
- Renewing a grant needs the scope account to exist. If the program closes it, existing grants keep
  authorizing until they expire or are revoked, and revocation does not read the scope.

## DD-062: The listener reads one transaction per message

Status: adopted

Recorded in fhevm-internal#2104 (Solana access control RFC review, finding 1, fixes A and B).

The listener used to subscribe to whole blocks with `account_include: [host]`. Yellowstone keeps every
transaction that lists the host program, including one that never calls it, and sends the block as
one message. Solana bounds a block by compute units, not by bytes: about 100 transactions of about
640 KB of CPI data each (estimate) make a block message above the listener's 64 MiB decoding limit.
Anyone can send them. Every coprocessor then refuses the same slot, retries it every 2 seconds, and
stops ingesting. `getBlock` with full details returns the same block unfiltered, so archive catch-up
(DD-059) had the same limit.

Decision:

The listener subscribes at `finalized` (DD-070) to `transactions { account_include: [host], vote: false,
failed: false }` and to `blocks_meta`. One message holds one transaction, which Solana bounds at
under 1 MB: 64 instructions in the trace, 10 KiB of data per CPI and about 10 KB of logs. The
listener already ignored failed transactions, so leaving them out changes no output.
Reconstruction reads only the host program's instructions, so each transaction is reduced to them
when it arrives (`prepare_transaction`). `BlockValidator` holds the open slot's prepared
transactions and seals the slot on its block meta. The ancestry check, the ingest path and the
checkpoint are unchanged, and a reconnect starts a new validator that `from_slot` replay refills.

Sealing on block meta requires the provider to send every transaction of a slot before its block
meta. Yellowstone does, live and on `from_slot` replay (`yellowstone-grpc-geyser/src/grpc.rs` at
`bfd1d7e`, the pinned `v16.0.0`: lines 1399-1433 live, 1440-1493 on replay). A slot's transactions and its block meta are
two separate broadcasts, and a client receives live broadcasts from before its filter and replay
are set. Two cases follow:

- The live messages buffered during a replay follow it, so recent slots can arrive again, whole or
  as their block meta alone: every slot broadcast between the subscription and the replay, a
  number nothing bounds. The validator keeps the last `REDELIVERY_WINDOW` (32) sealed slots and
  skips a slot that arrives again when its block meta equals the applied one and every transaction
  is one it already held. A re-delivery of an older slot it sealed is skipped unchecked: the next
  new slot must still extend the newest sealed one, so a fork cannot pass, but a late transaction
  for such a slot goes unnoticed.
- A start at the tip can receive its first slot's block meta without the transactions before it.
  The listener skips the first slot and applies from the next.

A transaction of another slot while one is open, a block meta for another slot, an earlier slot the
validator never sealed, or a slot that does not extend the last applied one stops the listener
without applying the slot. A transaction that a recent slot did not hold stops it too, but that slot is already
recorded without it, and the restart resumes from the checkpoint, past that slot. If the
transaction wrote a Store, the next write to that Store does not continue its recorded leaf count,
and the Merkle indexer, which follows the same stream, stops there until its record is rebuilt
(DD-066). Moving its checkpoint back does not repair the record. The listener continues
without the transaction's computation rows. The error, which names the slot and the
transaction, and the restart alarm are then the only trace, and the replay repair in the
host-listener README restores the rows. Whether other providers keep this order is
fhevm-internal#2087.

Archive catch-up applies the same bound to each transaction. `getBlock` with `transactionDetails:
"accounts"` lists each transaction's signatures, account keys and error without instruction data or
logs, and `getTransaction` fetches each successful transaction that names the host. The listing
still grows with the block's transaction count and account keys. A block filled with transactions
that load many lookup-table keys could list hundreds of megabytes (estimate, not measured), which a
provider can refuse or not return within the archive client's 30-second timeout; catch-up then
retries that block. A provider that does return it can exhaust the pod's memory: the RPC client
holds the whole response and its parsed JSON, and catch-up fetches up to 8 blocks at once. The pod
then restarts at the same slot for as long as catch-up needs that block. Listing blocks with
`transactionDetails: "signatures"` and selecting the host's transactions with
`getSignaturesForAddress` would cost about 90 bytes per transaction, but it relies on the archive's
address index being complete, which nothing checks (DD-059).

On a cluster with several validators a slot can have more than one bank: Alpenglow can replace a
slot's bank and signals it with `EntryUpdateParent`. Yellowstone v16 buffers each bank by `bank_id`
and drops a replaced or losing bank (`yellowstone-grpc-geyser/src/block_reconstruction_v2.rs` at
`bfd1d7e`). At `finalized` (DD-070) it sends one frozen bank per slot: the bank a Finalized or
Confirmed status names, or, when Finalized reaches the slot only from a descendant, the slot's one
remaining bank. The slot's transactions, its `Block` and `BlockMeta`, and its status all come from
that bank.

- Safety: the follower applies a block only when it names the last applied block by parent slot
  and parent block hash (`a_slot_that_does_not_extend_the_last_halts`), so a block built on a
  replaced bank is never applied. That a block's transactions and its block meta come from the
  same bank is an assumption on the provider (INVARIANTS #69): the follower does not compare their
  `bank_id`.
- Liveness: a block that does not extend the last applied one is a fail-closed error. The host
  listener or the Merkle indexer exits, and the pod restart replays from the checkpoint. Yellowstone
  v16 emits a slot's Finalized before the ancestors that only inherit it, so a descendant can arrive
  first; the replay after the restart delivers both in slot order. A slot left between two banks
  that no status names is never delivered at `finalized`, so the follower exits at its child on
  every restart while that slot is inside the provider's replay window. Nothing repairs that case
  yet (fhevm-internal#2105).

The single local validator never replaces a bank, so no local test exercises this.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Raise the decoding limit | A block of junk carries about 550 MB at 60M compute units (estimate), so any lower limit stays attackable and the ceiling moves with Solana's block limits. Tonic reserves the whole message and decodes a copy, so a 512 MiB limit lets one block use more than 1 GB of the pod's 2 GiB. A hosted provider can cap message size in front of Yellowstone anyway. |

Consequences:

The decoding limit stays at 64 MiB, which no valid transaction reaches
(`a_transaction_message_stays_far_below_the_decoding_limit`). A transaction that only lists the
host still arrives, one message of up to about 650 KB, and the listener keeps only its index and
signature. On catch-up each one costs a `getTransaction` call, so junk that lists the host
multiplies catch-up's RPC calls. The stream reconnects after 30 seconds without a block meta,
which is sent for every slot. Yellowstone's pings, sent whatever its feed does, do not count.

## DD-063: A leaf proof reads its path by position

Status: adopted

Recorded in fhevm-internal#2104 (Solana access control RFC review, findings 2 and 4, fix C).

The proof route loaded every leaf of the Store, recomputed its peaks and rebuilt the path from all
of them, for each requested entry. The review measured 30 to 40 seconds for 8 entries of a
1,000,000-leaf Store, where the KMS connector waits 10 seconds, so a large Store could not be
decrypted.

Decision:

The Merkle indexer records every MMR node of height 1 and above in its `nodes` table (DD-066), in
the transaction that appends the leaves completing it. `mmr_append` merges the new leaf node with
one peak per trailing one bit of the leaf index, and each merge is a node; the indexer records those
merges as it appends. zama-host's `mmr_append` is unchanged. Mountains are aligned to their size, so
node `(height, index)` covers leaves `[index << height, (index + 1) << height)` whatever the leaf
count. At 6 leaves, leaf 4's path is `[leaf 5]`; at 8 leaves it is
`[leaf 5, node (1, 3), node (2, 0)]`, and those nodes never change.

A proof takes three indexed reads: the Store's row, the first leaf below its `leaf_count` that
matches the query, and the path. The path's height-0 sibling is the neighboring leaf row, so the
node table does not store leaf nodes again. The route checks the leaf's commitment against its
row and the path with `mmr_verify` against the Store's recorded peaks before serving it; a leaf
with a missing or wrong row is answered `inconsistent` (DD-068). Leaves and nodes are never
rewritten and the Store's row only grows, so the three reads agree without a transaction.

The first matching leaf is served. It sits in the oldest mountain, whose path changes
least as the Store grows.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Store leaf nodes in the node table too | One uniform path query, for a second copy of the hash of a row already stored: about 2N node rows instead of N. |
| Cache proofs or peaks in memory | A cache has to be invalidated on every append, and a restarted server rebuilds it from every leaf. |

Consequences:

8 proofs of a 1,000,000-leaf Store answer in about 11 ms in a debug build, request included
(`eight_proofs_of_a_million_leaf_store_answer_well_within_the_connector_timeout`, run on request). A
path has at most 64 entries. The leaf record grows by about one node row per leaf.

## DD-064: Leaf proofs are served apart from ingestion

Status: adopted

Recorded in fhevm-internal#2104 (Solana access control RFC review, findings 2 and 3, fix D).

The proof route ran inside `solana_host_listener`, on the 8-connection pool ingestion writes
through, and stopped whenever ingestion stopped: a fatal ingestion error or a restart took the route
down with it. A proof from a record that is behind still verifies against the peaks the KMS
connector reads on chain, as long as no later append to the Store has merged that leaf's mountain.
An ingestion stop therefore also stopped decryptions the record could still serve.

Decision:

`solana_merkle_proof_server` serves `POST /v1/solana/merkle-proofs` and the health routes as its own
Deployment and ClusterIP Service, `<release>-solana-merkle-proof-server`, from the host-listener
image and with its own pool (`--database-pool-size`, 8 by default). It answers only requests signed
by the tx-sender of a live KMS context (DD-067). It only reads the Merkle proof service's database
(DD-066), so it can run several replicas and roll without downtime. The Merkle indexer, which writes
that database, and the listener, which serves only `/healthz` and `/liveness`, roll like the EVM
listeners: one replica by default, `RollingUpdate` with `maxSurge: 1` and `maxUnavailable: 0`, and
more on request. Every replica applies every finalized block. The indexer applies one block at a
time under a transaction-scoped advisory lock, reads its checkpoint under it and skips a block at
or below it: its leaf and cursor writes are not idempotent, and before the first block is recorded
there is no checkpoint row to lock. The listener's row writes change nothing when a block is
applied again, a replay holds back no step (consensus may have healed one since), and its
checkpoint only moves forward. More than one listener replica costs latency, not correctness: a
replica that did not write a block learns which dependence chains that block gated only from its
periodic reload (`SEALED_CHAIN_REFRESH_INTERVAL_SECS` in `host-listener`), and until then it can
schedule later work behind them, as on EVM. The connector's `solana_proof_servers` entries name the
proof server's Service and its coprocessor's signer address.

On EVM the connector reads the ACL from the host chain, and no coprocessor serves proofs. The split
follows the coprocessor's one Deployment per role: `host_listener`, `host_listener_poller` and
`host_listener_consumer` already run from one image.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Keep the route in the listener with a second pool | Slow proof reads could no longer starve ingestion, but the route would still stop with every ingestion stop and restart. |
| A separate image | The server is one small binary, shipped with the Merkle indexer in the host-listener image; a second image adds a CI build and a tag to keep in step. |

Consequences:

The Merkle database has the server as a second client, with 8 connections by default. Proofs keep
being served while the Merkle indexer is down, for the leaves it recorded before it stopped. Such a
proof verifies until a later append to its Store merges its mountain, and the connector's check
fails from then until the indexer catches up. The newest leaf sits in the smallest mountain, so it
goes stale first, and anyone can append to a Store with a zero-value transfer. The connector asks
the next coprocessor as soon as an answer leaves a proof unverified (DD-067), so this costs one
more request while one indexer runs; while every coprocessor's indexer is stopped, a Store someone
keeps appending to cannot be decrypted until one catches up. The connector retries a Gateway
request up to its `max_decryption_attempts` before marking it failed, and an HTTP caller gets
`acl_denied` and resubmits. A grant made after the stop has no proof until the indexer catches up.
`ci/preview-env/solana-host/test_charts.py` pins the two Deployments.

## DD-065: A public decryption is accepted on-chain by its certificate alone

Status: adopted

Recorded in zama-ai/fhevm#4201.

`verify_public_decrypt` used to take the handle's Store and an MMR inclusion proof of its public
leaf, and checked the proof against the Store's current peaks before it accepted the KMS
certificate. The KMS connectors already check that public leaf before they decrypt (DD-048), so a
certificate exists only for a handle sealed public, unless the KMS committee is dishonest at its
threshold (INVARIANTS #23). The on-chain check repeated a check the certificate already depends on,
and it made every consumer fetch a proof from a coprocessor's leaf record. That proof went stale as
soon as a later append merged its mountain. The newest leaf sits in the smallest mountain, so one
append makes its proof stale half the time, and a busy Store could reject a settle more than once.

Decision:

`verify_public_decrypt` checks only the KMS certificate over `(handle, cleartext)`, against the
context the certificate names (DD-040), as EVM `FHE.checkSignatures` does. It reads no Store and
takes no proof, and no instruction takes an MMR proof. A consumer binds the certificate to its own
state by comparing the certified handle with one it pinned in an account it owns.
`redeem_burned_amount` compares it with `PendingBurn.burned_handle`, and the batcher's `settle`
passes its pinned `burned_total_handle` to that check. `disclose_secp` emits
`HandleDisclosedEvent { handle, cleartext_amount }`, as ERC-7984 `discloseEncryptedAmount` emits
`AmountDisclosed`, and reads no token state. A reader links the handle to an account through the
token's own handle events.

Merkle proofs travel only from a coprocessor's leaf record to the KMS connector. The SDK, the demo
dapp and the test harness fetch, build and check none.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Keep the on-chain proof check | It repeats the connectors' check, adds `12 + 32·depth` bytes to every consume transaction, and makes the consumer fetch a proof that one append can make stale. |
| Keep a ring of recently removed peaks in the Store, so a slightly stale proof still verifies | A ring that gives real margin under load needs dozens of peaks, paid in rent by every Store. |
| Write a `PublicHandle` record at make-public and have disclose read it | One more account per public handle, to say what the certificate and the handle history already say. |
| Return the proof with the certificate and submit it on-chain | It moves the fetch into the connector response but keeps the staleness race and the transaction size. |

Consequences:

On-chain, publicness rests on the connectors' leaf check and on the KMS committee's threshold
honesty (INVARIANTS #21, #23), as on EVM. A program that wants on-chain proof that a handle it did
not pin is public would need a separate proof-taking entry point; none is wanted today. Consume
transactions shrink: `redeem_burned_amount` and `disclose_secp` each fit one version 1 transaction
(4,096 bytes) at the host's maximum of 16 KMS signatures (DD-041). Removing
`PublicDecryptProofInvalid` renumbered the later Anchor error codes of zama-host and
confidential-token.

## DD-066: The leaf record has its own indexer and database

Status: adopted

Recorded in zama-ai/fhevm#4215.

The host listener wrote the leaf record into the coprocessor database, in the transaction that
wrote the compute rows (DD-048). The record could only start where the listener started. A
listener started at the tip, or on a database restored without the record, met Stores whose
earlier leaves it never saw. It marked them `history_complete = false` and answered
`historyIncomplete` for them until someone replayed it from before their creation. A fault in the
record, such as a leaf count that skips, also stopped compute ingestion, and the only repair was
editing the coprocessor database by hand.

Decision:

The leaf record belongs to the `solana-merkle-proof-service` crate, in its own Postgres database
with its own migrations: `encrypted_stores`, `leaves`, `nodes` and `checkpoint`. The crate has two
binaries:

- `solana_merkle_indexer` follows the finalized stream through `solana-host-follower`, the crate
  the host listener follows it with. It writes each block's leaves, nodes, Store rows and
  checkpoint in one transaction. It resumes from its checkpoint. On an empty database it replays
  from `--start-slot`, a finalized block before the first Store was created, such as the zama-host
  deployment slot, whose hash it reads from the archive endpoint. It never starts at the tip. It
  stops on a Store first seen above leaf zero (`UnrecordedHistory`) and on a leaf count that
  skips.
- Re-handing the checkpoint block with the same hash writes nothing. A different hash at that
  slot or a block below it stops the indexer. The finalized follower applies one block at a time
  and filters older redeliveries. Moving the checkpoint back does not rewind the Store cursors:
  re-applying recorded leaves fails on their previous leaf count. A wrong record is rebuilt from
  the start slot into an empty database or restored from another record's dump.
- `solana_merkle_proof_server` serves the proofs from that database over
  `POST /v1/solana/merkle-proofs` (DD-063, DD-064).

A record holds every Store from leaf zero or has not seen it, so a proof answer is `found`,
`notFound`, `unknownAccount`, or `inconsistent` for a leaf the record is known to hold wrong
(DD-068); there is no incomplete history. The host listener writes only
compute rows and its own checkpoint.

A lost or broken record is rebuilt in one of two ways. A `pg_dump` of a healthy record restored
into an empty database resumes from the checkpoint inside it and catches up
(`a_restored_dump_resumes_and_catches_up`). An empty database replays from the start slot.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Keep the record in the listener and mark incomplete histories | A Store the record missed has no proofs from that coprocessor until a manual replay, and a record fault stops compute ingestion. |
| Start an empty record at the tip and backfill the missed history | The backfill is a second ingestion path, with its own ordering and checkpoint, racing the live one. |
| A custom export and import of the record | `pg_dump` and `pg_restore` already move a consistent snapshot with its checkpoint, and operators know them. |
| A separate image | As in DD-064: one more CI build and tag to keep in step. Both binaries ship in the host-listener image. |
| One follower feeding both the compute rows and the record | The two start differently: the listener may start at the tip, the record only before the first Store. One process would also couple their failures again, so a record fault would stop compute ingestion. |

Consequences:

Each coprocessor runs one more Deployment and one more database on its Postgres server, and opens a
second Yellowstone subscription. The record and the compute rows commit separately, so they
can disagree about which blocks were applied. Nothing reads both: the connector checks each proof
against the peaks it reads on chain. A deployment must know a start slot before its first Store:
the chart requires `solanaHostListener.merkleIndexer.startSlot` and the Merkle database's URL.
The preview wipe (DD-051) closes Stores, and a Store created again at the same address starts
again at leaf zero, which the record reads as a skipped count. The preview rollout therefore
recreates the Merkle database and takes a new start slot on every run. The coprocessor database
has no leaf tables.

## DD-067: A Merkle proof request is signed by a KMS context's tx-sender

Status: adopted

Recorded in zama-ai/fhevm#4219.

The Merkle proof server took one bearer key, shared by every KMS connector that asked it. Each
coprocessor had to hand that key to every KMS party and rotate it with them, and the server could
not tell one connector from another, so it could not limit or attribute what a connector asked.

Decision:

kms-worker signs each request with its party's tx-sender wallet, the key its tx-sender submits
Gateway transactions with: a local private key or an AWS KMS key, never a session key. The
signature is `FhevmSig` (RFC 038, the scheme RFC 033 names for its own hops): EIP-712 over
`Request(string path, bytes32 bodyDigest, uint64 expires, address audience)` in the domain
`{name: "fhevm-http-auth", version: "1", chainId, verifyingContract}`, which names the canonical
`ProtocolConfig` and its chain, so two networks on one chain do not accept each other's requests.
`audience` is the registered signer address of the coprocessor the request is sent to. Each
`solana_proof_servers` entry of the connector names its server's address, and the server is started
with its own (`--coprocessor-signer-address`), until `ProtocolConfig` lists the coprocessors. The
signature travels as `Authorization: FhevmSig expires=<unix seconds>, sig=0x<65 bytes>`. The
`shared/request-authorization` crate builds and checks it for both sides, and pins its wire bytes
against viem. A server refuses a signature with `expires < now - CLOCK_SKEW` (30 s) as
`auth_expired` (401, retryable), and one with `expires > now + MAX_AUTH_VALIDITY + CLOCK_SKEW`
(300 + 30 s) as too long-lived. It refuses a request with a query string as `malformed` (400),
since the signature does not cover one. kms-worker signs for 30 seconds, once per coprocessor and
in parallel before it asks the first, and waits at most `host_rpc_call_timeout` for the signatures.
It asks the coprocessors one after another in a random order: the next one as soon as an answer
leaves a query without a verified proof, or after `HEDGE_DELAY` (250 ms) without an answer.

`solana_merkle_proof_server` recovers the signer and answers only the tx-senders of the live KMS
contexts. It reads them from the canonical `ProtocolConfig` at the finalized block every 60 seconds:
the live context ids, then each context's nodes from its `NewKmsContext` event at the context's
anchor block, checked against the anchor's `contextInfoHash`. kms-worker reads a previous context
the same way, through `shared/kms-context`. A refresh that fails, or takes more than 30 seconds,
keeps the last set. Until the first read succeeds, every request gets `upstream_transient` (502,
retryable) and `/healthz` answers 503. A missing, malformed, too long-lived or unknown signature,
or one for another audience, gets `sender_authentication_failed` (401) before the database is read.

Request and success bodies are CBOR (RFC 8949) over HTTP/2 without TLS negotiation (prior
knowledge). Hashes and keys travel as 32-byte byte strings, and the requests kms-worker sends one
coprocessor share one connection. `solana/test-fixtures/merkle-proofs/merkle_proofs_v1.json` pins
the request and response bytes for both sides. Error bodies are JSON `{code, message, retryable}`,
the RFC 033 shape, and the OpenAPI document `openapi/solana_merkle_proofs.json` describes the route.

The audience stops a coprocessor from replaying a batch to another: that server rebuilds the
request with its own address and recovers another signer. Copies of one signed request can still
reach the server it was signed for until the server stops accepting it, at `expires + CLOCK_SKEW`:
resent by anyone who reads the plain-HTTP traffic inside the cluster, or by the connector itself.
The answers are public proofs, so a copy learns nothing, and it must not cost anything either. Each
server remembers every signed request it admitted until then (`AnswerCache`, keyed by the EIP-712
signing hash). A request whose body does not decode is refused before the cache. A request is
admitted once its signer's rate accepts it, charged under the cache's lock, so two copies arriving
together are charged once; an over-rate request is refused and never remembered. The first copy to
arrive is answered once, in a task of its own, so a caller that disconnects does not cancel it.
Every other copy, sent at the same time or later, waits for that answer and gets the same bytes,
including a refusal the answer ended in. A server therefore charges and reads at most once per
signed request. The exception is `overloaded`: that first copy found no free database connection
and read nothing, so the server forgets it, and a copy sent after its `Retry-After` is admitted and
charged again. A copy that arrives after its request was forgotten is refused as `auth_expired`
rather than charged again. The signing hash names no signer, so KMS nodes that sign the same body
for the same coprocessor in the same second share one answer. A worker retry signed within the same
second as the batch it retries has the same signing hash, and gets the earlier answer; the worker
loop retries it again later. The relayer does not call the Merkle proof server.

Each KMS tx-sender may ask one server for `--kms-tx-sender-leaves-per-second` (4000) queried
leaves per second, in bursts of as many and at least `MAX_LEAVES_PER_REQUEST` (64). The rate is a
backstop against a faulty or compromised connector, not sized from load data; a sustained load
meets the cache budget below first. A server remembers at most
`--answer-cache-mib-per-kms-tx-sender` (16) MiB of each tx-sender's requests with their answers,
counting `ENTRY_OVERHEAD_BYTES` (768) per request beside its answer. A new request past that is
refused rather than admitted unremembered, since a copy of it would then cost work again. The
budget is per tx-sender, so a faulty or compromised connector refuses only its own requests. These
two refusals are `rate_limited` (429, retryable). All but one connection of the pool
(`--database-pool-size`, 8, at least 2) serve proof reads, one request each at a time, so
`/healthz` always has one. A request waits up to `PROOF_READ_WAIT` (200 ms) for its turn, and is
otherwise refused as `overloaded` (503, retryable). That wait is below the connector's
`HEDGE_DELAY`, so a refusal sends the connector to the next coprocessor no later than its hedge
would have. Both 429 and 503 carry `Retry-After: 1`, and the connector treats every refusal as a
failed read and asks the next coprocessor at once.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| A bearer key per coprocessor | Every KMS party holds every coprocessor's key, and the server still cannot tell connectors apart. |
| A key per connector, listed in each coprocessor's config | Each coprocessor edits its config when a KMS context changes; `ProtocolConfig` already lists the tx-senders. |
| mTLS | Each coprocessor runs a certificate authority for the KMS parties, and the identity is not the on-chain one. |
| Every coprocessor asked at once | Each coprocessor serves every proof read, so the load grows with the number of coprocessors, for an answer one coprocessor usually gives alone. |
| One signature per batch, valid at every coprocessor | Any coprocessor asked could replay the batch to the others, and RFC 038 signs the audience for that reason. |
| Refuse a signature a server has already seen | A copy that reaches the server before the connector's own, resent by a traffic reader, would have the connector's read refused; answering every copy from one read costs the same. |
| JSON success bodies | Hex doubles every hash: a full answer of 64 proofs with 64 siblings is about 280 KB instead of 150 KB, and both sides parse hex by hand. |
| Protobuf or gRPC | A schema compiler and generated code in two workspaces, for four message types. |
| A session key kms-worker registers with its tx-sender key | A second key to rotate and a registration round, for signatures that AWS KMS already serves. |

Consequences:

kms-worker needs signing access to the tx-sender key: the same `KMS_CONNECTOR_PRIVATE_KEY` or
`KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID`, which the chart passes to kms-worker only when a Solana host
chain is configured. With AWS KMS, kms-worker's own service account needs `kms:GetPublicKey` and
`kms:Sign` on that key, or kms-worker exits at startup. A compromised kms-worker can then sign as
the party's tx-sender, which submits its Gateway transactions. Each batch costs one AWS KMS
signature per coprocessor, from the quota the tx-sender also uses. kms-worker waits for every
signature before it asks the first coprocessor, so one failed or late signing call fails the batch,
and the worker loop retries it later. A coprocessor's signer address is
configured twice, in its proof server and in every connector's `solana_proof_servers` entry for it;
a wrong address makes every read of that server fail with 401, and the connector asks the next. A
new KMS context is answered once its creation is finalized, up to 60 seconds later. The proof
server reads the canonical `ProtocolConfig` over the RPC of the coprocessor's `chains[]` entry
named by `commonConfig.canonicalProtocolConfigChainId`, and the chart refuses to render without it
or without `solanaHostListener.proofServer.coprocessorSignerAddress`. The preview mints no proof
secret. A 64-leaf answer with 20-hash paths (a store of a million leaves) holds under 48 KiB, so a
tx-sender's 16 MiB holds about 22,000 leaves. A server remembers each request for the connector's
30-second validity plus the 30-second skew, so that is 365 leaves per second, and about 225 in
1-leaf requests. This budget, not the rate, bounds a connector's sustained reads from one server.
kms-worker shuffles the coprocessors for each batch, so one server sees about its share of the
batches that the first coprocessor asked resolves; a batch with a query left unresolved, such as
one whose leaf is not recorded yet, reaches every coprocessor. A tx-sender past its budget is
refused until older requests expire, and the connector asks another coprocessor. 13 KMS nodes in
two live contexts hold at most 416 MiB, inside the chart's 512 MiB limit. The cache and the rate
are per replica, so a copy that reaches another replica of the same server is charged there again.

## DD-068: The Merkle indexer checks its record against the chain and quarantines a store that disagrees

Status: adopted

Recorded in zama-ai/fhevm#4221.

Checking each new write's previous leaf count (DD-066) did not compare the recorded peaks with
the chain. A record restored from a diverged dump, edited by hand, damaged on disk or built by
an indexer bug served proofs that the KMS connector rejected one by one. Nobody was told,
and the connector paid a failed read per request until someone looked.

Decision:

`solana_merkle_indexer` checks every recorded store once at start and then every
`--store-check-interval-secs` (600). It reads the stores 100 at a time with `getMultipleAccounts` at
finalized commitment, judges each account as the KMS connector does (`validate_store`, from
`zama_solana_acl`), and compares at the chain's leaf count `n`:

- no account: `absent`, the store was closed, and its quarantine lifts;
- an account that is not a valid store: `mismatch`;
- a record holding fewer than `n` leaves, read after the account: `behind`, compared once more
  at the end of the check;
- otherwise the record's peaks at `n`, read from its `nodes` rows, must equal the account's peaks:
  `match` or `mismatch`.

Leaves are only appended, so the peaks of the first `n` leaves never change once recorded, and the
comparison holds whichever slot the chain was read at. A `mismatch` writes the store into
`quarantined_stores`, and a later `match` removes it. `solana_merkle_proof_server` answers every
leaf of a quarantined store `inconsistent`, in a 200 that still proves the request's other
leaves, and the connector takes that leaf's proof from another coprocessor. Any other
coprocessor's answer about the leaf decides the entry, whichever arrives first. A connector that
gets `inconsistent` from every coprocessor fails the request `upstream_transient`, retryable: the
user's access is unknown, not denied. Only a failed database read refuses the whole request
(`upstream_transient`, 502).

The check compares the peaks only. A wrong row below correct peaks is caught when it is served:
before answering, the proof server recomputes the leaf's commitment from the row's store, index,
handle and key, and verifies its path against the recorded peaks (DD-063). A row that fails
either is answered `inconsistent`.

`solana_merkle_indexer_quarantined_stores` above 0, any `inconsistent` leaf the proof server
counts, and any `invalid` answer a KMS connector counts by coprocessor
(`kms_connector_worker_solana_proof_answers_counter`), page the protocol on-call. The connector
counts a proof `invalid` when it was built against as many leaves as the chain holds or more: a
correct proof from a longer record is cut to the chain's count and verifies, so only a proof from
a shorter record can fail by being stale. The connector's count names the coprocessor, so a
partner's wrong record is seen from the connectors too. `RUNBOOK.md` in the crate describes the
repair: replace the database with a `pg_dump` from before the divergence, or rebuild it from the
chain, and wait for a clean check after the indexer catches up. `/healthz` does not look at the
record: a record behind the chain is a lag alarm, and a record that disagrees is a quarantine.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Recompute every store's peaks from all its leaves | Reads every leaf of every store each interval; the peaks at `n` are already rows. |
| Repair a mismatching store in place | The record has no partial repair: a store's later leaves depend on its earlier ones. A whole-database restore is the one path that is tested (`a_restored_dump_resumes_and_catches_up`). |
| Stop the indexer on a mismatch | Every other store keeps serving from a stopped indexer's record, but it goes stale; the quarantine stops only the wrong store. |
| Fail `/healthz` on a mismatch or on lag | Kubernetes would restart a server whose other stores serve correctly, and a restart fixes neither. |

Consequences:

A quarantined store has no proofs from that coprocessor until a check after the restored or rebuilt
record catches up matches. One `getMultipleAccounts` per 100 stores per interval reaches the
indexer's RPC provider. A store closed and recreated at the same address is reported `mismatch`
once the new store holds a leaf, until the record is rebuilt, as DD-066 already requires. An RPC
node that answers no account for a store it has not seen yet lifts that store's quarantine until
the next check; the connector still verifies every proof against its own read of the chain.

## DD-069: Only an output its transaction stores is computed and recorded as a block producer

Status: adopted

Context:

Every coprocessor publishes, per host block, a signed manifest of the handles the block produced and
their ciphertext digests, and compares it with its peers' manifests (the consensus detector). On
EVM a handle is a block's product only when its producing transaction persists it with an
`Allowed` or `AllowedForDecryption` event. The host listener marks that output `is_allowed` and
writes its `handle_producer_block` row. Any other output is a transient value: its computation row
is stored `is_allowed = false, is_completed = true`, and the tfhe-worker computes it only for an
allowed consumer in the same transaction.

On Solana the TransientStore lives for one transaction. A handle outlives the transaction only when
the transaction writes it into an EncryptedStore: a slot write, an allow leaf or a public leaf of an
`fhe_execute` effect. An effect names a result of its own execution. `make_store_handle_public`
writes a handle that an earlier transaction produced.

Decision:

`normalize_solana_records_for_db` allows a step output when the same transaction requests the
output's material. The listener requests it for each store write (`HostOperation::store_writes`).
An allowed output gets `is_allowed = true` and a `handle_producer_block` row. Every other output is
inserted as EVM inserts a transient value.

The set is per transaction, not per execution. A trivial-encrypt handle depends on its plaintext
alone, so two executions of one transaction can produce the same handle, and `computations` keeps
only the first row of a handle in a transaction. That row must be allowed when a later execution
stores the handle.

Rationale:

Every producer handle has a material request, so it gets a ct128 and a digest: its block's manifest
seals as soon as the handles are computed, and healing has bytes to fetch from peers. An
intermediate has no material request, so a manifest listing it could seal only as `uncomputed`,
after the timeout. Using EVM's transient model keeps one tfhe-worker scheduling path for both
host types.

Rejected alternative: compute every output and add a separate producer list to `LogTfhe`. It
computes values no one can read, and EVM ingest would carry a field it never sets.

Consequences:

A held-back step whose output is not stored is a terminal error row that is not allowed. The
tfhe-worker drains its consumers as it does for any dead producer
(`errored_local_producer_drains_consumer_and_is_not_reexecuted`). Scheduling stays decoupled from
authorization: the KMS validates the live EncryptedStore and any leaf proof before it releases
plaintext (INVARIANTS #31).

Pinned by `only_an_output_its_transaction_stores_is_allowed`,
`a_handle_produced_twice_is_allowed_when_a_later_execution_stores_it`,
`storing_an_older_handle_allows_no_output` and
`solana_records_reach_the_shared_sql_and_scheduler_path`.

## DD-070: Solana is read at finalized commitment

Status: adopted

Context:

Every off-chain component reads the Solana host chain. The host listener and the Merkle indexer
follow its blocks, the KMS connector reads Stores and delegation rows before it releases plaintext,
the relayer pre-checks delegations, and clients (the SDK, the demo dapp, `solana/deploy`, the test
suite) send transactions and read accounts. A reader's commitment level is the point at which it
treats a block as settled: at `confirmed` a supermajority has voted for the block, at `finalized`
it can no longer be rolled back.

What `finalized` costs depends on the consensus. Under TowerBFT, which mainnet runs today, a block
finalizes 32 slots, about 12.8 s, after it is confirmed. Under Alpenglow (SIMD-0326) a block is
final once its votes certify it, and `confirmed` and `finalized` name the same slot. Devnet runs
Alpenglow (feature `A1pengvuM6JEcyNuTnMqepBKhwHE3N6PmUrdATGawhJS`, active since epoch 1167), where a
block was final a median of 408 ms, and at the 90th percentile 585 ms, after it completed. The
local validator runs Agave 4.3.0 with `--alpenglow` (TESTING.md), where a block is final as it
completes; the same validator on TowerBFT finalized 14.9 s later.

Decision:

Every Solana read and every confirmation wait in the port uses `finalized` commitment. The port
assumes Alpenglow on every cluster it runs on. This covers:

- the Yellowstone subscription the host listener and the Merkle indexer share
  (`solana-host-follower`'s `build_subscribe_request`), their live RPC client, the archive
  checkpoint read (`block_checkpoint`) and the `solana_host_follower_finalized_slot` metric;
- the Merkle indexer's store check (DD-068);
- the KMS connector's account snapshot (`kms-worker/src/core/solana/snapshot.rs`);
- the relayer's delegation pre-check (`relayer/src/host/acl_checker.rs`);
- the SDK, the demo dapp, `solana/deploy`, the test suite and the preview scripts: account reads,
  blockhashes, simulation and confirmation waits.

Rationale:

One commitment makes every component describe the same chain. The listener and the Merkle indexer
apply only blocks that cannot roll back, so no computation runs on a minority fork and there is
nothing to unwind (INVARIANTS #32). The KMS connector releases plaintext only against a grant on the
finalized chain, where EVM host ACL reads take the node's latest block
(`kms-worker/src/core/event_processor/rpc.rs`).

The leaf record and the Store the connector reads are both at `finalized`, so they differ only by
the indexer's lag. Mixing commitments would cost retries, never a wrong answer: the connector
verifies each proof against the Store's live peaks. A proof built at an older leaf count still
verifies while the appends since then left the leaf's peak intact. A proof built ahead of the
observed count is cut down to it (`MmrProof::for_leaf_count`). Otherwise the request fails with
`ProofDoesNotVerify`, `ProofRecordBehind`, `LeafIndexOutOfRange` or `NoLeaf`, all retried.

Rejected alternative: keep `confirmed` and add reorg unwind to the listener and the Merkle indexer.
It needs a rollback path through computations and the leaf record, and still cannot take back a
share the KMS released on a rolled-back fork. On Alpenglow it would save about 400 ms.

Consequences:

A client that waits for less than `finalized`, such as a third-party wallet, can ask for a
decryption before its grant is finalized. Neither service turns that into a terminal refusal:

- the KMS connector records every outcome of such a read as recoverable. An absent Store, a missing
  leaf (`NoLeaf`), a record behind the chain (`ProofRecordBehind`), a leaf beyond the observed count
  (`LeafIndexOutOfRange`), a proof that no longer verifies (`ProofDoesNotVerify`) and a delegation
  that is not live are ACL denials that a later attempt may clear. Pinned by
  `every_solana_authorization_failure_is_recorded_as_written`,
  `every_solana_public_decrypt_failure_is_recorded_as_written` and
  `a_missing_delegation_rejects_its_entry`;
- the relayer pre-check reads a refused delegation again on its retry policy (`max_attempts`,
  `retry_interval_ms`) before the refusal stands. Pinned by
  `a_grant_that_finalizes_after_the_first_row_read_passes` and
  `rows_still_dead_on_the_last_attempt_refuse`. The example policy, three reads one second apart,
  covers Alpenglow's finality, not TowerBFT's 12.8 s. A refusal that stands costs at least
  `(max_attempts - 1) × retry_interval_ms` of waiting: two seconds with that policy.

On a cluster with several validators a slot can have more than one bank. How Yellowstone v16
delivers one bank per slot at `finalized`, and what the follower guarantees from it, is in DD-062.

Pinned by `request_subscribes_to_host_transactions_and_block_meta` (the subscription),
`finalized_rpc_preserves_order_null_accounts_and_context_slot` (the KMS snapshot) and
`row_read_requires_the_first_read_slot` (the relayer pre-check's row read).

## DD-071: The listener records each Solana block as a finalized host block, numbered by height

Status: adopted

Context:

Every coprocessor publishes a signed manifest of each host block's produced handles and compares it
with its peers' (the consensus detector). The detector finds a chain, seeds it and walks it only
through `host_chain_blocks_valid`. The shared `block-manifest` crate requires a manifest's range to
be numbered `n, n+1, …` with matching parent hashes, history windows are aligned power-of-two
ranges of numbers, and a manifest is due when `number mod cadence == 0`.

A Solana slot can be empty: a leader that produces no block leaves its slot skipped. The block
height counts the blocks of a fork, so a child is always its parent plus one. Yellowstone's
`BlockMeta` and `getBlock` both carry it.

Decision:

`apply_block` (`host-listener/src/solana_listener.rs`) writes the block's `host_chain_blocks_valid`
row through `mark_block_as_valid`, in the transaction that holds the block's computation rows and
the checkpoint. Every Solana row uses the block height as `block_number`: `computations`,
`pbs_computations`, `handle_producer_block` and the host row. The slot stays in the checkpoint, the
Merkle record and the logs. Example: slots 100, 102 and 103, with 101 skipped, are heights 5000,
5001 and 5002.

- A block without a height or a block time is fatal, and so is a parent row that is not at the
  height below. Only the chain's first row may have no parent row. These checks run before anything
  is written. `mark_block_as_valid` records an ingested finalized block without refusing a parent
  that disagrees, so the listener checks the parent first.
- The row is `finalized` when it is recorded, since the listener reads at `finalized` (DD-070) and
  never unwinds a block (INVARIANTS #32). Every reader of `host_chain_blocks_valid` sees Solana rows
  as it sees EVM rows: manifest discovery, pruning, the upgrade controller's `check_dry_run_ready`,
  and the consensus detector's state-hash writer and GCS watermark. A Solana chain's
  `consensus_epoch_block_window.start_block` is therefore a block height.
- Every 100 heights, after the commit, the listener prunes old finalized rows with
  `prune_finalized_block_history`, as EVM ingest does after a finalization pass.
- The detector's default cadence for a Solana chain id is 150 heights, about one minute at 400 ms
  slots with a few percent skipped. EVM chains publish about once a minute too.

Rejected alternative: number rows by slot. A skipped slot breaks the contiguity checks every
coprocessor runs on its peers' manifests (`shared/block-manifest/src/lib.rs`), the aligned history
windows, the n−1 parent check of `update_block_as_finalized`, and the `mod cadence` trigger, since
a skipped multiple never comes due. Fixing them is a wire change across the EVM network, and the
parent hashes already prove that no block is missing.

Consequences:

Apart from pruning, only the operator repair of DD-056 removes a recorded Solana block.
`revert_coprocessor_db_state.sql` reads the height of the checkpoint block from its row. It refuses
a Solana chain whose listener has no checkpoint, whose checkpoint names no recorded block, or whose
checkpoint is not the block at `to_block_number`, and names both heights.
`revert_coprocessor_db_state.sh` takes the rewind slot as `SOLANA_SLOT`, apart from the height in
`TO_BLOCK_NUMBER`. The gw-listener's automatic drift revert runs the same script, so on a Solana
chain it refuses until an operator rewinds the checkpoint.

A repair reaches back only as far as the host rows. `prune_finalized_block_history` deletes
finalized rows more than 10,000 heights below the tip and older than 7 days, so the rewind slot must
be newer than the 7-day retention. The revert refuses a checkpoint at a pruned block, and a replay
from one would stop at the parent check. A deeper replay is out of scope.

A revert deletes the host rows above `to_block_number` but not their `block_manifest_state` rows, as
on EVM. The replay records the same blocks at the same heights, so those rows close again, and a
sealed but unpublished one is resealed. A manifest already published for a block above
`to_block_number` keeps its pre-revert content. No chain prunes `block_manifest_state`: Solana adds
about 216,000 rows a day per coprocessor, against about 7,200 for Ethereum. Both are tracked in
fhevm-internal#2123.

Pinned by `rows_are_numbered_by_height_across_a_skipped_slot`,
`an_incomplete_block_or_a_wrong_parent_writes_nothing`,
`a_reverted_slot_replays_with_fresh_rows` and `publication_cadence_matches_known_chains`.

## Open product decisions

Not settled by the decisions above. Forward requirements are detailed in
[`FUTURE_DESIGN.md`](./FUTURE_DESIGN.md); this list is the short index.

- Whether confidential balances move to the staged inbound-credit profile (DD-016).
- Rent and archival policy for the Store MMR (DD-049): one stable PDA serves a Store for its whole
  life and its size is bounded at `121 + 64·slots + 32·peaks` bytes, so compaction is a rent question,
  not a liveness one. The off-chain leaf history the Merkle indexer keeps for proofs is not bounded
  (about 540 bytes per leaf per coprocessor with its MMR node, estimate, never pruned); fhevm-internal#2060 tracks row
  shrinking and per-Store cold archival.
- General `HostConfig` config-version rotation semantics beyond the KMS-context pointer.
- Full production KMS-connector wiring and real ZKPoK and transciphering behind the input attestation
  (both are shortcuts today, DD-028).
- Production Yellowstone/Geyser and archive providers, and their replay windows and rate limits
  (DD-003, DD-059, fhevm-internal#2087).
- Historical handle discovery conventions for apps.
- Production role and governance names for public-decrypt and grant authority.
- Leaf-record availability (DD-048): the connector asks the next coprocessor when one answers
  without a proof, fails, or takes longer than `HEDGE_DELAY` (250 ms), so one behind, stalled or
  unreachable cannot sink a request another can serve, or hold it longer than that delay. A record
  rebuilt by replay from the start slot catches up at the archive's speed. A faster heal restores a
  `pg_dump` of Zama's record, as `RUNBOOK.md` in the `solana-merkle-proof-service` crate describes
  (DD-068). The dump schedule, and a data-only export a partner could import without running
  another operator's SQL, are open.
- No Solana-native composition pattern for contract-to-contract confidential calls is designed.
  The receiver-callback flow is in DESIGN_HISTORY.md (DD-011).
- There is no per-Store cap on allows (Solana access control RFC): allows are leaves, and the app-side wall is the
  builder's heap budget (INVARIANTS #54). Whether a policy cap on allows per write is wanted for the
  leaf record is open.
- Mint-as-PDA authority consolidation (fhevm-internal#1862 Wave 3) is deferred as unnecessary. The
  mint stays a non-PDA keypair account, the total-supply write authority a mint-scoped PDA, and the
  vault-authority and per-owner token-account PDAs unchanged. Consolidation would touch every CPI and
  vault path without reducing attack surface enough to justify the churn.
- Compliance, freeze and `TokenInterface` (fhevm-internal#1862) are partially closed by DD-045. The
  product stance is no token-owned sanctions list; host deny stays on the allow path; compliance for
  the underlying exit is the SPL freeze authority on the wrapped mint. Wrap and redeem freeze-check
  the SPL accounts they move; confidential transfer and burn check each relevant owner's associated
  token account through `check_underlying_ata_not_frozen`, treating an uninitialized address as not
  frozen; `cancel_pending_burn` is not freeze-gated. What these checks do not reach is
  fhevm-internal#1981.

## DD-072: One source for everything that can change

Status: adopted

Context:

Anything that can change needs one source: PDA seeds and derivations, instruction building, account
and event decoders, types, structs and constants. Several are restated by hand today. The SDK spells
the zama-host seeds and derives stores, transient stores, delegations and permit watermarks itself.
The deployment, the demo dapp and the test suite derive event authorities and other programs' PDAs.
The KMS connector, the relayer, the host follower and the Merkle proof service rebuild store and
delegation addresses in Rust.

A restated copy agrees with the program only until one of them changes, and on Solana the mismatch
is silent. A wrong recipe still yields a valid address: the instruction built on it fails an account
constraint, or the read finds no account. Third-party dapps make it worse: they generate or vendor a
client once and do not regenerate it.

Decision:

Everything that can change has one source. Every consumer imports from it or generates from it.
Nothing is restated by hand: a handwritten client, or an adapter with its own PDA or
instruction-building code, is not acceptable. The norm is the program's IDL rendered by Codama, or a
shared Rust crate. A golden test catches an accidental change; it is not a second source.
Scope: code from feature/solana and what sits next to it, not unrelated EVM code.

- PDA recipes live in the programs, as Anchor `seeds = [...]` constraints. The bump is stored in the
  account wherever checking the address again would otherwise cost compute. The IDL then declares
  every PDA, and Codama generates the `find*Pda` helpers and the builders' account defaults from it.
  This settles fhevm-internal#2108 open question 5 for Anchor seeds and reverses its earlier
  proposal of Codama visitors in the codegen script, which would restate the seeds in JS.
- Off-chain Rust takes a recipe from the program crate, or from `zama-solana-acl` for the store,
  delegation and permit-invalidation recipes, through one seed-list function per recipe.
- Logic that cannot be generated, such as the SDK's cleartext client or the TypeScript chain-type
  checks, may be repeated only when one shared fixture in `solana/test-fixtures` is asserted from
  both Rust and TypeScript.

`solana/scripts/dead-surface-check.sh` check 8 enforces the PDA part. It fails on
`getProgramDerivedAddress`, `findProgramAddress` and `createProgramAddress` (including the `Sync`
forms) in TypeScript, on `find_program_address` and `create_program_address` in Rust, and on seed
literals, in production code outside generated clients, the programs, `zama-solana-acl` and
`solana/test-kit`. It reads the seed literals from those sources. Tests are not swept: a test that
restates a recipe is a pin and fails loudly when the recipe moves. Today's copies are listed in
`HAND_DERIVATIONS_ALLOWED`, each with its count of lines and its tracking task. The script checks
counts; review rejects new entries. Check 8 also keeps the PendingBurn seed to one raw literal
in the program, in `constants.rs`.

Compatibility:

Nothing on Solana is deployed, the dapps included. Until it is, Solana code keeps no backward
compatibility: a recipe, a layout or a discriminator changes in place, the old path is deleted, and
no fallback or compatibility branch is written for it. EVM paths keep their compatibility, because
they have consumers and a release policy. Once Solana is deployed, its default flips to no breaking
changes, forward and backward compatible.

Rationale:

The program is where a recipe is enforced, so it is the only place a copy cannot drift from. Anchor
`seeds` put the recipe in the IDL, where every generator can read it: the JS clients today, and Rust
or other clients later. A Codama visitor would keep a JS-only copy, and a test per PDA would be
needed to hold it to the program.

Consequences:

- A program that references another program's PDA restates its seeds only where a client default needs them; runtime tests pin that order.

- fhevm-internal#2108 task 2 adds the missing `seeds` to zama-host and confidential-token, then to
  the demo batcher and demo vault, regenerates the clients, deletes the TypeScript copies and
  shrinks the allow-list. A golden test pins every PDA address for fixed inputs, so a recipe change
  is a deliberate edit.
- fhevm-internal#2108 task 2 moves the off-chain Rust derivations behind `zama-solana-acl` seed-list
  functions. It also replaces other programs' PDAs with their maintained clients:
  `findAssociatedTokenPda` from `@solana-program/token`. The BPF loader's program data address is stored in the
  program account, so the chain is its source and a client reads it rather than derives it.
  `@solana-program/loader-v3` 0.7.0 ships no decoder for that account, so until one does, a single
  helper in `solana/deploy` derives it and the test suite imports that helper.

Pinned by `dead-surface-check.sh` check 8 and its `--self-test` fixtures.

## DD-073: The SDK reads decryption trust from zama-host

Status: adopted

Context:

A decryption needs to know which KMS to trust: the context and epoch a request is routed to, the
signers that answer it, and the gateway domain their signatures are made under. The Solana private
decrypt client took all of it from the caller, as a `trust` object, while the public decrypt client
already read the context and epoch from `HostConfig`. Every caller, from the demo dapp to the test
suite, restated values the host program already holds, and a stale copy failed only when the KMS
answered under another context.

The EVM SDK reads the same trust from the chain. It mints under
`ProtocolConfig.getCurrentKmsContextAndEpoch()`, verifies a response against the signers
`KMSVerifier` returns for the context in the permit's `extraData`, numbers them `1..n` in that order,
and caches both reads for 15 minutes.

Decision:

The SDK reads decryption trust from zama-host, as the EVM SDK reads it from Ethereum.

- A permit and a public decrypt are routed to `HostConfig.current_kms_context_id` and
  `current_kms_epoch_id`. An unset pair is refused.
- A user-decrypt response is verified against the `KmsContext` the permit names, not the current
  one, so a permit stays usable after a context switch until its context is destroyed. A missing or
  destroyed context is refused.
- That context's signers are parties `1..n` in their registered order. The client passes no
  threshold: the KMS WASM derives it from the signer count, as on EVM.
- The gateway domain is `Decryption`, version `1`, on `HostConfig.gateway_chain_id` and
  `decryption_contract`.
- The FHE parameter stays deployment configuration: zama-host does not hold it.
- `HostConfig` and each `KmsContext` are read at finalized and cached for 15 minutes per client,
  sharing in-flight reads. A failed read is not cached. The cache lives in the client, because the
  client fixes the cluster and RPC; Solana program ids can repeat across clusters.

This answers RFC 036 open question 1.

Rationale:

The host program is the record the KMS connector and the on-chain verifier already act on. A copy in
each caller can only drift from it. Reading it keeps one source (DD-072) and gives Solana the EVM
SDK's outcome and revocation window.

Consequences:

- zama-host's current context and epoch must be a pair Ethereum's `ProtocolConfig` holds as valid,
  because the KMS connector validates each request against it. A permit minted under a pair
  Ethereum never activated, or has destroyed, is refused by the connector. zama-host must follow
  Ethereum's context switches and epoch rotations.
- The public decrypt client keeps a fresh read after the certificate arrives, so a context destroyed
  while a request waits is still refused.
- Callers no longer pass signers, routing or a domain; the demo dapp and the test suite drop those
  values from their configuration.
