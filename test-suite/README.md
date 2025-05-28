# Fhevm Test Suite

## Table of Contents

- **[About](#about)**
- **[Running the demo](#running-the-demo)**
- **[Security Policy](#security-policy)**
- **[FAQ](#faq)**
- **[Support](#support)**

## About

This repository provides a docker based setup to locally run an integration of Zama Fhevm and Zama KMS (Key Management System).

For overview of the system, architecture and details on individual components, refer to our [documentation](https://docs.zama.ai/fhevm).

KMS can be configured to two modes:

- Centralized
- Threshold

## Demo :tada:

### All-in-One CLI

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

## Security Policy

### Handling Sensitive Data

This document outlines security best practices for the fhevm project, particularly regarding the handling of sensitive configuration data.

#### Environment Files

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
#### Development Environment

When developing locally:

- Use `.env.local` files (added to `.gitignore`) for your personal secrets
- Rotate keys regularly, especially if used for shared development environments
- Consider using environment-specific configuration files (dev, staging, prod)
- Use fake/test data for local development whenever possible


#### Common Sensitive Data
The following values should NEVER be committed to repositories:

- Private keys
- Mnemonics
- API keys
- Database credentials
- JWT secrets

### Reporting Security Issues
Please report security vulnerabilities to `security@zama.ia` rather than creating public issues.

Include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested mitigation (if any)


## FAQ

**Is Zama’s technology free to use?**

> Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
> Everything we do is open source and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**

> To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us hello@zama.ai for more information.

**Do you file IP on your technology?**

> Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**

> We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/08656d0a-3f44-4126-b8b6-8c601dff5380">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/1c9c9308-50ac-4aab-a4b9-469bb8c536a4">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.
