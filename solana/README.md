# Solana FHEVM PoC

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

solana/programs/confidential-token
  App-side PoC program.
  Models a minimal confidential token / cUSDC wrapper.
  Its local fhe.rs module is the current dev-facing wrapper around raw ZamaHost CPI calls.

solana/runtime-tests
  Fast LiteSVM tests for Solana accounts, PDAs, CPI, events, and ACL behavior.
  tests/support/fhe_runtime.rs adds a cleartext backend that consumes real ZamaHost events.

coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  Maps typed Solana host events into the existing coprocessor DB model.

coprocessor/fhevm-engine/tfhe-worker/src/tests/solana_poc.rs
  Worker-backed end-to-end tests with real small TFHE ciphertexts.
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

Use this guide as the central handoff document for the branch. Do not add a second guide unless the
team decides the branch layout itself has changed.

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
```

For OpenZeppelin follow-up work, the safe area is:

```text
solana/programs/confidential-token
  Improve the confidential token flow against the current ZamaHost CPI surface.

solana/runtime-tests/tests/host_events.rs
  Add behavior tests here first.
```

Avoid adding a separate Anchor workspace, a standalone ACL program, or TypeScript-only tests for core
authorization behavior unless the guild explicitly decides to change direction.

## PoC Progress

Use this checklist to see where the branch stands. Keep it updated when a PR changes the PoC surface.

### Working Now

- [x] Anchor workspace with `zama-host`, `confidential-token`, and LiteSVM runtime tests.
- [x] ZamaHost emits typed Anchor CPI events for real host operations and clearly named
      `test_emit_*` shims used by listener / worker tests.
- [x] Solana host events normalize into the existing coprocessor DB event shape.
- [x] Worker-backed tests use real small TFHE ciphertexts for Solana-originated events.
- [x] Confidential token can initialize a mint and token accounts.
- [x] Confidential token can wrap SPL-like USDC into a confidential balance handle.
- [x] Confidential token can transfer by rotating Alice and Bob balance handles.
- [x] Canonical confidential token scenario covers wrap, transfer, current decrypt, historical
      decrypt, and expected failures.
- [x] Compute-time ACL is enforced by `zama-host::fhe_binary_op` before event emission.
- [x] Computed output handles are born inside `zama-host`; token instructions pass output ACL
      record accounts, not caller-chosen output handles.
- [x] Generic persistent ACL binding requires an already-authorized source ACL record, matching the
      EVM rule that only an allowed caller can grant durable access.
- [x] Token account initialization creates the initial balance handle through a host-owned
      `trivial_encrypt_and_bind` path, not through caller-supplied handle binding.
- [x] Runtime tests include a cleartext FHE backend that consumes emitted ZamaHost events and checks
      the plaintext semantics of transfer and wrap flows.
- [x] Confidential token uses a small `fhe` helper module so app logic calls named FHE helpers
      instead of hand-assembling raw Anchor CPI calls.
- [x] Keyed-nonce ACL records avoid deriving Solana account addresses from opaque handles.
- [x] User decrypt is modeled with signed authorization plus ACL record verification.
- [x] Current and historical balance decrypt are both modeled when the relevant ACL record still
      exists.
- [x] Public decrypt is modeled through `allow_for_decryption`.
- [x] Negative tests cover wrong signer, wrong ACL record, wrong handle, wrong domain key, stale
      current ACL, and unauthorized public decrypt.

### Partly Modeled

- [ ] KMS verification is modeled in Rust tests, not wired into the real KMS connector.
- [ ] Input handles use an explicit `mock_input_verified_and_bind` mock short-circuit instead of a
      real Solana input verifier or transciphering path.
- [ ] `test_emit_fhe_rand` exists for worker-backed tests, but the final random-handle birth API is
      not designed yet.
- [ ] ACL records are born bound through Anchor `init`; the future predeclared
      `Empty -> Bound` account lifecycle is documented but not implemented.
- [ ] The subject list has a PoC capacity. Overflow/chunking is not designed yet.
- [ ] Historical handle lookup is assumed to be app/indexer responsibility for now.

### Missing Next

- [ ] Define and implement the real Solana input path for external encrypted inputs.
- [ ] Model transient allow semantics for same-transaction intermediate handles.
- [ ] Decide how subject overflow works without imposing a small protocol-level subject limit.
- [ ] Decide account cleanup, rent refund, compaction, and archival rules.
- [ ] Wire the KMS connector to verify Solana ACL records instead of using only test-local checks.
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
  |
  | CPI
  v
zama-host program
  protocol state:
    ACL record PDAs
  events:
    test_emit_trivial_encrypt (test shim)
    test_emit_fhe_rand (test shim)
    fhe_binary_op
    bind_acl_record
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

`test_emit_*` instructions are not protocol APIs. They emit typed events without proving or writing
the corresponding ACL record and exist only to keep listener / worker tests fast while the real
Solana input, trivial-encrypt, and random-handle birth paths are still being designed.

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
  -> CPI zama-host::fhe_binary_op(...)
       checks operand ACL records
       derives the output handle
       initializes the output ACL record
       emits typed Anchor event
```

The app program does not perform a separate pre-check for normal compute. It passes operand ACL
accounts and the predeclared output ACL record account to `zama-host`. `zama-host::fhe_binary_op`
rejects the operation before emitting the FHE event if either operand is not allowed for the compute
signer. If checks pass, `zama-host` derives the result handle and stores it in the output ACL record.

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
by different verification paths. `zama-host::fhe_binary_op` interprets `compute_signer` membership as
permission to compute, while the KMS/user-decrypt path interprets Alice membership as permission to
decrypt. Public decrypt is the separate `public_decrypt` flag on the ACL record.

For computed outputs, the address is known before execution but the handle is not:

```text
before tx:
  A7 address = PDA("acl-record", nonce_key, 7)
  h7 is unknown

during zama-host::fhe_binary_op:
  h7 = H("FHE_comp", op, lhs, rhs, scalar, zama_host, chain_id, previous_bank_hash, timestamp)
  A7.handle = h7
  A7.subjects = [Alice, compute_signer]
```

The PoC uses the previous slot hash when LiteSVM or the cluster exposes it. Local bootstrap tests can
fall back to zero when no prior slot hash exists; this is test glue, not the intended production
entropy source.

Creating a persistent ACL record has two shapes in this PoC.

First birth is owned by a trusted host path:

```text
zama-host::trivial_encrypt_and_bind(...)
  creates a trivial-encrypt handle and its first ACL record

zama-host::mock_input_verified_and_bind(...)
  mock short-circuit for future InputVerifier/transciphering birth

zama-host::fhe_binary_op(...)
  checks operand ACL records, derives the computed handle, and creates the output ACL record
```

Generic persistent grants are stricter. `bind_acl_record` requires the caller to already be allowed
on the handle being granted:

```text
zama-host::bind_acl_record(handle = h7, authority = Alice, authorizing_acl_record = A7, ...)

requires:
  A7 is owned by zama-host
  A7 is the canonical PDA for its stored nonce fields
  A7.handle == h7
  A7.subjects contains Alice
  app_account_authority.key == AliceTokenAccount
  app_account_authority.is_signer
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
  +--> CPI zama-host::trivial_encrypt_and_bind(125)
  |      app_account = AliceTokenAccount
  |      output ACL = A0
  |      subjects = [Alice, compute_signer]
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
  +--> CPI zama-host::fhe_binary_op(Sub)
  |      compute_subject = compute_signer
  |      checks:
  |        A0 stores hA0 and subjects includes compute_signer
  |        X  stores hX  and subjects includes compute_signer
  |      creates A1 and stores:
  |        hA1 = FHE.sub(hA0, hX)
  |        subjects = [Alice, compute_signer]
  |      emits FHE.sub(hA0, hX) -> hA1
  |
  +--> CPI zama-host::fhe_binary_op(Add)
  |      compute_subject = compute_signer
  |      checks:
  |        B0 stores hB0 and subjects includes compute_signer
  |        X  stores hX  and subjects includes compute_signer
  |      creates B1 and stores:
  |        hB1 = FHE.add(hB0, hX)
  |        subjects = [Bob, compute_signer]
  |      emits FHE.add(hB0, hX) -> hB1
  |
  +--> stores:
         AliceTokenAccount.balance_handle = hA1
         AliceTokenAccount.balance_acl_record = A1
         AliceTokenAccount.next_balance_nonce_sequence = 2
         BobTokenAccount.balance_handle = hB1
         BobTokenAccount.balance_acl_record = B1
         BobTokenAccount.next_balance_nonce_sequence = 2
```

The amount handle ACL is temporary mock glue. Today the tests use `mock_input_verified_and_bind` as an
explicit short-circuit for the future input verifier / transciphering boundary:

```text
ACL domain key = Alice
app account    = Alice
label          = "input"
subjects       = [compute_signer]
handle         = hX
```

This instruction deliberately trusts the caller-supplied input handle. It is also deliberately not
`bind_acl_record`: the first ACL record for an input handle must come from a trusted input path, not
from a generic grant API. The real design still needs the Solana equivalent of transient input
authorization from the input verifier path.

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
  +--> CPI zama-host::trivial_encrypt_and_bind(amount)
  |      creates amount ACL record and emits hDeposit
  |
  +--> CPI zama-host::fhe_binary_op(Add)
  |      checks:
  |        current balance ACL stores hA0 and allows compute_signer
  |        amount ACL stores hDeposit and allows compute_signer
  |      creates one output ACL record and stores:
  |        hA1 = FHE.add(hA0, hDeposit)
  |        subjects = [Alice, compute_signer]
  |      emits FHE.add(hA0, hDeposit) -> hA1
```

The deposit amount is public in this slice because wrapping an underlying token starts from a known
SPL amount. The wrapper now uses the host `trivial_encrypt_and_bind(...)` path so the amount handle
has durable ACL state before it is used by `fhe_binary_op`. Tests that need an encrypted-input shape
can use the `mock_input_verified_and_bind(...)` short-circuit, but app flows should eventually use
the final input path and the real ZKPoK/input verifier or transciphering boundary.

## App-Side FHE Helper

The confidential token program intentionally does not call raw generated ZamaHost CPI bindings from
business logic. It goes through:

```text
solana/programs/confidential-token/src/fhe.rs
```

Current helper surface:

```text
fhe::trivial_encrypt_u64(...)
  -> CPI zama-host::trivial_encrypt_and_bind(...)
  -> returns the host-born output handle stored in the output ACL record

fhe::binary_op(...)
  -> CPI zama-host::fhe_binary_op(...)
  -> checks operand ACL records inside ZamaHost
  -> returns the host-born output handle stored in the output ACL record
```

This helper is still token-local. If the shape survives the PoC, it can become the seed for a shared
Solana FHE app SDK.

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

## Operator Encoding Notes

The current PoC only models encrypted/encrypted binary ops, so scalar metadata is still simplified. When adding scalar or ternary operators, Solana host events must preserve the EVM `scalarByte` convention.

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

Solana-born ciphertext transfer:

```text
LiteSVM emits:
  trivial_encrypt_and_bind(125) -> hA0
  trivial_encrypt_and_bind(20)  -> hB0
  trivial_encrypt_and_bind(100) -> hX

tfhe-worker
  -> creates real ciphertexts for hA0, hB0, hX

LiteSVM confidential_transfer(hX)
  -> emits FHE.sub(hA0, hX) -> hA1
  -> emits FHE.add(hB0, hX) -> hB1
  -> creates output ACL records for hA1/hB1

test decrypt
  -> hA1 = 25
  -> hB1 = 120
```

Random ciphertext creation:

```text
LiteSVM emits:
  test_emit_fhe_rand(seed, Uint8) -> hRand

tfhe-worker
  -> creates a real random ciphertext for hRand

test decrypt
  -> hRand is a Uint8 plaintext
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

Solana events can enter the existing coprocessor DB path.
  Worker tests compile against the Solana event adapter and use real ciphertexts.
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
solana/runtime-tests/tests/host_events.rs
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

Worker test compile check:

```bash
cd coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p tfhe-worker solana_user_decrypt_acl_invariants_match_evm_semantics --no-run
```

Running the ignored worker-backed tests requires the usual coprocessor Postgres/test harness setup.

`Cargo.lock` is part of this PoC. It keeps the Anchor/Solana dependency graph compatible with the Cargo version embedded in the local Solana SBF toolchain.
