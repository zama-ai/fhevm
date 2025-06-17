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

# Run specific tests
./fhevm-cli test input-proof
# Trivial
./fhevm-cli test user-decryption
# Trivial
./fhevm-cli test public-decryption
./fhevm-cli test erc20

# Upgrade a specific service
./fhevm-cli upgrade coprocessor

# View logs
./fhevm-cli logs relayer

# Clean up
./fhevm-cli clean
```

### Forcing Local Builds (`--build`)

The `fhevm-cli` script is configured to use specific versions for Docker images for each service. While the default `./fhevm-cli deploy` command would typically attempt to pull these images, **please note that these pre-built Docker images are currently hosted in a private registry and are not publicly available for direct pulling.**

Therefore, for external developers or anyone setting up the stack for the first time without access to our private registry, **using the `--build` option is the recommended and necessary way to get started:**

```sh
./fhevm-cli deploy --build
```

This command instructs Docker Compose to:
1.  Build the images locally using the `Dockerfile` and context specified in the respective `docker-compose/*.yml` files for each service. This process uses the source code available in your local checkout (or cloned sub-repositories).
2.  Tag the newly built images with the versions specified in the `fhevm-cli` script.
3.  Then, start the services using these freshly built local images.

**Why `--build` is essential for external developers:**
*   **Image Access:** Since pre-built images are private, `--build` allows you to construct the necessary images from the publicly available source code.
*   **Local Modifications:** If you have made local changes to any of the Dockerfiles or the build context of a service (e.g., you've cloned one of the sub-repositories like `fhevm-contracts` or `fhevm-coprocessor` into the expected relative paths and made changes), `--build` ensures these changes are incorporated.
*   **Ensuring Correct Setup:** It guarantees that you are running with images built directly from the provided source, eliminating discrepancies that could arise from attempting to pull non-existent or inaccessible public images.

In summary, until public images are made available, external users should always use `./fhevm-cli deploy --build` to ensure a successful deployment.

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
