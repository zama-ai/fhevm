# relayer-http runbook

One section per alert of `alerts.yaml`; the anchor is the alert name in lowercase. Thresholds are unvalidated first guesses.

Dashboards (`design.md`, Dashboards): start on **relayer-http operator** (the alert's flow section, then its KMS node health table), follow a node or an outcome to **relayer-http lookup**, click a request to read its summary and every line of it. **relayer-http client stats** is for usage questions, not incidents. `<flow>` below is `Public decrypt`, `User decrypt` or `Pad decrypt`.

## RelayerHttpServerErrors

More than 5% of the requests answered 5xx.

1. Operator dashboard, the failing flow's section: "<flow>: error codes" says which `code` dominates. `upstream_transient`, `timeout`: the KMS nodes, go to RelayerHttpKmsNodeFailing. `internal`: the relayer's own bug, take a request from the logs and escalate. A rollout drains: in-flight requests finish normally.
2. Click "<flow>: outcomes" to list the failed requests in the lookup; click one: "Selected request: summary" (`failed_kms_nodes`, `cancelled_kms_nodes`, `reason`) and "Selected request: KMS node lines" name the culprits.

## RelayerHttpSlowRequests

The 95th percentile of a flow's requests is above 0.8 x the call timeout: they are about to hit the deadline.

1. Operator dashboard, the slow flow's section: "<flow>: attempt latency p95 by KMS node", one slow node or all? "<flow>: time spent after threshold" high, or "<flow>: wait after threshold" rising while the latency p50 is fine: one slow node delays every answer after the threshold was reached.
2. All nodes flat but "Call slot wait p95" (Service row) rising: the pod is saturated, see RelayerHttpCallSlotWaitHigh.
3. A hung node shows as slow: its attempts are recorded when cut at the deadline.

## RelayerHttpKmsNodeFailing

More than half of the attempts to one KMS node fail with an error that blames the node or the path to it. The codes that do not (client-caused errors, cut attempts, relayer panics) are not counted: the list is `not_node_health()` in `metrics.rs`, also used by the operator dashboard's "<flow>: KMS node health" (`error %`) and "<flow>: attempt error ratio by KMS node".

1. Operator dashboard, "<flow>: KMS node health" then "<flow>: failed attempts by KMS node and code": the code says why. `sender_authentication_failed`: the relayer's FhevmSig key is not accepted by that node's connector. `upstream_transient`, `overloaded`: the node or the network to it. A node that hangs is not counted here: it shows as slow in "<flow>: attempt latency p95 by KMS node", and in `cut`.
2. Click the node in the health table: the lookup lists the requests it failed or was cut from. Contact the node's operator with a `decryption_id` from one of them: their connector computes the same hash.

## RelayerHttpCallSlotWaitHigh

Attempts wait for an outbound call slot (`kms_aggregator.max_concurrent_calls`): the pod is saturated.

1. If KMS node latency is flat while the slot wait rises, scale the deployment out.
2. If a node's latency rose first, slots are held by slow attempts: handle that node first.

## RelayerHttpInternalErrors

At least one internal error in the last 5 minutes: a request ended with the code `internal` (an internal error of the aggregation, or a panic of a request handler), or a KMS node call panicked. It is the relayer's own defect: the KMS nodes and the client are not at fault.

1. Operator dashboard, "Internal errors" (Service row): click it to list the requests in the lookup. Logs: an ERROR line `internal error` carries the `request_id`, the `reason` (`handler panicked` for a panic, else the internal error's text) and, for a panic, its `location`; the client got `500` `internal`; the request's summary line (`request completed`, INFO, `code` `internal`) has the rest. Every line of the request carries its `request_id`.
2. A panic inside one KMS node call is one ERROR line `node task panicked` with the `kms_node` and the `location`, and `panic` in "<flow>: failed attempts by KMS node and code". The request may still succeed.
3. Escalate with the `request_id` and the log lines: a defect needs a fix, not a restart.

## RelayerHttpSignerFailing

Signing a connector call fails: the relayer's signing key (a local key, or an AWS KMS key) is unreachable or refuses. Nothing was sent to the KMS nodes for those calls, so they are not failed attempts of the nodes.

1. Logs: WARN `signing failed` lines carry the `error` of the signer (for AWS KMS: the permission or network error) and the `kms_node` whose call it was.
2. Check the signing key: for AWS KMS the pod's role and the key policy, and the endpoint's reachability. The relayer's requests fail with `unknown` while it lasts; they show in `failed_kms_nodes` of the summary line.

## Incident debugging

Not an alert: how to get more detail from the logs.

- DEBUG is on by default for relayer-http's own lines (`RUST_LOG=warn,relayer_http=debug`): every request has its per-node steps, in the lookup's Selected request section with the Lines box on `debug`.
- Never a global `debug` (`RUST_LOG=debug`): the libraries' DEBUG output (reqwest, rustls, hyper) may include URLs and connection details, which `OBSERVABILITY.md` says are never logged.
- `RUST_LOG=warn,relayer_http=info` drops the DEBUG lines: for a load test whose log volume matters (`design.md`, Traffic profile).
