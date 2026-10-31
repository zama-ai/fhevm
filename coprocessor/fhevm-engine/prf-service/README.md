# PRF service

`prf-service` sends PRF outputs to the KMS Connectors for pad decryption (RFC-030). RFC-038 defines
the endpoint. Only `coprocessor-proxy` calls `prf-service`. The proxy checks the `FhevmSig` signature.

## Status

- The endpoint sends a **mock** output and an **unsigned** attestation.
- Admission (`pads.rs`), waiting (`waiter.rs`) and the request limits (`in_flight.rs`) have tests, but
  the handler does not use them yet.

## Endpoint

```
GET  /v1/prf/{prf}/labels/{label}/{padLength}
HEAD /v1/prf/{prf}/labels/{label}/{padLength}
```

- `prf`: decimal, 0 to 65535, no leading zeros.
- `label`: 64 lowercase hex characters, no `0x`.
- `padLength`: decimal, 1 to 2048, no leading zeros.

| Answer | When |
|---|---|
| `200` | The ct128 output as the body (`GET` only), and the attestation in `X-Prf-Attestation` |
| `400 non_canonical_path` | A parameter in a different encoding, or a query string |
| `404 unknown_prf` | The coprocessor does not know this `prf` instance |
| `405` | A method other than `GET` or `HEAD` |

Error bodies are JSON `{code, message, retryable}`, with `Cache-Control: no-store`.

## Ports

| Port | Flag | Use |
|---|---|---|
| 8090 | `--http-endpoint` (`PRF_HTTP_ENDPOINT`) | The endpoint, HTTP/1.1. Only the proxy calls it |
| 8080 | `--health-check-port` | `/healthz`, `/liveness` and `/version` for Kubernetes |
| 9100 | `--metrics-addr` | `coprocessor_prf_service_requests_total{method, status}` |

## Database

The table `pads` has one row for each output `(prf_instance, label, pad_length)`. `prf-service` adds
and reads rows. The `pad-worker` computes the output and writes it to the row.

`db-migration/db-scripts/grant_pad_roles.sql` gives the role `prf_service` its rights: read `pads` and
add rows, nothing more. The migration job runs it on each deploy. The operator creates the role.

## Limits

| Limit | Where | Applies to |
|---|---|---|
| Pending pad bits of each connector (the budget) | Postgres | All pods |
| One row for each output | Postgres | All pods |
| Admissions in progress of each connector (`max_adds_per_connector`) | Memory | Each pod |
| One request for each `(connector, label)` | Memory | Each pod |
| Requests in progress of each connector (`max_per_connector` of `InFlight`) | Memory | Each pod |

Do not autoscale `prf-service`. With more pods, one connector can have more requests in progress.

## Tests

```sh
cargo test -p prf-service
```

The database tests start a Postgres container, so Docker must run. After you change a `sqlx::query!`,
update `coprocessor/fhevm-engine/.sqlx` against a migrated database.
