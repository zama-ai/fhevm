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

solana/runtime-tests
  Fast LiteSVM tests for Solana accounts, PDAs, CPI, events, and ACL behavior.

coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  Maps typed Solana host events into the existing coprocessor DB model.

coprocessor/fhevm-engine/tfhe-worker/src/tests/solana_poc.rs
  Worker-backed end-to-end tests with real small TFHE ciphertexts.
```

For a shorter contributor guide focused on OpenZeppelin follow-up work, see [`OZ_BRANCH_GUIDE.md`](./OZ_BRANCH_GUIDE.md).

## Handoff Path

Use this order when picking up the branch:

```text
1. Run it
   cd solana
   anchor build
   cargo test --workspace

2. Read the flow
   Start with "Global Flow", then "Confidential Transfer", then "User Decrypt Shape".

3. Change the PoC
   App behavior usually belongs in confidential-token.
   Host-chain FHEVM semantics usually belong in zama-host.
   Event normalization belongs in host-listener/src/solana_adapter.rs.
   Worker/KMS-shaped checks belong in tfhe-worker/src/tests/solana_poc.rs.

4. Keep the negative tests
   Every new happy path should have a wrong signer, wrong ACL record, wrong handle,
   or wrong subject test when the feature touches authorization.
```

Known gaps for the next iteration:

```text
input handles still use PoC glue instead of a real Solana input verifier path
transient allow semantics are not modeled yet
account growth / rent cleanup is intentionally unresolved
historical handle indexing is app/indexer responsibility for now
KMS verification is modeled in tests, not wired to the real KMS connector
```

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
    trivial_encrypt
    fhe_rand
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
  Examples: Alice for user decrypt, computeSigner for compute.

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
       emits typed Anchor event
```

The app program does not perform a separate pre-check for normal compute. It passes operand ACL accounts to `zama-host`, and `zama-host::fhe_binary_op` rejects the operation before emitting the FHE event if either operand is not allowed for the compute signer.

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
    subjects              = [Alice, computeSigner]
```

Writing an ACL record requires the app account to sign:

```text
zama-host::bind_acl_record(app_account = AliceTokenAccount, ...)

requires:
  app_account_authority.key == AliceTokenAccount
  app_account_authority.is_signer
```

For token balances, `confidential-token` signs as:

```text
PDA("token-account", mint, owner)
```

`zama-host` does not know token PDA seeds. It only verifies:

```text
the app account pubkey signed
the ACL PDA address matches ("acl-record", nonce_key, nonce_sequence)
the stored ACL fields match the instruction fields
the requested subject is in subjects[]
```

That keeps token-specific semantics inside the token program.

## Confidential Transfer

`confidential-token::confidential_transfer` is the first SPL-like flow.

Initial state:

```text
AliceTokenAccount
  owner = Alice
  balanceHandle = hA0
  balanceAclRecord = A0
  nextBalanceNonceSequence = 1

BobTokenAccount
  owner = Bob
  balanceHandle = hB0
  balanceAclRecord = B0
  nextBalanceNonceSequence = 1

computeSigner = PDA("fhe-compute", cUSDCMint)
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
  |      compute_subject = computeSigner
  |      checks:
  |        A0 stores hA0 and subjects includes computeSigner
  |        X  stores hX  and subjects includes computeSigner
  |      emits:
  |        hA1 = FHE.sub(hA0, hX)
  |
  +--> CPI zama-host::fhe_binary_op(Add)
  |      compute_subject = computeSigner
  |      checks:
  |        B0 stores hB0 and subjects includes computeSigner
  |        X  stores hX  and subjects includes computeSigner
  |      emits:
  |        hB1 = FHE.add(hB0, hX)
  |
  +--> CPI zama-host::bind_acl_record(...)
  |      creates A1 for hA1 with subjects [Alice, computeSigner]
  |
  +--> CPI zama-host::bind_acl_record(...)
  |      creates B1 for hB1 with subjects [Bob, computeSigner]
  |
  +--> stores:
         AliceTokenAccount.balanceHandle = hA1
         AliceTokenAccount.balanceAclRecord = A1
         AliceTokenAccount.nextBalanceNonceSequence = 2
         BobTokenAccount.balanceHandle = hB1
         BobTokenAccount.balanceAclRecord = B1
         BobTokenAccount.nextBalanceNonceSequence = 2
```

The amount handle ACL is temporary PoC glue. Today the tests pre-bind:

```text
ACL domain key = Alice
app account    = Alice
label          = "input"
subjects       = [computeSigner]
handle         = hX
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
  |        current balance ACL stores hA0 and allows computeSigner
  |        amount ACL stores hDeposit and allows computeSigner
  |      emits:
  |        hA1 = FHE.add(hA0, hDeposit)
  |
  +--> CPI zama-host::bind_acl_record(...)
         creates one output ACL record for hA1:
           subjects = [Alice, computeSigner]
```

The deposit amount is public in this slice because it uses `trivial_encrypt(amount)`. A later encrypted-input path should replace that step with `input_verified(...)` and the real ZKPoK/input verifier boundary.

## User Decrypt Shape

The PoC keeps the RFC016-style split:

```text
Signed by Alice:
  userPubkey = Alice
  reencryptionPublicKey = pkR
  allowedAclDomainKeys = [cUSDCMint]
  validity
  extraData

Unsigned handle entry:
  handle = hA1
  ownerPubkey = Alice
  aclRecordPubkey = A1
```

KMS-style verification:

```text
1. Verify Alice signed the top-level authorization.
2. Read aclRecordPubkey.
3. Verify the ACL account is owned by zama-host.
4. Verify:
     expected_nonce_key = H(record.acl_domain_key, record.app_account, record.encrypted_value_label)
     aclRecordPubkey == PDA("acl-record", expected_nonce_key, record.nonce_sequence)
     record.acl_domain_key is in allowedAclDomainKeys
     record.handle == handle
     record.subjects contains ownerPubkey
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
  balanceHandle = hA1
  balanceAclRecord = A1

request carries:
  handle = hA1
  aclRecordPubkey = A1
```

For older handles, the request must carry an older ACL record pubkey that the app observed or indexed from prior transactions. KMS does not guess or scan; it reads the provided account and verifies the stored fields.

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
  bind ACL record allowing h1

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

The app account must sign ACL writes.
  A caller cannot create ACL for someone else's token account.

The host op enforces compute ACL before event emission.
  Wrong current ACL or wrong amount ACL rejects the transfer.

User decrypt checks signed authorization plus on-chain ACL state.
  Changing allowedAclDomainKeys fails.
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
each output ACL record stores both subjects: user + computeSigner
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
anchor build
cargo test --workspace
```

Worker test compile check:

```bash
cd coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p tfhe-worker solana_user_decrypt_acl_invariants_match_evm_semantics --no-run
```

Running the ignored worker-backed tests requires the usual coprocessor Postgres/test harness setup.

`Cargo.lock` is part of this PoC. It keeps the Anchor/Solana dependency graph compatible with the Cargo version embedded in the local Solana SBF toolchain.
