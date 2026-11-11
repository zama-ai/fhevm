<!-- Generated from the code: do not edit. Regenerate with `make -C relayer-http/dev references` (UPDATE_REFERENCES=1 on the test `references_are_current`). -->

# Summary line

One INFO line per request, message `request completed`, when the response is sent, whatever the outcome (the probes `/liveness` and `/healthz` are not requests). It is the only line carrying these fields. The JSON output nests them under `fields`.

| Field | Meaning |
| --- | --- |
| `request_id` | random UUIDv4 of the request; the `x-request-id` header and the `requestId` of the answer |
| `flow` | `user_decrypt`, `public_decrypt`, or `none` when answered before a flow is known |
| `outcome` | `succeeded` (2xx), `rejected` (4xx), `failed` (5xx, a panic included) or `cancelled` (the client closed the connection before the answer) |
| `status` | HTTP status of the answer, as text; `none` when the client closed the connection first; `500` for a panic |
| `code` | `ok`, the API error code of the answer, or `client_closed`; `internal` is a relayer defect (an internal error or a panic) |
| `elapsed_ms` | milliseconds from receipt to the answer |
| `handles` | ciphertext handles, comma-separated; at most `http.max_handles_per_decryption` (the validation limit), then `+N more`, so only a request that failed validation is cut; `none` if the body was unreadable |
| `contract_addresses` | contract addresses of the handles, comma-separated, each once, cut like `handles`; `none` if the body was unreadable or the flow has none (public decrypt) |
| `allowed_contracts` | the `allowedContracts` of the request (user decrypt), comma-separated, at most `http.max_allowed_contracts` (the validation limit), then `+N more`; `none` otherwise |
| `user_address` | the user's address (user decrypt); `none` otherwise |
| `chain_id` | chain id the handles carry; `none` if unreadable |
| `body_len` | bytes of the request body; 0 when it was not received (unreadable, over the size limit, or refused for its content type) |
| `decryption_id` | EIP-712 signing hash of the connector request, the id KMS operators know; `none` if the request was never forwarded |
| `counted` | KMS responses that counted toward the threshold; 0 if the request was never forwarded |
| `threshold` | KMS responses the flow needs; 0 if the request was never forwarded |
| `threshold_reached_ms` | milliseconds from receipt until the counted responses reached the threshold, as text; the answer waited for the remaining nodes after it; `none` if the threshold was never reached or the request was never forwarded |
| `failed_kms_nodes` | KMS nodes whose call failed, comma-separated, including a call that failed to sign (the `signing failed` line says so); `none` if none |
| `rejected_kms_nodes` | KMS nodes whose response a check refused, comma-separated; `none` if none |
| `cancelled_kms_nodes` | KMS nodes whose call was cut (deadline, fail fast, client hangup), comma-separated; `none` if none |
| `reason` | why the request did not succeed: the validation field and issue, the body error with its field path, the aggregation verdict, `client closed` or `handler panicked`; `none` on success |

Every other line about a request carries its `request_id` and, when it concerns one KMS node, `kms_node`, plus fields specific to the event it reports (an attempt number, an `error`, a `cause`). In the JSON output the lines the endpoint emits (`internal error`, `call cut` after a hangup) have `request_id` under `fields`; the lines of the aggregator, emitted inside its `aggregation` span, have it under `span.request_id` instead, and `kms_node` under `fields`. To find every line of a request, search its `request_id` as text.
