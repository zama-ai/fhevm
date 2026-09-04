# Relayer User-Decryption Share Metrics

**Monitoring the fault tolerance handed to user-decryption clients, and the deduplication of POST requests.**

Reconstructing a user decryption needs the threshold in _valid_ shares. A client that receives exactly the threshold has spent its entire error budget before it starts: one corrupted share and the decryption fails. Every share beyond the threshold buys back one fault. These metrics make that budget observable — a cluster that has silently stopped delivering spares looks perfectly healthy on latency and status metrics alone.

## 1. Metric Specifications

### A. Histogram: Spare Shares

#### Metric Name: `relayer_user_decrypt_spare_shares`

- **Type**: HistogramVec
- **Description**: Shares returned to the client beyond the reconstruction threshold (`served - threshold`), i.e. how many corrupted shares that client can survive. Recorded on the terminal `200` only, never on the `202` holds — the GET is polled, so recording on every response would measure polling frequency rather than health. A client that keeps polling after success records a repeat observation; a well-behaved client stops at the first `200`.
- **Unit**: Shares (count, `>= 0` by construction)
- **Buckets**: `0`, `1`, `2`, `3`, `4`, `6`, `8`, `12`. Zero is its own bucket: it is the alerting case.
- **Labels**:
  - `req_type`: `user_decrypt` (direct and delegated are counted together)

A centralized KMS returns a single share and the threshold is 1, so this metric is legitimately always `0` there. It is meaningful only against a threshold committee.

### B. Counter: POST Request Deduplication

#### Metric Name: `relayer_request_cache_total`

- **Type**: CounterVec
- **Description**: Outcome of the deduplication check on each v2 POST. A `miss` was a new request and was dispatched to the orchestrator; a `hit` was already known and was served from the existing row without dispatching.
- **Labels**:
  - `req_type`: `input_proof`, `user_decrypt`, `public_decrypt`
  - `result`: `hit`, `miss`

---

## 2. Logs

A `WARN` is emitted alongside a `0` observation when the wait window was enabled (`user_decrypt_additional_shares > 0`) and the response still carried no spare. It carries `collected`, `required_threshold`, `elapsed_secs` and `ext_job_id`, so a zero-tolerance response can be traced to the request that produced it.

---

## 3. Grafana Dashboard Panels

### Row 1: Client Fault Tolerance

#### Panel 1: Zero-Tolerance Response Rate

- **Visualization**: Time Series
- **Description**: Fraction of user decryptions returned with no spare share at all.
- **Goal**: Should be ~0 on a threshold committee. Sustained `1` means the cluster is one corrupted share away from failing every decryption — the condition worth alerting on.
- **Query**:
  ```promql
  sum(rate(relayer_user_decrypt_spare_shares_bucket{le="0"}[5m]))
    / sum(rate(relayer_user_decrypt_spare_shares_count[5m]))
  ```

#### Panel 2: Median and P10 Spare Shares

- **Visualization**: Time Series
- **Description**: Typical and worst-case tolerance delivered to clients.
- **Goal**: Track the P10 — the median can look healthy while a tail of requests gets nothing.
- **Query**:
  ```promql
  histogram_quantile(0.5, sum(rate(relayer_user_decrypt_spare_shares_bucket[5m])) by (le))
  histogram_quantile(0.1, sum(rate(relayer_user_decrypt_spare_shares_bucket[5m])) by (le))
  ```

---

### Row 2: Request Deduplication

#### Panel 3: Dedup Hit Ratio

- **Visualization**: Time Series
- **Description**: Share of POSTs served from an existing row.
- **Goal**: Stable. A collapse means clients stopped reusing request identifiers; a spike means they are retrying.
- **Query**:
  ```promql
  sum by (req_type) (rate(relayer_request_cache_total{result="hit"}[5m]))
    / sum by (req_type) (rate(relayer_request_cache_total[5m]))
  ```

#### Panel 4: New Request Rate

- **Visualization**: Time Series (Bars)
- **Description**: Rate of POSTs that resulted in an actual dispatch, by request type.
- **Goal**: The real inbound workload, with retries excluded.
- **Query**:
  ```promql
  sum by (req_type) (rate(relayer_request_cache_total{result="miss"}[5m]))
  ```
