# solana-listener (Solana host PoC)

PoC listener for Solana host ingestion, aligned with:

- `/Users/work/.codex/worktrees/66ae/fhevm/docs/protocol/explorations/solana-host-listener/INTERFACE_V0.md`

Current behavior:

1. finalized Solana RPC source -> canonical event envelopes
2. canonical event envelopes -> existing DB ingestion contracts
3. replay-safe cursor updates and idempotent writes

## Local feedback tiers

Tier 0 (fast mapping tests):

```bash
cd /Users/work/.codex/worktrees/66ae/fhevm/coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p solana-listener database::ingest::tests
```

Tier 2 scaffold (validator + parser smoke):

```bash
/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-tier2-localnet.sh
```

Tier 3 e2e (encrypt/request/compute/decrypt):

```bash
/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-tier3-e2e.sh --case all
```

Notes:

1. Tier 3 uses ignored integration tests and requires Docker, Anchor, and Solana CLI tooling.
2. `SQLX_OFFLINE=true` is recommended for deterministic local compilation of test binaries.
3. CLI mode writes to DB by default (`SOLANA_DRY_RUN=false`); set `SOLANA_DRY_RUN=true` for ingest preview without SQL writes.
