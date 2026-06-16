# Phase 0 Report — Honest L0

Date: 2026-06-16
Commit: `1ed069aa5` (worktree `wf_787a7740-071-3`)
Gate: L0 — helm-render golden record (no cluster)

---

## What changed

Single commit (`1ed069aa5`) touching only `stack/` — no chart source files modified.

**`stack/.migration/` additions:**

- `normalize.sh` — canonical filter: strips volatile lines (`helm.sh/chart`, `app.kubernetes.io/version`, `checksum/`, `uid:`, `creationTimestamp:`, `# Source:` comments) then sorts. Makes the golden reproducible from first principles.
- `run-l0.sh` — renders all 5 charts per topology case (anvil-node, contracts, N×coprocessor, kms-connector, listener), counts documents, normalizes, asserts sha256 + doc-count against stored goldens.
- `golden/default/{doc-count,manifests.norm,sha256}` — baseline for single-coprocessor topology.
- `golden/two-of-three/{doc-count,manifests.norm,sha256}` — baseline for three-coprocessor topology.

**`stack/values/` additions:**

- `default.yaml`, `two-of-three.yaml` — restructured to use real chart top-level value schemas (previously keys were nested under `contracts:`/`coprocessor:`/`kmsConnector:` and never reached helm template). `_topology.numCoprocessors` embedded; read by render harness to expand N coprocessor releases.
- `kind-local.yaml` — shared kind-local overrides.
- `kms-connector-default.yaml`, `kms-connector-two-of-three.yaml` — isolate the kms-connector `commonConfig` key from the coprocessor chart's `commonConfig` key (same top-level key, incompatible schemas).

Charts under `charts/` were not touched by this commit. Chart edits visible in the branch diff belong to a pre-existing merge commit (`2d4083906`) that predates Phase 0 work.

---

## Gate evidence

| Level | Status | Notes |
|-------|--------|-------|
| L0 | PASS | All three criteria met |
| L1–L3 | Not run | No cluster required at L0 |

**L0 golden values:**

| Case | Doc count | SHA-256 |
|------|-----------|---------|
| `default` | 25 | `26e1e84c7ca2bb7f7e1ec424c447590c726a73ccad30e80382bc3eaf26de3331` |
| `two-of-three` | 49 | `0fb7ab315e945c5e1cd03e130cc12261cd05e1af06389c2267770cbdea313850` |

Hashes are distinct. Doc-count difference (25 vs 49) is consistent with three coprocessor releases in `two-of-three` vs one in `default`: three coprocessor releases render 11 docs each vs 9 docs for one release (the extra 2 docs per release come from `hostListenerPoller` enabled on the polygon chain in `two-of-three.yaml`); plus topology-specific differences in kms-connector endpoints account for the remaining delta.

Three topology-specific values survive normalization and differ meaningfully between cases: `NUM_COPROCESSORS`, `COPROCESSOR_THRESHOLD`, `COPROCESSOR_SIGNER_ADDRESS_0..N-1`, `kmsCoreEndpoints`, and chain URLs. The global-sort normalization would not detect a value swap between env vars of equal string content — acknowledged as a theoretical weakness, no actual regression found.

---

## Verifier outcome

Three independent verifiers ran against worktree `wf_787a7740-071-3` (and a second live re-execution in `wf_6d7bda03-919-1`).

**Verifier 1 — not refuted.** Fresh re-run reproduced both golden hashes exactly (default=`26e1e84c...`, two-of-three=`0fb7ab31...`). `normalize.sh` confirmed git-tracked at blob `890ff350`. Doc-count assertions passed (25 and 49). No failures, unexpected diffs, or skipped levels.

**Verifier 2 — not refuted.** Both hashes reproduced live. Three adversarial angles investigated and cleared: (1) over-aggressive normalization hiding a real diff — topology-specific values survive normalization, no swap regression found; (2) skipped test reported as passed — listener chart renders 1 doc because `.Values.listeners` is unset, explicitly acknowledged as open item; (3) intended diff that is actually a behavior change — the structural difference is real and meaningful. One stale artifact noted: `L0-dry-run.md` contains sha256 values from a prior iteration where both cases produced the same hash, but this is not a test artifact and the committed goldens are correct.

**Verifier 3 — not refuted.** Confirmed commit `1ed069aa5` adds exactly 13 files, all within `stack/values/` and `stack/.migration/`. `git diff --name-only 1ed069aa5^..1ed069aa5` shows no `charts/` files touched. Extra files (`stack/cli/`, `stack/lib/`, `stack/runbooks/`, `stack/EXEMPLAR.md`, etc.) are untracked working-tree files, not committed. No remote tracking branch exists for this commit (not pushed). `run-l0.sh` uses `helm template` (dry-run render) with no `kubectl apply`, `helm install`, or `kind create` calls — no cluster leak. All three L0 criteria confirmed.

---

## Open items (non-blocking at L0)

- `normalize.sh` global-sort has a theoretical value-swap blind spot (equal-content env vars would not produce distinct hashes if swapped). Low risk at present; consider per-doc deterministic normalization in a future pass.
- Listener Deployment, ConfigMap, and Service templates are untested (gated on `.Values.listeners`, unset in both topology cases).
- `stack/values/default.yaml` image-tag overrides need verification that they reach rendered manifests (schema wiring done; not spot-checked in L0 assertions).

---

## What this run did NOT do

- Did not push any branch to remote.
- Did not merge into main.
- Did not run `helm install`, `helm upgrade`, or create any kind cluster.
- Did not advance to L1, L2, or L3.

---

## Next action for the human

1. Review commit `1ed069aa5` in worktree `wf_787a7740-071-3` (or the equivalent branch tip).
2. Confirm the `stack/values/` schema changes and the `normalize.sh`/`run-l0.sh` scripts look correct.
3. Merge to the working branch (or open a PR if the project workflow requires one).
4. After merge, start Phase 1: wire `acceptance.yml` as the standing CI gate using the recorded sha256 goldens so manifest drift is caught on every PR.
