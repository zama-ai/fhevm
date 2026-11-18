# One-shot four-worker SNS load QA

This QA branch runs PR #5000's published SNS binary (`c0e6428`) against a stack
resolved from its release base (`31e76b13612880bfeb214fed2ff545687459860a`). It does
not change the PR or a deployed environment.

Dispatch `test-suite-e2e-tests.yml` on this branch with:

- `build=false`
- `scenario=sns-four-worker-load`
- `test-profile=sns-four-worker-load`
- `coprocessor-sns-worker-version=c0e6428`

The ordinary encrypted-input → add → user/public-decrypt smoke test runs first.
Then SNS stops, 16 clients each set up a fresh encrypted input and produce 10,000
distinct public outputs total (20 independent additions per transaction, with a
different scalar per output). Input proofs are setup rather than regenerated for
every batch: the benchmark targets SNS capacity. TFHE prepares
every CT64. Four identical SNS processes start against this single operator DB,
with batches of 20, four Rayon threads each, and S3 migration disabled. Keeping a
prebuilt backlog separates SNS capacity from producer/proof/TFHE throughput.

The test records committed handles and failures per worker, CPU time, cohort
backlog and upload progress every two seconds. It requires all four workers to
process tasks, observed CPU activity on all four within the same sampling
interval, zero task failures and zero database/logged deadlocks. It checks DB
completion and canonical CT64/CT128 object presence for every handle, then public
decryption of 64 evenly spread handles. Decryption is outside drain timing.

The Actions summary and `sns-four-worker-load-*` artifact contain the manifest,
CSV measurements, final metrics, worker logs and summary. Concurrent CPU activity
does not establish linear speedup or eliminate ordinary waits on shared stripes;
this run has no one-worker throughput control or unpatched baseline.
