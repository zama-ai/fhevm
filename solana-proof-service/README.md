# Solana proof service

Standalone service that ingests confirmed Solana blocks and serves ACL MMR
proofs (RFC-024 / fhevm-internal #1682).

This workspace holds:

- Yellowstone completed-block source (`solana-proof-source`)
- Atomic PostgreSQL store + sequential ingest runner (`solana-proof-store`)
- Service binary with typed internal proof HTTP API + derived readiness
  (`solana-proof-service`)

Bounded RPC recovery and multi-replica wiring land in later slices.

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
timeout (`code: timeout`), and a shed-on-saturate concurrency limit
(`code: overloaded`). Liveness / readiness / metrics are outside that gate so
probes cannot be starved. Upstream RPC uses a shorter 10s budget so chain
failures surface as typed `chain_error` inside the HTTP window.

**Readiness vs proof trust:** `/health/readiness` is the bootstrap / ingest gate
(history complete + writer live + Yellowstone successfully subscribed). After a
successful subscribe, program-filtered idle streams are healthy; never-connected
or reconnecting sources are `source_lagging`. Per-request proof trust is
peak-equality against confirmed chain state, not the readiness probe.

Readiness classifications: `database_unavailable`, `writer_missing`,
`source_lagging`, `history_incomplete`, `recovery_required`, `integrity_halted`.

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
- **Bootstrap A incomplete until recovery:** a fresh empty Postgres starts
  with `history_complete=false` on the first applied block. Continuity from
  the configured start is proven only by an explicit bounded recovery pass
  (`SqlProofStore::set_history_complete_after_recovery` is the seam; recovery
  itself is not implemented yet). Readiness stays `history_incomplete` /
  `recovery_required` until then.
- **Program-filtered Yellowstone gaps:** the source subscribes with
  `account_include` for the host program, so empty intermediate slots are
  omitted. Consecutive filtered blocks may not satisfy
  `parent_slot == previous applied slot`. Ingest still requires contiguous
  parent links and surfaces a gap as `RecoveryRequired` / source `Ancestry`
  (never a silent skip). **TODO:** bounded RPC recovery must fill missing
  blocks before live ingest can continue across gaps.
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

Use `NO_DNA=1` for all Solana-related cargo commands (already set in the
Makefile targets).
