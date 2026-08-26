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

## Running the first working fhevm coprocessor smoke test

Reload database and apply schemas from scratch
```
make recreate_db
```
Run the background FHE worker

```
cargo run -- --run-bg-worker --worker-polling-interval-ms 1000
```

## Reportable Main baseline ERC20 workloads

The reportable Main one-shot targets default to `independent_300`: 300
independent five-operation ERC20 transfers in one logical block. Canonical
workload #2 is `dependent_1000_50x20`: 1,000 transfers arranged as 50
independent chains of 20 dependent transfers. Each chain forwards both
encrypted balances from one transfer to the next and exposes only its final
two balance outputs (100 terminal outputs total). The reportable Make target
automatically raises Main's work-item batch to 5,000 for this scenario, so all
50 chains are acquired in one scheduling batch; callers can override that
value only for an intentional dispatch-shape experiment.

Select it with `BENCH_ONE_SHOT_SCENARIO=dependent_1000_50x20`, for example:

```
FHEVM_BENCH_DATABASE_URL=postgresql://... \
  make benchmark_erc20_main_block_baseline_gpu \
  BENCH_ONE_SHOT_SCENARIO=dependent_1000_50x20
```

Canonical workload #3 is `traffic_1000_200x5_10x100_lag2`: 1,000 transfers
arriving as 200 immediately committed logical L1 blocks of five transfers.
It contains ten balance chains of 100 transfers. A transfer in block `n >= 2`
uses the sender and destination balance outputs from the same chain in block
`n - 2`; even and odd blocks therefore advance disjoint sets of five chains.
The harness does not drain, sleep, or otherwise pace between commits, so
worker computation overlaps fixture ingestion. Its primary metric starts just
after the first block commit and ends when the final 20 balance outputs (two
per chain across blocks 198 and 199) are worker-visible.

Select it with `BENCH_ONE_SHOT_SCENARIO=traffic_1000_200x5_10x100_lag2`.

Both traffic workloads stage through the REAL host-listener ingestion path:
each L1 block is a `BlockLogs` of raw TFHE + ACL logs handed to
`ingest_block_logs`, so dependence chains come from production formation
(cross-block linear extension, join gating) rather than hand-written
`dependence_chain` rows, and the fixture cannot drift from formation
semantics. Carried balances are allowed on every link, matching the
production ACL rule.

Canonical workload `traffic_join_1000_200x5_20acct_lag2` keeps workload #3's
cadence but braids value between twenty accounts (two parity decks of ten,
neighbouring pairs rotating): every transfer consumes the balance tails of
two different account chains, so no linear extension is possible and
formation must gate each transfer on both parents. This is the worst-case
join shape for dependence-chain scheduling.

Select it with `BENCH_ONE_SHOT_SCENARIO=traffic_join_1000_200x5_20acct_lag2`.

Canonical workload `cross_tx_dependent_1000_50x20`: 1,000 transfers in one
logical L1 block, arranged as 50 independent balance chains of 20. Unlike
`dependent_1000_50x20` (which deliberately uses one transaction ID per chain
as an intra-transaction dependency control), each transfer in a chain receives
its own transaction ID while the chain retains one dependence chain (DCID).
Each of the 950 successor links therefore carries both balances across a
transaction boundary (1,900 cross-transaction balance dependencies), directly
measuring the transaction-boundary materialization path. Carried balances are
marked allowed, matching the production ACL rule that a handle consumed by
another transaction receives a persistent allow() by consumption time —
without it, Main never schedules the consuming transactions at all. It has the same
5,000-operation staging shape as the dependent control and is acquired in one
Main scheduling batch by default.

Select it with `BENCH_ONE_SHOT_SCENARIO=cross_tx_dependent_1000_50x20`.
