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
# WIP: Build images locally (when private registry not available)
./fhevm-cli deploy --build

# Run blockchain integration tests
./fhevm-cli test input-proof        # Test input proofs
./fhevm-cli test user-decryption    # Test user decryptions
./fhevm-cli test public-decryption  # Test public decryptions  
./fhevm-cli test erc20              # Test ERC20 operations
./fhevm-cli test debug              # Debug mode testing

# Database connector testing (runs gateway-stress via fhevm-cli wrapper)
./fhevm-cli db-test --track-responses            # Test with all DB URLs from config
./fhevm-cli db-test --duration 60s -t mixed      # Run for 60s with mixed requests
./fhevm-cli db-test -b 100 --clear-db            # Batch size 100, clear DB first

# Upgrade specific services
./fhevm-cli upgrade host
./fhevm-cli upgrade gateway
./fhevm-cli upgrade connector
./fhevm-cli upgrade coprocessor
./fhevm-cli upgrade relayer
./fhevm-cli upgrade test-suite

# View logs for any service
./fhevm-cli logs [SERVICE]

# Clean up all containers and volumes
./fhevm-cli clean
```

### Database Connector Testing

The `fhevm-cli db-test` command provides database-level stress testing:

```sh
# Implemented options:
-n, --num-connectors NUM  # Number of DB URLs to use from config
-t, --type TYPE           # Request type: public, user, or mixed
-b, --batch-size SIZE     # Requests per batch
--duration TIME           # Test duration (e.g., 30s, 5m, 1h)
-i, --interval TIME       # Batch interval (e.g., 1s, 500ms, 2s)
-c, --config FILE         # Path to custom config file
--track-responses         # Enable response tracking
--clear-db                # Clear DB tables before test

# Examples:
./fhevm-cli db-test --clear-db --track-responses         # Clear DB, then test with tracking
./fhevm-cli db-test -n 2 -t mixed                        # Test 2 DBs with mixed requests
./fhevm-cli db-test --duration 60s -i 500ms              # 60s test, 500ms intervals
./fhevm-cli db-test -c custom.toml -b 100 --clear-db     # Custom config, clear DB, batch 100

# Default config: test-suite/gateway-stress/config/config.toml
```

### Advanced Gateway Stress Testing

For more control, use the standalone `gateway-stress` tool directly (in `test-suite/gateway-stress`):

1. **Blockchain-based testing** - Sends actual transactions through the blockchain
2. **Database-level testing** - Directly inserts requests into PostgreSQL for focused DB testing

#### Blockchain-based Testing
Sends decryption requests through the blockchain (requires deployed contracts):

```sh
cd test-suite/gateway-stress

# Build the tool first
cargo build --release

# Send public decryption transactions
./target/release/gateway-stress public

# Send user decryption transactions  
./target/release/gateway-stress user

# Note: 'mixed' mode is not yet implemented
```

#### Database-level Testing  
Bypasses blockchain and directly inserts into PostgreSQL databases:

```sh
cd test-suite/gateway-stress

# Build the tool
cargo build --release

# Basic database test with default settings from config
./target/release/gateway-stress db-connector

# Override test duration and request type
./target/release/gateway-stress db-connector --duration 60s --request-type public

# Enable response tracking to verify sync across databases
./target/release/gateway-stress db-connector --track-responses --batch-size 1000

# Use custom configuration
./target/release/gateway-stress --config custom-config.toml db-connector
```

#### Prerequisites
- Rust toolchain (for building gateway-stress)
- For blockchain testing: Deployed FHEVM contracts and configured ct_handles
- For database testing: Running PostgreSQL with credentials in `config/config.toml`

#### DB Connector Options
- `--request-type <TYPE>` - Request type: `public`, `user`, or `mixed`
- `--duration <TIME>` - Test duration (e.g., `30s`, `5m`, `1h`) 
- `--batch-size <NUM>` - Requests per batch (for load control)
- `--track-responses` - Monitor response processing and sync status

#### Configuration File
The tool reads from a TOML configuration file (default: `config/config.toml`):
- Database connection strings
- Batch intervals and sizes
- Connection pool settings
- Request generation parameters

See `gateway-stress/README.md` for detailed configuration examples.

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
*   **Image Access:** Since pre-built images are private, `--build` allows you to construct the necessary images from the publicly available source code.
*   **Local Modifications:** If you have made local changes to any of the Dockerfiles or the build context of a service (e.g., you've cloned one of the sub-repositories like `fhevm-contracts` or `fhevm-coprocessor` into the expected relative paths and made changes), `--build` ensures these changes are incorporated.
*   **Ensuring Correct Setup:** It guarantees that you are running with images built directly from the provided source, eliminating discrepancies that could arise from attempting to pull non-existent or inaccessible public images.

🚧 **In summary:** Until public images are made available, external users should always use `./fhevm-cli deploy --build` to ensure a successful deployment.

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
