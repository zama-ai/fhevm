# KMS Connector

KMS Connector is a Rust-based service that connects the KMS Core with the HTTPZ Gateway  (Arbitrum) smart contracts, handling decryption requests and key management operations.

## Features

- Event-driven architecture with MPSC orchestration
- Support for public/user decryption operations
- Key generation with extended finality support
- CRS generation and management
- Operation status notifications
- Arbitrum-specific finality rules
- CLI interface for configuration management and validation
- S3 ciphertext retrieval with configurable endpoint support
- Non-failable S3 URL processing with graceful fallbacks
- Optional S3 configuration for flexible deployment scenarios
- Multiple wallet initialization options (AWS KMS, signing key file, private key, mnemonic)

## CLI Usage

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

# Wallet configuration - one of the following must be provided:
# 1. BIP39 mnemonic phrase for wallet generation
mnemonic = "test test test test test test test test test test test junk"

# 2. Path to a serialized signing key file (relative to execution directory)
signing_key_path = "../keys/CLIENT/SigningKey/e164d9de0bec6656928726433cc56bef6ee8417ad5a4f8c82fbcc2d3e5f220fd"

# 3. Private key as a hex string (with or without 0x prefix)
private_key = "0x0000000000000000000000000000000000000000000000000000000000000001"

# 4. AWS KMS configuration (for using AWS KMS for signing)
[aws_kms_config]
# AWS KMS key ID (required for AWS KMS wallet)
key_id = "alias/my-kms-key"
# AWS region (optional, will use default AWS config if not specified)
region = "us-east-1"
# AWS endpoint URL (optional, for testing or non-standard endpoints)
endpoint = "http://localhost:4566"

# Account index for mnemonic-based wallets (optional, default: 0)
account_index = 0

# KMS Core endpoint (required)
kms_core_endpoint = "http://localhost:50052"

# GateWay L2 WebSocket RPC URL endpoint (required)
gwl2_url = "ws://localhost:8757"

# Chain ID (required)
chain_id = 1337

# Decryption manager contract address (required)
decryption_manager_address = "0x..."

# HTTPZ contract address (required)
httpz_address = "0x..."

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
   export KMS_CONNECTOR_GWL2_URL="ws://localhost:8547"
   export KMS_CONNECTOR_KMS_CORE_ENDPOINT="http://localhost:50052"
   
   # Wallet configuration (one of the following is required)
   export KMS_CONNECTOR_MNEMONIC="your mnemonic here"
   # OR
   export KMS_CONNECTOR_SIGNING_KEY_PATH="/path/to/signing/key"
   # OR
   export KMS_CONNECTOR_PRIVATE_KEY="0x0000000000000000000000000000000000000000000000000000000000000001"
   # OR for AWS KMS
   export KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID="alias/my-kms-key"
   export KMS_CONNECTOR_AWS_KMS_CONFIG__REGION="us-east-1"
   export KMS_CONNECTOR_AWS_KMS_CONFIG__ENDPOINT="http://localhost:4566"
   
   export KMS_CONNECTOR_CHAIN_ID="31337"
   export KMS_CONNECTOR_DECRYPTION_MANAGER_ADDRESS="0x..."
   export KMS_CONNECTOR_HTTPZ_ADDRESS="0x..."

   # Optional configuration with defaults
   export KMS_CONNECTOR_ACCOUNT_INDEX="0"
   export KMS_CONNECTOR_CHANNEL_SIZE="1000"
   export KMS_CONNECTOR_SERVICE_NAME="kms-connector"
   export KMS_CONNECTOR_DECRYPTION_TIMEOUT_SECS="300"
   export KMS_CONNECTOR_REENCRYPTION_TIMEOUT_SECS="300"
   export KMS_CONNECTOR_RETRY_INTERVAL_SECS="5"

   # S3 configuration (optional)
   # Note the double underscore (__) for nested configuration
   export KMS_CONNECTOR_S3_CONFIG__REGION="us-east-1"
   export KMS_CONNECTOR_S3_CONFIG__BUCKET="my-ciphertext-bucket"
   export KMS_CONNECTOR_S3_CONFIG__ENDPOINT="http://localhost:9876"

   > **Note on Nested Configuration**: For nested configuration structures like `s3_config` and `aws_kms_config`, use double underscores (`__`) in environment variables to represent the nesting. For example, `s3_config.region` in TOML becomes `KMS_CONNECTOR_S3_CONFIG__REGION` as an environment variable.

   # Start the connector without a config file
   cargo run --bin kms-connector start
   ```

2. **Config File Only**

   ```bash
   # Use a TOML config file
   cargo run --bin kms-connector start --config ./config/environments/config-base.toml
   ```

3. **Combined Configuration**

   ```bash
   # Set specific overrides
   export KMS_CONNECTOR_GWL2_URL="ws://localhost:8547"
   export KMS_CONNECTOR_CHAIN_ID="31337"

   # Use config file for other values
   cargo run --bin kms-connector start --config ./config/environments/config-base.toml
   ```

### Configuration Precedence

The configuration values are loaded in the following order, with later sources overriding earlier ones:

1. Default values (lowest priority)
2. TOML config file (if provided)
3. Environment variables (highest priority)

### Default Values

When neither environment variables nor config file values are provided, the following defaults are used:

```toml
gwl2_url = "ws://localhost:8545"
kms_core_endpoint = "http://[::1]:50052"
chain_id = 31337
decryption_manager_address = "0x5fbdb2315678afecb367f032d93f642f64180aa3"
httpz_address = "0x0000000000000000000000000000000000000001"
channel_size = 1000
service_name = "kms-connector"
decryption_timeout_secs = 300
reencryption_timeout_secs = 300
retry_interval_secs = 5
```

### List Of Environment Variables

All environment variables are prefixed with `KMS_CONNECTOR_`. Here's the complete list:

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `KMS_CONNECTOR_ACCOUNT_INDEX` | Account index for the wallet | 0 |
| `KMS_CONNECTOR_GWL2_URL` | Gateway L2 WebSocket URL | ws://localhost:8545 |
| `KMS_CONNECTOR_KMS_CORE_ENDPOINT` | KMS Core service endpoint | http://[::1]:50052 |
| `KMS_CONNECTOR_MNEMONIC` | Wallet mnemonic phrase | (required if other wallet options not provided) |
| `KMS_CONNECTOR_SIGNING_KEY_PATH` | Path to a serialized signing key file | (optional) |
| `KMS_CONNECTOR_PRIVATE_KEY` | Private key as a hex string | (optional) |
| `KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID` | AWS KMS key ID | (optional) |
| `KMS_CONNECTOR_AWS_KMS_CONFIG__REGION` | AWS region for KMS | (optional) |
| `KMS_CONNECTOR_AWS_KMS_CONFIG__ENDPOINT` | AWS endpoint URL for KMS | (optional) |
| `KMS_CONNECTOR_CHAIN_ID` | Blockchain network chain ID | 31337 |
| `KMS_CONNECTOR_DECRYPTION_MANAGER_ADDRESS` | Address of the Decryption Manager contract | 0x5fbdb2315678afecb367f032d93f642f64180aa3 |
| `KMS_CONNECTOR_HTTPZ_ADDRESS` | Address of the HTTPZ contract | 0x0000000000000000000000000000000000000001 |
| `KMS_CONNECTOR_CHANNEL_SIZE` | Size of the event processing channel | 1000 |
| `KMS_CONNECTOR_SERVICE_NAME` | Name of the KMS connector instance | kms-connector |
| `KMS_CONNECTOR_DECRYPTION_TIMEOUT_SECS` | Timeout for decryption operations | 300 |
| `KMS_CONNECTOR_REENCRYPTION_TIMEOUT_SECS` | Timeout for re-encryption operations | 300 |
| `KMS_CONNECTOR_RETRY_INTERVAL_SECS` | Interval between retry attempts | 5 |
| `KMS_CONNECTOR_DECRYPTION_MANAGER_DOMAIN_NAME` | EIP-712 domain name for DecryptionManager contract | DecryptionManager |
| `KMS_CONNECTOR_DECRYPTION_MANAGER_DOMAIN_VERSION` | EIP-712 domain version for DecryptionManager contract | 1 |
| `KMS_CONNECTOR_HTTPZ_DOMAIN_NAME` | EIP-712 domain name for HTTPZ contract | HTTPZ |
| `KMS_CONNECTOR_HTTPZ_DOMAIN_VERSION` | EIP-712 domain version for HTTPZ contract | 1 |
| `KMS_CONNECTOR_PRIVATE_KEY` | Private key as a hex string | (optional) |
| `KMS_CONNECTOR_VERIFY_COPROCESSORS` | Whether to verify coprocessors against HTTPZ contract | false |
| `KMS_CONNECTOR_S3_CONFIG__REGION` | AWS S3 region for ciphertext storage | (optional) |
| `KMS_CONNECTOR_S3_CONFIG__BUCKET` | AWS S3 bucket name for ciphertext storage | (optional) |
| `KMS_CONNECTOR_S3_CONFIG__ENDPOINT` | AWS S3 endpoint URL for ciphertext storage | (optional) |

> **Note on Nested Configuration**: For nested configuration structures like `s3_config` and `aws_kms_config`, use double underscores (`__`) in environment variables to represent the nesting. For example, `s3_config.region` in TOML becomes `KMS_CONNECTOR_S3_CONFIG__REGION` as an environment variable.

### Best Practices

1. Use a config file for development and testing environments where values change infrequently
2. Use environment variables for production deployments and when values need to be changed dynamically
3. Store sensitive information (like mnemonics) as environment variables rather than in config files

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

The connector supports multiple S3 URL formats:

1. **Virtual-hosted style**: `bucket-name.s3.region.amazonaws.com`
2. **Path-style**: `s3.region.amazonaws.com/bucket-name`
3. **Custom endpoints**: with region and bucket in path segments

The system will extract the region, endpoint URL, and bucket name directly from the URL when possible. If URL parsing fails, it will gracefully fall back to the configured values.

### Non-Failable Design

The S3 URL processing is designed to be non-failable:

- Returns `Option<(String, String, String)>` instead of `Result`
- Uses warning logs instead of errors for non-critical issues
- Gracefully falls back to provided configuration values
- Continues processing other URLs even when one fails

This approach ensures that temporary issues with S3 URL formats don't disrupt the high-frequency operation of the KMS Connector.

### Optional Configuration

S3 configuration is optional. If not provided, the connector will log warnings but continue operating with limited functionality. This allows for flexible deployment scenarios where S3 retrieval might not be required.

## Wallet Configuration

The KMS Connector supports three methods for configuring the wallet used for signing decryption responses:

### 1. Mnemonic-based Wallet

You can provide a BIP39 mnemonic phrase to generate a deterministic wallet:

```toml
# In config file
mnemonic = "test test test test test test test test test test test junk"
```

```bash
# Or as environment variable
export KMS_CONNECTOR_MNEMONIC="test test test test test test test test test test test junk"
```

### 2. Signing Key File

Alternatively, you can load a serialized signing key from a file:

```toml
# In config file
signing_key_path = "../keys/CLIENT/SigningKey/e164d9de0bec6656928726433cc56bef6ee8417ad5a4f8c82fbcc2d3e5f220fd"
```

```bash
# Or as environment variable
export KMS_CONNECTOR_SIGNING_KEY_PATH="../keys/CLIENT/SigningKey/e164d9de0bec6656928726433cc56bef6ee8417ad5a4f8c82fbcc2d3e5f220fd"
```

The path is relative to the execution directory of the application.

### 3. Private Key String

You can also provide a private key directly as a hex string:

```toml
# In config file
private_key = "8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f"
```

```bash
# Or as environment variable
export KMS_CONNECTOR_PRIVATE_KEY="8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f"
```

The private key can be provided with or without the '0x' prefix.

### 4. AWS KMS Wallet

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

1. Signing key file (if provided)
2. Private key string (if provided)
3. Mnemonic (if provided)
4. AWS KMS configuration (if provided)

At least one of these four options must be provided.

### Security Considerations

- For production environments, it's recommended to use the signing key file approach with proper file permissions
- The signing key file should be securely stored and accessible only to the KMS Connector process
- Private keys provided as strings should be handled with extreme caution to avoid exposure
- In development environments, the mnemonic approach can be more convenient

## Architecture: Adapter-Provider Pattern

The connector uses a two-layer architecture to separate L2 chain interaction from business logic:

```diagram
┌────────────────────┐     ┌────────────────┐
│  DecryptionAdapter │     │  HTTPZAdapter  │
│    <Domain Logic>  │     │ <Domain Logic> │
└────────┬───────────┘     └───────┬────────┘
         │                         │
         │      implements         │
         │          ▼              │
         │    ┌──────────┐         │
         └────┤ Provider ◄─────────┘
              │ Interface│
              └────┬─────┘
                   │      implements
                   ▼
         ┌─────────────────────┐
         │  ArbitrumProvider   │
         │ <L2 Communication>  │
         └─────────┬───────────┘
                   │
                   ▼
        [Arbitrum L2 Contracts]
        DecryptionManager, HTTPZ
```

### 1. Provider (Infrastructure Layer)

```rust
// Provider handles raw L2 interaction
trait Provider {
    async fn send_transaction(&self, to: Address, data: Vec<u8>) -> Result<()>;
    fn decryption_manager_address(&self) -> Address;
    fn httpz_address(&self) -> Address;
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

        // 2. Encode for L2
        let mut data = Vec::new();
        response.encode_data_to(&mut data);

        // 3. Send via provider
        self.provider
            .send_transaction(
                self.provider.decryption_manager_address(),
                data
            )
            .await
    }
}

## Key Points

1. **Provider**
   - Single responsibility: L2 communication
   - Knows addresses but not contract logic
   - Generic transaction sending
   - No business rules

2. **Adapters**
   - Contract-specific logic
   - Event encoding/decoding
   - Business rule validation
   - Uses provider for L2 access

3. **Benefits**
   - Clean separation of L2 access and business logic
   - Easy to mock provider for testing
   - Type-safe contract interaction
   - Reusable L2 connection layer

## Current Status

See [CHANGELOG.md](./changelog.md) for current implementation status.

## Development

### Prerequisites

- Rust 1.86+
- Access to Arbitrum L2 node
- KMS Core instance

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```
