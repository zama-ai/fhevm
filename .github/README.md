# GitHub Actions / Workflows

This directory contains the CI/CD workflows for the fhevm repository.

## Docker Build Workflows

The repository uses a set of reusable workflows to build and publish Docker images efficiently. The system is designed to:

1. **Build images only when relevant files change** - avoiding unnecessary builds
2. **Re-tag existing images when no changes occur** - ensuring every commit on `main` has corresponding Docker image tags without rebuilding

### Architecture Overview

```
┌─────────────────────────────────────┐
│  *-docker-build.yml                 │  (caller workflow, e.g., coprocessor-docker-build.yml)
│  - Triggered on push/release        │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│  check-changes-for-docker-          │  (reusable workflow)
│  build.yml                          │
│  - Detects file changes             │
│  - Finds previous image base commit │
│  - Outputs: changes, base-commit    │
└───────────────┬─────────────────────┘
                │
        ┌───────┴──────────────────────┐
        ▼                              ▼
┌───────────────────────────┐ ┌─────────────────────────┐
│ common-docker.yml (build) │ │ re-tag-docker-image.yml │
│ if changes                │ │ if no changes           │
└───────────────────────────┘ └─────────────────────────┘
```

### Reusable Workflows

#### `check-changes-for-docker-build.yml`

Determines whether a Docker image needs to be rebuilt by checking if relevant files have changed since the last commit that has a published Docker image.

**How it works:**

1. On `push` events to `main`, it searches through recent commits to find the most recent one that has a published Docker image
2. Uses [dorny/paths-filter](https://github.com/dorny/paths-filter) to check if any relevant files changed between that commit and the current one
3. Outputs whether changes were detected and the base commit for potential re-tagging

#### `re-tag-docker-image.yml`

Creates a new tag for an existing Docker image without rebuilding it.

### Docker Build Workflow Pattern

Each service has its own docker build workflow (e.g., `coprocessor-docker-build.yml`, `kms-connector-docker-build.yml`) that follows this pattern:

```yaml
jobs:
  # 1. Check for changes using the reusable workflow
  check-changes-my-service:
    uses: ./.github/workflows/check-changes-for-docker-build.yml
    secrets:
      GHCR_READ_TOKEN: ${{ secrets.GHCR_READ_TOKEN }}
    permissions:
      actions: 'read'
      contents: 'read'
      pull-requests: 'read'
    with:
      caller-workflow-event-name: ${{ github.event_name }}
      caller-workflow-event-before: ${{ github.event.before }}
      docker-image: fhevm/my-service
      filters: |
        my-service:
          - .github/workflows/my-service-docker-build.yml
          - my-service/**

  # 2. Build if changes detected (or on release/workflow_dispatch)
  build-my-service:
    needs: check-changes-my-service
    if: |
      github.event_name == 'release'
      || (github.event_name != 'workflow_dispatch' && needs.check-changes-my-service.outputs.changes == 'true')
      || (github.event_name == 'workflow_dispatch' && inputs.build_my_service)
    uses: zama-ai/ci-templates/.github/workflows/common-docker.yml@<version>
    # ... build configuration

  # 3. Re-tag if no changes detected (push events only)
  re-tag-my-service-image:
    needs: check-changes-my-service
    if: |
      needs.check-changes-my-service.outputs.changes != 'true' && github.event_name == 'push'
    uses: ./.github/workflows/re-tag-docker-image.yml
    with:
      image-name: "fhevm/my-service"
      previous-tag-or-commit: ${{ needs.check-changes-my-service.outputs.base-commit }}
      new-tag-or-commit: ${{ github.event.after }}
```
