# GitHub Actions / Workflows

This directory contains the CI/CD workflows for the fhevm repository.

## Docker Build Workflows

The repository uses a set of reusable workflows to build and publish Docker images efficiently. The system is designed to:

1. **Build images only when relevant files change** - avoiding unnecessary builds
2. **Re-tag existing images when no changes occur** - ensuring every commit on `main`/`release/vx.y.z` has corresponding Docker image tags without rebuilding
3. **Deterministic build cancellation** - ensuring that when multiple PRs are merged simultaneously on `main`/`release/vx.y.z`, only the latest commit's workflow runs (see [Deterministic Build Cancellation](#deterministic-build-cancellation))

### Architecture Overview

```
┌─────────────────────────────────────┐
│  *-docker-build.yml                 │  (caller workflow, e.g., coprocessor-docker-build.yml)
│  - Triggered on push/release        │
└───────────────┬─────────────────────┘
                │
     ┌──────────┴──────────┐
     ▼                     ▼
┌─────────────────┐  ┌─────────────────────────────────────┐
│ is-latest-      │  │  check-changes-for-docker-          │
│ commit.yml      │  │  build.yml                          │
│ (push only)     │  │  - Detects file changes             │
│ - Checks if     │  │  - Finds previous image base commit │
│   current SHA   │  │  - Outputs: changes, base-commit    │
│   is latest on  │  └───────────────┬─────────────────────┘
│   branch        │                  │
└────────┬────────┘                  │
         │                           │
         └───────────┬───────────────┘
                     │
                     ▼
        ┌─────────────────────────────┐
        │  build-decisions (optional) │  (job in caller workflow, only for
        │  - Centralizes build logic  │   multi-image workflows)
        │  - Outputs: build/retag/skip│
        └───────────────┬─────────────┘
                        │
         ┌──────────────┼──────────────┐
         ▼              ▼              ▼
┌────────────────┐ ┌──────────────┐ ┌──────┐
│ common-docker  │ │ re-tag-docker│ │ skip │
│ (build)        │ │ -image.yml   │ │      │
└────────────────┘ └──────────────┘ └──────┘
```

### Reusable Workflows

#### `is-latest-commit.yml`

Checks whether the current commit is the latest on the target branch. This enables deterministic build cancellation by allowing workflows to skip execution if a newer commit has been pushed.

**How it works:**

1. Uses `git ls-remote` to fetch the latest commit SHA from the remote branch
2. Compares it with the current workflow's commit SHA (`github.sha`)
3. Outputs `is_latest: true` if they match, `false` otherwise

#### `check-changes-for-docker-build.yml`

Determines whether a Docker image needs to be rebuilt by checking if relevant files have changed since the last commit that has a published Docker image.

**How it works:**

1. On `push` events to `main`/`release/vx.y.z`, it searches through recent commits to find the most recent one that has a published Docker image
2. Uses [dorny/paths-filter](https://github.com/dorny/paths-filter) to check if any relevant files changed between that commit and the current one
3. Outputs whether changes were detected and the base commit for potential re-tagging

#### `re-tag-docker-image.yml`

Creates a new tag for an existing Docker image without rebuilding it.

### Docker Build Workflow Patterns

Each service has its own docker build workflow. There are two patterns depending on the number of images built:

#### Simple Pattern (Single Image)

Used by workflows that build a single image (e.g., `gateway-contracts-docker-build.yml`, `host-contracts-docker-build.yml`, `test-suite-docker-build.yml`). The decision logic is embedded directly in the job `if` conditions:

```yaml
jobs:
  # 1. Check if this is the latest commit (push events only)
  is-latest-commit:
    uses: ./.github/workflows/is-latest-commit.yml
    if: github.event_name == 'push'

  # 2. Check for changes
  check-changes:
    if: github.event_name == 'push' || inputs.is_workflow_call
    uses: ./.github/workflows/check-changes-for-docker-build.yml
    # ... configuration

  # 3. Build with inline decision logic
  build:
    needs: [is-latest-commit, check-changes]
    concurrency:
      group: my-service-build-${{ github.ref_name }}
      cancel-in-progress: true
    if: |
      always()
      && (
        github.event_name == 'release'
        || github.event_name == 'workflow_dispatch'
        || (github.event_name == 'push' && needs.is-latest-commit.outputs.is_latest == 'true' && needs.check-changes.outputs.changes == 'true')
        || (inputs.is_workflow_call && needs.check-changes.outputs.changes == 'true')
      )
    uses: zama-ai/ci-templates/.github/workflows/common-docker.yml@<version>
    # ... build configuration

  # 4. Re-tag with inline decision logic
  re-tag-image:
    needs: [is-latest-commit, check-changes]
    if: |
      always()
      && (
        github.event_name == 'push' && needs.is-latest-commit.outputs.is_latest == 'true' && needs.check-changes.outputs.changes != 'true'
      )
    uses: ./.github/workflows/re-tag-docker-image.yml
    # ... configuration
```

#### Complex Pattern (Multiple Images)

Used by workflows that build multiple images (e.g., `coprocessor-docker-build.yml`, `kms-connector-docker-build.yml`). A centralized `build-decisions` job computes the action for each service to avoid duplicating decision logic:

```yaml
jobs:
  # 1. Check if this is the latest commit (push events only)
  is-latest-commit:
    uses: ./.github/workflows/is-latest-commit.yml
    if: github.event_name == 'push'

  # 2. Check for changes for each service
  check-changes-service-a:
    uses: ./.github/workflows/check-changes-for-docker-build.yml
    # ... configuration

  check-changes-service-b:
    uses: ./.github/workflows/check-changes-for-docker-build.yml
    # ... configuration

  # 3. Centralized decision logic for all services
  build-decisions:
    runs-on: ubuntu-latest
    if: always()
    needs: [is-latest-commit, check-changes-service-a, check-changes-service-b]
    outputs:
      service_a: ${{ steps.decide.outputs.service_a }}
      service_b: ${{ steps.decide.outputs.service_b }}
    steps:
      # ... decide which images need to be built

  # 4. Build if decision is "build"
  build-service-a:
    needs: build-decisions
    concurrency:
      group: service-a-build-${{ github.ref_name }}
      cancel-in-progress: true
    if: always() && needs.build-decisions.outputs.service_a == 'build'
    uses: zama-ai/ci-templates/.github/workflows/common-docker.yml@<version>
    # ... build configuration

  # 5. Re-tag if decision is "retag"
  re-tag-service-a-image:
    needs: [build-decisions, check-changes-service-a]
    if: always() && needs.build-decisions.outputs.service_a == 'retag'
    uses: ./.github/workflows/re-tag-docker-image.yml
    # ... configuration
```

### Deterministic Build Cancellation

When multiple PRs are merged to `main` in quick succession, GitHub's concurrency groups cannot guarantee which workflow will "win" - the ordering is arbitrary. This could result in an older commit's workflow completing while a newer commit's workflow gets cancelled.

To solve this, the workflows now use a **deterministic cancellation** approach:

1. **`is-latest-commit.yml`** checks at runtime if the current commit is still the latest on the branch
2. If the commit is the latest: proceed with build or retag. If a newer commit exists: skip all work.

This is used only if the docker build workflow is triggered by a push on `main`!

This ensures that only the workflow for the most recent commit on `main` will actually build or retag images, regardless of the order in which GitHub starts the workflows.

**Note:** Concurrency groups are still used on individual build jobs to prevent duplicate builds of the same service, but the `is-latest-commit` check handles the cross-workflow coordination.
