mod common;

use crate::common::{
    CHAIN_ID, PAD_REQUESTS_TABLE, PUBLIC_REQUESTS_TABLE, USER_REQUESTS_TABLE, complete_pad,
    complete_public, complete_udec, rand_public_request, rand_user_request, setup,
    wait_for_request_row,
};
use alloy::{
    sol_types::SolValue,
    transports::http::reqwest::{self, StatusCode},
};
use connector_utils::tests::rand::rand_pad_request;
use connector_utils::types::db::{OperationStatus, RequestSource};
use kms_connector_api::{
    PadDecryptionResponse, PadEntry, PublicDecryptionResponse, UserDecryptionResponse,
    VERSION_ROUTE, VersionResponse,
};
use rstest::rstest;
use sqlx::Row;
use std::time::Duration;
use tracing::info;

#[path = "cases/solana_public_decrypt.rs"]
mod solana_public_decrypt;
#[path = "cases/solana_user_decrypt.rs"]
mod solana_user_decrypt;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_public_decrypt_happy_path() -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let response = reqwest::get(endpoint.url(VERSION_ROUTE)).await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<VersionResponse>().await?,
        VersionResponse::default()
    );

    let request = rand_public_request();
    let id = request.id();
    let pending = endpoint.spawn_post(request.clone());

    info!("Checking the stored request row...");
    let row = wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS_TABLE, id).await;
    assert_eq!(row.get::<RequestSource, _>("source"), RequestSource::Http);
    assert_eq!(
        row.get::<OperationStatus, _>("status"),
        OperationStatus::Pending
    );
    let ct_handles: Vec<Vec<u8>> = row.get("ct_handles");
    assert_eq!(
        ct_handles,
        request
            .ctHandles
            .iter()
            .map(|h| h.to_vec())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        row.get::<Vec<u8>, _>("extra_data"),
        request.extraData.to_vec()
    );

    info!("Playing the worker: inserting the response row...");
    let inserted = complete_public(&endpoint.db, id).await?;

    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body: PublicDecryptionResponse = response.json().await?;
    assert_eq!(body.decryption_id, id);
    assert_eq!(body.decrypted_result.to_vec(), inserted.decrypted_result);
    assert_eq!(body.signature.to_vec(), inserted.signature);
    assert_eq!(body.extra_data.to_vec(), inserted.extra_data);

    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_user_decrypt_happy_path() -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = rand_user_request();
    let id = request.id();
    let pending = endpoint.spawn_post(request.clone());

    info!("Checking the stored RFC016 request row...");
    let row = wait_for_request_row(&endpoint.db, USER_REQUESTS_TABLE, id).await;
    let payload = &request.payload;
    assert_eq!(row.get::<RequestSource, _>("source"), RequestSource::Http);
    assert_eq!(
        row.get::<OperationStatus, _>("status"),
        OperationStatus::Pending
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("ct_handles"),
        vec![payload.handles[0].handle.to_vec()]
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("handle_owner_addresses"),
        vec![payload.handles[0].ownerAddress.to_vec()]
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("handle_contract_addresses"),
        vec![payload.handles[0].contractAddress.to_vec()]
    );
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("allowed_contracts"),
        vec![payload.allowedContracts[0].to_vec()]
    );
    assert_eq!(
        row.get::<Vec<u8>, _>("user_address"),
        payload.userAddress.to_vec()
    );
    assert_eq!(
        row.get::<Vec<u8>, _>("public_key"),
        payload.publicKey.to_vec()
    );
    assert_eq!(
        row.get::<i64, _>("start_timestamp"),
        payload.requestValidity.startTimestamp as i64
    );
    assert_eq!(
        row.get::<i64, _>("duration_seconds"),
        payload.requestValidity.durationSeconds as i64
    );
    assert_eq!(
        row.get::<Option<Vec<u8>>, _>("signature"),
        Some(request.signature.to_vec())
    );

    info!("Playing the worker: inserting the response row...");
    let inserted = complete_udec(&endpoint.db, id).await?;

    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body: UserDecryptionResponse = response.json().await?;
    assert_eq!(body.decryption_id, id);
    assert_eq!(
        body.user_decrypted_shares.to_vec(),
        inserted.user_decrypted_shares
    );
    assert_eq!(body.signature.to_vec(), inserted.signature);

    endpoint.stop().await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pad_decrypt_happy_path() -> anyhow::Result<()> {
    let endpoint = setup().await?;

    let request = rand_pad_request(CHAIN_ID);
    let id = request.id();
    let pending = endpoint.spawn_post(request.clone());

    info!("Checking the stored pad request row...");
    let row = wait_for_request_row(&endpoint.db, PAD_REQUESTS_TABLE, id).await;
    let payload = &request.payload;
    assert_eq!(
        row.get::<OperationStatus, _>("status"),
        OperationStatus::Pending
    );
    assert_eq!(
        row.get::<i16, _>("witness_version"),
        payload.witnessVersion as i16
    );
    assert_eq!(row.get::<i32, _>("prf_id"), i32::from(payload.prfId));
    assert_eq!(row.get::<i64, _>("chain_id"), payload.chainId as i64);
    let pads: Vec<PadEntry> = SolValue::abi_decode(&row.get::<Vec<u8>, _>("pads"))?;
    assert_eq!(pads, payload.pads);
    assert_eq!(
        row.get::<Vec<u8>, _>("signature"),
        request.signature.to_vec()
    );

    info!("Playing the worker: inserting the response row...");
    let inserted = complete_pad(&endpoint.db, id).await?;

    let response = pending.await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.json::<PadDecryptionResponse>().await?, inserted);

    endpoint.stop().await
}
