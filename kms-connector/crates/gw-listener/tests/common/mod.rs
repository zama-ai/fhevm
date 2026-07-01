#![allow(dead_code)]

use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use anyhow::anyhow;
use connector_utils::{
    tests::{
        db::requests::TestEventType,
        rand::{
            rand_address, rand_digest, rand_kms_node_params, rand_kms_thresholds, rand_pcr_values,
            rand_public_key, rand_signature, rand_u256,
        },
        setup::TestInstance,
    },
    types::{KMS_CONTEXT_COUNTER_BASE, ProtocolEventKind, db::ParamsTypeDb},
};
use fhevm_gateway_bindings::decryption::{
    Decryption::{
        HandleEntry, PublicDecryptionRequest, UserDecryptionRequest_0 as UserDecryptionRequest,
        UserDecryptionRequest_1 as UserDecryptionRequestV2,
    },
    IDecryption::{
        ContractsInfo, RequestValidity, RequestValiditySeconds, UserDecryptionRequestPayload,
    },
};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::{
        AbortCrsgen, AbortKeygen, CrsgenRequest, KeygenRequest, PrepKeygenRequest,
    },
    protocol_config::ProtocolConfig::{NewKmsContext, NewKmsEpoch},
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
    config.protocol_config_contract.address = *test_instance.protocol_config_contract().address();
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
    event_type: TestEventType,
) -> anyhow::Result<(ProtocolEventKind, Option<u64>)> {
    info!("Mocking {event_type} on Anvil...");
    let (pending_tx, event) = match event_type {
        TestEventType::PublicDecryption => {
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
        TestEventType::UserDecryption => {
            let rand_user_addr = rand_address();
            let rand_pub_key = rand_public_key();
            let event = UserDecryptionRequest {
                userAddress: rand_user_addr,
                publicKey: rand_pub_key.clone().into(),
                ..Default::default()
            };
            let tx = test_instance
                .decryption_contract()
                .userDecryptionRequest_1(
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
        TestEventType::UserDecryptionV2 => {
            let handles = vec![HandleEntry {
                handle: rand_digest(),
                contractAddress: rand_address(),
                ownerAddress: rand_address(),
            }];
            let payload = UserDecryptionRequestPayload {
                userAddress: rand_address(),
                publicKey: rand_public_key().into(),
                allowedContracts: vec![],
                requestValidity: RequestValiditySeconds::default(),
                extraData: vec![].into(),
                signature: rand_signature().into(),
            };
            let event = UserDecryptionRequestV2 {
                decryptionId: U256::ZERO,
                snsCtMaterials: vec![],
                handles: handles.clone(),
                payload: payload.clone(),
            };
            let tx = test_instance
                .decryption_contract()
                .userDecryptionRequest_0(
                    handles,
                    payload.userAddress,
                    payload.publicKey,
                    payload.allowedContracts,
                    payload.requestValidity,
                    payload.signature,
                    payload.extraData,
                )
                .send()
                .await?;
            (tx, event.into())
        }
        TestEventType::PrepKeygen => {
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
        TestEventType::Keygen => {
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
        TestEventType::Crsgen => {
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
        // TODO should this be changed to take a prepKeygenId as input?
        TestEventType::AbortKeygen => {
            let rand_prep_id = rand_u256();
            let event = AbortKeygen {
                prepKeygenId: rand_prep_id,
            };
            let tx = test_instance
                .kms_generation_contract()
                .abortKeygen(rand_prep_id)
                .send()
                .await?;
            (tx, event.into())
        }
        // TODO should this be changed to take a crsId as input?
        TestEventType::AbortCrsgen => {
            let rand_crs_id = rand_u256();
            let event = AbortCrsgen { crsId: rand_crs_id };
            let tx = test_instance
                .kms_generation_contract()
                .abortCrsgen(rand_crs_id)
                .send()
                .await?;
            (tx, event.into())
        }
        TestEventType::NewKmsContext => {
            let thresholds = rand_kms_thresholds();
            let kms_node_params = vec![rand_kms_node_params()];
            let software_version = format!("v{}", rand_u256());
            let pcr_values = vec![rand_pcr_values()];
            // `defineNewKmsContextAndEpoch` emits a non-genesis switch: contextId = base + 2,
            // previousContextId = base + 1 (a real, non-sentinel predecessor).
            let event = NewKmsContext {
                contextId: KMS_CONTEXT_COUNTER_BASE + U256::from(2),
                previousContextId: KMS_CONTEXT_COUNTER_BASE + U256::ONE,
                kmsNodeParams: kms_node_params.clone(),
                thresholds: thresholds.clone(),
                softwareVersion: software_version.clone(),
                pcrValues: pcr_values.clone(),
            };
            let tx = test_instance
                .protocol_config_contract()
                .defineNewKmsContextAndEpoch(
                    kms_node_params,
                    thresholds,
                    software_version,
                    pcr_values,
                )
                .send()
                .await?;
            (tx, event.into())
        }
        TestEventType::NewKmsEpoch => {
            let event = NewKmsEpoch {
                kmsContextId: KMS_CONTEXT_COUNTER_BASE + U256::ONE,
                previousContextId: KMS_CONTEXT_COUNTER_BASE + U256::ONE,
                epochId: KMS_EPOCH_ID_COUNTER + U256::ONE,
                previousEpochId: KMS_EPOCH_ID_COUNTER,
                // The mock emits `block.number`; the value isn't asserted (the DB check matches on
                // context_id + epoch_id only), so any placeholder works here.
                materialBlockNumber: U256::ZERO,
            };
            let tx = test_instance
                .protocol_config_contract()
                .defineNewEpochForCurrentKmsContext()
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

pub async fn fetch_from_db(
    db: &Pool<Postgres>,
    event_type: TestEventType,
) -> sqlx::Result<Vec<PgRow>> {
    info!("Checking {event_type} is stored in DB...");
    let query = match event_type {
        TestEventType::PublicDecryption => "SELECT * FROM public_decryption_requests",
        TestEventType::UserDecryption | TestEventType::UserDecryptionV2 => {
            "SELECT * FROM user_decryption_requests"
        }
        TestEventType::PrepKeygen => "SELECT * FROM prep_keygen_requests",
        TestEventType::Keygen => "SELECT * FROM keygen_requests",
        TestEventType::Crsgen => "SELECT * FROM crsgen_requests",
        TestEventType::AbortKeygen => "SELECT * FROM abort_keygen_requests",
        TestEventType::AbortCrsgen => "SELECT * FROM abort_crsgen_requests",
        TestEventType::NewKmsContext => "SELECT * FROM new_kms_context",
        TestEventType::NewKmsEpoch => "SELECT * FROM new_kms_epoch",
    };
    sqlx::query(query).fetch_all(db).await
}

pub async fn poll_db_for_event(
    db: &Pool<Postgres>,
    event_type: TestEventType,
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
        ProtocolEventKind::UserDecryptionV2(e) => {
            for r in rows {
                if e.payload.signature.to_vec() == r.try_get::<Vec<u8>, _>("signature")?
                    && e.payload.userAddress
                        == Address::from(r.try_get::<[u8; 20], _>("user_address")?)
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
        // TODO not completely sure this is the right approach since it is not used for prep key gen and crs gen
        ProtocolEventKind::AbortKeygen(e) => {
            for r in rows {
                if e.prepKeygenId
                    == U256::from_le_bytes(r.try_get::<[u8; 32], _>("prep_keygen_id")?)
                {
                    return Ok(());
                }
            }
        }
        ProtocolEventKind::AbortCrsgen(e) => {
            for r in rows {
                if e.crsId == U256::from_le_bytes(r.try_get::<[u8; 32], _>("crs_id")?) {
                    return Ok(());
                }
            }
        }
        ProtocolEventKind::NewKmsContext(e) => {
            for r in rows {
                if e.softwareVersion == r.try_get::<String, _>("software_version")? {
                    return Ok(());
                }
            }
        }
        ProtocolEventKind::NewKmsEpoch(e) => {
            if matches_context_epoch(rows, e.kmsContextId, e.epochId)? {
                return Ok(());
            }
        }
    };
    Err(anyhow!("Event not found in DB..."))
}

fn matches_context_epoch(rows: &[PgRow], context_id: U256, epoch_id: U256) -> anyhow::Result<bool> {
    for r in rows {
        let row_context: Vec<u8> = r.try_get("context_id")?;
        let row_epoch: Vec<u8> = r.try_get("epoch_id")?;
        if context_id.as_le_slice() == row_context && epoch_id.as_le_slice() == row_epoch {
            return Ok(true);
        }
    }
    Ok(false)
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

pub const KMS_EPOCH_ID_COUNTER: U256 = U256::from_be_bytes([
    8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);
