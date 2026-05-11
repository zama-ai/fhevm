# Solana FHEVM PoC

This workspace is the Solana host-chain PoC for the `openzeppelin-solana-track` branch.

It is meant to be a readable base for:

```text
1. getting familiar with the Solana end-to-end flow
2. adding PoC features without guessing the existing intent
3. testing Solana behavior against EVM-derived FHEVM invariants
```

The PoC is not trying to finish the Solana product shape yet. It is trying to make one host-chain path real enough that ACL, event listening, worker compute, and user decrypt can be reasoned about with code and tests.

## Where To Start

```text
solana/programs/zama-host
  Protocol-side host program.
  Owns FHE event emission and ACL enforcement.

solana/programs/confidential-token
  App-side PoC program.
  Models a minimal confidential token / cUSDC wrapper.

solana/runtime-tests
  Fast LiteSVM tests for Solana accounts, PDAs, CPI, events, and ACL behavior.

coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  Maps typed Solana host events into the existing coprocessor DB model.

coprocessor/fhevm-engine/tfhe-worker/src/tests/solana_poc.rs
  Worker-backed end-to-end tests with real small TFHE ciphertexts.
```

## Global Flow

One picture for the whole PoC:

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
    trivial_encrypt
    fhe_rand
    fhe_binary_op
    bind_acl_record
  |
  | Anchor self-CPI event bytes
  v
host-listener Solana adapter
  converts:
    SolanaHostEvent
      -> existing TFHE event / ACL DB model
  |
  v
coprocessor DB
  stores:
    computations
    allowed handles
  |
  v
tfhe-worker
  computes real ciphertexts
  |
  v
test decrypt / future KMS path
  reads result handles
  verifies ACL-shaped user decrypt model
```

Boundary rule of thumb:

```text
confidential-token decides app semantics.
zama-host enforces FHEVM host semantics.
host-listener normalizes Solana events into the existing coprocessor model.
tfhe-worker computes ciphertexts from DB work items.
KMS-style verification must combine signed authorization + handle entry + ACL state.
```

## Vocabulary

Use these words consistently when discussing this PoC:

```text
handle
  FHEVM opaque pointer to a ciphertext.
  Never assume the handle is predictable.

scope
  Solana pubkey used as the app/account context for an ACL record.
  Example: AliceTokenAccount.

subject
  Solana pubkey that receives the permission.
  Example: Alice for UserDecrypt, fheAuthority for Compute.

acl_nonce
  Per-scope nonce used to derive an ACL PDA before the output handle is known.

acl_record
  PDA owned by zama-host containing:
    scope
    subject
    handle
    permission

fheAuthority
  App-chosen Solana pubkey allowed to compute on balance handles.
  In this token PoC it is PDA("fhe-authority", mint).

appContext
  User-decrypt request field that maps to ACL scope.
  It is not the ACL subject.
```

The current ACL PDA shape is:

```text
PDA("acl", scope_pubkey, subject_pubkey, acl_nonce)
```

The handle is stored inside the ACL account. The handle is not part of the PDA seed.

That choice is deliberate:

```text
Solana requires accounts up front.
Computed FHEVM handles are not predictable.
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
       emits typed Anchor event
```

The app program does not perform a separate pre-check for normal compute. It passes the operand ACL accounts to `zama-host`, and `zama-host::fhe_binary_op` rejects the operation before emitting the FHE event if either operand is not allowed for the compute subject.

## Host Program Surface

`programs/zama-host` exposes the protocol-side surface:

```text
trivial_encrypt(...)
fhe_rand(...)
fhe_binary_op(...)
allow_handle(...)
input_verified(...)
bind_acl_record(...)
assert_acl_record(...)
```

Current meanings:

```text
trivial_encrypt
  Emit a host event asking the worker to create a ciphertext from a public value.

fhe_rand
  Emit a host event asking the worker to create a random ciphertext.

fhe_binary_op
  Verify compute ACL for operands, then emit a binary FHE op event.

bind_acl_record
  Persist one ACL record in a PDA.

assert_acl_record
  Test/debug helper for directly asserting one ACL record.
  It is not the normal app compute path.
```

Each host event uses Anchor `emit_cpi!`. That makes the event IDL-backed and decodable from self-CPI instruction data.

## ACL Account Model

The PoC stores permissions in `zama-host` PDA accounts.

```text
                         owns
zama-host program --------------------+
                                      |
                                      v
ACL record PDA
  address = PDA("acl", scope, subject, nonce)
  data:
    scope      = AliceTokenAccount
    subject    = fheAuthority
    handle     = hA1
    permission = Compute
```

Writing an ACL record requires the scope to sign:

```text
zama-host::bind_acl_record(scope = AliceTokenAccount, ...)

requires:
  scope_authority.key == AliceTokenAccount
  scope_authority.is_signer
```

For token balances, `confidential-token` signs as:

```text
PDA("token-account", mint, owner)
```

`zama-host` does not know the token PDA seeds. It only verifies:

```text
the scope pubkey signed
the ACL PDA address matches ("acl", scope, subject, nonce)
the stored ACL fields match the instruction fields
```

That keeps token-specific semantics inside the token program.

## Confidential Transfer

`confidential-token::confidential_transfer` is the first SPL-like flow.

Initial state:

```text
AliceTokenAccount
  owner = Alice
  balanceHandle = hA0
  nextAclNonce = 1

BobTokenAccount
  owner = Bob
  balanceHandle = hB0
  nextAclNonce = 1

amount handle
  hX
```

Transaction shape:

```text
Alice signs tx
  |
  v
confidential-token::confidential_transfer(amount = hX)
  |
  +--> CPI zama-host::fhe_binary_op(Sub)
  |      checks:
  |        ACL(scope = AliceTokenAccount, subject = fheAuthority, handle = hA0, permission = Compute)
  |        ACL(scope = Alice,             subject = fheAuthority, handle = hX,  permission = Compute)
  |      emits:
  |        hA1 = FHE.sub(hA0, hX)
  |
  +--> CPI zama-host::fhe_binary_op(Add)
  |      checks:
  |        ACL(scope = BobTokenAccount, subject = fheAuthority, handle = hB0, permission = Compute)
  |        ACL(scope = Alice,           subject = fheAuthority, handle = hX,  permission = Compute)
  |      emits:
  |        hB1 = FHE.add(hB0, hX)
  |
  +--> stores:
  |      AliceTokenAccount.balanceHandle = hA1
  |      BobTokenAccount.balanceHandle   = hB1
  |
  +--> CPI zama-host::bind_acl_record(...)
         creates output ACL records for hA1 and hB1
```

The output ACL records are:

```text
hA1:
  UserDecrypt: scope = AliceTokenAccount, subject = Alice
  Compute:     scope = AliceTokenAccount, subject = fheAuthority

hB1:
  UserDecrypt: scope = BobTokenAccount, subject = Bob
  Compute:     scope = BobTokenAccount, subject = fheAuthority
```

The amount handle ACL is temporary PoC glue. Today the tests pre-bind:

```text
ACL(scope = Alice, subject = fheAuthority, handle = hX, permission = Compute)
```

The real design still needs the Solana equivalent of transient input authorization from the input verifier path.

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
  +--> CPI zama-host::trivial_encrypt(amount)
  |      emits hDeposit
  |
  +--> CPI zama-host::fhe_binary_op(Add)
  |      checks:
  |        ACL(scope = AliceTokenAccount, subject = fheAuthority, handle = hA0,       permission = Compute)
  |        ACL(scope = Alice,             subject = fheAuthority, handle = hDeposit, permission = Compute)
  |      emits:
  |        hA1 = FHE.add(hA0, hDeposit)
  |
  +--> stores:
  |      AliceTokenAccount.balanceHandle = hA1
  |
  +--> CPI zama-host::bind_acl_record(...)
         creates UserDecrypt + Compute ACL records for hA1
```

The deposit amount is public in this slice because it uses `trivial_encrypt(amount)`. A later encrypted-input path should replace that step with `input_verified(...)` and the real ZKPoK/input verifier boundary.

## User Decrypt Shape

The PoC keeps the RFC016-style split:

```text
Signed by Alice:
  userPubkey = Alice
  reencryptionPublicKey = pkR
  allowedAccounts = [AliceTokenAccount]
  validity
  extraData

Unsigned handle entry:
  handle = hA1
  appContext = AliceTokenAccount
  ownerPubkey = Alice
  aclRecordPubkey = PDA("acl", AliceTokenAccount, Alice, 1)
```

KMS-style verification:

```text
1. Verify Alice signed the top-level authorization.
2. Verify appContext is in allowedAccounts.
3. Read aclRecordPubkey.
4. Verify the ACL account is owned by zama-host.
5. Verify the ACL account stores:
     scope      = appContext
     subject    = ownerPubkey
     handle     = handle
     permission = UserDecrypt
```

Visual version:

```text
Alice signature
  says: "I allow decrypts for AliceTokenAccount during this validity window"
       |
       v
Handle entry
  says: "decrypt hA1 in AliceTokenAccount for Alice"
       |
       v
ACL record PDA
  proves on-chain:
    Alice is allowed to user-decrypt hA1 in AliceTokenAccount
```

Important mapping:

```text
appContext == ACL scope
ownerPubkey == ACL subject for UserDecrypt
```

For the current balance, the frontend does not need to search:

```text
read AliceTokenAccount:
  balanceHandle = hA1
  nextAclNonce = 2

current user-decrypt ACL nonce = nextAclNonce - 1 = 1
aclRecordPubkey = PDA("acl", AliceTokenAccount, Alice, 1)
```

For older handles, this shortcut is not enough. The request must carry an ACL record pubkey that the app observed or indexed from prior transactions. KMS does not guess or scan; it reads the provided account and verifies the stored fields.

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

When a Solana ACL event appears in the same transaction as a TFHE event, the adapter marks the TFHE output as allowed before inserting it. This mirrors the EVM listener shape:

```text
tx:
  FHE.add(h0, hDeposit) -> h1
  allow(h1, AliceTokenAccount)

DB:
  allowed_handles(h1, AliceTokenAccount)
  computations(output_handle = h1, is_allowed = true)
```

Remaining listener boundary:

```text
Future Solana poller must only feed events emitted by the configured zama-host program.
The adapter does not treat arbitrary bytes as trusted host events.
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
  trivial_encrypt(125) -> hA0
  trivial_encrypt(20)  -> hB0
  trivial_encrypt(100) -> hX

tfhe-worker
  -> creates real ciphertexts for hA0, hB0, hX

LiteSVM confidential_transfer(hX)
  -> emits FHE.sub(hA0, hX) -> hA1
  -> emits FHE.add(hB0, hX) -> hB1

test decrypt
  -> hA1 = 25
  -> hB1 = 120
```

Random ciphertext creation:

```text
LiteSVM emits:
  fhe_rand(seed, Uint8) -> hRand

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

The ACL scope must sign ACL writes.
  A caller cannot create ACL for someone else's token account.

The host op enforces compute ACL before event emission.
  Wrong scope or wrong fheAuthority rejects the transfer.

User decrypt checks signed authorization plus on-chain ACL state.
  Changing allowedAccounts fails.
  Signing as Bob for Alice fails.
  Passing the wrong ACL record fails.

Solana events can enter the existing coprocessor DB path.
  Worker computes real ciphertexts from Solana-origin events.
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

Current `confidential_transfer` LiteSVM snapshot:

| Metric | Current |
| --- | ---: |
| Instruction account metas | 14 |
| Compiled transaction account keys | 15 |
| Writable instruction metas | 7 |
| Signer instruction metas | 1 |
| Inner instructions | 16 |
| Max CPI depth | 3 |
| Host compute events | 2 |
| Output ACL accounts created | 4 |
| Compute units consumed in LiteSVM | 126,349 |

Open optimization question: event emission mode.

This PoC uses Anchor `emit_cpi!` for host events. That makes events typed and easy to decode from self-CPI instruction data, including in runtime tests and listener code.

The cost is visible:

```text
confidential-token
  -> zama-host
      -> zama-host event self-CPI
```

Anchor `emit!` log events may be cheaper, but it changes the listener and explorer decoding assumptions. Keep `emit_cpi!` for now because this slice optimizes for correctness and typed decoding. Revisit after the ACL and token model stabilize.

## Commands

Solana program build and runtime tests:

```bash
cd solana
NO_DNA=1 anchor build
cargo test --workspace
```

`Cargo.lock` is part of this PoC. It keeps the Anchor/Solana dependency graph compatible with the Cargo version embedded in the local Solana SBF toolchain.

Listener adapter tests:

```bash
cd coprocessor/fhevm-engine
cargo test -p host-listener solana_adapter --lib
```

Worker-backed smoke tests:

```bash
cd coprocessor/fhevm-engine

SQLX_OFFLINE=true cargo test -p tfhe-worker \
  solana_confidential_transfer_with_real_ciphertexts_computes_and_decrypts \
  --lib -- --ignored --nocapture

SQLX_OFFLINE=true cargo test -p tfhe-worker \
  solana_trivial_encrypt_then_confidential_transfer_computes_and_decrypts \
  --lib -- --ignored --nocapture

SQLX_OFFLINE=true cargo test -p tfhe-worker \
  solana_fhe_rand_creates_ciphertext_and_decrypts \
  --lib -- --ignored --nocapture
```

Pre-push checks used on this branch:

```bash
cd solana
cargo fmt --check
cargo test --workspace

cd ../coprocessor/fhevm-engine
cargo fmt -p host-listener -p tfhe-worker --check
SQLX_OFFLINE=true cargo clippy -p host-listener -p tfhe-worker --tests -- -D warnings
```

## Deliberate Non-Goals For This Slice

```text
No TypeScript client yet.
No web3.js.
No KMS payload migration yet.
No ZK proof metadata migration yet.
No private SDK input / InputVerifier path yet.
No KMS quorum decrypt path yet.
```

Those pieces come after the host-chain event boundary is validated.
