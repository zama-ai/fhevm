# Best Practices - CI/Infrastructure

Current best practices for CI/CD and infrastructure with sources.

---

## GitHub Actions

### Reusable Workflows

Create reusable workflows for common patterns:

```yaml
# .github/workflows/reusable-rust-build.yml
name: Reusable Rust Build

on:
  workflow_call:
    inputs:
      working-directory:
        required: true
        type: string
      features:
        required: false
        type: string
        default: ""

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --features "${{ inputs.features }}"
        working-directory: ${{ inputs.working-directory }}
```

**Source**: [GitHub Reusable Workflows](https://docs.github.com/en/actions/using-workflows/reusing-workflows)

### Pin Actions with SHA

```yaml
# Version tag (acceptable)
- uses: actions/checkout@v4

# SHA pinning (more secure)
- uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4.1.1
```

**Source**: [GitHub Security Hardening](https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions)

### Matrix Builds

```yaml
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable, beta]
        exclude:
          - os: macos-latest
            rust: beta
    runs-on: ${{ matrix.os }}
    steps:
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
```

**Source**: [GitHub Matrix Strategy](https://docs.github.com/en/actions/using-jobs/using-a-matrix-for-your-jobs)

### Concurrency Control

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

**Source**: [GitHub Concurrency](https://docs.github.com/en/actions/using-jobs/using-concurrency)

---

## Docker

### Multi-Stage Builds

```dockerfile
# Build stage
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/myapp /usr/local/bin/
USER nobody
CMD ["myapp"]
```

**Source**: [Docker Multi-Stage Builds](https://docs.docker.com/build/building/multi-stage/)

### Layer Ordering

Order layers from least to most frequently changing:

```dockerfile
# 1. Base image (rarely changes)
FROM rust:1.75

# 2. System dependencies (occasionally changes)
RUN apt-get update && apt-get install -y libssl-dev

# 3. Application dependencies (changes with Cargo.lock)
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# 4. Application code (changes frequently)
COPY src ./src
RUN cargo build --release
```

### BuildKit Features

```dockerfile
# syntax=docker/dockerfile:1.4

# Use cache mounts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release
```

**Source**: [BuildKit](https://docs.docker.com/build/buildkit/)

---

## Kubernetes/Helm

### Resource Management

```yaml
resources:
  requests:
    memory: "128Mi"
    cpu: "100m"
  limits:
    memory: "256Mi"
    cpu: "500m"
```

**Source**: [Kubernetes Resource Management](https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/)

### Health Probes

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 10
  failureThreshold: 3

readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 5
```

**Source**: [Kubernetes Probes](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/)

### Helm Values Structure

```yaml
# values.yaml
replicaCount: 3

image:
  repository: ghcr.io/zama-ai/fhevm-coprocessor
  tag: v1.0.0
  pullPolicy: IfNotPresent

resources:
  requests:
    memory: "256Mi"
    cpu: "200m"

config:
  logLevel: info
  workers: 4
```

**Source**: [Helm Best Practices](https://helm.sh/docs/chart_best_practices/)

---

## Security

### Least Privilege

```yaml
permissions:
  contents: read
  pull-requests: write
```

### Secret Scanning

```yaml
- name: Scan for secrets
  uses: trufflesecurity/trufflehog@main
  with:
    path: ./
    base: ${{ github.event.pull_request.base.sha }}
    head: ${{ github.event.pull_request.head.sha }}
```

### Vulnerability Scanning

```yaml
- name: Scan image
  uses: aquasecurity/trivy-action@master
  with:
    image-ref: 'myapp:${{ github.sha }}'
    severity: 'CRITICAL,HIGH'
    exit-code: '1'
```

**Source**: [Trivy](https://github.com/aquasecurity/trivy)

---

## Caching

### Rust Cache

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    prefix-key: "v1-rust"
    cache-directories: |
      ~/.cargo/advisory-db
```

**Source**: [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache)

### Docker Layer Cache

```yaml
- uses: docker/build-push-action@v5
  with:
    context: .
    cache-from: type=gha
    cache-to: type=gha,mode=max
```

**Source**: [Docker GitHub Actions Cache](https://docs.docker.com/build/cache/backends/gha/)
