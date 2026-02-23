use alloy::primitives::{Address, FixedBytes, Log};
use bigdecimal::num_bigint::BigInt;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::types::AllowEvents;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    ClearConst, Database as ListenerDatabase, Handle, LogTfhe, ToType, Transaction,
};
use sqlx::types::time::PrimitiveDateTime;

use crate::tests::utils::{
    decrypt_ciphertexts, default_dependence_cache_size, setup_test_app,
    wait_until_all_allowed_handles_computed, DecryptionResult, TestInstance,
};

pub const TEST_CHAIN_ID: u64 = 42;

pub struct EventHarness {
    pub app: TestInstance,
    pub pool: sqlx::PgPool,
    pub listener_db: ListenerDatabase,
}

pub async fn setup_event_harness() -> Result<EventHarness, Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener_db = ListenerDatabase::new(
        &app.db_url().into(),
        ChainId::try_from(TEST_CHAIN_ID).unwrap(),
        default_dependence_cache_size(),
    )
    .await?;
    Ok(EventHarness {
        app,
        pool,
        listener_db,
    })
}

pub fn next_handle() -> Handle {
    #[expect(non_upper_case_globals)]
    static count: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let v = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut out = [0_u8; 32];
    // Keep generated test handles in a namespace disjoint from scalar-encoded handles.
    out[0] = 0x80;
    out[24..].copy_from_slice(&v.to_be_bytes());
    Handle::from(out)
}

pub fn zero_address() -> Address {
    "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap()
}

pub fn to_ty(ty: i32) -> ToType {
    ToType::from(ty as u8)
}

pub fn as_scalar_uint(value: u64) -> ClearConst {
    let (_, bytes) = BigInt::from(value).to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

pub fn scalar_flag(is_scalar: bool) -> FixedBytes<1> {
    FixedBytes::from([if is_scalar { 1_u8 } else { 0_u8 }])
}

pub fn scalar_u128_handle(value: u128) -> Handle {
    let mut out = [0_u8; 32];
    out[16..].copy_from_slice(&value.to_be_bytes());
    Handle::from(out)
}

pub fn tfhe_event(data: TfheContractEvents) -> Log<TfheContractEvents> {
    Log::<TfheContractEvents> {
        address: zero_address(),
        data,
    }
}

pub fn log_with_tx(
    tx_hash: Handle,
    inner: Log<TfheContractEvents>,
) -> alloy::rpc::types::Log<TfheContractEvents> {
    alloy::rpc::types::Log {
        inner,
        block_hash: None,
        block_number: None,
        block_timestamp: None,
        transaction_hash: Some(tx_hash),
        transaction_index: Some(0),
        log_index: None,
        removed: false,
    }
}

pub async fn insert_event(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    event: TfheContractEvents,
    is_allowed: bool,
) -> Result<(), sqlx::Error> {
    let event = LogTfhe {
        event: log_with_tx(tx_id, tfhe_event(event)).inner,
        transaction_hash: Some(tx_id),
        is_allowed,
        block_number: 0,
        block_timestamp: PrimitiveDateTime::MAX,
        dependence_chain: tx_id,
        tx_depth_size: 0,
    };
    listener_db.insert_tfhe_event(tx, &event).await?;
    Ok(())
}

pub async fn insert_trivial_encrypt(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    value: u64,
    to_type: i32,
    result: Handle,
    is_allowed: bool,
) -> Result<(), sqlx::Error> {
    use host_listener::contracts::TfheContract;
    insert_event(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
            caller: zero_address(),
            pt: as_scalar_uint(value),
            toType: to_ty(to_type),
            result,
        }),
        is_allowed,
    )
    .await
}

pub async fn allow_handle(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    handle: &Handle,
) -> Result<(), sqlx::Error> {
    listener_db
        .insert_allowed_handle(
            tx,
            handle.to_vec(),
            String::new(),
            AllowEvents::AllowedForDecryption,
            None,
        )
        .await?;
    Ok(())
}

pub async fn decrypt_handles(
    pool: &sqlx::PgPool,
    handles: &[Handle],
) -> Result<Vec<DecryptionResult>, Box<dyn std::error::Error>> {
    let request = handles.iter().map(|h| h.to_vec()).collect::<Vec<_>>();
    decrypt_ciphertexts(pool, request).await
}

pub async fn wait_until_computed(app: &TestInstance) -> Result<(), Box<dyn std::error::Error>> {
    wait_until_all_allowed_handles_computed(app).await
}

pub async fn wait_for_error(
    pool: &sqlx::PgPool,
    output_handle: &[u8],
    tx_id: &[u8],
) -> Result<(bool, Option<String>), Box<dyn std::error::Error>> {
    let mut last_error = None;
    for _ in 0..80 {
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        let row = sqlx::query_as::<_, (bool, bool, Option<String>)>(
            r#"SELECT is_error, is_completed, error_message
               FROM computations
               WHERE output_handle = $1 AND transaction_id = $2"#,
        )
        .bind(output_handle)
        .bind(tx_id)
        .fetch_optional(pool)
        .await?;
        if let Some((is_error, is_completed, msg)) = row {
            last_error = msg;
            if is_error || is_completed {
                return Ok((is_error, last_error));
            }
        }
    }
    Ok((false, last_error))
}
