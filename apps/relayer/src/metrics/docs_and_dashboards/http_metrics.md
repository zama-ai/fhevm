# Relayer HTTP API Metrics

**Comprehensive monitoring of the Relayer's REST API traffic, latency, and reliability.**

This document details the metrics exposed by the HTTP layer. These metrics follow the **R.E.D.** (Rate, Errors, Duration) methodology and are critical for monitoring the availability and performance of the Relayer service.

## 1. Metric Specifications

These metrics are defined in `http.rs` and exposed via the standard Prometheus endpoint.

### A. Counter: Request Traffic

#### Metric Name: `relayer_http_requests_total`

- **Type**: CounterVec
- **Description**: The total number of HTTP requests received by the application.
- **Labels**:
  - `endpoint`: `/input-proof`, `/public-decrypt`, `/user-decrypt`, `/keyurl`
  - `method`: `GET`, `POST`
  - `version`: `v1`, `v2`, `none`

### B. Counter: Response Outcomes

#### Metric Name: `relayer_http_responses_total`

- **Type**: CounterVec
- **Description**: The total number of HTTP responses sent, categorized by the specific status code.
- **Labels**:
  - `endpoint`: `/input-proof`, `/public-decrypt`, ...
  - `method`: `GET`, `POST`
  - `version`: `v1`, `v2`, `none`
  - `status`: The numeric status code (e.g., `200`, `400`, `429`, `500`).

### C. Histogram: Request Latency

#### Metric Name: `relayer_http_request_duration_seconds`

- **Type**: HistogramVec
- **Description**: Measures the time taken to process a request (from receiving headers to sending the response body).
- **Unit**: Seconds (Float)
- **Buckets**: Configured via `HttpMetricsConfig`.
- **Labels**:
  - `endpoint`, `method`, `version`, `status`

---

## 2. Grafana Dashboard Panels

The dashboard is structured to provide a Global Health view first, followed by dedicated sections for V1 (Legacy) and V2 (Current) API versions.

### Row 1: Global Health (Overall)

**High-level indicators aggregating all traffic regardless of version.**

#### Panel 1: Global Request Rate (RPS)

- **Description**: Total requests per second hitting the system.
- **Query**:
  ```promql
  sum(rate(relayer_http_requests_total[1m]))
  ```

#### Panel 2: Global Error Rate (5xx)

- **Description**: Percentage of total requests resulting in server errors (500, 502, etc.).
- **Query**:
  ```promql
  sum(rate(relayer_http_responses_total{status=~"5.."}[1m]))
  /
  sum(rate(relayer_http_requests_total[1m])) * 100
  ```

#### Panel 3: Global Latency (P95)

- **Description**: The 95th percentile latency across all endpoints and versions.
- **Query**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket[5m])))
  ```

#### Panel 4: Global Latency (P50 - Median)

- **Description**: The median latency across all endpoints.
- **Query**:
  ```promql
  histogram_quantile(0.50, sum by (le) (rate(relayer_http_request_duration_seconds_bucket[5m])))
  ```

---

### Row 2: V1 API Performance (Legacy)

**Dedicated monitoring for Version 1 endpoints.**

#### Panel 5: V1 /user-decrypt - Throughput

- **Query**: `sum(rate(relayer_http_requests_total{endpoint="/user-decrypt", version="v1"}[5m]))`

#### Panel 6: V1 /user-decrypt - Latency (P95 & P99)

- **Query (P95)**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/user-decrypt", version="v1"}[5m])))
  ```
- **Query (P99)**:
  ```promql
  histogram_quantile(0.99, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/user-decrypt", version="v1"}[5m])))
  ```

#### Panel 7: V1 /public-decrypt - Throughput

- **Query**: `sum(rate(relayer_http_requests_total{endpoint="/public-decrypt", version="v1"}[5m]))`

#### Panel 8: V1 /public-decrypt - Latency (P95 & P99)

- **Query (P95)**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/public-decrypt", version="v1"}[5m])))
  ```

#### Panel 9: V1 /input-proof - Throughput

- **Query**: `sum(rate(relayer_http_requests_total{endpoint="/input-proof", version="v1"}[5m]))`

#### Panel 10: V1 /input-proof - Latency (P95 & P99)

- **Query (P95)**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/input-proof", version="v1"}[5m])))
  ```

---

### Row 3: V2 API Performance (Current)

**Dedicated monitoring for Version 2 endpoints.**

#### Panel 11: V2 /user-decrypt - Throughput

- **Query**: `sum(rate(relayer_http_requests_total{endpoint="/user-decrypt", version="v2"}[5m]))`

#### Panel 12: V2 /user-decrypt - Latency (P95 & P99)

- **Query (P95)**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/user-decrypt", version="v2"}[5m])))
  ```
- **Query (P99)**:
  ```promql
  histogram_quantile(0.99, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/user-decrypt", version="v2"}[5m])))
  ```

#### Panel 13: V2 /public-decrypt - Throughput

- **Query**: `sum(rate(relayer_http_requests_total{endpoint="/public-decrypt", version="v2"}[5m]))`

#### Panel 14: V2 /public-decrypt - Latency (P95 & P99)

- **Query (P95)**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/public-decrypt", version="v2"}[5m])))
  ```

#### Panel 15: V2 /input-proof - Throughput

- **Query**: `sum(rate(relayer_http_requests_total{endpoint="/input-proof", version="v2"}[5m]))`

#### Panel 16: V2 /input-proof - Latency (P95 & P99)

- **Query (P95)**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/input-proof", version="v2"}[5m])))
  ```

---

### Row 4: Unversioned Endpoints

**Metrics for general utility endpoints.**

#### Panel 17: /keyurl - Throughput

- **Query**: `sum(rate(relayer_http_requests_total{endpoint="/keyurl"}[5m]))`

#### Panel 18: /keyurl - Latency (P95)

- **Query**:
  ```promql
  histogram_quantile(0.95, sum by (le) (rate(relayer_http_request_duration_seconds_bucket{endpoint="/keyurl"}[5m])))
  ```

---

### Row 5: Detailed Error Analysis

**Granular breakdown of HTTP status codes.**

#### Panel 19: Critical 500 Errors (Instant Rate)

- **Description**: Rate of unhandled internal server exceptions.
- **Query**:
  ```promql
  sum by (endpoint, version) (irate(relayer_http_responses_total{status="500"}[$__rate_interval]))
  ```

#### Panel 20: General 5xx Errors (Gateway/Timeout)

- **Description**: All server-side errors (502, 503), excluding 500.
- **Query**:
  ```promql
  sum by (endpoint, version) (rate(relayer_http_responses_total{status=~"5..", status!="500"}[5m]))
  ```

#### Panel 21: Client 4xx Errors

- **Description**: Rate of client errors (400, 401, 403), excluding 429.
- **Query**:
  ```promql
  sum by (endpoint, version) (rate(relayer_http_responses_total{status=~"4..", status!="429"}[5m]))
  ```

#### Panel 22: Rate Limit Hits (429)

- **Description**: Rate of requests throttled by the server.
- **Query**:
  ```promql
  sum by (endpoint, version) (rate(relayer_http_responses_total{status="429"}[5m]))
  ```
