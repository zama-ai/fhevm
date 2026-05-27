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

## DD-009: Operator Transfer Amounts Are Caller-Scoped

Status: adopted

Context:

ERC7984 `transferFrom` external input verification is bound to `msg.sender`, which is the operator
in an operator transfer. A Solana operator must not spend an amount input that was scoped to the
token owner.

Decision:

Direct holder transfers use owner-scoped transfer amount ACLs. Active-operator
`confidential_transfer_from` requires operator-scoped transfer amount ACLs.

Rationale:

The signer using an external input should match the authority the input was verified for.

Consequences:

Operator-scoped amount input authority is only an input-use bridge into the token compute signer.
The transferred amount output still grants durable roles according to token semantics.

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

## Open Product Decisions

These are not settled by the PoC design decisions above:

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
