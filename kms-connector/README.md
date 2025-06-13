## Introduction

**KMS Connector** is a Rust-based service that connects the KMS Core with the FHEVM Gateway smart contracts, handling decryption requests and key management operations.

## Main features
**Available features**
- Event-driven architecture with MPSC orchestration
- Support for public/user decryption operations
- CLI interface for configuration management and validation
- S3 ciphertext retrieval with configurable endpoint support
- Optional S3 configuration for flexible deployment scenarios
- Multiple wallet initialization options (AWS KMS, private key)

**Upcoming features**

- Key generation with extended finality support
- CRS generation and management
- Arbitrum-specific finality rules
- Operation status notifications

## Table of Contents
- [Introduction](#introduction)
- [Main Features](#main-features)
  - [Available Features](#available-features)
  - [Upcoming Features](#upcoming-features)
- [Get Started](#get-started)
  - [Start a Connector Instance](#start-a-connector-instance)
  - [List Available Configurations](#list-available-configurations)
  - [Validate Configuration](#validate-configuration)
  - [Environment Variables](#environment-variables)
  - [Configuration Structure](#configuration-structure)
- [Configuration](#configuration)
  - [Configuration Methods](#configuration-methods)
  - [Configuration Precedence](#configuration-precedence)
  - [Default Values](#default-values)
  - [List Of Environment Variables](#list-of-environment-variables)
  - [Best Practices](#best-practices)
- [S3 Configuration](#s3-configuration)
  - [Configuration Options](#configuration-options)
  - [S3 URL Processing](#s3-url-processing)
  - [Optional Configuration](#optional-configuration)
- [Wallet Configuration](#wallet-configuration)
  - [1. Private Key String](#1-private-key-string)
  - [2. AWS KMS Wallet](#2-aws-kms-wallet)
  - [Wallet Initialization Priority](#wallet-initialization-priority)
  - [Security Considerations](#security-considerations)
- [Architecture: Adapter-Provider Pattern](#architecture-adapter-provider-pattern)
  - [1. Provider (Infrastructure Layer)](#1-provider-infrastructure-layer)
  - [2. Adapters (Domain Layer)](#2-adapters-domain-layer)
- [Key Points](#key-points)
- [Current Status](#current-status)
- [Development](#development)
  - [Prerequisites](#prerequisites)
  - [Building](#building)
  - [Testing](#testing)
- [Support](#support)


## Get Started

The KMS Connector provides a command-line interface with the following commands:

### Start a Connector Instance

```bash
# Start with a specific config file
kms-connector start -c config/environments/config-1.toml

# Start with a custom service name
kms-connector start -c config/environments/config-1.toml -n "my-connector"

# Use custom config directory (via environment variable)
KMS_CONNECTOR_CONFIG_DIR=/path/to/configs kms-connector start -c config-1.toml
```

### List Available Configurations

```bash
# List configuration filenames
kms-connector list

# List full configuration paths
kms-connector list --full-path
```

### Validate Configuration

```bash
# Validate a specific configuration file
kms-connector validate -c config/environments/config-1.toml
```

### Environment Variables

- `KMS_CONNECTOR_CONFIG_DIR`: Override the default config directory location

### Configuration Structure

Configuration files use TOML format with the following structure:

```toml
# Service name for tracing (optional, default: "kms-connector")
service_name = "my-connector"

# 1. Private key as a hex string (with or without 0x prefix)
private_key = "0x0000000000000000000000000000000000000000000000000000000000000001"

# 2. AWS KMS configuration (for using AWS KMS for signing)
[aws_kms_config]
# AWS KMS key ID (required for AWS KMS wallet)
key_id = "alias/my-kms-key"
# AWS region (optional, will use default AWS config if not specified)
region = "us-east-1"
# AWS endpoint URL (optional, for testing or non-standard endpoints)
endpoint = "http://localhost:4566"

# KMS Core endpoint (required)
kms_core_endpoint = "http://localhost:50052"

# Gateway WebSocket RPC URL endpoint (required)
gateway_url = "ws://localhost:8757"

# Chain ID (required)
chain_id = 1337

# Decryption contract address (required)
decryption_address = "0x..."

# GatewayConfig contract address (required)
gateway_config_address = "0x..."

# Size of the event processing channel (optional)
channel_size = 1000

# S3 configuration for ciphertext storage (optional)
[s3_config]
# AWS S3 region for ciphertext storage
region = "us-east-1"

# AWS S3 bucket name for ciphertext storage
bucket = "my-ciphertext-bucket"

# AWS S3 endpoint URL for ciphertext storage
endpoint = "http://localhost:9876"
```

## Configuration

The KMS Connector supports flexible configuration through both TOML files and environment variables. You can use either method or combine them, with environment variables taking precedence over file-based configuration.

### Configuration Methods

1. **Environment Variables Only**

   ```bash
   # Set required configuration
   export KMS_CONNECTOR_GATEWAY_URL="ws://localhost:8547"
   export KMS_CONNECTOR_KMS_CORE_ENDPOINT="http://localhost:50052"

   # Wallet configuration (one of the following is required)
   export KMS_CONNECTOR_PRIVATE_KEY="0x0000000000000000000000000000000000000000000000000000000000000001"
   # OR for AWS KMS
   export KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID="alias/my-kms-key"
   export KMS_CONNECTOR_AWS_KMS_CONFIG__REGION="us-east-1"
   export KMS_CONNECTOR_AWS_KMS_CONFIG__ENDPOINT="http://localhost:4566"

   export KMS_CONNECTOR_CHAIN_ID="31337"
   export KMS_CONNECTOR_DECRYPTION_ADDRESS="0x..."
   export KMS_CONNECTOR_GATEWAY_CONFIG_ADDRESS="0x..."

   # Optional configuration with defaults
   export KMS_CONNECTOR_CHANNEL_SIZE="1000"
   export KMS_CONNECTOR_SERVICE_NAME="kms-connector"
   export KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS="300"
   export KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS="300"
   export KMS_CONNECTOR_RETRY_INTERVAL_SECS="5"
   export KMS_CONNECTOR_GAS_LIMIT = "6000"

   # S3 configuration (optional)
   # Note the double underscore (__) for nested configuration
   export KMS_CONNECTOR_S3_CONFIG__REGION="us-east-1"
   export KMS_CONNECTOR_S3_CONFIG__BUCKET="my-ciphertext-bucket"
   export KMS_CONNECTOR_S3_CONFIG__ENDPOINT="http://localhost:9876"

   > **Note on Nested Configuration**: For nested configuration structures like `s3_config` and `aws_kms_config`, use double underscores (`__`) in environment variables to represent the nesting. For example, `s3_config.region` in TOML becomes `KMS_CONNECTOR_S3_CONFIG__REGION` as an environment variable.

   # Start the connector without a config file
   cargo run -- start
   ```

2. **Config File Only**

   ```bash
   # Use a TOML config file
   cargo run -- start --config ./config/environments/config-base.toml
   ```

3. **Combined Configuration**

   ```bash
   # Set specific overrides
   export KMS_CONNECTOR_GATEWAY_URL="ws://localhost:8547"
   export KMS_CONNECTOR_CHAIN_ID="31337"

   # Use config file for other values
   cargo run -- start --config ./config/environments/config-base.toml
   ```

### Configuration Precedence

The configuration values are loaded in the following order, with later sources overriding earlier ones:

1. Default values (lowest priority)
2. TOML config file (if provided)
3. Environment variables (highest priority)

### Default Values

When neither environment variables nor config file values are provided, the following defaults are used:

```toml
gateway_url = "ws://localhost:8545"
kms_core_endpoint = "http://[::1]:50052"
chain_id = 31337
decryption_address = "0x5fbdb2315678afecb367f032d93f642f64180aa3"
gateway_config_address = "0x0000000000000000000000000000000000000001"
channel_size = 1000
service_name = "kms-connector"
public_decryption_timeout_secs = 300
user_decryption_timeout_secs = 300
retry_interval_secs = 5
```

### List Of Environment Variables

All environment variables are prefixed with `KMS_CONNECTOR_`. Here's the complete list:

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `KMS_CONNECTOR_GATEWAY_URL` | Gateway WebSocket URL | ws://localhost:8545 |
| `KMS_CONNECTOR_KMS_CORE_ENDPOINT` | KMS Core service endpoint | http://[::1]:50052 |
| `KMS_CONNECTOR_PRIVATE_KEY` | Private key as a hex string | (optional if `KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID` is configured) |
| `KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID` | AWS KMS key ID | (optional if `KMS_CONNECTOR_PRIVATE_KEY` is configured) |
| `KMS_CONNECTOR_AWS_KMS_CONFIG__REGION` | AWS region for KMS | (optional if `KMS_CONNECTOR_PRIVATE_KEY` is configured) |
| `KMS_CONNECTOR_AWS_KMS_CONFIG__ENDPOINT` | AWS endpoint URL for KMS | (optional if `KMS_CONNECTOR_PRIVATE_KEY` is configured) |
| `KMS_CONNECTOR_CHAIN_ID` | Blockchain network chain ID | 31337 |
| `KMS_CONNECTOR_DECRYPTION_ADDRESS` | Address of the Decryption contract | 0x5fbdb2315678afecb367f032d93f642f64180aa3 |
| `KMS_CONNECTOR_GATEWAY_CONFIG_ADDRESS` | Address of the GatewayConfig contract | 0x0000000000000000000000000000000000000001 |
| `KMS_CONNECTOR_CHANNEL_SIZE` | Size of the event processing channel | 1000 |
| `KMS_CONNECTOR_SERVICE_NAME` | Name of the KMS connector instance | kms-connector |
| `KMS_CONNECTOR_PUBLIC_DECRYPTION_TIMEOUT_SECS` | Timeout for public decryption operations | 300 |
| `KMS_CONNECTOR_USER_DECRYPTION_TIMEOUT_SECS` | Timeout for user decryption operations | 300 |
| `KMS_CONNECTOR_RETRY_INTERVAL_SECS` | Interval between retry attempts | 5 |
| `KMS_CONNECTOR_DECRYPTION_DOMAIN_NAME` | EIP-712 domain name for Decryption contract | Decryption |
| `KMS_CONNECTOR_DECRYPTION_DOMAIN_VERSION` | EIP-712 domain version for Decryption contract | 1 |
| `KMS_CONNECTOR_GATEWAY_CONFIG_DOMAIN_NAME` | EIP-712 domain name for GatewayConfig contract | GatewayConfig |
| `KMS_CONNECTOR_GATEWAY_CONFIG_DOMAIN_VERSION` | EIP-712 domain version for GatewayConfig contract | 1 |
| `KMS_CONNECTOR_PRIVATE_KEY` | Private key as a hex string | (optional) |
| `KMS_CONNECTOR_VERIFY_COPROCESSORS` | Whether to verify coprocessors against GatewayConfig contract | false |
| `KMS_CONNECTOR_S3_CONFIG__REGION` | AWS S3 region for ciphertext storage | (optional) |
| `KMS_CONNECTOR_S3_CONFIG__BUCKET` | AWS S3 bucket name for ciphertext storage | (optional) |
| `KMS_CONNECTOR_S3_CONFIG__ENDPOINT` | AWS S3 endpoint URL for ciphertext storage | (optional) |
| `KMS_CONNECTOR_GAS_LIMIT` | Gas limit for each transaction sent to the Gateway | (optional) |

> **Note on Nested Configuration**: For nested configuration structures like `s3_config` and `aws_kms_config`, use double underscores (`__`) in environment variables to represent the nesting. For example, `s3_config.region` in TOML becomes `KMS_CONNECTOR_S3_CONFIG__REGION` as an environment variable.

### Best Practices

1. Use a config file for development and testing environments where values change infrequently
2. Use environment variables for production deployments and when values need to be changed dynamically
3. Store sensitive information (like private keys) as environment variables rather than in config files

## S3 Configuration

The KMS Connector supports retrieving ciphertexts from S3-compatible storage. This functionality is optimized for high-frequency operation (500ms processing cycle).

### Configuration Options

```toml
# S3 configuration for ciphertext storage (optional)
[s3_config]
# AWS S3 region for ciphertext storage
region = "us-east-1"

# AWS S3 bucket name for ciphertext storage
bucket = "my-ciphertext-bucket"

# AWS S3 endpoint URL for ciphertext storage
endpoint = "http://localhost:9876"
```

### S3 URL Processing

The connector relies on the S3 URLs provided by the Gateway's events being properly formatted. If no URLs are provided, it will fall back to the optional S3 configured values.

### Optional Configuration

S3 configuration is optional. If not provided, the connector will log warnings but continue operating with limited functionality. This allows for flexible deployment scenarios where S3 retrieval might not be required.

## Wallet Configuration

The KMS Connector supports three methods for configuring the wallet used for signing decryption responses:

### 1. Private Key String

You can provide a private key directly as a hex string:

```toml
# In config file
private_key = "8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f"
```

```bash
# Or as environment variable
export KMS_CONNECTOR_PRIVATE_KEY="8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f"
```

The private key can be provided with or without the '0x' prefix.

### 2. AWS KMS Wallet

You can also use AWS KMS for signing:

```toml
# In config file
[aws_kms_config]
key_id = "alias/my-kms-key"
region = "us-east-1"
endpoint = "http://localhost:4566"
```

```bash
# Or as environment variable
export KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID="alias/my-kms-key"
export KMS_CONNECTOR_AWS_KMS_CONFIG__REGION="us-east-1"
export KMS_CONNECTOR_AWS_KMS_CONFIG__ENDPOINT="http://localhost:4566"
```

### Wallet Initialization Priority

The connector will attempt to initialize the wallet in the following order:

1. Private key string (if provided)
2. AWS KMS configuration (if provided)

At least one of these two options must be provided.

### Security Considerations

- Private keys provided as strings should be handled with extreme caution to avoid exposure

## Architecture: Adapter-Provider Pattern

The connector uses a two-layer architecture to separate Gateway interaction from business logic:

```diagram
┌────────────────────┐     ┌──────────────────────┐
│  DecryptionAdapter │     │ GatewayConfigAdapter │
│   <Domain Logic>   │     │    <Domain Logic>    │
└────────┬───────────┘     └───────┬──────────────┘
         │                         │
         │      implements         │
         │          ▼              │
         │    ┌──────────┐         │
         └────┤ Provider ◄─────────┘
              │ Interface│
              └────┬─────┘
                   │      implements
                   ▼
         ┌─────────────────────────┐
         │     ArbitrumProvider    │
         │ <Gateway Communication> │
         └─────────┬───────────────┘
                   │
                   ▼
           [Gateway Contracts]
        Decryption, GatewayConfig
```

### 1. Provider (Infrastructure Layer)

```rust
// Provider handles raw Gateway interaction
trait Provider {
    async fn send_transaction(&self, to: Address, data: Vec<u8>) -> Result<()>;
    fn decryption_address(&self) -> Address;
    fn gateway_config_address(&self) -> Address;
}
```

### 2. Adapters (Domain Layer)

```rust
// Adapters implement specific contract logic
struct DecryptionAdapter<P: Provider> {
    provider: P,
    event_tx: Sender<EventFilter>,
}

impl<P: Provider> DecryptionAdapter<P> {
    async fn handle_public_decryption(
        &self,
        id: U256,
        result: Vec<u8>
    ) -> Result<()> {
        // 1. Prepare contract data
        let response = PublicDecryptionResponse {
            id,
            result: result.into()
        };

        // 2. Encode for Gateway
        let mut data = Vec::new();
        response.encode_data_to(&mut data);

        // 3. Send via provider
        self.provider
            .send_transaction(
                self.provider.decryption_address(),
                data
            )
            .await
    }
}
```

## Key Points

1. **Provider**
   - Single responsibility: Gateway communication
   - Knows addresses but not contract logic
   - Generic transaction sending
   - No business rules

2. **Adapters**
   - Contract-specific logic
   - Event encoding/decoding
   - Business rule validation
   - Uses provider for Gateway access

3. **Benefits**
   - Clean separation of Gateway access and business logic
   - Easy to mock provider for testing
   - Type-safe contract interaction
   - Reusable Gateway connection layer

## Current Status

See [CHANGELOG.md](./changelog.md) for current implementation status.

## Development

### Prerequisites

- Rust 1.86+
- Access to a node of the Gateway's chain
- KMS Core instance

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

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
