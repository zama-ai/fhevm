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
  - [Local Overrides](#local-overrides)
  - [Resuming a Deployment](#resuming-a-deployment)
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

# Preview the resolved bundle and boot plan
./fhevm-cli up --target latest-supported --dry-run
./fhevm-cli up --target sha --sha 9587546 --dry-run

# Boot the stack
./fhevm-cli up --target latest-supported
./fhevm-cli deploy --target latest-supported
./fhevm-cli up --target sha --sha 9587546

# Deploy with threshold 2 out of 2 coprocessors (local multicoprocessor mode)
./fhevm-cli up --target latest-supported --scenario two-of-two

# Deploy with multi-chain (two host chains)
./fhevm-cli up --target latest-supported --scenario multi-chain

# Deploy with multi-chain + multi-coprocessor
./fhevm-cli up --target latest-supported --scenario two-of-two-multi-chain

# Resume a failed deploy from a specific step (keeps existing containers/volumes)
./fhevm-cli up --target latest-supported --resume --from-step kms-connector

# Run specific tests (works for both 1/1 and n/t topologies)
./fhevm-cli test input-proof
./fhevm-cli test user-decryption
./fhevm-cli test public-decrypt-http-mixed
./fhevm-cli test public-decrypt-http-ebool
./fhevm-cli test erc20
./fhevm-cli test hcu-block-cap
./fhevm-cli test multi-chain-isolation  # requires multi-chain scenario

# Boot with a local coprocessor override (all services)
./fhevm-cli up --target latest-supported --override coprocessor

# View logs
./fhevm-cli logs relayer

# Clean up
./fhevm-cli clean
```

For the local CLI entrypoint and architecture, see [test-suite/fhevm/README.md](fhevm/README.md) and [test-suite/fhevm/ARCHITECTURE.md](fhevm/ARCHITECTURE.md).

### Local overrides

To run one local component on top of an otherwise versioned stack, use `--override`:

```sh
# Override an entire group (builds all services locally)
./fhevm-cli up --target latest-supported --override coprocessor

Supported override groups are `coprocessor`, `kms-connector`, `gateway-contracts`, `host-contracts`, and `test-suite`.
Per-service override syntax is supported only for runtime groups: `coprocessor`, `kms-connector`, and `test-suite`.
Local overrides always build release images.
On `latest-supported`, per-service overrides for `coprocessor` and `kms-connector` are rejected by default when local DB migrations diverge from the tracked baseline profile. Use the full-group override, or pass `--allow-schema-mismatch` if you know the mixed stack remains compatible.

When specifying individual services, use the short suffix after the group prefix (e.g., `host-listener` not `coprocessor-host-listener`). Services that share a Docker image are automatically co-selected (e.g., `host-listener` includes `host-listener-poller`).

### Resuming a deployment

If a boot fails mid-way, you can resume from a specific step:

```sh
./fhevm-cli up --target latest-supported --resume --from-step kms-connector
```

Resume steps (in order):
`preflight`, `resolve`, `generate`, `base`, `kms-signer`, `gateway-deploy`, `host-deploy`, `discover`,
`regenerate`, `validate`, `coprocessor`, `kms-connector`, `bootstrap`, `relayer`, `test-suite`.

When resuming:
- Steps before the resume step are preserved
- Steps from the resume step onward are regenerated and restarted under `.fhevm`

## Security policy

### Handling sensitive data

This document outlines security best practices for the FHEVM project, particularly regarding the handling of sensitive configuration data.

#### Environment files

Our repository contains example environment files under `test-suite/fhevm/templates/env/.env.*` that include sensitive values like private keys, mnemonics, and API keys. **These values are for testing purposes only** and should never be used in production environments.

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

Please report security vulnerabilities to `security@zama.ai` rather than creating public issues.

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
