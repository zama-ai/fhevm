//! Worker-backed semantic backend for Solana scenario tests (Postgres + tfhe-worker).

use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheCiphertexts};
use host_listener::{
    database::tfhe_event_propagate::Handle,
    solana_adapter::{insert_solana_events, solana_transaction_id, SolanaBlockMeta},
};
use tfhe::prelude::FheTryEncrypt;
use time::{Date, Month, PrimitiveDateTime, Time};
use zama_solana_tests::{collect_zama_host_events, TransferScenario, BALANCE_FHE_TYPE};

use crate::tests::{
    event_helpers::{decrypt_handles, setup_event_harness, wait_until_computed, EventHarness},
    utils::latest_db_key,
};

pub struct WorkerSemanticBackend {
    pub harness: EventHarness,
    block_number: u64,
}

impl WorkerSemanticBackend {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            harness: setup_event_harness().await?,
            block_number: 1,
        })
    }

    pub async fn seed_u64(
        &self,
        handle: [u8; 32],
        value: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        seed_balance_ciphertext(&self.harness.pool, handle, value).await?;
        Ok(())
    }

    pub async fn ingest_transfer(
        &mut self,
        scenario: &TransferScenario,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let events = collect_zama_host_events(
            &scenario.meta,
            &scenario.account_keys,
            scenario.host_program_id,
        );
        let transaction_id = solana_transaction_id(scenario.signature.as_ref());
        let block = SolanaBlockMeta {
            block_number: self.block_number,
            block_timestamp: PrimitiveDateTime::new(
                Date::from_calendar_date(2026, Month::May, 11)?,
                Time::MIDNIGHT,
            ),
        };
        let mut db_tx = self.harness.listener_db.new_transaction().await?;
        insert_solana_events(
            &self.harness.listener_db,
            &mut db_tx,
            events,
            transaction_id,
            block,
        )
        .await?;
        db_tx.commit().await?;
        self.block_number += 1;
        Ok(())
    }

    pub async fn wait_compute(&self) -> Result<(), Box<dyn std::error::Error>> {
        wait_until_computed(&self.harness.app).await?;
        Ok(())
    }

    pub async fn decrypt_u64(&self, handle: [u8; 32]) -> Result<u64, Box<dyn std::error::Error>> {
        let decrypted = decrypt_handles(&self.harness.pool, &[Handle::from(handle)]).await?;
        let Some(result) = decrypted.first() else {
            return Err("expected one decryption result for handle".into());
        };
        Ok(result.value.parse::<u64>()?)
    }
}

pub async fn assert_transfer_worker(
    scenario: &TransferScenario,
    alice_initial: u64,
    bob_initial: u64,
    amount: u64,
    expect: zama_solana_tests::TransferExpect,
) -> Result<(), Box<dyn std::error::Error>> {
    use zama_solana_tests::assert_transfer_semantics;

    let mut backend = WorkerSemanticBackend::new().await?;
    backend
        .seed_u64(scenario.alice_before, alice_initial)
        .await?;
    backend.seed_u64(scenario.bob_before, bob_initial).await?;
    backend.seed_u64(scenario.amount_handle, amount).await?;
    backend.ingest_transfer(scenario).await?;
    backend.wait_compute().await?;

    let alice = backend.decrypt_u64(scenario.new_alice_handle).await?;
    let bob = backend.decrypt_u64(scenario.new_bob_handle).await?;
    assert_transfer_semantics(alice, bob, expect);
    Ok(())
}

async fn seed_balance_ciphertext(
    pool: &sqlx::PgPool,
    handle: [u8; 32],
    value: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let (key, _) = latest_db_key(pool).await;
    let (handle, ty, ciphertext) = tokio::task::spawn_blocking(move || -> Result<_, String> {
        let client_key = key.cks.expect("test key must include a client key");
        #[cfg(not(feature = "gpu"))]
        tfhe::set_server_key(key.sks);
        #[cfg(feature = "gpu")]
        tfhe::set_server_key(key.csks.decompress());

        let ciphertext =
            tfhe::FheUint64::try_encrypt(value, &client_key).map_err(|e| e.to_string())?;
        let supported = SupportedFheCiphertexts::FheUint64(ciphertext);
        let ty = supported.type_num();
        assert_eq!(ty, BALANCE_FHE_TYPE as i16);
        let compressed = supported.compress().map_err(|e| e.to_string())?;
        Ok((handle, ty, compressed))
    })
    .await?
    .map_err(std::io::Error::other)?;

    sqlx::query(
        r#"
            INSERT INTO ciphertexts(handle, ciphertext, ciphertext_version, ciphertext_type)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (handle, ciphertext_version) DO UPDATE
            SET ciphertext = EXCLUDED.ciphertext,
                ciphertext_type = EXCLUDED.ciphertext_type
        "#,
    )
    .bind(handle.to_vec())
    .bind(ciphertext)
    .bind(current_ciphertext_version())
    .bind(ty)
    .execute(pool)
    .await?;

    Ok(())
}
