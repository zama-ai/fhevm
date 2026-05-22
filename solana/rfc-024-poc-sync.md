# RFC 024 sync notes (PoC branch)

Push these validated choices to tech-spec branch `elias/rfc-024-solana-acl-design` ([PR #448](https://github.com/zama-ai/tech-spec/pull/448)).

Last synced from fhevm branch `openzeppelin-solana-track` after removing `app_account_authority` and adding `authorized_app_accounts`.

---

## Validated by this PoC

### Opaque handles — host assigns, app points

- Apps store and pass **existing** handles with their ACL account (`EncryptedValue { handle, acl_record }`).
- Apps must **not** precompute `H("FHE_comp", …)` for frame outputs.
- After `execute_frame`, apps read `output_acl.handle` from host-written ACL data.

### Durable ACL birth stores the frame step result handle

During `execute_frame`:

```text
step result h = H("FHE_comp", op, operands, …, previous_bank_hash, timestamp) + metadata
Allow action → output ACL PDA gets A.handle = h
```

There is **no** extra `H("FHE_bound_output", base, nonce_key, seq)` layer in the current implementation. Distinct durable outputs come from distinct `(nonce_key, nonce_sequence)` ACL addresses, not from re-hashing the step result.

### Event vs ACL sources of truth

| Surface | Authority |
|---------|-----------|
| `emit_cpi!` TFHE / ACL events | FHE operation graph for coprocessor compute |
| On-chain `AclRecord` account data | Durable permissions and decrypt policy |

The listener ingests events only; it does not reconcile against ACL account snapshots.

### Frame account indices are builder-internal

`remaining_accounts` + `Pubkey` operands in frame IR are host CPI plumbing. App code uses typed Anchor accounts and `fhe::execute`; only the builder maps accounts to pubkeys.

### Handle domain inputs (EVM parity)

Computed handles bind to: op/operands, host program id, chain id, `previous_bank_hash` (slot N−1), and block timestamp — not transaction id. Same-block identical operands yield identical handles; ACL separates authorization.

On-chain, handle derivation **fails closed** with `PreviousBankHashUnavailable` when slot N−1 has no entry in the SlotHashes sysvar.

---

## `execute_frame` — frame model (validated)

The host exposes a **single composable frame instruction** instead of per-op host calls. Apps build one linear program per logical FHE update (wrap debit, transfer `_update`, init balance birth).

### Instruction surface

```text
zama-host::execute_frame(
  authorized_app_accounts: Vec<Pubkey>,
  steps: Vec<FheFrameStep>,
  actions: Vec<FheFrameAction>,
)
```

**Accounts:**

| Account | Role |
|---------|------|
| `payer` | Pays rent for new ACL PDAs created by `Allow` actions |
| `compute_subject` | Signer — mint-level compute PDA (`fhe-compute` seeds on the app program). Must already be allowed on every encrypted operand ACL record used in the frame |
| `system_program` | Creates output ACL accounts |
| `remaining_accounts` | Operand ACL records, output ACL PDAs, etc. Referenced by `Pubkey` in frame IR |

**PoC limits:** `MAX_FRAME_STEPS = 16`, `MAX_FRAME_ACTIONS = 16`, `MAX_FRAME_RESULTS = 16`, `MAX_FRAME_TRANSIENT_ALLOWS = 32`.

### Frame IR

**Steps** (compute graph — emit TFHE events):

```text
FheFrameStep::TrivialEncrypt { plaintext, fhe_type }
FheFrameStep::Operation { opcode, operands, scalar_byte, output_fhe_type }
```

Supported binary ops in PoC: `Add`, `Sub`. Scalar RHS skips RHS ACL check (EVM `scalar` byte parity).

**Actions** (durable side effects on ACL accounts):

```text
FheFrameAction::Allow { source, output_acl_record, nonce_key, nonce_sequence,
                         acl_domain_key, app_account, encrypted_value_label,
                         subjects, public_decrypt }
FheFrameAction::AllowForDecryption { source, acl_record }
```

Execution order: **all steps first**, then **all actions**. Step results are referenced as `FheOperand::PreviousResult { index }`.

### App integration shape

```text
confidential-token::fhe::execute(ctx, |fhe| {
  let x = fhe.encrypted(...);           // durable input via ACL record
  let y = fhe.trivial_encrypt_u64(...); // frame-local
  let z = fhe.add(x, y, fhe_type)?;
  fhe.allow(&z, DurableAllow { app_account, acl_record, ... })?;
})
  -> one CPI to zama-host::execute_frame
  -> builder collects authorized_app_accounts from each allow()
```

ERC7984-style transfer uses **one frame** with two `Allow` actions (debit + credit), mirroring a single EVM `_update` call.

### Host instructions outside frames

| Instruction | Role |
|-------------|------|
| `allow_acl_subjects` | Extend subjects on an **existing** canonical ACL; caller must already be allowed on the handle |
| `allow_for_decryption` | Set `public_decrypt` on a durable ACL; caller must already be allowed (EVM: subject with allow/transient allow may mark decrypt) |
| `assert_acl_record` | Read-only verification helper for KMS / clients |

---

## Transient allow model (validated)

Transient allowance is **instruction-local** to a single `execute_frame` invocation. It is **not** stored in on-chain ACL accounts and **does not** survive the instruction.

### Behavior

```text
On each step result (TrivialEncrypt or Operation):
  1. Host computes handle h
  2. Host records in-memory (handle, compute_subject) in frame.transient_allows
  3. Host emits TFHE event with result = h

On FheOperand::PreviousResult { index }:
  Require (result.handle, compute_subject) ∈ frame.transient_allows
  Else → AclSubjectMismatch

On `FheOperand::AclRecord { handle, acl_record }`:
  Require compute_subject ∈ record.subjects for that handle
  (durable ACL — same as EVM isAllowed on operands)
```

### Durable vs transient

| Kind | Where stored | Created by | Survives instruction? |
|------|--------------|------------|------------------------|
| **Transient** | Frame-local vector only | Automatic after each step | No |
| **Durable** | `AclRecord` PDA | Explicit `Allow` action (or `allow_acl_subjects` on existing record) | Yes |

**Rule:** Intermediate handles are opaque bytes32 values. They are not durable by default. Only an explicit `Allow` action (or a separate `allow_acl_subjects` call) creates persistent ACL state.

### EVM parity notes

- Matches `FHEVMExecutor` granting `allowTransient(result, msg.sender)` after each op.
- `AllowForDecryption` inside a frame may target a **transient** step result: the frame checks the source handle against `transient_allows` before setting `public_decrypt` on the output ACL created by a prior `Allow` in the same frame.
- Standalone `allow_for_decryption` requires the signer to already appear in the durable ACL record's `subjects` for that handle (transient allowance alone is insufficient across instructions).

---

## Host authorization (validated — `app_account_authority` removed)

Early PoC carried `ExecuteFrame.app_account_authority` as a second signer (token-account PDA seeds for CPI). The host **never validated** it against frame metadata, and it **could not** represent multi-slot frames (e.g. transfer with `Allow` for both `from` and `to`).

**Removed.** Current model:

### `compute_subject` (signer)

- Mint-level app PDA (`b"fhe-compute", acl_domain_key`).
- Analog of EVM `msg.sender` on `FHEVMExecutor` / `ACL.allow` when the caller is the app contract.
- Must be listed in `subjects` on every **encrypted operand** ACL record used in the frame.

### `authorized_app_accounts` (instruction arg)

- Vec of app state slot pubkeys the calling program declares it will update in this frame.
- Host enforces: every `FheFrameAction::Allow.app_account` must be **∈ authorized_app_accounts**.
- Violation → `UnauthorizedAppAccount`.
- The token builder auto-collects pubkeys from each `fhe.allow(...)` call.

### User / business authorization (app layer, not host)

- Token instructions require `owner: Signer` and validate account ownership (`OwnerMismatch`, mint match, current ACL record match).
- Analog of ERC7984 `FHE.isAllowed(amount, msg.sender)` / operator checks on the outer transaction.
- Host does **not** deserialize app account layouts; it trusts the app program to only list slots it validated.

### EVM mapping (ERC7984 transfer)

```text
EVM                                    Solana PoC
────────────────────────────────────────────────────────────────────
User signs confidentialTransfer        owner signs confidential_transfer
Token contract = ACL writer            compute_subject PDA signs CPI
FHE.allow(ptr, from/to) in _update     Allow actions; app_account metadata
No recipient ACL signature             authorized_app_accounts lists from+to
One atomic _update                     One execute_frame CPI
```

---

## Test defaults

LiteSVM fixtures seed `SlotHashes` with `SHA256(b"zama-solana-test-bank-hash-v1")` so handle derivation is production-shaped without per-test setup.

Runtime tests: **35** in `runtime-tests/tests/host_events.rs` (includes `execute_frame_rejects_allow_for_unauthorized_app_account`).

---

## Still open in RFC / not settled here

- Real external input path (replacing `poc_authorize_transfer_amount`)
- Subject overflow / chunking beyond `MAX_ACL_SUBJECTS = 8`
- ACL account cleanup and rent policy
- Production Solana ingestion (Geyser/RPC → listener)
- Rand / full opcode surface in `execute_frame`
