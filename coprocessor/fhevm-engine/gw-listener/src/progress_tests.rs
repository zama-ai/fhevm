use super::*;
use serial_test::serial;
use test_harness::instance::ImportMode;

#[test]
fn drift_sender_seed_does_not_pass_a_rewound_cursor() {
    let mut progress = ListenerProgress {
        last_processed_block_num: Some(99),
        earliest_open_ct_commits_block: Some(200),
        ..Default::default()
    };
    assert_eq!(progress.sender_seed_block(), Some(99));
    progress.last_processed_block_num = Some(250);
    assert_eq!(progress.sender_seed_block(), Some(199));
    progress.earliest_open_ct_commits_block = Some(0);
    assert_eq!(progress.sender_seed_block(), Some(0));
    assert_eq!(ListenerProgress::default().sender_seed_block(), None);
}

#[tokio::test]
#[serial(db)]
async fn migration_preserves_existing_green_progress() -> anyhow::Result<()> {
    let db = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db.db_url.as_str())
        .await?;
    sqlx::query("ALTER TABLE public.gw_listener_last_block DROP COLUMN progress_revision")
        .execute(&pool)
        .await?;
    let green = green_pool(&pool, db.db_url.as_str()).await?;
    sqlx::query("INSERT INTO gw_listener_last_block (last_block_num) VALUES (42)")
        .execute(&green)
        .await?;
    sqlx::raw_sql(include_str!(
        "../../db-migration/migrations/20261012160000_gateway_progress_revision.sql"
    ))
    .execute(&pool)
    .await?;
    let (height, revision): (i64, sqlx::types::Uuid) = sqlx::query_as(
        "SELECT last_block_num, progress_revision FROM gw_listener_last_block WHERE dummy_id",
    )
    .fetch_one(&green)
    .await?;
    assert_eq!(height, 42);
    assert!(!revision.is_nil());
    sqlx::query("INSERT INTO public.gw_listener_last_block (last_block_num) VALUES (43)")
        .execute(&pool)
        .await?;
    let blue_revision: sqlx::types::Uuid = sqlx::query_scalar(
        "SELECT progress_revision FROM public.gw_listener_last_block WHERE dummy_id",
    )
    .fetch_one(&pool)
    .await?;
    assert_ne!(blue_revision, revision);
    Ok(())
}

fn headers(from: i64, to: i64) -> Vec<ListenerHeader> {
    (from..=to)
        .map(|number| ListenerHeader {
            number,
            hash: vec![number as u8; 32],
            parent_hash: vec![number.saturating_sub(1) as u8; 32],
        })
        .collect()
}

#[tokio::test]
#[serial(db)]
async fn replica_completion_and_pruning_are_serialized() -> anyhow::Result<()> {
    let db = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(db.db_url.as_str())
        .await?;
    let conf = ConfigSettings::default();
    let initial = get_listener_progress(&pool, false, 42).await?;
    chain_recovery::save_checkpoints(&pool, ListenerSource::Gateway, false, 42, &headers(1, 400))
        .await?;
    let mut count = 0;
    let advanced = update_listener_progress(
        &pool,
        &conf,
        false,
        42,
        ListenerProgress {
            last_processed_block_num: Some(200),
            ..initial
        },
        &mut count,
    )
    .await?;
    // Completed history can be pruned; older scans cannot republish it.
    // Admitted blocks beyond durable progress must remain pinned.
    assert!(
        chain_recovery::checkpoint_hash(&pool, ListenerSource::Gateway, false, 42, 100)
            .await?
            .is_none()
    );
    assert!(
        chain_recovery::checkpoint_hash(&pool, ListenerSource::Gateway, false, 42, 400)
            .await?
            .is_some()
    );
    assert!(update_listener_progress(
        &pool,
        &conf,
        false,
        42,
        ListenerProgress {
            last_processed_block_num: Some(100),
            ..initial
        },
        &mut count
    )
    .await
    .is_err());
    assert_eq!(get_listener_progress(&pool, false, 42).await?, advanced);
    let mut left_count = 0;
    let mut right_count = 0;
    let (left, right) = tokio::join!(
        update_listener_progress(
            &pool,
            &conf,
            false,
            42,
            ListenerProgress {
                last_processed_block_num: Some(300),
                ..advanced
            },
            &mut left_count
        ),
        update_listener_progress(
            &pool,
            &conf,
            false,
            42,
            ListenerProgress {
                last_processed_block_num: Some(400),
                ..advanced
            },
            &mut right_count
        ),
    );
    assert_ne!(left.is_ok(), right.is_ok());
    assert_eq!(left_count + right_count, 1);
    let saved = get_listener_progress(&pool, false, 42).await?;
    assert_eq!(saved, left.or(right)?);
    // Missing admission must roll back both progress and its revision.
    assert!(update_listener_progress(
        &pool,
        &conf,
        false,
        42,
        ListenerProgress {
            last_processed_block_num: Some(500),
            ..saved
        },
        &mut count
    )
    .await
    .is_err());
    assert_eq!(get_listener_progress(&pool, false, 42).await?, saved);
    assert!(!chain_recovery::is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn empty_cursor_recreation_invalidates_old_scan() -> anyhow::Result<()> {
    let db = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db.db_url.as_str())
        .await?;
    let old = get_listener_progress(&pool, false, 42).await?;
    sqlx::query("TRUNCATE gw_listener_last_block")
        .execute(&pool)
        .await?;
    let fresh = get_listener_progress(&pool, false, 42).await?;
    assert_ne!(old.revision, fresh.revision);
    chain_recovery::save_checkpoints(&pool, ListenerSource::Gateway, false, 42, &headers(1, 1))
        .await?;
    let mut count = 0;
    let conf = ConfigSettings::default();
    assert!(update_listener_progress(
        &pool,
        &conf,
        false,
        42,
        ListenerProgress {
            last_processed_block_num: Some(1),
            ..old
        },
        &mut count
    )
    .await
    .is_err());
    assert_eq!(get_listener_progress(&pool, false, 42).await?, fresh);
    update_listener_progress(
        &pool,
        &conf,
        false,
        42,
        ListenerProgress {
            last_processed_block_num: Some(1),
            ..fresh
        },
        &mut count,
    )
    .await?;
    assert!(!chain_recovery::is_halted(&pool).await?);
    Ok(())
}

async fn green_pool(pool: &Pool<Postgres>, url: &str) -> anyhow::Result<Pool<Postgres>> {
    sqlx::query(&format!("CREATE SCHEMA {GCS_SCHEMA_QUOTED}"))
        .execute(pool)
        .await?;
    sqlx::query(&format!("CREATE TABLE {GCS_SCHEMA_QUOTED}.gw_listener_last_block (LIKE public.gw_listener_last_block INCLUDING ALL)"))
        .execute(pool).await?;
    let options = url.parse::<sqlx::postgres::PgConnectOptions>()?.options([(
        "search_path",
        fhevm_engine_common::database::GCS_SEARCH_PATH,
    )]);
    Ok(PgPoolOptions::new()
        .max_connections(4)
        .connect_with(options)
        .await?)
}

#[tokio::test]
#[serial(db)]
async fn rewind_fences_delayed_completion_and_repeated_height() -> anyhow::Result<()> {
    let db = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(db.db_url.as_str())
        .await?;
    let green = green_pool(&pool, db.db_url.as_str()).await?;
    let initial = get_listener_progress(&green, true, 42).await?;
    chain_recovery::save_checkpoints(&green, ListenerSource::Gateway, true, 42, &headers(1, 250))
        .await?;
    let conf = ConfigSettings::default();
    let mut count = 0;
    let before = update_listener_progress(
        &green,
        &conf,
        true,
        42,
        ListenerProgress {
            last_processed_block_num: Some(200),
            ..initial
        },
        &mut count,
    )
    .await?;
    sqlx::query("INSERT INTO upgrade_state (stack_role, host_chain_id, state, status, gw_start_block) VALUES ('GCS', 1, 'UpgradeActivated', 'in_progress', 100)")
        .execute(&pool).await?;
    let (left, right) = tokio::join!(
        align_cursor_to_active_window(&green, Some(200), 250, None, 42, test_rewind_header),
        align_cursor_to_active_window(&green, Some(200), 250, None, 42, test_rewind_header),
    );
    let outcomes = [left?, right?];
    assert_eq!(outcomes.iter().filter(|c| **c == Some(99)).count(), 1);
    assert_eq!(outcomes.iter().filter(|c| c.is_none()).count(), 1);
    let rewound = get_listener_progress(&green, true, 42).await?;
    assert_eq!(rewound.last_processed_block_num, Some(99));
    let delayed = ListenerProgress {
        last_processed_block_num: Some(250),
        ..before
    };
    assert!(
        update_listener_progress(&green, &conf, true, 42, delayed, &mut count)
            .await
            .is_err()
    );
    assert_eq!(get_listener_progress(&green, true, 42).await?, rewound);
    chain_recovery::save_checkpoints(
        &green,
        ListenerSource::Gateway,
        true,
        42,
        &headers(100, 200),
    )
    .await?;
    let replayed = update_listener_progress(
        &green,
        &conf,
        true,
        42,
        ListenerProgress {
            last_processed_block_num: Some(200),
            ..rewound
        },
        &mut count,
    )
    .await?;
    assert_ne!(replayed.revision, before.revision);
    assert!(
        update_listener_progress(&green, &conf, true, 42, delayed, &mut count)
            .await
            .is_err()
    );
    assert_eq!(get_listener_progress(&green, true, 42).await?, replayed);
    assert!(!chain_recovery::is_halted(&pool).await?);
    let blue_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM public.gw_listener_last_block")
        .fetch_one(&pool)
        .await?;
    assert_eq!(blue_rows, 0);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn checkpoint_routing_survives_queued_cutover() -> anyhow::Result<()> {
    use fhevm_engine_common::versioning::CUTOVER_LOCK_ID;
    let db = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(db.db_url.as_str())
        .await?;
    let green = green_pool(&pool, db.db_url.as_str()).await?;
    let initial = get_listener_progress(&green, true, 42).await?;
    chain_recovery::save_checkpoints(&green, ListenerSource::Gateway, true, 42, &headers(1, 2))
        .await?;
    let conf = ConfigSettings::default();
    let mut count = 0;
    let before = update_listener_progress(
        &green,
        &conf,
        true,
        42,
        ListenerProgress {
            last_processed_block_num: Some(2),
            ..initial
        },
        &mut count,
    )
    .await?;
    let mut cutover = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *cutover)
        .await?;
    let queued_pool = green.clone();
    let admission = tokio::spawn(async move {
        chain_recovery::save_checkpoints(
            &queued_pool,
            ListenerSource::Gateway,
            true,
            42,
            &headers(3, 3),
        )
        .await
    });
    // Wait for a real lock wait, not a scheduler-dependent sleep.
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks WHERE locktype = 'advisory' AND NOT granted AND classid = $1::bigint::oid AND objid = $2::bigint::oid)")
                .bind(CUTOVER_LOCK_ID >> 32).bind(CUTOVER_LOCK_ID & 0xffff_ffff)
                .fetch_one(&pool).await?;
            if waiting { break; }
            tokio::task::yield_now().await;
        }
        Ok::<_, sqlx::Error>(())
    }).await??;
    // Exercise the schema transition with the listener's copied mode still
    // true. Controller integration separately covers the full promotion.
    sqlx::query(&format!("INSERT INTO public.gw_listener_last_block SELECT * FROM {GCS_SCHEMA_QUOTED}.gw_listener_last_block"))
        .execute(&mut *cutover).await?;
    sqlx::query("INSERT INTO public.listener_block_checkpoints SELECT source, 'BCS', chain_id, block_number, block_hash, parent_hash FROM public.listener_block_checkpoints WHERE stack_role = 'GCS'")
        .execute(&mut *cutover).await?;
    sqlx::query(&format!(
        "ALTER SCHEMA {GCS_SCHEMA_QUOTED} RENAME TO \"pre-cutover\""
    ))
    .execute(&mut *cutover)
    .await?;
    cutover.commit().await?;
    admission.await??;
    assert!(
        chain_recovery::checkpoint_hash(&pool, ListenerSource::Gateway, false, 42, 3)
            .await?
            .is_some()
    );
    assert!(
        chain_recovery::checkpoint_hash(&pool, ListenerSource::Gateway, true, 42, 3)
            .await?
            .is_none()
    );
    let live = update_listener_progress(
        &green,
        &conf,
        true,
        42,
        ListenerProgress {
            last_processed_block_num: Some(3),
            ..before
        },
        &mut count,
    )
    .await?;
    assert_eq!(get_listener_progress(&pool, false, 42).await?, live);
    assert!(!chain_recovery::is_halted(&pool).await?);
    Ok(())
}
