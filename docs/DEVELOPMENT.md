# Development Guide

This document covers building, testing, linting, and running the relayer locally.
It is intended for people who hack on the relayer code.
For operator-facing self-hosting instructions, see [SELF_HOSTING.md](SELF_HOSTING.md).

For contribution guidelines, see [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Table of Contents

- [First-time setup](#first-time-setup)
- [Running tests](#running-tests)
- [Local Stack](#local-stack)
- [Lint and format](#lint-and-format)
- [Database management](#database-management)
- [Docker builds](#docker-builds)
- [Troubleshooting](#troubleshooting)

## First-time setup

```bash
make setup    # Start Postgres, run migrations, copy config templates
```

This single command handles everything needed for local development:

1. Starts a local PostgreSQL instance via Docker Compose on **port 5433** (not the default 5432, to avoid conflicts with any system Postgres).
2. Runs database migrations.
3. Copies `config/local.yaml.example` to `config/local.yaml` if it does not already exist.

Run `make help` for the full list of available targets.

## Running tests

```bash
make test-unit                    # Fast unit tests (no Postgres required)
make test-all-no-long-running     # Full suite (requires Postgres)
```

## Local Stack

This mode runs the entire Zama protocol locally using `fhevm-cli` from the [fhevm repository](https://github.com/zama-ai/fhevm).

### Deploy the Zama Protocol

```bash
git clone git@github.com:zama-ai/fhevm.git
cd fhevm/test-suite/fhevm
./fhevm-cli deploy
```

This requires at least **12 GB** of Docker memory.

### Build and inject a local relayer

Build local images with registry-prefixed names, which is what the fhevm-cli Docker Compose stack expects:

```bash
LOCAL_RELAYER_TAG=local-relayer-$(date +%Y%m%d%H%M%S)
make docker-release TAG=${LOCAL_RELAYER_TAG}
```

Then upgrade the relayer in the fhevm stack:

```bash
# From fhevm/test-suite/fhevm
RELAYER_VERSION=${LOCAL_RELAYER_TAG} \
RELAYER_MIGRATE_VERSION=${LOCAL_RELAYER_TAG} \
./fhevm-cli upgrade relayer
```

### Validate the running image

```bash
docker inspect fhevm-relayer --format '{{.Config.Image}}'
docker inspect relayer-db-migration --format '{{.Config.Image}}'
```

### Run E2E tests via fhevm-cli

```bash
./fhevm-cli test input-proof
```

### Stop the local stack

```bash
./fhevm-cli clean
```

## Lint and format

```bash
make check          # fmt + clippy (quick pre-push gate)
make fix            # Auto-fix fmt + clippy issues
make clippy         # Clippy only
make fmt            # Format check only
```

`make check` is the recommended pre-push gate. It runs both `fmt --check` and `clippy` but does not start Postgres or run tests.

## Database management

### Lifecycle

```bash
make db-start       # Start local Postgres (port 5433) and wait for ready
make db-stop        # Stop Postgres (preserves data)
make db-destroy     # Stop Postgres and wipe all data
make db-reset       # Wipe and re-run migrations from scratch
make db-status      # Show container status + connection test
make db-shell       # Open psql shell
```

### sqlx-cli targets

These require `sqlx-cli`:

```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

Then:

```bash
make sqlx-migrate   # Run migrations via sqlx-cli
make sqlx-prepare   # Regenerate offline metadata for CI/Docker builds
```

The Docker build relies on pre-computed query metadata in `.sqlx/`. After adding or modifying SQL queries, run `make sqlx-prepare` before building Docker images.

## Docker builds

```bash
make docker-build           # Build relayer image
make docker-build-migrate   # Build relayer-migrate image
make docker-build-all       # Both of the above
make docker-release TAG=v0.9.0-rc.1  # Build with registry prefix
```

The relayer Dockerfile requires a real `.git/` directory (not a worktree) for build-time version embedding.
In a Git worktree, `.git` is a file rather than a directory, which causes the mount to fail. Build from a primary clone instead.

## Troubleshooting

See the [Troubleshooting section](../README.md#troubleshooting) in the main README.
