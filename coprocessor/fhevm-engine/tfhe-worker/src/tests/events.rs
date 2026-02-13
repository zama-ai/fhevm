use alloy::primitives::Log;
use fhevm_engine_common::chain_id::ChainId;
use sqlx::types::time::PrimitiveDateTime;

use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, LogTfhe, ToType, Transaction,
};

use crate::tests::utils::{default_dependence_cache_size, TestInstance};

fn tfhe_event(data: TfheContractEvents) -> Log<TfheContractEvents> {
    let address = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    Log::<TfheContractEvents> { address, data }
}

pub fn next_handle() -> Handle {
    #[expect(non_upper_case_globals)]
    static count: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let v = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Handle::right_padding_from(&v.to_be_bytes())
}

pub fn to_ty(ty: i32) -> ToType {
    ToType::from(ty as u8)
}

pub fn tfhe_log(
    event: TfheContractEvents,
    transaction_hash: Handle,
) -> alloy::rpc::types::Log<TfheContractEvents> {
    alloy::rpc::types::Log {
        inner: tfhe_event(event),
        block_hash: None,
        block_number: None,
        block_timestamp: None,
        transaction_hash: Some(transaction_hash),
        transaction_index: Some(0),
        log_index: None,
        removed: false,
    }
}

pub async fn listener_db(app: &TestInstance) -> ListenerDatabase {
    ListenerDatabase::new(
        &app.db_url().into(),
        ChainId::try_from(12345_u64).unwrap(),
        default_dependence_cache_size(),
    )
    .await
    .unwrap()
}

pub async fn insert_tfhe_event(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    log: alloy::rpc::types::Log<TfheContractEvents>,
    is_allowed: bool,
) -> Result<bool, sqlx::Error> {
    let event = LogTfhe {
        event: log.inner,
        transaction_hash: log.transaction_hash,
        is_allowed,
        block_number: log.block_number.unwrap_or(0),
        block_timestamp: PrimitiveDateTime::MAX,
        dependence_chain: log.transaction_hash.unwrap_or_default(),
        tx_depth_size: 0,
    };
    db.insert_tfhe_event(tx, &event).await
}

pub async fn allow_handle(
    db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    handle: &[u8],
) -> Result<bool, sqlx::Error> {
    let account_address = String::new();
    let event_type = AllowEvents::AllowedForDecryption;
    db.insert_allowed_handle(tx, handle.to_owned(), account_address, event_type, None)
        .await
}
