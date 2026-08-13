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

### Running the canonical suite

`BENCH_ONE_SHOT_CANONICAL_SCENARIOS` in the Makefile lists the canonical
workloads in reporting order, and the suite targets run each of them in turn:

```
FHEVM_BENCH_DATABASE_URL=postgresql://... \
  make benchmark_erc20_main_block_one_shot_suite_gpu \
  BENCH_ONE_SHOT_RECREATE_DB=1
```

`FHEVM_BENCH_ISOLATED_DB=1` only requires an explicitly named database, it does
not create one per run, so without `BENCH_ONE_SHOT_RECREATE_DB=1` every scenario
runs against the rows the previous ones left behind — progressively larger
tables and a residual processed-DCID backlog. Topology assertions still hold
(they are scoped by randomized transaction IDs), but the timings are not
comparable to CI, which always recreates.

The reportable targets build without LTO, for the benchmark and for its
dependencies. `cargo bench` compiles the benchmark target with the `bench`
profile but its dependencies with `release`, where this workspace sets fat LTO,
so disabling it for one profile alone both left the dependency graph — tfhe
included — linked with LTO and recorded the opposite in the run artifact.
Disabling it for both is what makes `bench_lto` true, and it is most of the
build time.

A failing scenario does not abort the suite; the remaining scenarios still run
and the target exits non-zero at the end. `BENCH_ONE_SHOT_SCENARIOS="a b"`
restricts the run to a subset of `BENCH_ONE_SHOT_CANONICAL_SCENARIOS` (an empty
or unrecognized list is rejected), and `make print_bench_one_shot_scenarios`
prints what a suite run would cover. Do not override
`BENCH_ONE_SHOT_SCENARIO` (singular) on a suite target: it pins every iteration
to one scenario.

### Reporting in CI

The `coprocessor-benchmark-cpu` and `coprocessor-benchmark-gpu` workflows run
the suite when their "Benchmark set" input is `main_block_one_shot`, with the
`one_shot_scenarios` input restricting it to a subset. Reportable one-shot runs
bypass Criterion and write one JSON artifact per scenario under
`target/criterion/benchmark-runs`, so the workflows parse them with
`ci/benchmark_one_shot_parser.py` (rather than `ci/benchmark_parser.py`) before
sending the points to Slab. Each scenario reports its primary metric and the
commit-start upper bound as latencies, plus FHE-operation and transfer rates
derived from the primary metric, with the workload topology attached as
parameters. The workflows fail if an expected scenario produced no artifact — or
if an artifact carries an unknown schema version or another commit's revision —
after sending the scenarios that did complete. The "batch size" input does not
apply to this set: each scenario derives its own work-item batch.
