# relayer-http observability policy

Applies to `crates/relayer-http/src/` and `monitoring/`. Read before adding a log line, a metric, an alert or a dashboard.

This policy states rules. What exists (log fields, metrics, label values) is defined in code and published as generated references. Why each signal exists, what is planned and what is deliberately not measured: `monitoring/design.md`.

## Sources of truth

| What                                    | Defined in                    | Generated reference         |
| --------------------------------------- | ----------------------------- | --------------------------- |
| Summary line fields                     | code, one definition          | `monitoring/summary_log.md` |
| Metrics: name, type, labels, help text  | code, the metric registry     | `monitoring/metrics.md`     |
| Labels: names, values, bound            | code: enums and configuration | `monitoring/labels.md`      |

- A test fails when a generated reference differs from the code. Regenerate it; never edit it.
- A test fails when an alert or dashboard query names a metric, label or log field that does not exist.

## Release notes

Operators build their own dashboards and alerts on the generated references, so every change to them is announced. A change that touches `monitoring/summary_log.md`, `monitoring/metrics.md` or `monitoring/labels.md` adds an entry under **Observability** in the relayer-http release notes:

| Change                                     | Entry                                      |
| ------------------------------------------ | ------------------------------------------ |
| A field, metric, label or label value added | added                                     |
| One renamed, removed, or its meaning changed | **breaking**, with the replacement        |

TODO: the relayer-http release notes do not exist yet; create them with the first release that ships this policy, and add a CI check that a change to a generated reference comes with a release-notes entry.

## Identifiers

| Identifier      | Made by                                             | Same request sent twice |
| --------------- | --------------------------------------------------- | ----------------------- |
| `request_id`    | relayer-http, random UUIDv4, returned to the client | different               |
| `decryption_id` | the request content: its EIP-712 signing hash       | identical               |
| `kms_node`      | the KMS node's name in the configuration            | —                       |

- `request_id` is random: it reveals nothing about the server, such as arrival time.
- `decryption_id` is the key to quote to a KMS operator: their connector computes the same hash.
- No traces: relayer-http is one process, and KMS operators run separate backends, so a shared trace could never join. `decryption_id` covers cross-operator correlation.

## Logs

**Summary line.** One INFO line per request, when the response is sent, whatever the outcome. It carries every identifier, the public request fields needed to reproduce it (handles, contract addresses, user address, chain id), the outcome, the latency, and which KMS nodes failed, were rejected or were cut. These fields and `decryption_id` appear on this line only.

**Other lines** carry `request_id`, plus `kms_node` when they concern one KMS node.

**Levels describe the relayer's health**, not the request's outcome:

| Level | Use                                                                                          |
| ----- | -------------------------------------------------------------------------------------------- |
| ERROR | the relayer's own software is broken: startup, server, panic, internal error                  |
| WARN  | degradation: one per retry or skipped retry, per KMS node call failed or cut, per response rejected |
| INFO  | the summary line; process lifecycle                                                           |
| DEBUG | per-step detail; on by default for relayer-http's own lines (`RUST_LOG=warn,relayer_http=debug`) |

A failed request is INFO: its summary line's outcome says so. WARN comes only from the degradation behind it.

**Fields.** Values go in fields; messages are short fixed phrases. Names: `request_id` (never `req_id`, `id`), `kms_node` (never `node`, `kms`, `connector`, `endpoint`), `error` in Display form (never `err`, `e`, Debug), `<name>_ms` (never `duration`, `latency`, `took`). Never a field named `message`: tracing writes one already.

**Bodies are never logged, at any level**: DEBUG is on in production, and logs are kept longer and read more widely than requests.

**Libraries** that log through the `log` crate (reqwest, rustls-platform-verifier) reach the same subscriber and filter. Their DEBUG output may carry URLs and connection details: never enable a global `debug`; incidents use `RUST_LOG=warn,relayer_http=debug`, the default.

**Panics** are one ERROR line, with their location and never their payload (it may hold request data): a request's `internal error` line, a KMS node task's `node task panicked` line, or `panicked` elsewhere. A failure logs a digest instead: the error with its field path, the body size, and the summary line's public fields. A KMS node's error response is logged as its code and message, truncated.

**Never logged:** bodies, user signatures, shares, keys, header values (the outgoing FhevmSig header is a credential; the parsed `Retry-After` wait, `retry_after_ms`, is a number, not a header value), URLs.

**Tests** capture the production JSON output with `logging::capture` and assert on fields and levels, never on substrings of the text. They cover: the summary line (exactly one per request, every outcome, its defined fields), the WARN of each degradation, and absence: no body, signature or header value appears in any line, whatever the outcome. Style rules (field names, fixed messages, `error` in Display form) are left to review against this policy, not to a lint.

**Lookup**, for humans and AI tooling: a handle or `decryption_id` finds one summary line per related request; its `request_id` finds every line of that request. The lookup dashboard implements both steps; a skill may wrap them.

## Metrics

| Signal                                                     | Role                                                            |
| ---------------------------------------------------------- | --------------------------------------------------------------- |
| Request rate, outcomes, **HTTP latency**                   | service level: what clients experience                          |
| Per KMS node: latency, errors by code, attempts per call   | analytics on the upstream KMS nodes                             |
| Outbound call slot wait                                    | pod saturation: rising while KMS node latency is flat means scale out |

- A KMS node attempt cut in flight is recorded at the time it was cut: a hung KMS node shows as slow.
- Exposed as Prometheus text on `GET /metrics`, on a port of its own, as in the relayer.
- Names start with `relayer_http_`, and `relayer_http_kms_` for KMS node metrics. Counters end in `_total`; durations are `_seconds` histograms. `promtool check metrics` passes on a scrape.
- Every metric has help text saying what one observation is: it becomes the generated reference.
- Label values come from enums or the configuration, never from request data: every label has a bound on its number of values.
- Prometheus stores one series per label combination, and a histogram about 14 (one per bucket, plus sum and count). A test computes the worst case from the label bounds and fails above 5,000 series per pod.
- So histograms carry few labels: KMS node latency is split by `kms_node` and `flow`, and errors by `code` are a separate counter. Splitting the histogram by `code` would cost 20 times more for a question rarely asked.
- A failure class needing an operator increments a metric: no metric, no alert.
- The aggregator's behaviour scenarios assert the KMS node metrics they produce.

## Alerts and dashboards

Shipped with the code in `monitoring/`, mixin-style, and changed in the same commit as the metrics they read. Imported by hand until a chart provisions them.

- **Alerts** fire on metrics, never on log text. They are Prometheus rule groups; each fires on a symptom an operator can act on, never on a cause alone, carries a summary, a description and a runbook link, and has a `promtool test rules` unit test.
- **Dashboards** are Grafana JSON with datasources as variables, and implement the lookup. Their goals and layout: `monitoring/design.md`.
