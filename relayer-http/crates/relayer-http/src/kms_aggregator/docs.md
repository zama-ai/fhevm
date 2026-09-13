# kms_aggregator

Posts one decryption request to every KMS connector endpoint and turns their answers into one relayer response.
Reading order: this file, `aggregator.rs`, `call.rs`, `client.rs`, `flows/`, `config.rs`.

## 1. What it does, and what it does not

- Fans out `POST /v1/user-decrypt` or `POST /v1/public-decrypt` (DTOs from the `kms-connector-api` crate) to the `n`
  configured nodes, collects the answers until every node has finished or the deadline passes, and answers when
  `counted >= threshold`.
- **Trust boundary.** The gateway used to recover each KMS signer from the response signature and bind it to the
  transaction sender. Over HTTP there is no transaction sender, and the relayer holds neither the KMS signer set nor the
  Decryption EIP-712 domain the KMS core signs under. Stage 1 therefore verifies **no signature**, as the old relayer never
  did. Consequences, stated plainly: a byzantine node returning the honest public result with a garbage 65-byte signature
  lands in `signatures` (the SDK verifies public-decrypt signatures against the KMS signer set itself), and a bogus user
  share counts toward the threshold (the SDK drops it at reconstruction). Next step, when the source-authentication scheme
  is decided: signer recovery in `Flow::check`, or a connector-side signature over the response.
- **Two ids.** The relayer's `request_id` is passed to `run` for tracking and logs only. The connector's `decryptionId`
  is the content hash of the request body, recomputed by every node with the shared api crate; it is logged, never checked.

## 2. Vocabulary

| term | meaning |
|---|---|
| node | one KMS connector endpoint (`endpoints[i]`, named in logs) |
| accepted | a `200` response that passed `Flow::check` |
| counted | accepted responses that count toward the threshold: all of them for user decrypt, the largest agreeing group for public decrypt |
| threshold | counted responses needed to answer (configured per flow, `1 <= threshold <= n`; 9 and 5 at n = 13) |
| deadline | `call.timeout` after the start: the aggregation, and every call in it, ends by then |
| fail fast | `counted + pending < threshold`: even if every pending node answered we could not make it, so stop now |
| dominant error | the most frequent connector error code among the failed calls; what a handler maps to a user-facing error |

## 3. How an aggregation runs (`Aggregator::run`)

1. Serialise the request once; spawn one task per node (`Caller::call`) sharing the body, the deadline and a
   cancellation token (child of the process shutdown token).
2. Loop on the next finished call or the deadline timer (`select!`, deadline first).
3. A `200` goes through `Flow::check` → accepted or rejected. A failed call is counted with its error code. A cancelled
   call is counted.
4. After every result, fail fast if `counted + pending < threshold`: cancel the token, the pending calls return at once.
5. At the deadline: cancel the token, drain the pending calls (immediate), decide.
6. Decide once, after the loop:

| state | result |
|---|---|
| `counted >= threshold` | `Ok(Flow::output(accepted))` |
| shutdown token cancelled | `Err(Cancelled)` |
| deadline hit | `Err(Timeout { counted, threshold, dominant })` |
| otherwise (all nodes finished or fail fast) | `Err(ThresholdNotReached { counted, threshold, dominant })` |

One hung node therefore makes a request last the full `call.timeout` (accepted: more shares for the SDK, simplest loop).

## 4. One node call (`Caller::call`)

`permit → POST → decode → retry?` under the cancellation token. The semaphore (`max_concurrent_calls`) bounds HTTP calls
in flight across all aggregations; a permit is held for one attempt only, never while sleeping. Retries happen only for
retryable errors, at most `max_retries` times, with delay `min(delay * 2^(k-1), backoff_max)`, and never when the delay
would end after the deadline. The reqwest client keeps its own `timeout(call.timeout)` as a safety net for a socket that
stops answering; the aggregator's deadline is the decision.

| connector answer | `AttemptError` | retried? |
|---|---|---|
| 2xx, valid DTO | — | — |
| 2xx, unreadable or oversized body (> 4 MiB) | `Body` | no |
| non-2xx with the connector error body | `Api { status, error }` | `error.code.retryable()`: `malformed`, `sender_authentication_failed`, `kms_context_destroyed`, `unprocessable` are final; the other codes (incl. 429 `rate_limited`, 503 `overloaded`) retry with backoff; `unknown` follows the body flag |
| 401 or `sender_authentication_failed` | `Api` / `Status(401)` | never (our key is wrong for that node) |
| non-2xx without a JSON body (empty 404, HTML 502, 3xx: never followed) | `Status` | 408 and 5xx only |
| refused, reset, TLS, client timeout | `Transport` | yes |

## 5. Response checks (`flows/`)

What the gateway did, what the old relayer did, what we do:

| check | gateway | old relayer | here |
|---|---|---|---|
| KMS signer recovery + signer set + tx sender | yes | no | no (section 1) |
| one response per signer | yes | dedup by share index | byte-identical signature → `Duplicate` |
| public: count per identical `(decryptedResult, extraData)` | yes (per digest) | took the event | yes; answer = largest group, its signatures only |
| user: shares differ, counted as they arrive | yes | sorted by share index | yes, acceptance order |
| signature is 65 bytes | implicit | no | yes → `BadSignature` (the SDK asserts it on every share) |
| `decryptionId` echo | n/a | matched by id | logged, not checked |

## 6. Configuration → behaviour

```yaml
kms_aggregator:
  allow_insecure_http: false   # local only: plain http and `auth: none` to non-loopback hosts (an api_key over http is sent in clear)
  max_concurrent_calls: 64     # semaphore size, >= number of endpoints; one aggregation takes up to n permits
  call:
    timeout: 5000ms            # the deadline (<= 60s)
    retries: { max_retries: 0, delay: 500ms, backoff_max: 4s }   # max_retries <= 9; 0 < delay <= backoff_max
  public_decrypt: { threshold: 5 }
  user_decrypt: { threshold: 9 }
  endpoints:
    - { name: kms_00, url: "https://kms-00.example.net:8443", auth: { type: api_key, value_env: KMS_00_API_KEY } }
```
Rules: endpoints non-empty, unique names and URLs, http(s) with a host, no credentials/query/fragment; plain http or
`auth: none` only towards loopback unless `allow_insecure_http`. The API key is read from the env var at startup
(`Caller::new`) and sent as `authorization: Bearer <key>`; it is never in the config or the logs. The container image
needs CA certificates (rustls uses the platform verifier). `APP_KMS_AGGREGATOR__CALL__TIMEOUT=7s` overrides a field;
durations need a unit.

## 7. Cancellation and shutdown

- **Deadline / fail fast**: the aggregation cancels its token; every pending call returns `Cancelled` immediately with its
  attempt count and elapsed time (logged at debug), so the final log line accounts for every node.
- **Client gone** (the handler's future is dropped): the `JoinSet` drops, the node tasks are aborted, connections are
  closed. Nothing outlives the request: there is no sink to feed.
- **SIGTERM / SIGINT**: `main` cancels the process token; every running aggregation ends with `Cancelled`.

## 8. Handlers (next iteration)

```rust
async fn user_decrypt(State(app): State<App>, Json(request): Json<UserDecryptionRequest>) -> Response {
    let request_id = uuid::Uuid::now_v7().to_string();
    match app.user_decrypt.run(&request_id, request).await {
        Ok(output) => Json(output).into_response(),                    // {"result":[{payload,signature,extraData}]}
        Err(error) => error_response(&request_id, error),
    }
}
```

| `AggregationError` | `dominant` | suggested HTTP answer |
|---|---|---|
| `Timeout { .. }` | any | 504, "KMS nodes did not answer in time" |
| `ThresholdNotReached` | `acl_denied`, `user_signature_rejected` | 403 |
| `ThresholdNotReached` | `ciphertext_not_found` | 404 (or 503 "not ready" if the relayer retries later) |
| `ThresholdNotReached` | `malformed`, `unprocessable` | 400 |
| `ThresholdNotReached` | `sender_authentication_failed` | 502 (our credential) |
| `ThresholdNotReached` | other / none | 502 |
| `Cancelled` | — | 503 (shutting down) |
| `Internal` | — | 500 |

Outputs: user decrypt `{"result":[{"payload":"<hex>","signature":"<hex>","extraData":"0x00"}]}`; public decrypt
`{"decryptedValue":"<hex>","signatures":["<hex>"],"extraData":"0x00"}` (hex without `0x` for bytes, `0x` for extraData,
as the current relayer answers).

## 9. Logging

Span `aggregation{flow, request_id, decryption_id, handles}`. Events: `aggregation started` (info: nodes, threshold,
timeout_ms), `response accepted` (info: node, attempts, elapsed_ms, counted), `response rejected` (warn: reason),
`call failed` (warn: attempts, error), `retrying` / `giving up` / `call cancelled` / `deadline reached` /
`threshold unreachable` (debug), `node task failed` (error: a panic in a node task), `aggregation succeeded` (info) /
`aggregation failed` (warn) with all counters, `dominant` and `elapsed_ms`. Never logged: bodies, shares, signatures,
header values, URLs.

## 10. Testing

- Unit tests next to every function (`cargo test -p relayer-http`).
- Wire tests for the HTTP client against a `wiremock` server (headers, body, statuses, redirects, body cap).
- Scenarios: `scenarios/*.yaml`, run by `cargo test yaml_scenarios` under paused tokio time (a 5 s scenario runs in
  microseconds, `elapsed` is exact). Adding one = adding a file:

```yaml
flow: user_decrypt            # user_decrypt | public_decrypt
threshold: 9
timeout: 5s
max_retries: 0                # optional
delay: 10ms                   # optional, per attempt
nodes:                        # groups in node order
  - { count: 9, replies: [ok] }                    # ok | divergent | bad_signature | duplicate | hang | refused | <error_code>
  - { count: 2, replies: [rate_limited, ok] }      # one reply per attempt, the last one repeats
expect:                       # every field except outcome is optional
  outcome: ok                 # ok | timeout | threshold_not_reached
  counted: 11
  responses: 11               # shares or signatures in the answer
  elapsed: 520ms
  dominant: acl_denied
  attempts: 13
```

Before every commit: `cargo fmt -- --check`, `cargo clippy --workspace --all-targets -- -W clippy::perf
-W clippy::suspicious -W clippy::style -D warnings`, `cargo test --workspace`.

## 11. Later

- **Response cache.** There is no sink and the pending calls are cancelled when the aggregation answers, so shares that
  arrive after the answer are lost. A response cache in the handlers would need share registration first (persist every
  accepted response as it arrives); fine for stage 1 without a cache.
- Moving the module to its own crate: it only depends on `config.rs` structs and the api crate.
- A new auth scheme: `Endpoint.auth` becomes an enum, `Endpoint::from_config` builds it.
- Transciphering: one more `Flow` implementation.
- Signer verification: `Flow::check` gets the signer set and the EIP-712 domain.
