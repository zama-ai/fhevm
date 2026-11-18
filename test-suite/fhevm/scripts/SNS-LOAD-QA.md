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
Then SNS stops, 16 clients each set up a fresh encrypted input and produce 2,000
distinct public outputs total (20 independent additions per transaction, with a
different scalar per output). Input proofs are setup rather than regenerated for
every batch: the benchmark targets SNS capacity. TFHE prepares
every CT64. Four identical SNS processes start against this single operator DB,
with batches of 20, four Rayon threads each, and S3 migration disabled. Keeping a
prebuilt backlog separates SNS capacity from producer/proof/TFHE throughput.

The test records committed handles and failures per worker, CPU time, cohort
backlog and upload progress every two seconds. It requires all four workers to
process tasks, observed CPU activity on all four within the same sampling
interval. Ordered locking requires zero task failures and zero database/logged
deadlocks; retry-only records recovered deadlocks and task failures for comparison. It checks DB
completion and canonical CT64/CT128 object presence for every handle, then public
decryption of 64 evenly spread handles. Decryption is outside drain timing.

The Actions summary and `sns-four-worker-load-*` artifact contain the manifest,
CSV measurements, final metrics, worker logs, PostgreSQL logs and summary. Concurrent CPU activity
does not establish linear speedup or eliminate ordinary waits on shared stripes;
these runs have no one-worker throughput control or unpatched baseline.
The 2,000-handle cohort is 100 full batches of 20; at the first measured rate of
about 1.4 handles/s, SNS processing takes roughly 24 minutes, plus setup.
The three-hour ceiling is a failure bound rather than an expected duration.
