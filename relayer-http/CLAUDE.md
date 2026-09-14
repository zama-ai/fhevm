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
├── Cargo.toml, rust-toolchain.toml        # workspace, pinned deps, toolchain 1.97.1
├── config/config.yaml                     # example configuration
└── crates/relayer-http/src/
    ├── main.rs                            # load settings → init logging → build App → wait for SIGINT/SIGTERM → cancel
    ├── lib.rs                             # App: everything a handler needs
    ├── settings.rs                        # Settings: YAML + APP_<SECTION>__<FIELD> env overrides
    ├── logging.rs                         # tracing subscriber (text or JSON)
    └── kms_aggregator/                    # fan-out and aggregation; start with its docs.md
```

Every module ships a `docs.md` next to its code. The `kms_aggregator` one is the reference for what a module doc
contains: what it does, an overall scheme, how it runs, configuration → behaviour, testing, scope.

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

## Rules

**No gateway interaction.** The relayer never talks to the gateway chain and carries no gateway-specific
configuration, optional checks included. Every on-chain read goes to a host chain through host contracts, Ethereum
being the reference.

**No unexpected panic.** Both crate roots deny `clippy::unwrap_used`, `expect_used`, `indexing_slicing` and `panic`
(allowed in tests). Every failure is a typed error returned to the caller. Configuration validation bounds every value
the runtime arithmetic relies on (`call.timeout <= 60s`, `max_retries <= 9`, semaphore size capped) so that no
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
response bodies, shares, signatures, header values or URLs. The subscriber is installed by `main`, never by a module.

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
