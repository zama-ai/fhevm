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

Configuration (`http:` in `config/config.yaml`): `endpoint` (bind address), `max_body_bytes` (default 2 MiB),
`supported_chain_ids` (host chain ids a handle may carry). One port for everything; no version endpoint yet.

## 2. Request payloads

The shapes are the current relayer's, so the SDK keeps working. Hex fields are typed: a handle is 32 bytes, an
address 20 bytes, `Bytes` any `0x`-prefixed hex; a wrong length or an unknown field is a `400 malformed` naming the
field. The body must be `application/json`.

### `POST /v4/exp/user-decrypt`

```json
{
  "attestationType": "eip712-unified-user-decrypt-v1",
  "attestedPayload": {
    "version": "2.0",
    "type": "user_decryption",
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

Conversion to the connector request (`kms-connector-api::UserDecryptionRequest`): `handles[].ctHandle` →
`handles[].handle`, the other handle fields, `userAddress`, `publicKey`, `allowedContracts`, `requestValidity`,
`signature` and `extraData` as they are. The envelope fields (`attestationType`, `version`, `type`) are checked, then
dropped.

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
| `attestedPayload.version`, `.type` | `2.0`, `user_decryption` |
| `attestedPayload.handles`, `ciphertextHandles` | non-empty; each handle's FHE type byte (index 30) is decryptable (`ebool`, `euint8..256`, `eaddress`); all handles carry one chain id (bytes 22..30, big-endian) that is in `supported_chain_ids`; total plaintext size at most 2048 bits |
| `attestedPayload.allowedContracts` | at most 10 |
| `attestedPayload.publicKey` | non-empty |
| `attestedPayload.requestValidity` | `startTimestamp <= now` and `startTimestamp + durationSeconds > now` |
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
| 400 | `malformed` | unreadable or oversized body, invalid JSON, unknown field, wrong content type, or a validation rule; the message names the field |
| 404 | `not_found` | no such route |
| 405 | `method_not_allowed` | the route exists, the method does not |
| 500 | `internal` | a bug (`AggregationError::Internal`) |
| 503 | `shutting_down` | the process received SIGTERM/SIGINT while this request was running (`AggregationError::Cancelled`) |
| 504 | `timeout` | the deadline (`call.timeout`) passed with fewer than `threshold` responses (`AggregationError::Timeout`) |
| connector's | connector's | `AggregationError::ThresholdNotReached`: the most frequent connector error among the failed calls is answered with the connector's own status and code |
| 502 | `upstream_transient` | `ThresholdNotReached` with no connector error (only rejected or unreachable nodes) |

Connector codes that can come back this way, with the status the connector assigns them (`kms-connector-api`):
`malformed` 400, `sender_authentication_failed` 401 (the relayer's API key is wrong for that node),
`acl_denied` 403, `user_signature_rejected` 403, `ciphertext_not_found` 404 (the ciphertext is not committed yet),
`kms_context_destroyed` 410, `kms_context_invalid` 412, `unprocessable` 422, `rate_limited` 429,
`copro_consensus_failed` 502, `upstream_transient` 502, `overloaded` 503, `timeout` 504, `unknown` 500.
The message always carries the counts: `KMS threshold not reached (<counted> of <threshold> responses, <rejected> rejected)`.

**First version, to be reworked.** There is no `retryable` indication in the body yet; forwarding the connector's
status for the dominant error is a shortcut that an explicit mapper (one table, one place) should replace; and the
dominant-error algorithm (most frequent code) may need a priority order or a per-code policy.

## 6. Request id and request hash

Each HTTP request gets a UUIDv7 `requestId`: in the body, in `x-request-id`, on every log line, and passed to the
aggregator, which uses it for its logs only. Nothing keys on it: there is no map of requests.

The request hash is the connector's `decryptionId`: the aggregator computes it, with the connector's crate, from
the connector request it forwards (never from the envelope the relayer received), so it is exactly the id every KMS
node derives. Two identical payloads get two request ids and one hash; both appear on the same log lines
(`request_id`, `decryption_id`).

## 7. Shutdown

`main` cancels the process token on SIGINT or SIGTERM: `/healthz` answers 503 so Kubernetes stops routing to the
pod, axum stops accepting connections and drains the in-flight requests, and every running aggregation ends with
`Cancelled` (503 `shutting_down`). The drain is bounded by `call.timeout`. A client that disconnects drops its
handler, which drops its aggregation and aborts the node calls.

## 8. Probes

`GET /liveness` → 200 `{"status":"alive"}` always. `GET /healthz` → 200 `{"status":"ready"}`, or 503
`{"status":"shutting_down"}` after the token is cancelled. Use `/liveness` for the liveness probe and `/healthz` for
readiness.

## 9. Logging

Per request: `user decrypt request` / `public decrypt request` (info: request_id, decryption_id, handles) once the
body is validated, then the aggregator's lines (its span carries `request_id`, `decryption_id`, `handles`, and
`node` on every per-node line), then `request succeeded` (info: request_id, status) or `request rejected` (info,
4xx: request_id, status, code, message) / `request failed` (warn, 5xx: request_id, status, code). Bodies, keys,
signatures and shares are never logged. JSON output is the default (`log.format`); `RUST_LOG` sets the level.

## 10. Testing

Unit tests next to every function (types, validation rules, conversions, error mapping) and integration tests in
`mod.rs` that drive the router in memory with the mock connector: the success envelopes, every 400 case, 404, 405,
the oversized body, the dominant-error mapping, shutdown, the probes. `cargo test -p relayer-http endpoint`.

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
  hardcoded in-flight cap; `503 overloaded` only if a cap is ever wanted.
- Request caching keyed by the request hash (`decryption_id`), HPA-compatible through Redis: the same payload
  waits on the same aggregation, a completed request is answered from the store. It needs the accepted responses to
  be registered first (the aggregator cancels late shares today).
- Payload simplification or additions once the SDK's needs on the sync path are settled.
