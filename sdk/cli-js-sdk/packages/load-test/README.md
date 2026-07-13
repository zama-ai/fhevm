# @cli-fhevm-sdk/load-test

Fully automated load-test tool for **Relayer v2**. It provides model-aware
load generation built on the workspace toolkit and `@fhevm/sdk`, with
pre-generated payload pools, pluggable collectors, and machine-readable
reports.

```text
run = prepare → execute → collect → report
```

**Operators:** see [`docs/OPERATIONS.md`](./docs/OPERATIONS.md) — the
one-command workflow is `suite run <smoke|standard|capacity|endurance>`,
which plans pool requirements, prepares deficits, runs the grouped
scenarios, and writes a suite summary.

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
accepted by both alpha.8 protocol paths. SDK verification/reconstruction
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

With `--relayer-b`, SDK-driven A/B measurements are independent target legs
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
- `SEPOLIA_RPC_URL` (per-network RPC override), `ZAMA_FHEVM_API_KEY` (optional).
- `LOAD_TEST_RELAYER_URL`, `LOAD_TEST_NETWORK`, `LOAD_TEST_DATA_DIR`,
  `LOAD_TEST_RELAYER_CONFIG` — defaults for the matching CLI flags.

All commands run from this directory with `node --import tsx index.ts …`
(alias: `pnpm load-test …`).

## Pools

```bash
# Single-use input-proof payloads (CPU-bound; uses a worker pool)
node --import tsx index.ts pool add --flow input-proof --count 2000 --types uint64

# Decrypt handle pools (on-chain FHETest transactions; the expensive step)
node --import tsx index.ts pool add --flow public-decrypt --count 20 --lanes 4
node --import tsx index.ts pool add --flow user-decrypt --count 4
node --import tsx index.ts pool add --flow delegated-user-decrypt --count 4

node --import tsx index.ts pool inspect
```

`pool add` rejects options that do not apply to the selected flow: `--threads`
is input-proof-only, lane/encryption options are handle-only, and
`--delegation-days` is delegated-user-decrypt-only. `pool inspect` labels each
delegated owner ACL as healthy, expired, or missing.

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
node --import tsx index.ts run baseline
node --import tsx index.ts run open-steady --rps 20 --duration 600 --flow input-proof
node --import tsx index.ts run open-ramp --rps 10 --duration 120
node --import tsx index.ts run closed-steady --vus 20 --duration 600 --flow user-decrypt
node --import tsx index.ts run closed-ramp --vus 5 --duration 120 --flow delegated-user-decrypt
node --import tsx index.ts run drain --count 1000 --rps 200
node --import tsx index.ts run ./my-scenario.json --baseline baselines/testnet/open-steady.json
```

Built-ins (`scenario list` / `scenario show <name>`): `baseline`,
`open-steady`, `open-ramp`, `open-spike`, `open-soak`, `open-mixed`,
`closed-steady`, `closed-ramp`, `closed-soak`, `drain`. Custom scenarios are
JSON documents validated against the schema in `src/scenario/schema.ts` (flow
mix and weights, load shape, timeouts, thresholds, saturation stop).

## Collectors (all optional; absence degrades the report, never the run)

- **Prometheus** — scrapes the relayer `/metrics` every 5 s; the report
  records capability and scrape-health information, then reports only signals
  actually exported by that relayer. Scrape counts, current attempt state,
  timestamps, and the most recent failure are retained so a recovered scrape
  is distinguishable from both uninterrupted collection and current outage.
  Legacy request-status gauges provide queue depth and stage durations, while
  legacy throttler gauges report peak and final in-process queue depth.
  Relayer v2 metrics remain explicitly v2
  shaped: input-proof inserts/duration, transaction outcomes/duration/errors,
  recovery runs/items/duration, wallet-lease state/transitions, DB errors, and
  HTTP endpoint/status counts. Counter or histogram resets are marked as
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
- `metrics-a.jsonl` and, for dual-target runs, `metrics-b.jsonl` — retained
  Prometheus collector time series.

The report records the run model (`open`, `closed`, or `drain`) and each flow's
driver (`raw-http` for input-proof, `sdk` for decrypt flows). It leads with a
**Diagnosis** section synthesized from all
collected signals — a one-line verdict, the dominant pipeline stage (as a %
of e2e when the exported metrics provide that signal), which limiters
saturated, severity-tagged flags, and concrete recommendations. Below it:
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

Exit code is non-zero on threshold breach or baseline regression:

```bash
node --import tsx index.ts report diff baselines/testnet/open-steady.json runs/<dir>/report.json
```

Baseline reads are strictly schema-validated. After reviewing a completed
suite, publish all of its threshold-passing reports in one explicit operation:

```bash
node --import tsx index.ts baseline bless runs/<suite-dir> --baselines-dir baselines
# Add --accept-regressions only when intentionally accepting measured regressions.
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

The input-proof worker uses the public alpha.8 `generateZkProof` action and
keeps one initialized SDK context per worker thread.
