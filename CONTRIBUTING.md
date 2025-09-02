# Contribution Guidelines - Relayer

The goal of this document is to help us evolve our process and align our
development practices. Hence, it is intentionally kept lean. For now, this
only contains a coding guidelines section. We could add other topics as and
when we see fit.

## Coding Conventions

### Logging

- **Log once at the boundary** (e.g. request handler or task root), not at every layer.
- **Levels:**
  - `ERROR` → user-visible failure (e.g. request failed, HTTP 5xx).
  - `WARN` → degraded path but request succeeded (fallbacks, SLA breach).
  - `INFO` → state transitions (accepted, tx_submitted, event_matched, replied).
  - `DEBUG` → retries, decisions, non-sensitive details (sampled/disabled in prod).
  - `TRACE` → very verbose, only enable temporarily per module.

- **Schema fields** (include consistently in logs):
  `request_id, component, step, chain_id, network, tx_hash?, nonce?, gas?, block?, log_index?, retry_attempt?, error_kind?, latency_ms?, policy_version, tenant_id?`
- **Redaction:** never log secrets, private keys, tokens, or sensitive calldata.

### Tracing (OpenTelemetry)

- **Enabled by default.**
- **Span model:**
  - Root span per request (`handle_request`).
  - Child spans, each time a handler is dispatched by the tokio orchestrator, using `.instrument(Span::current())`.
  - Background spans for long-running tasks (`event_listener`).

- **On error:** mark span `Status=ERROR` and add OTEL `exception.*` fields.

  <details>
  <summary>Recommended OTEL exception fields</summary>
  - `exception.type`
  - `exception.message`
  - `exception.stacktrace`

  </details>

- **Sampling:**
  - App: head-sampling (\~5–10%).
  - Collector: tail-sampling to keep all error traces and optionally slow traces.

### Error Handling

We use two libraries for error handling, each serving a distinct purpose:

- **`thiserror`**
  - Used in library and lower-level modules.
  - Provides **typed, inspectable errors** that can be matched and documented.
  - Helps expose clear error variants when consumed by other crates.

- **`anyhow`**
  - Used at the application/boundary layer.
  - Provides ergonomic **dynamic errors** for bubbling failures upward.
  - Best practices:
    - Use `.context("…")` or `.with_context(|| format!("…"))` to add helpful context at each step.
    - Use `anyhow!("…")` to construct new errors with messages.
    - Use `Result<T> = anyhow::Result<T>` as the return type in application code.
    - Use `?` to propagate errors, ensuring they carry their source and any attached context.
    - For top-level handlers, log the error chain once and mark the span with `StatusCode::Error`.

**Retries:**

- Attempts → `DEBUG`.
- Fallback worked → `WARN`.
- Retry exhausted → boundary logs `ERROR`, components just return error (no double logging).

**Backtraces:**

- Capture at origin (typed errors with `#[backtrace]`).
- `anyhow` automatically captures one backtrace at error creation.
- Keep/emit backtraces only for server-side errors (5xx).
- Do not attach backtraces for expected client errors (4xx).

### HTTP ↔ Logging/Tracing Mapping

- **5xx errors:**
  - Log at `ERROR` with backtrace.
  - Span `Status=ERROR` + `exception.*` event.
  - Return 5xx to client.

- **4xx errors:**
  - Log at `INFO` or `WARN`.
  - Span `Status=UNSET`.
  - No backtrace.

- **2xx success:**
  - Log state transitions at `INFO`.
  - Span `Status=OK`.

### Metrics (for alerts, not logs)

- **Counters:** `requests_total{status}`, `tx_submit_total{result}`, `event_match_total{result}`, `retries_total{reason}`.
- **Histograms:** `send_tx_latency_ms`, `event_wait_latency_ms`, `e2e_latency_ms`.
- **Gauges:** `listener_block_lag`, `inflight_requests`, `queue_depth`.

### Do & Don’t Quicklist

- ✅ Log at boundaries, not everywhere.
- ✅ Use WARN for degradations, ERROR for user failures.
- ✅ Keep backtraces only for 5xx.
- ✅ Map HTTP status ↔ span status consistently.
- ❌ Don’t double-log the same error.
- ❌ Don’t leak secrets/PII.
- ❌ Don’t alert on logs (alerts come from metrics).
