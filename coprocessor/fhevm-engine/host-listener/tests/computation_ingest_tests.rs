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
    let computation = Computation::new(
        O::FheAdd,
        vec![
            Operand::Encrypted(Handle::repeat_byte(1)),
            Operand::Clear(vec![2; 32]),
        ],
        outputs.clone(),
    )
    .map_err(anyhow::Error::msg)?;
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
        insert_solana_block_records, solana_transaction_id, SolanaBlockMeta,
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
    let stats = insert_solana_block_records(
        &db,
        &mut tx,
        [(
            transaction_id,
            vec![
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
        )],
        block,
        0,
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

#[cfg(feature = "solana-reconstruct")]
#[tokio::test]
async fn solana_block_priority_preserves_transaction_origins_replay_and_slow_parents(
) -> anyhow::Result<()> {
    use fhevm_engine_common::types::SchedulePriority;
    use host_listener::solana_adapter::{
        insert_solana_block_records, solana_transaction_id, SolanaBlockMeta,
        SolanaHostRecord as R, SolanaMaterialRequest,
    };
    use zama_host::{
        records::{FheBinaryOp, TrivialEncrypt},
        FheBinaryOpCode, EVENT_VERSION,
    };
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let db =
        Database::new(&instance.db_url, ChainId::try_from(12345_u64)?, 100)
            .await?;
    let pool = db.pool.read().await.clone();
    let block = SolanaBlockMeta {
        block_number: 10,
        block_timestamp: PrimitiveDateTime::new(
            time::Date::from_calendar_date(2026, time::Month::September, 10)?,
            time::Time::MIDNIGHT,
        ),
        block_hash: [10; 32],
        parent_hash: [9; 32],
    };
    let trivial = |value| {
        R::TrivialEncrypt(TrivialEncrypt {
            version: EVENT_VERSION,
            plaintext: [value; 32],
            fhe_type: 5,
            result: [value; 32],
        })
    };
    let add = |input, output| {
        R::FheBinaryOp(FheBinaryOp {
            version: EVENT_VERSION,
            op: FheBinaryOpCode::Add,
            lhs: [input; 32],
            rhs: [0; 32],
            scalar: true,
            result: [output; 32],
        })
    };
    let material = |value| {
        R::MaterialRequest(SolanaMaterialRequest {
            handle: Handle::repeat_byte(value),
        })
    };
    let ids = [1, 2, 3].map(|n| solana_transaction_id(&[n; 64]));
    let records = vec![
        (ids[0], vec![trivial(1), material(1), material(1)]),
        (ids[1], vec![add(1, 2)]),
        (ids[2], vec![trivial(3), material(3)]),
    ];
    let mut tx = pool.begin().await?;
    let stats =
        insert_solana_block_records(&db, &mut tx, records.clone(), block, 1)
            .await?;
    assert_eq!(stats.tfhe_events, 3);
    assert_eq!(stats.material_requests, 2);
    assert_eq!(stats.inserted_rows, 5);
    let rows = sqlx::query("SELECT c.output_handle, c.dependence_chain_id, c.schedule_order, c.operand_boundary_mask, d.schedule_priority FROM computations c JOIN dependence_chain d ON c.dependence_chain_id = d.dependence_chain_id ORDER BY c.output_handle")
        .fetch_all(&mut *tx).await?;
    assert_eq!(rows.len(), 3);
    let parent = rows[0].get::<Vec<u8>, _>("dependence_chain_id");
    assert_eq!(rows[1].get::<Vec<u8>, _>("dependence_chain_id"), parent);
    for row in &rows[..2] {
        assert_eq!(
            row.get::<i16, _>("schedule_priority"),
            i16::from(SchedulePriority::Slow)
        );
    }
    assert_eq!(
        rows[2].get::<i16, _>("schedule_priority"),
        i16::from(SchedulePriority::Fast),
        "PBS preparation is not a dependent computation"
    );
    assert_eq!(
        (rows[1].get::<PrimitiveDateTime, _>("schedule_order")
            - block.block_timestamp)
            .whole_microseconds(),
        1
    );
    assert_eq!(
        rows[1].get::<Vec<u8>, _>("operand_boundary_mask")[31],
        1,
        "a previous transaction's result is a boundary input"
    );
    for (value, id) in [(1u8, ids[0]), (3u8, ids[2])] {
        let stored: Vec<u8> = sqlx::query_scalar(
            "SELECT transaction_id FROM pbs_computations WHERE handle = $1",
        )
        .bind(vec![value; 32])
        .fetch_one(&mut *tx)
        .await?;
        assert_eq!(stored, id.to_vec());
    }
    sqlx::query("UPDATE dependence_chain SET status = 'processed'")
        .execute(&mut *tx)
        .await?;
    let replay =
        insert_solana_block_records(&db, &mut tx, records, block, 1).await?;
    assert_eq!(replay.inserted_rows, 0);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM dependence_chain WHERE status != 'processed'"
        )
        .fetch_one(&mut *tx)
        .await?,
        0
    );
    tx.commit().await?;

    let mut tx = pool.begin().await?;
    let next = SolanaBlockMeta {
        block_number: 11,
        block_hash: [11; 32],
        parent_hash: block.block_hash,
        ..block
    };
    insert_solana_block_records(
        &db,
        &mut tx,
        [
            (solana_transaction_id(&[4; 64]), vec![add(2, 4)]),
            (solana_transaction_id(&[5; 64]), vec![add(2, 5)]),
        ],
        next,
        10,
    )
    .await?;
    let children = sqlx::query("SELECT c.dependence_chain_id, d.schedule_priority FROM computations c JOIN dependence_chain d ON c.dependence_chain_id = d.dependence_chain_id WHERE c.output_handle = ANY($1)")
        .bind(vec![vec![4u8; 32], vec![5u8; 32]]).fetch_all(&mut *tx).await?;
    assert_eq!(children.len(), 2);
    for child in children {
        assert_ne!(
            child.get::<Vec<u8>, _>("dependence_chain_id"),
            parent,
            "forked consumers must split from the parent"
        );
        assert_eq!(
            child.get::<i16, _>("schedule_priority"),
            i16::from(SchedulePriority::Slow),
            "a small child inherits its parent's slow priority"
        );
    }
    tx.commit().await?;
    // The CLI invokes this on startup when the cap is zero; upsert's GREATEST alone cannot reset Slow.
    assert!(db.promote_all_dep_chains_to_fast_priority().await? > 0);
    assert_eq!(sqlx::query_scalar::<_, i64>("SELECT count(*) FROM dependence_chain WHERE schedule_priority != 0").fetch_one(&pool).await?, 0);
    Ok(())
}

#[tokio::test]
async fn disabling_slow_lane_waits_for_locked_rows() -> anyhow::Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let db =
        Database::new(&instance.db_url, ChainId::try_from(12345_u64)?, 100)
            .await?;
    let pool = db.pool.read().await.clone();
    let statement_timeout: String =
        sqlx::query_scalar("SHOW statement_timeout")
            .fetch_one(&pool)
            .await?;
    assert_eq!(
        statement_timeout, "2min",
        "the reset's row-lock waits are bounded"
    );
    sqlx::query("INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, schedule_priority) VALUES ($1, 'updated', NOW(), NOW(), 1, 1)")
        .bind(vec![1u8; 32]).execute(&pool).await?;
    let mut lock = pool.begin().await?;
    sqlx::query("SELECT dependence_chain_id FROM dependence_chain FOR UPDATE")
        .fetch_all(&mut *lock)
        .await?;
    let promotion = db.promote_all_dep_chains_to_fast_priority();
    tokio::pin!(promotion);
    // Poll the real reset while the last Slow row is locked. The old SKIP LOCKED query
    // returned Ok(0) here and permanently left this row Slow.
    assert!(tokio::time::timeout(
        std::time::Duration::from_millis(100),
        &mut promotion
    )
    .await
    .is_err());
    lock.commit().await?;
    assert_eq!(
        tokio::time::timeout(std::time::Duration::from_secs(5), promotion)
            .await??,
        1
    );
    let priority: i16 =
        sqlx::query_scalar("SELECT schedule_priority FROM dependence_chain")
            .fetch_one(&pool)
            .await?;
    assert_eq!(priority, 0);
    Ok(())
}
