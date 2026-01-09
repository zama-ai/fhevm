# Gateway V2 Phase Review (2026-01-09)

## Phase 1A (commit 8e666c37)
Build: `forge build` in `gateway-contracts/` (success, warnings only).

Findings:
- InputVerificationRegistry was not part of this commit; it was introduced later in Phase 2 (81f0bf2e). This breaks the plan's “Phase 1A atomic” requirement and leaves Phase 1A incomplete by itself.
- No new Foundry tests for V2 contracts (DecryptionRegistry/InputVerificationRegistry). The plan requires new unit tests for these contracts.
- `gateway-contracts/test/upgrades/upgrades.ts` still references removed V1 contracts (CiphertextCommits, MultichainACL). These tests will fail now that the contracts are deleted.

## Phase 1B (commit e53c893b)
Build: `forge build` in `host-contracts/` (success, warnings only).

Findings:
- No missing plan items detected. KMSVerifierV2 with grace period + tests are present.

## Phase 2 (commit 81f0bf2e)
Build: `cargo build` in `coprocessor/fhevm-engine/` (success, warnings only).

Findings:
- Missing DB migration for V2 input verification requests. Code inserts into `input_verification_requests` with `commitment`/`user_signature`, but no migration exists for this table/columns.
- EIP-712 signing and epoch binding are not implemented: `/v1/verify-input` returns no signature; `/v1/ciphertext/{handle}` returns `epoch_id` hard-coded to `1` with empty signature.
- Event reconciliation/backfill for `InputVerificationRegistered` is not implemented (only KMSGeneration uses a catchup path).
- API unit tests are missing (`gw-listener/src/api/tests/` does not exist).
- InputVerificationRegistry contract addition belongs to Phase 1A but was included here, which undermines phase isolation.

## Phase 3 (commit 405b7d11)
Build: `cargo build` in `kms-connector/` (success).

Findings:
- KMS HTTP API server for `/v1/share/{requestId}` and `/v1/health` is not implemented (no `kms-worker/src/api` module).
- Coprocessor API ciphertext fetch + signature verification not implemented; the decryption processor logs "not yet implemented" and returns empty ciphertexts.
- Direct Host Chain ACL checks are not implemented (no ACL query module added).
- Event reconciliation/reorg safety for the KMS gw-listener is not implemented.
- API unit tests are missing; decryption-response tests are marked ignored instead of being updated for V2.

## Phase 4 (Relayer)
- Not present in this repository. No phase-4 commit found in the local git log; cannot validate here.

## Phase 5 (Cold Path)
- Explicitly out of scope per the plan.

---

# Follow-up After Fix Commits (2026-01-09)

## Phase 1A (squashed into 8e666c37)
Build: `forge build` in `gateway-contracts/` (success, warnings only).

Findings:
- Phase isolation still pending: InputVerificationRegistry changes still live in Phase 2 commit history. Requires an additional history edit if we want strict Phase 1A atomicity.
- V2 Foundry tests added; V1 upgrade tests cleaned up (no remaining gaps detected in this phase).

## Phase 2 (squashed into 81f0bf2e)
Build: `cargo build` in `coprocessor/fhevm-engine/` (success, warnings only).

Findings:
- No remaining functional gaps detected for Phase 2. Commit history still contains InputVerificationRegistry (phase isolation pending).

## Phase 3 (squashed into 405b7d11)
Build: `cargo build` in `kms-connector/` (success).
Tests: `cargo test -p kms-worker --lib --no-run` in `kms-connector/` (success).

Findings:
- Legacy S3-focused tests/modules remain (V1 path). Runtime no longer uses S3, but tests may need updating or pruning to fully drop legacy paths.

## Phase 4 (Relayer)
- Not present in this repository. No phase-4 commit found in the local git log; cannot validate here.

## Phase 5 (Cold Path)
- Explicitly out of scope per the plan.
