use fhevm_engine_common::chain_id::ChainId;
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use test_harness::instance::ImportMode;

#[tokio::test]
#[serial(db)]
async fn catchup_cursor_survives_restart_and_preserves_overlapping_gaps(
) -> anyhow::Result<()> {
    let instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    let chain_id = ChainId::try_from(42_u64)?;
    let db = Database::new(&instance.db_url, chain_id, 128).await?;
    let first = db.initialize_catchup_cursor("legacy", 40).await?;
    assert_eq!(first.next_block_number, 40);

    // Realtime may be hundreds of blocks ahead when this process dies before
    // finishing its historical page. A new process must retain the old gap.
    let restarted = Database::new(&instance.db_url, chain_id, 128).await?;
    assert_eq!(
        restarted.initialize_catchup_cursor("legacy", 200).await?,
        first
    );
    let advanced = restarted
        .finish_catchup_page("legacy", 40, 80, first.generation)
        .await?;
    assert_eq!(advanced.next_block_number, 80);
    assert_eq!(
        db.finish_catchup_page("legacy", 40, 50, first.generation)
            .await?,
        advanced
    );

    // A peer discovers a new gap inside an older in-flight page (40..59).
    // The old page must not erase the new gap by advancing 50 to 60.
    let rewind = db.initialize_catchup_cursor("legacy", 50).await?;
    assert_eq!(rewind.next_block_number, 50);
    assert_ne!(rewind.generation, first.generation);
    assert_eq!(
        restarted
            .finish_catchup_page("legacy", 40, 60, first.generation)
            .await?,
        rewind
    );
    let repaired = db
        .finish_catchup_page("legacy", 50, 60, rewind.generation)
        .await?;
    assert_eq!(repaired.next_block_number, 60);

    // An earlier rewind also cannot be skipped by a later page.
    let earlier = db.initialize_catchup_cursor("legacy", 20).await?;
    assert_eq!(
        restarted
            .finish_catchup_page("legacy", 60, 80, earlier.generation)
            .await?,
        earlier
    );
    let repaired = db
        .finish_catchup_page("legacy", 20, 80, earlier.generation)
        .await?;
    assert_eq!(repaired.next_block_number, 80);

    // Independent replay configurations and chains cannot move this cursor.
    assert_eq!(
        db.initialize_catchup_cursor("manual", 500)
            .await?
            .next_block_number,
        500
    );
    let other_chain =
        Database::new(&instance.db_url, ChainId::try_from(43_u64)?, 128)
            .await?;
    assert_eq!(
        other_chain
            .initialize_catchup_cursor("legacy", 500)
            .await?
            .next_block_number,
        500
    );
    assert_eq!(
        db.initialize_catchup_cursor("legacy", 900)
            .await?
            .next_block_number,
        80
    );
    Ok(())
}
