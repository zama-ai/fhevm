# Solana proof service

Standalone service that ingests confirmed Solana blocks and serves ACL MMR
proofs (RFC-024 / fhevm-internal #1682).

This workspace currently holds:

- Yellowstone completed-block source (`solana-proof-source`)
- Atomic PostgreSQL store + sequential ingest runner (`solana-proof-store`)

HTTP proof API, bounded RPC recovery, and multi-replica wiring land in later
slices.

## PoC gaps (non-prod TODOs)

- **Single replica:** process restart or deploy causes brief API + ingest
  downtime. There is no rolling zero-downtime ingest handoff.
- **Internal only:** intended for localhost / Tailscale-class networks. No
  app auth, mTLS, or rate limits in v1.
- **Bootstrap A incomplete until recovery:** a fresh empty Postgres starts
  with `history_complete=false` on the first applied block. Continuity from
  the configured start is proven only by an explicit bounded recovery pass
  (`SqlProofStore::set_history_complete_after_recovery` is the seam; recovery
  itself is not implemented yet).
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
