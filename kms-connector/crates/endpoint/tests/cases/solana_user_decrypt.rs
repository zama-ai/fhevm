use super::*;

#[tokio::test]
async fn solana_http_and_gateway_reconstruct_the_same_request() -> anyhow::Result<()> {
    use connector_utils::{
        monitoring::otlp::PropagationContext,
        types::{
            ProtocolEventKind, db::insert_solana_user_decryption, event::from_user_decryption_row,
            solana_request::SolanaUserDecryptionRequestV1,
        },
    };
    use fhevm_gateway_bindings::decryption::{
        Decryption::UserDecryptionRequest_4, IDecryption::RequestValiditySeconds,
    };
    use kms_connector_api::{
        SolanaHandleEntry, SolanaUserDecryptionPayload, SolanaUserDecryptionRequest,
    };
    use zama_solana_permit::PermitWireFields;
    use zama_solana_request::{
        SolanaHandleEntryWire, SolanaUserDecryptRequestWire, encode_solana_request,
    };
    const CHAIN_ID: u64 = 72057594037940281;
    let endpoint = setup_with(|mut config| {
        config.supported_chain_ids = vec![CHAIN_ID];
        config
    })
    .await?;
    let handle = rand_handle(CHAIN_ID);
    let typed =
        connector_utils::tests::rand::solana_user_decryption_request(U256::from(987), handle);
    let permit = typed.request.permit();
    let payload = SolanaUserDecryptionPayload {
        handles: vec![SolanaHandleEntry {
            handle,
            allowedKey: (*permit.user_pubkey().as_bytes()).into(),
            encryptedStore: [3; 32].into(),
        }],
        userPubkey: (*permit.user_pubkey().as_bytes()).into(),
        publicKey: permit.transport_key().as_bytes().to_vec().into(),
        allowedScopes: vec![],
        requestValidity: RequestValidity {
            startTimestamp: permit.start_timestamp(),
            durationSeconds: permit.duration_seconds(),
        },
        hostProgramId: (*permit.verifying_program_id().as_bytes()).into(),
        extraData: permit.extra_data().to_extra_data().into(),
    };
    let request = SolanaUserDecryptionRequest {
        attestationType: AttestationType::SolanaSrfc38UserDecryptV1.to_string(),
        payload,
        signature: typed.request.signature().as_bytes().to_vec().into(),
    };
    let id = request.id();
    let client = endpoint.client.clone();
    let url = endpoint.url(USER_DECRYPTION_ROUTE);
    let body = request.clone();
    let pending = tokio::spawn(async move { client.post(url).json(&body).send().await });
    let http_row = wait_for_request_row(&endpoint.db, USER_REQUESTS, id).await;
    assert_eq!(
        http_row.get::<String, _>("attestation_type"),
        "solana-srfc38-user-decrypt-v1"
    );
    assert!(http_row.get::<Option<Vec<u8>>, _>("user_address").is_none());
    let wire = SolanaUserDecryptRequestWire {
        permit: PermitWireFields {
            user_pubkey: request.payload.userPubkey.to_vec(),
            transport_key: request.payload.publicKey.to_vec(),
            allowed_scopes: vec![],
            start_timestamp: permit.start_timestamp(),
            duration_seconds: permit.duration_seconds(),
            verifying_program_id: request.payload.hostProgramId.to_vec(),
            chain_id: CHAIN_ID,
            extra_data: request.payload.extraData.to_vec(),
        },
        signature: request.signature.to_vec(),
        handles: vec![SolanaHandleEntryWire {
            handle: handle.to_vec(),
            allowed_key: vec![1; 32],
            encrypted_store: vec![3; 32],
        }],
    };
    let event = UserDecryptionRequest_4 {
        decryptionId: typed.decryption_id,
        ctHandles: vec![handle],
        requestValidity: RequestValiditySeconds {
            startTimestamp: U256::from(permit.start_timestamp()),
            durationSeconds: U256::from(permit.duration_seconds()),
        },
        publicKey: request.payload.publicKey.clone(),
        extraData: request.payload.extraData.clone(),
        solanaRequest: encode_solana_request(&wire)?.into(),
    };
    let gateway_request = SolanaUserDecryptionRequestV1::try_from(event)?;
    insert_solana_user_decryption(
        &endpoint.db,
        &gateway_request,
        Some(B256::repeat_byte(9)),
        sqlx::types::chrono::Utc::now(),
        &PropagationContext::default(),
        RequestSource::OnChain,
    )
    .await?;
    let gw_row = sqlx::query("SELECT * FROM user_decryption_requests WHERE decryption_id = $1")
        .bind(typed.decryption_id.as_le_slice())
        .fetch_one(&endpoint.db)
        .await?;
    let ProtocolEventKind::SolanaUserDecryptionV1(http) = from_user_decryption_row(&http_row)?.kind
    else {
        panic!("wrong HTTP row type")
    };
    let ProtocolEventKind::SolanaUserDecryptionV1(gateway) =
        from_user_decryption_row(&gw_row)?.kind
    else {
        panic!("wrong Gateway row type")
    };
    assert_eq!(http.request, gateway.request);
    assert_ne!(http.decryption_id, gateway.decryption_id);

    // Invalid identities, missing fields, and misaligned/non-vector arrays cannot enter the queue.
    for change in [
        "user_address = decode(repeat('00', 20), 'hex')",
        "user_pubkey = NULL",
        "signature = decode('00', 'hex')",
        "allowed_keys = ARRAY[NULL]::bytea[]",
        "allowed_keys = ARRAY[]::bytea[]",
        "allowed_keys = ARRAY[allowed_keys]",
        "host_program_id = decode('00', 'hex')",
        "allowed_scopes = ARRAY[decode('00','hex')]",
        "allowed_scopes = ARRAY[decode(repeat('ff',64),'hex'),decode(repeat('00',64),'hex')]",
        "allowed_scopes = ARRAY[decode(repeat('00',64),'hex'),decode(repeat('00',64),'hex')]",
        "ct_handles = ARRAY[ct_handles[1],set_byte(ct_handles[1],23,99)], allowed_keys = ARRAY[allowed_keys[1],allowed_keys[1]], encrypted_stores = ARRAY[encrypted_stores[1],encrypted_stores[1]]",
        "attestation_type = 'legacy'",
        "duration_seconds = 0",
    ] {
        let result = sqlx::query(&format!(
            "UPDATE user_decryption_requests SET {change} WHERE decryption_id = $1"
        ))
        .bind(db_id(id).as_le_slice())
        .execute(&endpoint.db)
        .await;
        assert!(result.is_err(), "invalid row accepted: {change}");
    }
    complete_udec(&endpoint.db, id).await?;
    assert_eq!(pending.await??.status(), StatusCode::OK);
    endpoint.stop().await
}

#[tokio::test]
async fn solana_http_rejects_mismatched_scheme() -> anyhow::Result<()> {
    let endpoint = setup().await?;
    let mut body = serde_json::to_value(user_request())?;
    body["attestationType"] = "solana-srfc38-user-decrypt-v1".into();
    let response = endpoint
        .post_raw(USER_DECRYPTION_ROUTE, serde_json::to_string(&body)?)
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.json::<ErrorResponse>().await?.code,
        ErrorCode::Malformed
    );
    endpoint.stop().await
}
