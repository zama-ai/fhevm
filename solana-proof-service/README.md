# Solana proof service

Standalone service that ingests confirmed Solana blocks and serves ACL MMR
proofs (RFC-024 / fhevm-internal #1682).

This workspace currently holds only the Yellowstone completed-block source.
SQL store, HTTP API, recovery, and multi-replica wiring land in later slices.

## PoC gaps (non-prod TODOs)

- **Single replica:** process restart or deploy causes brief API + ingest
  downtime. There is no rolling zero-downtime ingest handoff.
- **Internal only:** intended for localhost / Tailscale-class networks. No
  app auth, mTLS, or rate limits in v1.
- **Bootstrap A incomplete until recovery:** a fresh empty Postgres starts
  with `history_complete=false`. Continuity from the configured start is
  proven only by an explicit bounded recovery pass (not in this scaffold).

## Develop

```bash
make check
make fmt
make clippy
make test
```

Use `NO_DNA=1` for all Solana-related cargo commands (already set in the
Makefile targets).
