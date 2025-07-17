use crate::blockchain::calldata::public_decryption_req;
use crate::utils::parse_hex_string;
use crate::{FhevmError, Result};
use alloy::primitives::FixedBytes;
use tracing::debug;

/// Public decrypt request structure
#[derive(Debug, Clone)]
pub struct PublicDecryptRequest {
    /// Ciphertext handles to decrypt
    pub ct_handles: Vec<FixedBytes<32>>,
}

/// Builder for creating public decrypt requests
#[derive(Debug, Clone, Default)]
pub struct PublicDecryptRequestBuilder {
    ct_handles: Vec<FixedBytes<32>>,
}

impl PublicDecryptRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            ct_handles: Vec::new(),
        }
    }

    /// Add handles from byte arrays
    ///
    /// # Arguments
    /// * `handles` - Array of 32-byte handles
    ///
    /// # Errors
    /// * If no handles are provided
    /// * If any handle is not exactly 32 bytes
    /// * If more than 256 handles are provided (protocol limit)
    pub fn with_handles_from_bytes(mut self, handles: &[Vec<u8>]) -> Result<Self> {
        validate_handles(handles)?;

        for (i, handle) in handles.iter().enumerate() {
            validate_handle_size(handle, i)?;
            let fixed_bytes = FixedBytes::<32>::from_slice(handle);
            self.ct_handles.push(fixed_bytes);
        }

        Ok(self)
    }

    /// Add handles from hex strings
    ///
    /// # Arguments
    /// * `hex_handles` - Array of hex strings (with or without 0x prefix)
    ///
    /// # Example
    /// ```ignore
    /// builder.with_handles_from_hex(&[
    ///     "0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200",
    ///     "a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890"
    /// ])?;
    /// ```
    pub fn with_handles_from_hex(mut self, hex_handles: &[&str]) -> Result<Self> {
        validate_handles_count(hex_handles.len())?;

        for (i, hex_handle) in hex_handles.iter().enumerate() {
            let handle_bytes = parse_hex_string(hex_handle, &format!("handle {i}"))?;
            validate_handle_size(&handle_bytes, i)?;
            let fixed_bytes = FixedBytes::<32>::from_slice(&handle_bytes);
            self.ct_handles.push(fixed_bytes);
        }

        Ok(self)
    }

    /// Add a single handle from bytes
    pub fn with_handle(mut self, handle: &[u8]) -> Result<Self> {
        validate_handle_size(handle, 0)?;
        let fixed_bytes = FixedBytes::<32>::from_slice(handle);
        self.ct_handles.push(fixed_bytes);
        Ok(self)
    }

    /// Clear all handles (useful for reusing the builder)
    pub fn with_handles_cleared(mut self) -> Self {
        self.ct_handles.clear();
        self
    }

    /// Get the number of handles currently added
    pub fn handle_count(&self) -> usize {
        self.ct_handles.len()
    }

    /// Build the request and generate calldata
    ///
    /// This is the final step that creates the transaction calldata ready to send to the blockchain.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - The encoded calldata bytes ready for your transaction
    /// * `Err(FhevmError)` - If no handles were added or validation fails
    pub fn build_and_generate_calldata(self) -> Result<Vec<u8>> {
        let request = self.build()?;
        let calldata = public_decryption_req(request.ct_handles)?;
        Ok(calldata.to_vec())
    }

    /// Build just the request object (without calldata generation)
    ///
    /// Use this method if you need the structured request object for other purposes
    /// besides generating calldata.
    pub fn build(self) -> Result<PublicDecryptRequest> {
        self.validate()?;
        debug!("✅ PublicDecryptRequest built successfully");
        debug!("   📊 Handles: {}", self.ct_handles.len());
        Ok(PublicDecryptRequest {
            ct_handles: self.ct_handles,
        })
    }

    fn validate(&self) -> Result<()> {
        if self.ct_handles.is_empty() {
            return Err(FhevmError::InvalidParams(
                "Missing handles: Call `with_handles_from_bytes()` or `with_handles_from_hex()` first.".to_string(),
            ));
        }

        if self.ct_handles.len() > 256 {
            return Err(FhevmError::InvalidParams(
                "Maximum 256 handles allowed in a single public decryption request".to_string(),
            ));
        }

        Ok(())
    }
}

// Constants
const MAX_HANDLES: usize = 256;
const HANDLE_SIZE_BYTES: usize = 32;

// Validation helpers
fn validate_handles(handles: &[Vec<u8>]) -> Result<()> {
    if handles.is_empty() {
        return Err(FhevmError::InvalidParams(
            "At least one ciphertext handle is required".to_string(),
        ));
    }
    validate_handles_count(handles.len())
}

fn validate_handles_count(count: usize) -> Result<()> {
    if count > MAX_HANDLES {
        return Err(FhevmError::InvalidParams(format!(
            "Maximum {MAX_HANDLES} handles allowed in a single public decryption request"
        )));
    }
    Ok(())
}

fn validate_handle_size(handle: &[u8], index: usize) -> Result<()> {
    if handle.len() != HANDLE_SIZE_BYTES {
        return Err(FhevmError::InvalidParams(format!(
            "Handle {} must be exactly {} bytes, got {}",
            index,
            HANDLE_SIZE_BYTES,
            handle.len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_valid_handles() {
        let handles = vec![vec![1u8; 32], vec![2u8; 32]];
        let result = PublicDecryptRequestBuilder::new()
            .with_handles_from_bytes(&handles)
            .unwrap()
            .build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.ct_handles.len(), 2);
    }

    #[test]
    fn test_builder_validates_empty_handles() {
        let result = PublicDecryptRequestBuilder::new().build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing handles"));
    }
}
