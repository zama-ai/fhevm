# Relayer Database Infrastructure Metrics

**Monitoring the health, performance, and capacity of the PostgreSQL interaction layer.**

This document outlines the metrics used to track the low-level database operations within the Relayer. Unlike the _Status Metrics_ (which track business logic flow), these metrics track the raw performance of SQL queries and database connectivity errors.

## 1. Metric Specifications

### A. Histogram: Query Latency

#### Metric Name: `relayer_db_query_duration_seconds`

- **Type**: HistogramVec
- **Description**: Measures the time taken to execute SQL queries (from sending the query to receiving the result/acknowledgment).
- **Unit**: Seconds (Float)
- **Buckets**: `1ms`, `5ms`, `10ms`, `25ms`, `50ms`, `100ms`, `250ms`, `500ms`, `1s`, `2.5s`, `5s`, `10s`.
- **Labels**:
  - `table`: `user_decrypt_req`, `public_decrypt_req`, `input_proof_req`, `gateway_block_number_store`

### B. Counter: Database Errors

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

### Row 2: Health (Errors)

#### Panel 3: DB Error Rate

- **Visualization**: Time Series (Bars)
- **Description**: Rate of SQL errors occurring per second.
- **Goal**: Should be 0.
- **Query**:
  ```promql
  sum by (table) (rate(relayer_db_errors_total[5m]))
  ```
