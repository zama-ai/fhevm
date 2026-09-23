# Local validation at e9180f14c

Source: the completed campaign's final report supplied by its operator. This
note records that report; updating it did not rerun tests or independently
revalidate the raw artifacts. The original tested revision is `e9180f14c`.
Subsequent commit-message edits preserve its committed tree; subsequent wording
corrections change documentation, inventory prose and assertion detail only.
Results remain attributed to the original revision, not relabeled to a new SHA.

## Reported outcome

All **13 required inventory cases passed**, with zero failures. Both the full
aggregate and `aggregate --ci` passed. The campaign finished; the operator
reported that no tests remained running.

| Leg | Passing required cases | Execution |
| --- | --- | --- |
| Harness | HAR-02, HAR-03, MAT-05 | Inventory, readiness and comparator contracts |
| Rust regression | REG-01, REG-02 | Selected listener and daemon regressions |
| Byte agreement | MAT-01, MAT-02, MAT-03, REORG-01 | CPU, three-of-three |
| Fork | FORK-01, FORK-02, FORK-03 | CPU, three-of-three-fork |
| GPU | SCH-01 | Heterogeneous scheduling, host CUDA workers |

The report confirms live execution of F3's recovery wait (74 seconds), separate
fault receipts for each fork case, raw-byte canaries in all three reported
suite executions, and topology checks accepting the three real stacks. The GPU
session also executes the materialization fixture's byte and plaintext checks;
it records SCH-01, not a separate GPU pass for every MAT/reorg/fork case.

## Provenance and remaining validation

- Coprocessor binaries were built from the branch. Without GitHub authentication,
  the campaign used a reconstructed `branch-local.json` lock and surrounding
  services from cached bundle `7234af0`. This is not validation against a freshly
  resolved latest-main service bundle.
- `aggregate --ci` checks the required CI backend assignments. Its success here
  does **not** establish `--require-build-mode checkout`: the campaign did not
  produce the cold-up checkout build receipt required for that stronger gate.
  The workflow's begin/build/finish/attach sequence still needs validation in
  CI or an equivalent local cold-up run; it is not inherently CI-only.
- The successful live campaign did not enter the sibling-preservation failure
  path in `rs_finalize_results`. Regression tests exist in the harness, but this
  report does not establish live failure attribution or interrupted cleanup.
- The additional live negative controls recommended by the runbook, such as
  stopping the forked operator after F2, were not run. The live digest/raw-byte
  canaries did run; they should not be conflated with those other controls.
- SCH-02's multi-GPU split remains explicitly deferred and is not a missing
  required case. Single-GPU CI remains the supported campaign target.

F3 proves ingestion of the fresh sentinel's receipt-identified block on the
canonical history and completion of that sentinel, not identification of the
initial replacement block. Materialization treats a nonzero suite exit as a
shared failure; fork can retain individually completed siblings. See the
[runbook](RUNBOOK.md#failure-attribution-and-recovery) for the exact distinction.

## Review and merge assessment

This result supports proceeding to final review of the declared campaign scope.
It is not an unconditional merge sign-off: required CI checks, including the
checkout build-receipt provenance path, must pass for the final branch. Keep the
cached-service baseline and unexercised live controls visible in the PR. No
two-GPU or broader failure-campaign coverage is claimed or required by this
layer's passing full inventory.
