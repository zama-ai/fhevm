use crate::db_manager::DecryptionRequestType;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use anyhow::Result;
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest, SnsCiphertextMaterial, UserDecryptionRequest,
};
use rand::{thread_rng, Rng};
use std::time::SystemTime;

/// Builder for creating different types of decryption requests
pub struct RequestBuilder {
    counter: std::sync::atomic::AtomicU64,
}

impl RequestBuilder {
    /// Create a new request builder
    pub fn new() -> Self {
        Self {
            counter: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    /// Build a batch of decryption requests
    pub fn build_requests(
        &self,
        request_type: DecryptionRequestType,
        count: usize,
    ) -> Result<Vec<DecryptionRequest>> {
        let mut requests = Vec::new();
        
        for _ in 0..count {
            let request = match request_type {
                DecryptionRequestType::Public => {
                    DecryptionRequest::Public(self.build_public_request()?)
                }
                DecryptionRequestType::User => {
                    DecryptionRequest::User(self.build_user_request()?)
                }
                DecryptionRequestType::Mixed => {
                    // 50/50 distribution for mixed mode
                    if thread_rng().gen_bool(0.5) {
                        DecryptionRequest::Public(self.build_public_request()?)
                    } else {
                        DecryptionRequest::User(self.build_user_request()?)
                    }
                }
            };
            requests.push(request);
        }
        
        Ok(requests)
    }
    
    /// Build a single public decryption request
    fn build_public_request(&self) -> Result<PublicDecryptionRequest> {
        // Generate a unique decryption ID as U256
        let id_bytes = self.generate_unique_id();
        let mut id_array = [0u8; 32];
        id_array[..id_bytes.len().min(32)].copy_from_slice(&id_bytes[..id_bytes.len().min(32)]);
        let decryption_id = U256::from_be_bytes(id_array);
        
        // Generate mock SNS ciphertext materials (1-3 materials per request)
        let num_materials = rand::thread_rng().gen_range(1..=3);
        let mut sns_ct_materials = Vec::new();
        
        for i in 0..num_materials {
            // Generate random bytes for each field
            let mut ct_handle = [0u8; 32];
            let mut sns_digest = [0u8; 32];
            thread_rng().fill(&mut ct_handle);
            thread_rng().fill(&mut sns_digest);
            
            sns_ct_materials.push(SnsCiphertextMaterial {
                ctHandle: FixedBytes::from(ct_handle),
                keyId: U256::from(i + 1), // Simple key ID
                snsCiphertextDigest: FixedBytes::from(sns_digest),
                coprocessorTxSenderAddresses: vec![{
                    let bytes: [u8; 20] = self.generate_random_bytes(20).try_into().unwrap();
                    Address::from(bytes)
                }],
            });
        }
        
        // Generate extra data as Bytes
        let extra_data_vec = self.generate_extra_data();
        let extra_data = Bytes::from(extra_data_vec);
        
        Ok(PublicDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct_materials,
            extraData: extra_data,
        })
    }
    
    /// Build a single user decryption request
    fn build_user_request(&self) -> Result<UserDecryptionRequest> {
        // Generate a unique decryption ID as U256
        let id_bytes = self.generate_unique_id();
        let mut id_array = [0u8; 32];
        id_array[..id_bytes.len().min(32)].copy_from_slice(&id_bytes[..id_bytes.len().min(32)]);
        let decryption_id = U256::from_be_bytes(id_array);
        
        // Generate mock SNS ciphertext materials (1-3 materials per request)
        let num_materials = rand::thread_rng().gen_range(1..=3);
        let mut sns_ct_materials = Vec::new();
        
        for i in 0..num_materials {
            // Generate random bytes for each field
            let mut ct_handle = [0u8; 32];
            let mut sns_digest = [0u8; 32];
            thread_rng().fill(&mut ct_handle);
            thread_rng().fill(&mut sns_digest);
            
            sns_ct_materials.push(SnsCiphertextMaterial {
                ctHandle: FixedBytes::from(ct_handle),
                keyId: U256::from(i + 1), // Simple key ID
                snsCiphertextDigest: FixedBytes::from(sns_digest),
                coprocessorTxSenderAddresses: vec![{
                    let bytes: [u8; 20] = self.generate_random_bytes(20).try_into().unwrap();
                    Address::from(bytes)
                }],
            });
        }
        
        // Generate user address
        let user_addr_bytes: [u8; 20] = self.generate_random_bytes(20).try_into().unwrap();
        let user_address = Address::from(user_addr_bytes);
        
        // Generate public key as Bytes (33 bytes for compressed secp256k1)
        let public_key_vec = self.generate_random_bytes(33);
        let public_key = Bytes::from(public_key_vec);
        
        // Generate extra data as Bytes
        let extra_data_vec = self.generate_extra_data();
        let extra_data = Bytes::from(extra_data_vec);
        
        Ok(UserDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct_materials,
            userAddress: user_address,
            publicKey: public_key,
            extraData: extra_data,
        })
    }
    
    /// Generate a unique ID for a decryption request
    fn generate_unique_id(&self) -> Vec<u8> {
        let counter = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let mut id = Vec::new();
        id.extend_from_slice(&timestamp.to_be_bytes());
        id.extend_from_slice(&counter.to_be_bytes());
        
        // Add some random bytes
        let mut rng = thread_rng();
        let size = rng.gen_range(64..256);
        let mut random_bytes = vec![0u8; size];
        rng.fill(&mut random_bytes[..]);
        id.extend_from_slice(&random_bytes);
        
        id
    }
    
    /// Generate random bytes of specified size
    fn generate_random_bytes(&self, size: usize) -> Vec<u8> {
        let mut rng = thread_rng();
        let mut bytes = vec![0u8; size];
        rng.fill(&mut bytes[..]);
        bytes
    }
    
    /// Generate extra data
    fn generate_extra_data(&self) -> Vec<u8> {
        let mut rng = thread_rng();
        let size = rng.gen_range(16..64);
        let mut data = vec![0u8; size];
        rng.fill(&mut data[..]);
        data
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a decryption request that can be either public or user type
#[derive(Debug, Clone)]
pub enum DecryptionRequest {
    Public(PublicDecryptionRequest),
    User(UserDecryptionRequest),
}

impl DecryptionRequest {
    /// Get the ID of the request as a hex string
    pub fn id(&self) -> String {
        match self {
            DecryptionRequest::Public(req) => hex::encode(req.decryptionId.to_le_bytes::<32>()),
            DecryptionRequest::User(req) => hex::encode(req.decryptionId.to_le_bytes::<32>()),
        }
    }
}
