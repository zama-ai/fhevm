---
name: fhevm-ci-infrastructure
description: |
  Manage CI/CD pipelines, Kubernetes/Helm charts, and infrastructure for fhevm.
  Covers GitHub Actions workflows, Docker, and cloud deployments.
  Use when working on CI, charts/, or infrastructure configuration.
allowed-tools: [Bash, Read, Write, Edit, Grep, Glob]
---

# FHEVM CI/CD & Infrastructure

Manage continuous integration, deployment pipelines, and infrastructure for the
fhevm ecosystem. This skill covers GitHub Actions (42+ workflows), Helm charts,
Docker, and AWS integrations.

## Scope

- `.github/workflows/` - GitHub Actions CI/CD
- `charts/` - Kubernetes Helm charts
- `docker/` - Dockerfiles and compose configs
- Infrastructure configuration files

## Commit Format

```text
type(scope): description

Scopes: ci, charts
Types: feat, fix, chore, ci, refactor, docs
```

## Merge Criteria

See [references/merge-criteria.md](references/merge-criteria.md) for detailed
requirements. Key points:

1. **PR Size**: 10-50 lines optimal for CI changes
2. **Issue Linking**: Always include `Closes #XXX`
3. **Testing**: CI changes must be tested in a branch first
4. **Secrets**: Never hardcode secrets; use GitHub secrets
5. **Idempotency**: Workflows must be safe to re-run

## Reviewer Focus Areas

See [references/reviewer-profiles.md](references/reviewer-profiles.md).

- **jatZama**: Architecture, naming consistency
- **rudy-6-4**: Robustness, retry patterns in CI
- **eudelins-zama**: Quick fixes, pragmatic solutions

## Anti-Patterns to Avoid

See [references/anti-patterns.md](references/anti-patterns.md).

- Never use `latest` tags in production
- Avoid hardcoded timeouts without justification
- Do not skip security scanning steps

## Architecture

See [references/architecture.md](references/architecture.md).

## Best Practices

See [references/best-practices.md](references/best-practices.md).

- Use reusable workflow patterns
- Pin action versions with SHA
- Use matrix builds for cross-platform testing

## Current Trajectory

See [references/trajectory.md](references/trajectory.md).

**Growing**: GPU-enabled CI runners, production hardening
**Stable**: Core build/test workflows
**Active**: Optimization of CI times

## Upgrade Priorities

See [references/upgrade-priorities.md](references/upgrade-priorities.md).

Focus on GPU CI integration and caching improvements.
