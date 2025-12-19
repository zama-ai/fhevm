# Supporting Infrastructure 📦

**Location**: Various directories
**Status**: Infrastructure / Maintenance
**Purpose**: Deployment, testing, and operational tooling

## Overview

Supporting infrastructure enables development, testing, and deployment of the FHEVM stack. While not part of the core protocol, these components are essential for operating and maintaining FHEVM systems.

## Directories

| Directory | Purpose |
|-----------|---------|
| `/charts/` | Helm charts for Kubernetes deployment |
| `/test-suite/` | E2E integration tests with docker-compose |
| `/golden-container-images/` | Base Docker images for Node.js and Rust |
| `/docs/` | Gitbook documentation source |
| `/sdk/` | Rust SDK for building applications |
| `/ci/` | CI/CD pipeline configurations |

## Testing Infrastructure

### Test Types

**Unit/Integration Tests:**
- **Hardhat tests**: Located in `host-contracts/test/`, `library-solidity/test/`
- **Foundry tests**: Solidity-native tests via `forge test`
- Written in TypeScript (Hardhat) or Solidity (Foundry)

**E2E Tests:**
- **Full-stack tests**: Located in `test-suite/`
- **Orchestration**: Docker Compose brings up entire stack
- **Mock FHE**: SQLite-backed mocking for fast testing without real FHE

### Mock FHE System

For development and testing, FHEVM includes a mock FHE system:
- Replaces expensive FHE operations with simple encryption
- Backed by SQLite for ciphertext storage
- Enables fast iteration without coprocessor
- Maintains API compatibility with real FHE

### Key Test Files

- `test-suite/docker-compose.yml` - Full stack orchestration
- `host-contracts/hardhat.config.ts` - Hardhat configuration
- `library-solidity/foundry.toml` - Foundry configuration

## Deployment Infrastructure

### Kubernetes / Helm

Located in `/charts/`:
- Helm charts for deploying FHEVM components
- Kubernetes manifests and configurations
- Emerging as primary deployment method

### Docker Images

Located in `/golden-container-images/`:
- Base images for Node.js services
- Base images for Rust services
- Standardized build environments

## Development SDK

Located in `/sdk/`:
- Rust SDK for building FHEVM applications
- Currently in maintenance mode (low activity)
- Provides client libraries for interacting with FHEVM

## CI/CD

Located in `/.github/workflows/`:
- GitHub Actions workflow definitions
- Automated testing on PR
- Build and publish pipelines
- Security scanning

## Key Files

- `test-suite/docker-compose.yml` - Full stack orchestration
- `host-contracts/hardhat.config.ts` - Hardhat configuration
- `.github/workflows/` - CI pipeline definitions

## Areas for Deeper Documentation

**[TODO: Testing infrastructure deep-dive]** - Document the mock FHE system, test fixtures, E2E testing patterns, and how to write effective tests for confidential contracts.

**[TODO: Deployment guide]** - Document the Helm charts, Kubernetes deployment process, configuration options, and operational best practices.

**[TODO: Docker compose stack]** - Detail the test-suite docker-compose setup, how components interact, and how to debug issues in local development.

**[TODO: CI/CD pipeline]** - Explain the GitHub Actions workflows, testing strategy, and release process.

---

**Related:**
- [Component Health](../component-health.md) - Infrastructure components have minimal activity
- [Technology Stack](../reference/tech-stack.md) - Tools and frameworks used
