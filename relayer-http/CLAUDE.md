# relayer-http — development guide

This file is living: add a rule or change a description here whenever a feature lands that affects them, in the same
change. It describes a minimal first version.

## What this repo is

The Zama relayer over HTTP. It replaces the gateway-chain decryption path: request handlers receive the SDK's
decryption requests and the `kms_aggregator` module fans them out to the KMS connector endpoints and aggregates the
t-of-n answers.

**Minimal first version**: the handlers and the fan-out/aggregation only.
- No ACL check and no ciphertext-commit (readiness) check in the relayer: the connector's worker performs them and their
  failures come back as connector error codes. Both can come back later as optional modules in front of the aggregator.
- No database, no queue, no cache, no response storage.

## Layout

```
relayer-http/
├── Cargo.toml, Cargo.lock, rust-toolchain.toml   # workspace, exact dependency pins (=x.y.z), toolchain 1.97.1
├── Dockerfile                                     # container image, built from the repository root (see Container image)
├── .gitignore                                     # target/, config/*.local.yaml (local variants of the configuration)
├── config/config.yaml                             # example configuration: name, log, http, kms_aggregator
├── monitoring/                                    # dashboard, alert rules and their tests, runbook, measurement design (design.md), generated references (OBSERVABILITY.md)
├── dev/                                           # local development tooling: Makefile, host run, flow scripts; start with its README.md
└── crates/relayer-http/src/
    ├── main.rs                # load config → init logging → App → serve until SIGINT/SIGTERM → drain → exit
    ├── lib.rs                 # App: the one shared state (both aggregators, http config, metrics registry, shutdown token: healthz and the servers, never an aggregation)
    ├── config.rs              # RelayerConfig, LogConfig, HttpConfig, MetricsConfig: YAML + APP_<SECTION>__<FIELD> overrides
    ├── logging.rs             # tracing subscriber (json | pretty | compact, RUST_LOG), the `log` bridge, the panic hook; test only: the JSON capture helpers
    ├── metrics.rs             # the metric registry, label values, the /metrics listener (its own port)
    ├── reference.rs, monitoring.rs   # test only: generated references and series budget; dashboard and alerts checked against the code
    ├── endpoint/              # the HTTP layer; start with its docs.md
    │   ├── mod.rs             # router(), serve() and the middleware that logs the summary line and counts the request
    │   ├── summary.rs         # Summary: one request's state, the summary line and its field list
    │   ├── error.rs           # ApiError: the one error body and its mapping
    │   ├── validate.rs        # rules shared by the routes (handles, extraData)
    │   ├── ops.rs             # GET /liveness, GET /healthz
    │   └── flows/{mod,user_decrypt,public_decrypt}.rs   # one file per route: wire types, validation, handler (fills the Summary)
    └── kms_aggregator/        # fan-out to the KMS connectors and t-of-n aggregation; start with its docs.md
        ├── mod.rs, config.rs, client.rs, call.rs, aggregator.rs, signer.rs   # signer: FhevmSig headers (RFC-038)
        ├── flows/{mod,user_decrypt,public_decrypt}.rs   # one file per flow: checks, counting, output
        ├── mock.rs, scenarios.rs                       # test only: scripted connector, YAML scenario runner
        └── scenarios/*.yaml                            # behaviour scenarios, run by the test suite
```

Two modules, each with its `docs.md` next to the code: `endpoint/docs.md` (routes, payloads, responses, every
error code and its mapping) and `kms_aggregator/docs.md` (how an aggregation runs, one node call, checks,
configuration → behaviour). The implementation plans live outside the repository (`relayer_specs_plans/`).

## Configuration

One YAML file (`config/config.yaml` is the example), five sections: `name`, `log` (optional, defaults to JSON
lines), `http` (bind address, body limit, body read timeout, supported chain ids, optional request limits defaulting to the connector's), `metrics` (optional: `endpoint`, default `0.0.0.0:9898`, the Prometheus listener on its own port) and `kms_aggregator` (deadline,
two retry budgets: `call.retries.waits`, the explicit list of waits for transient errors, and `call.rate_limit` for
rate limits, thresholds, optional checks, the KMS endpoints). Any field can be overridden with `APP_<SECTION>__<FIELD>` (durations carry a
unit, e.g. `APP_KMS_AGGREGATOR__CALL__TIMEOUT=7s`; a list is one comma-separated value). Every connector call carries an `FhevmSig` header (RFC-038)
signed for that endpoint's `audience`; the signing key is never in the file: `kms_aggregator.auth.signer` names the env
var holding it, or an AWS KMS key, and startup fails unless that key has the address `kms_aggregator.auth.address` and signs a test message. Local variants go to `config/*.local.yaml`, ignored by git.

The configuration does not live in this directory only. A change to it (key added, renamed, removed, default
changed) lands in the same change in every carrier: `config/config.yaml`, the local stack's template
`test-suite/fhevm/templates/config/relayer-http.yaml` (rendered by `test-suite/fhevm/src/generate/config.ts`, which
passes the untouched sections through verbatim, so a stale template fails the in-stack relayer at startup) and its
test, the preview-env values and the chart values once relayer-http is deployed there. Before merging and after
every rebase, grep the repository for the old keys; an in-flight integration branch (preview-env, e2e) is rebased
onto the new keys, never left on the old ones.

## Commands

From `relayer-http/`:

```sh
cargo fmt -- --check      # not --all: cargo-fmt would follow the kms-connector-api path dependency into generated bindings
cargo clippy --workspace --all-targets -- -W clippy::perf -W clippy::suspicious -W clippy::style -D warnings
cargo nextest run --workspace
cargo nextest run -p relayer-http yaml_scenarios   # the aggregator scenarios only
make -C dev test-aws      # the AWS KMS signer as deployed (IRSA credentials), against Floci (an AWS emulator); needs Docker
cargo run -p relayer-http -- config/config.yaml    # needs the signing key env var named in auth.signer (RELAYER_SIGNER_KEY)
```

`make -C dev references` regenerates `monitoring/{summary_log,metrics,labels}.md` after a change to the summary fields, the
metrics or the labels (a test fails while they differ); `make -C dev check-alerts` runs `promtool` on the alert rules
and their unit tests (Docker).

Every change must pass the first three; a change to the AWS signer or the AWS dependencies also `make -C dev test-aws`.
Tests that need an external service are `#[ignore = "<why>"]` and grouped in one module per service, run by name with
`--run-ignored only -E 'test(/<module>/)'` once the service is up (never an env var that silently skips). Check exit codes strictly (a grep on the output hides a failing build).

Against the local fhevm stack: `./fhevm-cli up` runs relayer-http in the stack (`fhevm-relayer-http`, :3002) from the
published image; `--override relayer-http` builds it from the working tree instead (`upgrade relayer-http` rebuilds and
restarts it after an edit); `make -C dev run` runs it on the host instead (:4002, the stack's config); `make -C dev public-decrypt` / `user-decrypt`
(one flow each, from the e2e container); `make -C dev check` (the three commands above). See `dev/README.md`.

## Container image

`Dockerfile` follows the relayer's and the kms-connector's model: the golden `rust-glibc` builder pinned by
`RUST_IMAGE_VERSION` (single source of truth: `rust-toolchain.toml`, read by the CI template), `cargo build --locked
--release`, a `glibc-dynamic` runtime running as the `fhevm` user, no configuration baked in. The build context is the
**repository root**: the crate depends on `kms-connector/crates/api` by path and that workspace references its
siblings, which the Dockerfile copies at the same relative places.

```sh
# from the repository root
docker build -f relayer-http/Dockerfile --build-arg RUST_IMAGE_VERSION=1.97.1 -t relayer-http .
docker run --rm -p 8080:8080 -v "$PWD/relayer-http/config/config.yaml:/app/config/config.yaml:ro" \
  -e RELAYER_SIGNER_KEY=... relayer-http        # the config is mounted at /app/config/config.yaml
```

CI publishes the image as `ghcr.io/zama-ai/fhevm/relayer-http` through `.github/workflows/relayer-http-docker-build.yml`
(pushes to `main`/`release/*`, releases, manual runs, or a call from the e2e orchestrator). The two other workflows,
`relayer-http-tests.yml` and `relayer-http-dependency-analysis.yml`, run on pull requests: the three commands above and
`make -C dev test-aws`, then `cargo update -w --locked`, `cargo-deny check license` against `deny.toml` (every allowed license must be used)
and `cargo-audit audit` with `.cargo/audit.toml`. All three are gated on changes under `relayer-http/**`,
`kms-connector/crates/api/**`, `kms-connector/Cargo.toml` and `shared/fhevm-sig-auth/**`.

## Rules

**No gateway interaction.** The relayer never talks to the gateway chain and carries no gateway-specific
configuration, optional checks included. Every on-chain read goes to a host chain through host contracts, Ethereum
being the reference.

**No unexpected panic.** A panicking handler still answers `500 internal` (`CatchPanicLayer`). Both crate roots deny `clippy::unwrap_used`, `expect_used`, `indexing_slicing` and `panic`
(allowed in tests). Every failure is a typed error returned to the caller. Configuration validation bounds every value
the runtime arithmetic relies on (`1s <= call.timeout <= 60s`, every retry wait starts before `call.timeout`, semaphore size
capped) so that no arithmetic can overflow later. A panic inside a spawned task is counted as a failure, never propagated.

**Fail fast.** Never wait for something that cannot change the outcome: when `counted + pending < threshold` the
remaining calls are cancelled at once. One deadline per request, no queue, no ETA; the client retries.

**HPA compatible.** No per-request state outside the request's own future: no shared map, no registry, no sticky
session. Every replica is identical; the call semaphore is per pod. If per-request state is ever needed, it goes behind
a trait so that Redis (or another store) can back it later.

**Errors.** Modules return typed errors (`thiserror`), with what the caller needs to act (counts, dominant connector
error). Errors are reported once, in the summary line of the request that the boundary answering the client emits. No `error!`
inside a module for an expected failure.

**Logging and metrics.** `OBSERVABILITY.md` is the policy: read it before adding a log line, a metric, an alert or a
dashboard. `tracing` only; the subscriber is installed by `main`, never by a module; `log:` is optional and defaults to
JSON lines (`log.format`: `json` | `pretty` | `compact`, plus the relayer's `show_*` switches); the level filter is
`RUST_LOG`, default `warn,relayer_http=debug` (never a global `debug`: library lines may carry URLs); `log`-crate
libraries reach the same subscriber; the panic hook logs a panic as one JSON ERROR line, location only. Log tests use `logging::capture` and assert on JSON fields, not
`tracing-test`.

**Configuration.** Nested structs use `deny_unknown_fields`; `validate()` messages name the field with its dotted path
(`kms_aggregator.call.timeout must …`); secrets appear only as env var names; durations carry a unit.

**Minimal code.** Few types, one function per loop, a one-line doc comment per item, no framework where a function does.
Production-ready means no panic, typed errors, logs, cancellation and tests, not features.

**Compatibility.** The connector interface comes from the `kms-connector-api` crate (path dependency, never its
`endpoint` feature): DTOs, routes, error codes and their `retryable` flag are used as they are, never redefined.

**Tests.** A unit test per function, next to it. Paused tokio time (`start_paused`) for anything timed, so durations
are exact and tests are instant. Behaviour is described in YAML scenarios run by the test suite; adding a case is adding
a file.

**Documentation.** Every module has a `docs.md` produced with the module and updated in the same change as any
behaviour change, plus a one-line doc comment on every item. The error code table in `endpoint/docs.md` (every code,
its status, when it happens, how aggregation errors map to it) is updated in the same change as any change to that
mapping. This `CLAUDE.md` is updated the same way: new rules and
changed descriptions land with the feature that motivates them.
