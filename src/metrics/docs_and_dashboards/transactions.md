# Relayer Blockchain Transaction Metrics

**Monitoring the reliability, latency, and error rates of on-chain transaction submissions.**

This document outlines the metrics used to track the interaction between the Relayer and the Blockchain (EVM). These metrics are crucial for detecting stuck transactions, RPC instability, and critical failures where transactions are dropped after exhaustion.

## 1. Metric Specifications

### A. Gauge: In-Flight Transactions

#### Metric Name: `relayer_transaction_pending_gauge`

- **Type**: GaugeVec
- **Description**: The number of blockchain transactions currently being processed (in the mempool, simulating, or retrying).
- **Labels**:
  - `transaction_type`: `input_request`, `user_decrypt_request`, `public_decrypt_request`

### B. Counter: Transaction Volume & Status

#### Metric Name: `relayer_transaction_count`

- **Type**: CounterVec
- **Description**: The total number of transactions processed, categorized by their final status (Confirmed or Failed).
- **Labels**:
  - `transaction_type`: `input_request`, ...
  - `transaction_status`: `confirmed`, `failed`

### C. Histogram: Submission Latency

#### Metric Name: `relayer_transaction_duration_secs`

- **Type**: HistogramVec
- **Description**: Measures the time from submission start to final confirmation (or failure).
- **Unit**: Milliseconds (float)
- **Buckets**: `0.01`, to `10.0` secs
- **Labels**:
  - `transaction_type`, `status`

### D. Counter: specific Error Breakdown

#### Metric Name: `relayer_transaction_errors_total`

- **Type**: CounterVec
- **Description**: Granular count of specific errors encountered during the transaction lifecycle.
- **Labels**:
  - `error_type`:
    - `max_retries_exceeded`: **CRITICAL**. The engine gave up on the transaction.
    - `nonce_error`: Account sequence mismatch.
    - `transport_error`: Network connection or HTTP issues.
    - `rpc_error`: EVM execution revert or internal node errors.
    - `reverted`: Transaction failed because of revert.
    - `invalid_address`: Malformed address format/Invalid contract destination address.
    - `unknown_error`: Error that has not been triaged on the transaction engine.

---

## 2. Grafana Dashboard Panels

### Panel 1: Error Breakdown by Type

**Visualizes the specific reasons why transactions are failing or retrying.**

- **Visualization**: Time Series (Stacked)
- **Description**: Rate of errors per type. High "nonce_error" indicates sync issues; "transport_error" indicates RPC node issues.
- **Query**:
  ```promql
  sum by (error_type) (increase(relayer_transaction_errors_total[$__range]))
  ```

### Panel 2: Transaction Latency (P95)

**Visualizes the time it takes for a transaction to be mined.**

- **Visualization**: Time Series
- **Description**: The 95th percentile duration in secs.
- **Query**:
  ```promql
  histogram_quantile(0.95, sum by (le, transaction_type) (
    rate(relayer_transaction_duration_secs[5m])
  ))
  ```

### Panel 3: In-Flight Transactions

**Visualizes the current load on the transaction manager.**

- **Visualization**: Stat / Gauge
- **Description**: Number of transactions currently pending (Mempool + Retries). Should not grow monotonically.
- **Query**:
  ```promql
  sum by (transaction_type) (relayer_transaction_pending_gauge)
  ```
