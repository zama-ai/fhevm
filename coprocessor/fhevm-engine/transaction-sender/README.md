# Transaction sender lifecycle

Sender state lives on `verify_proofs` and `ciphertext_digest`. It is independent
of local proof verification (`verified`) and S3 publication readiness.

| `sender_status` | Meaning |
| --- | --- |
| `QUEUED` | Waiting for prerequisites or selection. |
| `SENDING` | Claimed by a worker; preparation or submission is running. |
| `AWAITING_OUTCOME` | A signed submission exists, but its receipt is unresolved. |
| `RETRY_WAIT` | A new attempt is scheduled after backoff. |
| `COMPLETED` | The contribution succeeded or was already processed. |
| `STOPPED` | Permanent failure, exhausted finite retry budget, or obsolete request. |

Each claim increments `sender_attempt_no`. Updates compare that number so a
late result cannot overwrite a successor or a proof replay transition. This
number counts worker claims, including reconciliation; it is separate from
finite retry accounting. `sender_next_action_at` schedules retries and checks;
`sender_last_error` records sender diagnostics without overwriting proof-worker
errors for recoverable RPC failures.

The sender prepares and signs locally, commits `sender_tx_hash` and
`sender_raw_tx`, then broadcasts. After an uncertain response it immediately
checks the receipt. If unresolved, it preserves the signed bytes and nonce
reservation, then checks again or rebroadcasts those same bytes after backoff.
A missing receipt alone does not establish non-submission. A definite nonce
rejection triggers reconciliation without consuming the finite retry budget.
Automatic fee replacement is outside this change: a transaction that remains
pending keeps its reservation and `AWAITING_OUTCOME` state.

Startup acquires an exclusive PostgreSQL session advisory lock for this
sender database, recovers interrupted claims, validates saved transactions'
chain and signing account, and restores nonce reservations before new work.
Ownership lasts through shutdown; database/ownership loss stops the sender
for recovery. During blue/green cutover the retired sender drains and exits
so the new version can acquire ownership. The allocator still requires one
writer per signing address across independent databases and other services.

Apply migration `20261028120000_sender_task_status.sql` before this binary.
For the initial rollout, stop the previous sender before migrating and starting
this version: older binaries do not honor the new status or ownership protocol.
The transaction signer must match the nonce-managed account. Deploy the updated
SQL-exporter query after the migration so retained completed rows do not inflate
its backlog gauge.

Exhausted jobs remain `STOPPED`, including when the legacy
`verify_proof_remove_after_max_retries` option is true; that option is accepted
for configuration compatibility. Successful positive proof rows are eligible
for cleanup after one day. Gateway finalization and local replay retain their
existing cleanup responsibilities, but must preserve unresolved signed
submissions. These task rows are not a permanent transaction audit log.

`sender_recovery_tests` and the transition tests are unignored and run in the
existing coprocessor Cargo CI jobs, alongside the proof and ciphertext retry
regressions. Local integration tests require Docker and Anvil.
