# Solana PoC Design Decisions

This document is the stable rationale index for the Solana FHEVM PoC. It intentionally avoids
command logs and transient audit state. Use `DEVELOPMENT_ISSUES.md` and `DEVELOPMENT_LOGBOOK.md`
for implementation history; use this file to understand why the current design exists.

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

Status: adopted

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

Status: adopted

Context:

An ACL record can prove who may use or decrypt a handle. It does not prove that ciphertext material
is available, bound to the right key, or ready for KMS release.

Decision:

Use host-owned `HandleMaterialCommitment` accounts, committed by the configured material authority
for supported host-chain handles. Seal the material commitment pubkey, hash, and key id onto the ACL
record.

Rationale:

This lets KMS verify both authorization and decryptability without trusting app-local state or
events.

Consequences:

Public decrypt, certified disclosure, and burn redemption must verify the ACL record and material
commitment agree. Durable archival and compaction rules for ACL/material evidence remain
product-open.

## DD-007: External Inputs Bind Through A Verifier-Signed Intent

Status: product-open

Context:

The PoC needs a production-shaped encrypted input path, but real proof/transciphering validation
and threshold policy live outside the host program.

Decision:

`verify_input_and_bind` checks a native Ed25519 verifier pre-instruction over canonical
`SolanaInputProof` plus `SolanaInputBindIntent` bytes, then writes the selected handle into the
canonical ACL record named by the signed intent. `mock_input_verified_and_bind` remains test-only
glue.

Rationale:

The Solana host should verify account and transaction witness shape, but should not pretend to be
the external proof system. The signed bind intent gives the host an auditable bridge from verifier
policy to ACL birth.

Consequences:

Production still needs integration with the external input verifier service, threshold policy, and
real proof/transciphering validation.

## DD-008: Model Transient Allow As Explicit Solana Evidence

Status: adopted

Context:

EVM transient allowance uses transaction-local storage. Solana has no hidden transaction-local map
that a later instruction can read.

Decision:

Prefer, in order:

```text
1. instruction-local intermediates for expression graphs
2. signer propagation for CPI-chain authority over existing inputs
3. one-shot transient capability accounts for cross-instruction or cross-program handoff
```

Rationale:

Temporary permission must be explicit evidence on Solana. A one-shot capability is real state, not
"transient storage", and must not become durable ACL or decrypt authority by accident.

Consequences:

Transient capability consume must prove a matching earlier top-level creation instruction in the
same transaction. Same-slot expiry is defense in depth, not the transaction boundary. See
[`TRANSIENT_ALLOW.md`](TRANSIENT_ALLOW.md) for the detailed design.

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

## DD-011: Transfer-And-Call Uses Split Settlement

Status: product-open

Context:

The EVM-style single-instruction callback settlement does not map cleanly to Solana. SBF heap and
CPI/instruction-stack limits make deep token -> host -> receiver -> settlement flows expensive and
fragile.

Decision:

Use a split receiver-hook and callback-settlement flow. The hook returns an encrypted callback
success handle. Settlement verifies hook causality, recipient ACL binding, and replay markers, then
prepares and finalizes refund or transferred amount state.

Rationale:

The split design keeps Solana execution bounded and makes hook causality explicit instead of
assuming an EVM-like call stack.

Consequences:

The final transfer-and-call product/API shape remains open, but any production path must preserve
hook causality, replay protection, and recipient-signature-free refund relaying.

## DD-012: Native Solana KMS Flow Must Not Reuse EVM Routing

Status: product-open

Context:

The existing KMS Core client, protobuf, Gateway event enum, and tx-sender path are EVM-shaped.
Native Solana request and response rows carry different evidence: Solana account witnesses, replay
keys, request hashes, response route metadata, and certificate context.

Decision:

Keep Solana native-v0 request admission, storage, response verification, and tx-sender picking in
Solana-specific boundaries. Do not route verified native Solana response rows through the EVM
Gateway response sender.

Rationale:

Shared plumbing is useful only below the point where the data model is genuinely shared. Reusing
EVM publication paths for Solana rows would hide missing Solana-specific routing, status, replay,
and certificate checks.

Consequences:

The PoC has native-v0 parser/admission/store/response verification coverage, but production still
needs live KMS Core native work-item dispatch, frozen request/response encodings, and a concrete
native Solana response publisher.

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

## DD-014: Local-PoC Relaxations Are Chain-Id Confined

Status: adopted

Context:

Two relaxations exist for local testing: the zero birth-entropy fallback (used when the Mollusk
slot-hash sysvar is empty) and the `mock_input_verified_and_bind` short-circuit for the real signed
input-verifier path. Both were gated only by admin-toggled `HostConfig` flags
(`test_shims_enabled`, `mock_input_enabled`). `test_shims_enabled` also gates the `test_emit_*` event
shims, so enabling test events silently also re-opened the zero-entropy hole, and nothing bound either
relaxation to a non-production chain.

Decision:

Gate both relaxations at the consumption site on the local PoC chain id, via
`HostConfig::zero_birth_entropy_allowed()` (`test_shims_enabled && chain_id == SOLANA_POC_CHAIN_ID`)
and `HostConfig::mock_input_allowed()` (`mock_input_enabled && chain_id == SOLANA_POC_CHAIN_ID`). A
deployed (non-PoC) chain always takes the production branch — handle birth fails closed with
`PreviousBankHashUnavailable`, and the mock bind rejects — regardless of any admin flag.

Rationale:

The security boundary belongs at the point of use, not only at the setter, so the property holds even
if a future config path forgets a guard. Confining to the PoC sentinel chain id also decouples the
birth-entropy fallback from the `test_emit_*` shim concern on real chains: toggling test events can no
longer degrade entropy.

Consequences:

The host and app (`confidential-token`) derive the same gate from the same `HostConfig`, so
app-precomputed and host-verified handles stay in agreement. The `PreviousBankHashUnavailable`
negative test runs on the PoC chain with `test_shims_enabled = false` to exercise fail-closed birth.
`test_emit_*` event shims (no state mutation) are now also chain-confined via
`assert_test_shim_authority` (`test_shims_enabled && is_local_poc_chain()`), so an admin flag alone
cannot enable them on a deployed chain; they should still be compiled out for mainnet as defense in
depth.

## DD-015: Handle Birth Entropy Policy Remains Open

Status: product-open

Context:

Computed handles currently mix `previous_bank_hash` and `clock.unix_timestamp` into the digest, so the
same operation over the same inputs yields different handles across slots. Durable outputs are also
bound to `(output_nonce_key, output_nonce_sequence)` (DD-001), and the external 32-byte handle layout
is already fixed by version, chain id, FHE type, and computed-marker bytes.

Decision:

Keep the entropy-seeded derivation for now. Do not make deterministic handle birth part of the
current hardening plan. Any future native-v0 encoding freeze must decide the collision-resistance,
retry-idempotency, and KMS-reconstruction tradeoff explicitly.

Rationale:

Entropy helps avoid target collisions and preserves the current operational behavior. Switching now
would change every handle value and break test fixtures, client predictors, listener assumptions, and
any downstream material keyed by handle.

Consequences:

Handle byte layout remains stable, but handle birth is not idempotent across slots. The
`PreviousBankHashUnavailable` fail-closed surface remains part of the current design until a separate
handle-birth decision replaces it.

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
`fhe_rand_bounded_and_bind`, `verify_input_and_bind`, `mock_input_verified_and_bind` — plus one batched
eval instruction for composed plans: `fhe_eval`. The eval instruction accepts mixed binary/ternary,
trivial-encrypt, and rand steps with instruction-local transients; verified-input steps are
durable-output-only because input birth must bind ACL state immediately. It is the practical
successor to `execute_frame`. Every durable-output path takes a signer witness: either the fixed
`app_account_authority: Signer` account, or an explicit per-output authority account in
`remaining_accounts` that must be a signer and match `output_app_account`. The host then validates the
metadata with `assert_output_acl_metadata` (`instructions/common.rs`). This reinstates and now
enforces the signer the RFC had removed.

The OpenZeppelin-track `execute_frame` ABI is intentionally not ported as a host instruction. Its
useful ergonomic idea — symbolic previous results inside one instruction — is represented by
`FheEvalOperand::Transient` in the host ABI and by the app-facing `zama-fhe::EvalBuilder`. The SDK
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

## DD-018: Transfer-And-Call Refund Finalize Is Recoverable, Not Atomic With Prepare

Status: adopted (atomicity hardening product-open)

Context:

`confidential_prepare_transfer_callback` debits the recipient's refund and records a `PREPARED`
settlement; `confidential_finalize_transfer_callback` credits the sender from the durable
`refund_handle` snapshot and flips the settlement to `FINALIZED`. The two are separate instructions
with no same-transaction binding.

Decision:

Finalize does not require the recipient's live balance to still equal the prepare-time snapshot. It
credits the sender from the durable refund snapshot and is permissionless, so the credit is always
recoverable by anyone after prepare. An earlier `to.balance_handle == settlement.to_balance_handle`
guard was removed because it was unused by the credit math and would permanently strand the refund if
the recipient performed any balance op between prepare and finalize.

Rationale:

The refund is a sender credit; the recipient's balance after prepare is irrelevant to it. Pinning
settlement identity through the `(mint, sent_handle)` PDA seeds plus the `status` flip prevents double
finalize without coupling the credit to a frozen recipient balance.

Consequences:

Between prepare and finalize the system is in a temporary, recoverable imbalance (recipient debited,
sender not yet credited) rather than atomic. A future hardening may fuse prepare+finalize into one
instruction (CU/account budget permitting) or add an instructions-sysvar same-transaction binding; this
is tracked under Open Product Decisions.

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
success/debit values after the transaction. Burn and transfer-callback settlement flows still use their
own durable scratch records today and should be considered separately if their rent profile becomes a
product issue.

## Open Product Decisions

These are not settled by the PoC design decisions above:

- Whether to change handle birth entropy/idempotency policy at the native-v0 freeze (DD-015).
- Whether confidential balances move to the staged inbound-credit profile (DD-016).
- Fusing or same-transaction-binding transfer-and-call prepare/finalize (DD-018).
- Reclaiming rent for superseded durable `AclRecord` PDAs: there is no `close_acl_record` instruction
  today. Plain transfer scratch has been removed via DD-019, but burn/callback scratch and old balance
  history still need an archival/compaction policy if their rent profile becomes a product issue.
- Rejecting the PoC sentinel `chain_id` (`SOLANA_POC_CHAIN_ID = 12345`) in production builds. The
  relaxations already fail closed on every non-sentinel chain id; a compile-time `poc` cargo feature
  that both refuses 12345 at init and compiles out the mock/zero-entropy/`test_emit_*` paths would
  remove the residual misconfiguration risk entirely.
- `HostConfig` now stores the active input verifier set and version, with a one-time migration for
  legacy accounts. General config-version rotation semantics beyond verifier-set pointers remain
  product-open.
- Compiling out (not just chain-confining) the remaining `test_emit_*` shim for mainnet builds.

- External input verifier service integration, threshold policy, and real proof/transciphering.
- Live native-v0 KMS Core dispatch and protobuf/API shape.
- Concrete native Solana response publisher and relayer target.
- Durable archival and compaction policy for ACL, material, delegation, and replay evidence.
- Historical handle discovery conventions for apps and indexers.
- Production Yellowstone/Geyser provider, finality, replay, reconnect, and backfill policy.
- Production role and governance names for public decrypt and grant authority.
- Final transfer-and-call product/API shape.
- Transient-session SDK ergonomics.
- Frozen native-v0 request/response encodings and unsupported-version behavior.
