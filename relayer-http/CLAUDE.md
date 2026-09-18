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
└── crates/relayer-http/src/
    ├── main.rs                # load settings → init logging → App → serve until SIGINT/SIGTERM → drain → exit
    ├── lib.rs                 # App: the one shared state (both aggregators, http settings, shutdown token)
    ├── settings.rs            # Settings, LogConfig, HttpConfig: YAML + APP_<SECTION>__<FIELD> overrides
    ├── logging.rs             # tracing subscriber: json | pretty | compact, RUST_LOG filter
    ├── endpoint/              # the HTTP layer; start with its docs.md
    │   ├── mod.rs             # router() and serve()
    │   ├── error.rs           # ApiError: the one error body and its mapping
    │   ├── validate.rs        # rules shared by the routes (handles, extraData)
    │   ├── ops.rs             # GET /liveness, GET /healthz
    │   └── flows/{mod,user_decrypt,public_decrypt}.rs   # one file per route: wire types, validation, handler
    └── kms_aggregator/        # fan-out to the KMS connectors and t-of-n aggregation; start with its docs.md
        ├── mod.rs, config.rs, client.rs, call.rs, aggregator.rs
        ├── flows/{mod,user_decrypt,public_decrypt}.rs   # one file per flow: checks, counting, output
        ├── mock.rs, scenarios.rs                       # test only: scripted connector, YAML scenario runner
        └── scenarios/*.yaml                            # behaviour scenarios, run by cargo test
```

Two modules, each with its `docs.md` next to the code: `endpoint/docs.md` (routes, payloads, responses, every
error code and its mapping) and `kms_aggregator/docs.md` (how an aggregation runs, one node call, checks,
configuration → behaviour). The implementation plans live outside the repository (`relayer_specs_plans/`).

## Configuration

One YAML file (`config/config.yaml` is the example), four sections: `name`, `log` (optional, defaults to JSON
lines), `http` (bind address, body limit, supported chain ids) and `kms_aggregator` (deadline, retries, thresholds,
optional checks, the KMS endpoints). Any field can be overridden with `APP_<SECTION>__<FIELD>` (durations carry a
unit, e.g. `APP_KMS_AGGREGATOR__CALL__TIMEOUT=7s`). API keys are never in the file: each endpoint names the env var
holding its key. Local variants go to `config/*.local.yaml`, ignored by git.

## Commands

From `relayer-http/`:

```sh
cargo fmt -- --check      # not --all: cargo-fmt would follow the kms-connector-api path dependency into generated bindings
cargo clippy --workspace --all-targets -- -W clippy::perf -W clippy::suspicious -W clippy::style -D warnings
cargo test --workspace
cargo test -p relayer-http yaml_scenarios          # the aggregator scenarios only
cargo run -p relayer-http -- config/config.yaml    # needs the KMS_<i>_API_KEY env vars named in the config
```

Every change must pass the first three. Check exit codes strictly (a grep on the output hides a failing build).

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
  -e KMS_00_API_KEY=... relayer-http            # the config is mounted at /app/config/config.yaml
```

The CI workflow for this image (the reusable docker template, change filters on `relayer-http/**` and
`kms-connector/crates/api/**`) is not wired yet.

## Rules

**No gateway interaction.** The relayer never talks to the gateway chain and carries no gateway-specific
configuration, optional checks included. Every on-chain read goes to a host chain through host contracts, Ethereum
being the reference.

**No unexpected panic.** Both crate roots deny `clippy::unwrap_used`, `expect_used`, `indexing_slicing` and `panic`
(allowed in tests). Every failure is a typed error returned to the caller. Configuration validation bounds every value
the runtime arithmetic relies on (`call.timeout <= 60s`, `backoff_max <= call.timeout`, semaphore size capped) so that no
arithmetic can overflow later. A panic inside a spawned task is counted as a failure, never propagated.

**Fail fast.** Never wait for something that cannot change the outcome: when `counted + pending < threshold` the
remaining calls are cancelled at once. One deadline per request, no queue, no ETA; the client retries.

**HPA compatible.** No per-request state outside the request's own future: no shared map, no registry, no sticky
session. Every replica is identical; the call semaphore is per pod. If per-request state is ever needed, it goes behind
a trait so that Redis (or another store) can back it later.

**Errors.** Modules return typed errors (`thiserror`), with what the caller needs to act (counts, dominant connector
error). Errors are logged once, at the boundary that answers the client. No `error!` inside a module for an expected
failure.

**Logging.** `tracing` only, structured fields, short messages, `request_id` on every span. Never log request or
response bodies, shares, signatures, header values or URLs. The subscriber is installed by `main`, never by a module;
`log:` is optional and defaults to JSON lines (`log.format`: `json` | `pretty` | `compact`, plus the relayer's
`show_*` switches); the level filter is `RUST_LOG`, default `warn,relayer_http=info`.

**Configuration.** Nested structs use `deny_unknown_fields`; `validate()` messages name the field with its dotted path
(`kms_aggregator.call.timeout must …`); secrets appear only as env var names; durations carry a unit.

**Minimal code.** Few types, one function per loop, a one-line doc comment per item, no framework where a function does.
Production-ready means no panic, typed errors, logs, cancellation and tests, not features.

**Compatibility.** The connector interface comes from the `kms-connector-api` crate (path dependency, never its
`endpoint` feature): DTOs, routes, error codes and their `retryable` flag are used as they are, never redefined.

**Tests.** A unit test per function, next to it. Paused tokio time (`start_paused`) for anything timed, so durations
are exact and tests are instant. Behaviour is described in YAML scenarios run by `cargo test`; adding a case is adding
a file.

**Documentation.** Every module has a `docs.md` produced with the module and updated in the same change as any
behaviour change, plus a one-line doc comment on every item. The error code table in `endpoint/docs.md` (every code,
its status, when it happens, how aggregation errors map to it) is updated in the same change as any change to that
mapping. This `CLAUDE.md` is updated the same way: new rules and
changed descriptions land with the feature that motivates them.
