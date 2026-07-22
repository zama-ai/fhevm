# Solana proof service

Standalone service that ingests confirmed Solana blocks and serves ACL MMR
proofs (RFC-024 / fhevm-internal #1682).

This workspace holds:

- Yellowstone completed-block source (`solana-proof-source`)
- Bounded confirmed RPC recovery into the same `CompletedBlock` boundary
- Atomic PostgreSQL store + sequential ingest runner (`solana-proof-store`)
- Service binary with typed internal proof HTTP API + derived readiness
  (`solana-proof-service`)

Multi-replica wiring and embedded-relayer deletion land in later slices.

## HTTP surface

| Route | Purpose |
| --- | --- |
| `GET /health/liveness` | Process responds |
| `GET /health/readiness` | Derived live proof readiness (never stores `ready=true`) |
| `GET /metrics` | Prometheus metrics (bounded labels only) |
| `GET /internal/solana/mmr-proof?encrypted_value=<base58>&leaf_index=<u64>` | Verified MMR proof |
| `GET /swagger-ui` / `GET /api-docs/openapi.json` | Generated OpenAPI |

Success proof DTO (preserved until #1721):

```json
{
  "mmr_proof": { "leaf_index": 0, "siblings": ["<hex>", "..."] },
  "leaf_count": 1,
  "proof_slot": 1,
  "verified": true,
  "status": "verified"
}
```

The proof path is **read-only**: SQL `proof_snapshot` + confirmed on-chain peak
check; no request-triggered catch-up writer.

Invalid `leaf_index` (≥ on-chain `leaf_count`) → HTTP 400 with typed
`ErrorResponse` (`code: leaf_index_out_of_range`). Lagging store (behind **or
briefly ahead of** a different confirmed RPC) → HTTP 503 with
`status: "lagging"`. Equal-count peak divergence or snapshot inconsistency →
HTTP 500 with `status: "corrupt_cache"` (fail closed; wire name preserved for
relayer DTO parity until #1721). Other client/server failures use the same
`ErrorResponse` JSON envelope. Proof routes get an `x-request-id`, a 30s typed
timeout (`code: timeout`), and a shed-on-saturate concurrency limit sized to
`max_connections - 2` so ingest and readiness keep reserved DB pool slots
(`code: overloaded`). Liveness / readiness / metrics are outside the HTTP proof
gate; readiness DB probes time out after 2s and the pool uses a 2s acquire
timeout. Upstream RPC uses a shorter 10s budget so chain failures surface as
typed `chain_error` inside the HTTP window.

**Readiness vs proof trust:** `/health/readiness` is the bootstrap / ingest gate
(history complete + writer live + at least one Applied / AlreadyApplied on the
live stream, and not mid-recovery). A bare Yellowstone subscribe is not enough;
cursor continuity must be proven by progress. After that, program-filtered idle
streams are healthy; never-proven or reconnecting sources are `source_lagging`.
In-flight bounded RPC recovery reports `recovery_required`. Per-request proof
trust is peak-equality against confirmed chain state, not the readiness probe.

Readiness classifications: `database_unavailable`, `writer_missing`,
`source_lagging`, `history_incomplete`, `recovery_required`, `integrity_halted`.

## Recovery

Live ingest remains **filtered confirmed Yellowstone**. When the store or source
signals a contiguous parent-chain gap (`RecoveryRequired` / `Ancestry`), the
runner invokes bounded RPC `getBlocks`/`getBlock` (confirmed), normalizes into
the same `CompletedBlock` shape (program-filtered txs, sparse indexes), applies
through `SqlProofStore::apply_completed_block`, then **resubscribes from the
durable checkpoint** (inclusive replay). Cancellation during recovery is safe.

Errors are distinguished:

- **Transport** → `recovery_required` (retryable externally; fail closed for proofs)
- **History unavailable** (pruned / cleaned) → `recovery_required`, history stays incomplete
- **Bound exhaustion** (`max_slots_per_attempt` / `max_blocks_per_attempt`) → same; never silent complete
- **Ancestry conflict** on a recovered block apply → integrity halt
- **Empty recovery range** → `recovery_required` (never `Filled`, never marks complete)
- **Cancelled** mid-fetch / mid-apply → clean shutdown

Bootstrap **A**: first applied block keeps `history_complete=false`.
`history_complete=true` only after an explicit recovery pass proves continuity
from configured `recovery.bootstrap_slot` through the **confirmed tip** that
recovery established (`durable_tip.slot == confirmed_tip`). Single-slot
bootstrap alone does not flip the flag when the chain tip is ahead. Empty
recovery never marks complete.

## PoC gaps (non-prod TODOs)

- **O(n) proof construction:** each proof request loads every leaf for the
  lineage and rebuilds peaks/siblings in memory. Acceptable only as an
  explicitly bounded PoC. **TODO(prod):** persisted MMR nodes / checkpoints so
  proof cost is logarithmic, plus measured lineage size limits.
- **Single replica:** process restart or deploy causes brief API + ingest
  downtime. There is no rolling zero-downtime ingest handoff.
- **Internal only:** intended for localhost / Tailscale-class networks. No
  app auth, mTLS, or rate limits in v1. **TODO(prod):** add auth before any
  broader exposure.
- **Recovery horizon:** default `max_slots_per_attempt=256` /
  `max_blocks_per_attempt=128` are lean PoC bounds. Exhaustion fails closed
  (`recovery_required` / incomplete history). **TODO(prod):** size horizons
  against archive retention and catch-up SLOs; optional longer tip chase.
- **Ops:** schema is service-owned. Apply migrations via
  `SqlProofStore::migrate` (or `sqlx migrate`) before ingest. Compile-checked
  queries require committed `.sqlx` metadata (`make sqlx-prepare` against a
  live `DATABASE_URL`).

## Run

```bash
# from solana-proof-service/
export SOLANA_PROOF_CONFIG_PATH=config/app.yaml
# or override: SOLANA_PROOF__DATABASE__CONNECTION_STRING=postgres://...
NO_DNA=1 cargo run -p solana-proof-service
```

## Develop

```bash
make check
make fmt
make clippy
make test
```

Postgres integration tests (ignored by default in `make test`):

```bash
export DATABASE_URL='postgres://work@127.0.0.1:55432/solana_proof_service'
make test-db
```

Use `NO_DNA=1` for Solana-related cargo/CLI commands (already set in the
Makefile targets). It marks the caller as a non-human operator
([no-dna.org](https://no-dna.org/)) so compatible tools skip interactive
prompts/spinners and emit machine-friendly output — useful for agents and CI.
