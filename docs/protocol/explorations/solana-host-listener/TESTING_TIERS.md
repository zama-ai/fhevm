# Solana Host Listener Testing Tiers

Date: 2026-02-09
Last synced: 2026-02-10
Status: Active

## Goal

Define a strict local feedback ladder with increasing confidence and cost:

1. very fast host-program loop (Mollusk)
2. fast listener mapping loop
3. DB-level ingestion/replay checks
4. local validator integration
5. full e2e extension

## Tier Ladder

```mermaid
flowchart TD
  Tm1["Tier -1: Mollusk host-program smoke (<=5s warm)"] --> T0["Tier 0: Mapping unit tests (<=5s)"]
  T0 --> T1["Tier 1: DB assertions (15-45s)"]
  T1 --> T2["Tier 2: Local validator + listener (1-3 min)"]
  T2 --> T3["Tier 3: Full coprocessor e2e (5-10 min)"]
```

## Tier Matrix

| Tier | Loop | Scope | Required infra | Current status |
|---|---|---|---|---|
| T-1 | <=5s (warm) | host-program execution smoke (`request_add`) | none | Implemented (Mollusk) |
| T0 | <=5s | Event -> canonical mapping only | none | Implemented |
| T1 | 15-45s | Canonical DB effects + cursor checks | Postgres | Implemented (assert script) |
| T2 | 1-3 min | Local validator + host program + listener ingest | Postgres + solana-test-validator | Implemented (real finalized RPC source + DB assertions) |
| T3 | 5-10 min | Scheduler/worker path after ingest + decrypt sanity | full local stack | Implemented (3 localnet ignored tests) |

## T-1: Mollusk Host Smoke

Purpose:

1. Validate host program instruction execution in-process with near-zero setup.
2. Keep a very fast red/green loop before touching validator/docker tiers.

Command:

```bash
cd /Users/work/.codex/worktrees/66ae/fhevm/solana/host-programs
anchor build
cargo test -p zama_host --test mollusk_smoke
```

Notes:

1. `request_add` smoke test is active and passing once program ELF exists (`target/deploy/zama_host.so`).
2. Test uses Anchor-generated instruction data (no hardcoded discriminators).

## T0: Fast Mapping Loop

Purpose:

1. Prove mapping contract quickly.
2. Catch regressions before touching infra.

Command:

```bash
cd /Users/work/.codex/worktrees/66ae/fhevm/coprocessor/fhevm-engine
cargo test -p solana-listener database::ingest::tests
```

Pass gates:

1. `request_add` maps to one `computations` action.
2. `allow` maps to one `allowed_handles` and one `pbs_computations` action.
3. Generic opcode events map to expected TFHE operations (`binary`, `unary`, `if_then_else`, `cast`, `trivial_encrypt`, `rand`, `rand_bounded`).
4. `schedule_order` remains deterministic.

## T1: DB Assertion Loop

Purpose:

1. Validate exact row-level outcomes.
2. Validate replay/idempotency behavior.

Script:

`/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-tier1-db-assert.sh`

Example:

```bash
/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-tier1-db-assert.sh \
  --database-url postgresql://postgres:postgres@localhost:5432/coprocessor \
  --tenant-id 1 \
  --host-chain-id 4242 \
  --expected-computations 1 \
  --expected-allowed 1 \
  --expected-pbs 1 \
  --min-cursor 1
```

Pass gates:

1. Exact expected counts for `computations`, `allowed_handles`, `pbs_computations`.
2. Cursor exists and `last_caught_up_block >= min_cursor`.

## T2: Localnet Integration Loop

Purpose:

1. Validate host program emission on local validator.
2. Validate listener ingestion path with restart/replay checks.

Script:

`/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-tier2-localnet.sh`

Current behavior:

1. validates prerequisites
2. runs Tier 0 tests
3. builds Anchor program
4. starts local validator
5. prepares localnet environment for Tier 2 listener ingestion checks

Primary in-process runner (Rust + testcontainers):

`/Users/work/.codex/worktrees/66ae/fhevm/coprocessor/fhevm-engine/solana-listener/tests/localnet_harness_integration.rs`

Command:

```bash
cd /Users/work/.codex/worktrees/66ae/fhevm/coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p solana-listener \
  --features solana-e2e \
  --test localnet_harness_integration \
  -- --ignored --nocapture --test-threads=1
```

Notes:

1. Starts Postgres and Solana validator in Docker via Rust `testcontainers`.
2. Runs DB migrations and validates finalized RPC readiness.
3. Builds and mounts host program, submits `request_add` + `allow`, ingests via real finalized RPC source, and asserts DB rows + cursor advancement.
4. Includes replay idempotency assertion (`new rows = 0`) on emit-only flow.
5. Includes worker-queue readiness assertion on ingested rows (`is_allowed=true`, `is_completed=false`, non-null `transaction_id`) so data is consumable by `tfhe-worker` dequeue query.

## T3: Full E2E Loop

Purpose:

1. validate post-ingest scheduler/worker behavior
2. validate end-to-end effect after canonical DB rows are produced
3. validate semantic correctness via decrypt sanity check

Primary test:

`localnet_solana_request_add_computes_and_decrypts`

Additional tests:

1. `localnet_solana_request_sub_computes_and_decrypts`
2. `localnet_acl_gate_blocks_then_allows_compute`

Runner script:

`/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-tier3-e2e.sh`

Example:

```bash
/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-tier3-e2e.sh --case all
```

Coverage:

1. Seeds tenant keys in DB.
2. Seeds `lhs` and `rhs` ciphertext handles via worker `trivial_encrypt` gRPC.
3. Emits Solana `request_add` / `request_sub` and ingests via finalized RPC source.
4. Asserts worker completes the queued computation and writes output ciphertext.
5. Decrypts output handle and asserts expected plaintext value.
6. ACL gate behavior (`emit!`): without `allow`, computation stays non-runnable; after `allow`, computation becomes runnable and completes.
7. This tier is currently non-CI by default (heavy Docker/Anchor/tooling prerequisites); run locally before merge when touching Solana host/listener e2e behavior.

## Canonical v0 Sanity Acceptance (single flow)

Flow:

1. `request_add -> ingest -> compute -> allow -> decrypt`.

Pass conditions:

1. exactly one `computations` row is inserted for the requested output handle.
2. row ordering is deterministic (`schedule_order = slot_time + tx_index + op_index`).
3. exactly one `allowed_handles` row and one `pbs_computations` row are inserted after `allow`.
4. replaying the same finalized range inserts `0` new rows.
5. worker completes the output handle (`is_completed=true`, `is_error=false`).
6. decrypted output plaintext matches expected arithmetic result.

Reference command:

```bash
cd /Users/work/.codex/worktrees/66ae/fhevm/coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p solana-listener \
  --features solana-e2e \
  --test localnet_harness_integration \
  localnet_solana_request_add_computes_and_decrypts \
  -- --ignored --nocapture --test-threads=1
```

## Hard Gates

For any tier >= T1:

1. `missed_ops = 0`
2. `duplicate_effects = 0`
3. deterministic ordering
4. same DB contract for `emit!`
