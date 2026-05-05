# HTTP API Design

High-level description of the V2 HTTP API contract.
For the generated spec see [openapi.yml](../openapi.yml).

---

## Async POST + GET pattern

Every operation follows a two-step flow:

1. **POST** submits the request — returns **202 Accepted** with a `jobId` (UUID) and a `Retry-After` header
2. **GET** waits until `Retry-After` polls by `jobId` — returns **202** (still processing), **200** (completed), or an error code

Read-only endpoints (`GET /v2/keyurl`, health checks) respond synchronously.

## Uniform response envelope

All V2 responses share a single format:

```json
{
  "status": "queued | succeeded | failed",
  "requestId": "<uuid>",
  "result":    { /* endpoint-specific */ } | null,
  "error":     { /* error object */     } | null
}
```

- `status` is always present — the outcome can be determined from the body alone, independent of the HTTP code
- `result` and `error` are mutually exclusive; the unused field is `null`

## Structured errors with scoped labels

Error responses carry a machine-readable `label` and a human-readable `message`:

```json
{
  "label":   "validation_failed",
  "message": "Validation failed for 2 fields in the request",
  "details": [
    { "field": "contractAddress", "issue": "Must be 0x + 40 hex chars" }
  ]
}
```

- `details` array is present only for `validation_failed` and `missing_fields`
- Labels are drawn from a fixed canonical set (16 total), but **not every label applies to every endpoint or HTTP code** — they are scoped per endpoint
- See [../openapi.yml](../openapi.yml) for the full per-endpoint matrix

## HTTP status code semantics

| Code  | Meaning                                                              |
|-------|----------------------------------------------------------------------|
| `200` | Completed (GET success, keyurl)                                      |
| `202` | Accepted / still processing                                          |
| `400` | Client error (bad JSON, missing fields, validation, unsupported chain, ACL) |
| `404` | Job ID not found                                                     |
| `429` | Rate limited (queue full) — includes `Retry-After`                   |
| `500` | Internal error                                                       |
| `503` | Unavailable (timeout, paused contract, insufficient balance, gateway unreachable) |

## Request deduplication

Requests are SHA-256 hashed into an internal job ID (`int_job_id`). The database enforces uniqueness on active requests via a partial unique index.

- **Identical concurrent POSTs** return the same `ext_job_id` — no duplicate processing
- **Duplicate of a completed request** returns the stored result immediately
- See [ID.md](ID.md) for full identifier semantics

## Three IDs

| ID             | Exposed | Format       | Purpose                         |
|----------------|---------|-------------- |---------------------------------|
| `int_job_id`   | No      | SHA-256 hash | Content-based deduplication     |
| `ext_job_id`   | Yes     | UUID v4      | User facing, references an operation across requests. |
| `requestId`    | Yes     | UUID v7      | Per-HTTP-call logging/correlation |

`ext_job_id` is stable across retries of the same content; `requestId` is fresh on every HTTP call and not stored.
See [ID.md](ID.md) for detailed relationships.

## Dynamic Retry-After

Both POST (202) and GET (202) responses include a `Retry-After` header computed from real-time queue depth and drain rate — not a fixed value.

- Format: relative seconds for both 429 and 202
- POST: estimated from queue depth at submission time
- GET: estimated from the request's current position and processing stage
- See [dynamic-retry-after-design.md](dynamic-retry-after-design.md) for formulas and examples

## Business outcomes vs errors

A completed request can carry a negative business result that is **not** an error.

- Input proof `accepted: false` returns **200 OK** with `status: succeeded`
- The error envelope is reserved for actual failures (client mistakes, server faults, unavailability)

## Early validation before queuing

Chain ID support and host ACL checks run **before** the request enters any queue, so invalid requests don't consume queue capacity. These return 400 immediately.

## Request lifecycle

```
POST /v2/{endpoint}
  ├─ Parse + validate ──→ 400 (reject early)
  ├─ Dedup check
  │   ├─ Duplicate completed ──→ 200 (return stored result)
  │   └─ Duplicate processing ──→ 202 (reuse existing job_id)
  └─ New request ──→ 202 (queue + return job_id)

GET /v2/{endpoint}/{job_id}
  ├─ Not found ──→ 404
  ├─ Still processing ──→ 202 + Retry-After
  ├─ Completed ──→ 200 + result
  └─ Failed ──→ 4xx/5xx + error label
```

For internal status transitions (`queued → processing → tx_in_flight → …`), see [status-transitions.md](status-transitions.md).

## V2 endpoints

| Method | Path                                 | Async | Notes                          |
|--------|--------------------------------------|-------|--------------------------------|
| POST   | `/v2/public-decrypt`                 | Yes   | Chain ID validated early       |
| GET    | `/v2/public-decrypt/{job_id}`        | Yes   | + host ACL + revert classification |
| POST   | `/v2/user-decrypt`                   | Yes   | Chain ID validated early       |
| GET    | `/v2/user-decrypt/{job_id}`          | Yes   | Share threshold required       |
| POST   | `/v2/delegated-user-decrypt`         | Yes   | Chain ID validated early       |
| GET    | `/v2/delegated-user-decrypt/{job_id}`| Yes   | Same GET handler as user-decrypt |
| POST   | `/v2/input-proof`                    | Yes   | No chain ID / ACL validation   |
| GET    | `/v2/input-proof/{job_id}`           | Yes   | `accepted: false` is 200, not error |
| GET    | `/v2/keyurl`                         | No    | Read-only; waits up to 5s on first call |

## Operational endpoints

| Path         | Purpose                                  |
|--------------|------------------------------------------|
| `/liveness`  | Always 200 — for k8s liveness probe      |
| `/healthz`   | 200/503 — dependency health check        |
| `/version`   | Build version and git SHA                 |
| `/metrics`   | Prometheus metrics                        |

## Related docs

| Document                                                         | Covers                                        |
|------------------------------------------------------------------|-----------------------------------------------|
| [../openapi.yml](../openapi.yml) | Per-endpoint HTTP codes, labels, revert mapping |
| [ID.md](ID.md)                                                   | All identifier types and relationships        |
| [status-transitions.md](status-transitions.md)                   | Internal request state machine per endpoint   |
| [dynamic-retry-after-design.md](dynamic-retry-after-design.md)   | Retry-After computation formulas and examples |
