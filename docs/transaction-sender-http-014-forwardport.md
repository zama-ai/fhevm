# Transaction sender HTTP forward-port to release 0.14

Branch: `antoniu/forwardport/txn-sender-https-0.14`.
Base: `origin/release/0.14.x` at `d29d8f1d2`.
Source: the eleven commits on `antoniu/hotfix/txn-sender-https-baseline`
through `d4087082d`, after `f6da30cce`.

The port retains the source PR's HTTP transport, proof retry preservation,
fair scheduling, credential-safe diagnostics, and validation harnesses.
The coprocessor chart advances from **0.13.9 to 0.13.10**.

## Release-specific adaptations

- Preserve 0.14's rejected-proof retention. The selection query still excludes
  rejected proofs whose handles have already been cleared; retry deferral and
  fair ordering apply to pending work. Regenerate the matching SQLx cache.
- Campaign drain assertions use the same pending-work predicate, and still
  require matching on-chain events. Retaining a completed rejection must not
  look like stalled work.
- Adapt the added relayer campaign tests to 0.14's existing polling and mock
  success helpers. Do not import the old release's optimistic-share test suite.
- Keep the target's dependency versions. ruint is already 1.20.0, so remove the
  source PR's unnecessary ruint advisory exception. Retain the temporary h2
  exception for the locked 0.3.27 and 0.4.12 versions.
- SDK runners exercise the checked-out SDK. Historical deployed-SDK results
  cannot be attributed to this checkout without running that exact SDK again.

## Validation

Local checks use the pinned Rust 1.91.1 toolchain. Results:

- Clippy passes for the sender library, binary, and all tracked test targets,
  with CI warning flags and warnings denied. The pre-existing untracked
  baseline capacity driver is excluded.
- Sender unit tests: 6 passed. HTTP transport tests: 7 passed.
- Mixed selective-outage/restart campaign: passed in 53.10 seconds.
- Near-exhausted proof recovery after HTTP 503: passed in 28.94 seconds.
- Recovery after an unreadable HTML response: passed in 28.86 seconds.
- Gateway credential privacy across logs, health, and database: passed in 8.04 seconds.
- Relayer input-proof and user-decryption test targets compile.
- Relayer final-attempt/fresh-request readiness test: passed in 7.07 seconds.
- Checked-out SDK polling tests: 2 passed.
- Helm lint and rendering confirm separate HTTPS sender and WSS listener URLs,
  including the sender secret-reference override.
- Adapted SQL passes PostgreSQL PREPARE; offline cache hash matches.
- Rust formatting, changed-file spelling, and whitespace checks pass.

Historical full-stack and funded Gateway results in the campaign report belong to earlier
binaries; they are not fresh evidence for this port. Standard e2e and final-HEAD
CI remain required before deployment.

See [rollout prerequisites](transaction-sender-http-rollout.md), particularly
the sender-specific HTTPS URL override while the listener stays on WSS.
