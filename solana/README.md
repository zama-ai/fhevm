# Solana FHEVM PoC

This workspace is the first Solana host-chain PoC for the `openzeppelin-solana-track` branch.

The immediate goal is not a full confidential token. The first goal is a fast feedback loop for the host-chain boundary:

```text
Anchor instruction
  -> typed Anchor event
  -> Solana listener adapter
  -> existing TFHE event model
  -> existing coprocessor DB / worker path
```

## Current Scope

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

Each instruction emits an Anchor event through `emit_cpi!`, so the event schema is IDL-backed and can be decoded by Solana tooling.

`programs/confidential-token` is the app-side PoC. It owns the mint and token-account state, then calls `zama-host` by CPI for ACL checks, compute-event emission, and output ACL creation.

`bind_acl_record` is the first persistent ACL experiment. Its PDA is derived from:

```text
PDA("acl", scope_pubkey, subject_pubkey, acl_nonce)
```

The handle is stored inside the account after execution. The handle is not part of the PDA seed, so the account can be passed in the Solana transaction before a computed handle is known.

ACL writes require the `scope_pubkey` to sign the CPI:

```text
zama-host::bind_acl_record(scope = AliceTokenAccount, ...)
  requires scope_authority.key == AliceTokenAccount
  requires scope_authority.is_signer
```

For token balances, `confidential-token` signs as `PDA("token-account", mint, owner)`. `zama-host` does not know token PDA seeds; it only verifies the signer and the scope key.

`confidential-token::confidential_transfer` is the first SPL-like flow:

```text
Alice token account: hA0, next_acl_nonce = 1
Bob token account:   hB0, next_acl_nonce = 1

transfer(amount = hX)
  CPI -> zama-host: checks compute ACL for hA0 and hX, then emits hA1 = FHE.sub(hA0, hX)
  CPI -> zama-host: checks compute ACL for hB0 and hX, then emits hB1 = FHE.add(hB0, hX)
  stores hA1 and hB1 as the new balance handles
  CPI -> zama-host: binds owner decrypt + FHE compute ACL records for hA1 and hB1
```

Here `scope_pubkey` is the token account whose balance handle is being rotated. This keeps app/FHE-authority ACL records from sharing one global nonce lane.

The compute ACL check lives inside `zama-host::fhe_binary_op`, matching the EVM boundary where `FHEVMExecutor` validates ACL before an operation is emitted. `confidential-token` does not pre-check ACL as app logic; it supplies the operand ACL accounts and the host program rejects the operation if either operand is not allowed for the FHE authority.

The LiteSVM transfer test loads both programs, sends the transfer to `confidential-token`, decodes the actual `zama-host` Anchor self-CPI event bytes, and asserts the emitted FHE operations:

```text
FheSub(hA0, hX) -> hA1
FheAdd(hB0, hX) -> hB1
```

`confidential-token::wrap_usdc` is the first cUSDC-style wrapper flow. The test uses a normal SPL Token mint as mock USDC:

```text
Alice USDC token account --transfer_checked(amount)--> cUSDC vault token account

vault owner = PDA("vault-authority", ConfidentialMint)

then the confidential balance is updated:
  CPI -> zama-host: emits hDeposit = trivial_encrypt(amount)
  CPI -> zama-host: checks compute ACL for hA0 and hDeposit, then emits hA1 = FHE.add(hA0, hDeposit)
  stores hA1 as Alice's new cUSDC balance handle
  CPI -> zama-host: binds owner decrypt + FHE compute ACL records for hA1
```

This deliberately models a wrapper first, not a Token-2022 extension. The deposit amount is public in this slice because it uses `trivial_encrypt(amount)`; the later encrypted-input path should replace that step with `input_verified(...)` and the real ZKPoK/input verifier boundary.

Temporary PoC boundary: input/trivial amount handles are pre-bound under Alice's wallet scope with `permission = Compute` so `fhe_binary_op` can enforce both operands today. The real design still needs the Solana equivalent of transient input authorization from the input verifier path.

Current transfer budget snapshot:

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

This PoC currently uses Anchor `emit_cpi!` for host events. That makes events typed and easy to decode from self-CPI instruction data, including in runtime tests and listener code. The cost is visible in the budget above: each FHE op becomes `confidential-token -> zama-host -> zama-host event self-CPI`, which increases inner instruction count and compute units.

Anchor `emit!` log events may be cheaper, but it changes the listener and explorer decoding assumptions. Keep `emit_cpi!` for now because this slice optimizes for correctness and typed decoding. Revisit this after the ACL and token model stabilize.

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

KMS-style verification becomes:

```text
1. Alice signed the authorization.
2. appContext is in allowedAccounts.
3. `aclRecordPubkey` is a `zama-host` ACL account containing:
     scope = AliceTokenAccount
     subject = Alice
     handle = hA1
     permission = UserDecrypt
```

The runtime test now signs the top-level authorization bytes with Alice's Solana keypair and verifies that signature before checking ACL state. It also checks that changing the signed `allowedAccounts` or signing with Bob's key fails. The handle entry and `aclRecordPubkey` stay unsigned, following RFC016, but they must match the on-chain ACL record.

Important: in this PoC, `appContext` maps to ACL `scope`, not ACL `subject`. The `subject` is the user or compute authority being allowed. The `scope` is the app/account context in which the handle is valid.

The request carries `aclRecordPubkey` because ACL account addresses are not derived from handles. A client can pass the wrong ACL account, but the KMS-side check still rejects it because the stored `scope`, `subject`, `handle`, and `permission` must match the unsigned handle entry and Alice's signed authorization.

For the current balance, the frontend does not need to search:

```text
read AliceTokenAccount:
  balanceHandle = hA1
  nextAclNonce = 2

current user-decrypt ACL nonce = nextAclNonce - 1 = 1
aclRecordPubkey = PDA("acl", AliceTokenAccount, Alice, 1)
```

For older handles, this shortcut is not enough. The request must carry an ACL record pubkey that the app already observed or indexed from prior transactions. KMS still does not guess or scan; it only reads the provided account and verifies the stored fields.

The Rust adapter lives in:

```text
coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
```

It maps typed Solana events to the existing `TfheContractEvents` enum where possible. ACL allowances keep the Solana subject as a string instead of forcing a 32-byte pubkey into an EVM `address`.

The adapter now has the first real DB insertion path:

```text
Solana transaction signature
  -> sha256(signature) as the existing 32-byte transaction_id

Anchor self-CPI event bytes
  -> SolanaHostEvent
  -> TfheContractEvents / ACL allowance
  -> LogTfhe
  -> listener_db.insert_tfhe_event(...)
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

The remaining listener boundary is provenance: the future Solana poller must only feed events that were emitted by the configured `zama-host` program. The adapter intentionally does not treat arbitrary bytes as trusted host events.

## Feedback Gates

```text
C1: Anchor program builds and emits typed events.
C2: Runtime transfer events are decoded from Anchor self-CPI bytes.
C3: Mapped events can be inserted through the existing host-listener DB methods.
C4: ACL PDA design is tested with a minimal confidential-token transfer.
C5: User decrypt references the same app context / ACL subject shape.
C6: Real encrypted inputs flow through Solana events, the existing DB model, the real TFHE worker, and local test-key decrypt.
C7: Solana-born ciphertexts are created by worker computations from trivial_encrypt and fhe_rand events, not only by direct DB seeding.
```

## Commands

```bash
cd solana
NO_DNA=1 anchor build
cargo test --workspace
```

`Cargo.lock` is part of this PoC. It keeps the Anchor/Solana dependency graph compatible with the Cargo version embedded in the local Solana SBF toolchain.

```bash
cd coprocessor/fhevm-engine
cargo test -p host-listener solana_adapter --lib
```

The worker-backed smoke test is ignored by default because it starts disposable Postgres, runs migrations, creates test keys, loads the Solana programs in LiteSVM, inserts decoded Solana events through `host-listener`, runs the real `tfhe-worker`, and decrypts the result with the local test key:

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

That test uses small real `FheUint8` ciphertexts for speed. The Solana programs only see opaque handles, so this still exercises the real cross-component path:

```text
Enc(125) = hA0     Enc(20) = hB0     Enc(100) = hX

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

The second worker-backed transfer test removes direct input seeding for the happy path:

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

The `fhe_rand` worker-backed test proves the same event boundary for random ciphertext creation:

```text
LiteSVM emits:
  fhe_rand(seed, Uint8) -> hRand

tfhe-worker
  -> creates a real random ciphertext for hRand

test decrypt
  -> hRand is a Uint8 plaintext
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
