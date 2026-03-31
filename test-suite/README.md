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
  - [Resuming a Deployment](#resuming-a-deployment)
  - [Deploying a Single Step](#deploying-a-single-step)
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

# Deploy the Solana-host stack
./fhevm-cli deploy --local --solana

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

# Solana host compatibility e2e canaries
./fhevm-cli test solana-input-proof
./fhevm-cli test solana-user-decryption
./fhevm-cli test solana-public-decrypt-http-ebool
./fhevm-cli test solana-public-decrypt-http-mixed

# Upgrade a specific service
./fhevm-cli upgrade coprocessor

# View logs
./fhevm-cli logs relayer

# Clean up
./fhevm-cli clean
```

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

When running tests and you know your Hardhat artifacts are already up to date, you can skip compilation:

```sh
./fhevm-cli test input-proof --no-hardhat-compile
```

The Solana e2e commands do not use Hardhat. They run against the locally deployed stack, but with
the host side deployed in Solana mode:

- use `./fhevm-cli deploy --local --solana`
- the Anvil `host-node` step is replaced by a managed Solana local validator
- the EVM `coprocessor-host-listener` path is replaced by the new `solana-host-listener`
- the gateway, relayer, and the rest of the coprocessor remain in the normal deployed stack
- the Solana canaries then run against the Solana programs deployed on that validator

The current Solana-host deploy mode intentionally skips two EVM-only pieces:

- `host-sc`, because the host functionality is provided by the Solana programs instead of the EVM host contracts
- `kms-connector`, because its current `gw-listener` and `kms-worker` still hard-depend on an EVM host chain, `KMSVerifier`, and EVM ACL addresses

So the Solana-host stack today is:

- Solana validator for the host-chain role
- `coprocessor-host-listener` container running the Solana listener binary
- normal gateway node
- normal coprocessor workers
- normal relayer

Deploy and run the Solana canaries like this:

```sh
./fhevm-cli deploy --local --solana
./fhevm-cli test solana-input-proof
./fhevm-cli test solana-user-decryption
./fhevm-cli test solana-public-decrypt-http-ebool
./fhevm-cli test solana-public-decrypt-http-mixed
```

If the external Solana validator is not reachable when a Solana canary starts, the runner will
bootstrap it for the duration of that test run so the deployed stack can still be exercised:

```sh
./fhevm-cli test solana-input-proof
```

For local Solana runs, the listener is exercised with `confirmed` commitment in these canaries
because the local validator does not reliably advance `finalized` during this workflow. The
standalone Solana listener still defaults to `finalized`.

These Solana commands are compatibility canaries:

- `solana-input-proof` now runs through a Solana `TestInput`-style wrapper program, analogous to
  the Solidity `TestInput` contract. It mirrors `requestUint64NonTrivial`: `VerifyInput`, then a
  durable `Allow` on the verified handle. The Solana listener ingests that ACL event successfully.
  `VerifyInput` itself is still skipped by the listener, which is acceptable for this dapp-style
  flow because the durable ACL event is the downstream signal that matters.
- `solana-user-decryption` shows the current relayer user-decryption API still validates
  contract/user identifiers as Ethereum `0x` addresses, which blocks native Solana program/account
  IDs even though the Solana wrapper program now emits the same durable host events as the EVM
  `TestInput` flow.
- `solana-public-decrypt-http-ebool` and `solana-public-decrypt-http-mixed` now also run through
  the Solana wrapper program and make the handles publicly decryptable on the Solana host. The
  remaining blocker is higher in the stack: the relayer rejects the request with
  `host_chain_id_not_supported`, which means Solana host-chain support is not wired into the
  relayer host-chain registry/configuration yet.

The `./fhevm-cli deploy --local --solana` mode currently does not support `--resume` or `--only`.

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
