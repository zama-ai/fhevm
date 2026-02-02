# Upgrade Priorities - CI/Infrastructure

Current status and recommended upgrades for CI/CD tooling.

---

## GitHub Actions

### Action Version Upgrades

| Action                     | Current | Latest | Priority |
| -------------------------- | ------- | ------ | -------- |
| actions/checkout           | v3      | v4     | High     |
| actions/cache              | v3      | v4     | High     |
| actions/setup-node         | v3      | v4     | Medium   |
| docker/build-push-action   | v4      | v5     | Medium   |

### Upgrade Path

```yaml
# Batch update in single PR
- uses: actions/checkout@v4
- uses: actions/cache@v4
- uses: actions/setup-node@v4
```

---

## Docker Base Images

### Rust Images

| Current              | Recommended          | Notes                |
| -------------------- | -------------------- | -------------------- |
| rust:1.74            | rust:1.75            | Latest stable        |
| rust:latest          | rust:1.75-bookworm   | Pin version          |
| debian:bullseye      | debian:bookworm      | Newer base           |

### Node.js Images

| Current              | Recommended          | Notes                |
| -------------------- | -------------------- | -------------------- |
| node:18              | node:20              | LTS version          |
| node:18-alpine       | node:20-alpine       | Smaller image        |

---

## Kubernetes/Helm

### Chart Versions

All Helm charts should use:

```yaml
apiVersion: v2
```

### Kubernetes Features

Enable recent features where beneficial:

| Feature              | Min K8s Version | Status    |
| -------------------- | --------------- | --------- |
| PodDisruptionBudget  | 1.21            | Use       |
| TopologySpreadConst  | 1.19            | Use       |
| EphemeralContainers  | 1.25            | Consider  |

---

## CI Optimization Priorities

### Priority 1: Caching Improvements

Current cache hit rates need improvement:

```yaml
# Use Swatinem/rust-cache for Rust
- uses: Swatinem/rust-cache@v2
  with:
    prefix-key: "v1"
    cache-on-failure: true

# Use npm ci for deterministic installs
- run: npm ci
```

### Priority 2: Parallel Execution

Split long-running tests:

```yaml
strategy:
  matrix:
    shard: [1, 2, 3, 4]
steps:
  - run: cargo test --partition ${{ matrix.shard }}/4
```

### Priority 3: Selective Testing

Run tests based on changed files:

```yaml
- uses: dorny/paths-filter@v2
  id: changes
  with:
    filters: |
      rust:
        - 'coprocessor/**'
        - 'kms-connector/**'
      solidity:
        - 'contracts/**'
```

---

## Security Upgrades

### Required Additions

| Tool                 | Purpose                      | Priority |
| -------------------- | ---------------------------- | -------- |
| Trivy                | Image vulnerability scan     | High     |
| TruffleHog           | Secret detection             | High     |
| OSSF Scorecard       | Supply chain security        | Medium   |

### Implementation

```yaml
- name: Run Trivy
  uses: aquasecurity/trivy-action@master
  with:
    scan-type: 'fs'
    severity: 'CRITICAL,HIGH'

- name: Run secret scan
  uses: trufflesecurity/trufflehog@main
```

---

## Infrastructure Upgrades

### AWS Services

| Service              | Current              | Recommended          |
| -------------------- | -------------------- | -------------------- |
| EKS                  | 1.27                 | 1.29                 |
| RDS PostgreSQL       | 14                   | 15                   |

### Monitoring Stack

| Component            | Current              | Recommended          |
| -------------------- | -------------------- | -------------------- |
| Prometheus           | 2.45                 | 2.47                 |
| Grafana              | 9.x                  | 10.x                 |

---

## Migration Checklist

### GitHub Actions v4 Migration

- [ ] Update actions/checkout to v4
- [ ] Update actions/cache to v4
- [ ] Update actions/setup-node to v4
- [ ] Test all workflows in branch
- [ ] Merge in single PR

### Base Image Updates

- [ ] Update Rust base images
- [ ] Update Node.js base images
- [ ] Rebuild all images
- [ ] Test in staging
- [ ] Deploy to production

### Security Tooling

- [ ] Add Trivy to PR checks
- [ ] Add TruffleHog scanning
- [ ] Configure severity thresholds
- [ ] Set up alerting
