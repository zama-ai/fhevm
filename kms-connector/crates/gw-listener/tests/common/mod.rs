#![allow(dead_code)]

use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use anyhow::anyhow;
use connector_utils::{
    tests::{
        rand::{rand_address, rand_public_key, rand_signature, rand_u256},
        setup::TestInstance,
    },
    types::{
        ProtocolEventKind,
        db::{EventType, ParamsTypeDb},
    },
};
use fhevm_gateway_bindings::decryption::{
    Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    IDecryption::{ContractsInfo, RequestValidity},
};
use fhevm_host_bindings::kms_generation::KMSGeneration::{
    CrsgenRequest, KeygenRequest, PrepKeygenRequest,
};
use gw_listener::core::{Config, EthereumListener, EventListener, GatewayListener};
use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub async fn start_test_listener(
    test_instance: &mut TestInstance,
    cancel_token: CancellationToken,
    from_block_number: Option<u64>,
) -> anyhow::Result<JoinHandle<()>> {
    let mut config = Config::default();
    config.decryption_contract.address = *test_instance.decryption_contract().address();
    config.kms_generation_contract.address = *test_instance.kms_generation_contract().address();
    config.kms_verifier_address = test_instance.kms_verifier_address();
    config.decryption_from_block_number = from_block_number;
    config.kms_operation_from_block_number = from_block_number;
    config.decryption_polling = Duration::from_millis(300);
    config.key_management_polling = Duration::from_millis(300);

    let gateway_listener = GatewayListener::new(
        test_instance.db().clone(),
        test_instance.provider().clone(),
        &config,
        cancel_token.clone(),
    );
    let ethereum_listener = EthereumListener::new(
        test_instance.db().clone(),
        test_instance.provider().clone(),
        &config,
        cancel_token,
    );
    let event_listener = EventListener::new(gateway_listener, ethereum_listener);

    let listener_task = tokio::spawn(async move {
        event_listener.start().await.unwrap();
    });

    // Wait for 2 anvil blocks for listener to be ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    Ok(listener_task)
}

pub async fn mock_event_on_gw(
    test_instance: &TestInstance,
    event_type: EventType,
) -> anyhow::Result<(ProtocolEventKind, Option<u64>)> {
    info!("Mocking {event_type} on Anvil...");
    let (pending_tx, event) = match event_type {
        EventType::PublicDecryptionRequest => {
            let rand_extra_data = rand_signature();
            let event = PublicDecryptionRequest {
                extraData: rand_extra_data.clone().into(),
                ..Default::default()
            };
            let tx = test_instance
                .decryption_contract()
                .publicDecryptionRequest(vec![], rand_extra_data.into())
                .send()
                .await?;
            (tx, event.into())
        }
        EventType::UserDecryptionRequest => {
            let rand_user_addr = rand_address();
            let rand_pub_key = rand_public_key();
            let event = UserDecryptionRequest {
                userAddress: rand_user_addr,
                publicKey: rand_pub_key.clone().into(),
                ..Default::default()
            };
            let tx = test_instance
                .decryption_contract()
                .userDecryptionRequest(
                    vec![],
                    RequestValidity::default(),
                    ContractsInfo::default(),
                    rand_user_addr,
                    rand_pub_key.into(),
                    vec![].into(),
                    vec![].into(),
                )
                .send()
                .await?;
            (tx, event.into())
        }
        EventType::PrepKeygenRequest => {
            let event = PrepKeygenRequest {
                paramsType: ParamsTypeDb::Test as u8,
                ..Default::default()
            };
            let tx = test_instance
                .kms_generation_contract()
                .keygen(ParamsTypeDb::Test as u8)
                .send()
                .await?;
            (tx, event.into())
        }
        EventType::KeygenRequest => {
            let rand_prep_id = rand_u256();
            let event = KeygenRequest {
                prepKeygenId: rand_prep_id,
                ..Default::default()
            };
            let tx = test_instance
                .kms_generation_contract()
                .prepKeygenResponse(rand_prep_id, rand_signature().into())
                .send()
                .await?;
            (tx, event.into())
        }
        EventType::CrsgenRequest => {
            let rand_max_bit_length = rand_u256();
            let event = CrsgenRequest {
                maxBitLength: rand_max_bit_length,
                ..Default::default()
            };
            let tx = test_instance
                .kms_generation_contract()
                .crsgenRequest(rand_max_bit_length, ParamsTypeDb::Test as u8)
                .send()
                .await?;
            (tx, event.into())
        }
    };
    let receipt = pending_tx.get_receipt().await?;
    let block_number = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap()
        .block_number;
    info!(block_number, "Tx successfully sent!");
    Ok((event, block_number))
}

pub async fn fetch_from_db(db: &Pool<Postgres>, event_type: EventType) -> sqlx::Result<Vec<PgRow>> {
    info!("Checking {event_type} is stored in DB...");
    let query = match event_type {
        EventType::PublicDecryptionRequest => "SELECT * FROM public_decryption_requests",
        EventType::UserDecryptionRequest => "SELECT * FROM user_decryption_requests",
        EventType::PrepKeygenRequest => "SELECT * FROM prep_keygen_requests",
        EventType::KeygenRequest => "SELECT * FROM keygen_requests",
        EventType::CrsgenRequest => "SELECT * FROM crsgen_requests",
    };
    sqlx::query(query).fetch_all(db).await
}

pub async fn poll_db_for_event(
    db: &Pool<Postgres>,
    event_type: EventType,
    expected_event: &ProtocolEventKind,
) -> anyhow::Result<()> {
    let timeout = Duration::from_secs(30);
    let poll_interval = Duration::from_millis(200);
    let start = std::time::Instant::now();
    loop {
        let rows = fetch_from_db(db, event_type).await?;
        if check_event_in_db(&rows, expected_event.clone()).is_ok() {
            return Ok(());
        }
        if start.elapsed() > timeout {
            anyhow::bail!("Timed out waiting for {event_type} event in DB");
        }
        tokio::time::sleep(poll_interval).await;
    }
}

pub fn check_event_in_db(rows: &[PgRow], event: ProtocolEventKind) -> anyhow::Result<()> {
    match event {
        ProtocolEventKind::PublicDecryption(e) => {
            for r in rows {
                if e.extraData.to_vec() == r.try_get::<Vec<u8>, _>("extra_data")? {
                    return Ok(());
                }
            }
        }
        ProtocolEventKind::UserDecryption(e) => {
            for r in rows {
                if e.publicKey.to_vec() == r.try_get::<Vec<u8>, _>("public_key")?
                    && e.userAddress == Address::from(r.try_get::<[u8; 20], _>("user_address")?)
                {
                    return Ok(());
                }
            }
        }
        ProtocolEventKind::PrepKeygen(_) => {
            for r in rows {
                if r.try_get::<ParamsTypeDb, _>("params_type")? == ParamsTypeDb::Test {
                    return Ok(());
                }
            }
        }
        ProtocolEventKind::Keygen(e) => {
            for r in rows {
                if e.prepKeygenId
                    == U256::from_le_bytes(r.try_get::<[u8; 32], _>("prep_keygen_id")?)
                {
                    return Ok(());
                }
            }
        }
        ProtocolEventKind::Crsgen(e) => {
            for r in rows {
                if e.maxBitLength
                    == U256::from_le_bytes(r.try_get::<[u8; 32], _>("max_bit_length")?)
                {
                    return Ok(());
                }
            }
        }
    };
    Err(anyhow!("Event not found in DB..."))
}

pub const PUB_DECRYPTION_COUNTER: U256 = U256::from_be_bytes([
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

pub const USR_DECRYPTION_COUNTER: U256 = U256::from_be_bytes([
    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

pub const PREP_KEY_COUNTER: U256 = U256::from_be_bytes([
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

pub const KEY_COUNTER: U256 = U256::from_be_bytes([
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

pub const CRS_COUNTER: U256 = U256::from_be_bytes([
    5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

pub const KEY_RESHARE_ID_COUNTER: U256 = U256::from_be_bytes([
    6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);
