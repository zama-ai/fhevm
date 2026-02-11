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

Supported `--case` values:

`emit | sub | binary | unary | ite | cast | trivial | rand | rand-bounded | acl | all`

Explorer-visible CLI run (external validator + optional Docker Postgres):

Quickest path (one command):

```bash
/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-explorer-demo.sh
```

Cleanup behavior (default):

1. Stops validator started by the script after run completes.
2. Removes script-created ledger directory (`/tmp/solana-codex-ledger` by default).
3. Removes Docker Postgres container via runner default (`--docker-cleanup true`).

Keep artifacts only when needed:

```bash
/Users/work/.codex/worktrees/66ae/fhevm/test-suite/fhevm/scripts/solana-poc-explorer-demo.sh --keep-validator --keep-ledger
```

Manual path:

1. Start local validator with host program loaded under the canonical program id:

```bash
solana-test-validator \
  --reset \
  --ledger /tmp/solana-codex-ledger \
  --rpc-port 8899 \
  --faucet-port 9900 \
  --bpf-program Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq \
  /Users/work/.codex/worktrees/66ae/fhevm/solana/host-programs/target/deploy/zama_host.so
```

2. Run the PoC runner against that RPC:

```bash
cd /Users/work/.codex/worktrees/66ae/fhevm/coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo run -p solana-listener --features solana-e2e --bin solana_poc_runner
```

Defaults:

1. `rpc_url = http://127.0.0.1:8899`
2. `wallet = ~/.config/solana/id.json`
3. `postgres_mode = docker`
4. `publish_idl = true` (runs `anchor idl init`, falls back to `anchor idl upgrade`)

Common overrides:

```bash
SQLX_OFFLINE=true cargo run -p solana-listener --features solana-e2e --bin solana_poc_runner -- \
  --publish-idl false \
  --idl-path /absolute/path/to/idl.json \
  --program-id Fg6PaFpoGXkYsidMpWxTWqkZ4FK6s7vY8J3xA5rJQbSq
```

The runner prints:

1. request/allow tx signatures
2. explorer URLs (custom cluster URL pre-filled)
3. ingestion counters (`computations`, `allowed_handles`, `pbs_computations`)

Notes:

1. Tier 3 uses ignored integration tests and requires Docker, Anchor, and Solana CLI tooling.
2. `SQLX_OFFLINE=true` is recommended for deterministic local compilation of test binaries.
3. `solana_poc_runner` requires the target program id to match the program's declared id (Anchor `DeclaredProgramIdMismatch` otherwise).
4. Auto IDL publish requires `solana/host-programs/target/idl/zama_host.json` (or `--idl-path`) to exist.
