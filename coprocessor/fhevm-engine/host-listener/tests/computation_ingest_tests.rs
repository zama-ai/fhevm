use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::types::SupportedFheOperations as O;
use host_listener::database::computation::{Computation, Operand};
use host_listener::database::tfhe_event_propagate::{
    Database, Handle, LogTfhe,
};
use sqlx::Row;
use test_harness::instance::{setup_test_db, ImportMode};
use time::PrimitiveDateTime;

#[tokio::test]
async fn grouped_outputs_preserve_order_permissions_and_replay_counts(
) -> anyhow::Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let db =
        Database::new(&instance.db_url, ChainId::try_from(12345_u64)?, 100)
            .await?;
    let pool = db.pool.read().await.clone();
    let outputs = vec![Handle::repeat_byte(10), Handle::repeat_byte(11)];
    let computation = Computation {
        operation: O::FheAdd,
        operands: vec![
            Operand::Encrypted(Handle::repeat_byte(1)),
            Operand::Clear(vec![2; 32]),
        ],
        outputs: outputs.clone(),
    };
    let log = LogTfhe {
        operand_boundary_mask: Some(
            computation
                .boundary_mask(|_| false)
                .map_err(anyhow::Error::msg)?,
        ),
        computation,
        transaction_hash: Some(Handle::repeat_byte(20)),
        allowed_outputs: [outputs[1]].into_iter().collect(),
        block_number: 1,
        block_hash: Handle::repeat_byte(30),
        block_timestamp: PrimitiveDateTime::new(
            time::Date::from_calendar_date(2026, time::Month::September, 10)?,
            time::Time::MIDNIGHT,
        ),
        tx_depth_size: 0,
        dependence_chain: Handle::repeat_byte(20),
        log_index: Some(0),
        is_executor_minted: true,
    };
    let mut tx = pool.begin().await?;
    assert_eq!(db.insert_tfhe_event(&mut tx, &log).await?, 2);
    assert!(
        db.insert_tfhe_event(&mut tx, &log).await? == 0,
        "group replay must be idempotent"
    );
    sqlx::query("DELETE FROM computations WHERE output_handle = $1")
        .bind(outputs[1].to_vec())
        .execute(&mut *tx)
        .await?;
    assert_eq!(
        db.insert_tfhe_event(&mut tx, &log).await?,
        1,
        "partial replay counts only the missing row"
    );
    let rows = sqlx::query("SELECT output_handle, dependencies, is_scalar, group_id, output_index, output_count, is_allowed, operand_boundary_mask FROM computations ORDER BY output_index")
        .fetch_all(&mut *tx).await?;
    assert_eq!(rows.len(), 2);
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(
            row.get::<Vec<u8>, _>("output_handle"),
            outputs[index].to_vec()
        );
        assert_eq!(row.get::<Vec<u8>, _>("group_id"), outputs[0].to_vec());
        assert_eq!(row.get::<i16, _>("output_index"), index as i16);
        assert_eq!(row.get::<i16, _>("output_count"), 2);
        assert_eq!(row.get::<bool, _>("is_allowed"), index == 1);
        assert!(row.get::<bool, _>("is_scalar"));
        assert_eq!(
            row.get::<Vec<Vec<u8>>, _>("dependencies"),
            vec![vec![1; 32], vec![2; 32]]
        );
        assert_eq!(row.get::<Vec<u8>, _>("operand_boundary_mask")[31], 1);
    }
    tx.rollback().await?;
    Ok(())
}

#[cfg(feature = "solana-reconstruct")]
#[tokio::test]
async fn solana_records_reach_the_shared_sql_and_scheduler_path(
) -> anyhow::Result<()> {
    use host_listener::solana_adapter::{
        insert_solana_records, solana_transaction_id, SolanaBlockMeta,
        SolanaHostRecord,
    };
    use zama_host::{
        records::{FheMulDiv, TrivialEncrypt},
        EVENT_VERSION,
    };
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let db =
        Database::new(&instance.db_url, ChainId::try_from(12345_u64)?, 100)
            .await?;
    let pool = db.pool.read().await.clone();
    let transaction_id = solana_transaction_id(&[7; 64]);
    let block = SolanaBlockMeta {
        block_number: 4,
        block_timestamp: PrimitiveDateTime::new(
            time::Date::from_calendar_date(2026, time::Month::September, 10)?,
            time::Time::MIDNIGHT,
        ),
        block_hash: [5; 32],
        parent_hash: [4; 32],
    };
    let mut tx = pool.begin().await?;
    let stats = insert_solana_records(
        &db,
        &mut tx,
        [
            SolanaHostRecord::TrivialEncrypt(TrivialEncrypt {
                version: EVENT_VERSION,
                plaintext: [8; 32],
                fhe_type: 5,
                result: [1; 32],
            }),
            SolanaHostRecord::FheMulDiv(FheMulDiv {
                version: EVENT_VERSION,
                factor1: [1; 32],
                factor2: [2; 32],
                divisor: [3; 32],
                scalar: false,
                result: [4; 32],
            }),
        ],
        transaction_id,
        block,
    )
    .await?;
    assert_eq!(stats.inserted_rows, 2);
    let row = sqlx::query("SELECT dependencies, is_scalar, operand_boundary_mask, transaction_id, is_allowed FROM computations WHERE output_handle = $1")
        .bind(vec![4_u8; 32]).fetch_one(&mut *tx).await?;
    assert_eq!(
        row.get::<Vec<Vec<u8>>, _>("dependencies"),
        vec![vec![1; 32], vec![2; 32], vec![3; 32]]
    );
    assert!(
        !row.get::<bool, _>("is_scalar"),
        "MulDiv's clear divisor does not set factor2's flag"
    );
    assert_eq!(row.get::<Vec<u8>, _>("operand_boundary_mask")[31], 2);
    assert_eq!(
        row.get::<Vec<u8>, _>("transaction_id"),
        transaction_id.to_vec()
    );
    assert!(row.get::<bool, _>("is_allowed"));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM dependence_chain")
            .fetch_one(&mut *tx)
            .await?,
        1
    );
    tx.commit().await?;
    Ok(())
}
