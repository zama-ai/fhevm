# Trajectory - CI/Infrastructure

Where CI/CD and infrastructure are heading based on recent activity.

---

## Growing Areas

### GPU-Enabled CI Runners

**Status**: Active development

Adding GPU support for FHE computation tests:

- Self-hosted runners with NVIDIA GPUs
- GPU-specific test workflows
- CUDA toolkit integration

**Implication**: New compute-intensive tests should be GPU-aware:

```yaml
jobs:
  test-gpu:
    runs-on: [self-hosted, gpu]
    steps:
      - name: Run GPU tests
        run: cargo test --features gpu
```

### Production Hardening

Recent focus areas:

- Improved health checks
- Better graceful shutdown
- Enhanced monitoring
- Disaster recovery procedures

**Implication**: All new deployments must include:

- Liveness and readiness probes
- Resource limits
- Pod disruption budgets
- Proper termination handling

### CI Time Optimization

Active work on reducing CI times:

- Improved caching strategies
- Parallel test execution
- Selective testing based on changes

**Implication**: Use matrix builds and caching:

```yaml
strategy:
  matrix:
    shard: [1, 2, 3, 4]
steps:
  - uses: Swatinem/rust-cache@v2
```

---

## Stable Areas

### Core Build Workflows

Build pipelines are mature:

- Rust compilation workflows
- Solidity compilation
- Docker image builds

**Implication**: Follow existing patterns; avoid unnecessary changes.

### Release Process

Release automation is stable:

- Version tagging
- Changelog generation
- Artifact publishing

---

## Active Migrations

### Docker Compose to Kubernetes

For production deployments:

| Environment  | Current              | Target               |
| ------------ | -------------------- | -------------------- |
| Development  | Docker Compose       | Docker Compose       |
| Staging      | Docker Compose       | Kubernetes           |
| Production   | Kubernetes           | Kubernetes           |

**Implication**: New services need both docker-compose and Helm definitions.

### GitHub Actions v4

Upgrading action versions:

```yaml
# Old
- uses: actions/checkout@v3

# New
- uses: actions/checkout@v4
```

**Implication**: Use v4 for all standard actions.

---

## Declining Areas

### Legacy Docker Compose Production

Moving away from compose for production:

- Retained for local development
- Kubernetes for all deployed environments

### Manual Deployments

Reducing manual intervention:

- GitOps-based deployments
- Automated rollbacks
- Infrastructure as code

---

## Upcoming Changes

### Short Term (1-3 months)

- Complete GPU runner setup
- Optimize caching for Rust builds
- Add security scanning to all workflows

### Medium Term (3-6 months)

- Multi-region deployment support
- Enhanced observability
- Automated performance testing

### Long Term (6+ months)

- Serverless components
- Edge deployment capabilities
- Advanced auto-scaling

---

## Pattern Evolution

### Workflow Structure

```text
v1 (deprecated): Single large workflow files
v2 (current):    Reusable workflow patterns
v3 (emerging):   Composite actions for complex logic
```

### Deployment Model

```text
v1 (deprecated): Manual kubectl apply
v2 (current):    Helm with ArgoCD
v3 (emerging):   Full GitOps with Flux
```

### Caching Strategy

```text
v1 (deprecated): Basic actions/cache
v2 (current):    Specialized caches (rust-cache)
v3 (emerging):   Distributed cache with Buildkite
```
