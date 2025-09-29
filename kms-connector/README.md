## Introduction

**KMS Connector** connects the KMS Core with the FHEVM Gateway smart contracts.

## Main features

- Forward requests coming from the Gateway to the KMS Core
- Forward responses coming from the KMS Core to the Gateway via transaction
- The KMS Connector is composed of three Rust micro-services:
  - `GatewayListener`: listens and stores the Gateway events into a database
  - `KmsWorker`: forwards requests from a database to the KMS Core, and stores responses into the database
  - `TransactionSender`: sends the responses from the database to the Gateway via transactions

## Table of Contents
- [Introduction](#introduction)
- [Main features](#main-features)
- [Getting Started](#getting-started)
- [Configuration](#configuration)
  - [Configuration Precedence](#configuration-precedence)
  - [S3 Configuration](#s3-configuration)
  - [Wallet Configuration](#wallet-configuration)
    - [Security Considerations](#security-considerations)
- [Architecture](#architecture)
- [Support](#support)

## Getting Started

All services consist of a single binary. The service can be started via its `start` subcommand, with or without a config file (but environment variables would then need to be used instead):

```bash
./gw-listener start --config config/gw-listener.toml

KMS_CONNECTOR_DATABASE_URL="postgres://postgres:postgres@localhost" ./gw-listener start
```

## Configuration

The KMS Connector supports flexible configuration through both TOML files and environment variables. You can use either method or combine them, with environment variables taking precedence over file-based configuration.

See the [configuration examples](./config) for each service, which document all the fields of the configuration with the associated environment variable, as well as its default value.

### Configuration Precedence

The configuration values are loaded in the following order, with later sources overriding earlier ones:

1. Default values (lowest priority)
2. TOML config file (if provided)
3. Environment variables (highest priority)

### S3 Configuration

The KMS Connector retrieves ciphertexts from S3-compatible storage. The connector relies on the S3 URLs provided by the Gateway's events being properly formatted. If no URLs are provided, it will fall back to the optional S3 configured values.

### Wallet Configuration

The KMS Connector supports two methods for configuring the wallet used for signing decryption responses:
- Private key directly as a hex string
- AWS KMS Wallet

The connector will attempt to initialize the wallet in the following order:

1. Private key string (if provided)
2. AWS KMS configuration (if provided)

At least one of these two options must be provided.

#### Security Considerations

- Private keys provided as strings are supported for development purposes, but should not be used in production. These keys should be handled with extreme caution to avoid exposure

## Architecture

See the [architecture documentation](./docs/architecture.md) for more detail.

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../docs/.gitbook/assets/support-banner-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="../docs/.gitbook/assets/support-banner-light.png">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.

[![GitHub stars](https://img.shields.io/github/stars/zama-ai/fhevm?style=social)](https://github.com/zama-ai/fhevm/)
