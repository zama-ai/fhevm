# Solana design history

Decisions the Solana port no longer follows, kept in their original wording so a reader can see why
a current decision looks the way it does. Each entry's status line names what replaced it; the
replacing entry in [`DESIGN_DECISIONS.md`](DESIGN_DECISIONS.md) is the one the code follows. The
vocabulary here predates [`GLOSSARY.md`](GLOSSARY.md) and is intentionally left as written. When a
later decision replaced only part of a live entry, the replaced wording is under
[Replaced parts of live decisions](#replaced-parts-of-live-decisions).

| Decision                                                                                                   | Replaced by                  |
| ---------------------------------------------------------------------------------------------------------- | ---------------------------- |
| DD-001 Store Handles In ACL Records, Not PDA Seeds                                                         | DD-032, then DD-049          |
| DD-005 Public Decrypt Is A Post-Creation Release                                                           | DD-032, then DD-049          |
| DD-006 Material Commitment Is Separate From ACL Authorization                                              | DD-031                       |
| DD-009 Operator Transfer Model Removed                                                                     | removed; fhevm-internal#1692 |
| DD-010 Token Disclosure Paths Are Label-Scoped                                                             | DD-040                       |
| DD-011 Transfer-And-Call Removed In Favor Of App-Driven CPI Composition                                    | DD-042 composition           |
| DD-017 Role-Aware `fhe_execute` And Per-Op Bind Instructions Replace The RFC-024 `execute_frame` Prototype | DD-023                       |
| DD-018 Transfer-And-Call Refund Prepare/Finalize (replaced)                                                | DD-011                       |
| DD-019 Confidential Transfer Persists Only Final Balance And Transferred-Amount ACL Records                | DD-049                       |
| DD-022 Witness PDAs Created Before The secp Consume (request → consume-once)                               | DD-040, DD-045               |
| DD-032 `EncryptedValue` + MMR Replaces Keyed-Nonce `AclRecord` (RFC-024)                                   | DD-049                       |
| DD-034 Eager Compute Scheduling For Solana (Q11 Option A)                                                  | DD-069                       |
| DD-035 Standalone Untrusted Solana MMR Proof Service                                                       | DD-048                       |
| DD-036 Burn-Redemption Consume Authorizes By MMR Public-Decrypt Proof, Not Live Handle                     | DD-045                       |
| DD-037 `fhe_execute` Events — `emit_cpi!`-Only, No `emit!` Log Fallback (DD-033 addendum)                  | DD-038                       |
| DD-038 One Host-Owned Born-Public Lifecycle Batch Replaces Per-Operation Events                            | removed; fhevm-internal#2079 |
| DD-039 HCU Block Cap Meters The Signed `compute_subject`, Not A Separate Authority                         | DD-047                       |

## DD-001: Store Handles In ACL Records, Not PDA Seeds

Status: **replaced** — the keyed-nonce `AclRecord` was replaced by the stable
`EncryptedValue` + MMR encrypted value account (DD-032), and both handle-binding components (the
`nonce_sequence` leaf-count and the encrypted value ID) were **deleted from
persistent-output handle derivation** (DD-015): a persistent output handle is now the
plain base handle, matching EVM `FHEVMExecutor` (no per-slot/per-caller/per-encrypted value account
binding). The encrypted value ID survives only as the `EncryptedValue` PDA seed. The
description below is retained for historical context only.

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

## DD-005: Public Decrypt Is A Post-Creation Release

Status: **replaced** by DD-032 (`allow_for_decryption` and the `AclRecord.public_decrypt` flag are
deleted; public release is now `make_handle_public`, an exact-handle `PublicDecryptLeaf` sealed into
the `EncryptedValue` MMR)

Status (replaced): adopted

Context:

Public decrypt is mutable authorization state. Letting handle-creation instructions create an ACL
record that is already public-decryptable bypasses the dedicated authority check and release event.

Decision:

Host-owned handle creation paths initialize `public_decrypt = false`. Releasing a handle for public
decrypt must go through `allow_for_decryption` after ACL creation.

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

Status: **replaced** by DD-031 (`HandleMaterialCommitment` deleted; materiality moved to the
gateway's `CiphertextCommits`)

Context:

An ACL record can prove who may use or decrypt a handle. It does not prove that ciphertext material
is available, bound to the right key, or ready for KMS release.

Decision (replaced):

Use host-owned `HandleMaterialCommitment` accounts, committed by the configured material authority
for supported host-chain handles. Seal the material commitment pubkey, hash, and key id onto the ACL
record.

Rationale (replaced):

This lets KMS verify both authorization and decryptability without trusting app-local state or
events.

Consequences (replaced):

Public decrypt, certified disclosure, and burn redemption must verify the ACL record and material
commitment agree. Persistent archival and compaction rules for ACL/material evidence remain
product-open.

Why replaced: see DD-031.

## DD-009: Operator Transfer Model Removed

Status: replaced

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

Note: the per-instruction label-scoping described here was dissolved in fhevm-internal#1704, PR 2 (see
DD-040). The `request_disclose_amount` / `disclose_amount` (and balance) instructions no longer exist;
disclosure is now the single generic `disclose_secp` consumer of the host `verify_public_decrypt`
verifier. In place of per-instruction label-scoping, the token binds the disclosed `EncryptedValue`
encrypted value account to the mint's scope.

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

Status: replaced (issue #1593; updates DD-018)

Was: a ported multi-phase transfer-and-call callback flow (`confidential_transfer` →
`confidential_call_transfer_receiver` → `confidential_prepare_transfer_callback` →
`confidential_finalize_transfer_callback`) plus a `confidential-token-receiver` program + SDK.

Replaced because it transliterated an EVM workaround Solana doesn't need: a contract can't observe an
incoming transfer on EVM, so the token calls it back. On Solana signer authority propagates through
CPI, so a receiving app drives its **own** atomic `deposit` that CPIs `confidential_transfer` — the
user signs once, no operator, no callback, no refund phase (the all-or-zero transferred amount is the
accept signal). See `confidential-batcher::join`, which evolved the `confidential-deposit-app`
reference this decision introduced. Token-2022 transfer hooks were
rejected as a substitute: they are privilege-stripped veto tools, not receiver callbacks
(FUTURE_DESIGN §4). Some enum/error/event variants from the old flow remain inert to preserve Anchor
discriminants (FUTURE_DESIGN §6).

## DD-017: Role-Aware `fhe_execute` And Per-Op Bind Instructions Replace The RFC-024 `execute_frame` Prototype

Status: superseded by DD-023.

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

## DD-018: Transfer-And-Call Refund Prepare/Finalize (replaced)

Status: replaced (with DD-011, issue #1593)

Was: the split refund phases (`confidential_prepare_transfer_callback` /
`confidential_finalize_transfer_callback`) of the transfer-and-call flow, with a recoverable
(non-atomic) sender credit from a persistent refund snapshot. Removed with the whole callback flow — apps
now compose deposits by CPI (DD-011), so there is no refund phase.

## DD-019: Confidential Transfer Persists Only Final Balance And Transferred-Amount ACL Records

Status: adopted

Context:

A successful direct confidential transfer needs five FHE results: `ge(balance, amount)`,
`sub(balance, amount)`, `if_then_else(success, debit_candidate, balance)`, `sub(balance, new_from)`,
and `add(to_balance, transferred)`. The first implementation bound every result into a persistent ACL
record because `fhe_execute` was binary-only and the ternary select needed persistent inputs. That made one
plain transfer create five persistent records, including two pure scratch records (`transfer_success` and
`debit_candidate`) that are not meaningful historical decrypt targets.

Decision:

The token transfer path now uses one host `fhe_execute` batch instead of the older scratch-account
sequence. The eval emits `ge` and debit-candidate `sub` as instruction-local transient handles,
consumes them in a ternary `if_then_else`, persists the sender's new balance plus the transferred
amount, and then credits the recipient in the same batch using a per-output recipient authority
witness. The helper crate exposes typed persistent handles, scalar helpers, `EncryptedValueKey`,
`FheExecutionBuilder`, and batch-driven CPI resolution, so app code assembles this shape
without hand-maintaining raw producer indices, raw account indices, signer flags, writable flags,
nonce keys, ACL record addresses, or repeated output type bytes for common operations. A successful
direct transfer therefore binds exactly three persistent ACL records:

- sender balance output
- transferred amount
- recipient balance output

The old `transfer_success` and `debit_candidate` PDAs are not created on transfer success; their handles
remain observable only through host FHE operation events for coprocessor/event replay.

Rationale:

Only the final sender balance, the transferred amount, and the final recipient balance need persistent ACL
history for later permission checks or decryption. Persisting the boolean success bit and intermediate
debit candidate makes rent scale with scratch state, not product state. Keeping those values transient
avoids that rent cost without adding a close/refund path, while retaining the validated
`app_account_authority == output_app_account` signer rule for every persistent output.

Consequences:

Indexers replay transfer math from the `fhe_execute` batch and Yellowstone-provided entropy; there is
intentionally no ACL permission record for decrypting the scratch success/debit values after the
transaction. The burn flow still uses its own persistent scratch records today and should be considered
separately if its rent profile becomes a product issue.

## DD-022: Witness PDAs Created Before The secp Consume (request → consume-once)

Status: superseded by DD-040 and DD-045.

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

## DD-032: `EncryptedValue` + MMR Replaces Keyed-Nonce `AclRecord` (RFC-024)

Status: adopted — updates DD-005's `AclRecord.public_decrypt` model and the keyed-nonce ACL shape
referenced throughout DD-004–DD-008

Context:

The original ACL model (RFC-024) minted a fresh, keyed-nonce `AclRecord` PDA per handle creation.
Updating a handle meant updating its ACL record's address, which complicated stable addressing
for indexers, apps, and historical decrypt (an old handle's authorization evidence disappeared once its
record was replaced).

Decision:

One stable, `zama-host`-owned `EncryptedValue` PDA per logical encrypted value (seeds
`["encrypted-value", encrypted_value_id]`), reused across every handle update. A handle update updates the
previous handle by sealing one `HistoricalAccessLeaf` per allowed subject into an on-account SHA-256
Merkle Mountain Range (peaks + leaf count only — the MMR never stores the full leaf history
on-chain). The MVP ACL is a single allowed-subject set: `EncryptedValue.subjects` is the complete
authorization set, and the former parallel role byte vector is not part of the account layout. Any
allowed subject can use the current handle in compute and request user decrypt. Subject-list
mutation (`allow_subjects` / `remove_subject`) and exact-handle public sealing are gated like
persistent create/update: the signer must equal
`EncryptedValue.encrypted_value_account_authority` (the app-owned account
identity). Decrypt subjects are not co-admins — apps that store a PDA in
`encrypted_value_account_authority` rotate auditors by CPI + `invoke_signed` as
that PDA. Confidential-token Wave 1 (#1862) covers token-account-scoped values through owner
wrappers and total-supply values through mint-authority wrappers.
Public decrypt is an exact-handle `PublicDecryptLeaf`, so
publicness never survives a handle update (there is no live public flag to leak across updates — see
the connector-side rationale in the kms-connector/sdk commit message). Active lifecycle changes are
performed by `fhe_execute` persistent outputs, `allow_subjects`, and `make_handle_public`; no instruction
accepts a caller-chosen handle, because such a handle would carry no proof of ciphertext
provenance (the former fail-closed `create_encrypted_value` / `update_encrypted_value` ABI stubs
are deleted). Deleted:
`AclRecord`/`AclPermission` and their nonce-sequence machinery, the legacy single-op instructions
(`fhe_binary_op*`, `fhe_ternary_op*`, `fhe_rand*`, `trivial_encrypt_and_bind` — `fhe_execute` is now the
only compute path), and `allow_for_decryption`.

Rationale:

Stable addressing means indexers, apps, and CPI callers reference one PDA for a logical value's whole
lifetime instead of re-deriving a new one per creation. The MMR gives historical/public decrypt a
verifiable, compact (peaks-only) proof of past authorization state without keeping every past ACL
record alive. The `previous_handle`/`previous_subjects` args on persistent `fhe_execute` outputs are
verified against account state — redundant on-chain, but they make every transaction independently
interpretable, so indexers reconstruct MMR leaves statelessly from instruction data alone (see DD-033).
The shared `zama_solana_acl` crate (byte-identical MMR math and account codec) is the single source of
truth used by `zama-host`, the solana-proof-service (DD-035), and the KMS connector, so host↔KMS
lockstep is type-level rather than a convention both sides have to keep in sync by hand.

No "RFC-024 option F" or similarly labeled alternatives-considered note was found in the commit history
or code comments for this specific redesign; RFC-024 itself is the ACL/EncryptedValue spec being
implemented here (a same-numbered but unrelated `execute_frame` batching sketch is referenced
separately in DD-017/DD-023 and is not this decision).

Consequences:

`fhe_execute` operand/output authorization now targets `EncryptedValue` accounts (canonical PDA +
`current_handle` + membership in `subjects`) instead of `AclRecord`. Confidential-token's per-update
balance address prediction (nonce counters, `balance_acl_record`) disappears —
`ConfidentialTokenAccount` now just points at one stable `balance_encrypted_value`.

Membership gates every decrypt-relevant surface consistently: `fhe_execute` operands, current-handle
user-decrypt authorization in the KMS connector, delegation-mediated user decrypt, subject grants,
and public leaf creation. Historical authorization is the sealed `HistoricalAccessLeaf` for the
subject at the time of update, not a later live-role lookup.

Amendment (fhevm-internal#1741): current membership is immutable by default but not frozen — a
persistent-output update may explicitly replace the subject set (`output_subjects` need not equal the
stored set). Order matters: the outgoing audience is sealed into historical leaves first, then
the new set replaces current membership, so past authorization stays exactly as sealed. Every subject an
update adds passes the grant deny-list exactly as `allow_subjects` does (so audience replacement is not a
deny-list bypass); `previous_handle`/`previous_subjects` still pin the outgoing state exactly, keeping
updates stateless-replayable (DD-033). This lets a per-sender encrypted value account (e.g. confidential-token's
`transferred_amount`) re-target its audience across recipients instead of reverting.

Amendment (RFC 035, DD-047/DD-048): the account no longer stores who may decrypt. Its seeds are
`["encrypted-value", program, encrypted_value_account_authority, scope, label]` — four fixed-width
fields after the tag, stored in the clear, no derived id — and its body is those four, the current
handle, `leaf_count`, the peaks and the bump (`181 + 32·peaks` bytes, at most 2229). The
`subjects` vector, `MAX_ENCRYPTED_VALUE_SUBJECTS`, `previous_subjects`, `allow_subjects`,
`remove_subject` and `authorize_current` are deleted. A write declares the keys allowed on the
handle it installs and the host seals one keccak `HistoricalAccessLeaf` per key, then the public
leaf when the output is `make_public`; a user decrypt of the current handle proves the same leaf a
historical one does. "Subject" left the vocabulary with the list (GLOSSARY.md).

## DD-034: Eager Compute Scheduling For Solana (Q11 Option A)

Status: replaced by DD-069. Only an output its transaction writes into an EncryptedStore is
allowed and recorded as a block producer, as on EVM. What follows is decision history.

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

## DD-035: Standalone Untrusted Solana MMR Proof Service

Status: superseded by DD-048 (RFC 035). The `solana-proof-service` workspace is deleted. The leaf
record lives in each coprocessor's host listener, derived in the same database transaction as the
compute rows and served over `POST /v1/solana/leaf-proofs` behind an API key; the KMS connector
fetches every proof from there and verifies it against the peaks it read on chain, and a
client-supplied proof is rejected. What follows is decision history.

Context:

Historical and public decrypts need an MMR inclusion proof against `EncryptedValue`'s on-account
peaks. Building that proof requires replaying instruction history — work that belongs somewhere
between the chain and the KMS connector.

Decision:

Serve proofs from the standalone `solana-proof-service` workspace (Yellowstone completed-block
ingest + PostgreSQL store + semantic `GET /internal/solana/access-proof` and
`/internal/solana/public-proof`) rather than colocating leaf ownership
inside the relayer. The service stays in the same trust class the relayer already occupies —
availability-critical, but never an authorization anchor. The KMS connector re-verifies every proof
against live confirmed on-chain peaks (DD-032), so a bad or compromised proof service can only cause
a decrypt to fail, never to wrongly authorize one. Proof building cross-checks reconstructed peaks
against the live confirmed account and fails closed on divergence (`lagging` / `corrupt_cache`).

Rationale: extracting MMR ownership keeps the relayer focused on the Solana v3 decrypt envelope
(ed25519 attestation, `0x03` extraData validation, gateway forwarding) while the proof service owns
persistent ingest/recovery. Clients discover proofs over the internal HTTP endpoint and embed them in
signed user-decrypt requests.

Consequences:

The relayer no longer mounts the internal Solana proof endpoints and has no leaf/checkpoint/proof DB. Solana
user-decrypt still does **not** call the proof service in-process — clients (the test-suite
scenario suite) fetch proofs via `PROOF_SERVICE_URL` before submitting to `/v3/user-decrypt`.
That optional in-process integration remains a known product gap.

## DD-036: Burn-Redemption Consume Authorizes By MMR Public-Decrypt Proof, Not Live Handle

Status: adopted, then amended by DD-045 — the live-handle check is back. The heading and the
Decision below record the original design. `redeem_burned_amount` now requires
`current_handle == burned_handle` as well as the public-decrypt proof and the certificate; the
`PendingBurn` account is what keeps the burned handle current. The survives-a-later-update
property this record secured now lives on the disclosure path, where `disclose_secp` never reads
`current_handle`. Read the DD-045 amendment below before quoting this record.

Context:

`burned_amount` is one stable `EncryptedValue` encrypted value account per token account (DD-019/DD-032),
replaced in place on every burn to that burn's own delta handle. The secp redeem path
(`redeem_burned_amount_secp`) required `current_handle == burned_handle`. That stranded funds: a
redemption requested against handle `H1` (still `PENDING`, awaiting the off-chain KMS round-trip)
becomes unredeemable the moment a second burn updates the encrypted value account to `H2` — the redeem reverts
forever even though `H1`'s public-decrypt leaf and KMS cert are still valid. The encrypted value account was reusing
one shared, in-place-replaced slot as an implicit "pending operation" record.

Decision:

The consume authorizes the _pinned_ handle by an MMR public-decrypt proof rather than by live-handle
equality. `redeem_burned_amount_secp` gains a `proof` argument and calls
`zama_solana_acl::authorize_public(encrypted_value_account, value, burned_handle, proof)` against the
encrypted value account's current peaks (the same primitive and leaf commitments the KMS connector re-verifies,
DD-032/DD-035). A redemption therefore stays valid after later burns update the encrypted value account. The
`request` path is unchanged — it still requires the live handle, because that is where the
public-decrypt leaf is appended (while the handle is current). Double-redeem is still prevented by the
per-handle `burn-redemption` marker PDA (DD-022), independent of the dropped equality check. The proof
rides in as `MmrInclusionProof`, an Anchor-native mirror of `zama_solana_acl::MmrProof` (the shared ACL
crate is deliberately Anchor-free, so it cannot carry Anchor IDL metadata).

Amendment (2026-08-05, DD-045): the concurrent historical-handle design and permanent marker in the
paragraph above are superseded. Each token account now has one `PendingBurn`, the burned handle must
remain current, and redeem or cancel closes that account. This deliberately trades same-account burn
concurrency for a smaller act-once state machine.

Historical rationale for the superseded concurrent design: the design-idiomaticity audit confirmed
the write model is single-writer-per-value (Solana's write-lock scheduler serializes `mut` access), so this is a
sequential cross-transaction TOCTOU, not a race. Per-operation escrow accounts would double-provision
history for no soundness gain; the MMR is the mechanism this system already built for exactly this
"prove a past/public state after update" need. This is the first on-chain consumer of the
`authorize_*` MMR API.

Addendum (Vector 2 closed — created-public eval output):

The second vector is now closed by making the burn's delta _born_ publicly decryptable inside the
same `fhe_execute` CPI that produces it, rather than by a separate `make_handle_public` CPI after the
eval. `FheExecuteOutput::StoredValue` gains a `make_public: bool` (carried in instruction data, like
`previous_handle`/`previous_subjects`, so indexers reconstruct the leaf without reading the account).
When set, `bind_eval_output` — after writing the new `current_handle` — appends a public-decrypt leaf
for that NEW handle using the exact same `public_decrypt_leaf_commitment` + `mmr_append` as
`make_handle_public` (byte-identical). Leaf order on an update-with-`make_public`: the outgoing
handle's historical-access leaves (one per current subject) FIRST, then the new handle's
public-decrypt leaf LAST; on a create-with-`make_public`, just the new handle's public-decrypt leaf.

This mirrors EVM `unwrap`'s `makePubliclyDecryptable(unwrapAmount)` happening inside the burn's own
state transition, and — critically — it drops the second CPI that overflowed Solana's fixed 32 KiB
bump heap on every update burn (the production OOM). `confidential_burn` sets `make_public: true`
on the burned-delta output only; balance/total-supply outputs stay `make_public: false`.
In the superseded request design, `request_burn_redemption` pinned a burned handle and appended no
leaf — the burn owned the public-decrypt leaf. Authorization: the output binder already authorizes the
encrypted value account authority to bind the output; that same authority is what authorizes making it public
(the binder is _creating_ the value), so no separate subject check is required — consistent with, and
gated by the same deny-list path as, the rest of the binding. This is the opt-in relaxation of the
"created encrypted value accounts cannot be created public-decryptable" invariant: it holds for all outputs except those
that explicitly set `make_public`.

Addendum (disclose consume — now the host verifier; replaced by DD-040):

The same live-handle TOCTOU existed on the disclosure consume path. It described the old
`disclose_amount_secp` / `disclose_balance_secp` instructions authorizing the witness-pinned handle
via `authorize_disclosed_handle`. That whole disclosure lifecycle was dissolved in fhevm-internal#1704,
PR 2 (DD-040): the disclose consume is now the single generic `disclose_secp`, which CPIs the stateless
host `verify_public_decrypt` verifier; `authorize_disclosed_handle` is deleted along with the rest of
the disclosure witness machinery.

The survives-update property this addendum secured is preserved — now one layer down, by the
host verifier itself. The public-decrypt leaf is sealed permanently (via `make_handle_public`) and the
KMS honors historical public leaves, so `verify_public_decrypt` authorizes the caller-pinned exact
handle by its MMR public-decrypt inclusion proof plus a KMS cert, never reading the live
`current_handle`. An OLD sealed handle therefore stays disclosable after its encrypted value account is replaced
during the off-chain KMS round-trip, matching EVM's permanent public-decryptability. This closes the
same TOCTOU (including balance mode's third-party griefability) without a witness.

The burn-redemption path was later moved to the stateless verifier and then bounded by the sequential
`PendingBurn` lifecycle in DD-045. The historical description above is retained as decision history,
not as the current settlement contract.

## DD-037: `fhe_execute` Events — `emit_cpi!`-Only, No `emit!` Log Fallback (DD-033 addendum)

Status: replaced by DD-038

Context:

DD-033 kept compute-step (`fhe_execute`) events while making the ACL lifecycle event-free. Those
compute events used a size-based transport switch: `emit_cpi!` for batches of `≤ MAX_CPI_EVAL_EVENTS`
(8) events, falling back to plain `emit!` logs for larger frames (the self-CPI frames would otherwise
overflow the 32KiB bump heap). Two consumers exist: the host-listener indexer and the standalone MMR
proof service (DD-035).

Decision:

Delete the `emit!` log-transport half. `fhe_execute` events are emitted **only** via `emit_cpi!`, and a
batch with more than `MAX_CPI_EVAL_EVENTS` events carries **no** on-chain event at all. To keep this
safe, a created-public (`make_public`, DD-036) persistent output is **rejected at write time** if its batch
is too large for CPI transport (`ZamaHostError::FheExecuteCreatedPublicFrameTooLarge`,
`assert_created_public_frame_transportable`), because a created-public handle is derived from block entropy
(DD-015) and lives in no instruction argument, so its `emit_cpi!` event is the only way an off-chain
proof builder can recover it. The host-listener runs reconstruction-only (Yellowstone gRPC, DD-003):
it never needed these events and derives every handle from instruction data + sysvar-streamed block
entropy. The `emit_cpi!` path and the proof service's op-event resolution are retained as a
transitional indexing ABI until Carbon/Geyser indexing fully owns created-public handle recovery
(fhevm-internal#1665).

Rationale:

No consumer reads `emit!` logs: the proof service / host-listener path reads only inner-instruction `emit_cpi!`
results, and a `> 8`-step created-public batch already failed closed (its handle was unresolvable). So
the log fallback was dead weight that also hid a latent stranding case; deleting it and adding the
fail-closed batch guard turns "silently unrecoverable later" into "rejected now." Non-created-public
persistent handles are unaffected — they reconstruct from the `fhe_execute` persistent-output arguments and
need no event. `emit_cpi!` cannot yet be removed entirely: `solana-proof-service` still resolves
created-public handles from the op-event (block entropy is not recoverable from instruction args alone),
so the op event remains its sole source of those handles until #1665's Carbon/Geyser ingestion
(with SlotHashes+Clock sysvar subscriptions and a historical-bankhash backfill policy) lands.

Consequences:

- `event_budget.rs` loses the log-byte budget machinery; it keeps `MAX_CPI_EVAL_EVENTS` (now a hard
  cap, not a transport switch), `eval_event_capacity`, and `should_emit_eval_events_as_cpi`, and gains
  `assert_created_public_frame_transportable`.
- `event_transport.rs` emits `emit_cpi!` only; oversized batches return without emitting.
- The `FheExecuteEventLogBudgetExceeded` error was renamed in place to `FheExecuteCreatedPublicFrameTooLarge`
  (same discriminant; no error-code shift). The variant was later deleted with the rest of the
  retired guard (fhevm-internal#1859 §3-D2).
- No IDL/wire change: `make_public` was already a `StoredValue` output field (DD-036); the guard adds a
  validation, not an argument.
- #1665 must remove the op event only after migrating created-public handle recovery off it — treat the
  event as an ABI surface whose last consumer must move first.

Note (RFC 035): the proof service that was the event's last consumer is gone (DD-048), and the
host listener's leaf record recomputes created-public handles from instruction data plus streamed
block entropy without reading it. `PublicOutputsProducedEvent` is still emitted (DD-044) and now
has no consumer in this repository; retiring it is #1665's call.

## DD-038: One Host-Owned Born-Public Lifecycle Batch Replaces Per-Operation Events

Status: removed; fhevm-internal#2079

Ordinary `fhe_execute` computation facts remain reconstructed from instruction data plus Yellowstone
sysvars. The host no longer produces the general per-operation event stream or its eight-event
transport guard. Instead, a batch with one or more `make_public` persistent outputs emits exactly one
versioned Anchor self-CPI event after successful execution. Its ordered records contain only the
zero-based step index, the host-owned Store, and the host-derived output handle;
a batch with no produced public output emits no lifecycle event.

This narrow batch exists because block-entropy output handles are absent from instruction arguments.
At the maximum `MAX_FHE_EXECUTION_STEPS` batch (32), the records serialize to one 2,133-byte CPI
instruction — far below the 10,240-byte CPI instruction-data cap — avoiding the old
one-CPI-per-step heap growth. (Execution, not the batch, bounds the all-created-public batch shape:
the host's fixed 32 KB `solana-program-entrypoint` bump heap fits 20 persistent creates per batch,
measured and pinned by the `fhe_execute_boundary/all_created_public` snapshot entry.) The event is unconditional, as every event this program
emits now is (DD-044). Consumers must still validate the host program, its canonical
event-authority PDA, transaction success, record ordering, and one-to-one agreement with persistent
`make_public` outputs; the event grants no authority by itself.

Removed (fhevm-internal#2079): the event had no reader. The host listener reads `make_public` from the
`fhe_execute` arguments and seals the public-decrypt leaf from instruction data, and a public
decryption is proven against the Store's on-chain peaks. The event cost a self-CPI and an
instruction-trace entry on every execution that made a value public.

## DD-039: HCU Block Cap Meters The Signed `compute_subject`, Not A Separate Authority

Status: adopted

The per-slot HCU block cap keys its meter and trust-registry PDAs (`["hcu-block-meter", subject]`,
`["hcu-trusted", subject]`) on the batch's `compute_subject` — the mandatory signed caller identity
already used for persistent-input ACL admission (the `msg.sender` analog). The earlier design metered a
dedicated `hcu_authority` signer supplied alongside `compute_subject`. That extra account bound the
meter to nothing the batch otherwise required: a direct caller could hand a fresh `hcu_authority`
keypair on every call and receive a fresh per-slot meter each time, so any finite
`hcu_block_cap_per_app` was bypassable by signer rotation. The gap was dormant only because
`initialize_host_config` ships the cap at `u64::MAX` (unrestricted), which short-circuits before the
meter is consulted.

Metering the `compute_subject` closes the rotation bypass wherever the batch binds that identity to
something the caller cannot freely re-pick:

- The confidential-token program: `compute_subject` is the mint's `["fhe-compute", mint]` PDA, signed
  only via the program's CPI seeds, so a caller cannot swap it for a fresh key — the per-mint meter is
  unforgeable.
- Any batch consuming a persistent or verified input: the subject must be an allowed member of the input
  encrypted value account's ACL (persistent), or match the attestation's bound contract (verified), so substituting an
  unrelated key loses input access. No other account the caller controls (`payer`,
  `app_account_authority`, output authorities) yields a fresh meter.

Metering the `compute_subject` does NOT, by itself, constrain a **persist-nothing batch** — `Rand` /
`TrivialEncrypt` / scalar-only work with a transient (non-persistent) output and no verified input —
submitted by a direct caller: there `compute_subject` is an unconstrained signer, so such a caller
could still rotate it for a fresh per-slot meter. The value-less part of that residual case
(fhevm-internal#1744) is now closed: when `hcu_block_cap_per_app` is finite (`!= u64::MAX`),
`fhe_execute` preflight rejects any batch that binds no `StoredValue` operand, no `VerifiedInput`
operand, and no `StoredValue` output (`FheExecuteUnanchoredUnderBlockCap`). Such a batch persists
nothing and verifies nothing — `compute_subject` is a free variable and the batch is also value-less
(its transient outputs create no ACL leaf and are undecryptable) — so nothing legitimate is
forbidden. Under the ship default (`u64::MAX`) the check short-circuits, so behavior is unchanged
where no finite cap is deployed. This never affected the token program, whose compute subject is
always the mint PDA and whose batches always bind persistent balance inputs.

This is #1744's Option 1 broadened by a persistent-output allowance to preserve the legitimate
trivial-encrypt/`Rand` -> persistent-output bootstrap/mint path. That allowance is not a full close: a
persistent output does NOT pin `compute_subject` (output binding authorizes against
`app_account_authority`, never the subject), so a caller can still rotate the subject while binding a
throwaway output encrypted value account and get a fresh per-slot meter each time. That vector remains open but is
now rent-bounded — each rotation costs ~one `HcuBlockMeter` PDA rent rather than being free —
whereas the persist-nothing rotation was free. Closing it fully requires a host-registered app
identity an input-free batch must present to be metered (#1708 Option B / #1744 Option 2), still
deferred as speculative until input-free batches become a real metered workload.

`hcu_authority` is removed everywhere: the host `fhe_execute` account list (an intended ABI break,
resynced in the host-listener IDL), the `zama-fhe` CPI account struct, the confidential-token
`HcuAuthority` PDA and the account slot on all six token instructions, and the deposit app's
forwarded account. The token program now meters per mint automatically: `compute_subject` is that
mint's `["fhe-compute", mint]` compute-signer PDA, so the budget stays one-per-mint with one fewer
account threaded through every instruction.

Granularity note: because the meter keys on the compute subject, an application that spreads work
across several distinct compute subjects gets a separate per-slot budget per subject. Aggregating a
finite cap across many subjects under one logical app would need an on-chain app→subject registry;
that is rejected here as speculative (see #1708 Option B) until a concrete multi-subject app requires
it. The trust registry (`set_hcu_app_trusted`) already lets an admin bypass the cap for a specific
subject, which covers the known trusted-app cases without a registry.

Amendment (RFC 035, DD-047): `compute_subject` is deleted. The meter and trust records key on the
application `(program, scope)` — `["hcu-block-meter", program, scope]`, `["hcu-trusted", program,
scope]` — where `program` is proven on every write from the output authority's seeds and `scope`
is what that program declares. That closes the rotation vectors above **for a caller outside the
program**: a fresh keypair is not a PDA of any program, so it cannot be an output authority, and a
caller cannot mint applications under a program it does not control. It does not bound the program
itself. A program declares its own `scope`, so it can create any number of scopes and hold a
separate per-slot meter for each, at the cost of one `HcuBlockMeter` rent per scope. The
granularity note above therefore still stands, with `(program, scope)` in place of the identity it
used to key on: a finite cap binds one application only as far as that application declines to
spread itself across scopes. See DD-047's consequences and INVARIANTS #41.
Every output the default authority controls must
share one application (`FheExecuteMixedScopes`); an output under an additional signing authority
is metered as that execution's but deny-checked as its own. The persist-nothing residual keeps its guard: under a finite
block cap an execution that binds no stored operand and no persistent output has no application to
meter and is rejected (`FheExecuteUnanchoredUnderBlockCap`). The "registry Option B" this section
deferred had two halves, and the verified program answers one of them: an application identity a
caller cannot forge, with the program as its own registry entry. The other half, aggregating one
finite cap across the several identities a single application may hold, is still not built.

## Replaced parts of live decisions

A live entry in [`DESIGN_DECISIONS.md`](DESIGN_DECISIONS.md) states only its current rule. When a
later decision replaces part of it, the replaced wording is kept here, under the entry's number.

### DD-003, replaced in part by DD-031 and DD-040

Superseded in part by DD-031 and DD-040: the host keeps no material or replay witnesses, so
authorization is verified against the host's Store, delegation and KMS context accounts.

Decision, as first written:

Events are discovery and indexing signals. Production authorization must be rebuilt from
policy-approved transaction/account data and verified against host-owned ACL,
material, delegation, and replay witnesses.

### DD-007, replaced in part by DD-023

The input path was rebuilt around the `fhe_execute` operand. The change record and the replaced
design, as first written:

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

### DD-008, replaced in part by DD-050

DD-050's transaction-wide transient store holds intermediates across calls. Rationale and
Consequences, as first written:

Solana has no hidden transaction-local map a later instruction can read; temporary permission must be
explicit. Keeping intermediates instruction-local avoids rent and prevents a temporary compute grant
from silently becoming persistent ACL or decrypt authority.

The earlier persisted one-shot `TransientSession` / capability-account tier (a cross-instruction
handoff account with same-transaction creation proof) was **removed** (zama-ai/fhevm#2834): it was
real rent-bearing state that added a permission leak surface for no path the port needed. A Store
output derived from transient inputs still passes its authority check and declares its own allows;
nothing is public unless the output says so.

### DD-015, replaced in part by DD-043 and DD-050

Superseded in part by DD-043: deterministic handle preimages carry neither `context_id` nor
`op_index`, and `op_index` remains only in the rand seed. DD-050 adds the origin mask. The current
preimage is DD-043's.

Context, as first written:

outputs — no per-output binding. The former persistent-output binding (the per-value account ID, plus an
even earlier per-update `output_nonce_sequence` = that account's MMR `leaf_count` read at execution)
was **removed** entirely — see "Binding removal" below.

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

### DD-020, replaced in part by DD-040

Superseded in part by DD-040: no witness or request pins a context any more. A certificate is
accepted from any live context it names, and `destroy_kms_context` is the revocation lever.

Decision, as first written:

The VerifierSet subsystem was REMOVED. Witnesses and decrypt trust anchor to a `define_kms_context`
singleton keyed by `kms_context_id` (`zama_host::kms_context_address(context_id)`, seed
`[KMS_CONTEXT_SEED, context_id]` with a 32-byte id; `destroy_kms_context` exists for lifecycle). Decrypt
and disclosure witnesses pin the `kms_context_id` they were minted under.

Single source of truth, less divergence between a Solana-only set and the EVM KMS context. Invariant-
tested. A request pins its context id so a cert minted under context N cannot be replayed after rotation
to N+1.

### DD-021, replaced in part by DD-040 and DD-065

Superseded in part by DD-040 and DD-065: there is no witness or request context. `verify_public_decrypt`
verifies the certificate against the live `KmsContext` that its `extra_data` names and returns that
context id, so a caller can demand the current one.

Decision, as first written:

`zama_host::eip712::verify_kms_public_decrypt` recovers secp256k1 EVM signers from the cert
(`recover_evm_address`), requires a **distinct-signer threshold** (`verify_threshold`) against the
**witness-pinned `kms_context`'s** signer set / threshold (not the current context), **rejects high-s
(malleable) signatures** (`signature[32..64] > SECP256K1_HALF_ORDER`), and requires
`extract_kms_context_id(extra_data, current) == request kms_context_id`. `extract_kms_context_id` …

### DD-024, replaced in part by DD-070

DD-070 moved instruction reconstruction to `finalized`, so the rationale lost its finalization delay
and its rolled-back computation.

> Confirmed instruction reconstruction emits concrete material requests at handle creation and Store
> update.

> This removes the finalization delay and duplicate Solana RPC read. A rolled-back computation can waste
> work, but prepared ciphertext material is not authorization and cannot cause plaintext release.

### DD-025, replaced in part by DD-070

DD-070 moved the listener's ingest and the KMS connector's reads to `finalized`. This was the
confirmed-commitment rule and its accepted risk.

> Confirmed, eager materialization; live KMS authorization at release time.

> - (A) Eager-materialize and gate decrypt release on finality. Rejected: prepared material is not an
>   authorization, and the accepted confirmed authorization may release plaintext.
> - (D) Ingest only at finalized (+~13s latency).
>
> The accepted design is eager materialization from confirmed instruction reconstruction with no
> separate finality gate; KMS revalidates confirmed authorization at the plaintext-release boundary.
>
> The accepted product rule treats a valid confirmed authorization as sufficient. Coprocessor work is
> therefore scheduled from confirmed ingestion. The KMS connector's ACL read and the host listener's
> confirmed Yellowstone ingest use explicit confirmed commitment; KMS remains the only
> plaintext-release boundary.
>
> Decision provenance: accepted by the Solana feature owner during the review of
> [`zama-ai/fhevm#3122`](https://github.com/zama-ai/fhevm/pull/3122) on 2026-07-13. The accepted trade-off
> is irreversible plaintext release after a valid authorization observed on an exceptionally rolled-back
> confirmed fork; subsequent on-chain actions still follow the surviving fork.
>
> A finality gate adds latency without strengthening the chosen authorization rule: an allowed
> key authorized in confirmed state was legitimately allowed to receive that plaintext, even if the fork
> later rolls back.
>
> Open for debate:
>
> Reorg unwind may still be added for resource recovery, but is not an authorization dependency.

### DD-026, replaced in part by DD-052

The user-decrypt `extraData` debate is resolved by typed gateway fields. The chain-type marker is superseded by DD-052.

Earlier text of the decision:

- The input's `extraData` is the **coprocessor cert's EIP-712 `CiphertextVerification` extraData** — it
  is NOT, and never was, the `0x03` Solana user-decrypt blob. The input identity itself is a plain
  bytes32 host address (no version-byte blob).

- PREVIOUSLY a Solana user-decrypt packed its ed25519 auth into an `extraData` blob with version byte
  `0x03` (`0x03 ‖ context_id(32) ‖ ed25519(32) ‖ nonce(32) ‖ key_count(4) ‖ keys`), forwarded opaquely
  through relayer/gateway and decoded by the KMS connector.

Decision history:

The 2026/06/12 Solana guild weekly (Manoranjith + Jad) objected that identity and authorization scope
were being smuggled through `extraData` and should be a proper request type. A dedicated typed
entrypoint (`userDecryptionRequestSolana`) resolved that first. `solanaUserDecryptionRequest` and
its versioned blob replaced it, and `extraData` carries no Solana identity, scope or proof data. It
is a named entry rather than an overload of `userDecryptionRequest`, so the EVM entries keep their
generated binding names.

### DD-027, replaced in part by DD-052

The chain-type detector is superseded by DD-052.

What didn't work:

The reconciliation first relaxed this **unconditionally**, which weakened EVM — a CI integration test
caught empty-contracts / wrong-sig being accepted on the EVM path.

Branching on the chain type keeps EVM strictness intact while admitting Solana. The CI integration
test that caught the regression now passes for both. This entry only keeps that split.

### DD-033, replaced in part by DD-066

DD-066 moved the leaf reconstruction from the host listener to the Merkle indexer.

> The host listener reconstructs compute requests and MMR leaves from confirmed Yellowstone
> transaction instructions, […] and the listener reconstructs leaves from instruction data alone, in
> replay order, without reading account state first.

### DD-040, replaced in part by DD-045 and DD-065

Superseded in part by DD-065: the verifier takes no MMR inclusion proof and no Store; it reads only
`host_config` and `kms_context`.

Decision, as first written:

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

Earlier revisions of this DD verified against the CURRENT context
only and framed a cert-after-rotation as a hazard to fail closed on; that framing is replaced here —
rotation is no longer the revocation boundary, `destroy` is.

Return-data-only to start: today's KMS cleartexts are ≤32 bytes; if larger types are ever revealed the
fallback is a caller-provided scratch account. The proof-freshness (stale-proof) retry race is the
known bounded-retry surface (#1687): an update between proof generation and consume moves the MMR
peaks and fails the inclusion proof; the victim regenerates the proof and retries. The one wrong app
pattern is binding consume logic to the live `current_handle` instead of the sealed handle — the
sealed leaf is append-only, so the OLD sealed handle stays verifiable after an update (covered
today by `mollusk_historical_proof_round_trip_after_two_updates`).

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
cleartext_amount, signatures, extra_data)`. It CPIs `zama_host::verify_public_decrypt`, asserts the
certified handle equals the `burned_handle` pinned in `PendingBurn` and the certified cleartext
equals `cleartext_amount`, requires the burned Store's current handle to be that handle, then pays
out and closes `PendingBurn` (DD-045, DD-065). Every field the witness pinned is carried elsewhere
(destination integrity by the redeem-time signer check, handle binding by `PendingBurn`, owner and
mint by the Store), so the witness was pure scaffolding.

The stateless verifier replaces the request-time KMS pin: the cert is verified against the context it
names inside the verifier, not the witness's pinned `kms_context_id`. (This note originally said the
verifier used `host_config.current_kms_context_id` and failed closed on rotation; replaced by
fhevm-internal#1765, which accepts any live context and makes `destroy_kms_context` the revocation
lever — see "Any live context" above.)

### DD-041, replaced in part by DD-065

Status: adopted

Superseded in part by DD-065: public-decrypt consume transactions carry no MMR proof; their sizes
are in `runtime-tests/tests/disclose_packet_fit.rs`.

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
`HostConfig::SPACE` is 320, after the 32-byte KMS context id and the DD-058 pause flags) and
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

### DD-044, replaced in part by DD-048, DD-056, DD-058 and DD-061

Status: adopted

Revised by DD-056: `fhe_execute` also emits, one event per execution carrying what the host decided.
The rule below, that only administration emits, no longer covers that event.
DD-058 adds `pause`, `unpause` and `set_pauser`, so the instruction and event counts below are out of
date, and zama-host has build features again (`admin-sweep`, `cleartext`). The delegation facts
recorded under Decision no longer hold: the connector fetches and checks the delegation record on a
delegated decrypt (DD-048, DD-061; INVARIANTS #27).

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

### DD-045, replaced in part by DD-048, DD-049 and DD-065

Superseded in part by DD-065: settlement and disclosure take no proof, and disclosure reads no token
state.

Superseded in part by DD-048 and DD-049: allows are sealed on the write and the Store model replaces per-value accounts.

Earlier text:

`ConfidentialBurnEvent` intentionally does not duplicate the MMR `leaf_index`. Settlement binds the
pending account to the burned Store and handle; the supplied proof carries its own leaf index and
is checked against live peaks. The connector obtains that proof by handle from the coprocessors'
leaf record, so an event index would be redundant rather than an authorization input.

Disclosure names a token state kind and validates its entire binding before emitting: mint scope,
canonical Store, Store authority, slot key and handle proof. Scope-only validation was rejected
because two fields within the same mint would remain interchangeable in downstream events.

### DD-048, replaced in part by DD-049

DD-049 replaced the per-value account and its `PersistentOutput` API with the shared
`EncryptedStore` and its Store effects.

> 1. **Allows are sealed on the write.** A persistent output declares the keys allowed on the handle
>    it installs (`PersistentOutput::allow`); the host seals one `HistoricalAccessLeaf` per key in
>    list order, then the `PublicDecryptLeaf` when the output is `make_public`. The account stores no
>    list.

>    A request names only the encrypted value account and, for a delegated entry, the delegator as
>    allowed key; a client-supplied proof is rejected. Public-decrypt `extraData` v3 is exactly
>    `0x03 ‖ context id ‖ encrypted value account` (65 bytes).

> 4. **The deny list names applications.** `set_deny_scope` writes `DenyScopeRecord` at
>    `["deny-scope", program, scope]`; it gates every allow the host would seal — each persistent
>    write and `make_handle_public`, because sealing a public leaf is an allow.

> - Account layout: `181 + 32·peaks`, at most 2229 bytes (INVARIANTS Part II, MMR_ACL_MVP.md).
> - `fhe_execute` wire: `previous_subjects` and `output_subjects` gone; `allows` per persistent
>   output; the deny record an execution passes is its application's.

### DD-048, replaced in part by DD-060

DD-060 moved the public-decrypt Store out of `extraData`, beside the KMS routing.

>    client-supplied proof is rejected. Public-decrypt `extraData` names the Store (DD-049).

### DD-048, replaced in part by DD-066

DD-066 moved the leaf record out of the host listener, into the Merkle proof service's own database.

>    Both proofs are fetched by the KMS connector from the coprocessors' leaf record
>    (`POST /v1/solana/leaf-proofs`, API key; […])

> 3. **The leaf record lives in the host listener.** Leaves are recomputed from the confirmed
>    instruction stream and stored in the same database transaction as the compute rows, so the two
>    cannot disagree about which blocks were applied. The standalone `solana-proof-service`, the
>    relayer's proof passthrough, and the SDK's RPC evidence and proof-service clients are deleted
>    (DD-035 superseded). `solana_leaf_proof_server` serves the record apart from ingestion (DD-064).

> […] and lets the coprocessors, which already hold every instruction, own the record instead of
> a fourth service replaying the chain.

### DD-049, replaced in part by DD-065

Superseded in part by DD-065: generic disclosure emits the certified handle and cleartext and reads
no Store; only the KMS connectors check exact-handle MMR proofs.

Earlier text:

Adopted with RFC 035 (fhevm PR #3883). No compatibility with the retired per-value account model.

Decryption names each handle's Store beside the KMS routing (DD-060) and uses exact-handle MMR proofs. Current-slot publication …

This supersedes older per-value PDA seeds, StoredValue/PersistentOutput APIs, standalone
`make_handle_public`, v3 account extraData, and receipt-based transfer composition in this log.

### DD-056, replaced in part by DD-066

DD-066 moved the leaves to the Merkle indexer, which records them with the emitted handles. The
listener's replay repair leaves the record alone.

> The listener pairs each host `fhe_execute` with the one `FheExecutedEvent` from the host program
> that follows it before the next host `fhe_execute`. Only the host can sign its event authority, so
> an app cannot forge the event inside the host's instruction trace. It stores the emitted handles:
> computation rows, operands that name an earlier step, ACL leaves and allowed handles all use them.

> | A step whose emitted handle does not re-derive | […] Leaves and allowed handles keep the emitted handle. […] |

> Leaves are not reverted: a replayed write must reproduce the leaves recorded for it, or the
> listener stops. So a replay repairs computation rows, not a bug that recorded wrong leaves.

### DD-058, replaced in part by fhevm-internal#1909

fhevm-internal#1909 deleted `HostConfig.updated_slot` and its copy on `HostConfigUpdatedEvent`. Readers
take the current values from account state and see each change through the event CPI.

> A change stamps `updated_slot` and emits `HostConfigUpdatedEvent`, whose `signer` names the pauser
> or the admin.

### DD-060, replaced in part by DD-065

Superseded in part by DD-065: the host verifier no longer checks the public leaf; the KMS connectors
do.

Recorded in zama-ai/fhevm#4120. Supersedes the v4 `extraData` carrier of DD-049.

Earlier text:

A Solana public decrypt used to carry its Store inside `extraData` as version 4:
`0x04 ‖ contextId ‖ encryptedStore`. `extraData` is the KMS routing field on EVM, and the KMS signs
it, so the Store became part of a signed field whose version space EVM owns. A request could name
only one Store, and every layer (relayer, Gateway, connector, host verifier, SDK) had to parse a
Solana-only version.

connector keeps them in `handle_encrypted_stores` and proves each handle against its own Store in
one snapshot. The host verifier reads the context from v1 or v2 exactly as EVM `KMSVerifier`
does.

The KMS certificate no longer commits to the Store. It never bound it: the host verifier checks
the public leaf against the Store it is given (INVARIANTS #22). The Solana entry …

### DD-062, replaced in part by DD-066

DD-066 moved the stop at a Store write that does not continue the record from the listener to the
Merkle indexer.

> If the transaction wrote a Store, the next write to that Store does not continue its recorded leaf
> count, and the listener stops there until the leaf record, `solana_encrypted_state_nodes`
> included, is rewritten by hand: a replay computes leaves the record does not hold and stops too.
> If it wrote no Store, ingestion continues without its computation rows.

### DD-063, replaced in part by DD-066

DD-066 moved the nodes into the Merkle indexer's `nodes` table, created with the record.

> Ingestion records every MMR node of height 1 and above in `solana_encrypted_state_nodes`, in the
> transaction that appends the leaves completing it. `mmr_append` merges the new leaf node with one
> peak per trailing one bit of the leaf index, and each merge is a node; the listener records those
> merges as it appends.

> A database written before `solana_encrypted_state_nodes` existed has leaves without nodes, and its
> proofs fail verification: nothing is deployed, so no backfill exists.

### DD-063, replaced in part by DD-068

DD-068 answers a wrong leaf `inconsistent` in a 200, and the route also checks the leaf's
commitment against its row.

> The route checks the path with `mmr_verify` against the Store's recorded peaks before serving it;
> a missing or wrong row answers a retryable `upstream_transient`.

### DD-064, replaced in part by DD-066

DD-066 renamed the server and pointed it at the Merkle proof service's database, which the Merkle
indexer writes.

> `solana_leaf_proof_server` serves `POST /v1/solana/leaf-proofs` and the health routes as its own
> Deployment and ClusterIP Service, `<release>-solana-leaf-proof-server`, from the listener's image
> and with its own pool (`--database-pool-size`, 8 by default). It only reads the leaf record, so it
> can run several replicas and roll without downtime; the listener stays one replica with `Recreate`.

> | A separate image | The server is one small binary of the listener's crate; a second image adds a CI build and a tag to keep in step. |

> Each coprocessor database has one more client, with 8 connections by default. Proofs keep being
> served while the listener is down, for the leaves it recorded before it stopped. […] this costs
> nothing while one of them ingests; while every coprocessor's ingestion is stopped, a Store someone
> keeps appending to cannot be decrypted until one catches up.

### DD-064, replaced in part by DD-067

DD-067 made the server answer only signed requests, replaced the connector's proof routes (a URL
and an API key each) with `solana_proof_urls`, and made the connector ask the coprocessors one
after another.

> The connector takes the first proof that verifies from any coprocessor, so this costs nothing
> while one indexer runs; while every coprocessor's indexer is stopped, a Store someone keeps
> appending to cannot be decrypted until one catches up.

> The connector's proof routes name the proof server's Service.

### DD-066, replaced in part by DD-068

DD-068 added the `inconsistent` answer for a leaf of a quarantined store, or a leaf whose row fails
its checks.

> A record holds every Store from leaf zero or has not seen it, so a proof answer is `found`,
> `notFound` or `unknownAccount`; there is no incomplete history.
