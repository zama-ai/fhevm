# endpoint

Minimal first version of the relayer's HTTP layer: two synchronous decryption routes and the Kubernetes probes, on
one port, with raw axum. Reading order: this file, `mod.rs`, `flows/user_decrypt.rs`, `flows/public_decrypt.rs`,
`validate.rs`, `error.rs`.

The error mapping is a first version (section 5).

## 1. Routes

| method | path | purpose |
|---|---|---|
| `POST` | `/v4/user-decrypt` | user decryption: the KMS nodes' signcrypted shares for the caller's public key |
| `POST` | `/v4/public-decrypt` | public decryption: the plaintexts and the KMS signatures over them |
| `GET` | `/liveness` | the process runs |
| `GET` | `/healthz` | the pod is ready to serve (503 while shutting down) |

Configuration (`http:` in `config/config.yaml`): `endpoint` (bind address), `max_body_bytes` (default 1 MiB, the connector's limit),
`body_read_timeout` (default 10 s, total time to receive a request body), `supported_chain_ids` (host chain ids a
handle may carry). One port for everything; no version endpoint yet.

## 2. Request payloads

The user-decrypt body is the connector's `v1` envelope (`attestationType`, `payload`, `signature`); the current
relayer's field names are kept (`ctHandle`, `ciphertextHandles`, `extraData`). The current SDK body (`attestedPayload`
with `version` and `type`) is not accepted: an unknown field is a `400 malformed`, so the SDK has to send this shape.
Hex fields are typed: a handle is 32 bytes, an
address 20 bytes, `Bytes` any `0x`-prefixed hex; a wrong length or an unknown field is a `400 malformed` naming the
field. The body must be `application/json`.

### `POST /v4/user-decrypt`

```json
{
  "attestationType": "eip712-unified-user-decrypt-v1",
  "payload": {
    "handles": [
      {
        "ctHandle": "0x0000000000000000000000000000000000000000000000000000000000010401",
        "contractAddress": "0x3333333333333333333333333333333333333333",
        "ownerAddress": "0x4444444444444444444444444444444444444444"
      }
    ],
    "userAddress": "0x5555555555555555555555555555555555555555",
    "allowedContracts": ["0x3333333333333333333333333333333333333333"],
    "requestValidity": { "startTimestamp": "1799999900", "durationSeconds": "600" },
    "publicKey": "0x20002000",
    "extraData": "0x0207000000000000000000000000000000000000000000000000000000000000010800000000000000000000000000000000000000000000000000000000000001"
  },
  "signature": "0x6666…66"
}
```

`startTimestamp` and `durationSeconds` are decimal strings (numbers are accepted too). `signature` is the user's
EIP-712 signature, empty on the ERC-1271 path; the connector verifies it, not the relayer.

Conversion to the connector request (`kms-connector-api::UserDecryptionRequest`): `attestationType` is checked
(section 3) and forwarded, it is part of the connector's `decryptionId`; `payload.handles[].ctHandle` →
`payload.handles[].handle`; the other handle fields, `userAddress`, `publicKey`, `allowedContracts`,
`requestValidity`, `extraData` and `signature` as they are. Nothing is dropped. The body has the connector's v1
envelope topology (`attestationType`, `payload`, `signature`) with the relayer's field names, so a relayer-only field
can be added later without touching the connector DTO.

### `POST /v4/public-decrypt`

```json
{ "ciphertextHandles": ["0x0000000000000000000000000000000000000000000000000000000000010401"], "extraData": "0x0207000000000000000000000000000000000000000000000000000000000000010800000000000000000000000000000000000000000000000000000000000001" }
```

Conversion: `ciphertextHandles` → `ctHandles`, `extraData` as it is.

## 3. Validation

Stateless, before any fan-out. The current relayer's rules, plus the connector's cheap handle rules (it would reject
these requests anyway; failing here saves n calls). Every failure is `400 malformed` with `"<field>: <issue>"`.

### 3.1 Rules

| field | rule |
|---|---|
| `attestationType` | `eip712-unified-user-decrypt-v1` |
| `payload.handles`, `ciphertextHandles` | non-empty; at most `http.max_handles_per_decryption` (default 50, the connector's `max_handles_per_decryption`); each handle's FHE type byte (index 30) is decryptable (`ebool`, `euint8..256`, `eaddress`); all handles carry one chain id (bytes 22..30, big-endian) that is in `supported_chain_ids`; total plaintext size at most 2048 bits |
| `payload.allowedContracts` | at most `http.max_allowed_contracts` (default 10, the connector's `max_allowed_contracts`) |
| `payload.publicKey` | non-empty |
| `payload.requestValidity` | `durationSeconds` at most one year (the connector's limit); `startTimestamp <= now` and `startTimestamp + durationSeconds > now` |
| `extraData`, `payload.extraData` | version `0x02` (v2), at least 65 bytes; see 3.2 |

### 3.2 extraData

The relayer treats `extraData` as opaque and checks only its envelope:

- **Version**: the first byte must be `0x02` (v2). v0 (empty or `0x00`) and v1 (`0x01` + context id) are no longer
  accepted, nor is any other version.
- **Length**: at least 65 bytes (version + 32-byte context id + 32-byte epoch id). Trailing bytes are allowed, as the
  connector allows them.
- **Content**: never read. The context id and epoch id are not decoded or checked (no type tag, no lookup): parsing
  and validating them is the role of the protocol (KMS connector, host contracts), not of the relayer.

A failure is `400 malformed` with `"extraData: must be 0x02 + context id + epoch id"` (`payload.extraData` for user
decrypt). A valid `extraData` is forwarded to the connectors as it is.

### 3.3 Not checked

Deliberately not checked here: the user's signature (ERC-1271, connector worker), the signature length, the ACL, the
ciphertext readiness and the content of `extraData` (connector worker; they surface as the dominant error, section 5).

## 4. Responses

Success is the current relayer's envelope, `200`:

```json
{ "status": "succeeded", "requestId": "019a…", "result": … }
```

`result` for user decrypt, every accepted share in acceptance order (hex without `0x`, `extraData` with `0x`):

```json
{ "result": [ { "payload": "a1b2…", "signature": "1a2b…", "extraData": "0x02…" } ] }
```

`result` for public decrypt, the agreeing group's plaintexts and signatures:

```json
{ "decryptedValue": "0000…002a", "signatures": ["1a2b…", "…"], "extraData": "0x02…" }
```

Every response, success or error, carries `x-request-id: <requestId>`.

## 5. Errors

One body for every error:

```json
{ "code": "acl_denied", "message": "KMS threshold not reached (2 of 9 responses, 0 rejected)", "requestId": "019a…" }
```

**This table is updated with every change to the mapping** (rule in `CLAUDE.md`).

| status | code | when |
|---|---|---|
| 400 | `malformed` | unreadable or oversized body, a body not received within `http.body_read_timeout`, invalid JSON, unknown field, wrong content type, or a validation rule; the message names the field |
| 404 | `not_found` | no such route |
| 405 | `method_not_allowed` | the route exists, the method does not |
| 500 | `internal` | a bug: `AggregationError::Internal`, or a panic while handling the request (message `handler panicked`) |
| 504 | `timeout` | the deadline (`call.timeout`) passed with fewer than `threshold` responses (`AggregationError::Timeout`); the message carries the counts, the rejected count and the most frequent connector error |
| connector's | connector's | `AggregationError::ThresholdNotReached`: the most frequent connector error among the failed calls is answered with the connector's own status and code |
| 502 | `upstream_transient` | `ThresholdNotReached` with no connector error (only rejected or unreachable nodes) |

Connector codes that can come back this way, with the status the connector assigns them (`kms-connector-api`):
`malformed` 400, `unsupported_attestation_type` 400 (only reachable if the relayer and connector crates disagree:
the relayer rejects other schemes before the fan-out with `400 malformed "attestationType: must be …"`),
`sender_authentication_failed` 401 (the node rejects the relayer's `FhevmSig`: key not allowed or wrong audience),
`auth_expired` 401 (the node found the `FhevmSig` expired, even after re-signing: clock drift), `acl_denied` 403,
`user_signature_rejected` 403, `user_decryption_expired` 403 (the signed validity interval ended; not retried), `ciphertext_not_found` 404 (the ciphertext is not committed yet),
`kms_context_destroyed` 410, `kms_context_invalid` 412, `unprocessable` 422, `rate_limited` 429,
`copro_consensus_failed` 502, `upstream_transient` 502, `overloaded` 503, `timeout` 504, `unknown` 500.
The message always carries the counts: `KMS threshold not reached (<counted> of <threshold> responses, <rejected> rejected)`,
or for a timeout `KMS nodes did not answer in time (<counted> of <threshold> responses, <rejected> rejected, most
frequent error: <code or none>)`.

**First version, to be reworked.** There is no `retryable` indication in the body yet; forwarding the connector's
status for the dominant error is a shortcut that an explicit mapper (one table, one place) should replace; and the
dominant-error algorithm (most frequent code) may need a priority order or a per-code policy.

## 6. Request id and request hash

Each HTTP request gets a random UUIDv4 `requestId`, minted by `Summary::new` in the handler: in the body, in
`x-request-id`, in the summary line (section 9), and passed to the aggregator, which puts it on its lines through the
`aggregation` span. Nothing keys on it: there is no map of requests. It is random on purpose: it reveals nothing about
the server, such as arrival time.

The request hash is the connector's `decryptionId`, the EIP-712 signing hash of the connector request: computed with
the connector's crate from the request the relayer forwards (never from the body it received), so it is exactly the id
every KMS node derives. Two identical payloads get two request ids and one hash; both are on the request's summary
line.

## 7. Shutdown

`main` cancels the process token on SIGINT or SIGTERM: `/healthz` answers 503 so Kubernetes stops routing to the
pod, and axum stops accepting connections and drains the in-flight requests. A running aggregation is not cut: it ends
by its own threshold, fail fast or deadline, and its request gets its normal answer. The drain is bounded by
`body_read_timeout + call.timeout`. A client that disconnects drops its handler, which drops its aggregation and aborts
the node calls. The process exits 0 only after a clean drain; a
server or task error, before or after the signal, is logged and exits 1.

**Connections.** The body read and its JSON parse are bounded in total by `http.body_read_timeout`, counted from the
handler's start: after its headers, a request lasts at most `body_read_timeout + call.timeout` (10 s + 5 s by
default). Header reading and idle keep-alive connections are not bounded by the relayer: axum's `serve` sets no hyper
timer and does not expose its connection builder. In production the pods sit behind a Kubernetes ingress or load
balancer, which enforces header, body and idle timeouts: deployments rely on it.

## 8. Probes

`GET /liveness` → 200 `{"status":"alive"}` always. `GET /healthz` → 200 `{"status":"ready"}`, or 503
`{"status":"shutting_down"}` after the token is cancelled. Use `/liveness` for the liveness probe and `/healthz` for
readiness.

## 9. Logging and metrics

The rules are in `OBSERVABILITY.md`; the generated references are `monitoring/summary_log.md`, `metrics.md` and
`labels.md`. What the endpoint adds:

- **One summary line per request**, message `request completed`, emitted by the `record` middleware in `mod.rs` when
  the response is sent, whatever the outcome (probes excepted). Every other line about the request, from any module,
  carries `request_id` (and `kms_node` for one node) plus fields of its own. The fields are defined once, in
  `summary::FIELDS`.
- **`Summary`** (`summary.rs`) is the request's state: the handler creates it (`Summary::new(flow)`), fills the public
  request fields and `decryption_id` as it goes (`received`, `forwarded`), the aggregator fills its `tally`, and the
  success reply or the error carries it in the response extensions. The middleware logs it and counts the request
  (`relayer_http_requests_total`, `relayer_http_request_duration_seconds`) from that one value, so the line and the
  metrics cannot disagree.
- **`reason`** says why a request did not succeed: the validation field and issue, the aggregation verdict, `client
  closed`, `handler panicked`, or for an unreadable body one fixed text, the same one the client gets
  as the `message` of its `400 malformed`, built by `flows::body_error` and never from the parser's own. A wrong
  content type is refused before the body is read, and a body over `http.max_body_bytes` is `body exceeds <limit>
  bytes`: `body_len` is 0 for both.
- **Dropped or panicked handler**: if the handler's future is dropped before the answer, the `Guard` holding the
  `Summary` completes the request on drop: `outcome` `cancelled`, `status` `none`, `code` `client_closed`, with whatever
  was known by then; the node calls dropped with the handler are
  in `cancelled_kms_nodes`, each with a `call cut` WARN. If the handler panics, the unwinding drops the same `Guard`:
  `failed`, `status` `500`, `code` `internal`, `reason` `handler panicked`.
- **Panic answer**: `CatchPanicLayer` (tower-http) answers a panic with `500` and the `internal` error body (message
  `handler panicked`, the request's `requestId`); the payload is never read. `record` runs each request with a
  task-local slot that the panic fills while it unwinds: the panic hook (`logging::init_panic_hook`) puts the location
  there, the `Guard` the id of the request it logged. The answer reuses that id and carries no `Summary`, so the
  request is logged once; a panic outside a guarded handler (a fallback, a probe) is logged from the answer's
  `Summary` instead (`flow` `none`). Either way one ERROR line, `internal error`, with the `location`.
- **`internal`** (a panic, or `AggregationError::Internal`) is a relayer defect: the summary line stays INFO, and one
  more ERROR line, `internal error`, carries the `request_id` and the `reason`.
- `status` is text (`"200"`, or `none`) so that the field has one type; `contract_addresses` is the handles' contracts,
  `allowed_contracts` the request's `allowedContracts`.
- **Invalid request**: the list fields show at most validation's limits (`http.max_handles_per_decryption` handles
  and contracts, `http.max_allowed_contracts` `allowedContracts`), then `+N more`: a valid request is listed whole, and a request that failed validation, bounded
  only by the body limit, is cut. A DEBUG line `request invalid` carries the `field`, the `issue` and the offending
  `value` (the handle, or a count), never sent to the client.

JSON output is the default (`log.format`); `RUST_LOG` sets the level, default `warn,relayer_http=debug`. One successful
user decrypt, fields abridged:

```json
{"level":"INFO","fields":{"message":"request completed","request_id":"b3c…","flow":"user_decrypt","outcome":"succeeded","status":"200","code":"ok","elapsed_ms":412,"handles":"0x…0401","chain_id":"1","decryption_id":"0x…","counted":13,"threshold":9,"failed_kms_nodes":"none","reason":"none",…}}
```

## 10. Testing

Unit tests next to every function (types, validation rules, conversions, error mapping) and integration tests in
`mod.rs` that drive the router in memory with the mock connector: the success envelopes, every 400 case, 404, 405,
the oversized body, the dominant-error mapping, shutdown, the probes, and the log lines of three requests (success,
validation failure, unparsable body, failure, timeout, router fallbacks) captured with `logging::capture_runs` (the
production JSON, thread-local): exactly one `request completed` per request, INFO, with exactly the defined fields and
the expected values; and no marker carried by a request body, signature or header, nor any outgoing header or URL,
appears in any line at DEBUG. `cargo nextest run -p relayer-http endpoint`.

Manual smoke, with the API key env vars set:

```sh
cargo run -p relayer-http -- config/config.yaml
# 400 malformed, "ciphertextHandles: must not be empty"
curl -s -i localhost:8080/v4/public-decrypt -H 'content-type: application/json' \
  -d '{"ciphertextHandles":[],"extraData":"0x0207000000000000000000000000000000000000000000000000000000000000010800000000000000000000000000000000000000000000000000000000000001"}'
curl -s localhost:8080/healthz                         # {"status":"ready"}
```

## 11. Next stages

- A `retryable` indication in the error body, once the retry policy is decided; the error-layer rework (section 5).
- `GET /version`. (`/metrics` is served on its own port: `metrics.endpoint`, default `0.0.0.0:9898`.)
- A resource-pressure endpoint and HPA signals (in-flight requests, aggregator permits in use) rather than a
  hardcoded in-flight cap; `503 overloaded` only if a cap is ever wanted. `/healthz` (or that metric) should then
  reflect call-permit saturation: today a pod whose permits are all taken stays `ready`. Checked against the
  connector and KMS capacity before it may take a pod out of the Service.
- Request caching keyed by the request hash (`decryption_id`), HPA-compatible through Redis: the same payload
  waits on the same aggregation, a completed request is answered from the store. It needs the accepted responses to
  be registered first (the aggregator cancels late shares today).
- Payload simplification or additions once the SDK's needs on the sync path are settled.
