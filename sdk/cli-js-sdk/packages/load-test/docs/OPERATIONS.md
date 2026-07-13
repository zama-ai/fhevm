# Load-Test Operator Guide

The short version: set credentials, then run **one command**:

```bash
cd sdk/cli-js-sdk/packages/load-test
node --import tsx index.ts --relayer-url <relayer> suite run standard
```

That plans pool requirements for every scenario in the suite, generates or
creates whatever is missing (payloads off-chain, handles on-chain), runs the
scenarios sequentially, and writes per-run reports plus a suite summary.
Everything below is detail.

## 1. One-time setup

Put credentials in the workspace-root `.env` (`sdk/cli-js-sdk/.env`):

| Variable | Required for | Notes |
| --- | --- | --- |
| `MNEMONIC` | handle pools | preferred: enables parallel funded HD lanes (`--lanes`, default 4) |
| `PRIVATE_KEY` | handle pools | alternative; single lane only |
| `DELEGATE_PRIVATE_KEY` | delegated pools without `MNEMONIC` | sign-only, needs no funds |
| `SEPOLIA_RPC_URL` | `testnet`, `devnet` handle pools | Ethereum Sepolia host-chain RPC for FHETest transactions |
| `POLYGON_AMOY_RPC_URL` | `devnet-amoy` handle pools | Polygon Amoy host-chain RPC for FHETest transactions |
| `MAINNET_RPC_URL` | `mainnet` handle pools | Ethereum Mainnet host-chain RPC for FHETest transactions |
| `LOAD_TEST_RELAYER_URL` | everything | or pass `--relayer-url` |
| `LOAD_TEST_RELAYER_CONFIG` | config snapshot | optional; path to the relayer YAML in effect |
| `LOAD_TEST_DATA_DIR` | pools/runs root | default `.load-test` |

Funding: each handle costs one FHETest setter transaction (plus one
delegation transaction per lane for delegated pools). Lanes are checked for
a minimum balance before any transaction is sent and the run fails with the
addresses to fund.

## 2. The single command: suites

```bash
node --import tsx index.ts suite list            # what exists
node --import tsx index.ts suite plan standard   # no generation/on-chain writes; validates pool storage
node --import tsx index.ts suite run standard    # prepare + run + summarize
```

Built-in suites and when to run them:

| Suite | Contents | Duration | Question it answers |
| --- | --- | --- | --- |
| `smoke` | open steady 1 req/s ×60s, drain 20 | ~3 min | is this deployment functional? |
| `standard` | baseline, open-steady-10, open-mixed-10, drain-500 | ~45 min | did anything regress? (nightly set) |
| `capacity` | open-ramp (stops at saturation), open-spike | ≤ ~40 min | what is max sustainable throughput, and does it recover? |
| `endurance` | open-soak 5 req/s ×60 min | ~70 min | leaks/drift in the relayer process? |

Useful flags on `suite run`:

- `--prepare-only` — build all pools now (e.g. ahead of a scheduled window),
  run nothing.
- `--skip-prepare` — fail instead of creating anything (CI guard against
  accidental on-chain spend).
- `--baselines-dir <dir>` (default `baselines`) — each scenario is diffed
  against `<dir>/<network>/<label>.json` when present; regressions fail the
  suite.
- `--out <dir>` — output root; default `<data-dir>/runs/<timestamp>-suite-<name>/`.

Custom suites are JSON:

```json
{
  "name": "pre-release",
  "pauseSec": 60,
  "entries": [
    { "scenario": "open-steady", "params": { "rps": 25, "durationSec": 600 }, "label": "open-steady-25" },
    { "scenario": "scenarios/my-mixed.json" },
    { "scenario": "drain", "params": { "count": 2000, "rps": 200 } }
  ]
}
```

```bash
node --import tsx index.ts suite run suites/pre-release.json
```

Custom scenario and suite JSON is operator-local and ignored under
`scenarios/` and `suites/`. Paths are resolved from the load-test process
working directory, not from the suite file's directory, so suite entries must
use package-relative paths as above or absolute paths.
`params` overrides apply to built-in scenario names; a JSON scenario carries
its complete configuration in its own file.

Suites run scenarios **sequentially** with a pause in between — never run two
load tests against the same relayer at once; they corrupt each other's
measurements.

## 3. Outputs

```text
<out>/
├── suite-summary.md       # one row per scenario: errors, verify failures, thresholds, diff
├── suite-summary.json
└── <label>/               # one directory per scenario run
    ├── report.md          # human report
    ├── report.json        # canonical; diffable
    ├── requests.jsonl     # per-request records (requestId ↔ jobId correlation)
    ├── metrics-a.jsonl    # target A /metrics snapshots
    └── metrics-b.jsonl    # optional target B /metrics snapshots
```

Exit code is non-zero if any scenario breaches its thresholds or regresses
against its baseline — safe to wire into CI directly.

## 4. Lower-level commands (what `suite run` automates)

```bash
# Pools
node --import tsx index.ts pool add --flow input-proof --count 2000   # off-chain, CPU-bound
node --import tsx index.ts pool add --flow public-decrypt --count 40
node --import tsx index.ts pool add --flow user-decrypt --count 4
node --import tsx index.ts pool add --flow delegated-user-decrypt --count 4
node --import tsx index.ts pool inspect

# Single scenario
node --import tsx index.ts run open-steady --rps 20 --duration 600 --baseline baselines/testnet/open-steady-20.json
node --import tsx index.ts run closed-steady --vus 20 --duration 600 --flow user-decrypt
node --import tsx index.ts run closed-ramp --vus 5 --duration 120 --flow delegated-user-decrypt
node --import tsx index.ts run closed-steady --vus 20 --duration 600 --think-time 1000 --flow user-decrypt

# Reports
node --import tsx index.ts report render <run-dir>
node --import tsx index.ts report diff <baseline.json> <report.json>
```

Flow-specific pool options are enforced: `--threads` applies only to
input-proof generation, handle concurrency/lane options apply only to handle
pools, and `--delegation-days` applies only to delegated-user-decrypt. Inspect
reports every delegated owner ACL as healthy, expired, or missing.

Choose the model by the product question:

- **Open** (`open-steady`, `open-ramp`, `open-spike`, `open-soak`,
  `open-mixed`): fixes arrivals. Use it for "Can the relayer sustain X new
  requests/sec?" and capacity discovery.
- **Closed** (`closed-steady`, `closed-ramp`, `closed-soak`): fixes active
  clients. Use it for "Can Y active SDK clients run with acceptable latency,
  and does the resulting throughput meet acceptance criteria?" A VU starts
  the next request only after the previous request reaches terminal state or
  times out.
- **Drain** (`drain`): fixes backlog size. Use it for "If N jobs are queued,
  do they complete correctly and drain at the expected configured rate?"

`--think-time` is the closed-model pause after one complete workflow. It is
not the poll interval. Polling waits come from `Retry-After` while the current
job is still in progress.

Pool sizing rules the planner applies (so you can reason about costs):

- **input-proof**: one payload per scheduled request, single-use. Mixed-flow
  allocations exactly match the scheduler's smooth weighted round-robin.
- **public-decrypt**: each request consumes a unique handle *combination*;
  n handles serve C(n, k) requests (built-ins use k=2, so 40 handles ≈ 780
  requests). On-chain cost is n transactions, not the request count.
- **user-decrypt / delegated**: handles are reusable; 4 per pool is plenty
  at any rate.

Delegated pools also carry one ACL expiration per owner lane. Suite planning
treats an ACL refresh as preparation independent of handle count and extends all
owners through the estimated suite end (submission, request/drain windows,
pauses, and a safety margin). A count-ready pool can therefore still show
`needs ACL refresh`.

### Pool storage and durability

Pool schema v2 stores each item generation as an immutable
`items-<sha256>.jsonl` snapshot. `meta.json` names the committed snapshot and
is the sole commit pointer. Writers are serialized across processes, stream
records with awaited backpressure, fsync the completed snapshot, and only then
atomically publish new metadata. If a writer is killed, readers keep using the
previous snapshot; a process-liveness lock and orphan-temp cleanup allow the
next writer to recover. Pool paths, metadata, and snapshots must be real
directories and regular files—symbolic links are rejected.

Writer locks are local-host only. They record the hostname, PID, process-start
identity where the operating system exposes one, and a random owner token. A
dead local owner is reaped. On Linux and macOS, when the operating system
permits the process query, a live PID whose verified start identity differs is
also recognized as PID reuse. A live or suspended owner is never expired
merely because its heartbeat is old, and a lock from another host is never
reaped.

Do not run concurrent pool writers against the same data directory from
different hosts, including through NFS or another shared filesystem. Move or
copy a completed immutable pool between hosts instead. A writer verifies its
owner token after flushing the replacement metadata and immediately before the
atomic `meta.json` rename.

If a local lock remains blocked, inspect `.writer.lock/owner.json` (or the
cursor's adjacent `.lock/owner.json`). Confirm the hostname, then terminate the
recorded PID and verify that it has exited before retrying. Do not remove a lock
directory while its recorded process is alive: that process may still publish.
On a platform without verified process-start identities, this terminate-and-
retry procedure is also the safe response to suspected PID reuse. Foreign-host
locks require checking and stopping the owner on that host; never infer foreign
liveness from a local PID.

SHA-256 here detects accidental corruption and inconsistent snapshots; it is
not a signature or authenticity boundary. The parent of `LOAD_TEST_DATA_DIR`
is a trusted-path boundary: no untrusted process should be able to rename path
components or replace entries there while the load test runs. Protect it with
normal host access controls and do not consume pools supplied by an untrusted
party.

The durability contract is strongest on local POSIX filesystems: files are
fsynced before rename and directory entries are fsynced where the operating
system supports it. Windows and filesystems that reject directory fsync still
get exclusive temp files, file flushes, and atomic rename, but power-loss
durability of the directory entry depends on the platform/filesystem. Network
filesystems are unsupported for concurrent/shared pool writers even if they
offer atomic rename; use them only to transport a pool while no writer is
active.

Pool creation uses an owner-marked sibling temporary directory. A later local
creator removes only temporary directories whose recorded local process is
dead (or malformed marker directories older than the safety grace period).
Foreign-host owner markers are retained for manual inspection and cleanup.

Committed snapshots are intentionally retained: a lock-free reader may have
observed the previous metadata immediately before a writer publishes the next
version. To reclaim old `items-*.jsonl` files, stop every process using the
pool, read the `itemsFile` named by `meta.json`, and remove only the other
snapshots. Never prune a live pool.

## 5. Routine runbooks

**Suggested nightly regression:** run `suite run standard` with the
committed baselines; investigate any ❌ row in `suite-summary.md`, starting
with the per-run `report.md` **Diagnosis** section — it names the dominant
stage, any saturated limiter, and whether the fix is a config knob or
downstream capacity. The stage `Share` column and the **Backlog by Stage**
table localize *where* time/queue went.

**Before/after a relayer change:**

```bash
node --import tsx index.ts suite run standard --out runs/before   # on the old build
# deploy the new build
node --import tsx index.ts suite run standard --out runs/after
node --import tsx index.ts report diff runs/before/open-steady-10/report.json runs/after/open-steady-10/report.json
```

**Capacity check on a new environment:** `suite run capacity`. A ramp only
stops early when the target's Prometheus surface provides compatible
queue-depth feedback; otherwise it runs all configured steps.

Prometheus report sections distinguish an uninterrupted collector, a recovered
collector, and one whose latest scrape failed. The report retains the most
recent failure message and attempt/success/failure timestamps. Legacy reports
include both database request-status backlog and peak/final throttler queue
gauges. If a cumulative counter resets, affected deltas and derived HTTP totals
or rates are conservative lower bounds and are labelled as such.

**Blessing new baselines after an accepted perf change:** run the suite to a
completed output, review it, then use
`baseline bless <suite-output> --baselines-dir baselines`. Add
`--accept-regressions` only for an intentional accepted change, then commit
`baselines/`. Blessing recomputes the diff against each baseline currently at
the destination and refuses symlinked artifact path components; it does not
trust suite-summary diff metadata to authorize replacement. Corrupt or
incompatible existing baselines are never overwritten.

## 6. Troubleshooting

| Symptom | Cause / fix |
| --- | --- |
| `failed the readiness check` | wrong `--relayer-url` (must be a v2 relayer) or relayer down; `curl <url>/health/readiness` |
| `pool has N unused payloads; scenario needs M` | top up: `pool add --flow input-proof --count <M-N>`, or just use `suite run` which does this |
| `Lane X holds 0 ETH` | fund the printed address; lanes are HD indices 0..lanes-1 of `MNEMONIC` |
| many `client_poll_deadline_exceeded` | requests outlived `requestTimeoutSec` — relayer saturated or timeout too tight for the load |
| `verify_failed` > 0 | **stop and investigate the relayer** — it returned wrong plaintexts or bad signatures; never bless such a run |
| ramp never stops | the target does not expose a compatible Prometheus queue-depth signal; the ramp runs all configured steps |
