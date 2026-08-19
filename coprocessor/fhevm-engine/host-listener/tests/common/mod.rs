#![allow(dead_code)]

use alloy::primitives::{
    address, keccak256, Address, Bytes, FixedBytes, Log as PrimitiveLog, U256,
};
use alloy::sol_types::SolEvent;
use fhevm_host_bindings::acl::ACL::{Allowed, DelegatedForUserDecryption};
use fhevm_host_bindings::fhe_events::FHEEvents::{
    FheAdd, FheType, TrivialEncrypt,
};
use fhevm_host_bindings::kms_generation::IKMSGeneration::{KeyDigest, KeyType};
use fhevm_host_bindings::kms_generation::KMSGeneration::{
    ActivateCrs, ActivateKey,
};
use host_listener::cmd::block_history::BlockSummary;
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::Database;

pub const ACL_ADDRESS: Address =
    address!("0x000000000000000000000000000000000000ac11");
pub const TFHE_ADDRESS: Address =
    address!("0x0000000000000000000000000000000000007fe0");
pub const KMS_ADDRESS: Address =
    address!("0x00000000000000000000000000000000000004d5");
pub const PROTOCOL_CONFIG_ADDRESS: Address =
    address!("0x000000000000000000000000000000000000c0f1");

pub const BLOCK_TIMESTAMP: u64 = 1_700_000_000;
pub const NB_SYNTHETIC_CALLERS: usize = 15;
pub const TEST_KEY_ID: u64 = 16;
pub const KEY_BYTES: &[u8] = b"key_bytes";
pub const CRS_DIGEST: &[u8] = b"9\xf1\xe6\"\xf9L\xe2\xd9(\xf7DlBNZzg\xe1\xc8\x94\x0f\xa6\x95\xacJ\x8b\xc0\xdc\x86\xd0\x93$";

pub fn caller_at(index: usize) -> Address {
    let mut bytes = [0u8; 20];
    bytes[19] = (index + 1) as u8;
    Address::from_slice(&bytes)
}

pub fn synthetic_callers() -> Vec<Address> {
    (0..NB_SYNTHETIC_CALLERS).map(caller_at).collect()
}

pub fn block_hash_for(number: u64) -> FixedBytes<32> {
    let mut hash = [0u8; 32];
    hash[24..32].copy_from_slice(&number.to_be_bytes());
    FixedBytes::from(hash)
}

pub fn tx_hash_for(seed: u64) -> FixedBytes<32> {
    let mut hash = [0x11u8; 32];
    hash[24..32].copy_from_slice(&seed.to_be_bytes());
    FixedBytes::from(hash)
}

pub fn parent_hash_for(number: u64) -> FixedBytes<32> {
    if number == 0 {
        FixedBytes::ZERO
    } else {
        block_hash_for(number - 1)
    }
}

pub fn block_summary(number: u64) -> BlockSummary {
    BlockSummary {
        number,
        hash: block_hash_for(number),
        parent_hash: parent_hash_for(number),
        timestamp: BLOCK_TIMESTAMP + number,
    }
}

pub fn default_ingest_options() -> IngestOptions {
    IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: true,
        dependent_ops_max_per_chain: 0,
        is_protocol_config_listener: true,
    }
}

pub fn rpc_log(
    address: Address,
    data: alloy::primitives::LogData,
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> alloy::rpc::types::Log {
    alloy::rpc::types::Log {
        inner: PrimitiveLog { address, data },
        transaction_hash: Some(tx_hash),
        log_index: Some(log_index),
        ..Default::default()
    }
}

pub fn trivial_encrypt_log(
    caller: Address,
    pt: U256,
    to_type: u8,
    result: FixedBytes<32>,
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> alloy::rpc::types::Log {
    let event = TrivialEncrypt {
        caller,
        pt,
        toType: FheType::from_underlying(to_type).into(),
        result,
    };
    rpc_log(TFHE_ADDRESS, event.encode_log_data(), tx_hash, log_index)
}

pub fn allowed_log(
    caller: Address,
    account: Address,
    handle: FixedBytes<32>,
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> alloy::rpc::types::Log {
    let event = Allowed {
        caller,
        account,
        handle,
    };
    rpc_log(ACL_ADDRESS, event.encode_log_data(), tx_hash, log_index)
}

pub fn fhe_add_log(
    caller: Address,
    lhs: FixedBytes<32>,
    rhs: FixedBytes<32>,
    scalar_byte: u8,
    result: FixedBytes<32>,
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> alloy::rpc::types::Log {
    let event = FheAdd {
        caller,
        lhs,
        rhs,
        scalarByte: FixedBytes::<1>::from([scalar_byte]),
        result,
    };
    rpc_log(TFHE_ADDRESS, event.encode_log_data(), tx_hash, log_index)
}

#[allow(clippy::too_many_arguments)]
pub fn delegated_for_user_decryption_log(
    delegator: Address,
    delegate: Address,
    contract_address: Address,
    delegation_counter: u64,
    old_expiration_date: u64,
    new_expiration_date: u64,
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> alloy::rpc::types::Log {
    let event = DelegatedForUserDecryption {
        delegator,
        delegate,
        contractAddress: contract_address,
        delegationCounter: delegation_counter,
        oldExpirationDate: old_expiration_date,
        newExpirationDate: new_expiration_date,
    };
    rpc_log(ACL_ADDRESS, event.encode_log_data(), tx_hash, log_index)
}

pub fn kms_storage_urls() -> Vec<String> {
    (1..=4)
        .map(|i| format!("https://s3.region.amazonaws.com/test-bucket{i}"))
        .collect()
}

pub fn key_digest_bytes() -> Bytes {
    keccak256(KEY_BYTES).to_vec().into()
}

pub fn activate_key_log(
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> alloy::rpc::types::Log {
    let digest = key_digest_bytes();
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
    rpc_log(KMS_ADDRESS, event.encode_log_data(), tx_hash, log_index)
}

pub fn activate_crs_log(
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> alloy::rpc::types::Log {
    let event = ActivateCrs {
        crsId: U256::from(TEST_KEY_ID),
        kmsNodeStorageUrls: kms_storage_urls(),
        crsDigest: Bytes::from(CRS_DIGEST.to_vec()),
    };
    rpc_log(KMS_ADDRESS, event.encode_log_data(), tx_hash, log_index)
}

pub async fn ingest_logs(
    db: &mut Database,
    logs: Vec<alloy::rpc::types::Log>,
    summary: BlockSummary,
    finalized: bool,
    options: IngestOptions,
) -> Result<(), sqlx::Error> {
    let block_logs = BlockLogs {
        logs,
        summary,
        catchup: false,
        finalized,
    };
    ingest_block_logs(
        db.chain_id,
        db,
        &block_logs,
        &Some(ACL_ADDRESS),
        &Some(TFHE_ADDRESS),
        &Some(KMS_ADDRESS),
        &Some(PROTOCOL_CONFIG_ADDRESS),
        &None,
        options,
    )
    .await
}
