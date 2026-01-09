#[cfg(test)]
mod tests {
    use actix_web::{App, test, web::Data};
    use alloy::hex;
    use connector_utils::{
        monitoring::otlp::PropagationContext,
        tests::{rand::{rand_address, rand_contract_addresses, rand_handles, rand_public_key, rand_signature, rand_u256}, setup::TestInstanceBuilder},
        types::db::OperationStatus,
    };

    use crate::api::{ApiState, handlers::get_share_handler};

    #[actix_web::test]
    async fn test_get_share_user_ready() -> anyhow::Result<()> {
        let test_instance = TestInstanceBuilder::db_setup().await?;
        let request_id = rand_u256();
        let handles = rand_handles(1);
        let contract_addresses = rand_contract_addresses(1);
        let user_address = rand_address();
        let public_key = rand_public_key();
        let user_signature = rand_signature();
        let chain_id = rand_u256();
        let timestamp = rand_u256();
        let epoch_id = rand_u256();

        let handles_db: Vec<[u8; 32]> = handles.iter().map(|h| h.0).collect();
        let contract_addresses_db: Vec<[u8; 20]> = contract_addresses.iter().map(|a| a.0 .0).collect();

        sqlx::query(
            "INSERT INTO user_decryption_requests(\
                decryption_id, handles, contract_addresses, user_address, public_key, signature, chain_id, timestamp, epoch_id, otlp_context, already_sent, status\
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
        )
        .bind(request_id.as_le_slice())
        .bind(handles_db)
        .bind(contract_addresses_db)
        .bind(user_address.as_slice())
        .bind(&public_key)
        .bind(&user_signature)
        .bind(chain_id.as_le_slice())
        .bind(timestamp.as_le_slice())
        .bind(epoch_id.as_le_slice())
        .bind(bc2wrap::serialize(&PropagationContext::empty())?)
        .bind(false)
        .bind(OperationStatus::Pending as OperationStatus)
        .execute(test_instance.db())
        .await?;

        sqlx::query(
            "INSERT INTO user_decryption_responses(\
                decryption_id, user_decrypted_shares, signature, extra_data, otlp_context, status\
            ) VALUES ($1,$2,$3,$4,$5,$6)",
        )
        .bind(request_id.as_le_slice())
        .bind(vec![1u8, 2, 3])
        .bind(vec![4u8, 5, 6])
        .bind(Vec::<u8>::new())
        .bind(bc2wrap::serialize(&PropagationContext::empty())?)
        .bind(OperationStatus::Completed as OperationStatus)
        .execute(test_instance.db())
        .await?;

        let api_state = ApiState::new(test_instance.db().clone(), user_address, 3);
        let app = test::init_service(
            App::new()
                .app_data(Data::new(api_state))
                .route("/v1/share/{request_id}", actix_web::web::get().to(get_share_handler)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/v1/share/0x{}", hex::encode(request_id.to_be_bytes::<32>())))
            .to_request();
        let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

        assert_eq!(body.get("status").and_then(|v| v.as_str()), Some("ready"));
        assert_eq!(body.get("requestType").and_then(|v| v.as_str()), Some("user_decryption"));
        assert_eq!(body.get("shareIndex").and_then(|v| v.as_u64()), Some(3));
        assert_eq!(body.get("epochId").is_some(), true);

        Ok(())
    }

    #[actix_web::test]
    async fn test_get_share_pending() -> anyhow::Result<()> {
        let test_instance = TestInstanceBuilder::db_setup().await?;
        let request_id = rand_u256();
        let handles = rand_handles(1);
        let contract_addresses = rand_contract_addresses(1);
        let chain_id = rand_u256();
        let timestamp = rand_u256();

        let handles_db: Vec<[u8; 32]> = handles.iter().map(|h| h.0).collect();
        let contract_addresses_db: Vec<[u8; 20]> = contract_addresses.iter().map(|a| a.0 .0).collect();

        sqlx::query(
            "INSERT INTO public_decryption_requests(\
                decryption_id, handles, contract_addresses, chain_id, timestamp, otlp_context, already_sent, status\
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
        )
        .bind(request_id.as_le_slice())
        .bind(handles_db)
        .bind(contract_addresses_db)
        .bind(chain_id.as_le_slice())
        .bind(timestamp.as_le_slice())
        .bind(bc2wrap::serialize(&PropagationContext::empty())?)
        .bind(false)
        .bind(OperationStatus::Pending as OperationStatus)
        .execute(test_instance.db())
        .await?;

        let api_state = ApiState::new(test_instance.db().clone(), rand_address(), 0);
        let app = test::init_service(
            App::new()
                .app_data(Data::new(api_state))
                .route("/v1/share/{request_id}", actix_web::web::get().to(get_share_handler)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/v1/share/0x{}", hex::encode(request_id.to_be_bytes::<32>())))
            .to_request();
        let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

        assert_eq!(body.get("status").and_then(|v| v.as_str()), Some("pending"));
        assert_eq!(body.get("requestType").and_then(|v| v.as_str()), Some("public_decryption"));

        Ok(())
    }
}
