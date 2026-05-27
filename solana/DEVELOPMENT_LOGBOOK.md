# Solana PoC Development Logbook

Read this file first at the start of every development loop. Keep it compact:

- Soft cap: 180 lines.
- Hard cap: 220 lines.
- If the hard cap is reached, collapse older checkpoints into one-line summaries before adding new
  detail.
- `DEVELOPMENT_ISSUES.md` is the compact issue ledger. Do not duplicate long issue narratives here.

## Resume Protocol

1. Read this file, then `DEVELOPMENT_ISSUES.md`.
2. Run `git status --short` from `/home/nicolas/zama/fhevm`.
3. Do not revert unrelated user or generated changes.
4. After any Anchor program change, run `anchor build --ignore-keys` before LiteSVM runtime tests.
5. Keep Cargo verification mostly sequential; shared target dirs can cause build-lock waits.
6. If a new issue is confirmed, update both progress files before finalizing the pass.
7. Do not mark the overall goal complete until a requirement-by-requirement audit proves both
   `zama-host` and `confidential-token` match the referenced specs and EVM contracts.

## Current State

Date: 2026-05-27

Active objective:

- Finish and validate `solana/programs/zama-host`.
- Finish and validate `solana/programs/confidential-token`.
- Align against RFC 024, ACL storage spec/rationale, transient-equivalent notes, `ERC7984.sol`,
  `fhevm/host-contracts`, and `fhevm/gateway-contracts`.

Latest completed issue:

- Issue 127: confidential-token receiver hooks must be paired with a matching same-transaction
  transfer instruction, so normal transfers cannot be retroactively settled as transfer-and-call.

Latest full runtime result:

- Historical Issues 90-114 covered host config, event, native-v0, response, tx-sender, and token
  regressions; see `DEVELOPMENT_ISSUES.md` for the collapsed ledger.
- After the Yellowstone source-validation note, `cargo test -p host-listener
  solana_adapter::tests::` passed with 14 tests.
- After Issue 115, `anchor build --ignore-keys` passed and
  `cargo test -p zama-solana-runtime-tests --test host_events` passed with 155 tests.
- After Issue 116, focused entries-hash checks passed, `cargo test -p kms-worker solana_` passed
  with 102 filtered Solana tests, and `cargo test -p kms-worker --lib` passed with 121 tests.
- After Issue 117, focused extra-data live prefetch checks passed, `cargo test -p kms-worker
  solana_` passed with 105 filtered Solana tests, and `cargo test -p kms-worker --lib` passed with
  124 tests.
- After Issue 118, `anchor build --ignore-keys` passed and
  `cargo test -p zama-solana-runtime-tests --test host_events` passed with 156 tests.
- After Issue 119, `anchor build --ignore-keys` passed and
  `cargo test -p zama-solana-runtime-tests --test host_events` passed with 157 tests.
- After Issue 124, `anchor build --ignore-keys` passed,
  `cargo test -p zama-solana-runtime-tests --test host_events` passed with 164 tests, and
  `cargo test -p host-listener solana_adapter::tests::` passed with 14 tests.
- After Issue 125, `anchor build --ignore-keys` passed,
  `cargo test -p zama-solana-runtime-tests --test host_events` passed with 165 tests,
  `cargo test -p kms-worker solana_` passed with 107 filtered Solana tests, and
  `cargo test -p kms-worker --lib` passed with 126 tests.
- After Issue 126, focused role-matrix KMS checks passed, `cargo test -p kms-worker solana_`
  passed with 110 filtered Solana tests, and `cargo test -p kms-worker --lib` passed with 129
  tests.
- After Issue 127, `anchor build --ignore-keys` passed and
  `cargo test -p zama-solana-runtime-tests --test host_events` passed with 166 tests.

Current in-progress work:

- No active partial patch from the progress files.
- Yellowstone source validation refreshed the README event-transport note with slot
  finality/replay handling; production listener integration remains an external/product item.
- Continue the broader spec/contract audit for the next concrete mismatch. If a new candidate is
  confirmed, document it in both progress files before finalizing the pass.

Next safe action:

1. Re-run the normal audit loop against RFC 024, ACL storage spec/rationale, transient-equivalent
   notes, `ERC7984.sol`, host contracts, and gateway contracts.
2. Keep new fixes narrow and verify with Anchor rebuild before LiteSVM tests.
3. Update this file and `DEVELOPMENT_ISSUES.md` before reporting any new resolved issue.

## Decisions To Preserve

- Events are indexing hints only. Authorization must come from host-owned account state and KMS-like
  witness verification.
- Idempotent no-op branches must not emit duplicate logical events.
- Optional accounts use exact omitted/zero semantics.
- Account metas are ABI. Reject arbitrary trailing metas and malformed unused fixed slots.
- Missing deny record means `denied == false` only for a canonical system-owned, non-executable,
  zero-data account.
- Deny-list is a grant/public-enable gate in the native-v0 profile, not a hard KMS deny-list unless
  a future profile explicitly adds that behavior.
- Self-transfer in confidential-token is a no-op, but fixed account ABI exactness still applies.
- Transient durable-output policies must preserve subject and role exactness from transient inputs.

## Error-Prone Paths

- Do not treat full `host_events` as completion of the full PoC objective.
- Do not ignore stale SBF artifacts after program changes.
- Do not retry LiteSVM injection of system-owned executable empty accounts; LiteSVM rejects that
  setup before program execution.
- Do not expand progress files with full command transcripts. Keep them capped and summarize.
- Host-listener event matching is exhaustive; syncing a new IDL event can require an explicit
  ignored branch even when the listener does not ingest that audit event.
- Connector account-layout constants are ABI-sensitive; host account fields such as
  `AclRecord::created_slot` must be mirrored in KMS witness decoders and fixture encoders.
- One-shot transient capability consume must prove a matching earlier top-level
  `create_transient_session` in the current transaction; same-slot expiry is only defense in
  depth.
- Native-v0 connector admission must reject unsupported profile fields even when caller-supplied
  hashes are internally consistent.
- HostConfig initialization must reject zero chain id and zero authority pubkeys; downstream
  instructions assume these profile fields are usable.
- Host-listener must reject unsupported ZamaHost event versions before mapping to legacy TFHE rows.
- Callback settlement is a split token flow; hook causality and recipient ACL binding must allow
  refund relayers without requiring a post-hook recipient signature.
- Confidential mints must not store a zero KMS verifier authority because disclosure certificates
  later treat that field as the root Ed25519 verification key.
- User-decryption delegation delegates must be concrete nonzero pubkeys; `[0xff; 32]` is reserved
  only as the wildcard app-context row key.
- Native-v0 public requests have no replay key, but they still must obey payload expiration slots
  and live max-validity policy.
- Native-v0 material source mode is registry value `1` for Solana-program material; `0` is
  unsupported.
- Native-v0 account witnesses must carry the RPC executable flag, reject executable accounts before
  account layout decoding, and bind the flag into witness hashes.
- Native-v0 KMS response signer config/certificate context must equal the accepted request
  `kms_context_id`, not just be internally self-consistent.
- Native-v0 response golden vectors must track request-hash fixture changes; response hashes bind
  the accepted request hash directly.
- Native-v0 KMS response verification must reject inconsistent accepted request state, including an
  accepted request hash that is not re-derived from the supplied request payload.
- Native-v0 KMS response verification must also reject accepted replay-key shapes that do not match
  the request mode and signed-request replay tuple.
- Native-v0 response storage must not persist a public constructed publication unless its route,
  response hash, payload, raw body, and certificate context are mutually consistent.
- Native-v0 verified-request admission helpers must reject replay keys whose signer does not match
  the signature signer and public accepted records with nonzero signers.
- Native-v0 response rows must not be orphaned from their originating native request row; the
  response table carries a request-hash foreign key to the request table.
- Native-v0 inline ACL subjects are decisive; a same-subject overflow witness must not override a
  missing inline role.
- Native-v0 ACL/material witness creation slots must not be newer than the accepted observed slot.
- Native-v0 response routes must re-encode to the signed extra-data hash before release/storage.
- Native-v0 request parser must reject malformed raw extra-data layout/context/hash before account
  fetch.
- Native-v0 live admission must reject static payload/ACL-program/domain/key/signature mismatches
  before RPC account fetch.
- Native-v0 signed entry hashes are prefetch guards; parser and live admission must reject
  `entries_hash` mismatches before RPC account fetch.
- Native-v0 raw extra-data is also a prefetch guard; parser and live admission must reject
  malformed layout, KMS-context mismatch, and raw hash mismatch before RPC account fetch.
- Confidential-token transfer-family paths must verify the full current balance ACL metadata, not
  only the token account's stored ACL pubkey.
- ZamaHost binary operation outputs must obey operator-specific FHE type rules: comparison outputs
  are Bool, while Add/Sub outputs are numeric types supported by the EVM executor surface.
- ZamaHost random type gates are operator-specific too: unbounded random excludes Uint160, while
  bounded random excludes Bool and Uint160.
- ZamaHost binary operations must validate encrypted operand metadata too: Add/Sub result type
  follows the LHS type, Ge returns Bool, and encrypted RHS operands must match the LHS type.
- Confidential-token `transfer_from` amount inputs are caller-scoped: direct transfers use the
  sender owner, while operator transfers require an operator-scoped amount ACL.
- Confidential-token amount disclosure is label-scoped; balance, total-supply, and callback-success
  ACLs must not be routed through `request_disclose_amount`.
- ZamaHost handle birth must not set the public-decrypt release bit; public decrypt release goes
  through `allow_for_decryption` after ACL birth.
- Existing ACL records admitted for public decrypt, compute, or transient handoff must carry
  supported handle metadata for the active host chain. Durable grant extension stays handle-opaque.
- User-decryption requests and delegation rows must reject `[0xff; 32]` as a delegate; the value is
  reserved for wildcard app-context delegation rows only.
- Gateway-PoC Solana user-decryption witnesses must enforce the same direct/delegated role matrix
  as native-v0 admission; ACL membership alone is not enough.
- Confidential-token receiver hooks are the transfer-and-call intent marker. A hook must follow the
  matching transfer instruction in the same transaction; a settled normal transfer is not
  retroactively refundable.

## Recent Checkpoints

- Issue 116: native-v0 parser and live admission now reject signed handle-entry hash mismatches
  before Solana RPC account fetch.
- Issue 117: native-v0 live admission now rejects malformed, context-mismatched, or hash-mismatched
  raw extra-data before Solana RPC account fetch.
- Issue 118: ZamaHost direct, durable-bind, and composed eval binary paths now reject
  operator/output-type mismatches before emitting or binding handles.
- Issue 119: ZamaHost unbounded random handle birth now rejects unsupported random result types such
  as Uint160 before creating ACL state.
- Issue 120: ZamaHost direct, durable-bind, and composed eval binary paths now reject mismatched
  encrypted operand/result type metadata before emitting or binding handles.
- Issue 121: confidential-token `transfer_from` now rejects owner-scoped amount ACLs when the
  active operator is the transfer authority.
- Issue 122: confidential-token amount disclosure now rejects non-amount ACL labels before public
  decrypt release.
- Issue 123: ZamaHost durable handle birth paths now reject `output_public_decrypt = true` so
  public decrypt release always goes through `allow_for_decryption`.
- Issue 124: ZamaHost public-decrypt release, compute admission, and transient handoff now reject
  existing ACL records whose stored handle has unsupported chain/version/type metadata.
- Issue 125: ZamaHost delegation, native-v0 KMS requests, and the Gateway-PoC witness path now
  reject the reserved wildcard app-context sentinel when supplied as a delegate pubkey.
- Issue 126: Gateway-PoC Solana user-decryption witnesses and shared delegation witness checks now
  reject invalid direct/delegated role equality and reserved app-context forms.
- Issue 127: confidential-token receiver hooks now require a matching immediately preceding
  transfer instruction in the same transaction before callback settlement can be prepared.

## Pruning Rule

When adding the next checkpoint:

- Keep only the latest 8-12 checkpoints here.
- Move older details into the one-line issue ledger, not this file.
- If a command failed and changed direction, keep a one-line "development issue encountered" note
  under the relevant issue in `DEVELOPMENT_ISSUES.md`.
