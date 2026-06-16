# L0 Dry-Run Report

Date: 2026-06-16
Gate: L0 — helm-render golden record (kind-side baseline, no cluster)

---

## Per-case results

| Case | Rendered? | Golden action | SHA-256 | Verifier verdict |
|------|-----------|---------------|---------|-----------------|
| `default` | Yes (all 5 charts) | RECORDED | `4b8d1827d4dc1d6fda9ed8d78fb28349a2ba0063ccd1a7e32de7a1bb2f11bd4a` | Complete, reproducible, not refuted |
| `two-of-three` | Yes (all 5 charts) | RECORDED | `4b8d1827d4dc1d6fda9ed8d78fb28349a2ba0063ccd1a7e32de7a1bb2f11bd4a` | Complete, reproducible, not refuted |

Both cases produced identical normalized streams (same sha256). This is expected: neither overlay activates `.Values.listeners` or `erpc.enabled`, so the rendered output is structurally the same.

### Chart breakdown (both cases identical)

| Chart | Rendered? | Kinds emitted | Notes |
|-------|-----------|---------------|-------|
| `anvil-node` | Yes | Service, StatefulSet | |
| `contracts` | Yes | ConfigMap, Job, PersistentVolumeClaim, Role, RoleBinding, ServiceAccount | |
| `coprocessor` | Yes | Deployment, Job, Service | |
| `kms-connector` | Yes | Deployment (x3), Job, Service (x3) | 7 docs total; manifestKinds list correct but elides multiplicity |
| `listener` | Yes (1 doc) | ServiceAccount only | Expected: deployment.yaml is gated on `.Values.listeners` (not set in any overlay) and `erpc.enabled` is unset. Listener Deployment, ConfigMap, and Service paths are not exercised by either render case. |

---

## What L0 proves

- All 5 charts parse and template without error under the exemplar values overlays (`kind-local.yaml` + per-case overlay).
- Resolved manifests are well-formed Kubernetes YAML (helm exit 0 for every chart).
- A stable kind-side golden is captured at `stack/.migration/golden/` for future regression comparison.

## What L0 does NOT prove

- **Cross-engine equivalence.** The docker-compose CLI and the helm/kind path produce structurally different artifacts; a byte-diff between them is not meaningful. L0 only records the kind-side baseline. L2 e2e (real cluster, running workloads) is the cross-engine equivalence gate.
- **Per-chart values wiring.** `stack/values/default.yaml` has no effect on any individual chart render: its keys (`contracts:`, `coprocessor:`, `kmsConnector:`) are not top-level keys in any chart's values schema. The golden was produced using `kind-local.yaml` only. Image-tag overrides from `default.yaml` are silently ignored until the values structure is reconciled with chart expectations.
- **Listener correctness.** The listener Deployment, ConfigMap, and Service templates are untested by these render cases.
- **Install/upgrade behavior.** No `helm install` or `helm upgrade` was run; no cluster was created.

---

## Read-only guarantee

L0 was strictly read-only outside the golden directory. The only files written during this run:

```
stack/.migration/golden/default/manifests.norm   (795 lines, normalized manifest stream)
stack/.migration/golden/default/sha256
stack/.migration/golden/two-of-three/manifests.norm   (795 lines, normalized manifest stream)
stack/.migration/golden/two-of-three/sha256
```

No kind cluster was created. No `helm install` / `helm upgrade` was run. No writes were made outside `stack/.migration/golden/`.

---

## Open concerns (from verifier, non-blocking at L0)

1. **Normalization algorithm undocumented.** The golden was produced by stripping volatile lines (matching `helm.sh/chart`, `app.kubernetes.io/version`, `checksum/`, `uid:`, `creationTimestamp:`) plus `level=INFO` helm symlink-resolution messages from the listener chart's `configs/` symlinks, then sorting. This filter set is implicit in the golden but not captured in any script or spec. A naive re-record with a different stripping policy would silently diverge from the stored sha256. Fix: commit a `normalize.sh` alongside the golden files before L1.

2. **`default.yaml` values not wired to charts.** See "What L0 does NOT prove" above. Needs reconciliation before CLI renders image tags from that file.

3. **kms-connector multiplicity not asserted.** kms-connector renders 3 Deployments + 3 Services + 1 Job. The kinds list masks this. A future silent drop of one replica would not be caught by a kinds-only check. Fix: add a doc-count assertion to the acceptance test.

---

## Next gate steps

1. **Document normalization.** Commit a `normalize.sh` (or equivalent spec) to `stack/.migration/` so any re-record uses the identical filter set and the sha256 is reproducible from first principles.

2. **Wire `acceptance.yml` as the standing CI gate.** The L0 golden sha256s become the expected values. On every PR, re-render and compare; fail if sha differs. This prevents silent manifest drift.

3. **Reconcile `default.yaml` values structure** with per-chart schemas so image-tag overrides actually take effect.

4. **Record the compose-side L2 behavioral golden** from the current CLI before migrating further, so the cross-engine baseline exists for comparison.

5. **Run L1–L3 on a real kind cluster**: L1 = `helm install` dry-run / lint, L2 = cluster up + workload health checks (cross-engine equivalence), L3 = upgrade path smoke test.
