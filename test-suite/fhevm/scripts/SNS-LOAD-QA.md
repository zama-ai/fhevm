# One-shot four-worker SNS load QA

This QA branch compares PR #5000's ordered-lock SNS binary (`c0e6428`) with
PR #4998's immediate-deadlock-retry binary (`4c5425f`) against a stack
resolved from its release base (`31e76b13612880bfeb214fed2ff545687459860a`). It does
not change the PR or a deployed environment.

Dispatch `test-suite-e2e-tests.yml` on this branch with:

- `build=false`
- `scenario=sns-four-worker-load`
- `test-profile=sns-four-worker-load`
- `coprocessor-sns-worker-version=c0e6428` (ordered locks) or `4c5425f` (retry only)

The ordinary encrypted-input → add → user/public-decrypt smoke test runs first.
The load phase uses PR #5024's publication scenario, scaled to 2,000 handles.
Four identical SNS processes warm their keys on an empty queue, then Docker pauses
them. A host contract issues 125 transactions with 16 fresh publicly decryptable
`euint8` handles each, using trivial encryption. Once all CT64 and pending SNS
tasks are ready, the four processes resume together against this single operator
database. They use batches of 20, four Rayon threads each, 24 database connections,
four concurrent uploads, two-second polling and five-second upload resubmission.
S3 migration stays disabled, and the original 256-stripe trigger definitions are
verified and saved without modification. No sleeps or scheduling barriers are
inserted into transactions or trigger functions.

The test records committed handles and failures per worker, CPU time, cohort
backlog and upload progress at approximately one-second intervals. Key warm-up CPU
is excluded, and generation, CT64 preparation, SNS computation and full upload
durations are reported separately. Host CPU details are retained for comparison.
All four workers must process tasks, with overlapping full batches on different
workers. Ordered locking requires zero task failures and zero database/logged
deadlocks. Retry-only must reproduce at least one PostgreSQL deadlock and an SNS
immediate retry; otherwise its deadlock comparison is explicitly inconclusive,
even if performance and functionality checks complete. Upload timeouts and missing
savepoints are counted separately from SNS deadlock retries.

The test checks DB completion, canonical CT64/CT128 objects and Gateway material
consensus for all 2,000 handles, then publicly decrypts 64 evenly spread handles.
Gateway and decryption checks are outside drain timing.

The Actions summary and `sns-four-worker-load-*` artifact contain the manifest,
CSV measurements, final metrics, worker logs, PostgreSQL logs and summary. Concurrent CPU activity
does not establish linear speedup or eliminate ordinary waits on shared stripes;
these runs have no one-worker throughput control or unpatched baseline.
The 2,000-handle cohort is 100 full batches of 20. Earlier throughput figures used
`euint64` additions and cold SNS startup, so they are not directly comparable to
this lighter, warmed publication workload.
The three-hour ceiling is a failure bound rather than an expected duration.
