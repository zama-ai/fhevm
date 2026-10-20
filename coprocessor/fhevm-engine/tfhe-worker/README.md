# Fhevm Coprocessor

## Dependencies installation

- `docker-compose`
- `rust`
- `sqlx-cli`:
```
cargo install sqlx-cli
```

## Development

Start the database
```
docker compose up -d
```

Export database url for development
```
export DATABASE_URL="postgres://postgres:postgres@localhost/coprocessor"
```

Create the database
```
sqlx db create
```

Run the migrations
```
sqlx migrate run
```

## Debugging database

Exec into postgresql shell
```
docker exec -u postgres -it fhevm-coprocessor-db-1 psql coprocessor
```

## Running tests
```
cargo test
```
`operators_from_events` uses the full type matrix by default. To run a lighter local matrix (up to `uint64`) set `TFHE_WORKER_EVENT_TYPE_MATRIX=local` before `cargo test`.

### Artifact ingestion recovery regression

From `coprocessor/fhevm-engine`, run the CPU-only end-to-end regression with:

```sh
SQLX_OFFLINE=true cargo test --locked -p tfhe-worker \
  --test artifact_ingestion_e2e --profile coverage -- --test-threads=1
```

This requires Anvil on `PATH` and Docker for an isolated PostgreSQL database.
The normal test harness imports the repository's test keys. The tests deploy
event-emitting contracts on Anvil, omit the producer's ACL log on first delivery,
and feed the actual RPC logs through production block ingestion. A real
`tfhe_worker` process uses the production scheduler and CPU FHE implementation.
An independent computation must complete while the producer and its descendants
remain stalled.

The four cases exercise full-block replay with a warm cache, full-block replay
after recreating the listener, ACL-only replay after recreating the listener, and
an ACL database-write failure followed by retry on the same listener. Recovery
must repair the legacy and branch computation rows and the block's event counts,
then produce and correctly decrypt the producer, both children, and a grandchild.
Duplicate replay must preserve their ciphertexts and completion flags. These
tests run in the worker's CPU CI job, which installs Anvil.

The fault injection is at the block-log delivery boundary; these tests do not
exercise WebSocket reconnect/cursor persistence, Gateway commits, or KMS
decryption. Existing listener transport tests cover catch-up separately.
The fixed release branch should pass; disabling
`Database::upgrade_allowed_computations` reproduces the original failure at
the assertion that replay promotes the existing producer.

## Running the first working fhevm coprocessor smoke test

Reload database and apply schemas from scratch
```
make recreate_db
```
Run the background FHE worker

```
cargo run -- --run-bg-worker --worker-polling-interval-ms 1000
```
