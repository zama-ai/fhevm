# Relayer Logging Policy

How we handle logging and correlation in the relayer's event-driven architecture.

The relayer processes jobs through async flows: HTTP requests trigger blockchain transactions, then gateway events complete the cycle. Correlation IDs track these flows end-to-end.

**Applies to:** `src/`

**Notation:**
- **For developers** - Sections for human understanding and implementation
- **For LLMs** - Sections for automated verification and compliance checking

**Goals:**
1. **Correlation** - Track requests and jobs end-to-end across async flows
2. **No duplicate logging** - Log once at boundaries, not in every layer
3. **Flow documentation** - INFO logs document standard flow milestones
4. **Structured logging** - Fields (searchable) + messages (concise)

---

## L1/L2 Debugging Strategy

**For developers:** Two levels of debugging for different audiences
**For LLMs:** Use this to determine appropriate log levels

| Level | Audience | Log Level | Has `int_job_id`? | Purpose |
|-------|----------|-----------|:-----------------:|---------|
| **L1** | First responder | INFO/WARN/ERROR | Yes | "Did job X succeed? Where did it stop?" |
| **L2** | Deep debug | DEBUG/WARN | Sometimes | "Why did it stop? What's infra doing?" |

### L1: Request Steps (First Responder)

Request steps track job progress through the system. Always include `int_job_id`.

- **INFO**: Happy path milestones (req_received, tx_sent, resp_sent)
- **WARN**: Degraded states that continue/recover (retrying, bounced, threshold_not_reached)
- **ERROR**: Failures at boundaries only

**Questions L1 answers:** "Did job X succeed?", "Where did it stop?", "Is it still in progress?"

### L2: Worker Steps (Deep Debug)

Worker steps track infrastructure operations. May or may not have `int_job_id`.

- **DEBUG**: Normal operations (tick_completed, task_enqueued, task_dequeued, tx_prepared, nonce_acquired)
- **WARN**: Infrastructure problems (worker_panicked, queue_full, nonce_too_low, transport_error)

**Enums:** `WorkerStep`, `ThrottlerStep`, `TxEngineStep`

**Questions L2 answers:** "Why did it fail?", "Is infra healthy?", "What's the tx engine state?"

### Overlap is OK

The same event can log both:
- Request step (INFO with `int_job_id`) - "Job X was bounced"
- Worker step (DEBUG) - "Throttler rejected task"

This is intentional: L1 and L2 serve different debugging needs.

---

## Quick Reference

**For developers:** At a glance reference for what to log where
**For LLMs:** Verify correlation IDs match this table

| Layer | Path | `request_id` | `ext_job_id` | `int_job_id` | Boundary? | ERROR? |
|-------|------|:------------:|:------------:|:------------:|:---------:|:------:|
| **HTTP handlers** | `src/http/endpoints/**/*.rs` | Yes | Yes | Yes | HTTP | Yes |
| **Gateway handlers** | `src/gateway/**/*_handler.rs` | No | No | Yes | Job | Yes |
| **Gateway listeners** | `src/gateway/arbitrum/listener.rs` | No | No | Yes | Events | Yes |
| **Background workers** | `src/store/sql/repositories/cron_task.rs` | No | No | Yes | Tasks | Yes |
| **Event dispatcher** | `src/orchestrator/` | No | No | Yes | No | No |
| **Repository** | `src/store/sql/repositories/` | No | No | No | No | No |
| **Transaction layer** | `src/gateway/arbitrum/transaction/` | No | No | Yes | No | No |
| **Core domain** | `src/core/` | No | No | No | No | No |

**Key rules:**
- Boundaries log errors - Non-boundaries return errors
- HTTP scope != Job scope (one job = multiple HTTP requests)
- Gateway handlers are job boundaries (catch repo/transaction errors and log them)
- `ext_job_id` only at HTTP boundary (never in internal processing)

**Special cases:**
- Repository: No logging, returns `SqlResult<T>`, callers log failures
- Transaction layer: Can use DEBUG (prep/nonce), WARN (retries with `attempt`, `max_attempts`, `backoff_ms`)
- `RelayerEvent`: Has `job_id` field, does NOT have `ext_job_id` field

---

## Correlation

**For developers:** Understand why we have three different IDs
**For LLMs:** Verify ID usage matches these definitions

| ID | Format | Purpose | Scope |
|----|--------|---------|-------|
| `request_id` | UUIDv7 | HTTP request | Single request |
| `ext_job_id` | UUIDv4 | User-facing job ID | Job (API boundary) |
| `int_job_id` | SHA256/UUIDv7 | Internal routing | Job (internal) |

**Relationships:**
- Multiple requests -> one job (POST create, GET status)
- Multiple `ext_job_id` -> one `int_job_id` (content deduplication)
- Database stores: `ext_job_id` <-> `int_job_id`

**Why separate?**
- `request_id` is HTTP-specific, not job-scoped
- `ext_job_id` stays at API boundary (no DB lookups for logging)
- `int_job_id` is the internal correlation ID

**Tracing:** `ext_job_id` (user) -> query DB -> `int_job_id` -> grep logs

---

## Logging Patterns

**For developers:** Follow these patterns when writing logs
**For LLMs:** Verify implementations match these patterns

### Structured Logging

**All logs must be structured:** Use fields for searchable data, keep messages concise.

**Format:**
```rust
// Correct - fields contain searchable data, message is concise
info!(
    int_job_id = %job_id,
    operation = "send_transaction",
    tx_hash = %hash,
    "Transaction sent"
);

// Wrong - searchable data in message
info!("Transaction sent for job {} with hash {}", job_id, hash);
```

**Field naming:** Use `snake_case`, prefer flat structure, correlation IDs always as fields.

**Messages:** Very concise description of the event (2-5 words typical). Message describes **what happened**, fields describe **details**.

**Predefined vs ad-hoc:**
- **INFO/WARN:** Use predefined steps from `src/logging.rs` (documents standard flow milestones)
- **ERROR/DEBUG:** Can be ad-hoc (situational), but still use structured format

Reference `src/logging.rs` for standardized step names and error codes.

### Error Handling

**Non-boundaries:** Return typed errors
```rust
// Repository/Core - return error, no logging
pub async fn update_status(&self, id: &str) -> SqlResult<()> {
    query.execute().await?  // Return to caller
}
```

**Boundaries:** Catch and log once with structured fields
```rust
// Gateway handler - catch and log with debugging context
if let Err(e) = repo.update_status(job_id).await {
    error!(
        int_job_id = %job_id,
        error = %e,
        db_operation = "update_status",
        "Status update failed"
    );
}

// HTTP handler - DB query error
error!(
    request_id = %request_id,
    ext_job_id = %job_id,
    error = %e,
    db_operation = "query_status_by_ext_id",
    "Query failed"
);
```

### Log Levels

| Where | Situation | Level | Required Fields |
|-------|-----------|-------|-----------------|
| **Boundaries (L1)** | Normal milestone | INFO | int_job_id |
| | Invalid user input (4xx) | INFO | int_job_id |
| | Bounced/rate-limited | WARN | int_job_id |
| | Retry succeeded | WARN | int_job_id |
| | Suspicious requests | WARN | int_job_id |
| | Operation fails (5xx) | ERROR | int_job_id, error |
| | Internal bug | ERROR | int_job_id, error |
| **Workers (L2)** | Normal ops (tick, dequeue) | DEBUG | - |
| | Task enqueued/dequeued | DEBUG | int_job_id (if available) |
| | Tx prepared/nonce acquired | DEBUG | nonce (if applicable) |
| | Tx submitted/receipt received | DEBUG | tx_hash, nonce |
| | Retry attempts | WARN | attempt, max_attempts, backoff_ms |
| | Infra degradation | WARN | Issue type, recovery |
| | Queue full/closed | WARN | queue_name, queue_size |
| | Nonce conflict (too low/high) | WARN | nonce, error |
| | Transport error | WARN | error |
| | Worker panicked | WARN | worker_name, error |

**Rules:**
- **WARN** = continues/recovers. **ERROR** = actually fails (boundaries only).
- **Bounced** = request rejected due to rate-limiting/backpressure (WARN, not ERROR).
- **DEBUG** = normal worker operations, only visible at L2 debug level.

### Required Fields

**Every log (as structured fields):**
- Correlation IDs per layer (see Quick Reference)

**ERROR logs add:**
- `error`: the error itself (contains type and details)
- Context fields as needed for debugging:
  - `db_operation`: for database errors (e.g., "insert_user_decrypt", "query_status_by_ext_id")
  - `operation`: for non-DB operations (e.g., "fetch_proof", "deserialize_response")
  - `shares_count`, `required`: for validation errors
  - Additional context as meaningful (avoid static config values)

**Guiding principle:** Only add fields that assist debugging. Metrics track error counts/types for alerting.

**Step names:** INFO/WARN use predefined steps from `src/logging.rs`. ERROR/DEBUG can be ad-hoc.

### Security & Performance

**Never log:** Secrets, full request bodies, personal data (unless explicitly safe/redacted)

**Performance:** No logging in tight loops, aggregate repeated warnings, never add DB queries for logging

### Redacted Data Fields

When logging requests/responses, redact sensitive cryptographic data. Show only sizes/counts.

| Type | Field | Log As |
|------|-------|--------|
| InputProof Request | `ciphertext_with_input_verification` | `len: {n}` |
| InputProof Response | `signatures` | `count: {n}` |
| UserDecrypt Request | `signature` | `len: {n}` |
| UserDecrypt Request | `public_key` | `len: {n}` |
| UserDecrypt Response | `result` | `count: {n}` |
| PublicDecrypt Response | `decrypted_value` | `len: {n}` |
| PublicDecrypt Response | `signatures` | `count: {n}` |

**Safe to log:** `contract_address`, `user_address`, `chain_id`, `handles`, `handle_contract_pairs`, `extra_data`, `request_validity`

---

## Multiple Relayers Operating

**For developers:** How logging behaves when multiple relayers are operated against the same contracts
**For LLMs:** Use this to verify log levels for gateway event handling

When multiple relayers are being operated against the same gateway contracts, each relayer routinely sees:

- Gateway events produced by other relayers
- Duplicate observations from its own redundant listeners
- Unmatched gateway reference IDs that are normal, not locally actionable

These are **normal operational traffic**, not failures.

### Gateway Event Observation (pre-correlation)

Before a gateway event is matched to a database request, the relayer is merely
observing chain activity. Pre-correlation logs use:

- **DEBUG** for the initial observation ("Observed gateway ... response")
- **DEBUG** for retry attempts when no match is found yet
- **DEBUG** for the final unmatched outcome ("No request matched gateway reference id; event ignored")

Pre-correlation logs should **not** include `int_job_id` when the value is only the
internal placeholder event ID. Use gateway identifiers instead: `gw_reference_id`,
`tx_hash`, `instance_id`, and event topic metadata.

### Post-correlation (match found)

After a database lookup resolves the event to a request, logs return to the
standard **INFO** trail with the real `int_job_id`:

- "Matched gateway response to request"
- "Response dispatched to HTTP handlers"
- Threshold reached, proof accepted/rejected

### Listener Traffic

Raw event ingestion and duplicate-event skips are **DEBUG** because they are
high-volume when multiple relayers share contracts. Listener lifecycle events (started, connected,
subscription active) remain **INFO**. Reconnects and dropped subscriptions remain **WARN**.

### Key Rules

| Situation | Level | Rationale |
|-----------|-------|-----------|
| Gateway event observed (pre-correlation) | DEBUG | May belong to another relayer |
| Retry looking for match | DEBUG | Normal timing race or foreign event |
| No match after retries | DEBUG | Expected when multiple relayers share contracts |
| Match found | INFO | Confirmed request progress |
| Duplicate listener event skipped | DEBUG | Expected with redundant listeners |
| Listener lifecycle (start/connect) | INFO | Operational milestone |
| Subscription dropped / provider retry | WARN | Locally actionable degradation |

---

## Verification

**For LLMs:** Use these checks to verify code compliance

### Static

```bash
# 1. No ERROR in non-boundaries (should find nothing)
rg 'error!\(' src/core/
rg 'error!\(' src/store/sql/repositories/ --glob '!cron_task.rs'
rg 'error!\(' src/orchestrator/

# 2. ERROR only in boundaries (should only find here)
rg 'error!\(' src/http/endpoints/
rg 'error!\(' src/gateway/ --glob '*_handler.rs'
rg 'error!\(' src/gateway/arbitrum/listener.rs
rg 'error!\(' src/store/sql/repositories/cron_task.rs

# 3. No ext_job_id in gateway handlers (should find nothing)
rg 'ext_job_id' src/gateway/ --glob '*_handler.rs'

# 4. Structured logging - check for string interpolation in messages (code smell)
# Look for patterns like: info!("Message {}", var) or error!("Error: {}", e)
# Should use structured fields instead
```

**Also check:**
- Structured format: Correlation IDs as fields (not in message string)
- `RelayerEvent` (`src/core/event.rs`): has `job_id`, NOT `ext_job_id`
- WARN in transaction layer includes structured fields: `attempt`, `max_attempts`, `backoff_ms`
- Boundaries handle repository errors (check for error handling when calling `repo.*()`)
- INFO/WARN steps use names from `src/logging.rs`

### Runtime

When reviewing logs:
- **Structured fields:** All correlation IDs appear as fields, not in messages
- **Concise messages:** Messages are 2-5 words, details in fields
- **Job correlation:** Same `int_job_id` across all logs for a job
- **Boundary IDs:** HTTP has all 3 IDs, gateway/listeners have only `int_job_id`
- **No duplicate errors:** One failure = one ERROR log
- **Traceability:** `ext_job_id` -> DB -> `int_job_id` -> grep finds all logs

---

**Reference:** Logging primitives in `src/logging.rs` - Database schema in `relayer-migrate/migrations/`
