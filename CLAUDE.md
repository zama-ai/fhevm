# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

The **fhevm-relayer** is a Rust service that bridges fhevm (Fully Homomorphic Encryption Virtual Machine) blockchains with the Zama Gateway. It processes public decryption, user decryption, input proof verification, and FHE key material distribution. All source code lives under `apps/relayer/`.

## Development Commands

All commands run from `apps/relayer/`. Run `make help` for the full target list.

### First-Time Setup

```bash
make setup          # Start Postgres, run migrations, copy config templates
```

### Build

```bash
cargo build --release
cargo build --bin fhevm-relayer
```

### Test

```bash
make test-unit                  # Unit tests only (no Postgres needed)
make test-all-no-long-running   # Full suite (requires Postgres — starts automatically with make ci)
make test-smoke-ok-all-v2       # Smoke tests (v2 happy paths)
make test-coverage-report-html  # Coverage report

# Single integration test file
RUSTC_WRAPPER= cargo test --test public_decrypt_v2_test -- --test-threads=1

# Single test by name
RUSTC_WRAPPER= cargo test --test input_proof_v1_test test_success_single_request -- --test-threads=4
```

Note: `RUSTC_WRAPPER=` is needed to avoid sccache permission issues. The Makefile handles this automatically.

### Lint & Format

```bash
make check          # fmt + clippy (quick pre-push gate)
make fix            # Auto-fix fmt + clippy issues
make clippy         # Clippy only
make fmt            # Format check only
make api-lint       # Validate OpenAPI spec (requires npx)
```

### Database

```bash
make db-start       # Start local Postgres (port 5433) and wait for ready
make db-stop        # Stop Postgres (preserves data)
make db-migrate     # Run migrations (uses relayer-migrate binary, no sqlx-cli needed)
make db-reset       # Wipe and re-run migrations from scratch
make db-status      # Show container status + connection test
make db-shell       # Open psql shell

# sqlx-cli targets (requires: cargo install sqlx-cli --no-default-features --features postgres,rustls)
make sqlx-migrate   # Run migrations via sqlx-cli
make sqlx-prepare   # Regenerate offline metadata for CI/Docker builds
```

### Run the Service

```bash
make run            # Run with default config
make run-testnet    # Run against Testnet (validates config + private key)
make run-devnet     # Run against Devnet (validates config + private key)
make dev            # Hot-reload with cargo-watch
make health         # Hit /liveness, /healthz, /version, /metrics
```

### Network Onboarding

```bash
make init-testnet           # Copy Testnet config example, prompt for private key
make init-devnet            # Copy Devnet config example, prompt for private key
make preflight-testnet      # Verify wallet address + ETH/$ZAMA balances on Testnet
make preflight-devnet       # Verify wallet address + ETH/$ZAMA balances on Devnet
make mint-zama-testnet      # Instructions to get $ZAMA on Testnet (not self-service)
make mint-zama-devnet       # Mint $ZAMA on Devnet (self-service)
```

### Workflows

```bash
make ci             # Reproduce CI locally (lint → db-start → migrate → full test suite)
make check          # Quick pre-push check (fmt + clippy, no tests)
make fix            # Auto-fix formatting and clippy issues
```

### Docker

```bash
make docker-build           # Build relayer image
make docker-build-migrate   # Build relayer-migrate image
make docker-build-all       # Both of the above
make docker-release TAG=v0.9.0-rc.1  # Build with registry prefix
```

The relayer Dockerfile requires a real `.git/` directory (not a worktree) for build-time version embedding.

## Architecture

### Event-Driven Orchestrator

The system uses a custom event-driven orchestrator (`src/orchestrator/`):

- **Events** implement the `Event` trait (`traits.rs`) with `event_name()`, `event_id()`, `job_id()`, `timestamp()`
- **Handlers** implement `EventHandler<E>` and are registered to event IDs via `HandlerRegistry`
- **Dispatcher** (`TokioEventDispatcher`) spawns async tasks per event
- **Orchestrator** is a concrete struct wrapping `TokioEventDispatcher<RelayerEvent>` with health checking and task management

### Request Flow

```
HTTP API (v2) ──→ Orchestrator ──→ Gateway Handler ──→ Gateway Blockchain
                       ↑                                      │
Gateway Listener ←──────────────── Orchestrator ←─────────────┘
```

V2 endpoints return a `job_id` for async polling.

### Request Deduplication

Requests are deduplicated via SHA-256 content hashing (`ContentHasher` trait). The `JobId` type is `[u8; 32]`. The database enforces uniqueness on active requests via a partial unique index on `int_job_id`, so identical concurrent requests return the same `ext_job_id`.

### Key Source Directories

| Directory                         | Purpose                                                 |
| --------------------------------- | ------------------------------------------------------- |
| `src/http/endpoints/`             | API handlers (v1/ and v2/ subdirs)                      |
| `src/orchestrator/`               | Event system, dispatcher, handler registry              |
| `src/gateway/arbitrum/`           | Blockchain listeners, handlers, TX engine               |
| `src/store/sql/`                  | SQL repositories (sqlx + PostgreSQL)                    |
| `src/config/`                     | Hierarchical config loading (YAML → env vars → CLI)     |
| `src/metrics/`                    | Prometheus metrics definitions                          |
| `relayer-migrate/`                | Separate crate for DB migrations                        |
| `tests/`                          | Integration tests (each file is a separate test binary) |
| `test-support/ethereum_rpc_mock/` | Mock Ethereum RPC for tests                             |

### Integration Test Pattern

Integration tests use **per-test schema isolation** (`tests/common/test_schema.rs`). Each test creates a unique PostgreSQL schema (`test_<uuid>`), applies migrations, and cleans up on drop. Tests connect to `localhost:5433` by default (override with `TEST_DATABASE_URL`).

### Configuration

Hierarchical: YAML file (`config/local.yaml`) → environment variables with `APP_` prefix and `__` nesting (e.g., `APP_GATEWAY__BLOCKCHAIN_RPC__HTTP_URL`) → CLI args.

## Coding Conventions

From `CONTRIBUTING.md`:

- **Logging**: Log once at boundaries, not at every layer. ERROR for user-visible failures, WARN for degraded paths, INFO for state transitions, DEBUG for retries/decisions.
- **Error handling**: `thiserror` for library/typed errors, `anyhow` for application/boundary layers. Always add `.context()` when propagating.
- **Tracing**: Root span per request, child spans per dispatched handler. Mark error spans with `Status=ERROR` + OTEL exception fields.
- **Metrics**: Counters for requests/TX/events, histograms for latencies, gauges for lag/queue depth. Alerts come from metrics, not logs.
