# Phase 0 Report — Honest L0

Date: 2026-06-16
Commit: `1ed069aa5` (worktree `wf_787a7740-071-3`)
Gate: L0 — helm-render golden record (no cluster)

---

## What changed

Single commit (`1ed069aa5`) touching only `stack/` — no chart source files modified.

**`stack/.migration/` additions:**

- `normalize.sh` — canonical filter: strips volatile lines (`helm.sh/chart`, `app.kubernetes.io/version`, `checksum/`, `uid:`, `creationTimestamp:`, `# Source:` comments) then sorts. Makes the golden reproducible from first principles; was undocumented in the prior dry-run.
- `run-l0.sh` — renders all 5 charts per topology case (anvil-node, contracts, N×coprocessor, kms-connector, listener), counts documents, normalizes, asserts sha256 + doc-count against stored goldens.
- `golden/default/{doc-count,manifests.norm,sha256}` — baseline for single-coprocessor topology.
- `golden/two-of-three/{doc-count,manifests.norm,sha256}` — baseline for three-coprocessor topology.

**`stack/values/` additions:**

- `default.yaml`, `two-of-three.yaml` — restructured to use real chart top-level value schemas (previously keys were nested under `contracts:`/`coprocessor:`/`kmsConnector:` and never reached helm template). `_topology.numCoprocessors` embedded; read by render harness to expand N coprocessor releases.
- `kind-local.yaml` — shared kind-local overrides.
- `kms-connector-default.yaml`, `kms-connector-two-of-three.yaml` — isolate the kms-connector `commonConfig` key from the coprocessor chart's `commonConfig` key (same top-level key, incompatible schemas).

Charts under `charts/` were not touched by this commit. The `charts/coprocessor/` edits visible in the branch diff belong to a pre-existing merge commit (`2d4083906`) that predates Phase 0 work.

---

## Gate evidence

| Level | Status | Notes |
|-------|--------|-------|
| L0 | PASS | — |
| L1–L3 | Not run | Skipped (no cluster required at L0) |

**L0 golden values:**

| Case | Doc count | SHA-256 |
|------|-----------|---------|
| `default` | 25 | `26e1e84c7ca2bb7f7e1ec424c447590c726a73ccad30e80382bc3eaf26de3331` |
| `two-of-three` | 49 | `0fb7ab315e945c5e1cd03e130cc12261cd05e1af06389c2267770cbdea313850` |

Hashes are distinct. Doc-count difference (25 vs 49) is consistent with three coprocessor releases in `two-of-three` vs one in `default` (each coprocessor release contributes one Deployment + one Job + one Service = 3 additional docs × 8 additional coprocessors = 24 additional docs).

---

## Verifier outcome

Three independent verifiers ran against worktree `wf_787a7740-071-3`.

**Verifier 1 — not refuted.** Fresh re-run reproduced both golden hashes exactly. `normalize.sh` confirmed git-tracked. All assertions pass.

**Verifier 2 — not refuted.** Both hashes reproduced live. Doc-count arithmetic independently verified. Three coprocessor releases render as structurally distinct (release-name suffixes preserved after normalization). One theoretical weakness noted: the global-sort approach in `normalize.sh` would not detect value swaps between env vars of equal string content. No actual regression found exploiting this.

**Verifier 3 — refuted claim is itself incorrect.** Verifier 3 claimed `1ed069aa5` modified `charts/coprocessor/Chart.yaml`, `charts/coprocessor/templates/coprocessor-db-migration.yaml`, and `charts/coprocessor/values.yaml`. This is false. `git diff --name-only 1ed069aa5^..1ed069aa5` shows no `charts/` files. Those chart edits belong to `2d4083906` (a pre-existing merge commit). The verifier conflated branch-wide diff (`main..HEAD`) with the Phase 0 commit diff. The gate PASS claim stands; the lane-boundary-crossed claim does not hold for this commit.

---

## Open items (non-blocking at L0)

- `normalize.sh` global-sort has the theoretical value-swap blind spot described by Verifier 2. Low risk at present; consider a deterministic per-doc normalization in a future pass.
- Listener Deployment, ConfigMap, and Service templates are untested (gated on `.Values.listeners`, unset in both topology cases).
- `stack/values/default.yaml` image-tag overrides need verification that they actually reach rendered manifests (schema wiring done; not explicitly spot-checked in L0 assertions).

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
4. After merge, start Phase 1: wire `acceptance.yml` as the standing CI gate using the recorded sha256 goldens, so manifest drift is caught on every PR.
