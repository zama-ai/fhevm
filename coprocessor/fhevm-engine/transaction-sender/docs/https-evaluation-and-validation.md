# HTTPS sender: evaluation and release validation

## Candidate and comparison

This candidate changes the release sender from WSS to HTTP/HTTPS. It retains
`eth_sendRawTransaction`, ordered acknowledgment under the nonce lock and concurrent
receipt waiting. It does **not** switch to `eth_sendRawTransactionSync` or allow
concurrent submission acknowledgments.

The complete WSS candidate, including the cancellation-retention patch, is saved at:

```
backup/txn-sender-before-https-20260910-eeca968c7
```

Record the final candidate SHA/image digest at execution time. Compare exact source
and settings, not historical report hashes. The backup also preserves the original
commit structure before this branch was reorganized. No tests or benchmarks were
executed locally for this migration. The locked all-target compilation passed.
Helm rendering also passed with literal and secret-backed sender overrides;
the listener retained its separate WSS URL in both rendered configurations.

The earlier HTTP PR #2757 (`rudy/feat/http-for-gw-listener-and-tx-sender`, fetched
head `6eb0c5cf1`, implementation `f3788d7b5`) was inspected as reference. It also
uses an HTTP provider, but changes the listener, adds transport retry layers and
longer deadlines. Those changes are not imported into this minimal hotfix.

## What changed and what was removed

- Sender startup chain-ID lookup and transaction RPCs use a pooled reqwest client.
- Only `http://` and `https://` Gateway URLs are accepted. Use HTTPS outside local
  test fixtures. Schemes are not automatically rewritten: infra must supply the
  actual HTTPS endpoint, path and credentials.
- Connect deadline: 4 seconds. Whole HTTP request deadline: 30 seconds, including
  response-body reading. Shorter operation phase deadlines still apply. Increasing
  a phase above 30 seconds does not increase this underlying request ceiling.
- No HTTP transport retry layer, reqwest retries or redirects. In particular an
  ambiguous send is returned to the nonce/operation layer rather than silently
  replayed. HTTP errors do not normally produce WSS `BackendGone`; ordinary
  operation retries must recover connectivity without a process restart.
- `--provider-max-retries` is accepted as a hidden legacy argument and ignored;
  remove it from new rollout settings. `--provider-retry-interval` now controls
  startup chain-ID retries only. Existing operation error backoff remains in force.
- The vendored Alloy pubsub override and WSS-retention-only tests are removed.
  Registry Alloy remains pinned by the lockfile; WS dependencies used by other
  services and library tests remain. Those other services do not acquire a local
  pubsub fix through this sender hotfix.
- The finite internal receipt-watcher timeout stays. An outer Tokio deadline alone
  does not retire Alloy's heartbeat watcher, regardless of HTTP versus WSS.

HTTP cancellation does not undo remote acceptance or stop work already queued at
Conduit. Admission still bounds active attempts, not all pending transactions or
remote requests whose responses have been abandoned. This candidate must be
validated for recovery, not described as guaranteeing cancellation at the server.

## Infrastructure and deployment

Provide a production-equivalent Conduit HTTPS endpoint, exclusive funded test key,
allowed load/spend limits and a test host in the deployment network/region. Confirm
HTTP and WSS baseline endpoints route to comparable nodes and have equivalent
quotas. Get the actual URLs from infra; do not assume scheme substitution works.
Keep secrets out of command lines, CSVs and reports. For real Gateway calls provide
contract addresses, authorizations and distinct valid work inputs. Transfers alone
can measure ordered-phase capacity without those permissions.

The chart now supports a sender-only URL override:

```yaml
txSender:
  config:
    gatewayUrl:
      valueFrom:
        secretKeyRef:
          name: conduit-sender-http
          key: url
```

It falls back to `commonConfig.gatewayUrl` if unset. Keep the shared WSS URL for
`gwListener`; this hotfix does not migrate that service. Inspect rendered manifests
for both components and for secret-backed and literal overrides. Confirm precisely
one effective `GATEWAY_URL` for the sender. Use the override explicitly if the
shared value is WSS, otherwise the new sender will reject it during startup.

Validate this bundle, keeping the other production settings:

| Setting | Value |
|---|---|
| verify/add batch limits | 10 / 10 |
| max-inflight-sends | 32 (effective active batch ceiling 20) |
| send-txn-sync-timeout-secs | 4, separately for lookup and submission |
| gas-estimation-timeout-secs | 20 |
| txn-receipt-timeout-secs | 30 |
| gas-limit-overprovision-percent | 300 |
| provider-retry-interval | 4s, startup probes only |
| graceful-shutdown-timeout | 8s |
| gas-limit | absent; removed from hotfix |

Set both submission and receipt flags explicitly. The old receipt alias value of
4 seconds must not remain in the deployment. One active process per key, including
upgrades, comparisons and rollback.

## 1. Build and regression checks on the server machine

From `coprocessor/fhevm-engine`:

```sh
SQLX_OFFLINE=true cargo check --locked -p transaction-sender --all-targets
cargo test --locked -p transaction-sender --test hotfix_http_transport_tests
cargo test --locked -p transaction-sender --test hotfix_pipelining_tests \
  dropped_transaction_watcher_stops_polling_after_receipt_deadline
cargo test --locked -p transaction-sender
```

The HTTP-only target requires no database/Anvil. It checks scheme rejection,
no automatic retry of a 503 raw-submission response, and repeated abandoned
estimates followed by recovery without replay. It does not prove cancellation
of remote work. The other targets require the usual Anvil, solc, DB and signer-test
fixtures. Explicit WSS tests are retained as library error-classification checks,
not production transport evidence.

Also exercise HTTP connection refusal/reset, delayed headers and stalled response
bodies; verify the configured phase returns and permits are released. Check
redirect responses do not forward transaction bodies to another endpoint. Cover
HTTP 429/502/503/504, and observe operation retry counters and terminal outcomes,
not just whether a request returns an error. There must be no unexpected terminal
retirement of ciphertext work. Proof expiry at the existing six-attempt cap remains
intentional and must be counted separately.

## 2. Isolated acknowledgment comparison

Use the measurement definitions and driver requirements in
[the acknowledgment protocol](conduit-ack-throughput-protocol.md). That document's
WSS setup is for the saved comparison build. For this candidate the external driver
must use `gateway_http_client` and `connect_reqwest`, matching the binary, rather
than an unconfigured `connect_http` client. A standalone external driver still
needs to be supplied on the server machine; existing Anvil tests cannot be pointed
at Conduit by setting an environment variable.

Compare the saved WSS build and HTTPS candidate from the same host, with equivalent
endpoints, signer type, payload, deadlines and offered load. Use separate funded
keys or fully reconcile between runs; never run both providers on one key. Keep
the request method asynchronous in both candidates. Do not mix an HTTP sync-send
benchmark into the transport comparison.

First use transfers to isolate the ordered phase; then real Gateway calls if
available. Sweep worker counts 8/20/32/64 within approved limits, warm up 60 seconds,
measure for 300 seconds and repeat useful operating points at least three times.
Measure lock hold/wait, fee filling, signing, raw acknowledgment, receipt latency,
successful inclusions, unresolved work, phase errors, RPC counts, open sockets and
memory. Record cold connection/TLS setup separately from steady pooled operation.

A saturated nonce lock establishes an acknowledgment-path limit; idle lock time
while all workers await receipts does not. Do not claim capacity from reciprocal
median latency. Report mean/p50/p90/p99, sustained rates and all repetitions,
including slow ones. Use a separate bounded observer path; failed probes are unknown.

## 3. Release workload and recovery

Run the final local load fixture using the commands in the minimal hotfix plan:
1024 items per operation, arrivals 2/s per operation, a 90-second fault window and
600-second recovery deadline. Run fresh, pre-retried and cold-restart cases.
The fixture still has its own provider setup, so passing it alone does not establish
the new production client's policy or behavior against Conduit.

Then run production-equivalent Nitro and real Gateway contracts over HTTPS with
the exact bundle. Keep the HTTP sender's real startup and client wiring in the
experiment. Induce faults at a controlled test proxy/endpoint, never by disrupting
shared production infrastructure:

1. Estimate queuing, delayed responses and blackholes with continuing arrivals.
2. Requests rejected before submission; accepted requests with lost responses;
   requests delayed before forwarding, so pending lookup cannot initially see them.
3. TCP/keep-alive close, reset, temporary refusal and load-balancer 429/5xx.
4. Receipt blackholes and a dropped/replaced hash that never mines. The latter must
   stop heartbeat polling after internal expiry while the provider stays alive.
5. Cold restart with pending work: assert the old sender terminated and transactions
   remain unmined, then start a fresh provider with no cached nonce/admission state.

Use phase-separated full preparation timing, not estimate latency alone. Measure
both classes independently. Stop arrivals, terminate the sender, settle the chain
and reconcile an independent ledger of inserted work against successes, intentional
proof expiries and remaining rows. Receipt timeouts are unresolved outcomes, not
failed acceptance. Stop a run at its agreed unresolved-work/spend ceiling.

## Acceptance and rollout

Agree production arrival rates, backlog-at-restoration and recovery deadline before
execution. Require useful output above arrivals plus backlog/deadline, separately
per operation, with an agreed margin. The nominal historical combined target is
`4 + 2048/600 = 7.4133 tx/s`; use actual backlog at restoration if it differs.
Queue shrinkage from proof expiry is not useful throughput.

Require all of the following before selecting HTTPS for release:

- No unexplained accounting loss, no persistent nonce gap, and both queues drain.
- Repeated runs meet the target and agreed margin; no favorable-run-only claim.
- Full preparation gate is measured, or explicitly left open; service-side estimate
  percentiles alone cannot establish it.
- Repeated interruption/recovery does not accumulate client-retained work, sockets
  or watchers, or trigger automatic replay of abandoned RPC requests.
- Successful contract execution/estimation, expected terminal-error handling and
  unchanged proof-expiry policy hold with representative payloads.

Then canary one operator, watching useful output, queue age, retry/expiry rate,
acknowledgment and receipt tails, memory and connections. Avoid simultaneous senders.
If rollback is needed, stop the HTTP sender, account for pending work, deploy the
backup WSS artifact and restore that sender's WSS URL and settings. Do not switch
only the URL on the HTTP-only binary. Record all artifacts, settings, raw ledgers,
one-second CSVs and the exact reason for selecting either transport.
