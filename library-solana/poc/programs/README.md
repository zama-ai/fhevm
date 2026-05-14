# Solana ACL PoC

Anchor workspace exploring how the FHEVM Access Control List (ACL) maps onto Solana's
explicit account model. Tracks the design discussed in [`gist-nico.md`](./gist-nico.md);
this README assumes familiarity with that document and with the EVM ACL it mirrors.

The semantic primitive is unchanged:

```text
isAllowed(handle, subjectPubkey)
```

The physical verifier is Solana-shaped — the caller passes the ACL record account as a
witness, and the program checks the account against the requested `(handle, subject)`:

```text
verifyAllowed(handle, subjectPubkey, aclRecordAccount)
```

## Workspace layout

| Program | Path | Role |
|---|---|---|
| `acl` | [`programs/acl`](./programs/acl) | The ACL program itself. Owns permission-list PDAs, gates writes through a `Config` authority. |
| `mock-external` | [`programs/mock-external`](./programs/mock-external) | Stand-in for an external-input program. Drives `init_handle` + `allow` over CPI to exercise the authority path that doesn't go through the FHE authority. |
| `mock-fhe` | [`programs/mock-fhe`](./programs/mock-fhe) | Empty stub. The FHE Executor PoC is not part of this workspace yet. |

## ACL design at a glance

### `Config` ([`state.rs`](./programs/acl/src/state.rs))

Singleton PDA at seeds `["acl_config"]`. Stores two pubkeys allowed to mutate any
permission list:

- `fhe_authority` — the FHE Executor (mocked as a keypair in tests).
- `external_input_authority` — a PDA owned by an external-input program (`mock-external` here).

`Config::authorize(user)` returns true if `user` matches either. `init` is restricted to
the ACL program's upgrade authority (PoC simplification).

### `HandlerPermissions` ([`state.rs`](./programs/acl/src/state.rs))

One account per handle, derived from `["permission_list", initial_key, output_index_le]`
under the ACL program. The seed pair `(initial_key, output_index)` is **intentionally
independent of the handle value** so the PDA can be predeclared in the transaction
before the handle is known — the handle is bound into the record afterwards. See
gist Part 3 ("Modified Option B layout") for the rationale.

Fields (trimmed v0 layout):

```rust
pub struct HandlerPermissions {
    pub handle: [u8; 32],
    pub state: HandlerState,            // Reserved | Bound
    pub allowed_accounts: [Pubkey; MAX_SUBJECTS],
    pub subject_count: u8,
    pub bump: u8,
    pub version: u8,
}
```

`MAX_SUBJECTS = 8` (see [`constants.rs`](./programs/acl/src/constants.rs)).

### Instructions ([`programs/acl/src/instructions`](./programs/acl/src/instructions))

| Instruction | Signer | Effect |
|---|---|---|
| `init(fhe_authority, external_input_authority)` | ACL upgrade authority | Creates the `Config` PDA. Single-shot. |
| `init_handle(handle, initial_key, output_index)` | `Config` authority | Reserves+binds a `HandlerPermissions` PDA. Idempotent for the same `(seeds, handle)`; reverts with `HandleMismatch` if the PDA is already `Bound` to a different handle. |
| `allow(handle, context_key, initial_key, output_index)` | `Config` authority | Appends `context_key` to `allowed_accounts`. Idempotent. Reverts with `HandleOverflow` past `MAX_SUBJECTS`, `HandleMismatch` on handle mismatch, `DefaultKeyAllow` on the zero pubkey. |
| `is_allowed(handle, subject_pubkey, initial_pubkey, output_index)` | none | Returns `Ok(())` iff `handle` matches the record and `subject_pubkey` is in `allowed_accounts`. |

## CPI example — `mock-external`

[`mock-external/src/lib.rs`](./programs/mock-external/src/lib.rs) shows how a program
registered as `external_input_authority` drives the ACL:

1. The external program owns a PDA at seeds `["external_input_authority"]`. That PDA's
   pubkey is what gets stored in `Config.external_input_authority` at `init` time.
2. `allow_external_input` CPIs `init_handle` then `allow` (twice — once for the user,
   once for the app), with the authority PDA signing via `invoke_signed`.
3. The permission-list PDA is derived under the **ACL program id** (`seeds::program =
   acl_program.key()`), since that's who owns it.

The test [`tests/mock_external.ts`](./tests/mock_external.ts) exercises the round-trip.

## Build & test

```sh
yarn install
anchor build
anchor test
```

> **Note**: `Anchor.toml` sets `[test] upgradeable = true`. This is required —
> `init`'s upgrade-authority constraint fails under Agave 3.x without it.

Tests:
- [`tests/init.ts`](./tests/init.ts) — `Config` initialization, re-init failure.
- [`tests/init_handle.ts`](./tests/init_handle.ts) — PDA creation, idempotency, handle mismatch, unauthorized signer.
- [`tests/allow.ts`](./tests/allow.ts) — append, idempotency, overflow, mismatch, unauthorized signer, default-key guard.
- [`tests/is_allowed.ts`](./tests/is_allowed.ts) — positive and negative authorization paths.
- [`tests/mock_external.ts`](./tests/mock_external.ts) — end-to-end CPI from `mock-external`.

`tests/utils.ts` carries shared fixtures (single `Config` authority across the run,
`ensureConfigInitialized`, PDA derivers).

## Known gaps / non-goals

This is a PoC. The following items are intentionally out of scope and tracked here so
they aren't mistaken for design decisions.

### 1. Canonicality is not enforced

`init_handle` binds whatever 32-byte handle the authority hands it; the handle preimage
is not required to include the ACL record pubkey. With this PoC alone, the
"caller-supplied ACL record" attack from gist Part 4 is not closed.

Canonicality has to be enforced where the handle is **derived**, which is the FHE
Executor program. That program is not in this workspace yet (`mock-fhe` is a stub), so
the check is deferred until the Executor PoC lands and we can include
`outputAclAccount` in the handle preimage as recommended by the gist.

### 2. `mock-external` does not validate the handle

The external-input CPI passes the handle straight through to `init_handle` without
verification (`// The handle is currently not checked.` in
[`mock-external/src/lib.rs`](./programs/mock-external/src/lib.rs)). In the real design,
this is where the ZK / cryptographic proof attached to an external input handle is
checked before granting access. That verification belongs to the external-input design
and is out of scope here.

### 3. Reserve and Bind are merged into `init_handle`

The gist describes a two-step lifecycle (`reserve_acl_record` → `bind_output_acl_record`)
so the executor can predeclare an output ACL account, then commit the computed handle
to it. This PoC collapses both into `init_handle` for simplicity; the
[`HandlerState`](./programs/acl/src/state.rs) enum (`Reserved` / `Bound`) exists for
the eventual split but isn't exercised. The two-instruction shape is the planned v1.

### 4. Fixed `MAX_SUBJECTS = 8`, no pagination

The subject list is a fixed-size array. Past 8 entries, `allow` reverts with
`HandleOverflow`. There is no extension record, no paginated page set, and no
per-`(handle, subject)` fallback PDA. If a real flow runs into this cap, v1 should add
extension records or paginated subject pages (gist Part 9).

### 5. `HandlerPermissions` is a trimmed layout

The struct keeps only what the PoC instructions need. Fields the gist recommends for a
production v0 are absent — no `app_context`, no `created_slot`, no `seed_version`, no
explicit account-type discriminator constant, no documented version-support set. The
[`state.rs`](./programs/acl/src/state.rs) comment marks this as a deliberate cut. These
fields are expected to come back as the design firms up; nothing in the PoC depends on
their absence.

### 6. No executor-bound handle birth

Distinct from gap 3 (which is about the *instruction shape*): the PoC's authority model
is a single `fhe_authority` keypair (and `external_input_authority` PDA) configured at
`init`. The gist requires the bind path to be signed by an **executor authority** —
an explicit PDA owned by the FHE Executor program, signing via `invoke_signed`, with the
ACL program checking both `is_signer` and that the key matches a config-stored
`authorized_executor_authority`. Output ACL accounts should also be **predeclared** by
the executor so their pubkey can enter the handle preimage (see gap 1). None of this is
present here — `init_handle` accepts any predeclared PDA and any handle from the global
authority. Lands together with the FHE Executor PoC.

### 7. Only positive per-subject grants — no delegation, public decrypt, or deny-list

`allow` appends a `Pubkey` to a fixed-size list; `is_allowed` checks membership. That's
the entire ACL semantic surface. Missing:

- **Delegation** (RFC-017 wildcard delegation) — there is no notion of "subject S has
  delegated its access for handle H to delegate D under scope X". The KMS Connector
  side of the gist (Part 6) assumes this exists.
- **Public / permissive decrypt** (RFC-016) — no way to mark a handle as publicly
  decryptable, and no app-context skip rule for permissive requests.
- **Deny-list / revocation** — records are append-only within capacity; there is no
  `disallow`, no expiry, no superseded state. Gap 4 (no pagination) and this gap
  together mean a handle is effectively locked to whoever was granted first.

These belong to the request-authorization layer rather than to storage, but the
storage today doesn't even expose the hooks they would need (e.g. per-entry flags, a
revoked bit, or a delegation reference field).

## References

- [`Nico's Gist`](https://gist.github.com/nicolasgarcia214/210d8eddc8266525605ebb2ac5a2ffc5) — design discussion this PoC tracks.