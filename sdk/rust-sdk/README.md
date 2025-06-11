# fheVM SDK for Rust

A Rust SDK for interacting with Fully Homomorphic Encryption Virtual Machine (fheVM) networks. This SDK simplifies the process of creating encrypted inputs, creating user EIP712 signature, and handling decryption operations for Gateway.

## Features

- **Encrypted Input Creation**: Generate encrypted inputs with zero-knowledge proofs
- **Multiple Data Types**: Support for various encrypted types (ebool, euint8, euint16, euint32, euint64, euint128, eaddress, euint256)
- **Chain Support**: Compatible with multiple EVM blockchains
- **Flexible Configuration**: Builder pattern for easy SDK configuration
- **EIP-712 Support**: Tools for generating and verifying signatures

## Quick Start

```toml
# Add to your Cargo.toml
[dependencies]
gateway_sdk = "0.1.0"
```

## Implementation Status

The following table shows the current implementation status of features compared to the JavaScript SDK:

| Feature | Status | Notes |
|---------|--------|-------|
| **Encryption Types** | | |
| Boolean (ebool) | ✅ Implemented | `add_bool()` |
| uint8 (euint8) | ✅ Implemented | `add_u8()` |
| uint16 (euint16) | ✅ Implemented | `add_u16()` |
| uint32 (euint32) | ✅ Implemented | `add_u32()` |
| uint64 (euint64) | ✅ Implemented | `add_u64()` |
| uint128 (euint128) | ✅ Implemented | `add_u128()` |
| address (eaddress) | ✅ Implemented | `add_address()` |
| uint256 (euint256) | ✅ Implemented | `add_u256()` |
| bytes64 (ebytes64) | ❌ Missing | Present in JS SDK's `addBytes64()` |
| bytes128 (ebytes128) | ❌ Missing | Present in JS SDK's `addBytes128()` |
| bytes256 (ebytes256) | ❌ Missing | Present in JS SDK's `addBytes256()` |
| **Decryption Operations** | | |
| Public Decrypt | ⚠️ Partial | Function signatures defined but implementation incomplete |
| User Decrypt | ✅ Implemented | Builder pattern with UserDecryptionRequest struct |
| Delegated User Decrypt | ⚠️ Partial | Function signatures defined but implementation incomplete |
| **Key Management** | | |
| Key Generation | ✅ Implemented | Can create new key sets |
| Key Loading | ✅ Implemented | Can load existing keys |
| Key Export | ✅ Implemented | Can save keys to disk |
| Key Import | ✅ Implemented | Can load keys from disk |
| **Other Features** | | |
| EIP-712 Signatures | ✅ Implemented | Ability to compute hash - sign |
| Signature Verification | ✅ Implemented  | Tested |
| Handle Generation | ✅ Implemented | Matches JS implementation |
| Zero-knowledge Proofs | ✅ Implemented | Uses TFHE-rs for ZK proofs |
| Configuration Export | ✅ Implemented | Can export config to YAML |
| CRS Support | ⚠️ Partial | Only support one CRS size |
| Auxiliary Data | ✅ Implemented | Properly generates auxiliary data for ZK proofs |
| Calldata Generation | ⚠️ Partial | Only callback req missing |
| Error Handling | ✅ Implemented | Good error type definitions |
| Logging | ✅ Implemented | Comprehensive logging support |



### Learn with minimal examples


```bash
cargo run --example minimal-sdk-setup
cargo run --example minimal-users-key-generation  
cargo run --example minimal-encrypted-input
cargo run --example minimal-eip712-signing
cargo run --example minimal-user-decryption-request
cargo run --example minimal-user-decryption-response
```


### Basic Usage

```rust
use gateway_sdk::{FhevmSdkBuilder, FhevmError};
use std::path::PathBuf;
use alloy::primitives::address;
use tracing::{Level, info};

fn main() -> Result<(), FhevmError> {
    // Initialize logging
    gateway_sdk::logging::init_from_env(Level::INFO);

    // Create SDK using builder
    let mut sdk = FhevmSdkBuilder::new()
        .with_keys_directory(PathBuf::from("./keys"))
        .with_gateway_chain_id(43113)
        .with_host_chain_id(11155111)  // Ethereum Sepolia
        .with_gateway_contract("Decryption", "0x1234567890123456789012345678901234567890")
        .with_gateway_contract(
            "input-verifier",
            "0x1234567890123456789012345678901234567aaa",
        )
        .with_host_contract("ACL", "0x0987654321098765432109876543210987654321")
        .build()?;

    // Set up test addresses
    let contract_address = address!("0x7777777777777777777777777777777777777777");
    let user_address = address!("0x8888888888888888888888888888888888888888");

    // Create encrypted input
    let mut builder = sdk.create_input_builder()?;
    builder.add_u64(18446744073709550042)?;
    
    // Encrypt for a specific contract and user
    let encrypted = builder.encrypt_and_prove_for(contract_address, user_address)?;
    
    // Use the encrypted data (handles) for blockchain interaction
    info!("Encryption successful!");
    info!("  - Handles: {}", encrypted.handles.len());
    info!("  - Ciphertext size: {} bytes", encrypted.ciphertext.len());

    Ok(())
}
```

## Configuration

The SDK can be configured using the builder pattern:

```rust
let sdk_builder = FhevmSdkBuilder::new()
    .with_keys_directory(PathBuf::from("./keys"))
    .with_gateway_chain_id(43113)
    .with_host_chain_id(11155111)
    .with_gateway_contract("decryption", "0x1234567890123456789012345678901234567890")
    .with_gateway_contract(
            "input-verification",
            "0x1234567890123456789012345678901234567aaa",
        )
    .with_host_contract("ACL", "0x0987654321098765432109876543210987654321");

// Export configuration to YAML
let yaml_config = sdk_builder.to_yaml()?;
```

## Key Management

The SDK provides utilities for generating and managing FHE keys:

```rust
// Generate new keys if they don't exist
let sdk_builder = FhevmSdkBuilder::new()
    .with_keys_directory_or_generate(PathBuf::from("./my_keys"))?
    // Add other configuration...
    .build()?;
```

## Examples

### Encrypting Different Data Types

```rust
let mut builder = sdk.create_input_builder()?;

// Add different types
builder.add_bool(true)?;
builder.add_u8(123)?;
builder.add_u16(12345)?;
builder.add_u32(1234567)?;
builder.add_u64(1234567890)?;
builder.add_u128(123456789012345678901234567890u128)?;
builder.add_address("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")?;
builder.add_u256(my_u256_value)?;

// Encrypt for contract and user
let encrypted = builder.encrypt_and_prove_for(contract_address, user_address)?;
```


## License

[MIT License](LICENSE)

## Acknowledgments

- This SDK uses the TFHE-rs library for homomorphic encryption operations
- Built with support for the Alloy Ethereum library



### License
This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE.txt) for more details.

#### FAQ
**Is Zama’s technology free to use?**
>Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
>Everything we do is open source and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**
>To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us hello@zama.ai for more information.

**Do you file IP on your technology?**
>Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**
>We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.


## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/08656d0a-3f44-4126-b8b6-8c601dff5380">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/1c9c9308-50ac-4aab-a4b9-469bb8c536a4">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.
