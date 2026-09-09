//! Shared Solana byte types and the host program's account addresses.
//!
//! Authorization itself lives in [`super::solana`]; this module holds what several of its rules
//! and the public-decrypt path share — the pubkey and handle aliases, the delegation witness
//! decoder, and the PDA derivations of the two singleton-shaped records the pipeline reads (the
//! host config and a delegation row). The encrypted value account's address is derived from its
//! own fields in [`super::solana::encrypted_value_account`].

use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;

pub type SolanaPubkeyBytes = [u8; 32];
pub type HandleBytes = [u8; 32];

// The record layout, its decoder and the wildcard sentinel live in the shared crate, so every
// off-chain reader (this connector's authoritative check, the relayer's advisory pre-check)
// decodes the same bytes through one implementation.
pub use zama_solana_acl::UserDecryptionDelegationRecord;
pub use zama_solana_acl::WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY;
pub use zama_solana_acl::delegation::DELEGATION_SEED;

pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
const ANCHOR_DISCRIMINATOR_LEN: usize = 8;

/// A decoded delegation record together with where it was read from.
///
/// The record is the shared crate's, held whole rather than restated field by field: a field
/// added there reaches this reader without a copy to keep in step, and the liveness rule is
/// asked of the very bytes that were decoded — `record.is_live_at`, whose one definition in
/// `zama-solana-acl` this reader shares with the relayer's advisory pre-check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserDecryptionDelegationWitness {
    pub account_key: SolanaPubkeyBytes,
    pub owner: SolanaPubkeyBytes,
    pub record: UserDecryptionDelegationRecord,
}

/// Why account bytes are not a delegation record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DelegationDecodeError {
    #[error("account discriminator is not the delegation record's")]
    BadDiscriminator,
    #[error("delegation record body does not decode")]
    BadAccountData,
}

pub fn decode_user_decryption_delegation_witness(
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    data: &[u8],
) -> Result<UserDecryptionDelegationWitness, DelegationDecodeError> {
    let record =
        zama_solana_acl::decode_user_decryption_delegation(data).map_err(|error| match error {
            zama_solana_acl::AclError::BadDiscriminator => DelegationDecodeError::BadDiscriminator,
            zama_solana_acl::AclError::BadAccountData => DelegationDecodeError::BadAccountData,
            // The remaining variants belong to the encrypted value account's authorization
            // rules; the delegation decoder cannot produce them, and enumerating them keeps a
            // new one from landing here silently.
            zama_solana_acl::AclError::MmrInconsistent
            | zama_solana_acl::AclError::MmrPeakCapacityExceeded
            | zama_solana_acl::AclError::HistoricalProofInvalid
            | zama_solana_acl::AclError::PublicDecryptProofInvalid => {
                DelegationDecodeError::BadAccountData
            }
        })?;
    Ok(UserDecryptionDelegationWitness {
        account_key,
        owner,
        record,
    })
}

pub fn host_config_address(host_program_id: SolanaPubkeyBytes) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(&[HOST_CONFIG_SEED], &host_program_id);
    (address.to_bytes(), bump)
}

pub fn user_decryption_delegation_address(
    host_program_id: SolanaPubkeyBytes,
    delegator: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    encrypted_value_account_authority: SolanaPubkeyBytes,
) -> (SolanaPubkeyBytes, u8) {
    let host_program_id = Pubkey::new_from_array(host_program_id);
    let (address, bump) = Pubkey::find_program_address(
        &[
            DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            encrypted_value_account_authority.as_ref(),
        ],
        &host_program_id,
    );
    (address.to_bytes(), bump)
}

pub fn anchor_account_discriminator(account_name: &str) -> [u8; ANCHOR_DISCRIMINATOR_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(b"account:");
    hasher.update(account_name.as_bytes());
    let digest = hasher.finalize();
    let mut discriminator = [0; ANCHOR_DISCRIMINATOR_LEN];
    discriminator.copy_from_slice(&digest[..ANCHOR_DISCRIMINATOR_LEN]);
    discriminator
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const OWNER: SolanaPubkeyBytes = [3; 32];
    const AUTHORITY: SolanaPubkeyBytes = [2; 32];
    const DELEGATE: SolanaPubkeyBytes = [5; 32];
    const OBSERVED_SLOT: u64 = 500;

    fn delegation() -> UserDecryptionDelegationWitness {
        let (account_key, bump) =
            user_decryption_delegation_address(HOST_PROGRAM_ID, OWNER, DELEGATE, AUTHORITY);
        UserDecryptionDelegationWitness {
            account_key,
            owner: HOST_PROGRAM_ID,
            record: UserDecryptionDelegationRecord {
                delegator: OWNER,
                delegate: DELEGATE,
                encrypted_value_account_authority: AUTHORITY,
                expiration_slot: OBSERVED_SLOT + 20,
                delegation_counter: 9,
                last_update_slot: OBSERVED_SLOT - 1,
                revoked: false,
                bump,
            },
        }
    }

    fn encode_delegation(delegation: &UserDecryptionDelegationWitness) -> Vec<u8> {
        let record = &delegation.record;
        let mut data = anchor_account_discriminator("UserDecryptionDelegation").to_vec();
        data.extend_from_slice(&record.delegator);
        data.extend_from_slice(&record.delegate);
        data.extend_from_slice(&record.encrypted_value_account_authority);
        data.extend_from_slice(&record.expiration_slot.to_le_bytes());
        data.extend_from_slice(&record.delegation_counter.to_le_bytes());
        data.extend_from_slice(&record.last_update_slot.to_le_bytes());
        data.push(record.revoked as u8);
        data.push(record.bump);
        data
    }

    #[test]
    fn decodes_anchor_delegation_account_data() {
        let delegation = delegation();
        let decoded = decode_user_decryption_delegation_witness(
            delegation.account_key,
            HOST_PROGRAM_ID,
            &encode_delegation(&delegation),
        )
        .expect("delegation decodes");
        assert_eq!(decoded, delegation);
    }

    #[test]
    fn rejects_invalid_anchor_account_data() {
        let mut invalid_bool = encode_delegation(&delegation());
        let revoked_offset = invalid_bool.len() - 2;
        invalid_bool[revoked_offset] = 2;
        assert_eq!(
            decode_user_decryption_delegation_witness([0; 32], HOST_PROGRAM_ID, &invalid_bool),
            Err(DelegationDecodeError::BadAccountData)
        );
    }
}
