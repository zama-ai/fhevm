use super::*;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::rand::solana_user_decryption_event,
    types::{
        ProtocolEventKind,
        db::{AttestationType as RowAttestationType, insert_solana_user_decryption},
        event::from_user_decryption_row,
        solana_request::SolanaUserDecryptionRequestV1,
    },
};
use kms_connector_api::{
    SolanaHandleEntry, SolanaUserDecryptionPayload, SolanaUserDecryptionRequest,
};
use sqlx::postgres::PgRow;

/// The Solana chain type byte over cluster tag 12345.
const SOLANA_CHAIN_ID: u64 = 0x0100_0000_0000_3039;

#[tokio::test]
async fn solana_http_and_gateway_requests_store_the_same_request() -> anyhow::Result<()> {
    let endpoint = setup_with(|config| Config {
        supported_chain_ids: vec![SOLANA_CHAIN_ID],
        ..config
    })
    .await?;
    let event = solana_user_decryption_event(U256::from(987), rand_handle(SOLANA_CHAIN_ID));
    let gateway = SolanaUserDecryptionRequestV1::try_from(event)?;
    let body = http_body(&gateway);
    let id = body.id();

    let pending = tokio::spawn({
        let request = endpoint.client.post(endpoint.url(USER_DECRYPTION_ROUTE));
        async move { request.json(&body).send().await }
    });
    let http_row = wait_for_request_row(&endpoint.db, USER_REQUESTS, id).await;
    assert_eq!(
        http_row.get::<RowAttestationType, _>("attestation_type"),
        RowAttestationType::Solana
    );
    assert_eq!(
        http_row.get::<Vec<u8>, _>("user_address"),
        gateway.permit().user_address().as_bytes()
    );

    insert_solana_user_decryption(
        &endpoint.db,
        &gateway,
        Some(B256::repeat_byte(9)),
        sqlx::types::chrono::Utc::now(),
        &PropagationContext::default(),
        RequestSource::OnChain,
    )
    .await?;
    let gateway_row =
        sqlx::query("SELECT * FROM user_decryption_requests WHERE decryption_id = $1")
            .bind(gateway.decryption_id.as_le_slice())
            .fetch_one(&endpoint.db)
            .await?;

    let mut from_http = stored_request(&http_row)?;
    assert_eq!(from_http.decryption_id, db_id(id));
    from_http.decryption_id = gateway.decryption_id;
    assert_eq!(from_http, gateway);
    assert_eq!(stored_request(&gateway_row)?, gateway);

    complete_udec(&endpoint.db, id).await?;
    assert_eq!(pending.await??.status(), StatusCode::OK);
    endpoint.stop().await
}

#[tokio::test]
async fn solana_rows_of_the_wrong_shape_are_unwritable() -> anyhow::Result<()> {
    let endpoint = setup().await?;
    let request = SolanaUserDecryptionRequestV1::try_from(solana_user_decryption_event(
        U256::from(1),
        rand_handle(SOLANA_CHAIN_ID),
    ))?;
    insert_solana_user_decryption(
        &endpoint.db,
        &request,
        None,
        sqlx::types::chrono::Utc::now(),
        &PropagationContext::default(),
        RequestSource::Http,
    )
    .await?;
    let id = request.decryption_id;
    for change in [
        "user_address = decode(repeat('00', 20), 'hex')",
        "handle_owner_addresses = NULL",
        "signature = NULL",
        "signature = decode(repeat('00', 63), 'hex')",
        "verifying_program_id = NULL",
        "handle_encrypted_stores = ARRAY[handle_encrypted_stores[1], handle_encrypted_stores[1]]",
        "allowed_contracts = ARRAY[decode(repeat('00', 20), 'hex')]",
    ] {
        let error = update(&endpoint.db, id, change).await.unwrap_err();
        assert_eq!(
            error
                .as_database_error()
                .and_then(|error| error.constraint()),
            Some("user_decryption_requests_attestation_columns"),
            "{change}: {error}"
        );
    }

    // Array element widths are the reader's to enforce, so a worker never acts on a truncated field.
    update(
        &endpoint.db,
        id,
        "handle_encrypted_stores = ARRAY[decode('00', 'hex')]",
    )
    .await?;
    let row = sqlx::query("SELECT * FROM user_decryption_requests WHERE decryption_id = $1")
        .bind(id.as_le_slice())
        .fetch_one(&endpoint.db)
        .await?;
    assert!(from_user_decryption_row(&row).is_err());
    endpoint.stop().await
}

#[tokio::test]
async fn solana_attestation_type_with_an_eip712_payload_is_malformed() -> anyhow::Result<()> {
    let endpoint = setup().await?;
    let mut body = serde_json::to_value(user_request())?;
    body["attestationType"] = AttestationType::SolanaSrfc38UserDecryptV1
        .to_string()
        .into();
    let response = endpoint
        .post_raw(USER_DECRYPTION_ROUTE, serde_json::to_string(&body)?)
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(error_body(response).await.code, ErrorCode::Malformed);
    assert_eq!(count_rows(&endpoint.db, USER_REQUESTS).await, 0);
    endpoint.stop().await
}

fn http_body(request: &SolanaUserDecryptionRequestV1) -> SolanaUserDecryptionRequest {
    let permit = request.permit();
    SolanaUserDecryptionRequest {
        attestationType: AttestationType::SolanaSrfc38UserDecryptV1.to_string(),
        payload: SolanaUserDecryptionPayload {
            handles: request
                .handles()
                .iter()
                .map(|entry| SolanaHandleEntry {
                    handle: entry.handle.into(),
                    ownerAddress: entry.owner_address.into(),
                    encryptedStore: entry.encrypted_store.into(),
                })
                .collect(),
            userAddress: (*permit.user_address().as_bytes()).into(),
            publicKey: permit.transport_key().as_bytes().to_vec().into(),
            allowedScopes: vec![],
            requestValidity: RequestValidity {
                startTimestamp: permit.start_timestamp(),
                durationSeconds: permit.duration_seconds(),
            },
            verifyingProgramId: (*permit.verifying_program_id().as_bytes()).into(),
            extraData: request.extra_data().into(),
        },
        signature: request.signature().as_bytes().to_vec().into(),
    }
}

async fn update(db: &Pool<Postgres>, id: U256, change: &str) -> sqlx::Result<()> {
    sqlx::query(&format!(
        "UPDATE user_decryption_requests SET {change} WHERE decryption_id = $1"
    ))
    .bind(id.as_le_slice())
    .execute(db)
    .await
    .map(drop)
}

fn stored_request(row: &PgRow) -> anyhow::Result<SolanaUserDecryptionRequestV1> {
    match from_user_decryption_row(row)?.kind {
        ProtocolEventKind::SolanaUserDecryptionV1(request) => Ok(request),
        other => anyhow::bail!("not a Solana row: {other:?}"),
    }
}
