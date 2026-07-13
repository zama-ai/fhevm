# Load-Test Operator Guide

The short version: inspect first, then explicitly authorize preparation only
when the plan requires it:

```bash
cd sdk/cli-js-sdk/packages/load-test
node --import tsx index.ts --relayer-url <relayer> suite plan standard --check
node --import tsx index.ts --relayer-url <relayer> suite run standard --prepare
```

The first command is read-only and exits 2 when work is required. After
reviewing the disclosed local CPU and funded on-chain actions, `--prepare`
explicitly authorizes them and then runs the scenarios sequentially. If pools
are already ready, omit `--prepare`. Everything below is detail.

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

## 2. Suite lifecycle

```bash
node --import tsx index.ts suite list            # what exists
node --import tsx index.ts suite show standard   # resolved suite JSON, including resolved entries
node --import tsx index.ts suite plan standard   # read-only; add --check for CI exit 2
node --import tsx index.ts suite prepare standard # explicit preparation; does not run
node --import tsx index.ts suite run standard    # runs only when pools are ready
node --import tsx index.ts suite run standard --prepare # explicitly prepare + run
```

Built-in suites and when to run them:

| Suite | Contents | Duration | Question it answers |
| --- | --- | --- | --- |
| `smoke` | burst of ~5 requests per flow (ip/pd/ud) | ~1 min | is this deployment functional end-to-end? |
| `standard` | baseline, open-steady-5, open-mixed (6 rps), drain-200 | ~15 min | did anything regress? (nightly set) |
| `capacity` | open-ramp (stops at saturation), open-spike | ≤ ~40 min | what is max sustainable throughput near the ~20 rps ceiling, and does it recover? |
| `endurance` | open-soak 3 req/s ×60 min | ~70 min | leaks/drift in the relayer process? |

The commands have deliberately separate authority:

- `suite plan` never creates payloads, handles, ACL grants, or an output
  directory unless `--out <dir>` is supplied. `--check` exits 2 if preparation
  is required.
- `suite prepare` is the explicit prepare-only operation. Input-proof work is
  local and CPU-bound; handle creation and delegated ACL refreshes send funded
  host-chain transactions. Readiness is checked before mutation unless
  `--skip-readiness` is deliberately supplied.
- `suite run` always records a live plan, but without `--prepare` it blocks
  with exit 2 instead of preparing pools when work is required. `--prepare`
  authorizes the disclosed local/on-chain work before execution.
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
`params` overrides apply identically to built-in names and JSON scenarios, and
are resolved before planning. Shape-aware rules reject irrelevant knobs:
`rps` scales a segmented profile from its first positive rate, `vus` scales
staged VUs from the first stage, and `durationSec` replaces every segment or
stage duration rather than the total. `flow` only replaces a single-flow
scenario and preserves its weight and `handlesPerRequest`.

Suites run scenarios **sequentially** with a pause in between — never run two
load tests against the same relayer at once; they corrupt each other's
measurements.

## 3. Outputs

```text
<out>/
├── pool-plan.json        # authoritative resolved-scenario + live-pool plan
├── pool-plan.md          # human mirror of the plan
├── preparation.json      # present after explicit preparation
├── preparation.md        # human mirror of preparation evidence
├── suite-summary.md       # one row per scenario: errors, verify failures, thresholds, diff
├── suite-summary.json
└── <label>/               # one directory per scenario run
    ├── report.md          # human report
    ├── report.json        # canonical; diffable
    ├── requests.jsonl     # per-request records (requestId ↔ jobId correlation)
    ├── metrics-a.jsonl    # target A /metrics snapshots
    └── metrics-b.jsonl    # optional target B /metrics snapshots
```

`suite prepare` additionally writes `suite-preparation.json/.md`. Standalone
suite preparation defaults to `<data-dir>/preparations/<timestamp>-suite-<name>/`;
standalone scenario preparation defaults to
`<data-dir>/preparations/<timestamp>-scenario-<name>/`. When `run --prepare`
actually performs preparation, plan, preparation, and run evidence share the
run root.

`scenario plan` has the same optional `--out <dir>` evidence behavior as suite
planning. `scenario run` always creates one stable run root and writes its live
plan there. If preparation is required but not authorized, it exits 2 before
readiness, pool preparation, or workload execution and leaves the plan for review.
When the live plan is already ready, `--prepare` is a no-op authorization: the
run does not repeat readiness or preparation work.

Plan summaries distinguish finite request budgets from duration-bound closed
decrypt workloads, for which they show the reusable-handle target rather than
a fictional request count. Inventory, availability, and deficit creation units
are separate. Every planned action is labelled `[LOCAL CPU]` or `[ON-CHAIN]`;
handle actions disclose funded setter transactions and delegated ACL work is
listed separately. Dedicated `scenario prepare` and `suite prepare` always
write preparation evidence, including ready no-op results, while the readiness
gate runs only immediately before a fresh plan's first actual action.

For `pool-plan` and `preparation`, JSON is authoritative and Markdown is the
human mirror. Each of those files is flushed and atomically renamed
independently, so a crash between JSON and Markdown publication can leave a
mirror from a different attempt. This durability claim does not cover suite
summaries, suite-preparation wrappers, or streaming JSONL. A plan's scenario
digest binds the fully resolved ordered scenarios and suite pause. Its
environment identity separately binds network, chain, contract, relayer
origins, and API paths; compare both. A stored plan is never executed:
preparation always plans live again and re-inspects after its actions.

Evidence omits RPC URLs/data directories, removes URL userinfo and queries,
and bounds and sanitizes recognized credential patterns in errors. This is
defense in depth, not proof that arbitrary provider text contains no secret;
inspect artifacts before sharing.

Exit codes are suitable for automation:

| Code | Meaning |
| ---: | --- |
| `0` | completed and accepted; unchecked plans may still report work |
| `1` | operation failed, preparation remained unready, or thresholds/baseline diff failed |
| `2` | checked plan needs work, or a run recorded evidence then stopped before pool preparation/workload execution |
| `130` | interrupted by the operator |

## 4. Lower-level commands

```bash
# Pools
node --import tsx index.ts pool add --flow input-proof --count 2000   # off-chain, CPU-bound
node --import tsx index.ts pool add --flow public-decrypt --count 40
node --import tsx index.ts pool add --flow user-decrypt --count 4
node --import tsx index.ts pool add --flow delegated-user-decrypt --count 4
node --import tsx index.ts pool inspect

# Single scenario
node --import tsx index.ts scenario plan open-steady --rps 20 --duration 600 --check --out /tmp/open-steady-plan
node --import tsx index.ts scenario prepare open-steady --rps 20 --duration 600
node --import tsx index.ts scenario run open-steady --rps 20 --duration 600 --baseline baselines/testnet/open-steady-20.json
node --import tsx index.ts run open-steady --rps 20 --duration 600 # thin alias for scenario run
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

Override semantics are shared by `scenario show|plan|prepare|run`, the root
`run` alias, custom JSON, and suite `params`. `--rps` scales all segmented
rates relative to the first positive rate; `--vus` scales all stages relative
to the first stage; `--duration` is per segment/stage; and `--flow` rejects
multi-flow scenarios. Model-mismatched options fail instead of being ignored.

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

**Suggested nightly regression:** prepare pools ahead of the window, then run
`suite run standard` without pool-preparation authority. Investigate any ❌ row
in `suite-summary.md`, starting with the per-run `report.md` **Executive Summary**
for status, correctness, performance evidence, injector health, and telemetry
coverage. Continue to **Diagnosis** when present for actionable findings, then
use the stage `Share` column and **Backlog by Stage** table to localize where
time or queue went.

**Before/after a relayer change:**

```bash
node --import tsx index.ts suite run standard --prepare --out runs/before # old build
# deploy the new build
node --import tsx index.ts suite run standard --prepare --out runs/after
node --import tsx index.ts report diff runs/before/open-steady/report.json runs/after/open-steady/report.json
```

**Capacity check on a new environment:** plan and explicitly prepare first,
then use `suite run capacity`. A ramp only stops early when the target's
Prometheus surface provides compatible queue-depth feedback; otherwise it runs
all configured steps.

Prometheus collection is implementation-agnostic and capability-detected.
Report sections distinguish an uninterrupted collector, a recovered collector,
and one whose latest scrape failed. Legacy metric families retain legacy queue
and stage semantics; v2 families retain their native transaction, recovery,
wallet-lease, database-error, and HTTP semantics. Missing families remain
unavailable rather than being guessed across implementations. If a cumulative
counter resets, affected deltas and derived HTTP totals or rates are
conservative lower bounds and are labelled as such.

**Blessing new baselines after an accepted perf change:** run the suite to a
completed output, review it, then use
`baseline bless <suite-output> --baselines-dir baselines`. Add
`--accept-regressions` only for an intentional accepted change, then commit
`baselines/`. Blessing recomputes the diff against each baseline currently at
the destination and refuses symlinked artifact path components; it does not
trust suite-summary diff metadata to authorize replacement. Corrupt or
incompatible existing baselines are never overwritten.

## Migration: gentler defaults

Built-in defaults were lowered to stay well under the protocol ceilings
(~20 rps input-proof; ~10 rps combined public + user + delegated decrypt).
Two default baseline keys changed:

- `open-steady` now resolves to `open-steady-5` (was `open-steady-10`).
- `closed-steady` now resolves to `closed-steady-5vu` (was `closed-steady-10vu`).

Other defaults also softened: `baseline` is 3 rps ×60s (ip/pd/ud), `open-mixed`
is 6 rps over equal ip/ud/pd thirds ×300s, `open-soak` is 3 rps, `drain` is
`count 200 @ 20 rps`, and the `standard` suite runs in ~15 min. `smoke` is now
a burst scenario/suite (~5 requests per flow) rather than an open-steady run.

To reproduce the old baseline keys, re-baseline against the new keys, or pass
`--rps 10` / `--vus 10` (e.g. `run open-steady --rps 10` → `open-steady-10`).
`delegated-user-decrypt` is excluded from every default flow mix (it behaves
like user-decrypt) and stays reachable via `--flow delegated-user-decrypt`.

## 6. Troubleshooting

| Symptom | Cause / fix |
| --- | --- |
| `failed the readiness check` | target does not expose `GET /health/readiness` or is down; verify its health surface, then use `--skip-readiness` only when another check proves it ready |
| `Pool preparation is required` | inspect `pool-plan.md`, then run `suite prepare`, `scenario prepare`, or rerun with explicit `--prepare` |
| `Lane X holds 0 ETH` | fund the printed address; lanes are HD indices 0..lanes-1 of `MNEMONIC` |
| many `client_poll_deadline_exceeded` | requests outlived `requestTimeoutSec` — relayer saturated or timeout too tight for the load |
| `verify_failed` > 0 | **stop and investigate the relayer** — it returned wrong plaintexts or bad signatures; never bless such a run |
| ramp never stops | the target does not expose a compatible Prometheus queue-depth signal; the ramp runs all configured steps |
