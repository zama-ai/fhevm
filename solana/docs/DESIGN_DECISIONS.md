# Solana PoC Design Decisions

This document is the stable rationale index for the Solana FHEVM PoC: why the current design exists.
Every statement here is true against the code on this branch. For the EVM mapping see
[`EVM_PARITY.md`](./EVM_PARITY.md); for forward requirements see [`FUTURE_DESIGN.md`](./FUTURE_DESIGN.md).

Status meanings:

```text
adopted
  The PoC relies on this design and tests should preserve it.

product-open
  The PoC has a clear direction, but production API, governance, service integration, or encoding
  details still need a final product decision.
```

## DD-001: Store Handles In ACL Records, Not PDA Seeds

Status: adopted

Context:

Solana accounts must be listed before instruction execution. FHEVM handles are opaque ciphertext
pointers and may be unpredictable before a compute operation finishes.

Decision:

Use app-controlled nonce metadata to derive ACL record addresses:

```text
nonce_key = H("zama-acl-nonce-key-v1", acl_domain_key, app_account, encrypted_value_label)
acl_record = PDA("acl-record", nonce_key, nonce_sequence)
```

Store the actual FHE handle inside the host-owned ACL record.

Rationale:

This lets an app prepare output ACL accounts before the transaction executes while preserving the
opacity of FHE handles. It also gives KMS and indexers a concrete account witness to verify instead
of requiring address derivation from secret or future data.

Consequences:

Historical decrypt requests must carry the observed ACL record. KMS does not guess, scan, or derive
ACL accounts from handles.

## DD-002: Keep App State And Host ACL State Separate

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
"is this subject allowed to use or decrypt this handle?"

Consequences:

KMS does not parse token state to authorize decrypts. Apps and indexers are responsible for current
or historical handle discovery, then KMS verifies the supplied host-owned witnesses.

## DD-003: Treat Events As Indexing Hints, Not Authorization

Status: adopted

Context:

EVM logs and contract state share one execution model. Solana log delivery is provider-dependent,
plain `emit!` logs can be truncated, and Anchor `emit_cpi!` adds nested CPI frames.

Decision:

Events are discovery and indexing signals. Production authorization must be rebuilt from
finalized or policy-approved transaction/account data and verified against host-owned ACL,
material, delegation, and replay witnesses.

Rationale:

Decrypt authorization cannot depend on whether a provider preserved a log line. It also cannot
require every production path to spend a self-CPI frame solely for observability.

Consequences:

The PoC can keep Anchor CPI events for tests and local listener compatibility, but production event
transport should use a Yellowstone/Geyser transaction and account stream with explicit finality,
reconnect, replay, and account-witness verification policy.

## DD-004: Account Metas And Witness Layouts Are ABI

Status: adopted

Context:

KMS verification depends on exact account shape. Accepting arbitrary extra accounts, malformed
unused slots, executable placeholders, or ambiguous optional accounts can create witness confusion.

Decision:

Instruction account lists, dynamic remaining accounts, optional accounts, inline ACL subjects,
overflow permission PDAs, and material witnesses are treated as ABI. Consumers reject trailing
metas, malformed unused fixed slots, duplicate subjects, invalid bumps, wrong lengths, and stale or
unsupported witness layouts.

Rationale:

The same account tuple must mean the same thing to the Solana program, listener, KMS verifier, and
tests. A loose ABI would let one layer accept evidence that another layer did not intend.

Consequences:

Negative tests are part of the contract. Changes to account layout must update program checks,
KMS witness decoders, fixture encoders, listener expectations, and docs together.

## DD-005: Public Decrypt Is A Post-Birth Release

Status: **superseded** by DD-032 (`allow_for_decryption` and the `AclRecord.public_decrypt` flag are
deleted; public release is now `make_handle_public`, an exact-handle `PublicDecryptLeaf` sealed into
the `EncryptedValue` MMR)

Status (superseded): adopted

Context:

Public decrypt is mutable authorization state. Letting handle-birth instructions create an ACL
record that is already public-decryptable bypasses the dedicated authority check and release event.

Decision:

Host-owned handle birth paths initialize `public_decrypt = false`. Releasing a handle for public
decrypt must go through `allow_for_decryption` after ACL birth.

Rationale:

This keeps the public-decrypt authority path explicit and auditable. It also separates ordinary ACL
membership from public release.

Consequences:

KMS public decrypt admission requires both sides:

```text
authorization state:
  acl_record.public_decrypt == true

decryptability state:
  material commitment exists, is committed, and is sealed onto the ACL record
```

## DD-006: Material Commitment Is Separate From ACL Authorization

Status: **superseded** by DD-031 (`HandleMaterialCommitment` deleted; materiality moved to the
gateway's `CiphertextCommits`)

Context:

An ACL record can prove who may use or decrypt a handle. It does not prove that ciphertext material
is available, bound to the right key, or ready for KMS release.

Decision (superseded):

Use host-owned `HandleMaterialCommitment` accounts, committed by the configured material authority
for supported host-chain handles. Seal the material commitment pubkey, hash, and key id onto the ACL
record.

Rationale (superseded):

This lets KMS verify both authorization and decryptability without trusting app-local state or
events.

Consequences (superseded):

Public decrypt, certified disclosure, and burn redemption must verify the ACL record and material
commitment agree. Durable archival and compaction rules for ACL/material evidence remain
product-open.

Why superseded: see DD-031.

## DD-007: External Inputs Verify Against An On-Chain secp256k1 Coprocessor Attestation (verify, not bind)

Status: adopted (June 2026 reconciliation — SUPERSEDES the earlier verifier-signed-intent design;
the verify-only refinement below SUPERSEDES the earlier "and-bind" ACL-creating shape)

Context:

The PoC needs a production-shaped encrypted input path. The earlier design (below) bound inputs
through a bespoke native Ed25519 "input verifier set" signing a `SolanaInputBindIntent`. That set
was a Solana-only trust root divorced from the EVM coprocessor trust model.

Decision:

Inputs enter compute through the `FheEvalOperand::VerifiedInput` operand consumed inside `fhe_eval`
— the Solana `FHE.fromExternal` analog. The operand carries the coprocessor's EIP-712
`CiphertextVerification` attestation; the shared `verify_input_attestation` verifier
(`zama_host::instructions::input_verification`) re-verifies it **in-frame** by recovering the EVM
coprocessor signer via `secp256k1_recover` and threshold-checking it against the configured signer
set, and asserts the attested `contract_chain_id` equals the host chain id (EVM's
`contractChainId == block.chainid`). On success the input is *transient-allowed for that eval only* —
there is no persistent `AclRecord` for the input, mirroring EVM `FHEVMExecutor.verifyInput` (verify ≠
allow). Solana has no transient-storage analog (DD-008); the transient allow lives only for the
duration of the frame.

**Binding model (the `contractAddress` analog).** EVM binds an attestation to a `contractAddress`.
The Solana equivalent is the consuming program's **compute-authority PDA** — a PDA the program signs
with via `invoke_signed`. In confidential-token this is the `[b"fhe-compute", mint]` compute signer.
It is never a user key and never the bare program id (program ids cannot sign). The host layer only
enforces `attestation.contract_address == compute_subject` (whatever signer consumes the input, the
msg.sender analog); the PDA convention is **app policy** — apps MUST bind attestations to their
compute-authority PDA, and MUST check the attested `user_address` themselves. Confidential-token
checks the attested user equals the token account owner. This mirrors EVM, where `userAddress` is
attested but the contract decides its meaning. Per-state-account (per-mint) scoping is deliberate and
finer-grained than EVM's per-contract binding.

**Derived outputs are NOT tainted by the input attestation.** Once verified, the input is an ordinary
operand; any *durable* ACL on an input-derived handle is the app's separate, explicit choice at
output-binding time — exactly EVM parity, where the input gets a transient allow and durable output
ACLs are the contract's decision. There is no output-taint from the input.

The gateway side is the RFC-021 bytes32 input path:
`InputVerification.verifyProofRequestSolana(contractChainId, bytes32 contractAddress,
bytes32 userAddress, ciphertextWithZKProof, extraData)` + `event VerifyProofRequestSolana`, which
shares the zkProofId counter and consensus state with the EVM path and stores the request in a
parallel `solanaZkProofInputs` mapping for bytes32 EIP-712 response validation. (Here `extraData`
is the coprocessor cert's EIP-712 `CiphertextVerification` extraData — NOT the `0x03` user-decrypt
auth blob; see DD-026.)

Why:

Reusing the coprocessor attestation makes Solana input trust identical to EVM input trust — one
trust root, recovered and threshold-checked on-chain — instead of a parallel verifier-set subsystem
that could drift. Consuming it as an in-frame eval operand (rather than a standalone verify + durable
receipt) restores EVM parity (verify ≠ allow) and removes a persistent ACL account per input — one of
the "3 ACLs" that inflated per-tx cost — so it is also a cost win.

What changed:

- The bespoke input verifier-set and the `verify_input_and_bind` Ed25519 path were REMOVED.
- Inputs are now the `FheEvalOperand::VerifiedInput` operand of `fhe_eval`. The earlier standalone
  `verify_coprocessor_input` instruction and its `InputVerifiedEvent` receipt were **deleted**, along
  with the short-lived output-taint binding (`VerifiedInputBinding` / output-ACL constraints): derived
  outputs are unconstrained by the input.
- The "caller is the attested contract" gate is enforced at input-consumption time
  (`attestation.contract_address == compute_subject`, the msg.sender analog).
- The `verify_input_and_bind` and standalone `mock_input_verified_and_bind` instructions were removed;
  the shared verifier `zama_host::eip712::verify_coprocessor_input` (via
  `instructions::input_verification::verify_input_attestation`) is invoked in-frame by `fhe_eval`.

Superseded design (stub): the earlier `verify_input_and_bind` bound inputs with a native Ed25519
"input verifier set" signing a `SolanaInputBindIntent`. Reversed because it was a Solana-only trust
root divorced from the EVM coprocessor; the coprocessor attestation is the canonical trust root.

Open for debate / follow-up: the input proof / ZKPoK / transciphering behind the attestation is still
a harness shortcut; real ZKPoK + transciphering is production work.

## DD-008: Model Transient Allow As Explicit Solana Evidence

Status: adopted

Context:

EVM transient allowance uses transaction-local storage. Solana has no hidden transaction-local map
that a later instruction can read.

Decision:

The lifetime model is `{AllowedLocal, AllowedDurable}`:

```text
AllowedLocal   an fhe_eval-local value (FheEvalOperand::AllowedLocal): produced by an earlier step,
               consumed by a later step in the same instruction. No AclRecord, no AclAllowedEvent.
AllowedDurable a durable ACL record (Output::durable()) bound with the nonce-key model (DD-001).
```

EVM `allowTransient(handle, account)` (tx-scoped) maps to `AllowedLocal` **plus CPI signer
propagation within one instruction's CPI tree**: the compute subject carried through the CPI chain
authorizes existing allowed inputs. A verified external input is transient-allowed for one eval this
way (DD-007). There is no mechanism to pass a not-yet-durable value across separate top-level
instructions — if a value must persist, it becomes `AllowedDurable`.

Rationale:

Solana has no hidden transaction-local map a later instruction can read; temporary permission must be
explicit. Keeping intermediates instruction-local avoids rent and prevents a temporary compute grant
from silently becoming durable ACL or decrypt authority.

Consequences:

The earlier persisted one-shot `TransientSession` / capability-account tier (a cross-instruction
handoff account with same-transaction creation proof) was **removed** (zama-ai/fhevm#2834): it was
real rent-bearing state that added a durable-ACL leak surface for no path the PoC needed. Any durable
output derived from transient inputs still passes explicit output-policy binding (output ACL domain,
app account, allowed roles) and is born with `public_decrypt = false`.

## DD-009: Operator Transfer Model Removed

Status: superseded

Context:

The earlier PoC mirrored ERC7984 operator/delegated transfer APIs with operator-scoped amount ACLs.
That improved parity, but it also added a second transfer authority model, operator PDA lifecycle
state, extra receiver-hook validation branches, and stale-approval/rent-cleanup cases.

Decision:

Remove the production operator model. Direct holder transfers use owner-scoped transfer amount ACLs,
and the transfer payer is only a rent/fee payer. `confidential_transfer_from`, operator rows, and
operator receiver-hook paths are intentionally absent from the production token API.

Rationale:

One transfer authority model is easier to audit and harder to misuse. Splitting `owner` from `payer`
keeps fee funding flexible without turning payer identity into transfer authority.

Consequences:

This is an intentional ERC7984 parity gap. Clients that need delegated spend must add a separate
product design instead of relying on hidden operator compatibility in the Solana token surface.

## DD-010: Token Disclosure Paths Are Label-Scoped

Status: adopted

Context:

Balances, total supply, transfer amounts, burn amounts, callback success flags, and refund amounts
have different app semantics even when they are all encrypted handles.

Decision:

`request_disclose_amount` and `disclose_amount` accept only token amount labels such as wrap,
transfer, burn, burned, transferred, and callback refund amounts. Current balances use the balance
disclosure path. Total-supply and callback-success handles are not accepted as generic token
amounts.

Rationale:

A generic amount API must not become a bypass around app-specific disclosure rules.

Consequences:

Disclosure fixtures and KMS tests must seed amount-shaped ACL records when testing amount
disclosure. Balance disclosure remains a separate path.

## DD-011: Transfer-And-Call Removed In Favor Of App-Driven CPI Composition

Status: superseded (issue #1593; supersedes DD-018)

Was: a ported multi-leg transfer-and-call callback flow (`confidential_transfer` →
`confidential_call_transfer_receiver` → `confidential_prepare_transfer_callback` →
`confidential_finalize_transfer_callback`) plus a `confidential-token-receiver` program + SDK.

Replaced because it transliterated an EVM workaround Solana doesn't need: a contract can't observe an
incoming transfer on EVM, so the token calls it back. On Solana signer authority propagates through
CPI, so a receiving app drives its **own** atomic `deposit` that CPIs `confidential_transfer` — the
user signs once, no operator, no callback, no refund leg (the all-or-zero transferred amount is the
accept signal). See the `confidential-deposit-app` reference program. Token-2022 transfer hooks were
rejected as a substitute: they are privilege-stripped veto tools, not receiver callbacks
(FUTURE_DESIGN §4). Some enum/error/event variants from the old flow remain inert to preserve Anchor
discriminants (FUTURE_DESIGN §6).

## DD-012: Solana User-Decrypt REUSES The Gateway/EVM Stack (REVERSED)

Status: adopted (June 2026 reconciliation — REVERSES the earlier "native Solana KMS flow must not
reuse EVM routing" decision). **Headline debate point.**

Context:

The earlier decision (below) deliberately kept a Solana-native KMS request/response model (`native-v0`)
out of the EVM Gateway routing, on the theory that the data models were too different to share. That
produced a large, separately-maintained native-v0 admission/store/response subsystem in the connector.

Decision:

Treat Solana as a **gateway-compatible host chain** and route its decrypt flows through the unified
Gateway V2 path (RFC-016) rather than a parallel native stack:

- **User-decrypt** flows through the unified Gateway V2 path, but the gateway is now **EXTENDED with a
  dedicated TYPED Solana entrypoint** — `userDecryptionRequestSolana(HandleEntry[],
  UserDecryptionRequestSolanaPayload)` + a `UserDecryptionRequestSolana` event — rather than smuggling
  Solana auth through `extraData`. The payload carries `bytes32 userIdentity`, `bytes32[]
  allowedAclDomainKeys`, `bytes32 nonce` as typed fields (plus shared publicKey, requestValidity,
  signature); `extraData` is context-only (`0x01 ‖ contextId`). A chain-aware validator branches on
  `contracts_chain_id` (see DD-027) so EVM stays strict and Solana is relaxed. The bytes32 handle
  surface still admits both EVM and Solana. (See DD-026 for the typed-vs-extraData history.)
- **Public-decrypt** certificates are verified **on-chain** via secp256k1: `zama_host` recovers EVM
  KMS signers from the cert and threshold-checks them, mirroring the EVM `KMSVerifier`
  (`verifyDecryptionEIP712KMSSignatures`). See DD-021.
- Solana is registered as a host chain (bytes32 ACL = the `zama_host` program id, high-bit chain id;
  added to the relayer `host_chains`).

Why:

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

Superseded design (stub): the earlier decision kept a Solana-native KMS request/response subsystem
(`native-v0`) out of EVM Gateway routing, on the theory the data models were too different to share.
Reversed to reuse one decrypt trust model and routing path. Some `native-v0` library/store code still
exists in the connector but is no longer the chosen path.

Open for debate: is unifying on the EVM/Gateway stack the right long-term call, or does a second
non-EVM chain eventually justify a native path? The KMS-connector decrypt is exercised in the harness,
not full production KMS wiring (DD-028).

## DD-013: Prefer Fail-Closed Chain Boundaries

Status: adopted

Context:

Solana handles and witnesses are not EVM contract calls. Accidentally applying EVM ACL checks to a
Solana handle would create a false sense of authorization.

Decision:

Connector paths with `chain_kind = "solana"` fail closed unless Solana-native witnesses are present
and accepted by the Solana verifier.

Rationale:

An unsupported chain path should be an explicit integration gap, not a permissive fallback.

Consequences:

Tests should cover both positive Solana witness acceptance and negative cases where EVM-shaped
checks are unavailable or inappropriate.

## DD-014: Local-PoC Relaxations Are Compile-Gated And Chain-Id Confined

Status: adopted

Context:

Local testing needs a zero birth-entropy fallback (used when the Mollusk slot-hash sysvar is empty)
and the `test_emit_*` event shims. Earlier these were gated only by admin-toggled `HostConfig` flags,
so enabling test events could silently re-open the zero-entropy hole, and nothing bound them to a
non-production chain.

Decision:

Two defenses stack. (1) The shim instructions (`test_emit_*`, `set_test_shims_enabled`,
`set_mock_input_enabled`) are `#[cfg(feature = "poc")]` — compiled out of default/production builds.
(2) The surviving state relaxation, the zero-entropy fallback, is gated at the consumption site by
`HostConfig::zero_birth_entropy_allowed()` (`test_shims_enabled && is_local_poc_chain()`, where
`is_local_poc_chain() = POC_FEATURE_ENABLED && chain_id == SOLANA_POC_CHAIN_ID`). A default build, or
any non-PoC chain id, always takes the production branch — handle birth fails closed with
`PreviousBankHashUnavailable`.

The former `mock_input_verified_and_bind` input short-circuit (and the never-consumed `mock_input_allowed`
gate) were **removed** — input mocking no longer exists; the real path is the in-frame secp256k1
attestation verify (DD-007). The vestigial `mock_input_enabled` flag on `HostConfig` remains only for
account-layout stability and should be dropped at the next ABI break.

Rationale:

The security boundary belongs at the point of use *and* is compiled out entirely by default, so the
property holds even if a future config path forgets a guard.

Consequences:

The host and app (`confidential-token`) derive the same gate from the same `HostConfig`, so
app-precomputed and host-verified handles stay in agreement. The `PreviousBankHashUnavailable`
negative test runs on the PoC chain with `test_shims_enabled = false` to exercise fail-closed birth.

## DD-015: Handle Birth Entropy Policy — RESOLVED: Keep The Entropy

Status: adopted (June 2026 reconciliation — RESOLVED; was product-open)

Context:

Computed handles are `bytes32`. ~88 bits are metadata (version, chain id, FHE type, computed marker),
leaving ~168 bits of keccak digest → roughly 2^84 birthday collision resistance. 2^84 is feasible to
grind offline for an extreme adversary. Computed handles therefore mix per-block entropy into the
digest (`previous_bank_hash` + `clock.unix_timestamp` on Solana). EVM does the identical thing via
`blockhash(block.number - 1)` (and `block.timestamp`) in `FHEVMExecutor._binaryOp` /
`_ternaryOp` / `_mulDivOp` / `_naryOp`. Durable outputs are additionally bound to
`(output_nonce_key, output_nonce_sequence)` (DD-001).

Decision:

Keep the per-block-entropy-seeded derivation. The alternative — widening `bytes32` → `bytes` (full
hash) to remove the collision concern without entropy — was rejected.

Why:

Per-block entropy denies an offline adversary the ability to grind a target collision: the block hash
isn't known until the block exists, so the 2^84 search cannot be done ahead of time. This is exactly
why EVM mixes `blockhash(block.number-1)`. The `bytes32 → bytes` alternative was rejected because it
roughly triples SSTORE/account-write cost and has no migration path for already-deployed handles.

What this means plainly (state it in the debate):

Handles are **block-bound and therefore reorg-unstable on EVERY chain** (EVM and Solana alike): a
resubmitted or reorged transaction over the same inputs yields a *different* handle. This is reconciled
by the listener's reorg handling on EVM (block-status machine, DD-025), and is an **open gap on
Solana** because the Solana poller is not yet wired into that substrate (DD-025, Boundaries).

Consequences:

Handle byte layout remains stable; handle birth is not idempotent across slots/blocks. The
`PreviousBankHashUnavailable` fail-closed surface (real chains) and the chain-id-confined zero-entropy
fallback (PoC chain only, DD-014) remain as designed.

## DD-016: Confidential Balances Use The Immediate-Available-Balance Profile

Status: product-open

Context:

`acl_storage_rationale.md` Part 5 describes two Solana token profiles: a staged inbound-credit profile
(recommended default for public-receivable tokens, where the recipient applies pending funds under
their own transaction timing) and an immediate available-balance profile (EVM-style, where the sender
updates the recipient's balance directly). The latter lets a sender force a write into the recipient's
balance handle and ACL record, which can invalidate a transaction the recipient already built against
their prior balance record.

Decision:

The PoC uses the immediate available-balance profile: `execute_transfer` credits the recipient by
rotating `to.balance_handle` / `to.balance_acl_record` and advancing `to.next_balance_nonce_sequence`
inside the sender's transaction, with no recipient participation in the base transfer.

Rationale:

It is the closest analog to ERC7984 `_update` and keeps the PoC's confidential-balance logic explicit
and EVM-parity-checkable. The stale-transaction / forced-inbound-write hazard is accepted for the PoC.

Consequences:

This is an explicitly accepted tradeoff, not the recommended production default. A production
public-receivable token should evaluate the staged inbound-credit profile (pending → available under
recipient timing) or otherwise predeclare/lock the recipient's next balance ACL sequence so the
inbound-write surface is bounded.

## DD-017: Role-Aware `fhe_eval` And Per-Op Bind Instructions Supersede The RFC-024 execute_frame Frame

Status: adopted

Context:

RFC-024 sketched one batched `execute_frame(authorized_app_accounts[], steps[], actions[])` entry
point and recorded removing an earlier `app_account_authority` signer that "was never validated by the
host." The implementation diverged from that sketch and the reversal was not previously recorded here.

Decision:

The host exposes per-handle-class binding instructions — `fhe_binary_op_and_bind_output`,
`fhe_ternary_op_and_bind_output`, `trivial_encrypt_and_bind`, `fhe_rand_and_bind`,
`fhe_rand_bounded_and_bind` — plus one batched eval instruction for composed plans: `fhe_eval`. The
eval instruction accepts mixed binary/ternary, trivial-encrypt, rand, and verified-input steps with
instruction-local transients. It is the practical successor to `execute_frame`. (Input birth is not a
separate instruction: external inputs enter through the `fhe_eval` `VerifiedInput` operand, DD-007.) Every durable-output path takes a signer witness: either the fixed
`app_account_authority: Signer` account, or an explicit per-output authority account in
`remaining_accounts` that must be a signer and match `output_app_account`. The host then validates the
metadata with `assert_output_acl_metadata` (`instructions/common.rs`). This reinstates and now
enforces the signer the RFC had removed.

The OpenZeppelin-track `execute_frame` ABI is intentionally not ported as a host instruction. Its
useful ergonomic idea — symbolic previous results inside one instruction — is represented by
`FheEvalOperand::AllowedLocal` in the host ABI and by the app-facing `zama-fhe::EvalBuilder`. The SDK
builder hides raw producer indices and `remaining_accounts` indices from app code, returns typed
`Encrypted<T>` values for intermediate results, derives durable output nonce keys / ACL record PDAs
from `DurableSlot`, stores ACL subjects behind `AccessPolicy`, and returns an opaque `EvalPlan`.
The `cpi` feature can resolve that plan through a pubkey-keyed account resolver, so app code does
not hand-maintain ordered host accounts. Output authority, role-aware ACLs, overflow permissions,
material commitments, and public-decrypt policy remain enforced by the current host ABI.

`fhe_eval` also owns its replay transport boundary. Frames with at most eight replay events use
Anchor event CPI for compatibility with existing event consumers. Larger frames emit the same replay
payloads through Anchor `Program data` logs to avoid self-CPI heap pressure. Durable ACL metadata
events remain log-only, and the listener rejects transactions that mix host CPI replay events with
host log replay events so DB log ordering stays unambiguous.

Rationale:

A validated `app_account_authority == output_app_account` signer makes the app account that receives
durable ACL output prove control via a Solana signature, rather than trusting an unsigned
`authorized_app_accounts[]` declaration. Per-output signer witnesses extend the same guarantee to
multi-app evals without making authorization a free-form unsigned list. Per-class instructions remain
for compatibility and individually testable handle-birth paths; `fhe_eval` provides batched multi-step
composition with transient/durable outputs when a single CPI is required.

Consequences:

This supersedes the older RFC-024 `execute_frame` sketch and its "app_account_authority removed"
note. Multi-account atomic effects (e.g. ERC7984 transfer crediting both sender and receiver) are
expressed as one eval frame with per-output authority witnesses rather than one frame with
`authorized_app_accounts[]`. Future multi-app eval extensions should keep that signer-witness model
and should not resurrect unsigned `authorized_app_accounts[]`.

## DD-018: Transfer-And-Call Refund Prepare/Finalize (superseded)

Status: superseded (with DD-011, issue #1593)

Was: the split refund legs (`confidential_prepare_transfer_callback` /
`confidential_finalize_transfer_callback`) of the transfer-and-call flow, with a recoverable
(non-atomic) sender credit from a durable refund snapshot. Removed with the whole callback flow — apps
now compose deposits by CPI (DD-011), so there is no refund leg.

## DD-019: Confidential Transfer Persists Only Final Balance And Transferred-Amount ACL Records

Status: adopted

Context:

A successful direct confidential transfer needs five FHE results: `ge(balance, amount)`,
`sub(balance, amount)`, `if_then_else(success, debit_candidate, balance)`, `sub(balance, new_from)`,
and `add(to_balance, transferred)`. The first implementation bound every result into a durable ACL
record because `fhe_eval` was binary-only and the ternary select needed durable inputs. That made one
plain transfer create five durable records, including two pure scratch records (`transfer_success` and
`debit_candidate`) that are not meaningful historical decrypt targets.

Decision:

The token transfer path now uses one host `fhe_eval` frame instead of the older scratch-account
sequence. The eval emits `ge` and debit-candidate `sub` as instruction-local transient handles,
consumes them in a ternary `if_then_else`, persists the sender's new balance plus the transferred
amount, and then credits the recipient in the same frame using a per-output recipient authority
witness. The helper crate exposes typed durable handles, scalar helpers, `DurableSlot`,
`AccessPolicy`, `EvalBuilder`, and plan-driven CPI resolution, so app code assembles this shape
without hand-maintaining raw producer indices, raw account indices, signer flags, writable flags,
nonce keys, ACL record addresses, or repeated output type bytes for common operations. A successful
direct transfer therefore binds exactly three durable ACL records:

- sender balance output
- transferred amount
- recipient balance output

The old `transfer_success` and `debit_candidate` PDAs are not created on transfer success; their handles
remain observable only through host FHE operation events for coprocessor/event replay.

Rationale:

Only the final sender balance, the transferred amount, and the final recipient balance need durable ACL
history for later permission checks or decryption. Persisting the boolean success bit and intermediate
debit candidate makes rent scale with scratch state, not product state. Keeping those values transient
avoids that rent cost without adding a close/refund path, while retaining the validated
`app_account_authority == output_app_account` signer rule for every durable output.

Consequences:

Indexers that replay transfer math must read the host `FheBinaryOpEvent` and `FheTernaryOpEvent` stream
for the scratch handles; there is intentionally no ACL permission record for decrypting the scratch
success/debit values after the transaction. The burn flow still uses its own durable scratch records
today and should be considered separately if its rent profile becomes a product issue.

## DD-020: VerifierSet Removed → Canonical KMS Context Singleton

Status: adopted (reconciliation)

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
`[KMS_CONTEXT_SEED, context_id.to_le_bytes()]`; `destroy_kms_context` exists for lifecycle). Decrypt
and disclosure witnesses pin the `kms_context_id` they were minted under.

Why / what worked:

Single source of truth, less divergence between a Solana-only set and the EVM KMS context. Invariant-
tested. A request pins its context id so a cert minted under context N cannot be replayed after rotation
to N+1.

Open for debate:

Context rotation governance (who may `define`/`destroy`, and the rotation choreography) is still
PoC-shaped.

## DD-021: On-Chain secp256k1 KMS Public-Decrypt Cert Verification

Status: adopted (reconciliation)

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
mirrors the EVM gateway `_extractContextId`: empty / version-0 `extra_data` selects the current context,
version 1 carries a big-endian context id in `extra_data[1..33]`.

Why / what worked:

Mirrors the EVM `KMSVerifier` so the same threshold cert verifies on both sides. Adversarial cases
(wrong threshold / wrong signer set / context mismatch — the "L4-b/c/d" harness rejections) are rejected
live.

Open for debate:

The harness exercises the KMS connector decrypt, not full production KMS-connector wiring (DD-028).

## DD-022: Witness PDAs Created Before The secp Consume (request → consume-once)

Status: adopted (reconciliation)

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

Request-before-consume gives a durable, replay-once witness with explicit expiry and pinned context.
This replaces the earlier "verify against `host_config.current_kms_context_id`" hazard where a cert for
context N could be consumed after rotation to N+1.

Open for debate:

Expiry slot policy and request-PDA rent reclamation cadence are PoC-shaped.

## DD-023: `fhe_eval` Composed Executor + Typed `EvalBuilder` DSL (DD-017 realized)

Status: adopted (reconciliation; cross-reference DD-017 / DD-019, do not duplicate)

Context:

DD-017 set the direction: a batched `fhe_eval` with instruction-local transients superseding the
RFC-024 `execute_frame` sketch. The reconciliation realized it end-to-end.

Decision:

`fhe_eval` is the composed-eval executor with steps **Binary / Ternary / TrivialEncrypt / Rand**.
External encrypted inputs are not a step type: they enter as the `FheEvalOperand::VerifiedInput`
operand of a step and are verified in-frame (DD-007). Intermediate results can be `Output::transient()`
(instruction-local, **no durable ACL record / no `AclAllowedEvent`**) and consumed by later steps; only
`Output::durable()` results bind an `EncryptedValue` account and its `current_handle`. The app-facing `zama-fhe` crate
(`solana/crates/zama-fhe`) exposes a typed `EvalBuilder` DSL returning `Encrypted<T>` for transients,
hiding raw producer/account indices, with a `cpi`-feature account resolver for plan execution.

Why:

Transient intermediates reduce durable PDA / rent footprint (a plain transfer binds 3 durable records,
not 5 — DD-019) while keeping per-output signer-witness authority (DD-017).

Open for debate:

`MAX_FHE_EVAL_OPS = 16` step cap, and the replay-event transport split (CPI ≤ 8 events vs log) — see
DD-024.

## DD-024: Finalized-Fetch Decrypt Trust Model (coprocessor side)

Status: adopted (reconciliation) — and see DD-025 for the OPEN finality-gate placement.

Context:

The host-listener must not trust unfinalized Solana event logs to release a handle for decryption
(reorg risk). A finalized re-read consumer existed but had been scaffolded, not connected.

Decision:

Rich ACL "allow" events schedule a **re-read of the on-chain ACL PDA at `finalized` commitment**. A
dedicated fetcher (`host-listener/src/bin/solana_finalized_account_fetcher.rs`,
`run_solana_finalized_account_fetcher`) polls `getMultipleAccounts` at `finalized`, and only a
confirmed, **`zama_host`-owned** account with a recognized allow reason releases the handle — inserting
`allowed_handle` + `pbs_computations` (→ SnS ct128 digest) in the same transaction as the witness store.

Why / what worked:

Anti-reorg: a handle is only released for decryption once the allowing ACL write is finalized. Finality
lags ~32 slots. Worked once the consumer was actually wired (it had been scaffolded but not connected).

Open for debate:

The RELEASE commitment level (finalized ~13s vs confirmed) — see DD-025.

## DD-025: WHERE The Finality Gate Sits (BIG OPEN debate item)

Status: OPEN — recommended direction recorded, not yet implemented.

Context:

The current Solana ingestion inserts computations **DORMANT** (`is_allowed = false`,
`is_completed = true`) and activates them per finalized-allow (`mark_solana_computation_allowed`). This
does **not compose with transient eval intermediates**: a `confidential_burn`'s burned-amount handle
depends on transient sub-handles that are never individually allowed, so a per-handle finalized-allow
gate can't activate the graph that produced the released handle.

Separately, the EVM reorg substrate already implements the recommended shape: a block-status machine
(`pending → finalized / orphaned` in the `host_chain_blocks_valid` table) plus ancestor catch-up in
`cmd/block_history.rs`. The **Solana poller (`bin/solana_host_listener.rs`) polls at `confirmed` and
inserts directly** — it is NOT wired into this substrate.

Options considered:

- **(A) Eager-materialize like EVM, and gate only the decrypt RELEASE on finality. [RECOMMENDED]**
  Reuses the existing EVM block-status substrate (option A's foundation already exists).
- (B) Keep the two-step dormant model + add transitive subgraph activation via a recursive CTE
  (activate the whole producing subgraph when the released handle is allowed).
- (C) Slot-level finality gate.
- (D) Ingest only at finalized (+~13s latency).

Also OPEN — the RELEASE commitment level: `finalized` (~13s, safe) vs `confirmed` (~1–2s, but a
confirmed-then-reorged release is an irreversible decrypt). Most Solana dapps run at `confirmed`.

Why this is open:

The dormant/activate model and transient eval intermediates were designed separately and don't compose;
the EVM substrate that would fix it (A) exists but isn't wired to Solana.

Open for debate:

Pick A/B/C/D and the release commitment level. (A) is recommended because the substrate is reusable.

## DD-026: Input / Identity Encoding (bytes32 non-EVM) and the Move To Typed User-Decrypt — RESOLVED

Status: adopted (reconciliation) — the user-decrypt `extraData` debate is now RESOLVED (typed gateway
fields).

Context:

The unified bytes32 input path must encode non-EVM (Solana) dapp/user identities. Separately, a Solana
*user-decrypt* request must carry ed25519 auth (user identity, nonce, allowed ACL-domain keys). These
are two DIFFERENT surfaces and the earlier docs conflated them — this DD disentangles them.

Decision:

**Input path (identities are bytes32; NO `0x03` blob):**

- Non-EVM bytes32 input via `InputVerification.verifyProofRequestSolana` + event
  `VerifyProofRequestSolana` (dapp/user are 32-byte host addresses; shares zkProofId + consensus with
  the EVM path; request stored in `solanaZkProofInputs` for bytes32 EIP-712 response validation).
- **Chain-id high bit (bit 63)** marks a non-EVM chain id (`SOLANA_CHAIN_TYPE_BIT = 1 << 63`; relayer
  `is_solana_host_chain_id`; the high bit survives into the chain-id word used in handle derivation).
- The input's `extraData` is the **coprocessor cert's EIP-712 `CiphertextVerification` extraData** — it
  is NOT, and never was, the `0x03` Solana user-decrypt blob. The input identity itself is a plain
  bytes32 host address (no version-byte blob).

**User-decrypt path (now TYPED — was the `0x03` blob):**

- PREVIOUSLY a Solana user-decrypt packed its ed25519 auth into an `extraData` blob with version byte
  `0x03` (`0x03 ‖ context_id(32) ‖ ed25519(32) ‖ nonce(32) ‖ key_count(4) ‖ keys`), forwarded opaquely
  through relayer/gateway and decoded by the KMS connector.
- NOW the gateway has a dedicated typed entrypoint `userDecryptionRequestSolana(HandleEntry[],
  UserDecryptionRequestSolanaPayload)` with a `UserDecryptionRequestSolana` event. The payload carries
  `bytes32 userIdentity`, `bytes32[] allowedAclDomainKeys`, `bytes32 nonce` as TYPED fields (plus shared
  publicKey, requestValidity, signature). `extraData` now carries ONLY the KMS context
  (`0x01 ‖ contextId(32)`). **No Solana auth data rides in `extraData` anywhere on the protocol or
  client surface.**
- The relayer builds the typed call (`SolanaUnifiedV1` core variant → `userDecryptionRequestSolanaCall`);
  the js-sdk `buildSolanaUserDecryptRequest` emits typed fields + context-only extraData (the signed
  ed25519 preimage is unchanged).
- INTERNAL connector transport detail: the KMS connector's gw-listener normalizes the typed event back
  into its existing internal `UserDecryptionV2` + `0x03` extraData representation at the decode boundary
  (the worker still routes to its Solana path on `extraData[0]==0x03`). This is internal to the
  connector; the gateway/protocol interface is typed.
- `Decryption.sol` version bumped MINOR 6→7 (reinitializer 7→8, reinitializeV6→V7).
- KMS-cert context: `extract_kms_context_id` (DD-021) handles `extra_data` versions 0 and 1 (the
  public-decrypt cert) — a *different* extraData from either path above.

Why:

A bytes32 identity + high-bit chain id keeps one input ABI for EVM and non-EVM hosts. For user-decrypt,
typed gateway fields make the Solana request a proper request type instead of an opaque blob, so the
protocol surface is self-describing and no longer overloads `extraData`.

Decision history (RESOLVED):

The 2026/06/12 Solana guild weekly (Manoranjith + Jad) objected that `extraData` was being misused to
smuggle Solana-specific data and should be a proper request type. RESOLVED by adding the typed
`userDecryptionRequestSolana` / `UserDecryptionRequestSolanaPayload` gateway entrypoint and reducing
`extraData` to context-only. The earlier uncertainty about whether `0x03` was an input or a user-decrypt
blob is settled: it was ALWAYS the user-decrypt auth blob (RFC-021), now removed from the wire in favor
of typed fields.

## DD-027: Chain-Aware V2 User-Decrypt Validation (didn't-work-then-fixed)

Status: adopted (reconciliation)

Context:

Admitting Solana over the unified V2 user-decrypt path (DD-012) required relaxing EVM input validation
(empty `contractAddresses`, 128-or-130-char signature).

What didn't work:

The reconciliation first relaxed this **unconditionally**, which weakened EVM — a CI integration test
caught empty-contracts / wrong-sig being accepted on the EVM path.

Decision / fix:

A **cross-field validator branches on `contracts_chain_id`** (via `is_solana_host_chain_id`, the bit-63
convention): EVM-strict (non-empty contracts, exact EIP-712 130-hex signature) vs Solana-relaxed (empty
contracts allowed, 128-or-130-char signature). Per-field validators stay permissive; strictness is
enforced in the cross-field branch.

Why / what worked:

Branching on the chain-type bit keeps EVM strictness intact while admitting Solana. The CI integration
test that caught the regression now passes for both.

Open for debate:

The Solana-relaxed signature acceptance (128 ed25519 vs 130) is the seam most likely to need tightening
once the input-identity encoding (DD-026) is frozen.

## DD-028: What This PoC Does NOT Do (explicit boundaries)

Status: adopted (reconciliation) — stated so the debate doesn't assume more than is built.

- **KMS connector decrypt** is exercised in the harness, **not** full production KMS-connector wiring.
- **Solana on-chain REORG handling is NOT wired** into the listener's block-status machine: the Solana
  poller (`bin/solana_host_listener.rs`) polls at `confirmed` and inserts directly, bypassing the EVM
  `host_chain_blocks_valid` / `block_history.rs` substrate. Reorg correctness is an **open gap** (DD-025).
- **Single local validator** in the harness — real reorgs / finality lag are not exercised end-to-end.
- **Input proof / transciphering** behind the coprocessor attestation is a PoC shortcut; real ZKPoK +
  transciphering is production work (DD-007).
- `mock_input_verified_and_bind`, `test_emit_*`, and the zero-birth-entropy fallback remain test-only
  and chain-id confined (DD-014); they should be compiled out for mainnet (Open Product Decisions).

## DD-029: `drift_revert` ≠ On-Chain Reorg (disambiguation)

Status: adopted (reconciliation) — code comments now cross-reference.

Context:

Two distinct "revert" notions were easy to conflate in the coprocessor.

Decision:

State them apart, explicitly:

- **`drift_revert`** = COPROCESSOR consensus: two coprocessors disagree on a ciphertext's bitwise
  representation. It **fires even on a chain that never reorgs** (`fhevm_engine_common::drift_revert`;
  consumer `check_if_drift_revert_is_over` / `latest_signal_for_chain`).
- **On-chain reorg** = the host chain orphaning an ingested block; handled by the listener's block-status
  machine / `cmd/block_history.rs`.

The discriminator now in the code comments: "would it fire on a chain that never reorgs?" — yes ⇒
`drift_revert`; only on an orphaned block ⇒ reorg.

Why:

They have different triggers, owners, and remedies; conflating them muddles both the reorg gap (DD-025)
and the consensus path.

## DD-030: Keep `verifyProofRequestSolana` (do NOT rename to V2) — DECIDED after debate

Status: adopted (reconciliation; an attempted rename was reverted)

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

Status: adopted — supersedes DD-006

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
gateway's `CiphertextCommits` for materiality and on the `EncryptedValue` MMR (DD-032) for
authorization. `HandleMaterialCommitmentWitness` is deleted from the KMS connector SDK.

## DD-032: `EncryptedValue` + MMR Replaces Keyed-Nonce `AclRecord` (RFC-024)

Status: adopted — supersedes DD-005's `AclRecord.public_decrypt` model and the keyed-nonce ACL shape
referenced throughout DD-004–DD-008

Context:

The original ACL model (RFC-024) minted a fresh, keyed-nonce `AclRecord` PDA per handle birth.
Superseding a handle meant superseding its ACL record's address, which complicated stable addressing
for indexers, apps, and historical decrypt (an old handle's authorization evidence disappeared once its
record was superseded).

Decision:

One stable, `zama-host`-owned `EncryptedValue` PDA per logical encrypted value (seeds
`["encrypted-value", value_key]`), reused across every handle update. A handle update supersedes the
previous handle by sealing one `HistoricalAccessLeaf` per allowed subject into an on-account SHA-256
Merkle Mountain Range (peaks + leaf count only — the MMR never stores the full leaf history
on-chain). The MVP ACL is a single allowed-subject set: `EncryptedValue.subjects` is the complete
authorization set, and the former parallel role byte vector is not part of the account layout. Any
allowed subject can use the current handle in compute, add another subject, request user decrypt, and
mark the exact current handle public. Public decrypt is an exact-handle `PublicDecryptLeaf`, so
publicness never survives a handle update (there is no live public flag to leak across updates — see
the connector-side rationale in the kms-connector/sdk commit message). New instructions:
`create_encrypted_value`, `allow_subjects`, `update_encrypted_value(new_handle, previous_handle,
previous_subjects)`, `make_handle_public`. Deleted:
`AclRecord`/`AclPermission` and their nonce-sequence machinery, the legacy single-op instructions
(`fhe_binary_op*`, `fhe_ternary_op*`, `fhe_rand*`, `trivial_encrypt_and_bind` — `fhe_eval` is now the
only compute path), and `allow_for_decryption`.

Rationale:

Stable addressing means indexers, apps, and CPI callers reference one PDA for a logical value's whole
lifetime instead of re-deriving a new one per birth. The MMR gives historical/public decrypt a
verifiable, compact (peaks-only) proof of past authorization state without keeping every past ACL
record alive. The `previous_handle`/`previous_subjects` args on `update_encrypted_value` are verified
against account state — redundant on-chain, but they make every transaction independently
interpretable, so indexers reconstruct MMR leaves statelessly from instruction data alone (see DD-033).
The shared `zama_solana_acl` crate (byte-identical MMR math and account codec) is the single source of
truth used by `zama-host`, the relayer proof service (DD-035), and the KMS connector, so host↔KMS
lockstep is type-level rather than a convention both sides have to keep in sync by hand.

No "RFC-024 option F" or similarly labeled alternatives-considered note was found in the commit history
or code comments for this specific redesign; RFC-024 itself is the ACL/EncryptedValue spec being
implemented here (a same-numbered but unrelated `execute_frame` batching sketch is referenced
separately in DD-017/DD-023 and is not this decision).

Consequences:

`fhe_eval` operand/output authorization now targets `EncryptedValue` accounts (canonical PDA +
`current_handle` + membership in `subjects`) instead of `AclRecord`. Confidential-token's per-rotation
balance address prediction (nonce counters, `balance_acl_record`) disappears —
`ConfidentialTokenAccount` now just points at one stable `balance_encrypted_value`.

Membership gates every decrypt-relevant surface consistently: `fhe_eval` operands, current-handle
user-decrypt authorization in the KMS connector, delegation-mediated user decrypt, subject grants,
and public leaf creation. Historical authorization is the sealed `HistoricalAccessLeaf` for the
subject at the time of supersession, not a later live-role lookup.

## DD-033: No ACL-Lifecycle Events — Self-Describing Args + Instruction-Replay Indexing

Status: adopted

Context:

The four `EncryptedValue` instructions could emit Anchor events (`emit!`/`emit_cpi!`) the way
compute-step events do, or stay event-free and let consumers decode instruction data instead.

Decision:

The four `EncryptedValue` instructions (`create_encrypted_value`, `allow_subjects`,
`update_encrypted_value`, `make_handle_public`) emit no Anchor events by design. The host-listener
(coprocessor) and the relayer's MMR proof service both decode them directly from transaction
instruction data — including inner/CPI instructions, since confidential-token and other app programs
invoke them via CPI — rather than subscribing to emitted events. `update_encrypted_value`'s
`previous_handle`/`previous_subjects` args are self-describing: verified against account state
on-chain (redundant there) specifically so every transaction is independently interpretable off-chain,
letting indexers reconstruct MMR leaves statelessly from instruction data alone, in replay order,
without reading account state first. Compute-step (`fhe_eval`) events are unchanged by this decision.

Rationale:

`DESIGN_DECISIONS.md` (DD-004 context) already notes that plain `emit!` logs can be truncated and
Anchor `emit_cpi!` adds nested CPI frames; avoiding events for a lifecycle that must survive CPI and be
replayed byte-for-byte from cold RPC history (the relayer proof service ingests from
`getSignaturesForAddress`/`getTransaction` alone, see DD-035) sidesteps both concerns for this
particular subsystem. No further code-comment rationale beyond this was found for the CPI-depth angle
specifically; `EVM_PARITY.md` separately notes `fhe_eval`'s own step batching is bounded partly to
limit CPI depth (DD-008), which is a related but distinct concern from why ACL lifecycle avoids events.

Consequences:

New coprocessor allow reasons: `encrypted_value_created`, `handle_superseded`, `handle_made_public`,
`subject_allowed`. `make_handle_public`'s leaf handle is derived from replay order alone (no event
payload to read it from). The host-listener's `solana_reconstruct.rs` decode arms and the relayer's
`solana_proof::decode` module both parse raw instruction data (Anchor discriminators + borsh args) for
these four instructions instead of dispatching on event types.

## DD-034: Eager Compute Scheduling For Solana (Q11 Option A)

Status: adopted

Context:

Under the old model, ACL "allow" signals gated whether the coprocessor would schedule an FHE
computation at all. With handles now living on stable, reusable `EncryptedValue` PDAs, that gate no
longer maps cleanly onto MMR-based historical authorization.

Decision:

Solana computations are inserted eager/schedulable immediately. Allow signals no longer gate FHE
computation scheduling — only decrypt availability. The finalized-account fetcher (re-reading
`EncryptedValue` accounts) remains the sole decrypt-release finality gate.

Rationale:

Reorg unwind stays unimplemented on the Solana listener path (DD-025/DD-028 note the same open gap for
block-status handling generally). Decrypt-release finality is the stated safety net for eager
scheduling: a minority-fork computation is wasted work — it can be scheduled and even executed
speculatively — but it is never released as a wrong decrypt, because decrypt release re-reads the
finalized account rather than trusting the eager schedule. No further rationale beyond this
wasted-work-not-wrong-decrypt argument was found in the commit history for why "Option A" specifically
(vs. gating scheduling on some other signal) was chosen over alternatives.

Consequences:

Coprocessor scheduling and decrypt availability are decoupled for Solana; a computation can run before
its ACL evidence is durably finalized, but nothing can be decrypted until the finalized
`EncryptedValue` account is re-read and agrees.

## DD-035: Relayer-Colocated, Untrusted Solana MMR Proof Service

Status: adopted

Context:

Historical and public decrypts need an MMR inclusion proof against `EncryptedValue`'s on-account
peaks. Building that proof requires replaying instruction history — work that belongs somewhere
between the chain and the KMS connector.

Decision:

Colocate the proof-builder with the relayer (`relayer/src/solana_proof`) rather than making it its own
service or putting it in the KMS connector. It ingests the four `zama-host` `EncryptedValue`
instructions by replaying plain RPC (`getSignaturesForAddress` + `getTransaction`, inner instructions
included) using ingestion/proof logic layered on the same `zama_solana_acl` crate `zama-host` uses
(DD-032), and exposes an interim internal HTTP endpoint (`GET /internal/solana/mmr-proof`) until the
Solana user-decrypt path lands and can call `build_proof` in-process instead.

Rationale (verbatim from the commit message): this service is in the same trust class as the relayer
already occupies — availability-critical, but never an authorization anchor. The KMS connector
re-verifies every proof against live finalized on-chain peaks (DD-032), so a bad or compromised proof
service can only cause a decrypt to fail, never to wrongly authorize one. Proof building cross-checks
its reconstructed peaks against the live finalized account and triggers targeted catch-up ingestion on
divergence before refusing a proof, rather than trusting its own replay blindly.

Consequences:

The relayer's own Solana user-decrypt flow does **not** yet call this proof service in-process — that
integration is a known gap; today it is reachable only over the interim internal HTTP endpoint, and
only once a deployment's `solana_proof` config section is present (`relayer/src/http/server.rs` mounts
it conditionally). The `FileLeafStore` backing it is a rebuildable cache: safe to delete, since a full
RPC re-replay reconstructs it from `start_signature`/`start_slot`.

## Open Product Decisions

Not settled by the decisions above. Forward requirements and the finality/reorg debate are detailed
in [`FUTURE_DESIGN.md`](./FUTURE_DESIGN.md); this list is the short index.

- Coprocessor input trust: single `coprocessor_signer` at threshold 1 → registered n-of-m set
  (FUTURE_DESIGN §1).
- Where the finality gate sits and the decrypt-release commitment level (DD-024/DD-025); wiring the
  Solana poller into the EVM reorg substrate (DD-028).
- Handle birth entropy/idempotency policy is RESOLVED (keep per-block entropy, DD-015); reorg-unstable
  handles are accepted on every chain.
- Whether confidential balances move to the staged inbound-credit profile (DD-016).
- Rejecting the PoC sentinel `chain_id` (`SOLANA_POC_CHAIN_ID = 12345`) unconditionally in production
  builds; the `poc` feature already compiles out the shims and confines the zero-entropy fallback.
- Rent/archival policy for the `EncryptedValue` MMR itself (DD-032): the account no longer needs
  per-supersession PDA closes (one stable PDA is reused for a lineage's whole life), but growth of
  `peaks`/`subjects` over a long-lived lineage's history still needs a compaction story if rent becomes
  a product issue.
- Dropping the vestigial `mock_input_enabled` flag and dead callback-flow enum variants at the next
  deliberate ABI break (FUTURE_DESIGN §6).
- General `HostConfig` config-version rotation semantics beyond the KMS-context pointer.
- Full production KMS-connector wiring and real ZKPoK / transciphering behind the input attestation
  (both PoC shortcuts today, DD-028).
- Production Yellowstone/Geyser provider finality/replay/reconnect/backfill policy (DD-003).
- Historical handle discovery conventions for apps and indexers.
- Production role and governance names for public-decrypt and grant authority.
- Reorg unwind is unimplemented on the Solana listener path (DD-028/DD-034); decrypt-release finality
  (re-reading `EncryptedValue` accounts) is the stated safety net today.
- `fhe_eval` has no `RandBounded` step; `create_random_bounded_amount`/bounded-rand eval were removed
  with the old model and are not yet rebuilt on `EncryptedValue` (DD-032).
- The relayer's own Solana user-decrypt path does not yet call the MMR proof service in-process
  (DD-035) — only the interim internal HTTP endpoint exists today. When it lands, the user-decrypt
  dedup hash must also cover the proof fields (`acl_value_key`, `proof_slot`, proof bytes).
- Proof-service high availability (DD-035): a single relayer-colocated instance with a file-backed
  rebuildable cache is the whole story today; replication/failover is unaddressed.
- The old 4-leg receiver-callback flow was deleted with the legacy model, but a Solana-native
  composition pattern for contract-to-contract confidential calls has not been designed to replace
  it.
- Subject-list overflow beyond `MAX_ENCRYPTED_VALUE_SUBJECTS` (8 subjects per `EncryptedValue`) is
  deferred; there is no overflow/paging account yet.
- Listener-core integration for the new event-free `EncryptedValue` instruction decoding (DD-033) is
  deferred; today decoding lives in the Solana-specific host-listener path.
