# FHEVM Gateway SDK for Rust

A Rust SDK for interacting with Fully Homomorphic Encryption Virtual Machine (FHEVM) networks. This SDK simplifies the process of creating encrypted inputs, generating EIP-712 signatures, and handling decryption operations for the Gateway.

## Features

- **Encrypted Input Creation**: Generate encrypted inputs with zero-knowledge proofs
- **Multiple Data Types**: Support for various encrypted types (ebool, euint8-256, eaddress)
- **Decryption Operations**: Handle both public and user-authorized decryption
- **EIP-712 Support**: Generate and verify typed signatures with fluent builder pattern
- **Chain Support**: Compatible with multiple EVM blockchains
- **Builder Pattern**: Intuitive API design for complex operations
- **Multiple Values**: Process multiple encrypted values in single operations

## Disclaimer

This library requires (for now) a private access to kms repository. 

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gateway-sdk = { path = "path/to/gateway-sdk" }
```

## Quick Start

```rust
use gateway_sdk::{FhevmSdkBuilder, Result};
use alloy::primitives::address;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Initialize SDK
    let sdk = FhevmSdkBuilder::new()
        .with_keys_directory(PathBuf::from("./keys"))
        .with_gateway_chain_id(43113)
        .with_host_chain_id(11155111)
        .with_decryption_contract("0x1234567890123456789012345678901234567bbb")
        .with_input_verification_contract("0x1234567890123456789012345678901234567aaa")
        .with_acl_contract("0x0987654321098765432109876543210987654321")
        .build()?;

    // Create encrypted input
    let mut builder = sdk.create_input_builder()?;
    builder.add_u64(42)?;
    builder.add_bool(true)?;
    
    let encrypted = builder.encrypt_and_prove_for(
        address!("0x7777777777777777777777777777777777777777"),
        address!("0x8888888888888888888888888888888888888888")
    )?;
    
    // Generate EIP-712 signature for user decryption
    let signature_result = sdk
        .eip712_builder()
        .public_key("2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331")
        .add_contract("0x7777777777777777777777777777777777777777")?
        .validity_period(1748252823, 30)
        .sign_with("7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f")
        .verify(true)
        .generate_and_sign()?; // ← Recommended method
    
    println!("Encrypted {} values", encrypted.handles.len());
    println!("Signature verified: {}", signature_result.is_verified());
    Ok(())
}
```

## Learn by Example

Run the minimal examples to understand core functionality:

```bash
cargo run --example minimal-sdk-setup
cargo run --example minimal-users-key-generation  
cargo run --example minimal-encrypted-input
cargo run --example minimal-eip712-signing
cargo run --example minimal-user-decryption-request
cargo run --example minimal-user-decryption-response
cargo run --example minimal-public-decryption-request
cargo run --example minimal-public-decryption-response
```

## Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| **Encryption Types** | | |
| Boolean (ebool) | ✅ | `add_bool()` |
| uint8-256 | ✅ | `add_u8()` through `add_u256()` |
| address (eaddress) | ✅ | `add_address()` |
| **Decryption** | | |
| Public Decrypt Request | ✅ | Full builder pattern support |
| Public Decrypt Response | ✅ | Full builder pattern support with EIP-712 signatures verification |
| User Decrypt Request | ✅ | With EIP-712 signatures generation |
| User Decrypt Response | ❌ | Response construction is working but issue using kms client response signature verification |
| Delegated Decrypt | ❌ | Not yet implemented |
| **Key Management** | | |
| Generation/Loading | ✅ | Automatic key management |
| Export/Import | ✅ | File-based storage |
| **Other Features** | | |
| EIP-712 Signatures | ✅ | Generate, sign, verify |
| Handle Generation | ✅ | Matches JS implementation |
| Zero-knowledge Proofs | ✅ | Using TFHE-rs |
| Multiple Values | ✅ | Efficient multi-value processing |

## Core Modules

### Encryption Module
Create encrypted inputs with ZK proofs for multiple data types.

```rust
let mut builder = sdk.create_input_builder()?;
builder
    .add_bool(true)?
    .add_u32(12345)?
    .add_u64(9876543210)?
    .add_address("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")?;

let encrypted = builder.encrypt_and_prove_for(contract_address, user_address)?;
```

### Decryption Module

#### Public Decryption
```rust
// Create request
let calldata = sdk.create_public_decrypt_request_builder()
    .add_handles_from_bytes(&handles)?
    .build_and_generate_calldata()?;

// Process response
let results = sdk.create_public_decrypt_response_builder()
    .kms_signers(signers)
    .threshold(2)
    .gateway_chain_id(54321)
    .verifying_contract_address("0x...")
    .ct_handles(handles)
    .json_response(&response)
    .process()?;
```

#### User Decryption
```rust
// Generate EIP-712 signature with builder pattern
let eip712_result = sdk
    .eip712_builder()
    .public_key(&public_key)
    .add_contract("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1")?
    .validity_period(start_timestamp, duration_days)
    .sign_with(&private_key)
    .verify(true)
    .generate_and_sign()?; // ← Primary recommended method

// Create decrypt request
let request = sdk.create_user_decrypt_request_builder()
    .add_handles_from_bytes(&handles, &contracts)?
    .user_address_from_str("0x...")?
    .signature_from_hex(&eip712_result.require_signature()?)?
    .public_key_from_hex(&public_key)?
    .validity(timestamp, duration)?
    .build_and_generate_calldata()?;
```

### EIP-712 Signature Module
Generate and verify EIP-712 typed signatures with a fluent builder pattern.

```rust
// Generate keypair
let keypair = sdk.generate_keypair()?;

// Method 1: Just generate hash (for manual signing)
let hash = sdk
    .eip712_builder()
    .public_key(&keypair.public_key)
    .add_contract("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1")?
    .validity_period(timestamp, 30)
    .generate_hash()?;

// Method 2: Sign without verification (fast)
let signed_result = sdk
    .eip712_builder()
    .public_key(&keypair.public_key)
    .add_contract("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1")?
    .validity_period(timestamp, 30)
    .sign_with(&private_key)
    .generate_and_sign()?; // ← Recommended for most use cases

// Method 3: Sign with verification (production recommended)
let verified_result = sdk
    .eip712_builder()
    .public_key(&keypair.public_key)
    .add_contracts(&["0x742d...", "0x853d..."])? // Multiple contracts
    .validity_period(timestamp, 30)
    .sign_with(&private_key)
    .verify(true)
    .generate_and_sign()?; // ← Same method, verification enabled

if verified_result.is_verified() {
    println!("✅ Signature verified successfully!");
}
```

#### EIP-712 Builder Methods

| Method | Description |
|--------|-------------|
| `public_key(key)` | Set user's public key for decryption |
| `add_contract(address)` | Add single contract address (accepts &str or Address) |
| `add_contracts(addresses)` | Add multiple contract addresses |
| `validity_period(start, days)` | Set validity period explicitly |
| `starts_now()` | Set start time to current timestamp |
| `valid_for_days(days)` | Set duration with automatic start time |
| `sign_with(private_key)` | Add private key for signing |
| `verify(bool)` | Enable/disable signature verification |
| `generate_hash()` | Generate hash only (for external signing) |
| `generate_and_sign()` | **Recommended**: Generate and sign (respects verification setting) |
| `build()` | Advanced: Build with full control (same as generate_and_sign) |

## Configuration

### Builder Pattern
```rust
let sdk = FhevmSdkBuilder::new()
    .with_keys_directory(path)
    .with_gateway_chain_id(chain_id)
    .with_host_chain_id(chain_id)
    .with_decryption_contract(address)
    .with_input_verification_contract(address)
    .with_acl_contract(address)
    .build()?;
```

### YAML Configuration
```rust
// Export configuration
let yaml_config = sdk_builder.to_yaml()?;

// Load from YAML
let sdk = FhevmSdk::from_yaml_file("config.yaml")?;
```

### Key Management
```rust
// Generate new keys if needed
let sdk = FhevmSdkBuilder::new()
    .with_keys_directory_or_generate(PathBuf::from("./keys"))?
    .build()?;

// Manual key generation
use gateway_sdk::utils::generate_fhe_keyset;
generate_fhe_keyset(Path::new("./keys"))?;
```

## API Reference

### FhevmSdkBuilder

| Method | Description |
|--------|-------------|
| `with_keys_directory(path)` | Set FHE keys directory |
| `with_gateway_chain_id(id)` | Set gateway chain ID |
| `with_host_chain_id(id)` | Set host chain ID |
| `with_*_contract(addr)` | Set contract addresses |
| `build()` | Create SDK instance |

### FhevmSdk Core Methods

| Method | Description |
|--------|-------------|
| `create_input_builder()` | Create encrypted input builder |
| `eip712_builder()` | Create EIP-712 signature builder |
| `create_public_decrypt_request_builder()` | Public decrypt builder |
| `create_user_decrypt_request_builder()` | User decrypt builder |
| `generate_verify_proof_calldata()` | Generate proof verification calldata |
| `generate_keypair()` | Generate cryptobox keypair |

### EncryptedInputBuilder

| Method | Description |
|--------|-------------|
| `add_bool(value)` | Add boolean value |
| `add_u{8,16,32,64,128,256}(value)` | Add unsigned integers |
| `add_address(value)` | Add Ethereum address |
| `encrypt_and_prove_for(contract, user)` | Generate encrypted input |

### User Decryption Builders

#### UserDecryptRequestBuilder
| Method | Description |
|--------|-------------|
| `add_handles_from_bytes(handles, contracts)` | Add encrypted handles |
| `user_address_from_str(address)` | Set user address |
| `signature_from_hex(signature)` | Set EIP-712 signature |
| `public_key_from_hex(key)` | Set public key |
| `validity(timestamp, days)` | Set validity period |
| `build_and_generate_calldata()` | Generate transaction calldata |

#### UserDecryptionResponseBuilder
| Method | Description |
|--------|-------------|
| `kms_signers(signers)` | Set KMS signer addresses |
| `user_address(address)` | Set user address |
| `gateway_chain_id(id)` | Set gateway chain ID |
| `verifying_contract_address(addr)` | Set verifying contract |
| `json_response(response)` | Set gateway response |
| `process()` | Process and decrypt response |

## Error Handling

The SDK uses a custom `FhevmError` type with helpful error messages:

```rust
match sdk.create_input_builder() {
    Ok(builder) => { /* use builder */ },
    Err(FhevmError::InvalidParams(msg)) => eprintln!("Invalid parameters: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

Common error patterns:
- **Missing required fields**: Clear messages about what's missing
- **Invalid addresses**: Address format validation
- **Invalid hex**: Hex string parsing errors
- **Threshold not reached**: KMS signature validation errors

## Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Run specific module tests
cargo test signature::
cargo test encryption::
cargo test decryption::
```

## Performance Notes

- **Key Loading**: Keys are loaded once and cached for the SDK lifetime
- **Multiple Values**: Use builders to include multiple values in single operations
- **Memory Usage**: Large encrypted inputs are handled efficiently with streaming



## Acknowledgments

- Built with [TFHE-rs](https://github.com/zama-ai/tfhe-rs) for homomorphic encryption
- Uses [Alloy](https://alloy.rs) for Ethereum interactions
- Developed with support from the Zama team



## License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE) for more details.

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
