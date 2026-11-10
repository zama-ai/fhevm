# relayer-http measurement design

What relayer-http measures, why, and what it deliberately does not. `OBSERVABILITY.md` states the rules; the generated references (`summary_log.md`, `metrics.md`, `labels.md`) list what exists. This file holds the goals behind them, including signals not built yet.

Status: ✅ exists · ➕ planned metric · 📜 computed from logs at query time.

## Traffic profile

| Profile    | Requests                                  | Summary lines (~1 KB each)        |
| ---------- | ----------------------------------------- | --------------------------------- |
| Normal     | a few hundred to a few thousand per day   | a few MB per day at most          |
| Load test  | ~100 per second, for the test's duration  | ~100 KB per second, ~360 MB per hour |

- Normal, measured on relayer v1 (testnet, 2026-11-06, a 53-minute sample extrapolated): ~2,100 public and ~870 user decryptions per day, bursty, test traffic included.
- Metric cost does not depend on traffic: one series per label combination, whatever the request rate.
- Log cost does: at normal traffic it is negligible, and a query over 30 days scans at most ~100,000 summary lines. Logs can therefore answer every question about request data, ad hoc, over any range.
- A load test is the heavy case for logs: query it over the test's window, not the whole day.
- DEBUG is on by default (`RUST_LOG=warn,relayer_http=debug`): a healthy request adds about 14 lines (`aggregation started`, one `response accepted` per node), so roughly 1,400 lines/s at a 100/s load test. Load tests may set `RUST_LOG=warn,relayer_http=info`.
- Normal traffic makes rates sparse: from a few requests per hour to a few per minute. Panels use `$__rate_interval` or longer windows, and ratio alerts need a minimum request count.

## Choosing the signal

| Signal   | When                                                                                     | Why                                                                                                   |
| -------- | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Metric   | bounded labels, and the question is alerted on or trended over weeks                     | cheap to keep, fast to query; labels come from enums or the configuration only (`OBSERVABILITY.md`)   |
| Logs     | the dimension is request data (contract, user, handle, `decryption_id`), or investigation | no cardinality cost; counted at query time over the selected range, within Loki's retention           |
| Neither  | the answer needs state across requests, or belongs to another component                  | relayer-http keeps no state across requests (HPA compatible); the KMS nodes publish their own metrics |

- Log counts see every pod at once: a repeat landing on another pod is still counted.
- If log volume grows until long-range queries get slow, the fix is a Loki recording rule: Loki evaluates the query every few minutes and stores a bounded result (`topk`, buckets) as a Prometheus series. Not needed at today's volume.
- An INFO line per KMS attempt is not worth it: 13 per request, while the attempt metrics already answer per-node questions. The per-node steps are DEBUG lines instead: on by default, and dropped by `RUST_LOG=warn,relayer_http=info` when their volume matters.

## Goals

### 1. Health: is the service answering? (alerts)

| Question                                   | Signal                                                    | Status |
| ------------------------------------------ | --------------------------------------------------------- | ------ |
| Request rate, outcomes, error codes        | `relayer_http_requests_total`                             | ✅     |
| Latency clients experience                 | `relayer_http_request_duration_seconds`                   | ✅     |
| Relayer defects (internal errors, panics)  | `code="internal"`, the `InternalErrors` alert             | ✅     |
| Saturation before latency rises; HPA input | requests in flight, by `flow`                             | ➕     |

### 2. KMS nodes: which node is slow, failing or useless?

| Question                                                    | Signal                                                                                                   | Status |
| ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- | ------ |
| Latency per node                                            | `relayer_http_kms_attempt_duration_seconds`                                                              | ✅     |
| Every attempt per node, by connector code, retryable or not, HTTP status | attempts by `kms_node`, `flow`, `code` (`ok` included), `retryable`, `status`; replaces `relayer_http_kms_attempt_errors_total` | ➕     |
| Retries per call                                            | `relayer_http_kms_attempts`                                                                              | ✅     |
| Does a node contribute answers, or get cut?                 | calls by `kms_node`, `flow`, `result`: counted, failed, rejected, cut by fail fast, cut by the deadline, cut by a client hangup | ➕     |
| Which nodes failed one request, and why                     | the summary line's node lists; the `attempt failed` lines (`kms_node`, `attempt`, `code`, `error`)       | 📜     |

- `retryable` and `status` follow from `code` (the connector's error codes, `kms-connector-api`): they add no series.

### 3. Capacity and cost: where does the time go?

| Question                                   | Signal                                                     | Status |
| ------------------------------------------ | ---------------------------------------------------------- | ------ |
| Outbound call slots too few                | `relayer_http_kms_call_slot_wait_seconds`                  | ✅     |
| Signing key unreachable or refusing        | `relayer_http_signer_errors_total`                         | ✅     |
| Signing latency (AWS KMS round trip)       | signer duration histogram, no labels                       | ➕     |
| Request shape, which drives KMS cost       | handles per request, by `flow`                             | ➕     |
| Time spent waiting after the threshold was reached | `relayer_http_kms_wait_after_threshold_seconds`, by `flow`: an aggregation collects until every node finished (`kms_aggregator/docs.md`), so one slow node delays every answer; per request, the summary field `threshold_reached_ms` | ✅     |

**Signing volume.** Each node call signs its own header (`kms_aggregator/docs.md`, One node call): one decryption is 13 AWS KMS `Sign` requests, so a 100 requests/s load test is ~1,300 per second.

- To verify: the account's AWS KMS request quota for asymmetric signing. It may be the first limit a load test hits.
- The signer histogram shows when signing starts to dominate a request.
- Signing once per request instead of once per node needs an RFC-038 change: the audience is per node.

### 4. Usage: how clients use the relayer

| Question                                             | Signal                                                                         | Status |
| ---------------------------------------------------- | ------------------------------------------------------------------------------ | ------ |
| Volume and success rate per flow                     | `relayer_http_requests_total`                                                  | ✅     |
| Requests per host chain                              | requests by `chain_id`, values from `http.supported_chain_ids` plus `other`    | ➕     |
| Top contracts, top users, distinct users             | `topk` / `count` over `contract_addresses`, `user_address`                     | 📜     |
| Repeats of one request, with their outcomes          | summary lines grouped by `decryption_id` and `outcome`                         | 📜     |
| Client retry behaviour: how often, how fast, why     | the repeats over time, with the `code` of each try                             | 📜     |
| Every request on a handle, a contract or a user      | the lookup: the value searched as text in the summary lines                    | 📜     |

- Repeats and retry behaviour come from logs only: detecting a repeat needs memory across requests.
- `contract_addresses` is one comma-separated value: a request on two contracts counts under the pair. A search for one contract still finds every request touching it.

## Not measured

| What                                                                             | Why not                                                                                   |
| -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Metrics labeled by contract, user, handle or `decryption_id` from request data | unbounded: any client can create series until the budget breaks; logs answer these        |
| `code` on histograms                                                             | 20 times the series for a rarely asked question; attempts by code are a separate counter  |
| Client IP, User-Agent                                                            | personal data; behind a load balancer the IP is the balancer's                            |
| KMS node internals (queue depth, compute time)                                   | the KMS nodes' own metrics; relayer-http sees the round trip only                         |
| Traces                                                                           | one process; KMS operators run separate backends (`OBSERVABILITY.md`, Identifiers)        |

## Series budget

- Prometheus memory and the hosted-Prometheus bill (Grafana Cloud, per active series) grow with the number of series: one per label combination, about 14 per histogram. With HPA the total is one pod's count times the pod count.
- The policy caps one pod's worst case at 5,000; a test computes it from the label bounds. The cap catches a multiplication by mistake (a `code` label on a histogram: ×24), not the cost of a metric chosen on purpose.
- Today 2,613; with the planned metrics about 2,900.
- The live count is far lower than the worst case: most node × code combinations never happen.

## Dashboards

| Dashboard (uid)                              | For, and when to open it                                                                                      | Holds                                                                                                                                                                                                                                                         | Source     |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| relayer-http operator (`relayer-http`)       | goals 1–3, on-call: open it first on an alert, or to check the service is healthy                             | Service row over the range (alerts firing, internal errors, signer errors, KMS nodes failing, requests by flow, call slot wait p95); one section per flow, never mixed: requests, outcomes, success, latency p50/p95, time spent after threshold (%), their charts (latency and wait after threshold p50/p95/p99 side by side), the KMS node health table next to node latency and failures, then the nodes over time (collapsed); Pod (collapsed) | Prometheus |
| relayer-http lookup (`relayer-http-lookup`)  | one request: a client's complaint, a node's or an outcome's requests from the operator dashboard              | one search box (handle, `decryption_id`, `request_id`, user or contract address); Matching requests: their counts by flow and outcome, latency p95 by flow, their table; Selected request (a row clicked): lines by level, the summary line as Field/Value, the KMS node lines, every line of the request | Loki       |
| relayer-http client stats (`relayer-http-usage`) | goal 4, usage questions: who uses the relayer, how, how often clients retry; not for incidents            | a summary (requests by flow, success per flow, distinct users and contract sets, repeats); who uses it (user decrypt: top contracts and users); repeats and retries (each repeated `decryption_id` by outcome, and the codes behind them; the Flow box filters this row only); a `decryption_id` clicked opens the lookup | Loki       |

- A nav bar on top of every dashboard: the same buttons in the same order (Operator · Lookup · Client stats), the current one highlighted; each keeps the time range, `app` and the Instance box. It is a text panel: Grafana's own dashboard links cannot highlight the current dashboard.
- Every dashboard has an Instance box: the label `instance` on metrics and logs (a pod in production; `fhevm-cli` or `host` locally).
- Flow sections fix their `flow` and prefix their titles with it; the latency panels carry no colour (typical range 500 ms – 2 s), the outcome and error ratio panels do.
- The lookup's Lines box (`level`) applies to the selected request's node lines and its every-line panel: `info` (INFO, WARN, ERROR) or `debug` (every line). DEBUG is on by default, so a healthy request shows its per-node steps too.
- The KMS node health table links a node to the lookup by searching its name as text: names that are prefixes of each other (`kms_1`, `kms_10`) would match each other; zero-padded names (`kms_00` … `kms_12`, as in `config/config.yaml`) do not.
- Two tests keep them true. `make check` checks statically that every query names existing metrics, labels and fields, and follows the Instance box. `make check-dashboards` (needs the local monitoring stack) runs every query against Prometheus and Loki, and compares the log panels with the counts of a scenario the code logs.
