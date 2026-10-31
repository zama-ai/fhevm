# PRF service

`prf-service` sends PRF outputs to the KMS Connectors for pad decryption (RFC-030). RFC-038 defines
the endpoint. Only `coprocessor-proxy` calls `prf-service`. The proxy checks the `FhevmSig` signature
and sends the signer in `X-Fhevm-Signer`. `prf-service` trusts this header: the NetworkPolicy of the
proxy chart lets only the proxy reach port 8090.

## Status

- The `pad-worker` (#2193) does not exist yet. So no row completes, and each request waits until the
  Connector or the proxy stops it.

## Request

1. Read the signer from `X-Fhevm-Signer`.
2. Check the path and the `prf` instance.
3. Take a place in `InFlight`: one request for each `(connector, label)`, and a limit for each
   connector.
4. Read the row of `(prf, label, padLength)`. With no row, add it within the budget of the
   connector.
5. Wait for the row (`LISTEN pad_ready`, and a poll), then answer from it.

The work continues when the Connector disconnects: the row owns it. A stop of `prf-service` does not
wait for held requests. The Connector retries, and the retry joins the same row.

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
| `401 auth_missing`, `401 auth_malformed` | No `X-Fhevm-Signer`, or not one address |
| `404 unknown_prf` | The coprocessor does not know this `prf` instance |
| `405` | A method other than `GET` or `HEAD` |
| `429 concurrency_limited` | A limit of the connector. `Retry-After: 1` |
| `500 prf_evaluation_failed` | The evaluation of the row failed |
| `503 unavailable` | A database error |

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
add rows, nothing more. The migration job runs it on each deploy. The operator creates the role, and
sets `prfService.databaseUser` and `prfService.databasePassword` in the chart. Without them,
`prf-service` uses the user of `commonConfig`.

`--database-url` (or `DATABASE_URL`) selects the database. Readiness checks the database. If the
`pad_ready` listener cannot connect again, the process exits.

## Limits

| Limit | Where | Applies to |
|---|---|---|
| Pending pad bits of each connector (`PRF_MAX_CONCURRENT_PER_CONNECTOR`) | Postgres | All pods |
| One row for each output | Postgres | All pods |
| Two admissions in progress of each connector | Memory | Each pod |
| One request for each `(connector, label)` | Memory | Each pod |
| Requests in progress of each connector (`PRF_MAX_IN_FLIGHT_PER_CONNECTOR`) | Memory | Each pod |

Do not autoscale `prf-service`. With more pods, one connector can have more requests in progress.

## Tests

```sh
cargo test -p prf-service
```

The database tests start a Postgres container, so Docker must run. After you change a `sqlx::query!`,
update `coprocessor/fhevm-engine/.sqlx` against a migrated database.
