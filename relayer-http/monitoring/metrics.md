<!-- Generated from the code: do not edit. Regenerate with `make -C relayer-http/dev references` (UPDATE_REFERENCES=1 on the test `references_are_current`). -->

# Metrics

Exposed as Prometheus text on `GET /metrics`, on the port of `metrics.endpoint` (default `0.0.0.0:9898`).

| Metric | Type | Labels | Worst-case series | What one observation is |
| --- | --- | --- | --- | --- |
| `relayer_http_requests_total` | counter | `flow`, `outcome`, `code` | 384 | One request answered, by flow, outcome and API error code. |
| `relayer_http_request_duration_seconds` | histogram (14 buckets) | `flow`, `outcome` | 272 | One request: seconds from receipt to the answer. |
| `relayer_http_kms_attempt_duration_seconds` | histogram (14 buckets) | `kms_node`, `flow` | 884 | One attempt that got an answer from a KMS node, or was cut while waiting for it: seconds from sending to the answer or the cut. No observation when the connection failed. |
| `relayer_http_kms_attempt_errors_total` | counter | `kms_node`, `flow`, `code` | 1248 | One failed attempt to a KMS node, by connector error code; `cancelled` for an attempt cut in flight, `panic` for a node task that panicked. |
| `relayer_http_kms_attempts` | histogram (8 buckets) | `kms_node`, `flow` | 572 | One call to a KMS node: how many attempts it made, the first try included; a failure to sign is not an attempt. |
| `relayer_http_kms_call_slot_wait_seconds` | histogram (14 buckets) | `flow` | 68 | One attempt: seconds spent waiting for an outbound call slot (`kms_aggregator.max_concurrent_calls`). |
| `relayer_http_signer_errors_total` | counter | `flow` | 4 | One failed attempt to sign a connector call (the signing key unreachable or refusing): nothing was sent, the KMS node is not at fault. |
| `relayer_http_kms_wait_after_threshold_seconds` | histogram (10 buckets) | `flow` | 52 | One aggregation that reached its threshold: seconds from reaching it to the answer, spent waiting for the remaining KMS nodes (until every node finished or the deadline). Buckets 5 ms to 5 s. |

Worst case for one pod: 3484 series (budget 5000).
