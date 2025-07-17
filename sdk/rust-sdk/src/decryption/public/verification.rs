use crate::utils::{parse_hex_string, validate_address_from_str};
use crate::{FhevmError, Result};
use alloy::primitives::{Address, FixedBytes};
use alloy::sol_types::SolStruct;
use std::collections::HashSet;
use tracing::{debug, info};

// Define the EIP-712 types
alloy::sol! {
    struct PublicDecryptVerification {
        bytes32[] ctHandles;
        bytes decryptedResult;
    }
}

/// Verify signatures using EIP-712
pub fn verify_signatures(
    kms_signers: &[String],
    threshold: usize,
    gateway_chain_id: u64,
    verifying_contract_address: &str,
    ct_handles: &[String],
    decrypted_result: &str,
    signatures: &[String],
) -> Result<()> {
    let verifying_address = validate_address_from_str(verifying_contract_address)?;

    // Prepare EIP-712 domain and message
    let domain = create_eip712_domain(gateway_chain_id, verifying_address);
    let message = create_verification_message(ct_handles, decrypted_result)?;
    let signing_hash = message.eip712_signing_hash(&domain);

    // Verify each signature
    let recovered_addresses = recover_addresses(signatures, &signing_hash)?;

    // Check threshold
    is_threshold_reached(kms_signers, &recovered_addresses, threshold)?;

    info!(
        "✅ Signature verification passed: {}/{} signers",
        recovered_addresses.len(),
        threshold
    );

    Ok(())
}

fn create_eip712_domain(
    chain_id: u64,
    verifying_contract: Address,
) -> alloy::sol_types::Eip712Domain {
    alloy::sol_types::eip712_domain! {
        name: "Decryption",
        version: "1",
        chain_id: chain_id,
        verifying_contract: verifying_contract,
    }
}

fn create_verification_message(
    ct_handles: &[String],
    decrypted_result: &str,
) -> Result<PublicDecryptVerification> {
    // Convert handles to bytes32
    let ct_handles_bytes32 = ct_handles
        .iter()
        .map(|h| parse_handle_to_bytes32(h))
        .collect::<Result<Vec<_>>>()?;

    let decrypted_result_bytes = parse_hex_string(decrypted_result, "decrypted result")?;

    Ok(PublicDecryptVerification {
        ctHandles: ct_handles_bytes32,
        decryptedResult: decrypted_result_bytes,
    })
}

fn parse_handle_to_bytes32(handle: &str) -> Result<FixedBytes<32>> {
    let cleaned = handle.strip_prefix("0x").unwrap_or(handle);
    let bytes = hex::decode(cleaned)
        .map_err(|e| FhevmError::InvalidParams(format!("Invalid handle hex: {}", e)))?;

    if bytes.len() != 32 {
        return Err(FhevmError::InvalidParams(
            "Handle must be 32 bytes".to_string(),
        ));
    }

    Ok(FixedBytes::<32>::from_slice(&bytes))
}

fn recover_addresses(
    signatures: &[String],
    signing_hash: &alloy::primitives::B256,
) -> Result<Vec<String>> {
    let mut recovered_addresses = Vec::new();

    for (i, sig_str) in signatures.iter().enumerate() {
        let sig_bytes = parse_hex_string(sig_str, &format!("signature {}", i))?;

        let signature = alloy::primitives::Signature::from_raw(&sig_bytes).map_err(|e| {
            FhevmError::DecryptionError(format!("Invalid signature {} format: {}", i, e))
        })?;

        let recovered = signature
            .recover_address_from_prehash(signing_hash)
            .map_err(|e| {
                FhevmError::DecryptionError(format!(
                    "Failed to recover address from signature {}: {}",
                    i, e
                ))
            })?;

        debug!("Signature {} recovered address: {}", i, recovered);
        recovered_addresses.push(recovered.to_string());
    }

    Ok(recovered_addresses)
}

/// Check if threshold is reached with proper validation
fn is_threshold_reached(
    kms_signers: &[String],
    recovered_addresses: &[String],
    threshold: usize,
) -> Result<()> {
    // Check for duplicates
    let mut seen = HashSet::new();
    for addr in recovered_addresses {
        if !seen.insert(addr) {
            return Err(FhevmError::DecryptionError(format!(
                "Duplicate KMS signer address found: {} appears multiple times",
                addr
            )));
        }
    }

    // Normalize addresses for comparison (lowercase)
    let kms_signers_lower: Vec<String> = kms_signers.iter().map(|s| s.to_lowercase()).collect();

    // Check all recovered addresses are valid KMS signers
    for addr in recovered_addresses {
        let addr_lower = addr.to_lowercase();
        if !kms_signers_lower.contains(&addr_lower) {
            return Err(FhevmError::DecryptionError(format!(
                "Invalid address found: {} is not in the list of KMS signers",
                addr
            )));
        }
    }

    if recovered_addresses.len() < threshold {
        return Err(FhevmError::DecryptionError(
            "KMS signers threshold is not reached".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold_validation() {
        let kms_signers = vec![
            "0x1234567890123456789012345678901234567890".to_string(),
            "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
        ];

        let recovered = vec!["0x1234567890123456789012345678901234567890".to_string()];

        // Should pass with threshold 1
        assert!(is_threshold_reached(&kms_signers, &recovered, 1).is_ok());

        // Should fail with threshold 2
        assert!(is_threshold_reached(&kms_signers, &recovered, 2).is_err());
    }

    #[test]
    fn test_duplicate_detection() {
        let kms_signers = vec!["0x1234567890123456789012345678901234567890".to_string()];

        let recovered = vec![
            "0x1234567890123456789012345678901234567890".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
        ];

        let result = is_threshold_reached(&kms_signers, &recovered, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate"));
    }

    #[test]
    fn test_invalid_signer_detection() {
        let kms_signers = vec!["0x1234567890123456789012345678901234567890".to_string()];

        let recovered = vec!["0x9999999999999999999999999999999999999999".to_string()];

        let result = is_threshold_reached(&kms_signers, &recovered, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not in the list"));
    }

    #[test]
    fn test_case_insensitive_comparison() {
        let kms_signers = vec!["0xABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCD".to_string()];

        let recovered = vec!["0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string()];

        // Should pass despite different case
        assert!(is_threshold_reached(&kms_signers, &recovered, 1).is_ok());
    }
}
