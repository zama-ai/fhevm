<!-- Generated from the code: do not edit. Regenerate with `make -C relayer-http/dev references` (UPDATE_REFERENCES=1 on the test `references_are_current`). -->

# Metrics

Exposed as Prometheus text on `GET /metrics`, on the port of `metrics.endpoint` (default `0.0.0.0:9898`).

| Metric | Type | Labels | Worst-case series | What one observation is |
| --- | --- | --- | --- | --- |
| `relayer_http_requests_total` | counter | `flow`, `outcome`, `code` | 288 | One request answered, by flow, outcome and API error code. |
| `relayer_http_request_duration_seconds` | histogram (14 buckets) | `flow`, `outcome` | 204 | One request: seconds from receipt to the answer. |
| `relayer_http_kms_attempt_duration_seconds` | histogram (14 buckets) | `kms_node`, `flow` | 663 | One attempt that got an answer from a KMS node, or was cut while waiting for it: seconds from sending to the answer or the cut. No observation when the connection failed. |
| `relayer_http_kms_attempt_errors_total` | counter | `kms_node`, `flow`, `code` | 936 | One failed attempt to a KMS node, by connector error code; `cancelled` for an attempt cut in flight, `panic` for a node task that panicked. |
| `relayer_http_kms_attempts` | histogram (8 buckets) | `kms_node`, `flow` | 429 | One call to a KMS node: how many attempts it made, the first try included; a failure to sign is not an attempt. |
| `relayer_http_kms_call_slot_wait_seconds` | histogram (14 buckets) | `flow` | 51 | One attempt: seconds spent waiting for an outbound call slot (`kms_aggregator.max_concurrent_calls`). |
| `relayer_http_signer_errors_total` | counter | `flow` | 3 | One failed attempt to sign a connector call (the signing key unreachable or refusing): nothing was sent, the KMS node is not at fault. |
| `relayer_http_kms_wait_after_threshold_seconds` | histogram (10 buckets) | `flow` | 39 | One aggregation that reached its threshold: seconds from reaching it to the answer, spent waiting for the remaining KMS nodes (until every node finished or the deadline). Buckets 5 ms to 5 s. |
| `relayer_http_kms_node_excluded` | gauge | `kms_node` | 13 | 1 while the circuit breaker leaves the KMS node out (open, or being probed), 0 while it is in use; 0 for every node at startup and after a reset. |
| `relayer_http_kms_exclusions_total` | counter | `kms_node`, `code` | 312 | One exclusion of a KMS node by the circuit breaker, after a call or after a failed probe; `code` is the node's last answer during that call (`upstream_transient`, `overloaded`, `timeout`), or `cancelled` when it never answered before the deadline. |
| `relayer_http_kms_probes_total` | counter | `kms_node`, `outcome` | 52 | One probe of an excluded KMS node that ended: `succeeded` (it answered, the node is in use again), `failed` (unhealthy, excluded for another window), `cancelled` (nothing known, probed again by the next request). |
| `relayer_http_kms_floor_hits_total` | counter | `kms_node` | 13 | One unhealthy KMS node kept in use because excluding it would leave fewer than `circuit_breaker.min_nodes`: more nodes are unhealthy than the breaker may exclude, requests wait for the deadline again. Starts at 0 for every node. |

Worst case for one pod: 3003 series (budget 5000).
