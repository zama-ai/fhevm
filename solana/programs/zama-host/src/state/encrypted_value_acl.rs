//! Host bindings for the shared encrypted-value ACL (`zama_solana_acl`).
//!
//! The account layout, MMR history, leaf commitments, and authorization rules
//! live in the shared crate, so the host and the KMS connector cannot drift.
//! This module re-exports them and adds the host-side PDA derivation.

use anchor_lang::prelude::*;

pub use zama_solana_acl::{
    authorize_current, authorize_historical, authorize_public, decode_account, encode_account,
    historical_access_leaf_commitment, public_decrypt_leaf_commitment, AclError, EncryptedValueAcl,
    ENCRYPTED_VALUE_ACL_SEED, MAX_ENCRYPTED_VALUE_SUBJECTS,
};

/// Canonical encrypted-value ACL lineage address for a value key.
pub fn encrypted_value_acl_address(value_key: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ENCRYPTED_VALUE_ACL_SEED, &value_key], &crate::ID)
}
