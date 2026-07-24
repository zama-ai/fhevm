# Solana proof service

Standalone service that ingests confirmed Solana blocks and serves ACL MMR
proofs (RFC-024 / fhevm-internal #1682).

This workspace holds:

- Yellowstone completed-block source (`solana-proof-source`)
- Bounded confirmed RPC recovery into the same `CompletedBlock` boundary
- Atomic PostgreSQL store + sequential ingest runner (`solana-proof-store`)
- Service binary with typed internal proof HTTP API + derived readiness
  (`solana-proof-service`)

The embedded relayer MMR path is deleted; test-suite compose boots this service for the Solana
vertical. Multi-replica / prod auth remain later slices.

## HTTP surface

| Route | Purpose |
| --- | --- |
| `GET /health/liveness` | Process responds |
| `GET /health/readiness` | Derived live proof readiness (never stores `ready=true`) |
| `GET /metrics` | Prometheus metrics (bounded labels only) |
| `GET /internal/solana/access-proof?encrypted_value=<base58>&handle=<hex32>&subject=<base58>` | Verified historical-access proof |
| `GET /internal/solana/public-proof?encrypted_value=<base58>&handle=<hex32>` | Verified public-decrypt proof |
| `GET /swagger-ui` / `GET /api-docs/openapi.json` | Generated OpenAPI |

Both proof endpoints are **semantic**: the caller asks the product question — "prove `subject`
had historical access to `handle`" or "prove `handle` is publicly decryptable" — and the service
resolves `(encrypted value account, handle[, subject], kind) → leaf_index` internally via one indexed lookup. A
historical-access key maps to a unique leaf by construction (a handle is superseded at most once
per encrypted value account, sealing one leaf per subject). A public-decrypt key may match several leaves for one
handle (a born-public output plus later `make_handle_public` re-releases, which have no
already-public guard); the service resolves to the earliest — any public leaf proves publicness,
and the earliest is deterministic and append-stable. Clients never compute, assume, or supply a
leaf index; `leaf_index` and `leaf_count` are OUTPUTS they pass through from the response.

`access-proof` serves `ZAMA_HIST_ACCESS_LEAF_V1` leaves (encrypted value account ‖ leaf_index ‖ handle ‖ subject);
`public-proof` serves `ZAMA_PUBLIC_DECRYPT_LEAF_V1` leaves (encrypted value account ‖ leaf_index ‖ handle).

Success proof DTO:

```json
{
  "mmr_proof": { "leaf_index": 0, "siblings": ["<hex>", "..."] },
  "leaf_index": 0,
  "leaf_count": 1,
  "rpc_context_slot": 1234,
  "encrypted_value_account_last_slot": 1230,
  "commitment": "confirmed",
  "proof_format_version": "v1",
  "verified": true,
  "status": "verified"
}
```

`leaf_index` is the resolved output (top-level, and inside `mmr_proof`); it is present only on a
verified proof.

Chain-context fields bind the served proof to observed state so a consumer can
reason about staleness when the store and the RPC provider see different
confirmed tips:

- `leaf_count` — encrypted value account leaf count the proof was built against.
- `rpc_context_slot` — confirmed Solana RPC context slot of the on-chain peak
  comparison that produced `verified`.
- `encrypted_value_account_last_slot` — per-encrypted value account durable ingest slot at which this encrypted value account's
  served leaves were last written (`solana_proof_encrypted_value_accounts.last_slot`). Omitted
  when no snapshot backed the response (store has not ingested the encrypted value account yet).
  This is deliberately distinct from the store's GLOBAL durable ingest checkpoint
  (`solana_proof_progress.checkpoint_slot`); surfacing that global checkpoint on
  the proof DTO remains a possible follow-up.
- `commitment` — commitment level of the on-chain authorization reads.
- `proof_format_version` — response wire-format marker (`v1`).

The `lagging` (503) and `corrupt_cache` (500) envelopes reuse this shape and
carry `leaf_count` + `rpc_context_slot` (the two tips being compared);
`encrypted_value_account_last_slot` appears only when a snapshot was in hand.

The proof path is **read-only**: SQL `proof_snapshot_for_leaf` (one consistent snapshot read that
also resolves the semantic key to its leaf index) + confirmed on-chain peak check; no
request-triggered catch-up writer.

**Operational rule — rebuild a pre-semantic store from genesis.** The semantic leaf columns
(`leaf_kind`, `handle`, `subject`) arrived in the `20260723120000` migration. A store whose leaf
rows were written before that migration carries NULL semantics on those rows, yet they still count
in `leaf_count`; a semantic query for a leaf that genuinely exists on chain would then resolve to
nothing and, at parity with chain, serve a terminal `404 leaf_not_found` instead of a proof. Such a
store MUST be rebuilt from genesis (drop + re-ingest from slot 0) — there is no backfill, because
the semantics cannot be recovered from a leaf hash. This build's ingest always populates the
columns, and startup validation fails closed if any NULL-semantic leaf row is found in a nonempty
store, so the trap surfaces as a loud boot failure rather than silent wrong 404s. (POC has no
persistent deployments today; the rule is written down against future ones.)

Invalid address / handle → HTTP 400 with typed `ErrorResponse`
(`code: invalid_address` / `invalid_handle`). Lagging store (behind **or briefly
ahead of** a different confirmed RPC) → HTTP 503 with `status: "lagging"`.
Equal-count peak divergence or snapshot inconsistency → HTTP 500 with
`status: "corrupt_cache"` (fail closed).

A semantic key that resolves to **no leaf** is classified against chain: while the snapshot's
`leaf_count` is still behind the live on-chain `leaf_count`, the miss is a retryable `503`
`status: "lagging"` (ingest has not caught up to a just-sealed leaf); once the snapshot is at
parity with chain, the miss is terminal — `404` with `status: "leaf_not_found"` (same proof
envelope, carrying the chain-context fields, `mmr_proof: null`). A encrypted value account with no on-chain
account at all is `404` `code: encrypted_value_account_not_found` (`ErrorResponse`).

Other client/server failures use the same
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
signals a contiguous parent-chain gap (`RecoveryRequired` / `Ancestry`), or when
a replay-capable geyser reports an **expired** inclusive cursor
(`ReplayCursorExpired`), the runner invokes bounded RPC `getBlocks`/`getBlock`
(confirmed), applies each normalized block **before** fetching the next (so the
durable checkpoint advances during catch-up), then **resubscribes from the
durable checkpoint** (inclusive replay). Cancellation during recovery is safe.

A geyser that rejects `from_slot` entirely (`ReplayUnsupported`) fails closed:
RPC catch-up cannot invent replay support, and resubscribing with `from_slot`
would loop forever. Cursorless live intake + ordered staging is tracked in
fhevm-internal #1823.

Errors are distinguished:

- **Transport** → `recovery_required` (retryable externally; fail closed for proofs)
- **History unavailable** (pruned / cleaned) → `recovery_required`, history stays incomplete
- **Bound exhaustion** (`max_slots_per_attempt` / `max_blocks_per_attempt`) → same; never silent complete
- **Ancestry conflict** on a recovered block apply → integrity halt
- **Empty recovery range** → `recovery_required` (never `Filled`, never marks complete)
- **Cancelled** mid-fetch / mid-apply → clean shutdown

Bootstrap **A**: with `recovery.bootstrap_slot` set, the first recovery attempt
fetches the bounded inclusive range from that slot through the observed
confirmed tip (`getSlot`), then may flip `history_complete=true` only when
durable `history_start` matches the configured bootstrap and the durable tip
equals that confirmed tip. Bound exhaustion stays fail-closed. Empty recovery
never marks complete.

## PoC gaps (non-prod TODOs)

- **O(n) proof construction:** each proof request loads every leaf for the
  encrypted value account and rebuilds peaks/siblings in memory. Acceptable only as an
  explicitly bounded PoC. **TODO(prod):** persisted MMR nodes / checkpoints so
  proof cost is logarithmic, plus measured encrypted value account size limits.
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

Docker (repo root context):

```bash
docker build -f solana-proof-service/Dockerfile -t solana-proof-service:local .
```

Solana vertical: `fhevm-cli up --scenario solana` starts compose service
`solana-proof-service` on `:8088`. Point clients at `PROOF_SERVICE_URL=http://127.0.0.1:8088`.
Full e2e: `NO_DNA=1 bash solana/scripts/e2e/clean-e2e.sh` then `TE_VALUE=55 bash solana/scripts/e2e/full-vertical.sh`.

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
