# Solana design history

Decisions the Solana port no longer follows, kept in their original wording so a reader can see why
a current decision looks the way it does. Each entry's status line names what replaced it; the
replacing entry in [`DESIGN_DECISIONS.md`](DESIGN_DECISIONS.md) is the one the code follows. The
vocabulary here predates [`GLOSSARY.md`](GLOSSARY.md) and is intentionally left as written.

| Decision                                                                                    | Replaced by                  |
| ------------------------------------------------------------------------------------------- | ---------------------------- |
| DD-001 Store Handles In ACL Records, Not PDA Seeds                                          | DD-032, then DD-049          |
| DD-005 Public Decrypt Is A Post-Creation Release                                            | DD-032, then DD-049          |
| DD-006 Material Commitment Is Separate From ACL Authorization                               | DD-031                       |
| DD-009 Operator Transfer Model Removed                                                      | removed; fhevm-internal#1692 |
| DD-010 Token Disclosure Paths Are Label-Scoped                                              | DD-040                       |
| DD-011 Transfer-And-Call Removed In Favor Of App-Driven CPI Composition                     | DD-042 composition           |
| DD-018 Transfer-And-Call Refund Prepare/Finalize (replaced)                                 | DD-011                       |
| DD-019 Confidential Transfer Persists Only Final Balance And Transferred-Amount ACL Records | DD-049                       |
| DD-032 `EncryptedValue` + MMR Replaces Keyed-Nonce `AclRecord` (RFC-024)                    | DD-049                       |
| DD-035 Standalone Untrusted Solana MMR Proof Service                                        | DD-048                       |
| DD-036 Burn-Redemption Consume Authorizes By MMR Public-Decrypt Proof, Not Live Handle      | DD-045                       |
| DD-037 `fhe_execute` Events — `emit_cpi!`-Only, No `emit!` Log Fallback (DD-033 addendum)   | DD-038                       |
| DD-038 One Host-Owned Born-Public Lifecycle Batch Replaces Per-Operation Events             | removed; fhevm-internal#2079 |
| DD-039 HCU Block Cap Meters The Signed `compute_subject`, Not A Separate Authority          | DD-047                       |

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
