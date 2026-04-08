//! Calldata generation for FHEVM on-chain transactions.
//!
//! Delegates to [`fhevm_client_core::blockchain::calldata`] for the actual encoding.

pub use fhevm_client_core::blockchain::calldata::{
    public_decryption_req, user_decryption_req, verify_proof_req,
};
