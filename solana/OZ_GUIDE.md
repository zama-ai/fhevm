# OpenZeppelin Solana PoC Guide

Short handoff for OpenZeppelin on branch `openzeppelin-solana-track`.

| Read first | Purpose |
|------------|---------|
| **This file** | Where to edit, what is stable, how to test |
| [README.md](./README.md) | Full flows, ACL model, listener/worker boundaries, progress checklist |
| [RFC 024 (draft)](https://github.com/zama-ai/tech-spec/pull/448) | **Normative ACL / host design** — update when PoC validates or rejects a design choice |

**Rule:** Design decisions belong in RFC 024. README tracks what this branch implements. Do not update other RFCs from this PoC work.

**RFC workflow:** Push RFC changes to tech-spec branch `elias/rfc-024-solana-acl-design` ([PR #448](https://github.com/zama-ai/tech-spec/pull/448)), not to tech-spec `main`. After an RFC push, sync README/OZ_GUIDE if the host surface or boundaries changed.

## Current host surface

All FHE host semantics go through one CPI batch:

```text
zama-host
  execute_frame          — FHE steps + explicit durable Allow actions
  allow_acl_subjects     — append subjects on canonical ACL record
  allow_for_decryption   — public decrypt flag
  assert_acl_record      — debug / test helper
```

Legacy per-op instructions (`fhe_binary_op`, `trivial_encrypt`, `mock_input_verified_and_bind`, `test_emit_*`) were **removed**.

## Current app surface

```text
confidential-token
  initialize_mint / initialize_token_account
  wrap_usdc
  confidential_transfer
  poc_authorize_transfer_amount   — PoC-only input stand-in (see Caveats)
  poc_demo_confidential_rand      — E2E rand demo + ConfidentialRandCreatedEvent

solana/crates/zama-fhe
  fhe::execute(ctx, |fhe| { ... })  — default app API, one execute_frame CPI
  fhe::protocol                      — protocol IR types for custom wrappers
```

**Safe area for OZ:** `confidential-token` behavior and tests. **Do not** add a separate ACL program unless the guild explicitly changes direction in RFC 024.

## ACL model (RFC 024 aligned)

```text
nonce_key = H("zama-acl-nonce-key-v1", acl_domain_key, app_account, encrypted_value_label)
acl_record = PDA("acl-record", nonce_key, nonce_sequence)
```

- Handle is stored **inside** the ACL record; never use handle bytes as a PDA seed.
- Token PoC: `acl_domain_key = mint`, balance label = `"balance"`, input label = `"input"`.
- Durable ACL only when the frame includes an explicit `Allow` action; intermediate handles are frame-local.

## What works end-to-end

```text
initialize -> wrap_usdc -> confidential_transfer
  -> BalanceHandleUpdatedEvent (app indexer only)
  -> ZamaHost events -> host-listener -> tfhe-worker (ignored E2E tests)
  -> user decrypt checks (RFC-016-shaped, test-local KMS model)
```

Self-transfer: **no-op** (no handle rotation, no output ACL).

## Boundaries (do not blur)

| Layer | Responsibility |
|-------|----------------|
| **confidential-token** | SPL-like semantics, owner/mint checks, which ACL PDAs to pass |
| **zama-host** | Compute ACL, handle birth inside frame, canonical PDA rules, generic events |
| **host-listener** | ZamaHost IDL events only → coprocessor DB |
| **App indexer** | `BalanceHandleUpdatedEvent`, historical handle discovery |
| **KMS (future)** | Verify signed auth + ACL record account; no SPL parsing |

Listener decoders come from the shared `solana/crates/zama-host-events` crate (IDL at `host-listener/idl/zama_host.json`). Sync with `bash scripts/sync-zama-host-idl.sh` after host changes.

**IDL events not emitted yet:** `InputVerifiedEvent`.

**Emitted today:** `FheBinaryOpEvent`, `TrivialEncryptEvent`, `FheRandEvent`, `AclAllowedEvent`, `AclPublicDecryptAllowedEvent`.

**Not wired:** `FheOpcode::RandBounded` (op 27) — no frame step or event yet.

**Host authorization:** `execute_frame` takes `authorized_app_accounts`. Every durable
`Allow.app_account` must appear in that list. The app account must either sign directly,
or the frame `compute_subject` must be `PDA("fhe-compute", acl_domain_key)` for the app
account owner program. Compute-time ACL is still enforced via `compute_subject`
membership on operand ACL records.

## Shared test crate

```text
solana/tests  (zama-solana-tests)
  src/              — fixtures, scenarios, event decoders, CleartextBackend, KMS helpers
  src/host_events.rs — integration tests
```

Add behavior tests in `tests/src/host_events.rs` before changing token logic.

## Caveats (intentional PoC shortcuts)

```text
Input:
  poc_authorize_transfer_amount = trivial_encrypt + allow via fhe::execute
  Not external ciphertext / input verifier — replace before production claims

Execution frame:
  Transient allow is instruction-local inside execute_frame
  Durable allow only via explicit Allow actions in the frame

Subjects:
  Plain Pubkey list; MAX_ACL_SUBJECTS = 8 in PoC code (overflow TBD in RFC 024)

Rand:
  poc_demo_confidential_rand — token ix calling fhe.rand_u64() + durable Allow
  E2E test: confidential_token_e2e_rand_demo_encrypt_compute_and_user_decrypt_request
  signed_confidential_rand_user_decrypt_request — frontend-shaped decrypt payload helper
  fheRandBounded (op 27) not planned for current PoC

KMS:
  Decrypt semantics tested in tests/src/kms.rs, not wired to KMS connector
```

## How to contribute

```text
1. LiteSVM test in tests/src/host_events.rs (happy + negative auth case)
2. Change confidential-token for app behavior
3. Change zama-host only for host-semantics gaps (coordinate RFC 024 update)
4. Sync README.md; open RFC 024 PR on tech-spec if design changed
5. Extend canonical wrap/transfer/decrypt scenario — avoid second demo flows
```

## Commands

```bash
cd solana
NO_DNA=1 anchor build --ignore-keys
bash scripts/check-zama-host-idl.sh   # fail if IDL drift
bash scripts/sync-zama-host-idl.sh  # copy target/idl → host-listener after host changes
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

LiteSVM integration tests live in `tests/src/host_events.rs`. On-chain handle derivation fails closed when the previous slot hash is missing; every fixture seeds a non-zero previous bank hash by default.

**Worker E2E (`#[ignore]`, CI `tfhe-worker-solana-poc`):** transfer tests run in CI; `solana_fhe_rand_creates_ciphertext_and_decrypts` exists locally but is not in the CI loop yet.

**CI toolchain:** GitHub Actions installs Anchor **1.0.2** via [AVM](https://www.anchor-lang.com/docs/installation) from `solana-foundation/anchor` (see `.github/workflows/solana-tests.yml`). Do not use `metadaoproject/setup-anchor` — its npm package (`@coral-xyz/anchor-cli`) only ships 0.31.x. The workflow uses a `check-changes` job (`dorny/paths-filter`) so Solana tests run only when `solana/**` (or the synced host IDL) changes; use **workflow_dispatch** to run manually.

Worker compile check (requires built programs in `solana/target/deploy/`):

```bash
cd ../coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p tfhe-worker solana_user_decrypt_acl_invariants_match_evm_semantics --no-run
```
