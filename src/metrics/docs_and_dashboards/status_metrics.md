# Relayer Request Status Metrics

**Note on `pg_cron`:** If `pg_cron` performs the update in the background, **Rust will not see it**, and the metrics will not update. To make the Timeout alerts work, you must either move the timeout logic to Rust or accept that Timeouts might be under-reported in these specific metrics.
So for now, timed_out alerts must not be implemented until this specific features is moved to the application logic and not in cron.

**Detailed monitoring of the internal state machine for Input Proofs, User Decryption, and Public Decryption requests.**

This document outlines the metrics used to track the lifecycle of requests within the Relayer. These metrics are generated in real-time as the application hooks into database updates.

## 1. Lifecycle & Status Definitions

Understanding the flow is essential for interpreting the metrics.

| Status                 | Business Context                                                                                                                                                                                                                                   | Expected Duration                                              |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- |
| **`queued`**           | **Ingestion**: Payload received. Waiting for ACL/Readiness checks. <br> _Critical:_ If high, the Relayer is overwhelmed or Readiness checks are hanging.                                                                                           | < 10s max (Internal) <br> ~Ms (Readiness)                      |
| **`processing`**       | **Internally Queued before Transaction Broadcasting**: ACL passed. Request crafted and internally queued, waiting to be picked up by transaction engine. <br> _Warning:_ If high, transaction engine is backlogged or internal queue is congested. | Queue-dependent (typically < 1s, but can be longer under load) |
| **`tx_in_flight`**     | **Transaction Broadcasting**: Request picked up for sending to blockchain. Transaction is being broadcast. <br> _Critical:_ If high, transaction engine is slow or RPC is congested.                                                               | Block Time (e.g., 50ms - 1s)                                   |
| **`receipt_received`** | **Tx confirmed** Waiting for Gateway Event (Consensus or Decryption). <br> _Critical:_ If long, KMS is stalling or Relayer Listener is broken.                                                                                                     | Network dependent (~5s - 30s)                                  |
| **`completed`**        | **Success**: Final happy path. Request served to user or proof accepted.                                                                                                                                                                           | N/A (End State)                                                |
| **`timed_out`**        | **Error**: <br>1. ACL Readiness failed/timeout.<br>2. Protocol/KMS timeout <br>3. event missed by the listener(30m).                                                                                                                               | > 30m                                                          |
| **`failure`**          | **Error**: Transaction Engine failure (Gas, RPC error, invalid payload).                                                                                                                                                                           | N/A (End State)                                                |

---

## 2. Metric Specifications

### A. Gauge: Queue & State Depth

#### Metric Name: `relayer_request_count`

- **Type**: GaugeVec
- **Description**: The current number of requests sitting in a specific status _known to the application instance_.
- **Labels**:
  - `req_type`: `user_decrypt`, `public_decrypt`, `input_proof`
  - `status`: `queued`, `processing`, `receipt_received`, `completed`, `timed_out`, `failure`

### B. Histogram: Transition Latency

#### Metric Name: `relayer_request_status_duration_seconds`

- **Type**: HistogramVec
- **Description**: Measures **"How long did the request stay in the previous status?"**. It records the duration at the moment of transition.
- **Buckets**: `0.1s` to `3600s` (1h).
- **Labels**:
  - `req_type`: `user_decrypt`, ...
  - `previous_status`: The status the request just _left_ (e.g., if transitioning `queued` -> `processing`, label is `queued`). `completed`, `failed` and `timed_out` not taken in account since they are final states.

---

## 3. Grafana Dashboard Panels

TODO: refine

### Row 1: Global Health (The "Is it burning?" view)

#### Panel 1: Current Queue Size (Critical)

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt.
- **Visualization**: Stat / Time Series
- **Description**: Number of requests currently waiting for ACL/Readiness checks.
- **Goal**: Should be as low as possible. Spikes indicate backlog.
- **Query**:
  ```promql
  sum by (req_type) (relayer_request_count{status="queued"})
  ```

#### Panel 2: Active Processing (Internally queued before broadcast and in-flight transactions)

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt, input_proof.
- **Visualization**: Time Series
- **Description**: Number of requests waiting for broadcasting and blockchain transaction broadcast confirmations.
- **Goal**: Should reflect current transaction pending or in-flight mode. Flat line high = Problem with the broadcaster and the processing.
- **Query**:
  ```promql
  sum by (req_type) (relayer_request_count{status="processing"})
  ```

#### Panel 3: Waiting for - KMS, Coproc, Gateway: conensus reached, public decrypt ok, input proof done (Receipt Received)

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt, input_proof.
- **Visualization**: Time Series
- **Description**: Requests confirmed on-chain, waiting for off-chain KMS/Consensus events.
- **Goal**: If this grows monotonically, the Listener is dead or KMS is down, or copro is down.
- **Query**:
  ```promql
  sum by (req_type) (relayer_request_count{status="receipt_received"})
  ```

---

### Row 2: Latency & Performance (The "Why is it slow?" view)

#### Panel 4: Readiness Check Latency

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt.
- **Visualization**: Heatmap
- **Description**: Time spent in `queued` status. Measures how long ACL checks take.
- **Query**:
  ```promql
  sum by (le) (rate(relayer_request_status_duration_seconds_bucket{previous_status="queued"}[5m]))
  ```

#### Panel 5: Transaction/Mempool Latency

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt, input_proof
- **Visualization**: Heatmap
- **Description**: Time spent in `processing` status. Measures blockchain block times.
- **Query**:
  ```promql
  sum by (le) (rate(relayer_request_status_duration_seconds_bucket{previous_status="processing"}[5m]))
  ```

#### Panel 6: Protocol Copro/KMS Latency, Listener Failure

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt, input_proof
- **Visualization**: Heatmap
- **Description**: Time spent in `receipt_received` status. Measures KMS network speed and listener.
- **Query**:
  ```promql
  sum by (le) (rate(relayer_request_status_duration_seconds_bucket{previous_status="receipt_received"}[5m]))
  ```

---

### Row 3: Outcomes (The "Happy/Sad" view)

#### Panel 7: Completion Rate (Throughput)

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt, input_proof
- **Visualization**: Time Series (Rate)
- **Description**: Successfully completed requests per second.
- **Query**:
  ```promql
  sum by (req_type) (rate(relayer_request_count{status="completed"}[5m]))
  ```

#### Panel 8: Engine Failures (Errors)

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt, input_proof
- **Visualization**: Time Series (Bars)
- **Description**: Failed transactions (RPC errors, etc).
- **Query**:
  ```promql
  increase(relayer_request_count{status="failure"}[1h])
  ```

#### Panel 9: Timed Out:

- **Affected flows (req_type labels)**: user_decrypt, public_decrypt, input_proof
- **Visualization**: Time Series (Bars)
- **Description**: Timed out request (Readiness check fails (ACL), KMS or Copro in failure mode, Event missed from listener (should not happens)).
- **Query**:
  ```promql
  increase(relayer_request_count{status="timed_out"}[1h])
  ```
