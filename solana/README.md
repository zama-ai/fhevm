# Solana FHEVM PoC

> Partially superseded (reconciliation, June 2026). The input-binding, KMS/decrypt, and trust-root
> framing in parts of this README predates the reconciliation: external inputs now bind via an on-chain
> secp256k1 coprocessor attestation (not an Ed25519 verifier set), the VerifierSet subsystem was
> replaced by a `define_kms_context` singleton, and user/public decrypt reuse the Gateway V2 / EVM stack
> with on-chain secp256k1 cert verification rather than a separate native-v0 KMS path. For the current,
> authoritative view (with every design choice as a debatable ADR), read
> `solana/docs/DESIGN_DECISIONS.md` — especially DD-007, DD-012, DD-015, and DD-020 through DD-030.

This workspace is the Solana host-chain PoC for the `openzeppelin-solana-track` branch.

It is meant to be a readable base for:

```text
1. getting familiar with the Solana end-to-end flow
2. adding PoC features without guessing the existing intent
3. testing Solana behavior against EVM-derived FHEVM invariants
```

The PoC does not settle the final Solana product shape. It makes one path real enough that ACL, event listening, worker compute, and user decrypt can be discussed from code and tests.

## Where To Start

```text
solana/programs/zama-host
  Protocol-side host program.
  Owns FHE event emission and ACL enforcement.
  Read the Rustdoc for account layouts, role flags, PDA helpers, and instruction invariants.

solana/programs/confidential-token
  App-side PoC program.
  Models a minimal confidential token / cUSDC wrapper.
  Its local fhe.rs module is the current dev-facing wrapper around raw ZamaHost CPI calls.

solana/runtime-tests
  Fast Mollusk tests for Solana accounts, PDAs, CPI, events, and ACL behavior.
  tests/support/fhe_runtime.rs adds a cleartext backend that consumes real ZamaHost events.

solana/docs
  Permanent design rationale.
  Start with DESIGN_DECISIONS.md before changing ACL, KMS, event transport, decrypt, or token
  transfer behavior.

coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  Maps typed Solana host events into the existing coprocessor DB model.

coprocessor/fhevm-engine/tfhe-worker/src/tests/solana_poc.rs
  Worker-backed end-to-end tests with real small TFHE ciphertexts.
```

## Current Architecture At A Glance

The PoC has one protocol program and one app program.

```text
confidential-token
  owns token semantics
  stores current balance handle pointers
  chooses ACL domain/app account/label/nonce sequence
  calls ZamaHost through named helpers in src/fhe.rs

zama-host
  owns canonical ACL records
  verifies host operation preconditions
  derives or checks FHE handles
  emits protocol events
  gates mock/test-only paths through HostConfig authorities

host-listener / tfhe-worker / KMS connector
  consume protocol events and account-shaped evidence
  must treat events as indexing hints
  must verify ACL state before authorizing decrypts
```

The most important account boundary is:

```text
App account state lives in confidential-token.
Host ACL state lives in zama-host.
Opaque handles are stored in ACL records; they are not PDA seeds.
```

This keeps Solana account discovery compatible with unpredictable FHEVM handles. Apps can prepare
the output ACL PDA before the transaction executes, while `zama-host` writes the actual computed
handle after it verifies the operation.

## Documentation Contract

Keep these documentation layers in sync:

```text
solana/README.md
  branch-level architecture, safety boundaries, commands, and handoff notes

solana/docs/
  permanent design rationale and focused notes for decisions that should outlive the issue ledger

solana/docs/DESIGN_DECISIONS.md
  stable decision index: problem, selected path, rationale, consequences, and open product work

solana/OZ_GUIDE.md
  short role-specific index for OpenZeppelin follow-up work

solana/DEVELOPMENT_ISSUES.md
  temporal development pitfalls, known failures, workarounds tried, and unresolved follow-up items

solana/DEVELOPMENT_LOGBOOK.md
  temporal resume state and current audit notes

Rustdoc in solana/programs/*
  authoritative inline documentation for public accounts, events, errors, helpers, and CPI-facing
  instruction contexts
```

Inline docs should explain the invariant or trust boundary, not restate the identifier. Public
account fields should document who writes them, who verifies them, or why they are part of an
off-chain witness. Test/mock instructions must say why they are not production APIs.

When a temporal issue discovers a durable architecture decision, promote the rationale into
[`docs/DESIGN_DECISIONS.md`](docs/DESIGN_DECISIONS.md) or a focused file under `docs/` before
handoff.

## Handoff Path

Use this order when picking up the branch:

```text
1. Run it
   cd solana
   bash scripts/check-zama-host-idl.sh
   cargo test --workspace
   cargo doc --no-deps -p zama-host -p confidential-token

2. Read the flow
   Start with docs/DESIGN_DECISIONS.md for rationale.
   Start with "Global Flow", then "Confidential Transfer", then "User Decrypt Shape".
   The current worker-backed product-shaped tests live in:
     coprocessor/fhevm-engine/tfhe-worker/src/tests/solana_poc.rs

3. Change the PoC
   App behavior usually belongs in confidential-token.
   Host-chain FHEVM semantics usually belong in zama-host.
   Event normalization belongs in host-listener/src/solana_adapter.rs.
   Worker/KMS-shaped checks belong in tfhe-worker/src/tests/solana_poc.rs.

4. Keep the negative tests
   Every new happy path should have a wrong signer, wrong ACL record, wrong handle,
   or wrong subject test when the feature touches authorization.
```

## Collaboration Contract

Use this guide as the central technical handoff document for the branch. The permanent
[`docs/DESIGN_DECISIONS.md`](./docs/DESIGN_DECISIONS.md) file records durable rationale, the short
[`OZ_GUIDE.md`](./OZ_GUIDE.md) file is a role-specific index for OpenZeppelin follow-up work, and
the temporal [`DEVELOPMENT_ISSUES.md`](./DEVELOPMENT_ISSUES.md) ledger records implementation
pitfalls and known failures. Keep technical details here or in `docs/`, and link back from temporal
files instead of duplicating large sections.

The PoC can still have breaking changes. The rule is:

```text
breaking changes are allowed
silent breaking changes are not allowed
```

When changing the ZamaHost CPI surface, ACL record shape, event payload, or decrypt-relevant fields,
update this guide and the affected tests in the same PR.

```text
Change ACL storage/checking?
  update zama-host + solana/runtime-tests

Change token behavior?
  update confidential-token + solana/runtime-tests

Change emitted host events?
  update zama-host + host-listener solana_adapter + worker tests

Change user decrypt semantics?
  update runtime KMS model tests + tfhe-worker solana_poc tests

Change public accounts/events/helpers?
  update Rustdoc and run cargo doc for zama-host + confidential-token
```

For OpenZeppelin follow-up work, the safe area is:

```text
solana/programs/confidential-token
  Improve the confidential token flow against the current ZamaHost CPI surface.

solana/runtime-tests/tests/host_mollusk.rs
  Add host behavior tests here first.

solana/runtime-tests/tests/token_mollusk.rs
  Add confidential-token behavior tests here first.
```

Current OZ handoff assumptions:

```text
1. Treat zama-host CPI as the protocol boundary.
2. Keep confidential-token SPL-like and app-owned.
3. Add Mollusk tests before changing token behavior.
4. Do not derive account addresses from handles.
5. Do not add a separate ACL program unless the guild explicitly changes direction.
```

Avoid adding a separate Anchor workspace, a standalone ACL program, or TypeScript-only tests for core
authorization behavior unless the guild explicitly decides to change direction.

## PoC Progress

Use this checklist to see where the branch stands. Keep it updated when a PR changes the PoC surface.

### Working Now

- [x] Anchor workspace with `zama-host`, `confidential-token`, and Mollusk runtime tests.
- [x] ZamaHost emits typed Anchor CPI events for real host operations and clearly named
      `test_emit_*` shims used by listener / worker tests.
- [x] Host-listener decodes ZamaHost protocol events through Rust generated at build time from
      the checked-in ZamaHost Anchor IDL snapshot.
- [x] Solana host events normalize into the existing coprocessor DB event shape.
- [x] Worker-backed tests use real small TFHE ciphertexts for Solana-originated events.
- [x] Confidential token can initialize a mint with an encrypted total-supply handle and initialize
      token accounts with zero encrypted balance handles; nonzero confidential supply enters
      through wrap or a future explicit mint flow.
- [x] Confidential token can wrap SPL-like USDC by rotating both the recipient balance handle and
      the mint-level encrypted total-supply handle.
- [x] Confidential token can transfer by rotating Alice and Bob balance handles through the
      all-or-zero ERC7984-style graph: `ge`, debit candidate `sub`, encrypted `if_then_else`,
      transferred amount `sub`, and recipient `add`.
- [x] Confidential token can settle a receiver callback result through a split prepare/finalize
      refund graph: prepare computes and debits the encrypted refund from the recipient, finalize
      credits the original sender and records the final encrypted transferred amount.
- [x] Confidential token can call an arbitrary receiver hook program between transfer and callback
      settlement through the owner-gated split instruction, then verify the hook's Solana return
      data binds the encrypted callback-success handle and ACL record to the prior transfer.
- [x] `confidential-token-receiver-sdk` packages the reusable receiver-hook return ABI
      (`TransferReceiverReturn`, `TRANSFER_RECEIVER_RETURN_MAGIC`,
      `transfer_receiver_return_data`, and a return-data setter) for external receiver programs.
- [x] Confidential token re-exports the receiver-hook ABI, and the separate
      `confidential-token-receiver` sample program uses the SDK in runtime tests.
- [x] Confidential token can burn through the same all-or-zero graph, rotating the holder balance
      and mint-level encrypted total supply while emitting the encrypted burned amount.
- [x] Confidential token intentionally omits operator/delegated-transfer APIs. Transfers are
      owner-authorized, with a separate payer account for rent/fees.
- [x] Confidential token mints can rotate disclosure/redemption verifier-set pointers through a
      mint-authority instruction; pending requests remain bound to the verifier set captured in the
      request witness unless that set is disabled.
- [x] Confidential token emits local balance-history events for app/front-end indexing. These
      events are not consumed by the generic coprocessor listener.
- [x] Confidential token emits local total-supply history events for app/front-end indexing.
- [x] Confidential token emits encrypted transferred/burned amounts and can request public
      disclosure for either the current balance or any token-scoped amount whose ACL grants the
      requester `ACL_ROLE_PUBLIC_DECRYPT`.
- [x] Confidential token can emit KMS-certified cleartext balance/amount disclosure events after
      verifying the ACL public-decrypt release flag, a native Ed25519 disclosure certificate, and
      host material commitment witness.
- [x] Confidential token can redeem a KMS-certified burned amount from the underlying vault once,
      protected by a material witness and burn-redemption replay marker PDA.
- [x] Canonical confidential token scenario covers wrap, transfer, current decrypt, historical
      decrypt, and expected failures.
- [x] Compute-time ACL is enforced inside `zama-host` before event emission.
- [x] Computed output handles are verified inside `zama-host`; durable ACL records are created by
      either the composed `fhe_eval` path or the per-class bind paths that emit the compute event
      and initialize ACL state in one host instruction.
- [x] Generic persistent ACL grants mutate the existing canonical ACL record instead of creating a
      second record for the same handle.
- [x] ACL subjects carry explicit role flags for compute/use, grant authority, public decrypt, and
      user decrypt policy checks; host writes and KMS witness decoding reject zero or unknown role
      bitsets and malformed subject slots.
- [x] More than eight subjects are represented with per-subject overflow permission PDAs instead of
      deriving any account address from the handle bytes.
- [x] Host-level config gates pause state, mock input binding, test event shims, and grant deny-list
      behavior behind configured authorities.
- [x] Public decrypt is role-gated through `ACL_ROLE_PUBLIC_DECRYPT`, not by plain subject
      membership.
- [x] Token account initialization creates a zero balance handle through a host-owned
      `trivial_encrypt_and_bind` path, not through caller-supplied handle binding or unbacked
      balance birth.
- [x] ZamaHost can create durable random ciphertext handles through `fhe_rand_and_bind` and
      `fhe_rand_bounded_and_bind`, deriving the random seed on-chain from slot context plus output
      nonce metadata, emitting worker-consumable random events, and binding the result into a
      canonical ACL record.
- [x] Confidential token can create token-scoped random and bounded-random amount handles through
      `create_random_amount` and `create_random_bounded_amount`, using the same host-owned ACL
      record shape as transfer and burn input amounts.
- [x] Runtime tests include a cleartext FHE backend that consumes emitted ZamaHost events and checks
      the plaintext semantics of transfer and wrap flows.
- [x] Confidential token uses a small `fhe` helper module so app logic calls named FHE helpers
      instead of hand-assembling raw Anchor CPI calls.
- [x] Keyed-nonce ACL records avoid deriving Solana account addresses from opaque handles.
- [x] User decrypt is modeled with signed authorization plus ACL record verification.
- [x] Current and historical balance decrypt are both modeled when the relevant ACL record still
      exists.
- [x] Public decrypt is modeled through `allow_for_decryption`.
- [x] User-decryption delegation records carry a monotonic counter and last-update slot, and reject
      same-slot double updates so signed witness counters cannot be invalidated twice in one slot.
- [x] Negative tests cover wrong signer, wrong ACL record, wrong handle, wrong domain key, stale
      current ACL, wrong output ACL, reused output ACL, scalar RHS account behavior, and
      unauthorized public decrypt, delegation counter updates, and same-slot
      delegation double updates.
- [x] Scalar RHS host behavior is modeled: encrypted RHS requires an ACL record; scalar RHS does
      not.
- [x] External encrypted-input birth has a production-shaped Solana path:
      `verify_input_and_bind` validates a signed `SolanaInputProof` through a native Ed25519
      pre-instruction before writing the canonical ACL record.
- [x] ZamaHost has host-owned material commitment PDAs for handle decryptability, committed by the
      configured material authority for supported host-chain handles, sealed onto the ACL record,
      and verified separately from ACL membership.
- [x] The real KMS connector has an explicit `chain_kind = "solana"` fail-closed path, so Solana
      handles are not checked through the EVM ACL contract ABI by accident.
- [x] The KMS connector has a native Solana ACL witness verifier that decodes raw Anchor account
      data for host-owned ACL records, overflow permission PDAs, and user-decryption delegation
      PDAs, then checks subject-slot exactness, domain scope, public decrypt flags, and material
      commitment witnesses.
- [x] The KMS connector has a native-v0 Solana request admission primitive for typed request modes,
      request hashes, native extra-data hashes, reencryption-key hashes, app-context membership,
      handle metadata and encrypted-bit limits, material witness binding, same-key batch
      enforcement, replay-key derivation, and same-hash/different-hash replay decisions.
- [x] The KMS connector has a native-v0 Ed25519 request-signature verifier over the canonical
      signed request-hash message.
- [x] The KMS connector has a durable native-v0 replay reservation table/helper for signed Solana
      requests, keyed by host chain, cluster, KMS context, signer, and nonce.
- [x] The KMS connector has a native-v0 request wire parser that extracts payload fields, handle
      entries, raw extra data, reencryption keys, request signatures, and the exact account fetch
      plan, then rejects missing, duplicate, or extra account witnesses before attaching fetched
      accounts into the admission request type.
- [x] The KMS connector has a native-v0 live-path policy boundary for account-fetch count, RPC
      response size, finalized-by-default commitment policy, signed validity windows, and explicit
      opt-in rechecks for non-finalized reads before release.
- [x] The KMS connector has a native-v0 Solana JSON-RPC `getMultipleAccounts` fetcher that reads a
      single response-context slot, decodes base64 account data, and preserves account owner pubkeys
      for witness attachment.
- [x] The KMS connector has a native-v0 finality recheck helper that re-fetches accepted
      non-finalized witness accounts at finalized commitment, compares them byte-for-byte against
      the accepted snapshot, and re-runs admission before release.
- [x] The KMS connector has a native-v0 admission service that combines witness verification,
      request-signature verification, and replay reservation for already-fetched requests.
- [x] The KMS connector has a native-v0 response verifier for accepted-request binding, response
      body hashes, release-pinned signer-set hashes, certificate thresholds, sorted signer
      identities, and Ed25519 KMS response signatures.
- [x] The KMS connector has a worker-side native-v0 library boundary that parses signed native
      request bytes, fetches and admits Solana witnesses, rechecks non-finalized reads before
      release, verifies native KMS response certificates, and returns routed response metadata. This
      boundary is not yet connected to the production worker loop.
- [x] The KMS connector has durable native-v0 Solana decryption request/response tables plus a
      store boundary for inserting parsed request bytes, picking pending native requests, recording
      release metadata, and publishing verified native response rows.
- [x] The KMS connector has a native-v0 Solana request/response notification and picker boundary
      over those tables, with polling fallback, without widening the legacy Gateway event enum.
- [x] tx-sender has a separate native-v0 Solana response row picker/status/publisher boundary for
      `solana_native_decryption_responses_v0`, so verified native responses are not routed through
      the EVM Gateway response sender.
- [x] The KMS connector keeps Solana Gateway decryption fail-closed instead of trusting
      requester-supplied `extraData` witnesses. Branch-local Gateway-PoC witness helpers remain as
      tests/scaffolding; production decryption is the RPC-verified native-v0 path above.

### Partly Modeled

- [ ] Native-v0 Solana verification exists at the library/store/picker boundary, but production
      live processing still needs KMS Core native work-item dispatch and a concrete tx-sender
      `SolanaNativeResponsePublisher` target from `acl_storage_spec.md`. Gateway-PoC `extraData`
      witness helpers remain branch-local scaffolding, not the live trust path.
- [ ] `verify_input_and_bind` checks the Solana account/transaction witness shape for external
      encrypted inputs and enforces the active input verifier-set threshold, but real
      proof/transciphering validation behind those verifier signatures still lives outside this
      program. The mock shim remains PoC-chain-gated test glue and requires a signer from the active
      input verifier set.
- [ ] ACL records are created already initialized through Anchor `init`; there is no stored
      `Empty -> Bound` enum in the PoC.
- [ ] Role names and production governance for public decrypt are still subject to product review,
      but compute-only subjects cannot flip `public_decrypt`.
- [ ] Subject overflow works through witness PDAs, but durable ACL/material compaction and archival
      are not designed yet. Durable ACL rows, overflow permissions, material commitments,
      delegation rows, and replay markers are intentionally not closeable in this PoC.
- [x] Transient allow is modeled with instruction-local `fhe_eval` outputs and host-owned
      one-shot `TransientSession` PDAs: one capability per session, one successful consume, and no
      implicit public decrypt authority. Public-decrypt propagation is an explicit capability bit
      and durable births still set `public_decrypt = false`. Consumes must prove an earlier
      top-level host `create_transient_session` for the same account in the current transaction, so
      slot expiry is only defense in depth. Sessions can be closed by authority before expiry or
      permissionlessly after expiry; the permanent rationale is captured in
      [`docs/TRANSIENT_ALLOW.md`](docs/TRANSIENT_ALLOW.md), while broader SDK ergonomics remain open.
- [x] Confidential transfers persist only the three ACL records that remain useful after the
      transaction: sender balance, transferred amount, and recipient balance. The success bit and debit
      candidate are `fhe_eval` instruction-local transients and remain replayable from host FHE
      events without creating durable ACL accounts.
- [ ] Historical handle lookup is assumed to be app/indexer responsibility for now.

### Missing Next

- [ ] Connect the active input verifier-set signatures to the real proof/transciphering service that
      produces them.
- [ ] Finalize client/app SDK ergonomics for transient allow sessions on top of the one-shot
      session convention. `zama-fhe::EvalBuilder` covers instruction-local mixed eval assembly,
      dynamic account role generation, and resolved-account CPI execution; automatic
      create/consume/close instruction assembly for cross-instruction transient sessions remains
      open.
- [ ] Decide archival/compaction rules for durable ACL and material evidence without breaking
      historical decrypt or replay safety.
- [ ] Replace the Gateway-PoC `extraData` witness envelope in the live connector path with KMS Core
      native work-item dispatch and a concrete native Solana response publisher from
      `acl_storage_spec.md`.
- [ ] Extend the canonical confidential token scenario when adding new token features, instead of
      creating a second product flow.
- [ ] Keep RFC 024 aligned when the PoC proves or disproves a design choice.

## Global Flow

```text
Solana transaction
  |
  v
confidential-token program
  app state:
    ConfidentialMint
    ConfidentialTokenAccount
  app events:
    BalanceHandleUpdatedEvent
    TotalSupplyHandleUpdatedEvent
    ConfidentialTransferEvent
    ConfidentialBurnEvent
    BalanceDisclosureRequestedEvent
    AmountDisclosureRequestedEvent
    RandomAmountCreatedEvent
    BalanceDisclosedEvent
    AmountDisclosedEvent
    BurnRedeemedEvent
      for frontend/app indexers only
  |
  | CPI
  v
zama-host program
  protocol state:
    ACL record PDAs
    handle material commitment PDAs
  generic protocol events exposed through the ZamaHost Anchor IDL:
    FheBinaryOpEvent
    TrivialEncryptEvent
    FheRandEvent
    AclAllowedEvent
    rich host config / ACL role / public decrypt / deny / delegation / material events
  |
  | Anchor self-CPI event bytes
  v
host-listener Solana adapter
  converts SolanaHostEvent into the existing TFHE event / ACL DB model
  |
  v
coprocessor DB
  stores computations and allowed handles
  |
  v
tfhe-worker
  computes real ciphertexts
  |
  v
test decrypt / future KMS path
  reads result handles and verifies ACL-shaped user decrypt
```

Boundary rule:

```text
confidential-token decides app semantics.
zama-host enforces FHEVM host semantics.
host-listener normalizes Solana events into the existing coprocessor model.
tfhe-worker computes ciphertexts from DB work items.
KMS-style verification combines signed authorization + handle entry + ACL state.
```

The listener boundary is intentionally generic:

```text
host-listener consumes:
  ZamaHost protocol events decoded from the ZamaHost Anchor IDL shape

host-listener does not consume:
  confidential-token BalanceHandleUpdatedEvent
  confidential-token TotalSupplyHandleUpdatedEvent
  confidential-token ConfidentialTransferEvent
  confidential-token ConfidentialBurnEvent
  confidential-token BalanceDisclosureRequestedEvent
  confidential-token AmountDisclosureRequestedEvent
  confidential-token RandomAmountCreatedEvent
  confidential-token BalanceDisclosedEvent
  confidential-token AmountDisclosedEvent
  confidential-token BurnRedeemedEvent
  token account state
  cUSDC-specific labels or nonce conventions
```

App-specific events are still useful, but for a different consumer:

```text
frontend / app indexer
  reads BalanceHandleUpdatedEvent
  learns "AliceTokenAccount moved from A7/h7 to A8/h8"
  builds current and historical decrypt requests
```

## Event Transport Note

### Why Not EVM-Style Event Reliance

The Solana PoC keeps the EVM-compatible listener shape for compute indexing, but Solana events are
not the authorization source. On the EVM side, contract logs can drive much of the coprocessor
workflow because the relevant contract state and log stream share the same execution model. On
Solana, log delivery is provider-dependent, plain `emit!` logs may be truncated, and
application-local events are not protocol authority. Production authorization must therefore be
rebuilt from finalized or policy-approved transaction/account data and verified against host-owned
ACL, material, delegation, and replay witnesses.

The current PoC keeps Anchor CPI events because the local listener and Mollusk tests already decode
that shape. This is a PoC convenience, not a free production choice: every `emit_cpi!` adds a
ZamaHost self-CPI frame, and Solana has a hard CPI/instruction-stack nesting limit. Deep app flows
such as token -> host -> hook/settlement paths can hit that limit before compute budget is the
binding constraint. Production event transport should not rely on `emit!` logs for Zama-critical
data, and should avoid adding a self-CPI to every app path solely for observability.

The recommended production direction is a Yellowstone gRPC / Geyser listener:

```text
Yellowstone regular transaction stream
  subscribe by ZamaHost program id and confidential-token program id
  receive finalized or policy-approved transaction updates with status metadata
  decode top-level and inner instruction data, including existing Anchor CPI event bytes
  if using processed/confirmed for latency, buffer by slot and release only after slot finality policy
  fetch/verify host-owned ACL/material/delegation state before authorizing decrypts

Yellowstone account stream
  subscribe by owner = ZamaHost program id and confidential-token program id
  index account writes for current app pointers and durable ACL/material evidence
  treat account data as discovery input until KMS witness verification accepts it

Yellowstone slot/replay handling
  subscribe to slot notifications alongside transaction/account streams
  dedupe reconnect replay from `from_slot` and use `SubscribeReplayInfo` to detect retention limits
  for finalized policy, retroactively finalize ancestors when a provider omits per-slot finalization

Yellowstone deshred stream
  optional low-latency pre-execution signal only
  not an authorization or event-completeness source because it lacks execution metadata
```

Source notes from the 2026-05-26 investigation:

- Anchor documents that `emit!` writes program logs that may be truncated by data providers, while
  `emit_cpi!` stores event data in CPI instruction data but adds CPI compute cost:
  `https://www.anchor-lang.com/docs/features/events`.
- Triton's Yellowstone Dragon's Mouth docs and the upstream `yellowstone-grpc` README document
  account, transaction, block, and slot subscriptions with server-side filters and commitment
  levels: `https://docs.triton.one/project-yellowstone/dragons-mouth-grpc-subscriptions` and
  `https://github.com/rpcpool/yellowstone-grpc`.
- Triton's docs recommend buffering transaction/account updates by slot when the client manages
  `confirmed` or `finalized` commitment, and document `from_slot`/`SubscribeReplayInfo` replay for
  short reconnects. They also note finalized slot notifications can be incomplete and require
  retroactively marking ancestors finalized when a finalized descendant is observed.
- The upstream protobuf exposes `SubscribeRequest` filters for accounts, slots, transactions,
  transaction status, blocks, block metadata, entries, commitment, and `from_slot`; normal
  transaction updates carry `TransactionStatusMeta`. The deshred stream is a separate RPC and is
  faster but explicitly lacks execution status, logs, inner instructions, balances, compute usage,
  and `TransactionStatusMeta`:
  `https://raw.githubusercontent.com/rpcpool/yellowstone-grpc/master/yellowstone-grpc-proto/proto/geyser.proto`.
- Solana's current CPI stack depth is still a production constraint. Official docs describe
  `MAX_INSTRUCTION_STACK_DEPTH = 5`, which means entry instruction plus four CPI levels before any
  SIMD-0268 increase is active on the target cluster:
  `https://solana.com/docs/core/cpi/cpi-execution`.

`test_emit_*` instructions are not protocol APIs. They emit typed events without proving or writing
the corresponding ACL record and exist only to keep listener / worker tests fast. Production-shaped
birth paths now live in `verify_input_and_bind`, `trivial_encrypt_and_bind`,
`fhe_rand_and_bind`, `fhe_rand_bounded_and_bind`, and the token-level random amount wrappers.

## Vocabulary

Use these words consistently when discussing this PoC:

```text
handle
  FHEVM opaque pointer to a ciphertext.
  Do not assume the handle is predictable or derivable.

ACL domain key
  App-wide domain for authorization.
  In the token PoC: ConfidentialMint / cUSDC mint pubkey.

app account
  App-owned account that carries the concrete state being authorized.
  In the token PoC: AliceTokenAccount or BobTokenAccount.

encrypted value label
  Domain-separated label for one encrypted field inside an app account.
  In the token PoC: "balance", "transfer_amount", "burn_amount", and scratch labels.

nonce key
  Hash of:
    "zama-acl-nonce-key-v1"
    ACL domain key
    app account
    encrypted value label

nonce sequence
  App-maintained monotonic counter for one nonce key.
  In the token PoC: ConfidentialTokenAccount.next_balance_nonce_sequence for balances and
  ConfidentialTokenAccount.next_amount_nonce_sequence for token-owned random amount birth.

ACL record
  PDA owned by zama-host:
    PDA("acl-record", nonce_key, nonce_sequence)

subject
  Pubkey plus role flags stored by an ACL record.
  Examples: Alice with user/use/public-decrypt roles, compute_signer with compute/use roles.

compute signer
  Program-controlled PDA that signs CPI calls into zama-host.
  In the token PoC: PDA("fhe-compute", ConfidentialMint).
```

The current ACL PDA shape is:

```text
nonce_key = H("zama-acl-nonce-key-v1", acl_domain_key, app_account, encrypted_value_label)

acl_record = PDA("acl-record", nonce_key, nonce_sequence)
```

The handle is stored inside the ACL account. The handle is not part of the PDA seed.

That choice is deliberate:

```text
Solana requires accounts up front.
Computed FHEVM handles are opaque and may be unpredictable.
Therefore ACL account addresses cannot depend on the computed handle.
```

## Core Invariant

The main EVM-derived invariant is:

```text
No FHE compute event should exist unless the host program has verified ACL.
```

In EVM terms:

```text
App contract
  -> FHEVMExecutor._binaryOp(...)
       checks ACL
       emits/records FHE op
```

In this Solana PoC:

```text
confidential-token
  -> CPI zama-host::fhe_eval(...) or a per-class bind instruction
       checks operand ACL records
       verifies the output handle against the host formula
       emits typed Anchor event
       initializes the output ACL record
```

The app program does not perform a separate pre-check for normal compute. It passes operand ACL
accounts to `zama-host`. `zama-host::fhe_eval` and the per-class bind instructions reject the
operation before emitting the FHE event if an encrypted operand is not allowed for the compute
signer. If checks pass, `zama-host` verifies the result handle, emits the compute event, and creates
the first canonical ACL record for each durable output handle in the same instruction.
Instruction-local scratch values should stay inside `fhe_eval` transients instead of becoming
durable ACL state.

## Host Config And Test Gates

`zama-host` has a singleton config PDA:

```text
host_config = PDA("host-config")
```

It stores the host-chain id used in handle derivation, admin authority, active input verifier set,
material authority, test-shim authority, pause state, and feature gates for the mock input path,
test event shims, and grant deny-list checks.

Production-shaped instructions read `host_config` and reject while paused. The PoC test-only
instructions are explicitly gated:

```text
mock_input_verified_and_bind
  requires host_config.mock_input_enabled
  requires the PoC feature and local PoC chain id

test_emit_*
  requires host_config.test_shims_enabled
  requires test_authority signer
```

These shims are still not protocol APIs. They exist to test listener/worker paths independently of
the production-shaped input, trivial-encrypt, and random-handle birth instructions.

The config also controls grant deny-list enforcement. When enabled, `allow_acl_subjects` requires a
matching deny-list witness for the grant authority and refuses the grant if that subject is denied.
User-decryption delegation is represented by host-owned delegation PDAs with monotonic counters and
last-update slots. Grant/regrant/revoke updates reject if the row was already updated in the current
slot, matching the native-v0 stale-counter guard. The Gateway/KMS witness format still needs to
carry and verify those records.

## ACL Account Model

```text
                         owns
zama-host program --------------------+
                                      |
                                      v
ACL record PDA
  address = PDA("acl-record", nonce_key, nonce_sequence)
  data:
    handle                = hA1
    nonce_key             = H(cUSDCMint, AliceTokenAccount, "balance")
    nonce_sequence        = 7
    acl_domain_key        = cUSDCMint
    app_account           = AliceTokenAccount
    encrypted_value_label = "balance"
    subjects              = [
      Alice: use + user + grant + public_decrypt,
      compute_signer: use + compute
    ]
    overflow_subject_count = 0
```

`subjects` stores Pubkeys plus role flags. `ACL_ROLE_USE` is the base membership bit used by normal
handle access checks. `ACL_ROLE_COMPUTE` is used by host compute operations, `ACL_ROLE_GRANT` is
required to extend persistent grants, and `ACL_ROLE_PUBLIC_DECRYPT` is required to flip the durable
public-decrypt flag. The token PoC gives users the user/grant/public-decrypt roles for their own
balance records and gives the compute signer compute/use only.

The first eight subjects are embedded in the canonical ACL record. Additional subjects use witness
PDAs:

```text
acl_permission = PDA("acl-permission", acl_record, subject)
```

`allow_acl_subjects` creates or updates those overflow permission accounts through
`remaining_accounts`. KMS/Gateway witness design must carry these PDAs when a decrypted handle is
authorized through an overflow subject.

For computed outputs, the address is known before execution but the handle is not:

```text
before tx:
  A7 address = PDA("acl-record", nonce_key, 7)
  h7 is unknown

during zama-host::fhe_binary_op_and_bind_output:
  base = H("FHE_comp", op, lhs, rhs, scalar, zama_host, chain_id, previous_bank_hash, timestamp)
  h7   = H("FHE_bound_output", base, nonce_key, 7)
  A7.handle = h7
  A7.subjects = [Alice: user/grant/public_decrypt, compute_signer: compute]
```

The durable output nonce is part of the bound output handle. This prevents two different ACL record
addresses for the same app account from intentionally binding the exact same computed handle. If an
app wants two durable outputs for the same operation, it gets two distinct opaque handles.
`trivial_encrypt_and_bind` uses the same output nonce metadata in its handle derivation, so equal
plaintexts born for different ACL records do not alias to the same host handle.

The PoC uses the previous slot hash when Mollusk or the cluster exposes it. Local bootstrap tests can
fall back to zero when no prior slot hash exists; this is test glue, not the intended production
entropy source.

Creating a persistent ACL record has two shapes in this PoC.

First birth is owned by a trusted host path:

```text
zama-host::trivial_encrypt_and_bind(...)
  creates a trivial-encrypt handle and its first ACL record

zama-host::mock_input_verified_and_bind(...)
  authority-gated mock short-circuit for future InputVerifier/transciphering birth

zama-host::verify_input_and_bind(...)
  checks a signed SolanaInputProof via the immediately preceding Ed25519 verifier instruction
  before creating the input handle ACL record

zama-host::fhe_binary_op_and_bind_output(...)
  checks operand ACL records, emits a compute event for the verified output handle,
  and creates the durable output ACL record
```

Generic persistent grants extend the existing canonical ACL record or create overflow permission
PDAs. `allow_acl_subjects` requires the caller to have the grant role on the same handle:

```text
zama-host::allow_acl_subjects(handle = h7, authority = Alice, acl_record = A7, subjects = [Bob])

requires:
  A7 is owned by zama-host
  A7 is the canonical PDA for its stored nonce fields
  A7.handle == h7
  A7.subjects contains Alice with ACL_ROLE_GRANT
  optional deny-list witness does not mark Alice as grant-denied

effect:
  A7.subjects now also contains Bob, or PDA("acl-permission", A7, Bob) exists if inline capacity is full
```

This prevents handle laundering:

```text
Mallory sees Alice's handle h7
Mallory tries to create M1 storing h7 and subjects = [Mallory]
zama-host rejects it unless Mallory has grant authority through a canonical ACL record for h7
```

For token balances, `confidential-token` signs as:

```text
PDA("token-account", mint, owner)
```

`zama-host` does not know token PDA seeds. For each ACL record it verifies:

```text
the app account pubkey signed
expected_nonce_key = H(acl_domain_key, app_account, encrypted_value_label)
record.nonce_key == expected_nonce_key
the ACL PDA address matches ("acl-record", expected_nonce_key, nonce_sequence, bump)
the stored ACL fields match the instruction fields
the requested subject is in subjects[] with the role required by the instruction
or a canonical overflow permission PDA witnesses that subject
```

That keeps token-specific semantics inside the token program.

## Initial Balance Birth

The token program does not accept caller-provided initial balance handles or nonzero initial
balances. Account initialization is zero-only so the wrapper cannot mint unbacked confidential
supply outside the explicit wrap path.

```text
confidential-token::initialize_token_account(initial_balance = 0)
  |
  +--> CPI zama-host::trivial_encrypt_and_bind(0)
  |      app_account = AliceTokenAccount
  |      output ACL = A0
  |      subjects = [Alice: user/grant/public_decrypt, compute_signer: compute]
  |      creates hA0 inside zama-host
  |
  +--> stores:
         AliceTokenAccount.balance_handle = hA0
         AliceTokenAccount.balance_acl_record = A0
         AliceTokenAccount.next_balance_nonce_sequence = 1
```

That keeps the same security rule as computed outputs:

```text
the host creates the handle
the host creates the first ACL record
the app stores the resulting handle/account pointer
```

## Confidential Transfer

`confidential-token::confidential_transfer` is the first SPL-like flow.

Initial state:

```text
AliceTokenAccount
  owner = Alice
  balance_handle = hA0
  balance_acl_record = A0
  next_balance_nonce_sequence = 1

BobTokenAccount
  owner = Bob
  balance_handle = hB0
  balance_acl_record = B0
  next_balance_nonce_sequence = 1

compute_signer = PDA("fhe-compute", cUSDCMint)
amount handle = hX
```

Transaction shape:

```text
Alice signs tx
  |
  v
confidential-token::confidential_transfer(amount = hX)
  |
  +--> CPI zama-host::fhe_eval(combined frame)
  |      verifies hOk = FHE.ge(hA0, hX)
  |      verifies hDebitCandidate = FHE.sub(hA0, hX)
  |      verifies hA1 = FHE.ifThenElse(hOk, hDebitCandidate, hA0)
  |      keeps hOk and hDebitCandidate instruction-local
  |      creates A1 for hA1 with Alice user/grant/public_decrypt and compute_signer compute roles
  |      verifies hMoved = FHE.sub(hA0, hA1)
  |      creates transferred-amount ACL with Alice, Bob, and compute_signer subjects
  |      verifies hB1 = FHE.add(hB0, hMoved)
  |      creates B1 for hB1 with Bob user/grant/public_decrypt and compute_signer compute roles
  |
  +--> stores:
         AliceTokenAccount.balance_handle = hA1
         AliceTokenAccount.balance_acl_record = A1
         AliceTokenAccount.next_balance_nonce_sequence = 2
         BobTokenAccount.balance_handle = hB1
         BobTokenAccount.balance_acl_record = B1
         BobTokenAccount.next_balance_nonce_sequence = 2
```

The transfer amount handle ACL can be created through the signed input path or through token-level
random amount creation. Transfer authority is the sender owner; the separate payer only pays rent
and fees and never becomes transfer authority:

```text
  ACL domain key = confidential mint
  app account    = sender owner
  label          = "transfer_amount"
  subjects       = [compute_signer: compute/use]
  handle         = hX selected from SolanaInputProof.handles[]
```

For randomized token amounts, `confidential-token::create_random_amount` and
`confidential-token::create_random_bounded_amount` call ZamaHost `fhe_rand_and_bind` or
`fhe_rand_bounded_and_bind` with the same `(mint, owner, transfer_amount)` metadata, consume
`ConfidentialTokenAccount.next_amount_nonce_sequence`, and emit `RandomAmountCreatedEvent` for
token-aware indexers.

The token program rejects transfer amount handles whose FHE type byte is not the confidential
balance type (`euint64` in this PoC) and rejects amount ACL records outside the
`(mint, owner, transfer_amount)` scope. Payer-supplied rent does not grant use, transfer, or decrypt
authority.

The input ACL record uses an explicit test nonce sequence. It must not derive placement from the
handle bytes; handles are opaque even in tests.

The mock instruction deliberately trusts the caller-supplied input handle and remains test glue for
legacy listener tests. The production-shaped path is `zama_host::verify_input_and_bind`:

```text
SolanaInputProof
  handles[]       = ordered encrypted-input handles certified by the verifier
  handle_index    = selected handle for this bind
  user            = user associated with the encrypted input
  app_account     = app account scope
  acl_domain_key  = app ACL domain scope
  extra_data      = opaque verifier transcript/proof metadata

SolanaInputBindIntent
  output_nonce_key
  output_nonce_sequence
  output_acl_domain_key
  output_app_account
  output_encrypted_value_label
  output_subjects
  output_public_decrypt

signed message =
  canonical input_proof_message bytes over
  (host program id, chain id, proof fields, bind intent fields)
```

The transaction must place native Ed25519 verifier instructions immediately before the host
instruction. `verify_input_and_bind` inspects the instructions sysvar, requires a quorum from the
active `HostConfig.input_verifier_set` over the canonical verifier-set-bound message, rejects
duplicate signer counts, checks that `handles[handle_index]` equals the requested input handle, and
then writes the same canonical ACL record as the mock path. Input handles must carry the host chain
id, a supported FHE type byte, the handle version byte, and the proof index in byte 21.

Remaining production work is real proof/transciphering validation by the external input verifier
service. The Solana host now has the account and transaction witness shape needed for that service
instead of a generic allow API.

## Confidential Burn

`confidential-token::confidential_burn` mirrors ERC7984 `_burn` value semantics without exposing the
requested amount. The burn amount uses its own signed-input/random-amount label:

```text
ACL domain key = confidential mint
app account    = owner
label          = "burn_amount"
subjects       = [compute_signer: compute/use]
```

The burn graph is all-or-zero:

```text
hOk              = FHE.ge(old_balance, burn_amount)
hDebitCandidate  = FHE.sub(old_balance, burn_amount)
new_balance      = FHE.ifThenElse(hOk, hDebitCandidate, old_balance)
hBurned          = FHE.sub(old_balance, new_balance)
new_total_supply = FHE.sub(old_total_supply, hBurned)
```

The holder balance output remains owned by the token account with owner and compute subjects. The
burned amount ACL is scoped to the token account and grants the owner public-decrypt authority so
the app can request disclosure of the actual burned amount. The encrypted total supply remains
scoped to `PDA("total-supply", mint)` and grants only compute/use to the compute signer.

Redeem/unwrap is intentionally two-phase because the KMS certificate cannot exist before the burn
transaction finalizes:

```text
confidential_burn
  -> rotates encrypted balance and encrypted total supply
  -> emits ConfidentialBurnEvent with hBurned

off-chain Gateway/KMS public decrypt
  -> certifies (token program id, mint, hBurned, cleartext burned amount)

redeem_burned_amount
  -> verifies burned-amount ACL public-decrypt release, sealed material,
     and native Ed25519 KMS certificate
  -> initializes PDA("burn-redemption", mint, hBurned)
  -> transfers cleartext underlying SPL from the vault to the owner destination
```

The redemption marker makes the cleartext vault transfer one-shot for each burned handle.

### Operators

Operator/delegated-transfer APIs are intentionally removed from the Solana token surface. The
production API keeps one transfer authority model: the token-account owner signs the transfer, while
an independent payer may fund rent and fees. This is a deliberate simplification and no longer aims
to mirror ERC7984 operator parity.

### Receiver Hook Interface

> **SUPERSEDED (issue #1593).** The transfer-and-call callback flow described in this section
> (`confidential_call_transfer_receiver` / `confidential_prepare_transfer_callback` /
> `confidential_finalize_transfer_callback`) and the `confidential-token-receiver` program were
> **removed**. Solana doesn't need the token to call receivers back: signer authority propagates
> through CPI, so a receiving app drives its own atomic `deposit` that CPIs `confidential_transfer`
> (user signs once — no operator, no callback, no refund leg). See the `confidential-deposit-app`
> reference program. The text below is retained for historical context only.

`confidential_call_transfer_receiver` invokes the caller-supplied receiver program with
caller-supplied instruction data and remaining accounts. The token program clears Solana return data
before the CPI and requires the receiver program to be the return-data writer.

External receiver programs should use `confidential-token-receiver-sdk`; the token crate re-exports
the same ABI for compatibility:

```text
TransferReceiverReturn::encode()
transfer_receiver_return_data(...)
set_transfer_receiver_return_data(...)

layout:
  magic = "ZAMA_CT_RECEIVER_RET_V0"
  mint
  from_token_account
  to_token_account
  sent_handle
  sent_acl_record
  callback_success_handle
  callback_success_acl_record
```

The token program decodes the same fixed layout with `TransferReceiverReturn::decode` and rejects
wrong length, wrong magic, wrong return program, or any mismatched field. The callback-success ACL
must already be scoped to the recipient owner and allow the compute signer to use the encrypted bool.
The hook return data proves only that the named receiver program *ran* and echoed back the exact
callback witness it was handed; in this PoC the sample receiver does **not** compute or attest the
encrypted success bit — it returns the caller-supplied `callback_success_handle` unchanged, so the
refund outcome is effectively chosen by whoever drives the call leg, not proven by the receiver's own
logic. This intentionally diverges from EVM `ERC7984._transferAndCall`, where `success` is derived by
`checkOnTransferReceived` actually evaluating the transfer. A production receiver must instead birth
or cryptographically attest the success bit itself (e.g. compute it via FHE in its own CPI to
ZamaHost) rather than echo it; the current sample receiver is a wiring stub, not a security model.
A successful hook call creates a `TransferReceiverHookCall` PDA keyed by `(mint,
sent_handle)`, so a repeated hook attempt for the same transferred handle fails before the external
receiver CPI is invoked. `confidential_prepare_transfer_callback` requires that marker and checks it
against the sent amount and callback-success witness before computing any refund. The later
callback-settlement PDA separately tracks refund preparation and finalization.

The `programs/confidential-token-receiver` sample program is intentionally small and exists as an
external-program reference for this ABI. It depends on the SDK crate, not the token program crate,
and is loaded in Mollusk receiver-hook tests instead of relying only on the token program's test
receiver endpoint.

## cUSDC Wrapper

`confidential-token::wrap_usdc` models a wrapper first, not a Token-2022 extension.

```text
Alice USDC token account
  |
  | SPL transfer_checked(amount)
  v
cUSDC vault token account

vault owner = PDA("vault-authority", ConfidentialMint)
```

Then the confidential balance is updated:

```text
wrap_usdc(amount)
  |
  +--> build one zama_fhe::EvalPlan
  |      step 0: trivial_encrypt(amount) -> transient hDeposit
  |      step 1: add(hA0, hDeposit) -> durable hA1 balance output
  |      step 2: add(hS0, hDeposit) -> durable hS1 total-supply output
  |
  +--> plan.resolve_accounts(...)
  |      orders current balance, total-supply, and output ACL accounts by plan role
  |      validates duplicate, unexpected, missing, and readonly accounts before CPI
  |      validates required app/output authorities before CPI
  |
  +--> CPI zama-host::fhe_eval(combined frame)
         checks operand ACL records
         verifies hDeposit, hA1, and hS1 against the host formulas
         emits the eval compute events
         creates durable output ACL records for hA1 and hS1
```

The deposit amount is public in this slice because wrapping an underlying token starts from a known
SPL amount. The wrapper keeps the amount as an instruction-local eval transient, so it does not
create a durable amount ACL record just to feed the two additions. Tests that need an encrypted-input
shape can use the `mock_input_verified_and_bind(...)` short-circuit or the signed
`verify_input_and_bind(...)` path, depending on whether the test needs verifier transaction
witnesses. Production app flows should use `verify_input_and_bind(...)` with the real ZKPoK/input
verifier or transciphering boundary.

The encrypted total supply is stored on `ConfidentialMint` as a handle plus current ACL record. The
mint remains the ACL domain, while `PDA("total-supply", mint)` signs as the app account authority for
total-supply ACL records. This mirrors ERC7984's `_totalSupply` pointer without forcing the mint
account itself to be a PDA.

## App-Side FHE Helper

The confidential token program intentionally does not call raw generated ZamaHost CPI bindings from
business logic. It goes through:

```text
solana/programs/confidential-token/src/fhe.rs
```

Current helper surface:

```text
zama_fhe::EvalBuilder
  -> starts from a validated EvalContextId and EvalAppAuthority
  -> builds an opaque EvalPlan from typed encrypted values, typed scalar RHS values,
     transient outputs, and DurableOutput targets
  -> composes binary ops, ternary if-then-else, trivial encrypt, random, and verified
     input bind steps in one fhe_eval frame
  -> derives durable output nonce keys / ACL record PDAs through DurableOutputBirth
     and records dynamic account requirements without exposing host indices
  -> exposes EvalAppAuthority plus per-output authority requirements

zama_fhe::AccessPolicy
  -> validated constructors for owner, compute, and use-only AccessSubject entries
  -> rejects empty, duplicate, default, or unsupported subject policies before CPI

zama_fhe::DurableLabel / DurableSlot
  -> keeps app-domain encrypted field labels distinct from raw host byte arrays

zama_fhe::BoundedU64UpperBound
  -> validates token bounded-random upper bounds before the dedicated host CPI
  -> provides power_of_two(u64), full_width(), and an explicit from_be_bytes(...) wire parser

fhe::eval(...)
  -> CPI zama-host::fhe_eval(...)
  -> executes an SDK-built EvalPlan through plan.resolve_accounts(...)
  -> preflights every dynamic account and output authority required by the plan by purpose/pubkey
  -> selects and orders required accounts from candidate lists instead of mirroring host indices
  -> rejects duplicate, extra, missing, or readonly dynamic accounts and output authorities before CPI
     with specific token errors instead of a generic eval-plan failure
  -> derives compute signer and app-account authority signer seeds from typed authority values

fhe::DurableOutput
  -> binds one unused ACL account to the exact durable slot it may create
  -> verifies the born ACL account metadata before returning the output handle

fhe::rand_bounded_u64(...)
  -> CPI zama-host::fhe_rand_bounded_and_bind(...)
  -> reuses DurableOutput and BoundedU64UpperBound instead of raw nonce/subject fields
```

The helper module keeps host indices, `FheBinaryOpCode`, raw nonce metadata, and generated CPI
account assembly out of token business logic.

The normal app-facing SDK does not expose raw host eval args or raw account ordering. Host-shaped
escape hatches live behind the `raw-host-api` feature under `zama_fhe::advanced` for adapters and
tests that deliberately need ABI-level pieces.

## User Decrypt Shape

The PoC keeps the RFC016-style split:

```text
Signed by Alice:
  user_pubkey = Alice
  reencryption_public_key = pkR
  allowed_acl_domain_keys = [cUSDCMint]
  validity
  extra_data

Unsigned handle entry:
  handle = hA1
  owner_pubkey = Alice
  acl_record_pubkey = A1
```

KMS-style verification:

```text
1. Verify Alice signed the top-level authorization.
2. Read acl_record_pubkey.
3. Verify the ACL account is owned by zama-host.
4. Verify:
     expected_nonce_key = H(record.acl_domain_key, record.app_account, record.encrypted_value_label)
     acl_record_pubkey == PDA("acl-record", expected_nonce_key, record.nonce_sequence)
     record.acl_domain_key is in allowed_acl_domain_keys
     record.handle == handle
     record.subjects contains owner_pubkey with ACL_ROLE_USE
     or PDA("acl-permission", acl_record, owner_pubkey) witnesses overflow access
```

Visual version:

```text
Alice signature
  says: "I allow decrypts for cUSDCMint during this validity window"
       |
       v
Handle entry
  says: "decrypt hA1 for Alice using ACL record A1"
       |
       v
ACL record A1
  proves on-chain:
    A1 belongs to cUSDCMint / AliceTokenAccount / balance / sequence 1
    A1 stores hA1
    A1 allows Alice for use/user decrypt
```

For the current balance, the frontend does not need to search:

```text
read AliceTokenAccount:
  balance_handle = hA1
  balance_acl_record = A1

request carries:
  handle = hA1
  acl_record_pubkey = A1
```

For older handles, the request must carry an older ACL record pubkey that the app observed or indexed from prior transactions. KMS does not guess or scan; it reads the provided account and verifies the stored fields.

## Public Decrypt Shape

Public decrypt is a durable flag on the canonical ACL record.

```text
Alice already allowed on A1 for hA1
  |
  v
zama-host::allow_for_decryption(handle = hA1, authority = Alice, acl_record = A1)
  checks:
    A1 stores hA1
    A1 subjects includes Alice with ACL_ROLE_PUBLIC_DECRYPT
  writes:
    A1.public_decrypt = true
```

Host-owned handle birth paths initialize this flag as `false`. Public decrypt release must happen
after ACL record birth through `allow_for_decryption`, so the release is subject to the public
decrypt authority checks and emits the dedicated host release event.

The public decrypt request does not need a user signature:

```text
request carries:
  handle = hA1
  acl_record_pubkey = A1
  material_commitment_pubkey = M1
```

KMS-style verification:

```text
1. Read acl_record_pubkey.
2. Verify the ACL account is owned by zama-host.
3. Recompute:
     expected_nonce_key = H(record.acl_domain_key, record.app_account, record.encrypted_value_label)
     acl_record_pubkey == PDA("acl-record", expected_nonce_key, record.nonce_sequence)
4. Verify:
     record.handle == handle
     record.public_decrypt == true
5. Read material_commitment_pubkey.
6. Verify:
     material_commitment_pubkey == PDA("handle-material", host_config, acl_record_pubkey)
     material.account is owned by zama-host
     material.acl_record == acl_record_pubkey
     material.handle == handle
     material.state == COMMITTED
     material.material_commitment_hash matches the decoded key/digest fields
     acl_record.material_commitment == material_commitment_pubkey
     acl_record.material_commitment_hash == material.material_commitment_hash
     acl_record.material_key_id == material.key_id
```

This is separate from ordinary `allow`.

```text
allow subject on handle
  -> subject can use the handle according to its stored role flags

allow_for_decryption on handle
  -> anyone can request public decrypt for that handle
```

Compute-only subjects cannot flip `public_decrypt`; negative Mollusk tests cover this case. ACL
membership and `public_decrypt` are authorization state, while `HandleMaterialCommitment` plus the
ACL record's sealed material fields are decryptability state. Native-v0 KMS admission needs both
sides to agree.

## Disclosure Proof Shape

`confidential-token` now has the token-local equivalent of ERC7984
`discloseEncryptedAmount`: `disclose_balance` and `disclose_amount` emit cleartext events only when
the disclosed ACL record has already been released for public decrypt, a finalized
`DisclosureRequest` witness matches the handle/material/verifier-set context, and the preceding
native Ed25519 verifier instructions prove a quorum from the mint's disclosure verifier set.

`disclose_amount` is limited to token amount labels: wrapped amounts, transfer inputs, burn inputs,
burned amounts, transferred amounts, and callback refund amounts. Current balance handles must use
the balance disclosure path, and total-supply/callback-success handles are not accepted as
disclosable token amounts.

The v2 signed message binds:

```text
token_program_id
host_program_id
host_config
chain_id
mint
mode
verifier_set
verifier_set_version
request_account
request_hash
acl_record
material_commitment_hash
material_key_id
handle
cleartext_amount_le_u64
```

This does not replace the Gateway/KMS request protocol. It is the Solana-side app verification
surface for a KMS response certificate once the Gateway request path carries native Solana
request/ACL/material witnesses. `redeem_burned_amount` uses a separate redemption verifier set and
certificate domain that also binds the owner, token account, underlying mint, destination owner, and
destination account. Negative Mollusk tests cover missing public release, missing material, unsealed
material, mismatched cleartext messages, and wrong verifier-set signatures.

## Operator Encoding Notes

`zama-host::fhe_binary_op` supports the same encrypted/scalar split as EVM-style binary operators:

```text
scalar = false:
  lhs is a handle
  rhs is a handle
  host checks both lhs_acl_record and rhs_acl_record
  host requires rhs type == lhs type

scalar = true:
  lhs is a handle
  rhs is plaintext scalar bytes
  host checks lhs_acl_record only
  rhs_acl_record may be a dummy account and is not deserialized
```

Add/Sub require the LHS handle type to match the declared output type. Ge requires a numeric LHS
handle and a Bool output. This mirrors the EVM executor rule that Add/Sub results inherit the LHS
type while comparison results are Bool.

The current confidential-token transfer/wrap/burn flows only use encrypted/encrypted binary ops. The
host-level scalar path is covered by Mollusk tests so future token helpers can pass `scalar = true`
without changing ZamaHost.

When adding ternary operators, Solana host events must preserve the EVM `scalarByte` convention.

```text
scalarByte is a bitmask of scalar arguments.
Index from the right-most argument.
Set bit i to 1 iff that argument is plaintext scalar.
No argument is implicit, even if it is always scalar today.
```

Examples:

```text
op(arg2, arg1, arg0)
  arg0 scalar => 0x01
  arg1 scalar => 0x02
  arg2 scalar => 0x04

mulDiv(lhs, rhs, divisor)
  enc x enc x scalar    => 0x01
  enc x scalar x scalar => 0x03
```

This is not an ACL rule. It is an event/worker compatibility rule so Solana compute events remain interpretable by the same coprocessor semantics as EVM events.

## Listener And Worker Flow

The Solana adapter lives in:

```text
coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
```

It maps typed Solana events into the existing coprocessor shape:

```text
Solana transaction signature
  |
  | sha256(signature)
  v
existing 32-byte transaction_id

Anchor self-CPI event bytes
  |
  v
SolanaHostEvent
  |
  v
TfheContractEvents / ACL allowance
  |
  v
LogTfhe
  |
  v
listener_db.insert_tfhe_event(...)
```

When Solana ACL events appear in the same transaction as TFHE events, the adapter does not treat the
log payload alone as sufficient to mark the TFHE output allowed. It inserts the TFHE row as pending
and schedules finalized account fetches for the ACL/material/request witnesses; finalized account
data is what later proves decryptability and allowance.

```text
tx:
  FHE.add(h0, hDeposit) -> h1
  creates/binds ACL record allowing h1

DB:
  finalized_account_fetches(acl/material/request witness PDAs)
  computations(output_handle = h1, is_allowed = false)
  witness rows become usable only after finalized fetch completion
```

Future Solana poller boundary:

```text
Only feed events emitted by the configured zama-host program.
The adapter must not treat arbitrary bytes as trusted host events.
```

## Worker-Backed E2E

The ignored worker tests prove this is not only a Solana program mock.

Real encrypted input transfer:

```text
Enc(125) = hA0
Enc(20)  = hB0
Enc(100) = hX

LiteSVM confidential_transfer(hX)
  -> emits FHE.ge(hA0, hX) -> hOk
  -> emits FHE.sub(hA0, hX) -> hDebitCandidate
  -> emits FHE.ifThenElse(hOk, hDebitCandidate, hA0) -> hA1
  -> emits FHE.sub(hA0, hA1) -> hMoved
  -> emits FHE.add(hB0, hMoved) -> hB1
  -> creates output ACL records for hA1/hMoved/hB1
  -> emits output ACL events for hA1/hMoved/hB1

host-listener::solana_adapter
  -> inserts computations + allowed handles

tfhe-worker
  -> computes real ciphertexts for hA1/hMoved/hB1

test decrypt
  -> hA1 = 25
  -> hB1 = 120
```

Solana-born ciphertext transfer:

```text
LiteSVM emits:
  initialize_token_account -> trivial_encrypt_and_bind(0) -> hA0
  initialize_token_account -> trivial_encrypt_and_bind(0) -> hB0
  verify_input_and_bind(100) -> hX

tfhe-worker
  -> creates real ciphertexts for hA0, hB0, hX

LiteSVM confidential_transfer(hX)
  -> emits FHE.ge(hA0, hX) -> hOk
  -> emits FHE.sub(hA0, hX) -> hDebitCandidate
  -> emits FHE.ifThenElse(hOk, hDebitCandidate, hA0) -> hA1
  -> emits FHE.sub(hA0, hA1) -> hMoved
  -> emits FHE.add(hB0, hMoved) -> hB1
  -> creates output ACL records for hA1/hMoved/hB1

test decrypt
  -> hA1 = 25
  -> hB1 = 120
```

Random ciphertext creation:

```text
LiteSVM emits:
  fhe_rand_and_bind(Uint8, nonce metadata) -> hRand + ACL record
  fhe_rand_bounded_and_bind(upper_bound, Uint8, nonce metadata) -> hRandBounded + ACL record
  create_random_bounded_amount(Transfer, upper_bound) -> hTokenRand + ACL record

tfhe-worker
  -> creates real random ciphertexts for hRand, hRandBounded, and hTokenRand

test decrypt
  -> hRand is a Uint8 plaintext
  -> hRandBounded is below upper_bound
  -> hTokenRand can drive a confidential_transfer through the existing transfer amount ACL checks
```

## Behavior Tests

Use this PoC like diff testing against EVM behavior.

Current tested invariants:

```text
ACL records are not derived from handles.
  A computed handle can be stored after the transaction starts.

The app account must sign ACL writes.
  A caller cannot create ACL for someone else's token account.

The host op enforces compute ACL before event emission.
  Wrong current ACL or wrong amount ACL rejects the transfer.

User decrypt checks signed authorization plus on-chain ACL state.
  Changing allowed_acl_domain_keys fails.
  Signing as Bob for Alice fails.
  Passing the wrong ACL record fails.
  Passing the wrong handle fails.

Public decrypt is separate from compute authority.
  A subject with compute/use only cannot enable public decrypt.

Persistent grants require grant authority and support overflow permission PDAs.
  The ninth subject is stored in PDA("acl-permission", acl_record, subject).

Mock input and test event shims are gated by HostConfig authorities.
  They are not generic protocol entrypoints.

Solana events can enter the existing coprocessor DB path.
  Worker tests compile against the Solana event adapter and use real ciphertexts.

KMS connector does not silently apply EVM ACL checks to Solana handles.
  host chain config with chain_kind = "solana" returns a recoverable fail-closed error.
```

When adding a feature, prefer a test shaped like:

```text
EVM invariant:
  name the behavior we already rely on

Solana equivalent:
  name the account / PDA / CPI shape that must preserve it

Negative case:
  show the wrong account, wrong signer, wrong subject, or wrong ACL fails
```

## Budget Snapshot

Current `confidential_transfer` Mollusk snapshot is tracked in:

```text
solana/runtime-tests/tests/token_mollusk.rs
  mollusk_confidential_transfer_rotates_accounts_and_acl_records
```

The important qualitative points:

```text
transfer uses one output ACL record per changed balance account plus one transferred-amount ACL
each balance output ACL stores both subjects: user + compute_signer
the transferred-amount ACL stores sender, recipient, and compute_signer subjects
max CPI depth remains 3 in the tested direct token -> zama-host -> event-CPI path
```

Open optimization question: event transport mode.

This PoC uses Anchor `emit_cpi!` for host events. That makes events typed and easy to decode from
self-CPI instruction data, including in runtime tests and listener code.

The cost is visible:

```text
confidential-token
  -> zama-host
      -> zama-host event self-CPI
```

Do not switch production-critical Zama events to plain `emit!` logs just to save CPI depth; provider
log truncation is not acceptable for this use case. Also do not assume the current `emit_cpi!` shape
scales to every production path, because each emitted event spends one nested CPI frame and can push
complex call graphs into Solana's hard CPI depth limit. The current production direction is the
Yellowstone/Geyser listener described in [Event Transport Note](#event-transport-note), using
transaction/account streams for indexing and account-state witnesses for authorization.

## Commands

Solana program build and runtime tests:

```bash
cd solana
bash scripts/check-zama-host-idl.sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

If the ZamaHost Anchor IDL intentionally changes, refresh the listener snapshot first:

```bash
cd solana
bash scripts/sync-zama-host-idl.sh
```

Rustdoc check for the two PoC programs:

```bash
cd solana
cargo doc --no-deps -p zama-host -p confidential-token
```

Worker test compile check:

```bash
cd coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p tfhe-worker solana_user_decrypt_acl_invariants_match_evm_semantics --no-run
```

Running the ignored worker-backed tests requires the usual coprocessor Postgres/test harness setup.

`Cargo.lock` is part of this PoC. It keeps the Anchor/Solana dependency graph compatible with the Cargo version embedded in the local Solana SBF toolchain.
