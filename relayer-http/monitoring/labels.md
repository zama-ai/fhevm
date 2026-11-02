<!-- Generated from the code: do not edit. Regenerate with `make -C relayer-http/dev references` (UPDATE_REFERENCES=1 on the test `references_are_current`). -->

# Labels

Label values come from enums or the configuration, never from request data.

| Label | Meaning | Values | Bound |
| --- | --- | --- | --- |
| `flow` | the decryption flow; `none` for requests answered before a flow is known | `user_decrypt`, `public_decrypt`, `pad_decrypt`, `none` | 4 |
| `outcome` | how a request ended: by status class, or `cancelled` when the client closed the connection first | `succeeded`, `rejected`, `failed`, `cancelled` | 4 |
| `code` | `ok`, an API error code of a request, or the connector error code of a failed KMS node attempt (`cancelled` for an attempt cut in flight, `panic` for a node task that panicked) | `ok`, `cancelled`, `client_closed`, `not_found`, `method_not_allowed`, `internal`, `panic`, `malformed`, `unsupported_attestation_type`, `sender_authentication_failed`, `auth_expired`, `rate_limited`, `overloaded`, `acl_denied`, `user_signature_rejected`, `user_decryption_expired`, `ciphertext_not_found`, `copro_consensus_failed`, `kms_context_invalid`, `kms_context_destroyed`, `unprocessable`, `upstream_transient`, `timeout`, `unknown` | 24 |
| `kms_node` | a KMS node, by its name in the configuration | names from `kms_aggregator.endpoints[].name` | 13 |
