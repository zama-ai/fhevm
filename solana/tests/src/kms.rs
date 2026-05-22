use anchor_lang::AccountDeserialize;
use confidential_token as token;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
};
use zama_host as host;

use crate::{
    acl::{acl_record_address, record_subjects},
    fixture::{token_account, TokenFixture},
};

#[derive(Clone)]
pub struct UserDecryptAuthorizationPayload {
    pub user: Pubkey,
    pub reencryption_public_key: [u8; 32],
    pub allowed_acl_domain_keys: Vec<Pubkey>,
    pub start_timestamp: i64,
    pub duration_seconds: u64,
    pub extra_data: Vec<u8>,
}

#[derive(Clone)]
pub struct UserDecryptRequest {
    pub authorization: UserDecryptAuthorizationPayload,
    pub signature: Signature,
    pub handles: Vec<UserDecryptHandleEntry>,
}

#[derive(Clone, Copy)]
pub struct UserDecryptHandleEntry {
    pub handle: [u8; 32],
    pub owner: Pubkey,
    pub acl_record: Pubkey,
}

pub fn signed_user_decrypt_request(
    fixture: &TokenFixture,
    signer: &Keypair,
    handles: Vec<UserDecryptHandleEntry>,
) -> UserDecryptRequest {
    let authorization = UserDecryptAuthorizationPayload {
        user: signer.pubkey(),
        reencryption_public_key: [7; 32],
        allowed_acl_domain_keys: vec![fixture.mint.pubkey()],
        start_timestamp: 1,
        duration_seconds: 300,
        extra_data: b"zama-solana-poc".to_vec(),
    };
    let signature = signer.sign_message(&authorization_payload_bytes(&authorization));

    UserDecryptRequest {
        authorization,
        signature,
        handles,
    }
}

pub fn signed_user_decrypt_request_with_domains(
    signer: &Keypair,
    allowed_acl_domain_keys: Vec<Pubkey>,
    handles: Vec<UserDecryptHandleEntry>,
) -> UserDecryptRequest {
    let authorization = UserDecryptAuthorizationPayload {
        user: signer.pubkey(),
        reencryption_public_key: [7; 32],
        allowed_acl_domain_keys,
        start_timestamp: 1,
        duration_seconds: 300,
        extra_data: b"zama-solana-poc".to_vec(),
    };
    let signature = signer.sign_message(&authorization_payload_bytes(&authorization));

    UserDecryptRequest {
        authorization,
        signature,
        handles,
    }
}

pub fn signed_current_balance_user_decrypt_request(
    fixture: &TokenFixture,
    token_account_address: Pubkey,
    signer: &Keypair,
) -> UserDecryptRequest {
    let account = token_account(&fixture.svm, token_account_address);
    signed_user_decrypt_request(
        fixture,
        signer,
        vec![UserDecryptHandleEntry {
            handle: account.balance_handle,
            owner: account.owner,
            acl_record: account.balance_acl_record,
        }],
    )
}

pub fn signed_confidential_rand_user_decrypt_request(
    fixture: &TokenFixture,
    signer: &Keypair,
    rand_handle: [u8; 32],
    acl_record: Pubkey,
) -> UserDecryptRequest {
    signed_user_decrypt_request(
        fixture,
        signer,
        vec![UserDecryptHandleEntry {
            handle: rand_handle,
            owner: signer.pubkey(),
            acl_record,
        }],
    )
}

pub fn authorization_payload_bytes(authorization: &UserDecryptAuthorizationPayload) -> Vec<u8> {
    let mut bytes = b"Zama Solana UserDecrypt v0".to_vec();
    bytes.extend_from_slice(authorization.user.as_ref());
    bytes.extend_from_slice(&authorization.reencryption_public_key);
    bytes.extend_from_slice(&(authorization.allowed_acl_domain_keys.len() as u32).to_le_bytes());
    for account in &authorization.allowed_acl_domain_keys {
        bytes.extend_from_slice(account.as_ref());
    }
    bytes.extend_from_slice(&authorization.start_timestamp.to_le_bytes());
    bytes.extend_from_slice(&authorization.duration_seconds.to_le_bytes());
    bytes.extend_from_slice(&(authorization.extra_data.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&authorization.extra_data);
    bytes
}

pub fn kms_like_user_decrypt_check(svm: &litesvm::LiteSVM, request: &UserDecryptRequest) -> bool {
    let authorization = &request.authorization;
    let signed_payload = authorization_payload_bytes(authorization);
    if !request
        .signature
        .verify(authorization.user.as_ref(), &signed_payload)
        || authorization.reencryption_public_key == [0; 32]
        || authorization.duration_seconds == 0
        || authorization.extra_data.is_empty()
        || authorization.start_timestamp < 0
        || request.handles.is_empty()
    {
        return false;
    }

    request.handles.iter().all(|entry| {
        if authorization.user != entry.owner {
            return false;
        }

        let Some(raw_account) = svm.get_account(&entry.acl_record) else {
            return false;
        };
        if raw_account.owner != host::id() {
            return false;
        }

        let mut data = raw_account.data.as_slice();
        let Ok(record) = host::AclRecord::try_deserialize(&mut data) else {
            return false;
        };
        let expected_nonce_key = token::nonce_key(
            record.acl_domain_key,
            record.app_account,
            record.encrypted_value_label,
        );
        let expected_acl_record =
            acl_record_address(host::id(), expected_nonce_key, record.nonce_sequence);

        authorization
            .allowed_acl_domain_keys
            .contains(&record.acl_domain_key)
            && record.handle == entry.handle
            && record.nonce_key == expected_nonce_key
            && entry.acl_record == expected_acl_record
            && record_subjects(&record).contains(&authorization.user)
    })
}

#[derive(Clone, Copy)]
pub struct PublicDecryptHandleEntry {
    pub handle: [u8; 32],
    pub acl_record: Pubkey,
}

pub fn kms_like_public_decrypt_check(
    svm: &litesvm::LiteSVM,
    entries: &[PublicDecryptHandleEntry],
) -> bool {
    if entries.is_empty() {
        return false;
    }

    entries.iter().all(|entry| {
        let Some(raw_account) = svm.get_account(&entry.acl_record) else {
            return false;
        };
        if raw_account.owner != host::id() {
            return false;
        }

        let mut data = raw_account.data.as_slice();
        let Ok(record) = host::AclRecord::try_deserialize(&mut data) else {
            return false;
        };
        let expected_nonce_key = token::nonce_key(
            record.acl_domain_key,
            record.app_account,
            record.encrypted_value_label,
        );
        let expected_acl_record =
            acl_record_address(host::id(), expected_nonce_key, record.nonce_sequence);

        record.handle == entry.handle
            && record.nonce_key == expected_nonce_key
            && entry.acl_record == expected_acl_record
            && record.public_decrypt
    })
}
