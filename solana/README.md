# Solana FHEVM PoC

This workspace is the Solana host-chain PoC for the `openzeppelin-solana-track` branch.

## Normative spec and doc map

| Document | Role |
|----------|------|
| [RFC 024: Solana ACL design](https://github.com/zama-ai/tech-spec/pull/448) | **Authoritative ACL / host design** (tech-spec). Update this when PoC proves or disproves a design choice. |
| This README | **Implementation handoff** — current code, tests, flows, and known gaps on this branch. |
| [OZ_GUIDE.md](./OZ_GUIDE.md) | **Short index for OpenZeppelin** — safe edit areas and contribution rules. |

When design and code diverge: decide in **RFC 024**, then sync this README (and OZ_GUIDE if boundaries change). Do not back-propagate Solana PoC learnings into other RFCs from this branch.

### Updating RFC 024 (iterate on the PR branch)

RFC 024 lives in **tech-spec PR #448**, branch `elias/rfc-024-solana-acl-design`. **Do not merge to tech-spec `main`** while the PoC is still moving — push RFC edits to the PR branch instead.

```text
1. Land PoC + tests on openzeppelin-solana-track (this repo)
2. Clone tech-spec, checkout elias/rfc-024-solana-acl-design
3. Edit rfcs/024-solana-acl-design.md (PoC validation, open questions, spec fixes)
4. Commit and push to origin/elias/rfc-024-solana-acl-design
5. Sync this README and OZ_GUIDE.md if boundaries or host surface changed
```

Link the RFC commit in fhevm PR descriptions when a design choice is promoted or rejected. Push validated text directly to [RFC 024 PR #448](https://github.com/zama-ai/tech-spec/pull/448).

It is meant to be a readable base for:

```text
1. getting familiar with the Solana end-to-end flow
2. adding PoC features without guessing the existing intent
3. testing Solana behavior against EVM-derived FHEVM invariants
```

The PoC does not settle the final Solana product shape. It makes one path real enough that ACL, event listening, worker compute, and user decrypt can be discussed from code and tests.

## Workspace layout (why these folders exist)

```text
solana/programs/zama-host
  Protocol-side host program. Owns FHE event emission and ACL enforcement.

solana/programs/confidential-token
  App-side PoC program (minimal confidential token / cUSDC wrapper).

solana/crates/zama-fhe
  App-facing FHE helper crate. Most apps should use `zama_fhe::execute`;
  advanced callers can build their own wrapper over the protocol types.

solana/tests
  PoC test crate (`zama-solana-tests`). Helpers and scenarios in `src/`; integration
  tests in `src/host_events.rs` (same crate — no nested `tests/` directory).
  tfhe-worker slow-path tests import the public helpers from this crate.

solana/crates/zama-host-events
  IDL codegen shared with host-listener (decode_anchor_cpi_event, event types).

coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  Maps typed Solana host events into the existing coprocessor DB model.

coprocessor/fhevm-engine/tfhe-worker/src/tests/solana_poc.rs
  Worker-backed end-to-end tests with real small TFHE ciphertexts.
  Imports zama-solana-tests instead of duplicating setup helpers.
```

## Handoff Path

Use this order when picking up the branch:

```text
1. Run it
   cd solana
   NO_DNA=1 anchor build --ignore-keys
   cargo test --workspace

2. Read the flow
   Start with "Global Flow", then "Confidential Transfer", then "User Decrypt Shape".
   The canonical product-shaped test is:
     confidential_token_e2e_wrap_transfer_and_decrypts_current_and_historical_balances

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

Use this guide as the central technical handoff document for the branch. The short
[`OZ_GUIDE.md`](./OZ_GUIDE.md) file is a role-specific index for OpenZeppelin follow-up work; keep
technical details here and link back from that file instead of duplicating large sections.

The PoC can still have breaking changes. The rule is:

```text
breaking changes are allowed
silent breaking changes are not allowed
```

When changing the ZamaHost CPI surface, ACL record shape, event payload, or decrypt-relevant fields,
update this guide and the affected tests in the same PR.

```text
Change ACL storage/checking?
  update zama-host + solana/tests/src/host_events.rs

Change token behavior?
  update confidential-token + solana/tests/src/host_events.rs

Change emitted host events?
  update zama-host + host-listener solana_adapter + worker tests

Change user decrypt semantics?
  update runtime KMS model tests + tfhe-worker solana_poc tests
```

For OpenZeppelin follow-up work, the safe area is:

```text
solana/programs/confidential-token
  Improve the confidential token flow against the current ZamaHost CPI surface.

solana/tests/src/host_events.rs
  Add behavior tests here first.
```

Current OZ handoff assumptions:

```text
1. Treat zama-host CPI as the protocol boundary.
2. Keep confidential-token SPL-like and app-owned.
3. Add LiteSVM tests before changing token behavior.
4. Do not derive account addresses from handles.
5. Do not add a separate ACL program unless the guild explicitly changes direction.
```

Avoid adding a separate Anchor workspace, a standalone ACL program, or TypeScript-only tests for core
authorization behavior unless the guild explicitly decides to change direction.

## PoC Progress

Use this checklist to see where the branch stands. Keep it updated when a PR changes the PoC surface.

### Working Now

- [x] Anchor workspace with `zama-host`, `confidential-token`, `tests`, and LiteSVM integration tests.
- [x] **Host surface is `execute_frame`-only** (`allow_acl_subjects`, `allow_for_decryption`, `assert_acl_record`). Legacy per-op / test-emit instructions were removed.
- [x] ZamaHost emits typed Anchor CPI events from real `execute_frame` operations (`FheBinaryOpEvent`, `TrivialEncryptEvent`, `FheRandEvent`, `AclAllowedEvent`, `AclPublicDecryptAllowedEvent`).
- [x] Host-listener decodes ZamaHost protocol events from the checked-in Anchor IDL snapshot (`host-listener/idl/zama_host.json`).
- [x] Solana host events normalize into the existing coprocessor DB event shape.
- [x] Worker-backed tests use real small TFHE ciphertexts for Solana-originated events (`tfhe-worker/src/tests/solana_poc.rs`, shared `tests` crate).
- [x] Confidential token: initialize mint/account, wrap SPL USDC, confidential transfer with balance handle rotation.
- [x] `BalanceHandleUpdatedEvent` for app/front-end indexers only (not consumed by host-listener).
- [x] Canonical E2E: wrap → transfer → current + historical user decrypt, plus negative authorization tests.
- [x] Compute-time ACL enforced inside `execute_frame` before FHE events are emitted.
- [x] Frame-local transient allow for intermediate handles; **durable ACL only via explicit `Allow` actions** in the frame.
- [x] `allow_acl_subjects` appends subjects on the canonical ACL record (no second record per handle).
- [x] Keyed-nonce ACL PDAs (handle stored in account, not used as seed) — matches RFC 024 Option C.
- [x] User decrypt modeled RFC-016-style (signed authorization + unsigned handle entries + ACL record verification).
- [x] `allow_for_decryption` / public decrypt flag; transient frame allow can authorize it inside `execute_frame`.
- [x] Scalar RHS: encrypted RHS requires ACL; scalar RHS does not.
- [x] Self-transfer is a no-op (no handle rotation, no output ACL records).
- [x] App code uses the shared `solana/crates/zama-fhe` helper (`zama_fhe::execute`) instead of assembling host CPI instructions in business logic.
- [x] Transfer amount inputs use `fhe.input_u64(...)`, which emits `InputVerifiedEvent` and checks a local, context-bound PoC proof.
- [x] **`fheRand`** via `FheFrameStep::Rand` + **`poc_demo_confidential_rand`** token demo with user-decrypt request roundtrip.
- [ ] **`fheRandBounded`** (op 27): not planned for current PoC scope.

### Partly Modeled

- [ ] KMS verification is modeled in Rust tests, not wired into the real KMS connector.
- [ ] Input handles use a local harness shape: deterministic handle material plus a PoC
      proof bound to `(handle, user, app account, ACL domain, fhe type, chain id)`.
      The real Rust SDK encryption / ZKPoK verifier or transciphering path is not
      implemented yet.
- [ ] ACL records are created already initialized through Anchor `init`; there is no stored
      `Empty -> Bound` enum in the PoC.
- [x] `allow_for_decryption` follows EVM semantics: any subject allowed on the handle may mark the
      handle as allowed for decryption.
- [ ] The subject list has a PoC capacity. Overflow/chunking is not designed yet.
- [ ] Historical handle lookup is assumed to be app/indexer responsibility for now.

### Missing Next

- [x] Add an `execute_frame` input step so apps can consume external input handles without
      pre-creating an amount ACL record.
- [ ] Replace the local input proof helper with real Rust SDK encryption / ZKPoK verification
      or the transciphering path.
- [x] Model transient allow semantics for same-instruction intermediate handles, including the EVM
      rule that transient allowance can authorize `allow_for_decryption`.
- [ ] Decide how subject overflow works without imposing a small protocol-level subject limit.
- [ ] Decide account cleanup, rent refund, compaction, and archival rules.
- [ ] Wire the KMS connector to verify Solana ACL records instead of using only test-local checks.
- [ ] Extend the canonical confidential token scenario when adding new token features, instead of
      creating a second product flow.
- [x] **`authorized_app_accounts`** on `execute_frame`; **`app_account_authority` removed** (see [RFC 024 PR #448](https://github.com/zama-ai/tech-spec/pull/448)).
- [ ] Keep [RFC 024](https://github.com/zama-ai/tech-spec/pull/448) aligned when the PoC proves or disproves a design choice (push updates to PR #448 branch).

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
      for frontend/app indexers only
  |
  | CPI
  v
zama-host program
  protocol state:
    ACL record PDAs
  generic protocol events exposed through the ZamaHost Anchor IDL:
    FheBinaryOpEvent
    TrivialEncryptEvent
    FheRandEvent
    AclAllowedEvent
    AclPublicDecryptAllowedEvent
    InputVerifiedEvent
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
  FheBinaryOpEvent / TrivialEncryptEvent / FheRandEvent
  AclAllowedEvent / AclPublicDecryptAllowedEvent

host-listener does not consume:
  InputVerifiedEvent (local harness registers input ciphertext material)
  confidential-token BalanceHandleUpdatedEvent
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

`FheOpcode::RandBounded` (op 27) is listed in the host enum but has no `execute_frame` step or IDL event yet.

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
  In the token PoC: "balance".

nonce key
  Hash of:
    "zama-acl-nonce-key-v1"
    ACL domain key
    app account
    encrypted value label

nonce sequence
  App-maintained monotonic counter for one nonce key.
  In the token PoC: ConfidentialTokenAccount.next_balance_nonce_sequence.

ACL record
  PDA owned by zama-host:
    PDA("acl-record", nonce_key, nonce_sequence)

subject
  Pubkey that is allowed by an ACL record.
  Examples: Alice for user decrypt, compute_signer for compute.

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
  -> CPI zama-host::execute_frame(...)
       checks operand ACL records
       computes and transiently allows intermediate output handles
       emits typed Anchor event
       initializes durable output ACL records only for explicit allow actions
```

The app program does not perform a separate pre-check for normal compute. It passes operand ACL
accounts to `zama-host`. `zama-host::execute_frame` rejects the operation before emitting FHE events
if any encrypted operand is not allowed for the compute signer. If checks pass, `zama-host` computes
frame-local result handles, emits the compute events, and creates canonical ACL records only for
results that the app explicitly allowed.

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
    subjects              = [Alice, compute_signer]
```

`subjects` is intentionally a plain allow-list. It does not store `Compute` or `UserDecrypt`
permission kinds. This matches the EVM ACL model: the same `(handle, subject)` authorization is used
by different verification paths. `zama-host::execute_frame` interprets `compute_signer` membership as
permission to compute, while the KMS/user-decrypt path interprets Alice membership as permission to
decrypt. Public decrypt is the separate `public_decrypt` flag on the ACL record.

For computed outputs, the address is known before execution but the handle is not:

```text
before tx:
  A7 address = PDA("acl-record", nonce_key, 7)
  h7 is unknown

during zama-host::execute_frame:
  h = H("FHE_comp", op, lhs, rhs, scalar, zama_host, chain_id, previous_bank_hash, timestamp)
  append handle metadata (marker, chain_id, fhe_type, version)
  Allow action writes that step result handle into the output ACL record
  A7.handle = h
  A7.subjects = [Alice, compute_signer]
```

The PoC stores the **frame step result handle** on the durable ACL record. App accounts point at that opaque handle; they never precompute it.

On-chain, `zama-host` fails closed with `PreviousBankHashUnavailable` when slot N−1 has no hash in the SlotHashes sysvar. The harness seeds a deterministic non-zero `previous_bank_hash` for every LiteSVM fixture so tests exercise the real sysvar path without per-test setup.

Creating a persistent ACL record has two shapes in this PoC.

First birth is owned by a trusted host path inside `execute_frame`:

```text
zama-host::execute_frame(...)
  input steps verify a context-bound external input proof and expose hX frame-locally
  trivial_encrypt / binary op steps produce frame-local handles
  explicit Allow actions create durable ACL records for chosen outputs

Future (not implemented):
  real Rust SDK ZKPoK verification or transciphering for external ciphertexts
```

PoC transfer amounts use a **local input verifier shortcut** until the real input path exists:

```text
test/local harness
  -> derives/registers amount handle + ciphertext material
  -> builds proof = H("zama-solana-poc-input-v0", handle, user, app account, ACL domain, fhe type, chain id)

confidential-token::confidential_transfer
  -> fhe::execute(...)
  -> fhe.input_u64(amount_handle, user, sender token account, proof)
  -> zama-host verifies the proof and emits InputVerifiedEvent
```

This is **not** the production input verifier. It exercises the VM/API shape the real Rust SDK
ZKPoK verifier or transciphering path should plug into.

Generic persistent grants extend the existing canonical ACL record. `allow_acl_subjects` requires
the caller to already be allowed on the same handle:

```text
zama-host::allow_acl_subjects(handle = h7, authority = Alice, acl_record = A7, subjects = [Bob])

requires:
  A7 is owned by zama-host
  A7 is the canonical PDA for its stored nonce fields
  A7.handle == h7
  A7.subjects contains Alice

effect:
  A7.subjects now also contains Bob
```

This prevents handle laundering:

```text
Mallory sees Alice's handle h7
Mallory tries to create M1 storing h7 and subjects = [Mallory]
zama-host rejects it unless Mallory is already allowed by a canonical ACL record for h7
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
the requested subject is in subjects[]
```

That keeps token-specific semantics inside the token program.

## Initial Balance Birth

The token program does not accept a caller-provided initial balance handle anymore.

```text
confidential-token::initialize_token_account(initial_balance = 125)
  |
  +--> CPI zama-host::execute_frame
  |      step: trivial_encrypt(125) -> hA0
  |      action: allow hA0 into A0
  |      app_account = AliceTokenAccount
  |      output ACL = A0
  |      subjects = [Alice, compute_signer]
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
  +--> one CPI zama-host::execute_frame
  |      steps:
  |        hA1 = FHE.sub(hA0, hX)
  |        hB1 = FHE.add(hB0, hX)
  |      checks:
  |        A0 stores hA0 and subjects includes compute_signer
  |        B0 stores hB0 and subjects includes compute_signer
  |        X  stores hX  and subjects includes compute_signer
  |      transiently allows compute_signer on hA1 and hB1 inside this frame
  |      explicit durable allows:
  |        creates A1 for hA1 with subjects = [Alice, compute_signer]
  |        creates B1 for hB1 with subjects = [Bob, compute_signer]
  |      emits:
  |        FHE.sub(hA0, hX) -> hA1
  |        FHE.add(hB0, hX) -> hB1
  |
  +--> stores:
         AliceTokenAccount.balance_handle = hA1
         AliceTokenAccount.balance_acl_record = A1
         AliceTokenAccount.next_balance_nonce_sequence = 2
         BobTokenAccount.balance_handle = hB1
         BobTokenAccount.balance_acl_record = B1
         BobTokenAccount.next_balance_nonce_sequence = 2
```

The amount handle is born outside the token account state and passed to `confidential_transfer`
with a local proof:

```text
hX = local_harness_input(amount = 100, nonce = 0)
proof = H("zama-solana-poc-input-v0", hX, Alice, AliceTokenAccount, cUSDCMint, Uint64, chain_id)

confidential_transfer(hX, proof)
  -> InputVerifiedEvent(hX)
  -> FHE.sub(hA0, hX) / FHE.add(hB0, hX)
```

`confidential_transfer` rejects the input unless the proof matches the exact handle, owner,
sender token account, mint ACL domain, FHE type, and PoC chain id. The input handle is only
frame-local unless a later frame action explicitly creates a durable ACL record.

The real design still needs real external input verification / transciphering behind this same
`fhe.input_u64(...)` app API.

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
  +--> one CPI zama-host::execute_frame
  |      steps:
  |        hDeposit = trivial_encrypt(amount)
  |        hA1 = FHE.add(hA0, hDeposit)
  |      checks:
  |        current balance ACL stores hA0 and allows compute_signer
  |      transiently allows compute_signer on hDeposit and hA1 inside this frame
  |      explicit durable allow:
  |        creates one output ACL record for hA1
  |        subjects = [Alice, compute_signer]
  |      emits:
  |        trivial_encrypt(amount) -> hDeposit
  |        FHE.add(hA0, hDeposit) -> hA1
```

The deposit amount is public in this slice because wrapping an underlying token starts from a known
SPL amount. The wrapper does not create durable ACL state for this temporary amount handle. It is a
frame-local result and only the new balance becomes durable through explicit `allow`.

Production input flows must use real Rust SDK ZKPoK verification or transciphering, not the local
proof hash helper.

## App-Side FHE API

The app-facing API is the shared helper crate:

```text
solana/crates/zama-fhe
```

Most app code should import it as `fhe` and use one entrypoint:

```text
fhe::execute(ctx, |fhe| { ... })
  -> collects a linear execution frame
  -> makes one CPI to zama-host::execute_frame

inside the closure:
  fhe.encrypted(...)            // durable input handle from an ACL record
  fhe.input_u64(...)            // external Uint64 input handle + local PoC proof
  fhe.trivial_encrypt_u64(...)  // frame-local handle
  fhe.add(...), fhe.sub(...)    // frame-local computed handle
  fhe.rand_u64()                // frame-local rand (Uint64 / type 5); see poc_demo_confidential_rand
  fhe.allow(...)                // explicit durable ACL record creation
```

Intermediate handles are ordinary bytes32 handles. They are not durable by default. `execute_frame`
transiently allows the frame's compute subject to use each step result, and `fhe.allow(...)` is the
point where an output ACL record is created.

Layering:

```text
zama_fhe::execute
  default app API; builds one frame and submits one host CPI

zama_fhe::protocol
  re-exported protocol IR types from zama-host for advanced callers building
  their own execute_frame wrapper

zama_host::execute_frame
  protocol entrypoint; use directly only when building a replacement for zama_fhe::execute
```

`zama-fhe` keeps frame IR details and generated CPI account assembly out of token business logic.
The internal `zama_fhe::frame::Builder` is intentionally not a public construction API yet; expose
it only if a second app proves that custom structured wrappers need it.

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
     record.subjects contains owner_pubkey
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
    A1 allows Alice
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
zama-host::allow_for_decryption(handle = hA1, acl_record = A1)
  checks:
    A1 stores hA1
    A1 subjects includes Alice
  writes:
    A1.public_decrypt = true
```

The public decrypt request does not need a user signature:

```text
request carries:
  handle = hA1
  acl_record_pubkey = A1
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
```

This is separate from ordinary `allow`.

```text
allow subject on handle
  -> subject can compute / user-decrypt through ACL checks

allow_for_decryption on handle
  -> anyone can request public decrypt for that handle
```

Design warning:

```text
This mirrors EVM ACL semantics.

Any subject that satisfies is_allowed(handle, subject) may call
allow_for_decryption for that handle.

On EVM, isAllowed includes both persistent allow and transient allow.
The Solana transient-allow design must preserve that behavior.
```

## Operator Encoding Notes

`zama-host::execute_frame` binary steps support the same encrypted/scalar split as EVM-style operators:

```text
scalar = false:
  lhs is a handle
  rhs is a handle
  host checks both lhs_acl_record and rhs_acl_record

scalar = true:
  lhs is a handle
  rhs is plaintext scalar bytes
  host checks lhs_acl_record only
  rhs_acl_record may be a dummy account and is not deserialized
```

The current confidential-token transfer/wrap flows only use encrypted/encrypted binary ops. The
host-level scalar path is covered by LiteSVM tests so future token helpers can pass `scalar = true`
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

When Solana ACL events appear in the same transaction as TFHE events, the adapter marks the TFHE output as allowed before inserting it. This mirrors the EVM listener shape:

```text
tx:
  FHE.add(h0, hDeposit) -> h1
  creates/binds ACL record allowing h1

DB:
  allowed_handles(h1, subject)
  computations(output_handle = h1, is_allowed = true)
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
  -> emits FHE.sub(hA0, hX) -> hA1
  -> emits FHE.add(hB0, hX) -> hB1
  -> creates output ACL records for hA1/hB1
  -> emits output ACL events for hA1/hB1

host-listener::solana_adapter
  -> inserts computations + allowed handles

tfhe-worker
  -> computes real ciphertexts for hA1/hB1

test decrypt
  -> hA1 = 25
  -> hB1 = 120
```

Solana-born ciphertext transfer (same flow; initial balances born via `execute_frame` in init):

```text
LiteSVM:
  initialize token accounts -> execute_frame trivial_encrypt + allow -> hA0, hB0
  local harness derives/registers input amount 100 -> hX + proof

tfhe-worker -> real ciphertexts for hA0, hB0, hX

LiteSVM confidential_transfer(hX, proof)
  -> InputVerifiedEvent(hX)
  -> FHE.sub / FHE.add -> hA1, hB1 + output ACL records

test decrypt -> hA1 = 25, hB1 = 120
```

Random ciphertext E2E demo (`poc_demo_confidential_rand`):

```text
confidential-token::poc_demo_confidential_rand
  -> fhe::execute(rand_u64 + durable Allow for owner)
  -> ZamaHost FheRandEvent + AclAllowedEvent
  -> app ConfidentialRandCreatedEvent { rand_handle, acl_record, ... }

CleartextBackend (fast local compute):
  ingest tx CPI events -> decrypt(rand_handle) == cleartext_rand_value(seed, 64)

User decrypt request (frontend-shaped, RFC-016 PoC model):
  signed_confidential_rand_user_decrypt_request(fixture, owner, handle, acl_record)
    authorization.user = owner
    authorization.allowed_acl_domain_keys = [mint]
    handles[0] = { handle, owner, acl_record }
  kms_like_user_decrypt_check(svm, request)  // ACL-only gate; KMS connector TBD

Canonical test:
  confidential_token_e2e_rand_demo_encrypt_compute_and_user_decrypt_request
```

Worker path (#[ignore], ~7min): `solana_fhe_rand_creates_ciphertext_and_decrypts` — same rand birth through Postgres/tfhe-worker.

Not planned for this PoC: `fheRandBounded`. Rand worker test not in CI yet.

## Rand vs bounded rand (EVM reference only)

Both share the same global seed/counter on EVM and in this PoC (`fhe-rand-counter` PDA). They differ in **output range** and **handle op**:

| | **`fheRand`** (op 26) | **`fheRandBounded`** (op 27) |
|---|---|---|
| **Range** | Full FHE type width (e.g. u64 → `[0, 2⁶⁴−1]`) | Uniform in `[0, upperBound)` |
| **Extra input** | None | `upperBound` (must be power of two, ≤ type max) |
| **Types** | Bool + Uint8 … Uint256 | Uint8 … Uint256 (**no Bool**) |
| **PoC status** | Implemented | Not implemented |

Use bounded rand when you need “random index below N” with N a power of two (lottery buckets, shuffles). Use unbounded rand for full-width encrypted randomness.

## Behavior Tests

Use this PoC like diff testing against EVM behavior.

Current tested invariants:

```text
ACL records are not derived from handles.
  A computed handle can be stored after the transaction starts.

The app declares which slots receive durable Allow output.
  authorized_app_accounts must contain every Allow.app_account (E4).
  The app account must either sign directly, or the frame compute subject must be
  PDA("fhe-compute", acl_domain_key) for the app account owner program.

The host op enforces compute ACL before event emission.
  Wrong current ACL or wrong input proof rejects the transfer.
  Failed frames emit no TFHE or ACL events (A4).

Transient frame results do not carry to a later execute_frame (B3).

Balance ACL subjects are owner + compute_signer (I4).

Subject extension respects MAX_ACL_SUBJECTS (E4).

User decrypt checks signed authorization plus on-chain ACL state.
  Changing allowed_acl_domain_keys fails.
  Signing as Bob for Alice fails.
  Passing the wrong ACL record fails.
  Passing the wrong handle fails.

Solana events can enter the existing coprocessor DB path.
  Worker tests compile against the Solana event adapter and use real ciphertexts.

Rand birth uses a global counter; repeated Rand steps in one frame or across
  instructions in one transaction produce distinct seeds/handles (counter bumps).
  Unsupported fhe_type reverts before counter bump or event emission.
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

Current `confidential_transfer` LiteSVM snapshot is tracked in:

```text
solana/tests/src/host_events.rs
  confidential_transfer_budget_snapshot
```

The important qualitative points:

```text
transfer uses one output ACL record per changed balance account
each output ACL record stores both subjects: user + compute_signer
max CPI depth remains 3 in the tested direct token -> zama-host -> event-CPI path
```

Open optimization question: event emission mode.

This PoC uses Anchor `emit_cpi!` for host events. That makes events typed and easy to decode from self-CPI instruction data, including in runtime tests and listener code.

The cost is visible:

```text
confidential-token
  -> zama-host
      -> zama-host event self-CPI
```

Anchor `emit!` log events may be cheaper, but it changes listener and explorer decoding assumptions. Keep `emit_cpi!` for now because this slice optimizes for correctness and typed decoding. Revisit after the ACL and token model stabilize.

## Commands

Solana program build and runtime tests:

```bash
cd solana
NO_DNA=1 anchor build --ignore-keys
cargo test --workspace
```

Worker test compile check (requires built Solana programs in `solana/target/deploy/`):

```bash
cd solana && NO_DNA=1 anchor build --ignore-keys
cd ../coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p tfhe-worker solana_user_decrypt_acl_invariants_match_evm_semantics --no-run
```

Running the ignored worker-backed tests requires the usual coprocessor Postgres/test harness setup.

`Cargo.lock` is part of this PoC. It keeps the Anchor/Solana dependency graph compatible with the Cargo version embedded in the local Solana SBF toolchain.
