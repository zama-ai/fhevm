# Dual-relayer dispatch

Status: implemented.

When `--relayer-b-url` is configured, one scheduler tick produces a paired record
for target A (primary) and target B (candidate). The pair shares one workload
item and correlation id, but transport semantics depend on the driver.

## Driver semantics

| Flow | A/B behavior |
| --- | --- |
| `input-proof` | The same prepared raw body is submitted independently to each target. |
| `public-decrypt` | Each target runs its own SDK public-decrypt journey over the same claimed handle combination. |
| `user-decrypt` | Each target runs its own SDK keypair, permit, submit, poll, verification, and reconstruction journey over the same pooled handle. |
| `delegated-user-decrypt` | Same independent SDK journey as user-decrypt, signed by the recorded delegate for the pooled owner. |

For SDK-driven flows, A and B are **independent target legs**, not identical
raw dispatches. In particular, user-decrypt generates a fresh transport key
and signed permit in each leg, so the relayer request hashes can differ. The
comparison answers “how did each target serve the same logical workload item?”
rather than “how did each target serve the same request bytes?”.

This distinction is deliberate. `@fhevm/sdk@0.13.2-1` publicly supports
the complete user-decrypt journey, including cancellation, progress, KMS share
verification, and clear-value reconstruction. It does not expose a supported
raw request builder. Recreating private wire structures in the load tool would
couple correctness to SDK internals and relayer schemas.

## Pairing invariant

The executor claims a pool item once per scheduler tick. Both target legs use
that item and the same `x-request-id` correlation value. A/B fields are stored
on one `RequestRecord`, but each leg has its own submit metadata, job id, poll
count, outcome, latency, and verification result.

The run `AbortSignal` is passed into both SDK legs. An aborted leg is recorded
as `aborted` with `client_aborted`; an SDK timeout is `timed_out` with
`client_poll_deadline_exceeded`. Relayer terminal failures remain `failed`,
pre-job failures are `submit_failed`, and plaintext mismatches are
`verify_failed`.

## Operational implications

- User/delegated handles are reusable, but every leg creates fresh permit and
  transport material.
- Do not describe user/delegated A/B results as byte-identical or deduplicated.
- If target B also receives mirrored traffic from A, the candidate can perform
  additional work because independently generated SDK requests do not share a
  request hash. Disable mirroring for measured load-test traffic when symmetric
  incremental load matters.
- Pool preflight rejects mismatched network, host chain, contract, account
  derivation, delegate identity, public handles in private pools, malformed
  expected plaintexts, or insufficient per-owner ACL delegation before load
  starts. Delegation state is checked both in pool metadata and on-chain.
- Primary and candidate must be two distinct targets, but they may share an
  origin: a path-routed deployment (one gateway host serving A and B under
  different base paths or API prefixes, e.g. `--relayer-api-prefix /v1` vs
  `--relayer-b-api-prefix /v2`) is supported. Only a fully identical target —
  same normalized URL *and* same effective API prefix — is rejected.
- `--max-connections` bounds sockets **per relayer target**, so a paired A/B run
  can open up to twice that many connections in aggregate (one pool per target).
- The two targets should use equivalent ingress paths and authentication so
  latency deltas reflect relayer behavior rather than proxy differences.
