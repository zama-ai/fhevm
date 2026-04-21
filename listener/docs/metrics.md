# Listener Metrics Catalog

Prometheus metrics exposed on the `/metrics` endpoint (default `:9090`).
Enabled via `telemetry.enabled: true` in config.

---

## Listener Core Metrics

### Cursor Liveness

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_cursor_iterations_total` | Counter | `chain_id` | Total main cursor loop iterations. **Stall detection:** `rate(...[5m]) == 0` means the cursor is stuck. |

### Chain Sync

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_db_tip_block_number` | Gauge | `chain_id` | Latest canonical block number persisted in the database. |
| `listener_chain_height_block_number` | Gauge | `chain_id` | Latest block number reported by the RPC node. |

**Sync lag** = `listener_chain_height_block_number - listener_db_tip_block_number`

### Reorgs

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_reorgs_total` | Counter | `chain_id` | Total chain reorganizations detected by the cursor. |

### Fetch Timing

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_block_fetch_duration_seconds` | Histogram | `chain_id` | Wall-clock time to fetch a single block (RPC call + receipts). |
| `listener_range_fetch_duration_seconds` | Histogram | `chain_id` | Wall-clock time to fetch and process an entire block range (producer + consumer). |

### Publishing

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_publish_errors_total` | Counter | `chain_id` | Failures when publishing block events to the broker. Incremented per failed publish attempt (after broker-level retries are exhausted). |

### Error Classification

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_transient_errors_total` | Counter | `chain_id`, `error_kind` | Transient (infrastructure) errors from the handler error classifier. These trigger circuit breaker and broker retries. |
| `listener_permanent_errors_total` | Counter | `chain_id`, `error_kind` | Permanent (logic) errors from the handler error classifier. These are dead-lettered without retry. |

**`error_kind` label values:**

| Value | Source Error | Transient/Permanent |
|-------|-------------|---------------------|
| `block_fetch` | `CouldNotFetchBlock` | Transient |
| `block_compute` | `CouldNotComputeBlock` | Transient |
| `database` | `DatabaseError` | Transient |
| `chain_height` | `ChainHeightError` | Transient |
| `slot_buffer` | `SlotBufferError` | Transient |
| `broker_publish` | `BrokerPublishError` | Transient |
| `payload_build` | `PayloadBuildError` | Transient |
| `message_processing` | `MessageProcessingError` | Transient |
| `invariant_violation` | `InvariantViolation` | Permanent |

### Block Compute Verification

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_compute_block_failure_total` | Counter | `chain_id`, `stalling` | Block hash verification failures. `stalling=true` means the listener halted on this error; `stalling=false` means it was skipped (allow_skipping mode). |
| `listener_compute_transaction_failure_total` | Counter | `chain_id`, `stalling` | Transaction root verification failures. Same `stalling` semantics. |
| `listener_compute_receipt_failure_total` | Counter | `chain_id`, `stalling` | Receipt root verification failures. Same `stalling` semantics. |

**`stalling` label values:**

| Value | Meaning |
|-------|---------|
| `true` | `compute_block_allow_skipping: false` — verification failure stalls the listener (error propagated) |
| `false` | `compute_block_allow_skipping: true` — verification failure logged and skipped (permissive mode) |

---

## RPC Provider Metrics (`SemEvmRpcProvider`)

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `listener_rpc_request_duration_seconds` | Histogram | `method`, `endpoint` | Wall-clock time per JSON-RPC call (includes semaphore wait time). |
| `listener_rpc_requests_total` | Counter | `method`, `endpoint`, `status` | Total RPC requests. `status` is `success` or `error`. |
| `listener_rpc_errors_total` | Counter | `method`, `endpoint`, `error_kind` | RPC errors broken down by failure type. |
| `listener_rpc_semaphore_available` | Gauge | `endpoint` | Available permits in the RPC concurrency semaphore. Low values indicate RPC saturation. |

### `endpoint` label

The `endpoint` label is the **host** portion of the configured `rpc_url`, extracted
via `url::Url::host_str()` at provider construction time (falling back to
`"unknown"` if the URL cannot be parsed).

**Examples:**

| Configured `rpc_url` | Emitted `endpoint` label |
|----------------------|--------------------------|
| `https://ethereum-rpc.publicnode.com` | `ethereum-rpc.publicnode.com` |
| `https://eth-mainnet.g.alchemy.com/v2/MY_SECRET_KEY` | `eth-mainnet.g.alchemy.com` |
| `https://mainnet.infura.io/v3/MY_PROJECT_ID` | `mainnet.infura.io` |

API keys embedded in the URL **path** or **query** are deliberately NOT emitted —
only the host is exposed. This keeps Prometheus scrapes free of secrets.

**Use cases:**
- Compare latency and error rates across multiple RPC providers (Alchemy vs Infura vs self-hosted).
- Detect provider-specific outages (one endpoint's error rate spikes while others are healthy).
- Attribute semaphore saturation to a specific endpoint when rotating providers.

### `method` label values

`eth_blockNumber`, `eth_chainId`, `eth_getBlockByNumber`, `eth_getBlockByHash`,
`eth_getTransactionReceipt`, `eth_getBlockReceipts`, `eth_getTransactionReceipt_batch`

**`error_kind` label values:**

| Value | Description |
|-------|-------------|
| `deserialization` | Response could not be deserialized (likely node bug or schema mismatch) |
| `unsupported_method` | RPC method not supported by the node |
| `rate_limited` | HTTP 429 or rate-limit error from the node |
| `transport` | Network/connection failure |
| `not_found` | Block or receipt returned null |
| `batch_error` | Batch request failed (HTTP error or parse failure) |
| `batch_unsupported` | Node does not support batch JSON-RPC |

---

## Broker Metrics (reference)

The `broker` crate emits its own metrics (prefixed `broker_*`).
See `crates/shared/broker/src/metrics.rs` for the full catalog, including:
- `broker_messages_published_total`
- `broker_publish_errors_total`
- `broker_publish_duration_seconds`
- `broker_messages_consumed_total`
- `broker_handler_duration_seconds`
- `broker_circuit_breaker_state`
- `broker_queue_depth_*`

---

## Grafana Query Examples

### Sync lag (blocks behind chain tip)
```promql
listener_chain_height_block_number - listener_db_tip_block_number
```

### Cursor stall alert (no iterations in 5 minutes)
```promql
rate(listener_cursor_iterations_total[5m]) == 0
```

### Reorg rate (per hour)
```promql
increase(listener_reorgs_total[1h])
```

### P99 block fetch latency
```promql
histogram_quantile(0.99, rate(listener_block_fetch_duration_seconds_bucket[5m]))
```

### RPC error rate by method and endpoint
```promql
sum by (method, endpoint) (rate(listener_rpc_errors_total[5m]))
```

### RPC latency by method and endpoint (P95)
```promql
histogram_quantile(0.95, sum by (method, endpoint, le) (rate(listener_rpc_request_duration_seconds_bucket[5m])))
```

### Per-endpoint error ratio (compare RPC providers)
```promql
sum by (endpoint) (rate(listener_rpc_errors_total[5m]))
  /
sum by (endpoint) (rate(listener_rpc_requests_total[5m]))
```

### Transient error rate by kind
```promql
sum by (error_kind) (rate(listener_transient_errors_total[5m]))
```

### RPC semaphore saturation
```promql
listener_rpc_semaphore_available == 0
```

### Compute verification failure rate (by stalling)
```promql
sum by (stalling) (rate(listener_compute_transaction_failure_total{chain_id="$chain_id"}[5m]))
```

### Any stalling compute failure (alert candidate)
```promql
sum(increase(listener_compute_block_failure_total{stalling="true"}[5m])
  + increase(listener_compute_transaction_failure_total{stalling="true"}[5m])
  + increase(listener_compute_receipt_failure_total{stalling="true"}[5m])) by (chain_id) > 0
```

---

## Initialization

All gauges are initialized to `0` at startup so that Grafana discovers the time series
on the first scrape, even before the first cursor iteration completes.

Block-compute failure counters (`listener_compute_{block,transaction,receipt}_failure_total`)
are **also** pre-seeded at `0` at startup for every `{chain_id, stalling}` combination
(`stalling=true` and `stalling=false`).

Why: Prometheus `increase()` / `rate()` needs at least two samples in the lookback window
to compute a delta. Without pre-seeding, a counter going from "absent" directly to `1`
on the first failure would make `increase(...[24h])` report `0` — there is no baseline
to compare against. Seeding the series at `0` makes the first real failure show up
immediately as `1` in stat panels and alerts.
