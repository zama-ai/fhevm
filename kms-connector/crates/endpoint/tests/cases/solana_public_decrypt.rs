use super::*;
use connector_utils::types::{
    ProtocolEventKind, event::from_public_decryption_row,
    solana_request::SolanaPublicDecryptionRequest,
};
use kms_connector_api::SolanaPublicDecryptionRequest as SolanaPublicDecryptionBody;
use sqlx::postgres::PgRow;

fn solana_body(stores: usize) -> SolanaPublicDecryptionBody {
    SolanaPublicDecryptionBody {
        ctHandles: vec![rand_handle(SOLANA_CHAIN_ID), rand_handle(SOLANA_CHAIN_ID)],
        extraData: vec![0x00].into(),
        encryptedStores: (0..stores)
            .map(|i| B256::repeat_byte(0x30 + i as u8))
            .collect(),
    }
}

async fn solana_endpoint() -> anyhow::Result<RunningEndpoint> {
    setup_with(|config| Config {
        supported_chain_ids: vec![SOLANA_CHAIN_ID],
        ..config
    })
    .await
}

fn stored_request(row: &PgRow) -> anyhow::Result<SolanaPublicDecryptionRequest> {
    match from_public_decryption_row(row)?.kind {
        ProtocolEventKind::SolanaPublicDecryption(request) => Ok(request),
        other => anyhow::bail!("not a Solana row: {other:?}"),
    }
}

#[tokio::test]
async fn a_solana_public_request_is_stored_with_the_store_of_each_handle() -> anyhow::Result<()> {
    let endpoint = solana_endpoint().await?;
    let body = solana_body(2);
    let id = body.id();
    assert_ne!(
        id,
        PublicDecryptionRequest {
            ctHandles: body.ctHandles.clone(),
            extraData: body.extraData.clone(),
        }
        .id(),
        "the stores are part of the request id"
    );

    let pending = tokio::spawn({
        let request = endpoint.client.post(endpoint.url(PUBLIC_DECRYPTION_ROUTE));
        let body = serde_json::to_value(&body)?;
        async move { request.json(&body).send().await }
    });
    let row = wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, id).await;
    assert_eq!(row.get::<RequestSource, _>("source"), RequestSource::Http);
    let stored = stored_request(&row)?;
    assert_eq!(stored.decryption_id, db_id(id));
    assert_eq!(stored.ct_handles(), body.ctHandles);
    assert_eq!(stored.encrypted_stores(), body.encryptedStores);
    assert_eq!(stored.extra_data(), body.extraData.as_ref());

    complete_public(&endpoint.db, id).await?;
    assert_eq!(pending.await??.status(), StatusCode::OK);
    endpoint.stop().await
}

#[tokio::test]
async fn a_solana_public_request_names_one_store_per_handle() -> anyhow::Result<()> {
    let endpoint = solana_endpoint().await?;
    for stores in [1, 3] {
        let response = endpoint
            .post_raw(
                PUBLIC_DECRYPTION_ROUTE,
                serde_json::to_string(&solana_body(stores))?,
            )
            .await;
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "{stores} stores"
        );
        assert_eq!(error_body(response).await.code, ErrorCode::Malformed);
    }
    assert_eq!(count_rows(&endpoint.db, PUBLIC_REQUESTS).await, 0);
    endpoint.stop().await
}

#[tokio::test]
async fn a_solana_public_row_naming_another_store_count_is_unwritable() -> anyhow::Result<()> {
    let endpoint = solana_endpoint().await?;
    let body = solana_body(2);
    let id = body.id();
    let _pending = tokio::spawn({
        let request = endpoint.client.post(endpoint.url(PUBLIC_DECRYPTION_ROUTE));
        let body = serde_json::to_value(&body)?;
        async move { request.json(&body).send().await }
    });
    wait_for_request_row(&endpoint.db, PUBLIC_REQUESTS, id).await;

    let error = sqlx::query(
        "UPDATE public_decryption_requests
         SET handle_encrypted_stores = handle_encrypted_stores[1:1] WHERE decryption_id = $1",
    )
    .bind(db_id(id).as_le_slice())
    .execute(&endpoint.db)
    .await
    .unwrap_err();
    assert_eq!(
        error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("public_decryption_requests_encrypted_stores"),
        "{error}"
    );
    endpoint.stop().await
}
