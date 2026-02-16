## Introduction

This repository provides a docker based setup to locally run an integration of Zama FHEVM and Zama KMS (Key Management System).

For overview of the system, architecture and details on individual components, refer to our [documentation](https://docs.zama.ai/fhevm).

## Main features

KMS can be configured to two modes:

- Centralized
- Threshold

## Table of contents

- [Introduction](#introduction)
- [Main Features](#main-features)
- [Get Started](#get-started)
  - [Quickstart](#quickstart)
  - [Forcing Local Builds](#wip---forcing-local-builds---build)
  - [Local Developer Optimizations](#local-developer-optimizations)
  - [CLI Reference](#cli-reference)
  - [Telemetry Checks](#telemetry-checks)
  - [Resuming a Deployment](#resuming-a-deployment)
  - [Deploying a Single Step](#deploying-a-single-step)
  - [Orchestration Source of Truth](#orchestration-source-of-truth)
  - [Troubleshooting Deploy Failures](#troubleshooting-deploy-failures)
  - [Behavior Parity Tests](#behavior-parity-tests)
  - [CLI Parity Diff Tests](#cli-parity-diff-tests)
- [Security Policy](#security-policy)
  - [Handling Sensitive Data](#handling-sensitive-data)
    - [Environment Files](#environment-files)
    - [Development Environment](#development-environment)
    - [Common Sensitive Data](#common-sensitive-data)
  - [Reporting Security Issues](#reporting-security-issues)
- [Support](#support)

## Get started

### Quickstart

The test suite offers a unified CLI for all operations:

```sh
cd test-suite/fhevm

# Deploy the entire stack
./fhevm-cli deploy

# Deploy with local BuildKit cache (disables provenance attestations)
./fhevm-cli deploy --local

# Deploy and fail if telemetry services are not visible in Jaeger
./fhevm-cli deploy --build --telemetry-smoke

# Deploy with versions scraped from the public testnet matrix
./fhevm-cli deploy --network testnet

# Deploy with threshold 2 out of 2 coprocessors (local multicoprocessor mode)
./fhevm-cli deploy --coprocessors 2 --coprocessor-threshold 2

# Resume a failed deploy from a specific step (keeps existing containers/volumes)
./fhevm-cli deploy --resume kms-connector

# Deploy only a single step (useful for redeploying one service)
./fhevm-cli deploy --only coprocessor

# Run specific tests (works for both 1/1 and n/t topologies)
./fhevm-cli test input-proof
# Skip Hardhat compile when artifacts are already up to date
./fhevm-cli test input-proof --no-hardhat-compile
# Trivial
./fhevm-cli test user-decryption
# Trivial
./fhevm-cli test public-decrypt-http-mixed
./fhevm-cli test public-decrypt-http-ebool
./fhevm-cli test erc20

# Upgrade a specific service
./fhevm-cli upgrade coprocessor

# View logs
./fhevm-cli logs relayer

# Clean up
./fhevm-cli clean

# Hard purge for reproducible A/B runs
./fhevm-cli clean --purge
```

If you prefer shorter commands with Bun scripts, you can run the same CLI via:

```sh
cd test-suite/fhevm
bun run deploy --network testnet
bun run test input-proof
bun run telemetry-smoke
bun run clean --purge-local-cache
```

All `clean` purge flags are fhevm-scoped:
- `--purge-images` removes images referenced by fhevm compose services.
- `--purge-build-cache` and `--purge-local-cache` remove local Buildx cache directory (`.buildx-cache` by default, or `FHEVM_BUILDX_CACHE_DIR` if set).

### WIP - Forcing Local Builds (`--build`)

⚠️ **IMPORTANT: THIS FEATURE IS STILL A WORK IN PROGRESS!** ⚠️
We are actively working to optimize caching for local machines and GitHub runners.

🚨 **SECURITY NOTICE:**
The pre-built Docker images for the FHEVM stack are currently hosted in a **private registry** and are **not publicly available** for direct pulling. This is intentional for security reasons.

Therefore, for external developers or anyone setting up the stack for the first time without access to our private registry, **using the `--build` option is the recommended and necessary way to get started:**

```sh
./fhevm-cli deploy --build
```

This command instructs Docker Compose to:

1.  Build the images locally using the `Dockerfile` and context specified in the respective `docker-compose/*.yml` files for each service. This process uses the source code available in your local checkout (or cloned sub-repositories).
2.  Tag the newly built images with the versions specified in the `fhevm-cli` script.
3.  Then, start the services using these freshly built local images.

**Why `--build` is essential for external developers:**

- **Image Access:** Since pre-built images are private, `--build` allows you to construct the necessary images from the publicly available source code.
- **Local Modifications:** If you have made local changes to any of the Dockerfiles or the build context of a service (e.g., you've cloned one of the sub-repositories like `fhevm-contracts` or `fhevm-coprocessor` into the expected relative paths and made changes), `--build` ensures these changes are incorporated.
- **Ensuring Correct Setup:** It guarantees that you are running with images built directly from the provided source, eliminating discrepancies that could arise from attempting to pull non-existent or inaccessible public images.

🚧 **In summary:** Until public images are made available, external users should always use `./fhevm-cli deploy --build` to ensure a successful deployment.

### Local developer optimizations

For faster local iteration, use `--local` to enable a local BuildKit cache (stored under `.buildx-cache/`) and disable default provenance attestations:

```sh
./fhevm-cli deploy --local
```

For code-path validation, prefer `--build --local` so your local changes are rebuilt while keeping warm cache layers.

To align local versions with currently deployed environments, you can ask deploy to scrape the public version dashboard:

```sh
./fhevm-cli deploy --network testnet
./fhevm-cli deploy --network mainnet
```

Notes:
- This is best-effort scraping from the public Grafana dashboard DOM.
- It applies known service version env vars (coprocessor services, kms-connector services, `CORE_VERSION`) before deployment.
- Contract/relayer versions continue to use local defaults unless explicitly overridden.
- If your Chromium path is custom, set `FHEVM_GRAFANA_CHROMIUM_BIN=/path/to/chromium`.
- For deterministic testing, set `FHEVM_GRAFANA_DASHBOARD_HTML_FILE=/path/to/dashboard.html`.

When running tests and you know your Hardhat artifacts are already up to date, you can skip compilation:

```sh
./fhevm-cli test input-proof --no-hardhat-compile
```

### CLI Reference

For agent workflows, prefer explicit command+flag forms from this table.

| Command | Flags | Notes |
| --- | --- | --- |
| `deploy` | `--build` | Build buildable services before `up -d`. |
| `deploy` | `--local` / `--dev` | Enable local BuildKit cache (`.buildx-cache` by default). |
| `deploy` | `--network testnet\|mainnet` | Apply version profile from public dashboard before deploy. |
| `deploy` | `--resume <step>` | Redeploy from a specific step onward. |
| `deploy` | `--only <step>` | Redeploy only one step. |
| `deploy` | `--telemetry-smoke` | Run Jaeger service smoke-check after deployment. |
| `deploy` | `--strict-otel` | Fail if OTEL endpoint expects Jaeger and Jaeger is not running. |
| `test` | `-n, --network <name>` | Test-runtime network selection (default: `staging`). |
| `test` | `-g, --grep <pattern>` | Override test grep pattern. |
| `test` | `-v, --verbose` | Verbose test output. |
| `test` | `-r, --no-relayer` | Disable Rust relayer in tests. |
| `test` | `--no-hardhat-compile` | Skip compile when artifacts are already up-to-date. |
| `clean` | `--purge` | Shorthand for all purge flags below. |
| `clean` | `--purge-images` | Remove images referenced by fhevm compose services only. |
| `clean` | `--purge-build-cache` | Remove local Buildx cache dir (`.buildx-cache` or `FHEVM_BUILDX_CACHE_DIR`). |
| `clean` | `--purge-networks` | Remove `fhevm_*` networks. |
| `clean` | `--purge-local-cache` | Remove local Buildx cache dir (`.buildx-cache` or `FHEVM_BUILDX_CACHE_DIR`). |
| `pause` / `unpause` | `host` or `gateway` | Contract pause controls. |
| `upgrade` | `<service>` | Restart selected service compose stack. |
| `logs` | `<service>` | Stream container logs for one service. |
| `telemetry-smoke` | _none_ | Validate required Jaeger services are present. |

Notes:
- `--network` on `deploy` selects a **version profile** (`testnet`/`mainnet`).
- `--network` on `test` selects a **test runtime network** (default `staging`).
- They are intentionally different and command-scoped.

### Resuming a deployment

If a deploy fails mid-way, you can resume from a specific step without tearing down containers or regenerating `.env` files:

```sh
./fhevm-cli deploy --resume kms-connector
```

Resume steps (in order):
`minio`, `core`, `kms-signer`, `database`, `host-node`, `gateway-node`, `coprocessor`,
`kms-connector`, `gateway-mocked-payment`, `gateway-sc`, `host-sc`, `relayer`, `test-suite`.

When resuming:
- Services **before** the resume step are preserved (containers + volumes kept)
- Services **from** the resume step onwards are torn down and redeployed

### Deploying a single step

To redeploy only a single service without touching others:

```sh
./fhevm-cli deploy --only coprocessor
```

This is useful when you need to restart or rebuild just one component. Only the specified step's containers are torn down and redeployed; all other services remain untouched.

You can combine `--only` or `--resume` with other flags:

```sh
# Redeploy only gateway-sc with a local build
./fhevm-cli deploy --only gateway-sc --build --local
```

### Telemetry Checks

The coprocessor env now ensures `OTEL_EXPORTER_OTLP_ENDPOINT` is present.
If it is missing, deploy defaults it to `http://jaeger:4317` in `.env.coprocessor.local`.

Use strict endpoint validation (requires Jaeger to be up first):

```sh
./fhevm-cli deploy --strict-otel
```

Run smoke validation on demand:

```sh
./fhevm-cli telemetry-smoke
```

`telemetry-smoke` retries for a short warm-up window before failing, to reduce false negatives while traces are still starting up.

Or include it in deploy:

```sh
./fhevm-cli deploy --telemetry-smoke
```

### Orchestration source of truth

The orchestration is Bun-first with shell entrypoint wrappers:

- `test-suite/fhevm/fhevm-cli`
- `test-suite/fhevm/scripts/deploy-fhevm-stack.sh`

Canonical deploy/test metadata now lives in one TypeScript source:

- `test-suite/fhevm/scripts/bun/manifest.ts`

Runtime implementation:

- `test-suite/fhevm/scripts/bun/cli.ts`
- `test-suite/fhevm/scripts/bun/process.ts`

Compatibility snapshots used for parity verification:

- `test-suite/fhevm/fhevm-cli.legacy`
- `test-suite/fhevm/scripts/deploy-fhevm-stack.legacy.sh`

You can force legacy mode explicitly with:

```sh
FHEVM_CLI_IMPL=legacy ./fhevm-cli deploy
```

Version updates do not require editing many per-service vars manually.
You can override them in one place:

- `FHEVM_STACK_VERSION` (gateway/host/coprocessor/kms-connector/test-suite)
- `FHEVM_CORE_VERSION`
- `FHEVM_RELAYER_VERSION` (relayer + relayer-migrate)

These can be set as environment variables, or in an optional file:

- `test-suite/fhevm/env/staging/.env.versions`
- (template: `test-suite/fhevm/env/staging/.env.versions.example`)

Example:

```sh
FHEVM_STACK_VERSION=v0.12.0-rc.1 \
FHEVM_CORE_VERSION=v0.14.0-rc.1 \
FHEVM_RELAYER_VERSION=v0.10.0-rc.1 \
./fhevm-cli deploy
```

### Troubleshooting deploy failures

When deploy fails, the script now surfaces explicit hints for common operational failure modes.

- OOM-killed critical service:
  - Symptom: failure includes `looks OOM-killed`.
  - Action: increase Docker memory and resume from the failed step, for example:
    - `./fhevm-cli deploy --resume coprocessor`

- Key bootstrap / CRS not ready:
  - Symptom: failure includes `Detected key-bootstrap-not-ready state`.
  - Action: wait for keygen/CRS generation to settle, then resume from gateway contracts:
    - `./fhevm-cli deploy --resume gateway-sc`

- Gateway helper image export conflict (`already exists`):
  - Symptom: build fails while starting gateway contracts.
  - Action: deploy now auto-retries once after removing conflicting `gateway-contracts` tags.
  - Manual fallback for repeated collisions:
    - `./fhevm-cli clean --purge-images --purge-build-cache`

### Behavior parity tests

A behavior-level shell test suite validates deploy orchestration outcomes (ordering, `--resume`, `--only`, build semantics, env patch timing, actionable failure hints, strict OTEL checks, purge flags, and telemetry smoke checks).

Run it with:

```sh
./test-suite/fhevm/scripts/tests/deploy-fhevm-stack.behavior.sh
```

### CLI parity diff tests

A dry-run parity harness executes legacy Bash and Bun CLI flows under the same mocked Docker environment, then diffs command traces and exit codes for sampled command cases.

Run it with:

```sh
./test-suite/fhevm/scripts/tests/fhevm-cli-parity-diff.sh
```

## Security policy

### Handling sensitive data

This document outlines security best practices for the FHEVM project, particularly regarding the handling of sensitive configuration data.

#### Environment files

Our repository contains example environment files `env/staging` that include sensitive values like private keys, mnemonics, and API keys. **These values are for testing purposes only** and should never be used in production environments.

For production deployments:

- **Do not** use the same keys, passwords, or mnemonics that appear in the example files
- **Do not** commit actual production secrets to any repository
- **Do** use a proper secrets management solution:
  - Environment variables managed by your deployment platform
  - HashiCorp Vault or similar secrets management service
  - AWS Secrets Manager, GCP Secret Manager, or Azure Key Vault
  - Kubernetes Secrets (with proper encryption)

Example of replacing sensitive data in production:

```bash
# Replace test mnemonic with environment variable reference
# TEST: MNEMONIC=coyote sketch defense hover finger envelope celery urge panther venue verb cheese
MNEMONIC=${PRODUCTION_MNEMONIC}

# Replace test private key with key stored in a secure vault
# TEST: TX_SENDER_PRIVATE_KEY=0x8f82b3f482c19a95ac29c82cf048c076ed0de2530c64a73f2d2d7d1e64b5cc6e
TX_SENDER_PRIVATE_KEY=${SECURE_PRIVATE_KEY}
```

#### Development environment

When developing locally:

- Use `.env.local` files (added to `.gitignore`) for your personal secrets
- Rotate keys regularly, especially if used for shared development environments
- Consider using environment-specific configuration files (dev, staging, prod)
- Use fake/test data for local development whenever possible

#### Common sensitive data

The following values should NEVER be committed to repositories:

- Private keys
- Mnemonics
- API keys
- Database credentials
- JWT secrets

### Reporting security issues

Please report security vulnerabilities to `security@zama.ia` rather than creating public issues.

Include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested mitigation (if any)

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../docs/.gitbook/assets/support-banner-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="../docs/.gitbook/assets/support-banner-light.png">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.
