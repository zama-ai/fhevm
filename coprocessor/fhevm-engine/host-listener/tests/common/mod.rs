#![allow(dead_code)]

use alloy::network::Network;
use alloy::primitives::{keccak256, Address, Bytes, FixedBytes, LogData, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::sol;
use alloy::sol_types::SolEvent;
use fhevm_host_bindings::kms_generation::IKMSGeneration::{KeyDigest, KeyType};
use host_listener::contracts::AclContract::{
    Allowed, DelegatedForUserDecryption,
};
use host_listener::contracts::KMSGeneration::{ActivateCrs, ActivateKey};
use host_listener::contracts::TfheContract::{FheAdd, TrivialEncrypt};

sol!(
    #[sol(rpc)]
    RawLog,
    "tests/fixtures/RawLog.json"
);

pub use RawLog::RawLogInstance;

pub const TEST_KEY_ID: u64 = 16;
pub const KEY_BYTES: &[u8] = b"key_bytes";
pub const CRS_DIGEST: &[u8] = b"9\xf1\xe6\"\xf9L\xe2\xd9(\xf7DlBNZzg\xe1\xc8\x94\x0f\xa6\x95\xacJ\x8b\xc0\xdc\x86\xd0\x93$";

pub fn emit_log_request<P, N>(
    emitter: &RawLogInstance<P, N>,
    log_data: LogData,
) -> TransactionRequest
where
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    emitter
        .emitLog(log_data.topics().to_vec(), log_data.data.clone())
        .into_transaction_request()
}

pub fn mock_trivial_encrypt_result(pt: U256, to_type: u8) -> FixedBytes<32> {
    let mut payload = Vec::with_capacity(
        "trivialEncrypt".len() + std::mem::size_of::<[u8; 32]>() + 1,
    );
    payload.extend_from_slice(b"trivialEncrypt");
    payload.extend_from_slice(&pt.to_be_bytes::<32>());
    payload.push(to_type);
    keccak256(&payload)
}

pub fn mock_fhe_add_result(
    lhs: FixedBytes<32>,
    rhs: FixedBytes<32>,
    scalar_byte: FixedBytes<1>,
) -> FixedBytes<32> {
    let mut payload = Vec::with_capacity("fheAdd".len() + 32 + 32 + 1);
    payload.extend_from_slice(b"fheAdd");
    payload.extend_from_slice(lhs.as_slice());
    payload.extend_from_slice(rhs.as_slice());
    payload.extend_from_slice(scalar_byte.as_slice());
    keccak256(&payload)
}

pub fn trivial_encrypt_request<P, N>(
    emitter: &RawLogInstance<P, N>,
    caller: Address,
    pt: U256,
    to_type: u8,
) -> TransactionRequest
where
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let event = TrivialEncrypt {
        caller,
        pt,
        toType: to_type,
        result: mock_trivial_encrypt_result(pt, to_type),
    };
    emit_log_request(emitter, event.encode_log_data())
}

pub fn allowed_request<P, N>(
    emitter: &RawLogInstance<P, N>,
    caller: Address,
    account: Address,
    handle: FixedBytes<32>,
) -> TransactionRequest
where
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let event = Allowed {
        caller,
        account,
        handle,
    };
    emit_log_request(emitter, event.encode_log_data())
}

pub fn fhe_add_request<P, N>(
    emitter: &RawLogInstance<P, N>,
    caller: Address,
    lhs: FixedBytes<32>,
    rhs: FixedBytes<32>,
    scalar_byte: FixedBytes<1>,
) -> TransactionRequest
where
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let event = FheAdd {
        caller,
        lhs,
        rhs,
        scalarByte: scalar_byte,
        result: mock_fhe_add_result(lhs, rhs, scalar_byte),
    };
    emit_log_request(emitter, event.encode_log_data())
}

#[allow(clippy::too_many_arguments)]
pub fn delegate_for_user_decryption_request<P, N>(
    emitter: &RawLogInstance<P, N>,
    delegator: Address,
    delegate: Address,
    contract_address: Address,
    delegation_counter: u64,
    old_expiration_date: u64,
    new_expiration_date: u64,
) -> TransactionRequest
where
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let event = DelegatedForUserDecryption {
        delegator,
        delegate,
        contractAddress: contract_address,
        delegationCounter: delegation_counter,
        oldExpirationDate: old_expiration_date,
        newExpirationDate: new_expiration_date,
    };
    emit_log_request(emitter, event.encode_log_data())
}

pub fn kms_storage_urls() -> Vec<String> {
    (1..=4)
        .map(|i| format!("https://s3.region.amazonaws.com/test-bucket{i}"))
        .collect()
}

pub fn activate_key_request<P, N>(
    emitter: &RawLogInstance<P, N>,
) -> TransactionRequest
where
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let digest = Bytes::from(keccak256(KEY_BYTES).to_vec());
    let event = ActivateKey {
        keyId: U256::from(TEST_KEY_ID),
        kmsNodeStorageUrls: kms_storage_urls(),
        keyDigests: vec![
            KeyDigest {
                keyType: KeyType::from_underlying(1).into(),
                digest: digest.clone(),
            },
            KeyDigest {
                keyType: KeyType::from_underlying(0).into(),
                digest,
            },
        ],
    };
    emit_log_request(emitter, event.encode_log_data())
}

pub fn activate_crs_request<P, N>(
    emitter: &RawLogInstance<P, N>,
) -> TransactionRequest
where
    P: Provider<N>,
    N: Network<TransactionRequest = TransactionRequest>,
{
    let event = ActivateCrs {
        crsId: U256::from(TEST_KEY_ID),
        kmsNodeStorageUrls: kms_storage_urls(),
        crsDigest: Bytes::from(CRS_DIGEST.to_vec()),
    };
    emit_log_request(emitter, event.encode_log_data())
}
