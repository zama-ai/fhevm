use std::sync::Arc;

use alloy::{providers::ProviderBuilder, signers::local::PrivateKeySigner, sol};
use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;
use transaction_sender::{ConfigSettings, TransactionSender};

sol!(
    #[sol(rpc)]
    ZKPoKManager,
    "artifacts/ZKPoKManager.sol/ZKPoKManager.json"
);

sol!(
    #[sol(rpc)]
    CiphertextStorage,
    "artifacts/CiphertextStorage.sol/CiphertextStorage.json"
);

#[tokio::test]
async fn verify_proof_response() -> anyhow::Result<()> {
    let signer = PrivateKeySigner::random();
    let provider = Arc::new(ProviderBuilder::new().on_anvil_with_wallet());
    let zkpok_manager = ZKPoKManager::deploy(&provider).await?;
    let ciphertext_storage = CiphertextStorage::deploy(&provider).await?;
    let cancel_token = CancellationToken::new();
    let conf = ConfigSettings::default();
    let txn_sender = TransactionSender::new(
        &zkpok_manager.address(),
        &ciphertext_storage.address(),
        signer.clone(),
        provider.clone(),
        cancel_token.clone(),
        conf.clone(),
    );

    let event_filter = zkpok_manager
        .VerifyProofResponseCalled_filter()
        .watch()
        .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&conf.db_url)
        .await?;

    sqlx::query!(
        "INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles)
                VALUES ($1, $2, $3, $4, $5)",
        1,
        42,
        signer.address().to_string(),
        signer.address().to_string(),
        &[1u8; 64],
    )
    .execute(&db_pool)
    .await?;

    let _event = event_filter
        .into_stream()
        .take(1)
        .collect::<Vec<_>>()
        .await
        .first()
        .unwrap()
        .clone()
        .unwrap();

    cancel_token.cancel();
    run_handle.await??;
    Ok(())
}
