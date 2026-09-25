# Solana design decisions

Last synced: 2026-09-17.

Each numbered entry records one decision the Solana port relies on and why it was taken. Entries
are appended, never renumbered. A decision that a later one replaces moves to
[`DESIGN_HISTORY.md`](DESIGN_HISTORY.md) with its original text, so this file holds only decisions
the code follows today. Where a live entry is partly superseded, a note under its status says by
which decision and on which points.

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
| [DD-008](#dd-008-model-transient-allow-as-explicit-solana-evidence)                                                                       | adopted; see the note under its status   | Model Transient Allow As Explicit Solana Evidence                                                                               |
| DD-009                                                                                                                                    | replaced by removed; fhevm-internal#1692 | Operator Transfer Model Removed, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                                      |
| DD-010                                                                                                                                    | replaced by DD-040                       | Token Disclosure Paths Are Label-Scoped, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                              |
| DD-011                                                                                                                                    | replaced by DD-042 composition           | Transfer-And-Call Removed In Favor Of App-Driven CPI Composition, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                     |
| [DD-012](#dd-012-solana-user-decrypt-reuses-the-gateway-stack)                                                                            | adopted                                  | Solana User Decrypt Reuses The Gateway Stack                                                                                    |
| [DD-013](#dd-013-prefer-fail-closed-chain-boundaries)                                                                                     | adopted                                  | Prefer Fail-Closed Chain Boundaries                                                                                             |
| [DD-014](#dd-014-host-handle-creation-has-no-local-test-relaxation)                                                                       | adopted                                  | Host Handle Creation Has No Local Test Relaxation                                                                               |
| [DD-015](#dd-015-handle-creation-keeps-per-block-entropy)                                                                                 | adopted                                  | Handle Creation Keeps Per-Block Entropy                                                                                         |
| [DD-016](#dd-016-confidential-balances-use-the-immediate-available-balance-profile)                                                       | product-open                             | Confidential Balances Use The Immediate-Available-Balance Profile                                                               |
| [DD-017](#dd-017-role-aware-fhe_execute-and-per-op-bind-instructions-replace-the-rfc-024-execute_frame-prototype)                         | adopted                                  | Role-Aware `fhe_execute` And Per-Op Bind Instructions Replace The RFC-024 `execute_frame` Prototype                             |
| DD-018                                                                                                                                    | replaced by DD-011                       | Transfer-And-Call Refund Prepare/Finalize (replaced), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                 |
| DD-019                                                                                                                                    | replaced by DD-049                       | Confidential Transfer Persists Only Final Balance And Transferred-Amount ACL Records, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md) |
| [DD-020](#dd-020-verifierset-removed--canonical-kms-context-singleton)                                                                    | adopted                                  | VerifierSet Removed → Canonical KMS Context Singleton                                                                           |
| [DD-021](#dd-021-on-chain-secp256k1-kms-public-decrypt-cert-verification)                                                                 | adopted                                  | On-Chain secp256k1 KMS Public-Decrypt Cert Verification                                                                         |
| [DD-022](#dd-022-witness-pdas-created-before-the-secp-consume-request--consume-once)                                                      | adopted                                  | Witness PDAs Created Before The secp Consume (request → consume-once)                                                           |
| [DD-023](#dd-023-fhe_execute-composed-executor--typed-fheexecutionbuilder-dsl-dd-017-realized)                                            | adopted                                  | `fhe_execute` Composed Executor + Typed `FheExecutionBuilder` DSL (DD-017 realized)                                             |
| [DD-024](#dd-024-eager-ciphertext-material-preparation-coprocessor-side)                                                                  | adopted                                  | Eager Ciphertext-Material Preparation (coprocessor side)                                                                        |
| [DD-025](#dd-025-where-the-release-gate-sits)                                                                                             | adopted                                  | Where The Release Gate Sits                                                                                                     |
| [DD-026](#dd-026-input-and-identity-encoding-is-bytes32-user-decrypt-is-typed)                                                            | adopted; see the note under its status   | Input And Identity Encoding Is bytes32, User Decrypt Is Typed                                                                   |
| [DD-027](#dd-027-chain-aware-v2-user-decrypt-validation)                                                                                  | adopted; see the note under its status   | Chain-Aware V2 User-Decrypt Validation                                                                                          |
| [DD-028](#dd-028-what-the-port-does-not-do)                                                                                               | adopted                                  | What The Port Does Not Do                                                                                                       |
| [DD-029](#dd-029-drift_revert--on-chain-reorg-disambiguation)                                                                             | adopted                                  | `drift_revert` ≠ On-Chain Reorg (disambiguation)                                                                                |
| [DD-030](#dd-030-keep-verifyproofrequestsolana-not-a-v2-rename)                                                                           | adopted                                  | Keep `verifyProofRequestSolana`, Not A V2 Rename                                                                                |
| [DD-031](#dd-031-materiality-moves-to-the-gateways-ciphertextcommits-dd-006-revision)                                                     | adopted                                  | Materiality Moves To The Gateway's `CiphertextCommits` (DD-006 revision)                                                        |
| DD-032                                                                                                                                    | replaced by DD-049                       | `EncryptedValue` + MMR Replaces Keyed-Nonce `AclRecord` (RFC-024), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                    |
| [DD-033](#dd-033-no-acl-lifecycle-events--self-describing-args--instruction-replay-indexing)                                              | adopted; see the note under its status   | No ACL-Lifecycle Events — Self-Describing Args + Instruction-Replay Indexing                                                    |
| [DD-034](#dd-034-eager-compute-scheduling-for-solana-q11-option-a)                                                                        | adopted                                  | Eager Compute Scheduling For Solana (Q11 Option A)                                                                              |
| DD-035                                                                                                                                    | replaced by DD-048                       | Standalone Untrusted Solana MMR Proof Service, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)                                        |
| DD-036                                                                                                                                    | replaced by DD-045                       | Burn-Redemption Consume Authorizes By MMR Public-Decrypt Proof, Not Live Handle, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)      |
| DD-037                                                                                                                                    | replaced by DD-038                       | `fhe_execute` Events — `emit_cpi!`-Only, No `emit!` Log Fallback (DD-033 addendum), in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)   |
| DD-038                                                                                                                                    | replaced by removed; fhevm-internal#2079 | One Host-Owned Born-Public Lifecycle Batch Replaces Per-Operation Events, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)             |
| DD-039                                                                                                                                    | replaced by DD-047                       | HCU Block Cap Meters The Signed `compute_subject`, Not A Separate Authority, in [DESIGN_HISTORY.md](DESIGN_HISTORY.md)          |
| [DD-040](#dd-040-app-public-decrypt-is-a-stateless-pull-oracle-verifier-not-a-request-lifecycle)                                          | adopted                                  | App Public-Decrypt Is A Stateless Pull-Oracle Verifier, Not A Request Lifecycle                                                 |
| [DD-041](#dd-041-coprocessor-input-trust-is-a-registered-n-of-m-signer-set-in-hostconfig)                                                 | adopted                                  | Coprocessor Input Trust Is A Registered n-of-m Signer Set In `HostConfig`                                                       |
| [DD-042](#dd-042-confidential-vaults-are-a-batcher-gateway-in-front-of-a-public-share-mint-vault)                                         | adopted; see the note under its status   | Confidential Vaults Are A Batcher-Gateway In Front Of A Public Share-Mint Vault                                                 |
| [DD-043](#dd-043-two-derivation-regimes--content-addressed-deterministic-handles-persistent-write-anchored-rand-seeds-context_id-deleted) | adopted                                  | Two Derivation Regimes — Content-Addressed Deterministic Handles, Persistent-Write-Anchored Rand Seeds (`context_id` deleted)   |
| [DD-044](#dd-044-every-event-goes-through-the-event-cpi-or-is-not-emitted-at-all-emit-events-deleted)                                     | adopted; see the note under its status   | Every Event Goes Through The Event CPI, Or Is Not Emitted At All (`emit-events` deleted)                                        |
| [DD-045](#dd-045-keep-burn-settlement-sequential-and-keep-wrapper-policy-separate-from-host-governance)                                   | adopted; see the note under its status   | Keep Burn Settlement Sequential and Keep Wrapper Policy Separate From Host Governance                                           |
| [DD-046](#dd-046-the-program-heap-is-fixed-at-32-kb--no-custom-allocator-raised-heap-deleted)                                             | adopted                                  | The Program Heap Is Fixed At 32 KB — No Custom Allocator (`raised-heap` deleted)                                                |
| [DD-047](#dd-047-the-application-is-program-scope--program-verified-scope-declared-rfc-035)                                               | adopted; see the note under its status   | The Application Is `(program, scope)` — Program Verified, Scope Declared (RFC 035)                                              |
| [DD-048](#dd-048-allows-are-sealed-on-the-write-the-deny-list-names-applications-one-connector-path-rfc-035)                              | adopted; see the note under its status   | Allows Are Sealed On The Write; The Deny List Names Applications; One Connector Path (RFC 035)                                  |
| [DD-049](#dd-049-shared-encrypted-store-and-transaction-local-result-grants)                                                              | adopted                                  | Shared Encrypted Store And Transaction-Local Result Grants                                                                      |
| [DD-050](#dd-050-transient-storage-shared-across-the-transaction)                                                                         | adopted                                  | Transient Storage Shared Across The Transaction                                                                                 |
| [DD-051](#dd-051-a-zama-is-one-host-program-id)                                                                                           | adopted                                  | A Zama Is One Host Program ID                                                                                                   |
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

## DD-002: Keep App Store And Host ACL Store Separate

Status: adopted

Context:

The confidential token program owns token semantics. The host program owns FHEVM authorization
semantics. Mixing those responsibilities would make it unclear which program is authoritative for
decrypt or compute permission.

Decision:

`confidential-token` stores token-local pointers such as current balance handles and emits
app-local indexing events. `zama-host` stores canonical ACL, material, delegation, and transient
authorization state.

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
policy-approved transaction/account data and verified against host-owned ACL,
material, delegation, and replay witnesses.

Rationale:

Decrypt authorization cannot depend on whether a provider preserved a log line. It also cannot
require every production path to spend a self-CPI frame solely for observability.

Consequences:

The port keeps Anchor CPI events for tests and local listener compatibility, but production event
transport should use a Yellowstone/Geyser transaction and account stream with explicit commitment,
reconnect, replay, and account-witness verification policy.

The current listener is built from source by the side-stack setup (`test-suite/fhevm/src/solana/deploy.ts`); the shared
host-listener container remains EVM-only and intentionally does not package the feature-gated Solana
binary. A production Solana image and deployment topology remain packaging work, not an implicit
fallback to the deleted RPC listener.

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

The port needs a production-shaped encrypted input path. The earlier design (below) bound inputs
through a bespoke native Ed25519 "input verifier set" signing a `SolanaInputBindIntent`. That set
was a Solana-only trust root divorced from the EVM coprocessor trust model.

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

The gateway side is the RFC-021 bytes32 input path:
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

What changed:

- The bespoke input verifier-set and the `verify_input_and_bind` Ed25519 path were REMOVED.
- Inputs are now the `FheExecuteOperand::VerifiedInput` operand of `fhe_execute`. The earlier standalone
  `verify_coprocessor_input` instruction and its `InputVerifiedEvent` receipt were **deleted**, along
  with the short-lived output-taint binding (`VerifiedInputBinding` / output-ACL constraints): derived
  outputs are unconstrained by the input.
- The "caller is the attested contract" gate is enforced at input-consumption time
  (`attestation.contract_address == program`, DD-047).
- The `verify_input_and_bind` and standalone `mock_input_verified_and_bind` instructions were removed;
  the shared verifier `zama_host::eip712::verify_coprocessor_input` (via
  `instructions::input_verification::verify_input_attestation`) is invoked in-execution by `fhe_execute`.

Replaced design (stub): the earlier `verify_input_and_bind` bound inputs with a native Ed25519
"input verifier set" signing a `SolanaInputBindIntent`. Reversed because it was a Solana-only trust
root divorced from the EVM coprocessor; the coprocessor attestation is the canonical trust root.

Open for debate / follow-up: the input proof / ZKPoK / transciphering behind the attestation is still
a harness shortcut; real ZKPoK + transciphering is production work.

## DD-008: Model Transient Allow As Explicit Solana Evidence

Status: adopted

The `StoredValue` output API named here was replaced by DD-049's Store slot writes.

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
explicit. Keeping intermediates instruction-local avoids rent and prevents a temporary compute grant
from silently becoming persistent ACL or decrypt authority.

Consequences:

The earlier persisted one-shot `TransientSession` / capability-account tier (a cross-instruction
handoff account with same-transaction creation proof) was **removed** (zama-ai/fhevm#2834): it was
real rent-bearing state that added a permission leak surface for no path the port needed. A Store
output derived from transient inputs still passes its authority check and declares its own allows;
nothing is public unless the output says so.

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
Gateway V2 path (RFC-016) rather than a parallel native stack:

- **User-decrypt** flows through the unified Gateway V2 path, through the Gateway's
  `solanaUserDecryptionRequest` entry, which types the handles, validity, transport key and
  `extraData` and carries the Solana permit fields in a versioned blob, rather than smuggling Solana auth through
  `extraData`. `extraData` is only the KMS routing (v0, v1 or v2, DD-060). A chain-aware validator
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
Reversed to reuse one decrypt trust model and routing path. The connector subsystem that read the
native-v0 tables was deleted earlier; the tables and the typed-column detour had no reader left, no
shared database ever applied them, and they are now gone from this branch.

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
outputs — no per-output binding. The former persistent-output binding (the per-value account ID, plus an
even earlier per-update `output_nonce_sequence` = that account's MMR `leaf_count` read at execution)
was **removed** entirely — see "Binding removal" below.

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
by the listener's reorg handling on EVM (block-status machine, DD-025). Solana accepts confirmed
eager scheduling and leaves reorg unwind as optional resource recovery (DD-025, Boundaries).

Consequences:

Handle byte layout remains stable; handle creation is not idempotent across slots/blocks. The
`PreviousBankHashUnavailable` fail-closed surface remains as designed; handle derivation never falls
back to zero entropy (DD-014).

Binding removal (persistent-output handle binding deleted entirely):

The persistent-output binding once folded two components into the handle hash: `output_nonce_sequence`
(that account's MMR `leaf_count` read at execution) and its account ID. Both
were vestiges of the retired keyed-nonce `AclRecord` (DD-001) and the root of the off-chain
reconstruction complexity (leaf-count tracking + "hints"). Both are now **deleted**. A persistent output
handle is now the plain `base_handle = computed_eval_handle(op, operands, scalar, fhe_type, chain_id,
previous_bank_hash, unix_timestamp, context_id, op_index)` — byte-identical to the transient (local)
handle. A Fable analysis confirmed the encrypted-value-ID binding was defense-in-depth: strictly _stricter_
than EVM, never required for collision safety, so removing it makes Solana match EVM's handle shape
exactly rather than weakening it.

This cannot introduce a new collision between two _distinct_ ciphertexts. A handle collision that
matters is two different ciphertext materials sharing one handle; material is fully determined by
`(op / plaintext / rand-seed, operands, fhe_type)`, all of which live in `base_handle`. So two
outputs with different material already differ in `base_handle` (birthday resistance made
non-grindable by per-block entropy, unchanged by this deletion). The binding only made _repeated
identical_ computations produce distinct handles; removing it means an identical recomputation now
yields the identical handle — which is exactly EVM's behavior (`FHEVMExecutor` binds **no**
per-output nonce, and **no** per-slot, per-caller or per-account value, for
binary/ternary/trivial/unary/cast; its only counter is the global `counterRand` folded into the rand
_seed_), so the deletion **improves** EVM parity. The original account and sequence binding are gone.

The current transaction model supersedes that historical collision analysis: all
result occurrences are recorded, including identical recomputations. Operand-bearing
preimages include the transaction-origin mask; the journal determines that mask before
recording the result. Equal handles refer to the same encrypted computation, while
Store identity and explicit grants independently determine who may use it. Store slot
writes use the initial snapshot and ordered effects, rather than a duplicate-handle
rejection. Random outputs retain their nonce-derived seed. See the canonical preimage
helpers in `state/mod.rs` and the current execution invariants.

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

## DD-017: Role-Aware `fhe_execute` And Per-Op Bind Instructions Replace The RFC-024 `execute_frame` Prototype

Status: adopted

Context:

RFC-024 sketched one batched `execute_frame(authorized_app_accounts[], steps[], actions[])` entry
point and recorded removing an earlier `app_account_authority` signer that "was never validated by the
host." The implementation diverged from that sketch and the reversal was not previously recorded here.

Decision:

The host exposes per-handle-class binding instructions — `fhe_binary_op_and_bind_output`,
`fhe_ternary_op_and_bind_output`, `trivial_encrypt_and_bind`, `fhe_rand_and_bind`,
`fhe_rand_bounded_and_bind` — plus one batched eval instruction for composed batches: `fhe_execute`. The
eval instruction accepts mixed binary/ternary, trivial-encrypt, rand, and verified-input steps with
instruction-local transients. It is the practical successor to `execute_frame`. (Input creation is not a
separate instruction: external inputs enter through the `fhe_execute` `VerifiedInput` operand, DD-007.) Every persistent-output path takes a signer witness: either the fixed
`app_account_authority: Signer` account, or an explicit per-output authority account in
`remaining_accounts` that must be a signer and match `output_app_account`. The host then validates the
metadata with `assert_output_acl_metadata` (`instructions/common.rs`). This reinstates and now
enforces the signer the RFC had removed.

The OpenZeppelin-track `execute_frame` ABI is intentionally not ported as a host instruction. Its
useful ergonomic idea — symbolic previous results inside one instruction — is represented by
`FheExecuteOperand::EarlierStep` in the host ABI and by the app-facing `zama-fhe::FheExecutionBuilder`. The SDK
builder hides raw producer indices and `remaining_accounts` indices from app code, returns typed
`Encrypted<T>` values for intermediate results, addresses Store slots through `Store.get` and
`Store.set`, declares allows directly, and returns an opaque `FheExecution`.
The `cpi` feature can resolve that batch through a pubkey-keyed account resolver, so app code does
not hand-maintain ordered host accounts. Output authority, allows and public-decrypt policy remain
enforced by the host ABI.

Event transport is DD-044: every event goes through the event CPI or is not emitted.

Rationale:

A validated `app_account_authority == output_app_account` signer makes the app account that receives
persistent ACL output prove control via a Solana signature, rather than trusting an unsigned
`authorized_app_accounts[]` declaration. Per-output signer witnesses extend the same guarantee to
multi-app evals without making authorization a free-form unsigned list. Per-class instructions remain
for compatibility and individually testable handle-creation paths; `fhe_execute` provides batched multi-step
composition with transient/persistent outputs when a single CPI is required.

Consequences:

This replaces the older RFC-024 `execute_frame` sketch and its "app_account_authority removed"
note. Multi-account atomic effects (e.g. ERC7984 transfer crediting both sender and receiver) are
expressed as one batch with per-output authority witnesses rather than a batch carrying an
unsigned `authorized_app_accounts[]`. Future multi-app eval extensions should keep that signer-witness model
and should not resurrect unsigned `authorized_app_accounts[]`.

## DD-020: VerifierSet Removed → Canonical KMS Context Singleton

Status: adopted

Context:

Witnesses and decrypt trust used to anchor to a `VerifierSet` subsystem
(`create_verifier_set` / `disable_verifier_set` / `migrate_verifier_set`), a Solana-only trust root
with its own lifecycle.

Options considered:

- (A) Keep the VerifierSet subsystem and its migration lifecycle.
- (B) Collapse trust to a single on-chain KMS context keyed by `kms_context_id`. **Chosen.**

Decision:

The VerifierSet subsystem was REMOVED. Witnesses and decrypt trust anchor to a `define_kms_context`
singleton keyed by `kms_context_id` (`zama_host::kms_context_address(context_id)`, seed
`[KMS_CONTEXT_SEED, context_id]` with a 32-byte id; `destroy_kms_context` exists for lifecycle). Decrypt
and disclosure witnesses pin the `kms_context_id` they were minted under.

Why / what worked:

Single source of truth, less divergence between a Solana-only set and the EVM KMS context. Invariant-
tested. A request pins its context id so a cert minted under context N cannot be replayed after rotation
to N+1.

Open for debate:

Context rotation governance (who may `define` and `destroy`, and the rotation choreography) is not
yet designed for production (fhevm-internal#1634).

## DD-021: On-Chain secp256k1 KMS Public-Decrypt Cert Verification

Status: adopted

Context:

Public-decrypt release needs the KMS threshold certificate verified somewhere. The earlier Solana path
verified an Ed25519 cert against a Solana verifier set; the reconciliation moves to the EVM KMS trust
model.

Decision:

`zama_host::eip712::verify_kms_public_decrypt` recovers secp256k1 EVM signers from the cert
(`recover_evm_address`), requires a **distinct-signer threshold** (`verify_threshold`) against the
**witness-pinned `kms_context`'s** signer set / threshold (not the current context), **rejects high-s
(malleable) signatures** (`signature[32..64] > SECP256K1_HALF_ORDER`), and requires
`extract_kms_context_id(extra_data, current) == request kms_context_id`. `extract_kms_context_id`
mirrors the EVM `KMSVerifier`: empty / version-0 `extra_data` selects the current context,
versions 1 and 2 carry a big-endian context id in `extra_data[1..33]`.

Why / what worked:

Mirrors the EVM `KMSVerifier` so the same threshold cert verifies on both sides. Adversarial cases
(wrong threshold / wrong signer set / context mismatch — the "L4-b/c/d" harness rejections) are rejected
live.

Open for debate:

The harness exercises the KMS connector decrypt, not full production KMS-connector wiring (DD-028).

## DD-022: Witness PDAs Created Before The secp Consume (request → consume-once)

Status: adopted

Historical decision, fully superseded. The disclosure witness was dissolved by DD-040
(fhevm-internal#1704), and the burn-redemption witness was dissolved by DD-040's deferral closure
(fhevm-internal#1763). Token disclosure is now the generic `disclose_secp` consumer of the stateless
host verifier; burn act-once state is the sequential `PendingBurn` described by DD-045.

Context:

Disclosure and burn-redemption decrypt-release flows need a replay-safe, context-pinned, expiring
request record so a cert can only be consumed once, against the context it was requested under.

Decision:

`confidential-token` creates request-witness PDAs **before** the secp consume:

- `request_disclose_balance` / `request_disclose_amount` → `DisclosureRequest` PDA.
- `request_burn_redemption` → `BurnRedemptionRequest` PDA.

Each carries `kms_context_id` (pinned at request time — "the response cert must verify against this
context's signer set, not the current one"), `request_nonce`, `expires_slot`, and `request_hash`
(plus the handle / ACL record / material commitment + hash + key id it is bound to). The consume
(`disclose_amount_secp` / balance / `redeem_burned_amount_secp`) verifies the secp cert against the
pinned context and consumes the request once; `close_consumed_*` and `close_expired_*` reclaim rent.
Replay / expiry / context-mismatch are rejected (Mollusk + live).

Why / what worked:

Request-before-consume gives a persistent, replay-once witness with explicit expiry and pinned context.
This replaces the earlier "verify against `host_config.current_kms_context_id`" hazard where a cert for
context N could be consumed after rotation to N+1.

Open for debate:

Expiry slot policy and request-PDA rent reclamation cadence are not yet designed for production.

## DD-023: `fhe_execute` Composed Executor + Typed `FheExecutionBuilder` DSL (DD-017 realized)

Status: adopted

Realizes DD-017.

Context:

DD-017 set the direction: a batched `fhe_execute` with instruction-local transients updating the
RFC-024 `execute_frame` sketch. The reconciliation realized it end-to-end.

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
slots and no intermediate, while each Store output keeps its authority check (DD-017).

Open for debate:

The step cap `MAX_FHE_EXECUTION_STEPS` is derived from measured instruction-data and compute-unit budgets
on the interned wire format (fhevm-internal#1853 W8; see the constant's doc in
`programs/zama-host/src/constants.rs`). The per-operation replay-event transport and the
created-public batch that replaced it are gone (DD-038, in DESIGN_HISTORY.md).

## DD-024: Eager Ciphertext-Material Preparation (coprocessor side)

Status: adopted

Context:

Ciphertext/SnS preparation is expensive but does not authorize plaintext release. The KMS separately
validates the Store and the leaf proof before decrypting.

Decision:

Confirmed instruction reconstruction emits concrete material requests at handle creation and Store
update. The listener inserts those handles directly into `pbs_computations`. Later allows reuse
already-prepared material. No account-fetch queue, witness
store, retry state machine, or coprocessor-owned ACL decision remains.

Why / what worked:

This removes the finalization delay and duplicate Solana RPC read. A rolled-back computation can waste
work, but prepared ciphertext material is not authorization and cannot cause plaintext release.

Open for debate:

None on the coprocessor side. KMS commitment and authorization semantics are documented separately.

## DD-025: Where The Release Gate Sits

Status: adopted

Confirmed, eager materialization; live KMS authorization at release time.

Context:

The previous Solana ingestion inserted computations dormant and activated them per finalized allow.
That did not compose with transient eval intermediates: a `confidential_burn`'s burned-amount handle
depends on transient sub-handles that are never individually allowed, so a per-handle allow gate could
not activate the graph that produced the released handle.

Separately, the EVM reorg substrate already implements the recommended shape: a block-status machine
(`pending → finalized / orphaned` in the `host_chain_blocks_valid` table) plus ancestor catch-up in
`cmd/block_history.rs`. The **Solana listener (`bin/solana_host_listener.rs`) reconstructs from a
Yellowstone stream at `confirmed` and inserts directly** — it is NOT wired into this substrate.

Options considered:

- (A) Eager-materialize and gate decrypt release on finality. Rejected: prepared material is not an
  authorization, and the accepted confirmed authorization may release plaintext.
- (B) Keep the two-step dormant model + add transitive subgraph activation via a recursive CTE
  (activate the whole producing subgraph when the released handle is allowed).
- (C) Slot-level finality gate.
- (D) Ingest only at finalized (+~13s latency).

The accepted design is eager materialization from confirmed instruction reconstruction with no
separate finality gate; KMS revalidates confirmed authorization at the plaintext-release boundary.

The accepted product rule treats a valid confirmed authorization as sufficient. Coprocessor work is
therefore scheduled from confirmed ingestion. The KMS connector's ACL read and the host listener's
confirmed Yellowstone ingest use explicit confirmed commitment; KMS remains the only
plaintext-release boundary.

Decision provenance: accepted by the Solana feature owner during the review of
[`zama-ai/fhevm#3122`](https://github.com/zama-ai/fhevm/pull/3122) on 2026-07-13. The accepted trade-off
is irreversible plaintext release after a valid authorization observed on an exceptionally rolled-back
confirmed fork; subsequent on-chain actions still follow the surviving fork.

The dormant/activate model and transient eval intermediates were designed separately and do not
compose. A finality gate adds latency without strengthening the chosen authorization rule: an allowed
key authorized in confirmed state was legitimately allowed to receive that plaintext, even if the fork
later rolls back.

Open for debate:

Reorg unwind may still be added for resource recovery, but is not an authorization dependency.

## DD-026: Input And Identity Encoding Is bytes32, User Decrypt Is Typed

Status: adopted

The user-decrypt `extraData` debate is resolved by typed gateway fields. The chain-type marker is superseded by DD-052.

Context:

The unified bytes32 input path must encode non-EVM (Solana) dapp/user identities. Separately, a Solana
_user-decrypt_ request must carry ed25519 auth (user identity, nonce, allowed scopes). These
are two DIFFERENT surfaces and the earlier docs conflated them — this DD disentangles them.

Decision:

**Input path (identities are bytes32; NO `0x03` blob):**

- Non-EVM bytes32 input via `InputVerification.verifyProofRequestSolana` + event
  `VerifyProofRequestSolana` (dapp/user are 32-byte host addresses; shares zkProofId + consensus with
  the EVM path; request stored in `solanaZkProofInputs` for bytes32 EIP-712 response validation).
- Which `u64` is a Solana host chain id is DD-052. Relayer `is_solana_host_chain_id`
  matches type byte `0x01`.
- The input's `extraData` is the **coprocessor cert's EIP-712 `CiphertextVerification` extraData** — it
  is NOT, and never was, the `0x03` Solana user-decrypt blob. The input identity itself is a plain
  bytes32 host address (no version-byte blob).

**User-decrypt path (typed identity and auth fields):**

- PREVIOUSLY a Solana user-decrypt packed its ed25519 auth into an `extraData` blob with version byte
  `0x03` (`0x03 ‖ context_id(32) ‖ ed25519(32) ‖ nonce(32) ‖ key_count(4) ‖ keys`), forwarded opaquely
  through relayer/gateway and decoded by the KMS connector.
- Now the gateway's `solanaUserDecryptionRequest(ctHandles, requestValidity, publicKey, extraData,
  solanaRequest)` types the fields it budgets and charges, and carries the rest in the
  `solanaRequest` blob: `0x05 ‖ borsh{user_address, allowed_scopes, verifying_program_id, signature,
  entries}`. It emits `SolanaUserDecryptionRequest`. `SolanaUserDecryptRequest::assemble`
  (`zama-solana-request`) joins the two parts into the typed request every authorizer reads, so no
  fact travels twice. One claim per handle names
  its owner and Store (DD-048, DD-049), and `extraData` is only the KMS routing (DD-060). The
  connector fetches leaf proofs itself, so neither client nor relayer can substitute proof data.
- The js-sdk builds the blob and the relayer submits the call. The KMS connector routes Solana
  requests by their event, joins the two parts and verifies the permit signature before using them.
- KMS-cert context: `extract_kms_context_id` (DD-021) handles `extra_data` versions 0, 1 and 2 (the
  public-decrypt cert) — a _different_ extraData from either path above.

A bytes32 identity plus a Solana chain id (DD-052) keeps one input ABI for EVM and non-EVM hosts. For user-decrypt,
typed gateway fields make the Solana identity and auth request self-describing, and `extraData` stays
the KMS routing field it is on EVM.

Decision history:

The 2026/06/12 Solana guild weekly (Manoranjith + Jad) objected that identity and authorization scope
were being smuggled through `extraData` and should be a proper request type. A dedicated typed
entrypoint (`userDecryptionRequestSolana`) resolved that first. `solanaUserDecryptionRequest` and
its versioned blob replaced it, and `extraData` carries no Solana identity, scope or proof data. It
is a named entry rather than an overload of `userDecryptionRequest`, so the EVM entries keep their
generated binding names.

## DD-027: Chain-Aware V2 User-Decrypt Validation

Status: adopted

The chain-type detector is superseded by DD-052.

Context:

Admitting Solana over the unified V2 user-decrypt path (DD-012) required relaxing EVM input validation
(empty `contractAddresses`, 128-or-130-char signature).

What didn't work:

The reconciliation first relaxed this **unconditionally**, which weakened EVM — a CI integration test
caught empty-contracts / wrong-sig being accepted on the EVM path.

Decision / fix:

A **cross-field validator branches on `contracts_chain_id`** via `is_solana_host_chain_id` (type byte
`0x01`; the predicate’s meaning is DD-052): EVM-strict (non-empty contracts, exact EIP-712 130-hex
signature) vs Solana-relaxed (empty contracts allowed, 128-or-130-char signature). Per-field
validators stay permissive; strictness is enforced in the cross-field branch.

Why / what worked:

Branching on the chain type keeps EVM strictness intact while admitting Solana. The CI integration
test that caught the regression now passes for both. This entry only keeps that split.

Open for debate:

The Solana-relaxed signature acceptance (128 ed25519 vs 130) is the seam most likely to need tightening
once the input-identity encoding (DD-026) is frozen.

## DD-028: What The Port Does Not Do

Status: adopted

- **KMS connector decrypt** is exercised in the harness, **not** full production KMS-connector wiring.
- **Solana on-chain REORG handling is NOT wired** into the listener's block-status machine: the Solana
  Yellowstone listener reconstructs at `confirmed` and inserts directly, bypassing the EVM
  `host_chain_blocks_valid` / `block_history.rs` substrate. KMS authorization remains independent;
  reorg unwind would recover wasted work (DD-025).
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

They have different triggers, owners, and remedies; conflating them muddles both the reorg gap (DD-025)
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

KMS public-decrypt admission no longer checks a sealed material commitment on-chain; it relies on the
gateway's `CiphertextCommits` for materiality and on the Store MMR (DD-049) for
authorization. `HandleMaterialCommitmentWitness` is deleted from the KMS connector SDK.

## DD-033: No ACL-Lifecycle Events — Self-Describing Args + Instruction-Replay Indexing

Status: adopted

Superseded in part by DD-049: Store identity, slot keys and sealed allows. Revised by DD-056: result
handles and their block context now travel in the execution's event.

Context:

Store lifecycle changes could emit Anchor events (`emit!`/`emit_cpi!`) the way compute-step events
do, or stay event-free and let consumers decode instruction data instead.

Decision:

Store-changing paths (`fhe_execute` Store outputs and `make_store_handle_public`) emit no ACL
lifecycle Anchor events by design. The host listener reconstructs compute requests and MMR leaves
from confirmed Yellowstone transaction instructions, including
inner CPI instructions, since confidential-token and other app programs invoke the host via CPI.
Store outputs carry the expected previous handle and leaf count, so every transaction is
independently interpretable off-chain and the listener reconstructs leaves from instruction data
alone, in replay order, without reading account state first. Compute facts, including which
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
The host listener's `solana_reconstruct.rs` decode arms parse raw instruction data (Anchor
discriminators + borsh args) instead of dispatching on ACL events.

## DD-034: Eager Compute Scheduling For Solana (Q11 Option A)

Status: adopted

Context:

Under the old model, ACL "allow" signals gated whether the coprocessor would schedule an FHE
computation at all. With handles living in Store slots, that gate no
longer maps cleanly onto MMR-based historical authorization.

Decision:

Solana computations are inserted eager/schedulable immediately. Concrete persistent handles are also
inserted directly into `pbs_computations` for SnS preparation. The coprocessor does not decide decrypt
availability; the KMS connector reads the Store and verifies the leaf proof.

Rationale:

Reorg unwind stays unimplemented on the Solana listener path (DD-025/DD-028). A minority-fork
computation can waste work, but KMS authorization is independent of coprocessor scheduling and material
preparation.

Consequences:

Coprocessor scheduling and decrypt authorization are decoupled for Solana. Material can be prepared
before a decrypt request; plaintext is released only after KMS authorization succeeds.

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

A new host instruction `verify_public_decrypt` is a CPI-able, stateless verifier. It verifies a KMS
`PublicDecryptVerification` secp256k1 threshold certificate plus an MMR public-leaf inclusion proof
(`zama_solana_acl::authorize_public`, exact-handle, no roll-forward) and returns the proven
`(handle, cleartext, context_id)` via `set_return_data` (96 bytes: `handle ++ cleartext ++
context_id`, the last 32 bytes the verified context id, well under the 1024-byte limit). It creates nothing, mutates nothing, emits nothing, and takes no signer — all three accounts
(`host_config`, `kms_context`, `encrypted_value`) are read-only. An app CPIs it, asserts the returned
handle equals the handle it pinned at request time, then applies its own state transition; act-once
and timeout live in the app's own state machine (a settled flag + deadline), which it needs anyway.
This generalizes the DD-036 precedent (burn-redemption authorizes by MMR public-decrypt proof

- cert) instead of the token's witness pattern. Note that DD-036's "rather than live state" half no
  longer holds for redemption: DD-045 restored `current_handle == burned_handle` there, because one
  `PendingBurn` per token account keeps the burned handle current. This verifier is the path where
  authorizing a handle the account has since replaced still works, since it reads no live handle at
  all.

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
ever want one — noted, not in scope. Earlier revisions of this DD verified against the CURRENT context
only and framed a cert-after-rotation as a hazard to fail closed on; that framing is replaced here —
rotation is no longer the revocation boundary, `destroy` is.

### Ops runbook: KMS context rotation (fhevm-internal#1862 #15)

1. **Rotate for hygiene:** `define_kms_context(N+1)` (new signer set becomes current). In-flight certs
   that name live context `N` remain verifiable — do **not** treat rotation alone as revocation.
2. **Revoke old set:** after grace (or immediately on compromise), `destroy_kms_context(N)`. That
   flips `destroyed` and fails every outstanding `N`-named cert at `verify_public_decrypt`.
3. **Compromise path:** `define` the replacement, then `destroy` the compromised context in the same
   ops window. Forgotten destroy = old signers stay powerful indefinitely (same as EVM).
4. **Token policy:** confidential-token `disclose_secp` / `redeem_burned_amount` accept any
   non-destroyed context the cert names (default). Apps that need current-only can compare
   `return_data`'s context id to `host_config.current_kms_context_id` — not wired in the token today.

The verified context id is surfaced in `return_data` (32 bytes appended after `handle ++
cleartext`, so 96 bytes total) precisely so a calling program can pick its own policy: an
informational consumer accepts any live context, while a value-releasing instruction can compare the
returned id against `host_config.current_kms_context_id` and demand current-only. Confidential-token's
`disclose_secp` and `redeem_burned_amount` both take the default (accept any live context), matching
EVM.

The verifier stops only under the `public_decrypt` pause flag (DD-058). An already-sealed leaf is
already-public information, so re-proving it reveals nothing new; the flag exists to stop programs
acting on forged certificates while a compromised KMS context is destroyed.

Return-data-only to start: today's KMS cleartexts are ≤32 bytes; if larger types are ever revealed the
fallback is a caller-provided scratch account. The proof-freshness (stale-proof) retry race is the
known bounded-retry surface (#1687): an update between proof generation and consume moves the MMR
peaks and fails the inclusion proof; the victim regenerates the proof and retries. The one wrong app
pattern is binding consume logic to the live `current_handle` instead of the sealed handle — the
sealed leaf is append-only, so the OLD sealed handle stays verifiable after an update (covered by
`mollusk_verify_public_decrypt_survives_update_after_seal`).

Scope: this PR added the host verifier additively. Dissolving the confidential-token `DisclosureRequest`
lifecycle (`request_disclose_*`, `disclose_*_secp`, `close_*_disclosure_request`,
`state/disclosure_request.rs`) and re-expressing token disclosure as a thin consumer of this verifier
landed in fhevm-internal#1704 (PR 2); the net code deletion is recorded in the Dissolution completed
note below.

Dissolution completed (fhevm-internal#1704, PR 2):

PR 2 has landed. The confidential-token disclosure request lifecycle is deleted and re-expressed as a
thin consumer of this verifier.

Deleted from `confidential-token`: instructions `request_disclose_balance`, `request_disclose_amount`,
`disclose_balance_secp`, `disclose_amount_secp`, `close_consumed_disclosure_request`,
`close_expired_disclosure_request`; the `state/disclosure_request.rs` account (`DisclosureRequest`);
the events `BalanceDisclosureRequestedEvent`, `AmountDisclosureRequestedEvent`, `BalanceDisclosedEvent`,
`AmountDisclosedEvent`; and the helpers `assert_disclosure_request_witness`, `authorize_disclosed_handle`,
`assert_current_balance_encrypted_value`, plus the now-orphaned `allow_public_decrypt` /
`assert_token_amount_encrypted_value`.

Added: ONE generic thin instruction `disclose_secp(kind, handle, cleartext, signatures, extra_data, proof)`
(`instructions/disclose_secp.rs`) that CPIs `zama_host::verify_public_decrypt`, reads its return_data
via `get_return_data` (asserting the program id is `zama_host` and the returned handle equals the
caller-pinned `handle`), binds the disclosed Store to the named token state field's mint scope,
canonical address, Store authority and slot key, and emits one event carrying that complete binding.

Request side has no request account: a token owner or mint authority calls a confidential-token
wrapper that validates one exact token state field, then signs the host `make_store_handle_public`
CPI as the Store authority. There is no per-request PDA, no `kms_context_id` pin, and no
`expires_slot`.

Verify against the cert-named context: the cert is verified by the host against the `KmsContext` the
cert names, for any live context (see "Any live context" above), not a request-time pin. (Originally
this read "against the CURRENT context, context rotation fails closed"; replaced by
fhevm-internal#1765 — `destroy` is now the revocation boundary, not rotation.)

Idempotent by design: act-once is intentionally NOT enforced on-chain. Disclosure is idempotent
information release with no replay marker; an app needing consume-once tracks it in its own state
(the EVM-callback analogy).

Burn-redemption was subsequently dissolved onto the stateless verifier. Its act-once state is the
single `PendingBurn` account per token account described by DD-045.

Deferral closed (fhevm-internal#1763):

The burn-redemption witness has now been dissolved onto the same stateless verifier, closing the
deferral above. Deleted: `request_burn_redemption`, both `close_*_burn_redemption_request`
instructions, the `BurnRedemptionRequest` account (and its address / request-hash helpers), the
`assert_burn_redemption_request_witness` + `assert_kms_public_decrypt_cert_for_request` helpers, and
the `BurnRedemptionRequestedEvent`. Added: ONE thin `redeem_burned_amount(burned_handle,
cleartext_amount, signatures, extra_data, proof)` that binds the burned Store, CPIs
`zama_host::verify_public_decrypt`, asserts the
proven handle equals `burned_handle` and the certified cleartext equals `cleartext_amount`, then
pays out and writes the marker. Every field the witness pinned is carried elsewhere (destination
integrity by the redeem-time signer check, handle binding by the created-public MMR leaf sealed in the
burn, owner and mint by the Store), so the witness was pure scaffolding.

The stateless verifier replaces the request-time KMS pin: the cert is verified against the context it
names inside the verifier, not the witness's pinned `kms_context_id`. (This note originally said the
verifier used `host_config.current_kms_context_id` and failed closed on rotation; replaced by
fhevm-internal#1765, which accepts any live context and makes `destroy_kms_context` the revocation
lever — see "Any live context" above.)

Deny policy applies when the host seals an allow (DD-048). Redemption and cancellation seal no
allow, so a later policy change cannot trap a pending burn.

Act-once is now the closeable `PendingBurn` PDA at `["pending-burn", mint, token_account]`. Exactly
one burn may be pending for a token account. Redeem pays underlying tokens and closes it; cancel
restores confidential balance and encrypted supply and closes it. A second settlement fails because
the account is gone, and a new burn cannot start until that close has committed.

## DD-041: Coprocessor Input Trust Is A Registered n-of-m Signer Set In `HostConfig`

Status: adopted

Input `CiphertextVerification` attestations are now verified against a **registered coprocessor
signer set + configurable threshold**, matching EVM `InputVerifier`'s trust model, instead of the
prior single hardcoded `coprocessor_signer` at threshold 1. The n-of-m recovery machinery already
existed (`eip712::verify_threshold`, distinct-signer counting + high-s rejection, shared with the KMS
cert path); this wires it into input verification.

The set lives **inline in `HostConfig`**, not in a dedicated PDA (the `KmsContext` shape was the other
option). `HostConfig` gains `coprocessor_signers: [[u8; 20]; MAX_COPROCESSOR_SIGNERS]` (cap 8) +
`coprocessor_signer_count: u8` + `coprocessor_threshold: u8`, replacing the single `[u8; 20]`. A
fixed-capacity array keeps the singleton's byte layout **pinned** (the account serializes to the same
size regardless of how many signers are active), and avoids threading a second account through
`fhe_execute`, which is byte-tight. The cap is 8: comfortably above realistic coprocessor-quorum sizes
while bounding both the account size (+142 bytes vs the single-signer layout; current
`HostConfig::SPACE` is 317 after the 32-byte KMS context id) and
the worst-case per-attestation recovery cost. Rotation is admin-driven today via the
admin-gated `set_coprocessor_signers` instruction (same admin/pause-neutral pattern as the other
`set_*` config setters); a gateway-sync authority would drive it from the EVM `GatewayConfig`
coprocessor registry in production.

Registration invariants (mirroring the KMS-context rules): non-empty set, within the cap,
`1 <= threshold <= len`, no duplicate signer (distinct-signer counting would otherwise silently raise
the effective quorum), no zero-address signer. Enforced identically by `initialize_host_config` and
`set_coprocessor_signers` via one shared validator. `InitializeHostConfigArgs` now carries
`coprocessor_signers: Vec<[u8; 20]>` + `coprocessor_threshold` (no legacy single-signer field — this
is a no-compat branch); the pinned `HostConfig` layout change is resynced across the IDL, the ABI
golden manifest, and every mirrored fixture.

**Signatures carried equal the threshold, not the party count.** A verifier needs `t` valid distinct
signatures over the attestation; the coprocessor sends `t`, not `n`. This holds for **both** EIP-712
attestation families — coprocessor `CiphertextVerification` inputs and KMS `PublicDecryptVerification`
certs — so the carried EIP-712 signature payload scales with `t` (t x 65 bytes), independent of how
many signers are registered. A threshold-4 `confidential_transfer` transaction (4 x 65B sigs over the
real token account list) serializes to **989 bytes**, well inside the 1232-byte
(`solana_packet::PACKET_DATA_SIZE`) single-packet limit.

Public-decrypt **consume** transactions additionally carry an MMR inclusion proof whose size scales
with MMR depth (depth x 32B), so high threshold x deep MMR is the binding corner. After
fhevm-internal#1704 the consume path is the thin `disclose_secp` (CPIing the stateless
`verify_public_decrypt`); its transaction is ~24B **larger** than the retired `disclose_amount_secp`
(dropping the DisclosureRequest witness account is offset by the added `zama_program` account, and the
cleartext widened from a `u64` to the raw 32-byte `uint256` the verifier signs over), so the envelope
narrowed. Measured `disclose_secp` wire sizes: `t=7`/depth-0 = 917B (fits), while `t=7`/depth-10 =
1237B, `t=9`/depth-10 = 1367B, and `t=7`/depth-20 = 1557B all **overflow** one packet. The
single-packet envelope for consumes is therefore effectively `t=7` at depth 0 — any nonzero proof
depth (or `t>=9`) needs the scratch-account two-transaction fallback reserved in fhevm-internal#1704.

Relates to DD-007 (input verification model) and closes the FUTURE_DESIGN §1 / EVM_PARITY "single
coprocessor signer at threshold 1" fragile item.

## DD-042: Confidential Vaults Are A Batcher-Gateway In Front Of A Public Share-Mint Vault

Status: adopted

`compute_subject` was deleted by DD-047; the batcher's identity is its Store authority.

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
inter-batch dust leak this prevents). Lifecycle is Pending -> Dispatched -> Finalized/Canceled with
permissionless dispatch/settle/claim and an exact-refund `quit` — no operator custody of principal.

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

Claim math changed for BOTH directions (fixes fhevm-internal#1774 item 1): a claim is the exact
proportional floor `encrypted(joined) x payout_received / total_joined` in one MulDiv — same FHE op
count as before — instead of `encrypted(joined) x rate / RATE_SCALE` on a pre-floored rate. The
double rounding stranded up to RATE_SCALE-scale dust per batch (6,148,914,726 raw units measured at
a u64-scale two-user batch, vs at most one unit per claim now; pinned by
`exact_division_strands_less_than_the_rate_would`). Sum-of-claims <= payout still holds:
`sum(floor(j_i * P / T)) <= floor(sum(j_i) * P / T) = P`. The MulDiv intermediate
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
token/host CPI passes deny-list records and HCU accounts (`deny_scope_records`,
`hcu_block_meter`, `hcu_trusted_app_record`) as hardcoded `None` — the program assumes
`grant_deny_list_enabled = false` and no binding HCU cap, which is how the host test fixtures run.

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
2. **Rand / rand-bounded seeds** are compulsorily fresh. As amended by RFC 035:
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
- Duplicate persistent-output accounts within an execution are still rejected
  (`ExecutionAccountTable::claim_persistent_output`), for the decode cache and the
  read-after-write rule, not for seed freshness any more.

The nonce stays global (fhevm-internal#2081). Every execution with a rand step write-locks it, so
rand executions of all applications run one at a time; an execution without a rand step does not
take it. A nonce per application would remove that contention, at the cost of rent and a lazy
creation per application. Revisit it if rand executions become frequent enough to contend. The
preimage already binds `(program, scope)`, so that change would touch only the account.

## DD-044: Every Event Goes Through The Event CPI, Or Is Not Emitted At All (`emit-events` deleted)

Status: adopted

Revised by DD-056: `fhe_execute` also emits, one event per execution carrying what the host decided.
The rule below, that only administration emits, no longer covers that event.

Context:

DD-037 deleted the `emit!` log fallback for `fhe_execute` events, on the grounds that no consumer read
logs and the fallback hid a stranding case. The admin and config events were left as they were: emitted
with `emit!`, behind a default-on `emit-events` cargo feature, described in the code as "indexing
hints". That left three problems.

The feature made the shipped IDL misleading. The event structs were declared unconditionally, so the
IDL advertised all nine of them under any feature set; what `anchor build -p zama_host --
--no-default-features` removed — the build the e2e deploys — was the code that emits seven of them. An
reader of the IDL would wait forever for an event the deployed program never sends.

The transport did not match the claim. A log can be truncated by whichever RPC provider a reader goes
through, so a logged event is a hint rather than a delivery. That is fine for something you can
reconstruct and not fine for something you cannot, and "indexing hint" did not distinguish the two.

And the grouping was wrong. `UserDecryptionDelegationUpdatedEvent` sat with the admin events, but
`delegate_for_user_decryption` takes a `delegator: Signer` and no admin: any user may delegate their
own decrypt rights. It is a user action, not administration.

Decision:

There are two options for an event and no third. Either it is emitted unconditionally through the event
CPI, or it is not emitted at all and off-chain readers reconstruct it from instruction data over
Yellowstone, which is the normal path. `emit!` is not used anywhere in `zama-host`, and the
`emit-events` feature is deleted.

Which option an event gets is decided by whether the instruction is administration, and by nothing
else. An admin instruction changes a protocol-level setting that off-chain components have to be able
to query directly, so it emits. Everything else is reconstructed on demand.

The five admin and config events — `HostConfigUpdatedEvent`,
`DenyScopeUpdatedEvent`, `HcuAppTrustUpdatedEvent`, `NewKmsContextEvent`, `KmsContextDestroyedEvent` —
take the first option. Their eleven instructions gain Anchor's `#[event_cpi]` accounts
(`event_authority`, `program`), which is a visible ABI change: `initialize_host_config` and
`define_kms_context` go from four accounts to six, `destroy_kms_context` from three to five,
`set_deny_scope` and `set_hcu_app_trusted` from five to seven, and the six `HostAdmin` config setters
from two to four.

Note that no in-tree component reads any of the five today; the only off-chain reader of host config
state reads the account, not an event (`host-listener`'s `parse_host_config`). That is deliberate and is
not an argument against emitting them: the transport exists because the category calls for it, so that
a component which needs an admin change does not have to replay instruction data to find one. The test
is the category, not the current existence of a reader — otherwise the rule would flip every time
somebody wrote or deleted a reader.

`UserDecryptionDelegationUpdatedEvent` takes the second option and is deleted, for the categorical
reason above: delegating is a user ability, so it is reconstructed from `delegate_for_user_decryption`
instruction data like every other user action. Two facts about delegation that are true but are _not_
the reason, recorded so they are not mistaken for it: nothing in the request path consumes delegation
at all (INVARIANTS #27 records the gap — the KMS connector's `verify_delegation` checks an
already-decoded record against its canonical PDA and fetches nothing, its only callers are its own unit
tests, and the signed user-decrypt payload has no delegation field), and a reader, when it arrives,
will have to fetch the record and hand it to that checker.

`fhe_execute`'s event shares one emitter with the admin events (`event_cpi.rs`), instead of keeping
its own copy of the expansion.

Rationale:

Reliable delivery costs an account pair on the instruction and a self-CPI per emission. That is nothing
on an admin instruction, which runs when an operator changes configuration, and would be real weight on
one event per compute step — which is why the per-step shapes are still not emitted. DD-003 said the
same thing in weaker terms ("events are indexing hints"); this entry replaces that framing for
`zama-host` but not its other half, which is that authorization never rests on an event. An execution
emits one event with what the host decided (DD-056); its steps stay in instruction data.

Anchor's `emit_cpi!` macro is not used, though the bytes it produces are. It reads a binding named
`ctx`, and six of the eleven instructions emit through a shared `emit_config_updated` helper that has
no `ctx`; using the macro would mean copying a nine-field event literal into each of them. One
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

Consequences:

The `--no-default-features` build in the (since retired) `setup-solana-side.sh` is gone, since
zama-host has no features left to vary. Callers of the eleven instructions pass two more accounts:
the Mollusk fixtures and the (since retired) e2e live client were updated here, and there were no
TypeScript callers yet. `dead-surface-check.sh`'s
never-emitted-event check learned the shared emitter, without which it would have reported all eight
surviving events as dead.

## DD-045: Keep Burn Settlement Sequential and Keep Wrapper Policy Separate From Host Governance

Status: adopted

Superseded in part by DD-048 and DD-049: allows are sealed on the write and the Store model replaces per-value accounts.

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

`ConfidentialBurnEvent` intentionally does not duplicate the MMR `leaf_index`. Settlement binds the
pending account to the burned Store and handle; the supplied proof carries its own leaf index and
is checked against live peaks. The connector obtains that proof by handle from the coprocessors'
leaf record, so an event index would be redundant rather than an authorization input.

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
the wrapper boundary. Every path immediately invokes a Host instruction that enforces the canonical
config and its pause flags (DD-058). This keeps the boundary aligned without paying for a second PDA
derivation or adding redundant IDL metadata.

Confidential accounts expose ATA-like demo UX: canonical derivation, permissionless create-for, and
`getOrCreateConfidentialTokenAccountInstruction`, which reads the derived PDA and returns either the
create instruction or `null` for an already initialized account. This remains demo application code,
not a claim that the protocol SDK owns the confidential-token program.

Disclosure names a token state kind and validates its entire binding before emitting: mint scope,
canonical Store, Store authority, slot key and handle proof. Scope-only validation was rejected
because two fields within the same mint would remain interchangeable in downstream events.

## DD-046: The Program Heap Is Fixed At 32 KB — No Custom Allocator (`raised-heap` deleted)

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
3. Store outputs no longer create an account per result, so the old create cap and
   per-result system-CPI trace argument no longer apply. The runtime snapshots now show
   32-step dependent chains and 32 public outputs with eight viewers each reaching the step
   cap, and updates across Stores with 8, 32 and 55 MMR peaks reaching 16, 7 and 4 steps.
   These are shape limits; the allocator decision does not make
   a host heap failure acceptable for an application we intend to support. A failing application
   benchmark is grounds to reopen fhevm-internal#1872.

The `raised-heap` Cargo feature was half a mechanism — it lifted the SDK's on-chain step ceiling
back to the host's maximum but shipped no allocator, so a program enabling it would keep the 32 KB
allocator and land in exactly the silent abort the ceiling exists to prevent. Nothing ever enabled
it. Deleted.

Reopening condition: a benchmark showing a real application blocked by the measured shape
boundaries after the copy-reduction work (argument clone, decode-once, packet pre-sizing) landed.

## DD-047: The Application Is `(program, scope)` — Program Verified, Scope Declared (RFC 035)

Status: adopted

Superseded in part by DD-049: the Store carries slot keys, which are not seeds.

Recorded as fhevm-internal RFC 035.

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
`compute_subject` is deleted from the host, the SDK, the token program and the deposit app;
reading a stored value into a computation is admitted by its authority's signature and nothing
else.

Rationale:

A PDA is the one thing on Solana that a program, and only that program, can sign for — the
`msg.sender` analog the port had been missing. Verifying the program through the authority costs one
`create_program_address` per output and buys an unforgeable identity with no registry: the program
is its own registry entry (the "Option B" DD-039 deferred). Declaring the scope rather than deriving
it keeps the host ignorant of app seed layouts while still letting the token program meter per
mint. Requiring an owned account instead of free bytes costs one owner comparison and gives the scope a
meaning a reader can check: a permit or a delegation names an account, not a number the program chose.
Every specimen program already had such an account at the call.

Consequences:

- Wallets cannot own Stores. A Store's authority is a program PDA, so the test suite drives two
  specimen programs (`encrypted-counter`, `dep-chain`) instead of a wallet-signed `fhe_execute`;
  the live operator matrix moved to the pure conformance layer (TESTING.md).
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
  A create-time check that `scope` names an existing account owned by `program` would ground the
  identifier — the reference confidential token already satisfies it, since its scope is the
  address of a keypair-generated, program-owned `ConfidentialMint` — and would stop a program from
  forwarding a caller-supplied scope and so handing out host-level trust or metering it was never
  granted. It would not make the deny list or the meter binding. Because grounding is transitive
  (updates and metering read the stored scope, `preflight.rs` `fold_app`), such a check has to be
  unconditional and land before any Store exists that will not be wiped; it cannot be
  retrofitted onto Stores already written.

## DD-048: Allows Are Sealed On The Write; The Deny List Names Applications; One Connector Path (RFC 035)

Status: adopted

Superseded in part by DD-049: the `EncryptedStore` layout. DD-060 moves the public-decrypt Store out of `extraData`.

Recorded as fhevm-internal RFC 035.

Context:

The account carried a mutable `subjects` list (capped at eight) beside the MMR, and a decrypt had
three paths: current membership, a historical leaf proof, a public leaf proof. The historical and
public proofs came from a standalone proof service, fetched by the client and embedded in the
signed request; the connector verified them, and the relayer passed them through. The deny list
named keys, and was consulted wherever a key joined the list.

Decision:

1. **Allows are sealed on the write.** A persistent output declares the keys allowed on the handle
   it installs (`PersistentOutput::allow`); the host seals one `HistoricalAccessLeaf` per key in
   list order, then the `PublicDecryptLeaf` when the output is `make_public`. The account stores no
   list. There is no instruction to add or remove an allow afterwards — the next write declares
   the next handle's allows (the token program's `allow_balance_viewers` /
   `allow_total_supply_viewers` are exactly that: a re-write by the authority). A viewer is a
   viewer: it decrypts, and cannot grant, seal or write.
2. **One decrypt path.** A user decrypt proves the allow leaf; the current handle and a replaced
   one authorize the same way, so `authorize_current` is gone. A public decrypt proves the public
   leaf. Both proofs are fetched by the KMS connector from the coprocessors' leaf record
   (`POST /v1/solana/leaf-proofs`, API key; the configured coprocessors asked in order, each only
   for the leaves the ones before it could not prove, with no retry inside an attempt) and verified
   against the peaks the connector read on chain.
   A request names only the Store and, for a delegated entry, the delegator as owner address; a
   client-supplied proof is rejected. A public decrypt names each handle's Store beside `extraData` (DD-060).
3. **The leaf record lives in the host listener.** Leaves are recomputed from the confirmed
   instruction stream and stored in the same database transaction as the compute rows, so the two
   cannot disagree about which blocks were applied. The standalone `solana-proof-service`, the
   relayer's proof passthrough, and the SDK's RPC evidence and proof-service clients are deleted
   (DD-035 superseded).
4. **The deny list names applications.** `set_deny_scope` writes `DenyScopeRecord` at
   `["deny-scope", program, scope]`; it gates every allow the host would seal — each persistent
   write and `make_handle_public`, because sealing a public leaf is an allow. A denied key is not a
   concept any more: an application is denied, or it is not.

Rationale:

A stored list was a second source of truth beside the MMR, needed a cap, needed admin
instructions, and made "current" a special case the connector had to read live. Sealing every
allow as a leaf leaves one authorization fact per (handle, key), permanent, proven the same way
whether the handle is current or replaced. Fetching proofs connector-side removes the client from
the trust path entirely — the client could never authorize anything, but it could carry stale or
malformed evidence into a signed request — and lets the coprocessors, which already hold every
instruction, own the record instead of a fourth service replaying the chain. Denying an application
rather than a key matches what a host operator can actually judge (a program and its scope) and
what the EVM `blockAccount` denies in practice (a contract).

Consequences:

- Account layout: `121 + 64·slots + 32·peaks`, at most 4,217 bytes (`EncryptedStore::account_size`,
  INVARIANTS Part II).
- `fhe_execute` wire: `previous_subjects` and `output_subjects` gone; `allows` per persistent
  output; the deny record an execution passes is its application's.
- The connector pipeline is one explicit sequence with one observation point
  (`kms-worker/src/core/solana/pipeline.rs`); the relayer pre-checks dead delegation rows
  advisorily and nothing else (INVARIANTS #50).
- Delegation records are consumed (INVARIANTS #27 closed).
- A handle with no allows and no public leaf is undecryptable by everyone, including its author;
  that is the author's choice, not a stranding.

## DD-049: Shared Encrypted Store And Transaction-Local Result Grants

Status: adopted

Adopted with RFC 035 (fhevm PR #3883). No compatibility with the retired per-value account model.

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

Decryption names each handle's Store beside the KMS routing (DD-060) and uses exact-handle MMR proofs. Current-slot publication
and fresh slotless permissions are supported. Adding new private/public permissions to a
history-only handle is deferred to fhevm-internal#2007. Generic disclosure authenticates
Store/handle/cleartext, not a token-kind label. Original token events establish provenance.

This supersedes older per-value PDA seeds, StoredValue/PersistentOutput APIs, standalone
`make_handle_public`, v3 account extraData, and receipt-based transfer composition in this log.
The existing input-attestation, threshold-KMS, program-upgrade and confirmed-RPC trust
assumptions still apply. Resource limits remain shape-dependent; see runtime cost snapshots.

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
`fhe.output(result, state.set(key)...)`. Initial slot snapshots, duplicate-write rejection and ordered MMR cursors remain.

Production membership determines operand origin independently of the supplied witness. The 256-bit big-endian mask
enters operand-bearing handle preimages; bit 0 marks input position 0. The listener reconstructs membership per
transaction. HCU total and depth use the same journal, while each application's block meter receives only that call's
cost. Return data remains immediate CPI transport, independent of permissions and result storage.

This removes per-call transient store opening, token result-scratch/result-authority account bundles, duplicate host metering
and redundant add-zero balance copies. All affected clients must migrate together; no compatibility path is kept
for the retired wire layout. Resource snapshots include lifecycle CU overhead and separate whole-transaction packet
checks. The branch retains current slot publication and PendingBurn semantics; historical re-sharing is still #2007.

## DD-051: A Zama Is One Host Program ID

Status: adopted

Adopted for identity. zama-host closes its accounts through `close_owned_accounts` (`admin-sweep` builds only) and the deployer's `host wipe`. The preview deploy runs `host wipe` before `host deploy --allow-upgrade`; wiping from the destroy workflow is follow-up.

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
empty or the next init fails. Accounts owned by the shared demo programs are not covered.

`preview-env-deploy.yml` runs `host wipe`, then `host deploy --allow-upgrade`, which uploads this
`.so` when the bytecode differs and runs `initialize_host_config` and `define_kms_context` for this
Gateway; a plain `host deploy` refuses differing bytecode. `preview-env-destroy.yml` will
close those accounts again before it deletes the namespace and will not initialize. If that
namespace uploaded a different `.so`, destroy will write the pinned baseline `.so` back.
Pull-request CI runs on the test validator and does not touch the `DPq5y89…` on Solana devnet.
Durable GitOps environments will upgrade bytecode in place on their own program IDs.

Coprocessor `host_chains` uses `chain_id BIGINT PRIMARY KEY`. That collides only if one
coprocessor database indexes both zama-devnet and zama-testnet. Separate databases, one per Zama,
do not need a schema change. RFC 035 and RFC 036 do not change.

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
same value. The listener today takes `chain_id` from its config and does not compare it to
HostConfig (#1972). A deployment on a named public row may also compare RPC `getGenesisHash`
with the hash above to confirm it is on the intended cluster, without that comparison defining
the id.

#1880 proposed this type byte and the genesis recipe. This entry accepts both and writes
the numbers down. It rejects deriving localnet from RPC at boot, and it rejects treating
“e2e targets any cluster” as part of the id.

This entry supersedes the chain-type marker in DD-026, the bit-63 detector in DD-027,
RFC-021’s high-bit reservation as the long-term marker, and the open-product #1635
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

The host listener rebuilds all coprocessor work from sealed Yellowstone blocks at `confirmed`. The
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

The listener pairs each host `fhe_execute` with the one `FheExecutedEvent` from the host program
that follows it before the next host `fhe_execute`. Only the host can sign its event authority, so
an app cannot forge the event inside the host's instruction trace. It stores the emitted handles:
computation rows, operands that name an earlier step, ACL leaves and allowed handles all use them.
It then re-derives each handle from the decoded step, the emitted context, the followed program id
and the chain id, and compares. The sysvar subscription and the per-slot join are deleted, and the
block time the listener records is the one Yellowstone sends with the block.

What the listener does when something does not line up:

| Case | Response |
|---|---|
| A host `fhe_execute` without exactly one event of the current version, or an event whose results do not match the execution's steps | Fatal: the block is not applied and the checkpoint does not move. The listener and the program disagree on the wire format. |
| A step whose emitted handle does not re-derive | The block is applied. That step is held back: its computation row is inserted as a terminal error, so the tfhe-worker never computes it and ends its dependents as errors. Leaves and allowed handles keep the emitted handle. After the commit the listener logs the slot, signature, execution, step and both handles, and counts `coprocessor_solana_host_listener_handle_check_failures_total`. |

A mismatch means our software is wrong, and the listener cannot tell which part. If the derivation
drifted, the ciphertext it would compute is still right. If it misdecoded the step, the ciphertext
is wrong, and stored under the chain's real handle it would decrypt to a wrong value and spread to
everything computed from it, with no visible failure. Holding the step back keeps the damage to
that handle and what is computed from it. Refusing the block instead would stall every app. The emitted handle wins
for the leaves because proofs must keep matching the peaks on chain.

The check finds our bugs, not a hostile provider: a provider that lies can forge the event and the
transaction consistently.

Repair is a replay of the affected slots with the fixed listener. The operator rewinds the listener
checkpoint to a slot `S` before the failure (`rewind_solana_listener_checkpoint.sql`) and reverts the
computation rows after `S` with the existing `revert_coprocessor_db_state.sql`, which deletes the held
rows and their errored dependents. The revert refuses a Solana chain whose checkpoint is still after
`S`, since the listener would never re-ingest what it deleted; `revert_coprocessor_db_state.sh` runs
both when given `SOLANA_BLOCK_HASH`. On restart the listener replays from `S` and inserts the rows
again as new work. Leaves are not reverted: a replayed write must reproduce the leaves recorded for
it, or the listener stops. So a replay repairs computation rows, not a bug that recorded wrong
leaves. A replay older than the provider's replay window comes from the archive RPC (DD-059), so
`S` must be in the archive's history, and nothing checks that before the rows are deleted. The
runbook is in the host-listener README.

A `getBlock` response prepares into the same block the stream produces (`prepare_rpc_block`), so
archive catch-up (DD-059) reuses one decoder. It requires inner instructions and loaded addresses, which a
full-detail `getBlock` returns. A test rebuilds a slot from `getBlock` output alone.

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

`FheExecuteRandomSeedsEvent` is replaced. The listener no longer needs historical sysvar state from
its provider, only blocks. A held step still gets its material request, since the Store write is
real on chain. The automated drift revert, which runs the same revert SQL, now fails on a Solana
chain whose checkpoint is ahead, which is always the case when drift is detected; before, it deleted
rows the listener would never re-ingest. A failed revert signal stops every coprocessor service on
that database from starting, including those of EVM chains, until an operator repairs by hand.

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
| `execution` | `fhe_execute`, with the allows, transient grants and public releases it writes; the token's burn and cancel through it | ACL pause |
| `verified_inputs` | `fhe_execute` steps that consume a `VerifiedInput` | None: `InputVerifier` cannot be paused; the gateway pause stops only new proofs |
| `acl_writes` | `create_encrypted_store`, `make_store_handle_public`, `delegate_for_user_decryption`, `revoke_delegation_for_user_decryption` | ACL pause |
| `public_decrypt` | `verify_public_decrypt`, and so the token's redeem and disclose | None: `KMSVerifier` cannot be paused; the gateway pause stops only new certificates |

`verified_inputs` and `public_decrypt` act when a signed result is used, not when it is requested.
They are the levers against compromised coprocessor or KMS signers, whose results the gateway pause
cannot recall.

A pauser is a `PauserRecord` PDA `("pauser", key)`, which the admin creates, enables or disables
with `set_pauser`, as it does deny and HCU-trusted records. `pause` takes the pauser's signature and
its enabled record, and sets the flags it names. A wallet pauser must call `pause` at the top level,
by the rule `delegate_for_user_decryption` applies to a wallet delegator: a wallet's signature reaches
every program of the transaction it signed, while EVM's `msg.sender` check keeps a called contract
from pausing with a pauser's right. A PDA pauser, such as a Squads vault, pauses through CPI, as only
its own program can sign for it. `unpause` takes the admin and clears them. As on
EVM, the admin pauses only if it also holds a pauser record. Pausing an area already paused
changes nothing and emits nothing. A change stamps `updated_slot` and emits `HostConfigUpdatedEvent`,
whose `signer` names the pauser or the admin. `set_pauser` emits `PauserUpdatedEvent`.

Programs act on KMS results only through `verify_public_decrypt`, so `public_decrypt` stops forged
redemptions while a compromised KMS context is destroyed (fhevm-internal#2082). It replaces the
token's own pause check, which read the single flag; the token now reads no pause flag and passes
the config through to the host.

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
| Keep one flag and add pausers | An operator could not stop forged redemptions without also stopping every application's execution. |
| Let the admin pause without a record | EVM requires `PauserSet` membership for `pause()` even from the owner. Keeping that rule makes the pauser set the one list of who can pause. |

Consequences: `set_host_pause` is gone. `HostConfig` grows by three bytes, so every reader of its
layout changes with it: the host listener decodes it with the program's type; the KMS connector's
host-pause check and the `zama-solana-acl` decoder it used are deleted, so user decryption pauses at
the gateway alone, as on EVM. New errors: `VerifiedInputsPaused`, `AclWritesPaused`, `PublicDecryptPaused`,
`NotPauser`, `PauserRecordMismatch`; `HostConfigPaused` is now `ExecutionPaused`.

## DD-059: The listener catches up from an archive when the stream cannot replay

Status: adopted

Recorded in fhevm-internal#2085.

A listener that was down longer than the provider's replay window, about a day on a hosted
provider, used to exit and need manual recovery. It now catches up from an archive RPC and then
returns to the stream. When Yellowstone refuses the checkpoint as outside its window, the listener
lists the produced slots after it with `getBlocks` and fetches each with `getBlock`, both at
`finalized`, with full transaction details. Each block goes through `prepare_rpc_block`, which
keeps the transactions naming the host program as the stream's account filter does. It must extend
the checkpoint as the stream's validator requires: an unapplied checkpoint first and unchanged, then
each block naming the last applied one as its parent. It is applied through the same path as a
streamed block. Catch-up stops at the slot the archive had finalized when it started, so it ends
however fast the chain moves. There the listener subscribes again from its checkpoint, and the
stream's own replay check takes over; if that catch-up outlasted the window, the next pass is
shorter.

The archive is `--archive-url`, which defaults to `--url` and may be another provider's. A provider
that cannot replay from a slot at all (`from_slot is not supported`) still stops the listener: after
catch-up, the stream could never take over. An archive behind the checkpoint, an archive missing
slots after it (a block whose parent is later than the checkpoint, or, for an unapplied checkpoint,
a later block descending from it), or a failed read, is retried like a dropped subscription. Any other block that does not extend the checkpoint is a fork and stops the
listener, as on the stream. `archive_catch_up_active` is 1 during catch-up, and the existing lag
metrics show its progress.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Subscribe first and backfill alongside, as the EVM listener does | `eth_subscribe` cannot resume from a block, so EVM must backfill beside a live stream. Yellowstone resumes from a slot inside its window, so one ordered source at a time keeps a single checkpoint and a single ancestry rule. |
| Fetch only the slots holding host transactions, found with `getSignaturesForAddress` | The checkpoint needs every block's hash to check ancestry, and the address index adds a second completeness assumption. It is an optimization for later if catch-up volume matters. |
| Several gRPC endpoints with failover | EVM has none. With catch-up, a provider outage only delays processing, and switching provider is a configuration change and a restart (fhevm-internal#2085). |

Consequences:

Catch-up reads every transaction of every block after the checkpoint. On mainnet a day is about
216,000 blocks with full details, so it takes hours and needs an archive provider whose rate limits
allow it; it runs eight `getBlock` calls at a time. The listener's Solana crates decode legacy and v0
transactions only, so `getBlock` asks for version 0, and the RPC refuses a block holding a v1
transaction. Catch-up then retries that block until fhevm-internal#2080 moves the listener to crates
that decode v1. A slot rewound for repair (DD-056) no longer has to be inside the replay window, only
in the archive's history.

## DD-060: A public decrypt names its stores beside the KMS routing

Status: adopted

Recorded in zama-ai/fhevm#4120. Supersedes the v4 `extraData` carrier of DD-049.

A Solana public decrypt used to carry its Store inside `extraData` as version 4:
`0x04 ‖ contextId ‖ encryptedStore`. `extraData` is the KMS routing field on EVM, and the KMS signs
it, so the Store became part of a signed field whose version space EVM owns. A request could name
only one Store, and every layer (relayer, Gateway, connector, host verifier, SDK) had to parse a
Solana-only version.

The Gateway now has a Solana entry, `solanaPublicDecryptionRequest(bytes32[] ctHandles,
bytes extraData, bytes32[] encryptedStores)`. It takes one Store per handle, in handle order, and
emits `SolanaPublicDecryptionRequest(decryptionId, ctHandles, extraData, encryptedStores)`. It is
named rather than overloaded, so the EVM `publicDecryptionRequest` keeps its generated binding
names. `extraData` holds only KMS routing (v0, v1 or v2) on both chains. The Gateway refuses a store
count that differs from the handle count before it takes the fee. The relayer requires `encryptedStores` for Solana
handles and refuses it for EVM handles; the stores are part of the request's content hash. The
connector keeps them in `handle_encrypted_stores` and proves each handle against its own Store in
one snapshot. The host verifier reads the context from v1 or v2 exactly as EVM `KMSVerifier`
does. The SDK sends v1, `0x01 ‖ contextId`, because the host `KmsContext` has no epoch.

Rejected alternatives:

| Alternative | Why not |
|---|---|
| Keep v4 and allow several stores in it | The KMS would still sign Solana account addresses inside a field whose versions EVM defines. The Store is not a KMS routing input. |
| Put the stores in an opaque blob, as `solanaRequest` does for user decryption | The user request needs one signed blob for its permit. Public decryption has no signature to bind, so named typed fields are easier to check at every layer. |

Consequences:

The KMS certificate no longer commits to the Store. It never bound it: the host verifier checks
the public leaf against the Store it is given (INVARIANTS #22). The Solana entry
shares the decryption counter and the fee with the EVM entry. It emits one event, so the relayer
looks for the Solana request event when the EVM one is absent from the receipt.

## DD-061: A delegation is keyed by application and expires on Unix time

Status: adopted

Recorded in zama-ai/fhevm#4120. The key must be confirmed by the product owner before #4120 merges.

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

## Open product decisions

Not settled by the decisions above. Forward requirements are detailed in
[`FUTURE_DESIGN.md`](./FUTURE_DESIGN.md); this list is the short index.

- Whether resource-recovery reorg unwind should be added after confirmed eager scheduling
  (DD-024, DD-025, DD-028). Reorg unwind is unimplemented on the listener path (DD-034); live KMS
  authorization remains the plaintext-release boundary.
- Whether confidential balances move to the staged inbound-credit profile (DD-016).
- Rent and archival policy for the Store MMR (DD-049): one stable PDA serves a Store for its whole
  life and its size is bounded at `121 + 64·slots + 32·peaks` bytes, so compaction is a rent question,
  not a liveness one. The off-chain leaf history the listener keeps for proofs is not bounded
  (about 370 bytes per leaf per coprocessor, never pruned); fhevm-internal#2060 tracks row
  shrinking and per-Store cold archival.
- General `HostConfig` config-version rotation semantics beyond the KMS-context pointer.
- Full production KMS-connector wiring and real ZKPoK and transciphering behind the input attestation
  (both are shortcuts today, DD-028).
- Production Yellowstone/Geyser and archive providers, and their replay windows and rate limits
  (DD-003, DD-059, fhevm-internal#2087).
- Historical handle discovery conventions for apps.
- Production role and governance names for public-decrypt and grant authority.
- Leaf-record availability (DD-048): the connector asks every configured coprocessor at once, and
  one behind, stalled or unreachable cannot sink or hold a request another can serve, but a Store first seen by a
  coprocessor through an update has no served proofs until that listener is replayed from before the
  Store's creation (`history_complete`). The replay and bootstrap policy is operational and
  undocumented beyond the listener's own flags.
- A Solana-native composition pattern for contract-to-contract confidential calls has not been
  designed since the receiver-callback flow was deleted (DD-011, in DESIGN_HISTORY.md).
- There is no per-Store cap on allows (RFC 035): allows are leaves, and the app-side wall is the
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
