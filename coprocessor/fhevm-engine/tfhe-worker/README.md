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
neighboring pairs rotating): every transfer consumes the balance tails of
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
derived from the primary metric. Points are reported under the same parameter
record every other benchmark uses, since that record is stored as typed columns;
the workload's own topology stays in the run artifact and in the test name,
which carries the scenario. The workflows fail if an expected scenario produced no artifact — or
if an artifact carries an unknown schema version or another commit's revision —
after sending the scenarios that did complete. The "batch size" input does not
apply to this set: each scenario derives its own work-item batch.

### Querying the results in Grafana

Slab stores each series in the PostgreSQL database its `--database` argument
names, so these runs land in a database called `coprocessor` — a different data
source from tfhe-rs's `tfhe_rs`, on the same server. Within it, `benchmark.metrics`
holds one row per point, joined to `benchmark.test`, `.hardware`, `.backend`,
`.branch` and `.project_version`.

Test names are `erc20::transfer::main_block_one_shot::<scenario>::<metric>`,
followed by the statistic and the run's name suffix, as every stored point is —
so `::`-splitting yields the scenario at position 4, and stripping from `_mean`
leaves the metric. Latencies are in nanoseconds; the rate metrics are already
per second.

The panels filter on hardware and branch, not on backend. Backend follows from
the machine — a GPU flavor only ever stores `cuda` — so filtering on it adds a
way for a panel to come back empty without adding a distinction. It is still
recorded, and the diagnostics below print it.

Primary metric per scenario, in milliseconds. The label differs between the
paced and unpaced workloads, which is why two are matched:

```sql
SELECT
  m.insert_time AS "time",
  split_part(test.name, '::', 4) AS metric,
  m.value / 1e6 AS latency_ms
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
JOIN benchmark.hardware AS h ON h.id = m.hardware_id
JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE $__timeFilter(m.insert_time)
  AND test.name LIKE 'erc20::transfer::main_block_one_shot::%'
  AND (test.name LIKE '%::worker_visible_to_terminals%'
       OR test.name LIKE '%::first_commit_to_final_terminals%')
  AND h.name = '$hardware'
  AND b.name = '$branch'
ORDER BY 1
```

FHE operations per second per scenario — swap the pattern for
`transfers_per_second` to chart transfer rate instead:

```sql
SELECT
  m.insert_time AS "time",
  split_part(test.name, '::', 4) AS metric,
  m.value AS fhe_ops_per_second
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
JOIN benchmark.hardware AS h ON h.id = m.hardware_id
JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE $__timeFilter(m.insert_time)
  AND test.name LIKE 'erc20::transfer::main_block_one_shot::%::fhe_operations_per_second%'
  AND h.name = '$hardware'
  AND b.name = '$branch'
ORDER BY 1
```

Transfer throughput per scenario — the rate of completed ERC20 transfers, which
is the headline number for these workloads. Each transfer is five FHE
operations, so `fhe_operations_per_second` is five times this and says the same
thing in operation terms; both are reported. Transaction identity is not a rate:
the scenarios vary it to isolate the cost of a transaction boundary, and dividing
by it would report a workload's fixture shape rather than its throughput.

```sql
SELECT
  m.insert_time AS "time",
  split_part(test.name, '::', 4) AS metric,
  m.value AS transfers_per_second
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
JOIN benchmark.hardware AS h ON h.id = m.hardware_id
JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE $__timeFilter(m.insert_time)
  AND test.name LIKE 'erc20::transfer::main_block_one_shot::%::transfers_per_second%'
  AND h.name = '$hardware'
  AND b.name = '$branch'
ORDER BY 1
```

A rate that reads as a whole number has been rounded somewhere. Check which end:

```sql
SELECT pg_typeof(m.value) AS value_type, m.value::text AS stored
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%::transfers_per_second%'
LIMIT 5
```

`double precision` with a fractional `stored` value means the panel is rounding:
set the field's Decimals, since Grafana's automatic choice drops the fraction on
values of this magnitude. An integer type means the store rounded, and no panel
setting brings the digits back.

That is recoverable, because the latency the rate derives from is recorded in
nanoseconds — integral at any useful precision — and each scenario's transfer
count is fixed by its definition. Divide in the query, keeping a column per
scenario so the series stay separate:

```sql
SELECT
  m.insert_time AS "time",
  max(300 / (m.value / 1e9)) FILTER (WHERE split_part(test.name, '::', 4) = 'independent_300') AS independent_300,
  max(1000 / (m.value / 1e9)) FILTER (WHERE split_part(test.name, '::', 4) = 'dependent_1000_50x20') AS dependent_1000_50x20,
  max(1000 / (m.value / 1e9)) FILTER (WHERE split_part(test.name, '::', 4) = 'cross_tx_dependent_1000_50x20') AS cross_tx_dependent_1000_50x20,
  max(300 / (m.value / 1e9)) FILTER (WHERE split_part(test.name, '::', 4) = 'auction_300') AS auction_300,
  max(1000 / (m.value / 1e9)) FILTER (WHERE split_part(test.name, '::', 4) = 'traffic_1000_200x5_10x100_lag2') AS traffic_1000_200x5_10x100_lag2,
  max(1000 / (m.value / 1e9)) FILTER (WHERE split_part(test.name, '::', 4) = 'traffic_join_1000_200x5_20acct_lag2') AS traffic_join_1000_200x5_20acct_lag2
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
JOIN benchmark.hardware AS h ON h.id = m.hardware_id
JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE $__timeFilter(m.insert_time)
  AND (test.name LIKE '%::worker_visible_to_terminals%'
       OR test.name LIKE '%::first_commit_to_final_terminals%')
  AND h.name = '$hardware'
  AND b.name = '$branch'
GROUP BY 1
ORDER BY 1
```

The count in each column is that scenario's own, which is why no CASE is needed;
they come from the scenario definitions above and change only if a canonical
workload is redefined, which also renames it. The two latency patterns cover the
paced and unpaced primary metrics.


Every point of a run carries the same `insert_time`: it is when Slab stored the
upload, not when a scenario ran. A run is therefore one position on the time
axis, with the suite's scenarios stacked above it — which looks like a single
vertical bar until a second run gives each series something to join to.

If those stacked values draw as one series rather than one per scenario, the
`metric` column is not being read as the series name: that convention applies
when the query's format is Time series, not Table. Naming a column per scenario
avoids relying on it and reads the same in either format, which is why the
panels below prefer it — any query returning a `metric` column can be rewritten
this way, and should be if its series collapse:

```sql
SELECT
  m.insert_time AS "time",
  max(m.value) FILTER (WHERE split_part(test.name, '::', 4) = 'independent_300') AS independent_300,
  max(m.value) FILTER (WHERE split_part(test.name, '::', 4) = 'dependent_1000_50x20') AS dependent_1000_50x20,
  max(m.value) FILTER (WHERE split_part(test.name, '::', 4) = 'cross_tx_dependent_1000_50x20') AS cross_tx_dependent_1000_50x20,
  max(m.value) FILTER (WHERE split_part(test.name, '::', 4) = 'auction_300') AS auction_300,
  max(m.value) FILTER (WHERE split_part(test.name, '::', 4) = 'traffic_1000_200x5_10x100_lag2') AS traffic_1000_200x5_10x100_lag2,
  max(m.value) FILTER (WHERE split_part(test.name, '::', 4) = 'traffic_join_1000_200x5_20acct_lag2') AS traffic_join_1000_200x5_20acct_lag2
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
JOIN benchmark.hardware AS h ON h.id = m.hardware_id
JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE $__timeFilter(m.insert_time)
  AND test.name LIKE 'erc20::transfer::main_block_one_shot::%::transfers_per_second%'
  AND h.name = '$hardware'
  AND b.name = '$branch'
GROUP BY 1
ORDER BY 1
```

Swap the metric in the `LIKE` to pivot a different one. Each column is a
scenario, so the legend names workloads rather than test paths, and a scenario
that did not report in some run leaves a gap instead of shifting another
scenario's line.

Latest run, as a table of every scenario and metric. `DISTINCT ON` keeps one row
per test name, and stripping the suffix leaves the metric label:

```sql
SELECT DISTINCT ON (test.name)
  split_part(test.name, '::', 4) AS scenario,
  regexp_replace(split_part(test.name, '::', 5), '_(mean|std_dev)_.*$', '') AS metric,
  m.value,
  pv.name AS commit,
  m.insert_time
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
JOIN benchmark.hardware AS h ON h.id = m.hardware_id
JOIN benchmark.branch AS b ON b.id = m.branch_id
LEFT JOIN benchmark.project_version AS pv ON pv.id = m.project_version_id
WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%'
  AND h.name = '$hardware'
  AND b.name = '$branch'
ORDER BY test.name, m.insert_time DESC
```

Transfer rate per workload per machine, grouped by commit, newest first. Cells
carry the rate and nothing else; a blank cell is a machine that commit was never
run on:

```sql
WITH scenario (bench, transfers, ord) AS (
  VALUES
    ('independent_300', 300, 1),
    ('dependent_1000_50x20', 1000, 2),
    ('cross_tx_dependent_1000_50x20', 1000, 3),
    ('auction_300', 300, 4),
    ('traffic_1000_200x5_10x100_lag2', 1000, 5),
    ('traffic_join_1000_200x5_20acct_lag2', 1000, 6)
), latest AS (
  SELECT DISTINCT ON (pv.name, s.bench, h.name)
    left(pv.name, 7) AS commit,
    s.bench,
    s.ord,
    h.name AS hardware,
    s.transfers::double precision / (m.value / 1e9) AS tps,
    m.insert_time
  FROM benchmark.metrics AS m
  JOIN benchmark.test AS test ON test.id = m.test_id
  JOIN scenario AS s ON s.bench = split_part(test.name, '::', 4)
  JOIN benchmark.hardware AS h ON h.id = m.hardware_id
  JOIN benchmark.branch AS b ON b.id = m.branch_id
  LEFT JOIN benchmark.project_version AS pv ON pv.id = m.project_version_id
  WHERE (test.name LIKE '%::worker_visible_to_terminals%'
         OR test.name LIKE '%::first_commit_to_final_terminals%')
    AND b.name = '$branch'
  ORDER BY pv.name, s.bench, h.name, m.insert_time DESC
), grouped AS (
  SELECT
    commit,
    bench,
    ord,
    max(tps) FILTER (WHERE hardware = 'n3-L40x1') AS l40,
    max(tps) FILTER (WHERE hardware = 'H100-1-80G') AS h100,
    max(insert_time) AS seen
  FROM latest
  GROUP BY commit, bench, ord
)
SELECT
  commit,
  bench,
  round(l40::numeric, 2) AS "n3-L40x1",
  round(h100::numeric, 2) AS "H100-1-80G"
FROM grouped
ORDER BY max(seen) OVER (PARTITION BY commit) DESC, commit, ord
```

The lookup carries each workload's transfer count and its position in the suite,
so both are declared once and joining to it also excludes anything that is not a
canonical scenario — where a CASE with a fallback would have quietly counted an
unknown workload as a thousand transfers. It holds no comments and no CASE, which
keeps its meaning if an editor delivers it as a single line.

The rate is derived from the primary latency rather than read from the stored
rate, which keeps its fractional digits whatever the store rounds to, and the two
latency patterns cover the paced and unpaced metrics. Commit groups are ordered
by their own most recent run, so a rerun of an older commit moves that whole
group to the top rather than interleaving its rows. Grafana's "Group to nested
tables" transformation on `commit` turns the repeated first column into
collapsible groups.

Each machine is a named column, which has to be edited when a new one appears.
To have the columns follow the data, select `commit, bench, hardware, tps` and
let Grafana pivot with "Rows to fields" — at the cost of ordering, which then
needs a run-time field to sort on.

Every workload on every machine, latest run each — the table for reading a whole
suite at once rather than one metric over time. Hardware is a column rather than
a filter, so it is not pinned here:

```sql
WITH latest AS (
  SELECT DISTINCT ON (test.name, h.name)
    split_part(test.name, '::', 4) AS scenario,
    regexp_replace(split_part(test.name, '::', 5), '_(mean|std_dev)_.*$', '') AS metric,
    h.name AS hardware,
    m.value,
    pv.name AS commit,
    m.insert_time
  FROM benchmark.metrics AS m
  JOIN benchmark.test AS test ON test.id = m.test_id
  JOIN benchmark.hardware AS h ON h.id = m.hardware_id
  JOIN benchmark.branch AS b ON b.id = m.branch_id
  LEFT JOIN benchmark.project_version AS pv ON pv.id = m.project_version_id
  WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%'
    AND b.name = '$branch'
  ORDER BY test.name, h.name, m.insert_time DESC
)
SELECT
  scenario,
  metric,
  hardware,
  CASE WHEN metric LIKE '%per_second' THEN value ELSE value / 1e6 END AS value,
  CASE WHEN metric LIKE '%per_second' THEN 'per second' ELSE 'ms' END AS unit,
  commit,
  insert_time
FROM latest
ORDER BY scenario, metric, hardware
```

Latency rows are converted to milliseconds and rates left alone, with the unit
named per row, since a suite's metrics do not share one. `DISTINCT ON
(test.name, h.name)` takes the most recent run per workload per machine, and the
`commit` column says which revision each row came from — machines are rarely
dispatched together, so a row's neighbor may be older than it looks. Grafana's
"Rows to fields" transformation will pivot hardware into columns if a matrix
reads better than a list.

Two caveats when comparing across machines. Runs carry the schedule and
optimization target in their test name, so rows from differently configured
dispatches are different tests and appear separately, which is intended but easy
to misread as duplication. And GPU runs use 16 CUDA streams per device by
default, which means a multi-GPU host also runs proportionally more of them —
`gpu_streams_per_device` in the run artifact records what a run used.

Dashboard variables, so a panel is not pinned to one machine or branch:

```sql
-- $hardware
SELECT DISTINCT h.name
FROM benchmark.hardware AS h
JOIN benchmark.metrics AS m ON m.hardware_id = h.id
JOIN benchmark.test AS test ON test.id = m.test_id
WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%'

-- $branch
SELECT DISTINCT b.name
FROM benchmark.branch AS b
JOIN benchmark.metrics AS m ON m.branch_id = b.id
JOIN benchmark.test AS test ON test.id = m.test_id
WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%'
```

### Watching the results evolve

The panels above are already time series: one point per run, per scenario, so a
line appears as runs accumulate. Two things keep such a line meaningful. Pin the
hardware and schedule — a line that mixes machines is measuring the fleet, not
the change. And keep a panel to one unit: latencies and rates differ
by seven orders of magnitude and share an axis badly.

Scenarios differ by that much between themselves too, which makes absolute
values awkward to compare across a suite. Normalizing each scenario against its
own first run in the window puts every workload on one axis, where the shape of
a regression is what stands out rather than the size of the workload:

```sql
SELECT
  m.insert_time AS "time",
  split_part(test.name, '::', 4) AS metric,
  100.0 * m.value / NULLIF(first_value(m.value) OVER (
    PARTITION BY test.name ORDER BY m.insert_time
    ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING), 0) AS pct_of_first
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
JOIN benchmark.hardware AS h ON h.id = m.hardware_id
JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE $__timeFilter(m.insert_time)
  AND test.name LIKE 'erc20::transfer::main_block_one_shot::%::worker_visible_to_terminals%'
  AND h.name = '$hardware'
  AND b.name = '$branch'
ORDER BY 1
```

Above 100 is slower than the window's first run, below is faster. Note that the
baseline moves with the dashboard's time range, so it answers "what changed
recently", not "how do we compare to a fixed reference".

The x axis is when the benchmark ran, not what it measured: a rerun of an old
commit lands to the right of a newer one. To read results by commit instead, plot
`pv.name` from `benchmark.project_version` on a bar chart, or list it:

```sql
SELECT
  pv.name AS commit,
  split_part(test.name, '::', 4) AS scenario,
  m.value / 1e6 AS latency_ms,
  m.insert_time
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
LEFT JOIN benchmark.project_version AS pv ON pv.id = m.project_version_id
WHERE $__timeFilter(m.insert_time)
  AND test.name LIKE 'erc20::transfer::main_block_one_shot::%::worker_visible_to_terminals%'
ORDER BY m.insert_time DESC, scenario
```

When a panel comes back empty, query without any filter first — in Grafana's
Explore with its format set to Table, or with psql — so the stored rows speak
for themselves. Left in the default time series format, Grafana rejects this
query for having no column named `time` rather than showing what is stored:

```sql
SELECT m.insert_time, test.name, bk.name AS backend, h.name AS hardware,
       b.name AS branch, m.value
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
LEFT JOIN benchmark.backend AS bk ON bk.id = m.backend_id
LEFT JOIN benchmark.hardware AS h ON h.id = m.hardware_id
LEFT JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%'
ORDER BY m.insert_time DESC
LIMIT 20
```

To find which predicate empties a panel without bisecting it by hand, count the
rows each one keeps. Run it as Table, with the dashboard's variables in place:

```sql
SELECT
  count(*) AS one_shot_points,
  count(*) FILTER (WHERE test.name LIKE '%::transfers_per_second%') AS transfers_points,
  count(*) FILTER (WHERE h.name = '$hardware') AS matching_hardware,
  count(*) FILTER (WHERE b.name = '$branch') AS matching_branch,
  count(*) FILTER (WHERE $__timeFilter(m.insert_time)) AS in_time_range,
  min(m.insert_time) AS earliest,
  max(m.insert_time) AS latest
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
LEFT JOIN benchmark.backend AS bk ON bk.id = m.backend_id
LEFT JOIN benchmark.hardware AS h ON h.id = m.hardware_id
LEFT JOIN benchmark.branch AS b ON b.id = m.branch_id
WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%'
```

Whichever count is zero is the filter to fix, and `earliest`/`latest` say whether
the panel's window covers the runs at all.

A count of zero against a dimension the rows plainly carry means the variable,
not the data. `SELECT '$hardware' AS hardware_var` in a table panel shows what
Grafana substitutes, which is empty for a variable whose option list was built
before the first run stored anything and cached — set its refresh to "on
dashboard load". Multi-value and Include All produce the same symptom by
rendering `'n3-L40x1,H100-1-80G'` or `'$__all'`; keep those variables
single-value, or write `h.name IN (${hardware:sqlstring})`.

Group the stored points to see what the variables have to match:

```sql
SELECT bk.name AS backend, h.name AS hardware, count(*) AS points
FROM benchmark.metrics AS m
JOIN benchmark.test AS test ON test.id = m.test_id
LEFT JOIN benchmark.backend AS bk ON bk.id = m.backend_id
LEFT JOIN benchmark.hardware AS h ON h.id = m.hardware_id
WHERE test.name LIKE 'erc20::transfer::main_block_one_shot::%'
GROUP BY 1, 2 ORDER BY 3 DESC
```

It also shows the hardware split, which is worth pinning: a panel left open
across machines charts the fleet rather than the change.

No rows at all means the data source is not connected to the `coprocessor`
database — the `benchmark` schema exists in `tfhe_rs` too, so a query against
the wrong database succeeds and returns nothing. Rows here but not in the panel
means a filter disagrees with what was stored: compare the printed `backend`,
`hardware` and `branch` against the dashboard variables, remembering that GPU
runs store `cuda`.

A single run is also invisible in a time series by default: one point per series
draws no line. Set the panel's "Show points" to Always, or read the first runs
from the latest-run table above, until several runs accumulate. If the rows fall
outside the panel's window, widen the range — `insert_time` is recorded in UTC
and a dashboard on browser time can put a fresh point outside "last hour".

A scenario comparison is a scenario filter away: add
`AND split_part(test.name, '::', 4) IN ('dependent_1000_50x20',
'cross_tx_dependent_1000_50x20')` to contrast the intra-transaction control
with its cross-transaction twin, which is the pair that isolates the
transaction-boundary materialization cost.
