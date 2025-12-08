Here is the `metrics_db.md` documentation file, strictly based on the code provided and the structure requested.

---

# Relayer Database Infrastructure Metrics

**Monitoring the health, performance, and capacity of the PostgreSQL interaction layer.**

This document outlines the metrics used to track the low-level database operations within the Relayer. Unlike the _Status Metrics_ (which track business logic flow), these metrics track the raw performance of SQL queries, connection pool saturation, and database connectivity errors.

## 1. Metric Specifications

### A. Histogram: Query Latency

#### Metric Name: `relayer_db_query_duration_seconds`

- **Type**: HistogramVec
- **Description**: Measures the time taken to execute SQL queries (from sending the query to receiving the result/acknowledgment).
- **Unit**: Seconds (Float)
- **Buckets**: `1ms`, `5ms`, `10ms`, `25ms`, `50ms`, `100ms`, `250ms`, `500ms`, `1s`, `2.5s`, `5s`, `10s`.
- **Labels**:
  - `table`: `user_decrypt_req`, `public_decrypt_req`, `input_proof_req`, `gateway_block_number_store`

### B. Gauge: Connection Pool State

#### Metric Name: `relayer_db_pool_active`

- **Type**: Gauge
- **Description**: The number of currently active (in-use) database connections.
- **Unit**: Count (Integer)

#### Metric Name: `relayer_db_pool_idle`

- **Type**: Gauge
- **Description**: The number of currently idle database connections waiting in the pool, ready to be used.
- **Unit**: Count (Integer)

### C. Histogram: Pool Wait Time

#### Metric Name: `relayer_db_pool_wait_duration_seconds`

- **Type**: HistogramVec
- **Description**: Measures the time the application spends **blocked** waiting to acquire a connection from the pool.
- **Goal**: This should ideally be near zero (microseconds). If high, the pool is undersized or transactions are too slow.
- **Buckets**: `0.1ms` to `3s`.
- **Labels**:
  - `pool`: `primary`

### D. Counter: Database Errors

#### Metric Name: `relayer_db_errors_total`

- **Type**: CounterVec
- **Description**: Total number of database errors encountered (e.g., connection drops, constraint violations, SQL syntax errors).
- **Labels**:
  - `table`: `user_decrypt_req`, `public_decrypt_req`, `input_proof_req`, `gateway_block_number_store`

---

## 2. Grafana Dashboard Panels

### Row 1: SQL Performance (Latency)

#### Panel 1: Query Latency Heatmap

- **Visualization**: Heatmap
- **Description**: Distribution of query execution times. Helps identify sporadic slow queries.
- **Goal**: Keep the heatmap density at the bottom (fast buckets).
- **Query**:
  ```promql
  sum by (le) (rate(relayer_db_query_duration_seconds_bucket[5m]))
  ```

#### Panel 2: P95 Query Duration per Table

- **Visualization**: Time Series
- **Description**: The 95th percentile duration for queries, grouped by table.
- **Goal**: Identify which specific table is causing slowdowns.
- **Query**:
  ```promql
  histogram_quantile(0.95, sum(rate(relayer_db_query_duration_seconds_bucket[5m])) by (le, table))
  ```

---

### Row 2: Capacity (Connection Pool)

#### Panel 3: Pool Saturation (Active vs Idle)

- **Visualization**: Time Series (Stacked Area)
- **Description**: Visualizes the load on the connection pool.
- **Goal**: Ensure `active` does not consistently hit the `max_connections` limit.
- **Query**:
  ```promql
  relayer_db_pool_active
  relayer_db_pool_idle
  ```

#### Panel 4: Application Blocking Time (Pool Wait)

- **Visualization**: Time Series / Stat
- **Description**: How long the app waits to get a connection.
- **Goal**: **CRITICAL metric.** If this spikes > 10ms, the application is stalling because the DB cannot handle the concurrency.
- **Query**:
  ```promql
  histogram_quantile(0.99, sum(rate(relayer_db_pool_wait_duration_seconds_bucket[1m])) by (le))
  ```

---

### Row 3: Health (Errors)

#### Panel 5: DB Error Rate

- **Visualization**: Time Series (Bars)
- **Description**: Rate of SQL errors occurring per second.
- **Goal**: Should be 0.
- **Query**:
  ```promql
  sum by (table) (rate(relayer_db_errors_total[5m]))
  ```