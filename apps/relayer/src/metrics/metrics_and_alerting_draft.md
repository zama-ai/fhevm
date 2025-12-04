## Health Check Configuration

### Service Health Status

The Relayer exposes a simple health check endpoint to verify the service is running and able to accept traffic.

- **Endpoint**: `/health`
- **Response**: Returns `200 OK` with body `"OK"` when healthy.
- **Usage**: Used by Kubernetes liveness/readiness probes or load balancers.

## Relayer Core Metrics

The Relayer exposes metrics via Prometheus format on the configured metrics endpoint (default: `:<PORT>/metrics`). This document lists and describes metrics supported by the Relayer to help operators monitor throughput, latency, and blockchain connectivity.

**Metric Naming**: All metrics use the prefix `relayer_`.

### Blockchain Gateway Metrics

_Defined in `blockchain.rs`_

These metrics track the interactions between the Relayer and the Gateway contract on the blockchain.

#### Metric Name: `relayer_gateway_events_total`

- **Type**: CounterVec
- **Description**: Total count of events received from the Gateway contract, categorized by type and ID recognition.
- **Alarm**: If this stops incrementing, the Relayer may have lost connection to the blockchain RPC or the Indexer.

**Labels**:

- `event_type`:
  - `input_proof_response`
  - `public_decrypt_response`
  - `user_decrypt_response`
- `request_id_status`:
  - `known` (Event matches a local DB record)
  - `unknown` (Event does not match any local record)

#### Metric Name: `relayer_gateway_tx_total`

- **Type**: CounterVec
- **Description**: Total count of transactions submitted to the Gateway blockchain by the Relayer.
- **Alarm**: High rates of `failed` status indicate RPC issues or out-of-gas errors.

**Labels**:

- `status`: `submitted`, `succeeded`, `failed`
- `sender`: The wallet address of the sender.

#### Metric Name: `relayer_gateway_pending_tx`

- **Type**: Gauge
- **Description**: Current dynamic count of transactions currently pending in the mempool (submitted but not confirmed).
- **Alarm**: If this value remains high (> 10-20) for an extended period, the account nonce may be stuck or gas price is too low.

#### Metric Name: `relayer_gateway_tx_confirmation_seconds`

- **Type**: HistogramVec
- **Description**: Histogram of time (in seconds) taken for a transaction to be confirmed on the Gateway.
- **Alarm**: If P95 latency exceeds block time significantly, network congestion is high.

**Labels**:

- `status`: `succeeded`, `failed`
- `sender`: The wallet address.

### Transaction Lifecycle Metrics

_Defined in `transaction.rs`_

These metrics track the internal lifecycle of business logic transactions (requests handled by the Relayer logic, distinct from raw blockchain TXs).

#### Metric Name: `relayer_transaction_count`

- **Type**: CounterVec
- **Description**: Total number of business transactions processed, categorized by final status.
- **Alarm**: Sudden spike in `failed` status requires immediate log investigation.

**Labels**:

- `transaction_type`:
  - `input_request`, `input_response`
  - `user_decrypt_request`, `user_decrypt_response`
  - `public_decrypt_request`, `public_decrypt_response`, `public_decrypt_callback_{id}`
- `transaction_status`: `confirmed`, `failed`

#### Metric Name: `relayer_transaction_pending_gauge`

- **Type**: GaugeVec
- **Description**: Current number of business transactions currently being processed (in-flight).
- **Alarm**: If this gauge grows monotonically without decreasing, the internal processing queue is stalled.

**Labels**:

- `transaction_type`: Same as above (`user_decrypt_request`, `input_request`, etc.)

### HTTP API Metrics

_Defined in `http.rs`_

Standard RED (Rate, Errors, Duration) metrics for the Relayer's REST API.

#### Metric Name: `relayer_http_requests_total`

- **Type**: CounterVec
- **Description**: Total count of HTTP requests received.
- **Alarm**: Sudden drop to zero indicates network ingress failure.

**Labels**:

- `endpoint`: `/input-proof`, `/public-decrypt`, `/user-decrypt`, `/keyurl`, `unknown`
- `method`: `GET`, `POST`, `UNKNOWN`

#### Metric Name: `relayer_http_responses_total`

- **Type**: CounterVec
- **Description**: Total count of HTTP responses sent, tagged by status code.
- **Alarm**: Rate of `5xx` status codes > 1%.

**Labels**:

- `endpoint`: See above.
- `method`: See above.
- `status`: The HTTP status code (e.g., `200`, `400`, `500`).

#### Metric Name: `relayer_http_request_duration_seconds`

- **Type**: HistogramVec
- **Description**: Histogram of HTTP request processing duration in seconds.
- **Alarm**: If P99 latency > 2s for non-blocking endpoints.

**Labels**:

- `endpoint`: See above.
- `method`: See above.
- `status`: See above.

### Cache Metrics

_Defined in `cache.rs`_

Metrics related to internal caching of requests and responses to reduce database/blockchain load.

#### Metric Name: `relayer_cache_operations_total`

- **Type**: CounterVec
- **Description**: Total number of cache operations recorded.
- **Alarm**: A hit rate of 0% implies the cache is not working or TTL is too short.

**Labels**:

- `cache_type`:
  - `user_decrypt_request`
  - `user_decrypt_response`
  - `public_decrypt`
- `operation`: `hit`, `miss`

## Monitoring Integration

### Endpoints

```bash
# Metrics endpoint (Prometheus format)
curl http://localhost:<PORT>/metrics

# Health endpoint (Liveness)
curl http://localhost:<PORT>/health    # Returns "OK"
```

### Prometheus Scrape Configuration

**Key Configuration Points**:

- **Metrics Endpoint**: `/metrics`
- **Scrape Interval**: Recommended 15-30 seconds (due to block times).
- **Target Labels**: Filter by `app=relayer` or similar Kubernetes label.

### Alerting Guidelines

Based on the exposed metrics, the following alerts are recommended:

1.  **Stuck Transactions**:

    - Condition: `relayer_gateway_pending_tx > 20` for > 5 minutes.
    - Impact: Transactions are not being mined; nonce might be stuck.

2.  **High Failure Rate (Blockchain)**:

    - Condition: `increase(relayer_gateway_tx_total{status="failed"}[5m]) > 5`
    - Impact: Wasting gas or invalid payloads being generated.

3.  **API Errors**:

    - Condition: `rate(relayer_http_responses_total{status=~"5.."}[5m]) > 0.1`
    - Impact: API consumers are facing outages.

4.  **Internal Processing Stall**:
    - Condition: `relayer_transaction_pending_gauge` is high while `rate(relayer_transaction_count[5m])` is 0.
    - Impact: The internal event loop or worker pool is deadlocked.

## Grafana Dashboard Suggestions

A Relayer dashboard should include:

- **Gateway Throughput**: Time series of `relayer_gateway_tx_total` and events.
- **Mempool Depth**: Stat panel for `relayer_gateway_pending_tx`.
- **API Latency**: Heatmap of `relayer_http_request_duration_seconds`.
- **Transaction Status**: Bar gauge of `relayer_transaction_count` (Confirmed vs Failed).
- **Cache Efficiency**: Pie chart of `relayer_cache_operations_total` (Hit vs Miss).


