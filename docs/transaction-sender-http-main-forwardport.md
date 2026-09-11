# HTTP sender forward-port to main

Source: `antoniu/hotfix/txn-sender-https-baseline` at `d4087082d`.
Source range: the 11 PR commits after `f6da30cce`.
Destination: `antoniu/forwardport/txn-sender-https-main`, based on
`origin/main` at `94f1f3b35`.

The release PR branch is unchanged. Coprocessor chart version is **0.13.19**,
following main's 0.13.18. The originally requested 0.13.9 was superseded by
0.13.19 after confirming the existing main version.

## Integration decisions

- Retain main's Alloy 2.2.0 and Rust 1.97.1 dependency/toolchain versions.
- Retain main's live-write guard when persisting transient retry timestamps.
- Preserve main's synthetic-proof exclusion and rejected-proof retention in the
  scheduling query. Fair ordering and per-proof deferral are applied on top.
- Update campaign queue-drain assertions: successful rejected proofs remain in
  the database with handles cleared until Gateway finalization. Their presence
  does not mean they are still pending sender work. Matching contract events
  remain required for recovery acceptance.
- Keep main's audit configuration: ruint 1.20.0 and production h2 0.4.16 already
  supersede the release dependency versions. Do not add the old ruint exception.
- Preserve main's default value of 10 for the unused legacy reconnect flag.
- Keep historical release validation records as history. The forwarded SDK
  runners import main's SDK; they are not evidence of deployed SDK 0.13.2 behavior.

## Local validation

- Sender library, binary and every tracked test target: Clippy passed with the
  CI warning flags and `-D warnings`. The pre-existing untracked baseline driver
  was excluded and left untouched.
- Six sender unit tests and seven HTTP transport tests passed.
- Mixed selective-failure/new-arrival/restart campaign passed in 54.12 seconds,
  retaining all 50 matching proof events across verification and rejection.
- HTTP 503 rejection recovery passed in 29.26 seconds; unreadable HTML estimation
  recovery passed in 29.21 seconds.
- The real logs/database credential-safety test passed in 8.26 seconds.
- Two SDK polling tests passed against main's checked-out SDK.
- Forwarded relayer input-proof and user-decrypt test targets compile. The build
  reports an existing generic-array deprecation warning in ciphertext-attestation.
- Helm lint and rendering passed, including chart 0.13.19 and separate sender
  HTTPS/listener WSS URLs. PostgreSQL accepted the merged selection query, and
  its SQLx cache hash matches the query text.

Artifacts are retained as `/tmp/fhevm-forwardport-*.log` and
`/tmp/fhevm-forwardport-chart.yaml`. Standard e2e and funded performance testing
have not been rerun for this main forward-port. CI must validate the new branch;
release-branch CI results do not cover it.
