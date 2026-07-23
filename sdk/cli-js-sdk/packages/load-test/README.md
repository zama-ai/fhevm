# @cli-fhevm-sdk/load-test

Protocol load-test tool for both the legacy relayer and relayer v2. It provides
model-aware load generation built on the workspace toolkit and `@fhevm/sdk`,
with pre-generated payload pools, implementation-agnostic client evidence,
capability-detected Prometheus metrics, and machine-readable reports.

```text
plan → explicit prepare (when needed) → execute → collect → report
```

**Operators:** see [`docs/OPERATIONS.md`](./docs/OPERATIONS.md). Start with
`suite plan <smoke|standard|capacity|endurance>`. A normal `suite run` never
creates missing pool material: use `suite prepare`, or add `--prepare` to
explicitly authorize the local CPU and funded on-chain work before execution.
`suite show <ref>` (built-in name or suite JSON path) prints the resolved
suite as JSON, including every resolved entry, mirroring `scenario show`.

## How each flow is driven

| Flow | Transport | Payload source | Verification |
| --- | --- | --- | --- |
| `input-proof` | raw HTTP (undici `Pool`) | pre-generated ZK-proof payloads (piscina workers) | `accepted` + returned handles equal locally computed handles |
| `public-decrypt` | `@fhevm/sdk` | unique k-combinations of pooled public FHETest handles | SDK signature verification and typed cleartexts vs known plaintexts |
| `user-decrypt` | `@fhevm/sdk` (`decryptValues`) | pooled private handles, reusable | SDK KMS verification/reconstruction; plaintext vs known value |
| `delegated-user-decrypt` | `@fhevm/sdk` (`decryptValues`) | pooled delegator-owned handles + ACL delegation | same as user-decrypt, with a delegated permit |

SDK-driven flows pass the run `AbortSignal`, timeout, and per-request
`x-request-id` through the SDK relayer options. Progress events provide job,
retry, submit, poll, timeout, and error metadata for the request record.
User-decrypt permit lifetime is derived from the request timeout plus a
two-minute clock-skew allowance, rounded up to the smallest whole-day duration
the v13 SDK accepts. SDK verification/reconstruction
failures after a successful relayer GET are recorded as `verify_failed`, not
ordinary transport failures.

Terminal client outcomes are intentionally disjoint:

| Outcome | Meaning |
| --- | --- |
| `succeeded` | terminal success and local verification passed |
| `submit_failed` | transport failure or non-202 response before a job existed |
| `failed` | the relayer reported a valid terminal failure |
| `verify_failed` | a valid success response failed cryptographic/value verification |
| `timed_out` | the per-request deadline elapsed before a terminal response |
| `protocol_error` | a success-status relayer envelope violated the runtime wire schema |
| `aborted` | the operator or suite lifecycle deliberately cancelled the leg |

`pollCount` is the number of GET attempts issued. It is a dimensionless count,
not a retry index or duration; reports expose count statistics without `Ms`
suffixes. Aborted legs are reported separately and excluded from workload error
rate denominators.

With `--relayer-b-url`, SDK-driven A/B measurements are independent target legs
over the same claimed workload item. User/delegated legs generate independent
transport keys and permits; they are not byte-identical raw dispatches and must
not be assumed to deduplicate. See
[`docs/design/dual-relayer-dispatch.md`](./docs/design/dual-relayer-dispatch.md).

**Dedup constraint.** Relayer dedup is permanent for successful requests, so
input-proof payloads and public-decrypt handle combinations are single-use
per environment. Pools are consumed through persisted cursors that advance
*before* use — an aborted run skips items, never reuses one. User-decrypt
requests are always unique because the dedup hash includes the per-leg
transport public key.

## Setup

Environment (read from the workspace-root `.env` or the environment):

- `MNEMONIC` — funding wallet; handle pools derive parallel HD lanes from it
  (`PRIVATE_KEY` works too, but limits pool creation to one lane).
- `DELEGATE_PRIVATE_KEY` — sign-only delegate account for
  `delegated-user-decrypt` pools when no mnemonic is set (with a mnemonic the
  delegate is derived at a reserved HD index).
- Host-chain RPC override: `SEPOLIA_RPC_URL` for `testnet` and `devnet`,
  `POLYGON_AMOY_RPC_URL` for `testnet-amoy` and `devnet-amoy`, and
  `MAINNET_RPC_URL` for
  `mainnet`. Public network defaults are used when the matching variable and
  `--rpc-url` are both absent.
- `ZAMA_FHEVM_API_KEY` — optional SDK and raw-relayer API-key authentication.
- `ZAMA_FHEVM_API_KEY_B` — optional distinct API key for the candidate relayer
  (`--relayer-b-url`); falls back to `ZAMA_FHEVM_API_KEY` when unset.
- `LOAD_TEST_RELAYER_URL`, `LOAD_TEST_NETWORK`, `LOAD_TEST_DATA_DIR`,
  `LOAD_TEST_RELAYER_CONFIG` — defaults for the matching CLI flags.

All commands run from this directory with `node --import tsx index.ts …`
(alias: `pnpm load-test …`).

## Pools

```bash
# Single-use input-proof payloads (CPU-bound; uses a worker pool)
node --import tsx index.ts pool add input-proof --count 2000 --value-types uint64

# Decrypt handle pools (on-chain FHETest transactions; the expensive step)
node --import tsx index.ts pool add public-decrypt --count 20 --lanes 4
node --import tsx index.ts pool add user-decrypt --count 4
node --import tsx index.ts pool add delegated-user-decrypt --count 4

node --import tsx index.ts pool status
```

`pool add` has one subcommand per flow, and each exposes only the flags that
flow consumes: `input-proof` takes `--threads`; the on-chain decrypt flows take
`--lanes` and `--encrypt-concurrency`; `delegated-user-decrypt` additionally
takes `--delegation-days`. All accept `--count` and `--value-types`. `pool
status` labels each delegated owner ACL as healthy, expired, or missing.

A pool of *n* public handles serves C(n, k) unique k-handle public-decrypt
requests (`handlesPerRequest` in the scenario), so handle creation amortizes
across many requests. Pools live under `<data-dir>/pools/<network>/`.
User/delegated pools retain reusable handles and account/delegation metadata;
they intentionally do not persist transport private keys or signed permits.
Those are generated independently inside each SDK target leg.

## Load models

- **Open** fixes arrivals: "Can the relayer sustain X new requests/sec?"
  Submissions are scheduled independently of response latency, so late jobs do
  not reduce offered load.
- **Closed** fixes active clients: "Can Y active clients run with acceptable
  latency, and what throughput results?" Each VU runs request → terminal or
  timeout → optional think time → next request.
- **Drain** fixes backlog size: submit N jobs near-instantly, then poll all to
  completion. This validates backlog correctness and configured drain rate.

`thinkTimeMs` is the closed-model pause between completed workflows. It is
not the poll interval; polling still honors `Retry-After` while one request is
in progress.

Closed scenarios without `maxIterations` are duration-bound. They are allowed
for reusable user/delegated handles. Closed `input-proof` and `public-decrypt`
scenarios must set `maxIterations`, because those pools are single-use.

## Running scenarios

```bash
node --import tsx index.ts scenario plan open-steady --rps 20 --duration 600 --out /tmp/open-steady-plan
node --import tsx index.ts scenario prepare open-steady --rps 20 --duration 600
node --import tsx index.ts scenario run open-steady --rps 20 --duration 600 --flow input-proof
node --import tsx index.ts scenario run open-ramp --rps 10 --duration 120
node --import tsx index.ts scenario run closed-steady --vus 20 --duration 600 --flow user-decrypt
node --import tsx index.ts scenario run closed-ramp --vus 5 --duration 120 --flow delegated-user-decrypt
node --import tsx index.ts scenario run drain --count 1000 --rps 200
node --import tsx index.ts scenario run scenarios/my-scenario.json --baseline baselines/testnet/open-steady.json
```

Built-ins (`scenario list` / `scenario show <name>`): `baseline`, `smoke`,
`open-steady`, `open-ramp`, `open-spike`, `open-soak`, `open-mixed`,
`closed-steady`, `closed-ramp`, `closed-soak`, `drain`. Steady-state defaults
are gentle (well under the protocol ceilings of ~20 rps input-proof and ~10 rps
combined decrypt), with one intentional exception: the saturation probes
`open-ramp` and `open-spike` deliberately reach or exceed the ceilings to
measure capacity and recovery. `open-ramp` steps above the ceiling and so
emits the advisory ceiling warning by design (expected, not a
misconfiguration). Any resolved scenario whose peak per-flow rate exceeds a
ceiling emits that advisory warning but still runs. Custom scenarios are
JSON documents validated against the schema in `src/scenario/schema.ts` (flow
mix and weights, load shape, timeouts, thresholds, saturation stop).

`scenario run` is the canonical entry point. The same overrides apply to
built-ins, custom JSON scenarios, and suite-entry `params`; they are resolved
before pool planning. Unsupported shape/option combinations fail instead of
being ignored:

- `--rps` replaces a constant or burst rate. For segmented shapes it scales
  the complete rate profile relative to its first positive configured rate.
- `--vus` replaces steady VUs. For staged closed shapes it scales every stage
  relative to the first stage, rounding to positive integers.
- `--duration` replaces a steady duration, or **each** segment/stage duration;
  it is not a total-duration override.
- `--count` is burst-only. `--think-time` and `--max-iterations` are
  closed-model-only.
- `--flow` is valid only for a single-flow scenario. It preserves that flow
  entry's weight and `handlesPerRequest`; multi-flow scenarios reject it.

Read-only commands (`scenario list`, `scenario plan`, `suite list`, `suite
plan`, `baseline list`, `pool status`, `report diff`) accept `--format
text|json`. In `json` mode they print exactly one JSON document to stdout and
suppress the info/success/warning log lines, so the output is safe to pipe into
`jq` or a script; errors still go to stderr.

Operator-authored JSON definitions are intentionally untracked by the local
ignore policies in `scenarios/` and `suites/`. JSON paths are resolved from the
load-test process working directory, not relative to a referring suite file.
The commands documented here run from this package directory; the workspace
`pnpm load-test` alias also launches the package there. Within a suite, use a
package-relative path such as `scenarios/my-scenario.json` or an absolute path
for a definition stored elsewhere.

## Collectors (all optional; absence degrades the report, never the run)

- **Prometheus** — scrapes the configured relayer `/metrics` every 5 s and is
  agnostic to the server implementation; the report
  records capability and scrape-health information, then reports only signals
  actually exported by that relayer. Scrape counts, current attempt state,
  timestamps, and the most recent failure are retained so a recovered scrape
  is distinguishable from both uninterrupted collection and current outage.
  Capabilities are detected from exported metric families. Legacy
  request-status gauges provide queue depth and stage durations, while legacy
  throttler gauges report peak and final in-process queue depth. Relayer v2
  metrics retain their native semantics: input-proof inserts/duration,
  transaction outcomes/duration/errors, recovery runs/items/duration,
  wallet-lease state/transitions, DB errors, and HTTP endpoint/status counts.
  Missing families stay unavailable rather than being inferred across
  implementations. Counter or histogram resets are marked as
  conservative lower bounds, including aggregate HTTP totals and rates.
  Neither current exporter exposes sqlx pool
  connection/acquire-wait families.
- **Config snapshot** (`--relayer-config <path>`) — embeds the relayer config
  file in the report after recursively redacting secret-bearing fields and
  credential material; throughput is config-capped, so reports without the
  non-secret operational settings are harder to interpret.

## Outputs

Each run writes to `<data-dir>/runs/<timestamp>-<scenario>/`:

- `report.json` — canonical machine-readable report.
- `report.md` — human rendering (also produced by `report render <dir>`).
- `requests.jsonl` — one record per request (sent/echoed request ids, jobId,
  submit latency, poll count, terminal latency, outcome, error label).
- `requests-a.jsonl` and, for dual-target runs, `requests-b.jsonl` — the same
  per-request records split by target, for isolated per-leg analysis.
- `metrics-a.jsonl` and, for dual-target runs, `metrics-b.jsonl` — retained
  Prometheus collector time series.
- `injector-runtime.jsonl` — the load-test process's own runtime samples
  (event-loop lag/utilization, CPU, RSS, GC) plus scheduler dispatch-lag,
  backpressure, drop, and abandon accounting, independent of the relayer.

Planning and preparation evidence uses the same stable names:

- `pool-plan.json` is the authoritative, versioned machine-readable live
  observation; `pool-plan.md` is its human-readable mirror.
- Explicit preparation also writes authoritative `preparation.json` plus
  `preparation.md`. Standalone preparation defaults to
  `<data-dir>/preparations/<timestamp>-scenario-<name>/`; when a `scenario run
  --prepare` actually needs preparation, it places that evidence in the run
  directory.
- Scenario and suite plan commands write evidence only when `--out <dir>` is
  given; without it they remain read-only and print the plan without creating
  directories.
- `suite prepare` defaults under `<data-dir>/preparations/`; `suite run` writes
  its plan, optional preparation evidence, scenario reports, and suite summary
  under one run root.
- `scenario run` likewise allocates one stable run root and records its initial
  live plan there before deciding whether to execute. A deficient run without
  `--prepare` exits 2 and leaves that plan evidence for review; a ready run
  proceeds without redundant preparation even if `--prepare` was supplied.

The shared operator summary keeps units explicit: finite workloads show a
request budget, duration-bound closed decrypt workloads show a reusable-handle
target instead of inventing a request count, and deficits are pool creation
units. Planned actions are labeled `[LOCAL CPU]` or `[ON-CHAIN]`; handle
actions state the funded setter-transaction count, while delegated ACL costs
are identified separately. Dedicated `scenario prepare` and `suite prepare`
still write no-op preparation evidence when pools are ready, but the readiness
gate runs only immediately before a fresh plan's first actual action.

Each `pool-plan` and `preparation` file is flushed and atomically renamed
independently. Their JSON is authoritative; a crash between JSON and Markdown
publication can leave the mirror from a different attempt. This durability
claim does not extend to suite summaries, suite-preparation wrappers, or
streaming JSONL. The scenario digest binds the fully resolved, ordered scenario
definitions and suite pause, while the recorded environment identity binds the
target network, chain, contract, relayer origins, and API paths. Compare both
fields—neither alone identifies the complete plan. Stored plans are evidence
only: preparation always computes a fresh live plan and then re-inspects before
declaring readiness.

Artifacts omit configured RPC URLs and data directories, strip relayer URL
userinfo/query data, and bound and sanitize recognized credential patterns in
error text. Sanitization is defensive rather than a guarantee against every
possible secret format; inspect artifacts before sharing them.

The report records the run model (`open`, `closed`, or `drain`) and each flow's
driver (`raw-http` for input-proof, `sdk` for decrypt flows). It leads with an
**Executive Summary**: target identity, model/status/window, planned versus
submitted workflows and achieved rate, threshold/correctness verdicts,
performance evidence, injector assessment, and telemetry coverage. When the
evidence supports actionable findings, **Diagnosis** follows with
severity-tagged flags and concrete recommendations. Below it:
per-flow client histograms; **client results by load stage** for ramp/sweep
scenarios; capability-gated relayer metric deltas and relayer-side **HTTP**
status distribution;
**backlog-by-stage** (where the queue sat, and whether it drained or grew);
**process/host** RSS·CPU·FD trends with drift-per-hour for soaks; and the
relayer config snapshot.

This package intentionally does not connect to PostgreSQL. Server-row
correlation, poll-free server e2e comparison, and database-timestamp pipeline
stages are future adapter boundaries in the report model, not data collected
by the current command surface.

Command exit codes are stable for automation:

- `0`: operation completed and acceptance checks passed (or an unchecked plan
  merely reported work).
- `1`: operational failure, threshold breach, baseline regression, or
  preparation that did not produce readiness.
- `2`: `plan --require-ready` found required work, or `run` recorded the plan and was
  safely blocked before pool preparation and workload execution because
  preparation was not explicitly authorized.
- `130`: interrupted by the operator.

```bash
node --import tsx index.ts report diff baselines/testnet/open-steady.json runs/<dir>/report.json
# --max-latency-increase <fraction>: allowed relative p95/p99 latency increase (default 0.20)
# --max-error-rate-increase <fraction>: allowed absolute error-rate increase, range [0, 1] (default 0.01)
```

Baseline reads are strictly schema-validated. After reviewing a completed
suite, publish all of its threshold-passing reports in one explicit operation:

```bash
node --import tsx index.ts baseline bless runs/<suite-dir> --baselines-dir baselines
# Add --accept-regressions only when intentionally accepting measured regressions.

node --import tsx index.ts baseline list --baselines-dir baselines
# Prints <network>/<label> and its last-updated time for every stored baseline.
```

Blessing recomputes each candidate against the baseline currently at the
destination; suite-summary diff metadata is informational and cannot authorize
a regression. Artifact paths containing symlink components below the selected
suite or baseline root are refused.

Commit the resulting `baselines/<network>/<label>.json` files. This repository
does not currently schedule load tests in GitHub Actions; wire the suite command
into environment-specific CI only where credentials, funding, and a dedicated
relayer are available.

## Development

```bash
pnpm typecheck
pnpm test        # vitest unit tests (scheduler, pools, parser, verification, reports)
```

The input-proof worker uses the public `generateZkProof` action and
keeps one initialized SDK context per worker thread.
