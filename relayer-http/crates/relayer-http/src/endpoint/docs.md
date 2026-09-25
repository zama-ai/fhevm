# endpoint

Minimal first version of the relayer's HTTP layer: two synchronous decryption routes and the Kubernetes probes, on
one port, with raw axum. Reading order: this file, `mod.rs`, `flows/user_decrypt.rs`, `flows/public_decrypt.rs`,
`validate.rs`, `error.rs`.

The routes are experimental (`/v4/exp/`): the request and response payloads may be simplified or extended in later
iterations, and the error mapping is a first version (section 5).

## 1. Routes

| method | path | purpose |
|---|---|---|
| `POST` | `/v4/exp/user-decrypt` | user decryption: the KMS nodes' signcrypted shares for the caller's public key |
| `POST` | `/v4/exp/public-decrypt` | public decryption: the plaintexts and the KMS signatures over them |
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

### `POST /v4/exp/user-decrypt`

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
    "extraData": "0x00"
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

### `POST /v4/exp/public-decrypt`

```json
{ "ciphertextHandles": ["0x0000000000000000000000000000000000000000000000000000000000010401"], "extraData": "0x00" }
```

Conversion: `ciphertextHandles` → `ctHandles`, `extraData` as it is.

## 3. Validation

Stateless, before any fan-out. The current relayer's rules, plus the connector's cheap handle rules (it would reject
these requests anyway; failing here saves n calls). Every failure is `400 malformed` with `"<field>: <issue>"`.

| field | rule |
|---|---|
| `attestationType` | `eip712-unified-user-decrypt-v1` |
| `payload.handles`, `ciphertextHandles` | non-empty; each handle's FHE type byte (index 30) is decryptable (`ebool`, `euint8..256`, `eaddress`); all handles carry one chain id (bytes 22..30, big-endian) that is in `supported_chain_ids`; total plaintext size at most 2048 bits |
| `payload.allowedContracts` | at most 10 |
| `payload.publicKey` | non-empty |
| `payload.requestValidity` | `startTimestamp <= now` and `startTimestamp + durationSeconds > now` |
| `extraData` | empty or `0x00` (v0), `0x01` + 32-byte context id (v1), `0x02` + context id + 32-byte epoch id (v2); trailing bytes allowed |

Deliberately not checked here: the user's signature (ERC-1271, connector worker), the signature length, the ACL and
the ciphertext readiness (connector worker; they surface as the dominant error, section 5).

## 4. Responses

Success is the current relayer's envelope, `200`:

```json
{ "status": "succeeded", "requestId": "019a…", "result": … }
```

`result` for user decrypt, every accepted share in acceptance order (hex without `0x`, `extraData` with `0x`):

```json
{ "result": [ { "payload": "a1b2…", "signature": "1a2b…", "extraData": "0x00" } ] }
```

`result` for public decrypt, the agreeing group's plaintexts and signatures:

```json
{ "decryptedValue": "0000…002a", "signatures": ["1a2b…", "…"], "extraData": "0x00" }
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
| 500 | `internal` | a bug (`AggregationError::Internal`) |
| 503 | `shutting_down` | the process received SIGTERM/SIGINT while this request was running (`AggregationError::Cancelled`) |
| 504 | `timeout` | the deadline (`call.timeout`) passed with fewer than `threshold` responses (`AggregationError::Timeout`); the message carries the counts, the rejected count and the most frequent connector error |
| connector's | connector's | `AggregationError::ThresholdNotReached`: the most frequent connector error among the failed calls is answered with the connector's own status and code |
| 502 | `upstream_transient` | `ThresholdNotReached` with no connector error (only rejected or unreachable nodes) |

Connector codes that can come back this way, with the status the connector assigns them (`kms-connector-api`):
`malformed` 400, `unsupported_attestation_type` 400 (only reachable if the relayer and connector crates disagree:
the relayer rejects other schemes before the fan-out with `400 malformed "attestationType: must be …"`),
`sender_authentication_failed` 401 (the relayer's API key is wrong for that node), `acl_denied` 403,
`user_signature_rejected` 403, `ciphertext_not_found` 404 (the ciphertext is not committed yet),
`kms_context_destroyed` 410, `kms_context_invalid` 412, `unprocessable` 422, `rate_limited` 429,
`copro_consensus_failed` 502, `upstream_transient` 502, `overloaded` 503, `timeout` 504, `unknown` 500.
The message always carries the counts: `KMS threshold not reached (<counted> of <threshold> responses, <rejected> rejected)`,
or for a timeout `KMS nodes did not answer in time (<counted> of <threshold> responses, <rejected> rejected, most
frequent error: <code or none>)`.

**First version, to be reworked.** There is no `retryable` indication in the body yet; forwarding the connector's
status for the dominant error is a shortcut that an explicit mapper (one table, one place) should replace; and the
dominant-error algorithm (most frequent code) may need a priority order or a per-code policy.

## 6. Request id and request hash

Each HTTP request gets a UUIDv7 `requestId`, minted by `Log::new` in the handler: in the body, in `x-request-id`,
on every log line, and passed to the aggregator, which uses it for its logs only. Nothing keys on it: there is no
map of requests.

The request hash is the connector's `decryptionId`: computed with the connector's crate from the connector request
the relayer forwards (never from the body it received), so it is exactly the id every KMS node derives. Two
identical payloads get two request ids and one hash; both are identifiers of the request's `Log` (section 9), so
every line of one request carries them once they are known.

## 7. Shutdown

`main` cancels the process token on SIGINT or SIGTERM: `/healthz` answers 503 so Kubernetes stops routing to the
pod, axum stops accepting connections and drains the in-flight requests, and every running aggregation ends with
`Cancelled` (503 `shutting_down`). The drain is bounded by `call.timeout`. A client that disconnects drops its
handler, which drops its aggregation and aborts the node calls. The process exits 0 only after a clean drain; a
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

## 9. Logging

Every line the endpoint writes about a request carries the same four identifiers, so a request can be followed
by any of them (handles are what the rest of the stack logs; the decryption id is what the KMS nodes log):

| field | value | known from |
|---|---|---|
| `request_id` | the relayer's UUIDv7 (section 6) | the first line |
| `flow` | `user_decrypt`, `public_decrypt`; `none` on the router fallbacks | the first line |
| `handles` | `[0x…, 0x…]`, the ciphertext handles of the request | the body is parsed; `none` before |
| `decryption_id` | `0x…`, the connector's content hash (section 6) | the connector request is built; `none` before |

They live in `Log` (`src/logging.rs`): the handler creates it (`Log::new("user_decrypt")`), fills `handles` and
`decryption_id` as it goes, and hands it to the reply or the error, which write the last line. One method per event:

| event | level | own fields | when |
|---|---|---|---|
| `request received` | info | — | the body is parsed (handles known) |
| `request body rejected` | info | `reason` | axum could not parse the body: invalid JSON, unknown field, wrong content type, oversized |
| `request validation failed` | info | `field`, `issue` | a rule of section 3 failed; the client's error, named precisely |
| `request forwarded` | info | — | the connector request is built and goes to the aggregator (decryption id known) |
| `request succeeded` | info | `status` | 200 answered |
| `request rejected` | info | `status`, `code`, `reason` | 4xx answered: the client's request is at fault |
| `request failed` | warn | `status`, `code` | 5xx answered: the relayer or the nodes are at fault |

Between `request forwarded` and the last line come the aggregator's lines (`aggregation started`, one line per node
outcome, `aggregation succeeded` / `aggregation failed`; see `kms_aggregator/docs.md` §10), whose span carries the
same `request_id`, `decryption_id` and `handles`. One successful user decrypt, JSON format, fields abridged:

```json
{"level":"INFO","fields":{"message":"request received","request_id":"019…","flow":"user_decrypt","handles":"[0x…0401]","decryption_id":"none"}}
{"level":"INFO","fields":{"message":"request forwarded","request_id":"019…","flow":"user_decrypt","handles":"[0x…0401]","decryption_id":"0x…"}}
{"level":"INFO","fields":{"message":"aggregation started","nodes":13,"threshold":9,"timeout_ms":10000},"span":{"request_id":"019…","decryption_id":"0x…","handles":"[0x…0401]","flow":"user_decrypt","name":"aggregation"}}
{"level":"INFO","fields":{"message":"aggregation succeeded","counted":9,…},"span":{…}}
{"level":"INFO","fields":{"message":"request succeeded","request_id":"019…","flow":"user_decrypt","handles":"[0x…0401]","decryption_id":"0x…","status":200}}
```

A validation failure is `request received`, `request validation failed` (`field`, `issue`), `request rejected`
(`status` 400, `code` `malformed`); an unparsable body is `request body rejected` then `request rejected`, both with
`handles` and `decryption_id` at `none`. Client-side outcomes are `info`, not `warn`: they are not relayer errors.

Adding a line is one call: `log!(debug, log, attempts = 2, "retrying")` prints the four identifiers then the
caller's fields; a recurring event becomes a method on `Log`. Adding an identifier is one field in `Log` and one
line in the `log!` macro. `Log` is meant to be handed to the aggregator in a later iteration, so that its lines and
the endpoint's share one shape. Bodies, keys, signatures and shares are never logged. JSON output is the default
(`log.format`); `RUST_LOG` sets the level, default `warn,relayer_http=info`.

## 10. Testing

Unit tests next to every function (types, validation rules, conversions, error mapping) and integration tests in
`mod.rs` that drive the router in memory with the mock connector: the success envelopes, every 400 case, 404, 405,
the oversized body, the dominant-error mapping, shutdown, the probes, and the log lines of three requests (success,
validation failure, unparsable body) captured through a thread-local JSON subscriber: every `request …` line has
the four identifiers, with the expected values at each step. `cargo test -p relayer-http endpoint`.

Manual smoke, with the API key env vars set:

```sh
cargo run -p relayer-http -- config/config.yaml
curl -s -i localhost:8080/v4/exp/public-decrypt -H 'content-type: application/json' \
  -d '{"ciphertextHandles":[],"extraData":"0x00"}'     # 400 malformed, "ciphertextHandles: must not be empty"
curl -s localhost:8080/healthz                         # {"status":"ready"}
```

## 11. Next stages

- A `retryable` indication in the error body, once the retry policy is decided; the error-layer rework (section 5).
- `GET /version`, `/metrics`.
- A resource-pressure endpoint and HPA signals (in-flight requests, aggregator permits in use) rather than a
  hardcoded in-flight cap; `503 overloaded` only if a cap is ever wanted. `/healthz` (or that metric) should then
  reflect call-permit saturation: today a pod whose permits are all taken stays `ready`. Checked against the
  connector and KMS capacity before it may take a pod out of the Service.
- Request caching keyed by the request hash (`decryption_id`), HPA-compatible through Redis: the same payload
  waits on the same aggregation, a completed request is answered from the store. It needs the accepted responses to
  be registered first (the aggregator cancels late shares today).
- Payload simplification or additions once the SDK's needs on the sync path are settled.
