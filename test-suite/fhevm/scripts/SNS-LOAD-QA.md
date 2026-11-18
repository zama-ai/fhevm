# Four-stripe SNS contention experiment (QA branch only)

This experiment reduces the 256 production advisory stripes to four. Neither
production PR is modified. The ordered-lock query and both installed mirror
trigger functions use the same four-stripe mapping. The DB override preserves
all other trigger logic, and is applied only to disposable GitHub Docker DBs.

Dispatch `coprocessor-docker-build.yml` on this branch with only
`build_sns_worker=true` and every other build flag false. After the ordered
four-stripe image builds, two matching 2,000-handle E2Es start in parallel:
PR #4998's published `4c5425f` retry-only image and this branch's ordered image.
Both use one DB, four SNS workers, batches of 20 and four Rayon threads each.

Both runs require complete DB/S3 coverage and 64 correct sampled decryptions.
The retry-only control must actually observe a database deadlock; otherwise it
reports an inconclusive comparison. Ordered locking requires zero deadlocks.
The workload adds no artificial sleeps or barriers to create contention.

Image tags appear in concurrency groups and artifact names so the two reusable
workflow legs cannot cancel one another or overwrite each other's artifacts.
