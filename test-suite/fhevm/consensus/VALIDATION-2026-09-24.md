# PR14 local CPU/GPU validation — complete

Completed **24 September 2026**. This summary records the final campaign notes
provided by the campaign owner. The underlying artifacts remain with the
execution environment; they were not rerun or independently re-audited during
this documentation refresh.

| Scope | Result |
| --- | --- |
| Required consensus/failure inventory, CPU and GPU | **70/70 PASS** |
| Blue/green rollout modes | **8/8 PASS** |
| Key-migration rollout modes | **7/7 PASS** |
| Final evidence audit | **PASS — no problems** |

These results supersede the preparation-time pending-execution statements in
the [delivery ledger](COVERAGE-DELIVERY.md). They cover the three distinct
campaign families in the [runbook](RUNBOOK.md#release-campaign-checklist), not
just the inventory workflow's `full` selection.

## Source provenance and affected reruns

This was a complete campaign assembled from unchanged-source evidence and
affected reruns, not a single-revision live sweep. The inventory, eight
blue/green modes and three unaffected migration modes ran before the final
focused fix. Four download-fault modes then ran as fresh complete rollouts with
the final fix.

That fix changed the long-lived download proxy client timeout and its isolated
regression test. The campaign's equivalence audit found all 4,626 other tracked
entries identical. Original evidence retains its actual execution revision;
results were not relabeled to pretend they came from one checkout. The strict
inventory aggregate passed with `--ci --require-build-mode checkout` and its
actual execution revision. Cold checkout build receipts and rollout image
identities, including the fresh final production-target build, were audited.

The branch was subsequently rebased. The campaign owner confirms the code is
equivalent for this documentation update; rewritten commit hashes alone are not
a coverage gap. The original artifact names below are retained for traceability.

## Fault observation and recovery

All four final download modes completed fresh rollout, observed fault, healthy
retry, cleanup, CPU legacy-key reload, normal input/proof/compute/decryption and
original retained-material checks on both host chains. Their audits checked
actual response bytes/hashes, retry ordering, all five restored recipient
services and cleared fault routes. Wrong-key used a validated independently
generated compressed-key fixture; malformed used the expected 31-byte payload.
The final wrong-key mode completed at `2026-09-24T01:51:02.541073Z`.

GPU migration verified compressed-key consumption on CUDA workers, both host
chains, worker restart/reload, retained material and restoration of the original
CPU fleet. The inventory also exercised real GPU reservation-pressure
timeout/retry and scheduling checks, in addition to lifecycle cases.

An earlier interrupted-download attempt recovered its workload but failed
cleanup when the proxy client hit the ordinary host-command timeout. That failed
attempt and its cleanup evidence remain archived. The focused timeout fix was
followed by successful reruns of all four affected modes. **No failure record was
relabeled as a pass.**

The controller and observer exited after completing their work. The deployment
was reported left up; do not assume that the execution machine has since been
torn down or is still available without checking it there.

## Environment and limits

- Local execution used public runtime images. Hosted CI and private Chainguard
  pulls were not exercised.
- Surrounding services came from a pinned compatible/cached bundle; branch
  coprocessor targets were built locally. GitHub authentication was unavailable.
- GPU execution used **one H100**. This is not L40 execution evidence or
  multi-device attribution. The separate operator suites and optional two-GPU
  SCH-02 were excluded as requested.
- GPU `Default`-parameter key generation used the documented test-only setup.
  This is neither a secrecy audit nor proof of secure distributed key generation,
  and it does not authorize a production key-representation policy change.
- The generated `test-suite/e2e/contracts/E2ECoprocessorConfigLocal.sol` was an
  execution-local runtime change, not part of the fix commit.
- Existing declared limits still apply: bounded fault models, no new
  Gateway-specific fault/finality campaign and no public multi-output operation.
  Successful migration against the pinned bundle is not evidence of every
  release pair, complete future contract upgrade plan or multi-epoch deployment.

## Evidence entry points

These are filenames supplied with the completion notes, not files copied into
this repository. Preserve their original names and checksums when transferring
the campaign archives:

- `f121909cc-final-completion-audit.json` — scope audit and artifact SHA-256 manifest.
- `f121909cc-validation-source-equivalence.json` — changed paths and unchanged-tree digest.
- `f121909cc-preserved-inventory-aggregate.log` — strict required-inventory verdict.
- `ee7676a65-final-blue-green-complete-checkpoint.json` — all eight blue/green modes.
- `ee7676a65-final-gpu-migration-completion-audit.json` — GPU migration evidence.
- `f121909cc-final-download-*-completion-audit.json` — affected download modes,
  with private evidence archives and checksums.
- `STATUS.md` — append-only campaign history, including failures and resolution.

Release documentation should link the transferred artifacts when their location
is available. Their absence from this checkout does not undo the owner's
completion report, and this summary does not claim to have inspected them here.
