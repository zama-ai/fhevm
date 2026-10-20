use alloy::primitives::Address;
use consumer::{BlockPayload, Broker};
use fhevm_engine_common::chain_id::ChainId;
use host_listener::consumer::{
    catchup::ManualCatchupArgs, run_consumer, ConsumerConfig,
};
use primitives::event::BlockFlow;
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis)"]
async fn startup_no_flags_without_previous_requests() {
    exercise_startup(Startup::NoFlags, false).await;
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis)"]
async fn startup_no_flags_with_unowned_requests() {
    exercise_startup(Startup::NoFlags, true).await;
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis)"]
async fn startup_new_catchup_without_previous_requests() {
    exercise_startup(Startup::NewCatchup, false).await;
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis)"]
async fn startup_new_catchup_with_unowned_requests() {
    exercise_startup(Startup::NewCatchup, true).await;
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis)"]
async fn startup_same_catchup_without_other_requests() {
    exercise_startup(Startup::SameCatchup, false).await;
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis)"]
async fn startup_same_catchup_with_unowned_requests() {
    exercise_startup(Startup::SameCatchup, true).await;
}

#[derive(Clone, Copy)]
enum Startup {
    NoFlags,
    NewCatchup,
    SameCatchup,
}

async fn exercise_startup(startup: Startup, previous_requests: bool) {
    use broker::{AsyncHandlerPayloadClassified, Topic};
    use host_listener::consumer::catchup::store;
    use primitives::{
        event::CatchupPayload, routing, utils::chain_id_to_namespace,
    };
    use test_harness::instance::{setup_test_db, ImportMode};

    tokio::time::timeout(std::time::Duration::from_secs(60), async {
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = sqlx::PgPool::connect(instance.db_url.as_str()).await.unwrap();
        let chain_id = ChainId::try_from(
            (uuid::Uuid::new_v4().as_u128() % 900_000_000) as u64 + 100_000_000,
        ).unwrap();
        let identity = format!("host-listener-consumer.{}", chain_id.as_u64());
        let manual = match startup {
            Startup::NoFlags => ManualCatchupArgs::default(),
            Startup::NewCatchup | Startup::SameCatchup => ManualCatchupArgs {
                catchup_from_block: Some(-100),
                catchup_up_to_block: None,
                catchup_unfinalized: false,
                no_override_catchup: true,
            },
        };
        let mut expected = Vec::new();
        if previous_requests {
            for (start, end) in [(300, 310), (400, 410)] {
                let args = ManualCatchupArgs {
                    catchup_from_block: Some(start),
                    catchup_up_to_block: Some(end),
                    catchup_unfinalized: false,
                    no_override_catchup: true,
                };
                let id = store::ensure_requested(
                    &pool, chain_id.as_i64(), &identity, &args, Some(1000), None, false,
                ).await.unwrap().unwrap();
                expected.push((id, start as u64, end as u64));
            }
        }
        if matches!(startup, Startup::SameCatchup) {
            let id = store::ensure_requested(
                &pool, chain_id.as_i64(), &identity, &manual, Some(1000), None, false,
            ).await.unwrap().unwrap();
            expected.push((id, 900, 999));
        }
        // These requests were already published before their listeners died.
        sqlx::query("UPDATE host_consumer_catchups SET request_pending = false WHERE chain_id = $1 AND consumer_id = $2")
            .bind(chain_id.as_i64()).bind(&identity).execute(&pool).await.unwrap();

        let broker_url = std::env::var("CATCHUP_BROKER_URL").unwrap();
        let broker = Broker::from_url(&broker_url).await.unwrap();
        let namespace = chain_id_to_namespace(chain_id.as_u64());
        let stop = CancellationToken::new();
        let (request_tx, mut requests) = mpsc::unbounded_channel();
        let control = broker
            .consumer(&Topic::namespaced(&namespace, routing::FINAL_CATCHUP))
            .group("startup-requests")
            .with_cancellation(stop.clone())
            .build().unwrap();
        control.ensure_topology().await.unwrap();
        let control = tokio::spawn(control.run(AsyncHandlerPayloadClassified::new(
            move |request: CatchupPayload| {
                let tx = request_tx.clone();
                async move {
                    tx.send(request).unwrap();
                    Ok(())
                }
            },
        )));
        let run = tokio::spawn(run_consumer(test_config(
            broker_url, instance.db_url.clone(), chain_id, manual,
        )));
        let publisher = broker.publisher("").await.unwrap();
        let observer = consumer::ListenerConsumer::new(&broker, chain_id.as_u64(), &identity);
        loop {
            let status = observer.group_status().await.unwrap();
            if [".new-event", ".final-catchup-event"].iter().all(|suffix| {
                status.iter().any(|(topic, groups)| topic.ends_with(suffix) && !groups.is_empty())
            }) {
                break;
            }
            assert!(!run.is_finished(), "listener stopped during startup");
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        let mut blocks = Vec::new();
        let mut pending_requests = Vec::new();
        if matches!(startup, Startup::NewCatchup) && previous_requests {
            let request = requests.recv().await.unwrap();
            assert!(expected.iter().any(|(id, _, _)| Some(*id) == request.catchup_id),
                "retained requests must resume while the new range waits for a live block");
            pending_requests.push(request);
        }
        if matches!(startup, Startup::NewCatchup)
            || (matches!(startup, Startup::NoFlags) && !previous_requests)
        {
            publisher.publish(
                &routing::consumer_new_event_routing(identity.clone()),
                &event_block(chain_id.as_u64(), 2000, BlockFlow::Live, None),
            ).await.unwrap();
            blocks.push(2000);
        }
        // Resuming existing requests needs no new live reference. A new request
        // resolves relative bounds against this startup's first live block.
        if matches!(startup, Startup::NewCatchup) {
            let id = loop {
                let id: Option<uuid::Uuid> = sqlx::query_scalar(
                    "SELECT id FROM host_consumer_catchups WHERE chain_id = $1 AND consumer_id = $2 AND requested_from = -100 AND requested_to = -1",
                ).bind(chain_id.as_i64()).bind(&identity).fetch_optional(&pool).await.unwrap();
                if let Some(id) = id { break id; }
                assert!(!run.is_finished(), "listener stopped before creating catchup");
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            };
            expected.push((id, 1900, 1999));
        }
        let mut actual = Vec::new();
        for _ in 0..expected.len() {
            let request = match pending_requests.pop() {
                Some(request) => request,
                None => requests.recv().await.unwrap(),
            };
            assert_eq!(request.consumer_id, identity);
            let id = request.catchup_id.unwrap();
            actual.push((id, request.block_start, request.block_end));
            for number in [request.block_start, request.block_end] {
                let fail_ingestion = matches!(startup, Startup::SameCatchup)
                    && !previous_requests && number == request.block_end;
                if fail_ingestion {
                    sqlx::raw_sql(
                        "CREATE SEQUENCE catchup_ingest_failures;
                         CREATE FUNCTION reject_catchup_block() RETURNS trigger LANGUAGE plpgsql AS $$
                         BEGIN
                             PERFORM nextval('catchup_ingest_failures');
                             RAISE EXCEPTION 'injected catchup ingestion failure';
                         END $$;
                         CREATE TRIGGER reject_catchup_block BEFORE INSERT OR UPDATE ON host_chain_blocks_valid
                         FOR EACH ROW EXECUTE FUNCTION reject_catchup_block();"
                    ).execute(&pool).await.unwrap();
                }
                publisher.publish(
                    &routing::consumer_final_catchup_event_routing(identity.clone()),
                    &event_block(chain_id.as_u64(), number, BlockFlow::FinalCatchup, Some(id)),
                ).await.unwrap();
                if fail_ingestion {
                    // Sequence increments survive rollback, proving ingestion
                    // actually failed before checking that DONE was not written.
                    loop {
                        let attempted: bool = sqlx::query_scalar("SELECT is_called FROM catchup_ingest_failures")
                            .fetch_one(&pool).await.unwrap();
                        if attempted { break; }
                        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                    }
                    let status: String = sqlx::query_scalar("SELECT status FROM host_consumer_catchups WHERE id = $1")
                        .bind(id).fetch_one(&pool).await.unwrap();
                    assert_eq!(status, "ONGOING", "failed ingestion must not complete the request");
                    assert_event_sets(&pool, &blocks).await;
                    sqlx::query("DROP TRIGGER reject_catchup_block ON host_chain_blocks_valid")
                        .execute(&pool).await.unwrap();
                }
                blocks.push(number);
                wait_for_events(&pool, &blocks).await;
                let expected_status = if number == request.block_end { "DONE" } else { "ONGOING" };
                loop {
                    let status: String = sqlx::query_scalar(
                        "SELECT status FROM host_consumer_catchups WHERE id = $1",
                    ).bind(id).fetch_one(&pool).await.unwrap();
                    if status == expected_status { break; }
                    assert_ne!(expected_status, "ONGOING", "catchup completed before its last block");
                    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                }
            }
        }
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected, "every retained request must be replayed exactly once");
        wait_for_events(&pool, &blocks).await;
        // Observe another reconciliation tick. Empty startup must also wait
        // through the ten-second WATCH propagation grace period.
        let observe_seconds = if expected.is_empty() { 16 } else { 6 };
        assert!(tokio::time::timeout(std::time::Duration::from_secs(observe_seconds), requests.recv()).await.is_err(),
            "startup must not create or reissue unexpected requests");
        let stored: Vec<(uuid::Uuid, i64, i64, String, bool)> = sqlx::query_as(
            "SELECT id, block_start, block_end, status, owner_id IS NOT NULL AND lease_until > clock_timestamp() FROM host_consumer_catchups WHERE chain_id = $1 AND consumer_id = $2 ORDER BY id",
        ).bind(chain_id.as_i64()).bind(&identity).fetch_all(&pool).await.unwrap();
        assert_eq!(stored, expected.iter().map(|(id, start, end)|
            (*id, *start as i64, *end as i64, "DONE".to_owned(), false)
        ).collect::<Vec<_>>());
        if run.is_finished() {
            panic!("listener stopped after replay: {:?}", run.await);
        }
        run.abort();
        let _ = run.await;
        stop.cancel();
        control.await.unwrap().unwrap();
        instance.parent_token.cancel();
    }).await.unwrap();
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis)"]
async fn no_flags_resume_all_retained_requests_and_observe_another_replicas_replacement(
) {
    use broker::{AsyncHandlerPayloadClassified, Topic};
    use host_listener::consumer::catchup::store;
    use primitives::{
        event::CancelCatchupPayload, event::CatchupPayload, routing,
        utils::chain_id_to_namespace,
    };
    use test_harness::instance::{setup_test_db, ImportMode};

    tokio::time::timeout(std::time::Duration::from_secs(90), async {
        let instance = setup_test_db(ImportMode::None).await.unwrap();
        let pool = sqlx::PgPool::connect(instance.db_url.as_str())
            .await
            .unwrap();
        let chain_id = ChainId::try_from(123456789u64).unwrap();
        let identity = format!("host-listener-consumer.{}", chain_id.as_u64());
        let flags = |from, to, keep| ManualCatchupArgs {
            catchup_from_block: Some(from),
            catchup_up_to_block: Some(to),
            catchup_unfinalized: false,
            no_override_catchup: keep,
        };
        let a = store::ensure_requested(
            &pool,
            chain_id.as_i64(),
            &identity,
            &flags(900, 910, true),
            Some(1000), None,
            false,
        )
        .await
        .unwrap()
        .unwrap();
        let mut unfinalized = flags(920, 930, true);
        unfinalized.catchup_unfinalized = true;
        let b = store::ensure_requested(
            &pool,
            chain_id.as_i64(),
            &identity,
            &unfinalized,
            Some(1000), None,
            false,
        )
        .await
        .unwrap()
        .unwrap();
        // Both former instances died after publishing; only their shared state remains.
        let former_owner = store::take_request_ownership(&pool, chain_id.as_i64(), &identity, false)
            .await.unwrap()[0].owner_id;
        store::requested(&pool, a, former_owner, false).await.unwrap();
        store::requested(&pool, b, former_owner, false).await.unwrap();
        let broker_url = std::env::var("CATCHUP_BROKER_URL").unwrap();
        let broker = Broker::from_url(&broker_url).await.unwrap();
        let namespace = chain_id_to_namespace(chain_id.as_u64());
        let stop = CancellationToken::new();
        let (request_tx, mut requests) = mpsc::unbounded_channel();
        let (cancel_tx, mut cancels) = mpsc::unbounded_channel();
        let mut controls = JoinSet::new();
        for (finalized, route, cancel_route) in [
            (true, routing::FINAL_CATCHUP, routing::CANCEL_FINAL_CATCHUP),
            (false, routing::CATCHUP, routing::CANCEL_CATCHUP),
        ] {
            let requests_consumer = broker
                .consumer(&Topic::namespaced(&namespace, route))
                .group("resume-requests")
                .with_cancellation(stop.clone()).build().unwrap();
            requests_consumer.ensure_topology().await.unwrap();
            let request_tx = request_tx.clone();
            controls.spawn(requests_consumer.run(AsyncHandlerPayloadClassified::new(
                move |request: CatchupPayload| {
                    let tx = request_tx.clone();
                    async move {
                        assert_eq!(finalized, request.catchup_id != Some(b));
                        tx.send(request).unwrap();
                        Ok(())
                    }
                },
            )));
            let cancels_consumer = broker
                .consumer(&Topic::namespaced(&namespace, cancel_route))
                .group("resume-cancels")
                .with_cancellation(stop.clone()).build().unwrap();
            cancels_consumer.ensure_topology().await.unwrap();
            let cancel_tx = cancel_tx.clone();
            controls.spawn(cancels_consumer.run(AsyncHandlerPayloadClassified::new(
                move |request: CancelCatchupPayload| {
                    let tx = cancel_tx.clone();
                    async move {
                        assert_eq!(finalized, request.catchup_id != b);
                        tx.send(request).unwrap();
                        Ok(())
                    }
                },
            )));
        }
        let config = test_config(
            broker_url,
            instance.db_url.clone(),
            chain_id,
            ManualCatchupArgs::default(),
        );
        let run = tokio::spawn(run_consumer(config));
        // An active owner prevents duplicate coordination by this no-flags process.
        assert!(tokio::time::timeout(std::time::Duration::from_millis(500), requests.recv()).await.is_err());
        sqlx::query("UPDATE host_consumer_catchups SET lease_until = clock_timestamp() - interval '1 second' WHERE id = $1")
            .bind(a).execute(&pool).await.unwrap();
        let first = requests.recv().await.unwrap().catchup_id.unwrap();
        assert_eq!(first, a, "only the abandoned request is claimed");
        assert!(tokio::time::timeout(std::time::Duration::from_millis(500), requests.recv()).await.is_err());
        sqlx::query("UPDATE host_consumer_catchups SET lease_until = clock_timestamp() - interval '1 second' WHERE id = $1")
            .bind(b).execute(&pool).await.unwrap();
        let mut ids = vec![first];
        // No live block is published: retained bounds must not need a new reference.
        ids.push(requests.recv().await.unwrap().catchup_id.unwrap());
        ids.sort();
        let mut expected = vec![a, b];
        expected.sort();
        assert_eq!(ids, expected);
        let publisher = broker.publisher("").await.unwrap();
        for (number, id) in [(900, a), (920, b)] {
            publisher
                .publish(
                    &if id == b { routing::consumer_catchup_event_routing(identity.clone()) }
                    else { routing::consumer_final_catchup_event_routing(identity.clone()) },
                    &event_block(
                        chain_id.as_u64(),
                        number,
                        if id == b { BlockFlow::Catchup } else { BlockFlow::FinalCatchup },
                        Some(id),
                    ),
                )
                .await
                .unwrap();
        }
        wait_for_events(&pool, &[900, 920]).await;
        // Simulate takeover while this process still has the old local token.
        sqlx::query("UPDATE host_consumer_catchups SET lease_until = clock_timestamp() - interval '1 second' WHERE id = $1")
            .bind(a).execute(&pool).await.unwrap();
        let takeover = store::take_request_ownership(&pool, chain_id.as_i64(), &identity, false)
            .await.unwrap();
        assert_eq!(takeover.len(), 1);
        assert_eq!(takeover[0].id, a);
        assert!(tokio::time::timeout(std::time::Duration::from_secs(6), requests.recv()).await.is_err(),
            "the old coordinator must not publish the new owner's pending request");
        let actual: (Option<uuid::Uuid>, bool) = sqlx::query_as(
            "SELECT owner_id, request_pending FROM host_consumer_catchups WHERE id = $1")
            .bind(a).fetch_one(&pool).await.unwrap();
        assert_eq!(actual, (Some(takeover[0].owner_id), true));
        // A second replica changes shared intent. The running no-flags replica
        // must discover the commands and must not retain its startup ID snapshot.
        let c = store::ensure_requested(
            &pool,
            chain_id.as_i64(),
            &identity,
            &flags(940, 950, false),
            Some(1000), None,
            false,
        )
        .await
        .unwrap()
        .unwrap();
        // The foreign owner also dies; cancellation must survive its expired lease.
        sqlx::query("UPDATE host_consumer_catchups SET lease_until = clock_timestamp() - interval '1 second' WHERE id = $1")
            .bind(a).execute(&pool).await.unwrap();
        assert_eq!(requests.recv().await.unwrap().catchup_id, Some(c));
        let mut cancelled = vec![
            cancels.recv().await.unwrap().catchup_id,
            cancels.recv().await.unwrap().catchup_id,
        ];
        cancelled.sort();
        assert_eq!(cancelled, expected);
        for (number, id) in [(901, a), (921, b), (940, c)] {
            publisher
                .publish(
                    &if id == b { routing::consumer_catchup_event_routing(identity.clone()) }
                    else { routing::consumer_final_catchup_event_routing(identity.clone()) },
                    &event_block(
                        chain_id.as_u64(),
                        number,
                        if id == b { BlockFlow::Catchup } else { BlockFlow::FinalCatchup },
                        Some(id),
                    ),
                )
                .await
                .unwrap();
        }
        wait_for_events(&pool, &[900, 920, 940]).await;
        let observer = consumer::ListenerConsumer::new(
            &broker,
            chain_id.as_u64(),
            &identity,
        );
        loop {
            let status = observer.group_status().await.unwrap();
            let drained = status
                .iter()
                .find(|(topic, _)| topic.ends_with(".final-catchup-event"))
                .is_some_and(|(_, groups)| {
                    !groups.is_empty()
                        && groups
                            .iter()
                            .all(|g| g.pending == 0 && g.lag == Some(0))
                });
            if drained {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        assert_event_sets(&pool, &[900, 920, 940]).await;
        assert!(
            !run.is_finished(),
            "catchup readers stay alive after the last delivered block"
        );
        run.abort();
        let _ = run.await;
        stop.cancel();
        while let Some(result) = controls.join_next().await {
            result.unwrap().unwrap();
        }
    })
    .await
    .unwrap();
}

/// Exercises public startup with real PostgreSQL ingestion and a broker control peer.
#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn consumer_catchup_and_listen() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_catchup(false, false),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn consumer_bounded_catchup_keeps_listening() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_catchup(true, false),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "requires PostgreSQL and CATCHUP_BROKER_URL (Redis or RabbitMQ)"]
async fn consumer_unfinalized_catchup_keeps_listening() {
    tokio::time::timeout(
        std::time::Duration::from_secs(90),
        exercise_catchup(true, true),
    )
    .await
    .unwrap();
}

async fn exercise_catchup(bounded: bool, unfinalized: bool) {
    use broker::{AsyncHandlerPayloadClassified, Topic};
    use primitives::{
        event::{CatchupPayload, WatchCommand},
        routing,
        utils::chain_id_to_namespace,
    };
    use test_harness::instance::{setup_test_db, ImportMode};
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let broker_url = std::env::var("CATCHUP_BROKER_URL").unwrap();
    let broker = Broker::from_url(&broker_url).await.unwrap();
    let chain_id = ChainId::try_from(
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            % 900_000_000) as u64
            + 100_000_000,
    )
    .unwrap();
    let config = test_config(
        broker_url,
        instance.db_url.clone(),
        chain_id,
        ManualCatchupArgs {
            catchup_from_block: Some(-100),
            catchup_up_to_block: bounded.then_some(950),
            catchup_unfinalized: unfinalized,
            no_override_catchup: false,
        },
    );
    let pool = sqlx::PgPool::connect(instance.db_url.as_str())
        .await
        .unwrap();
    let namespace = chain_id_to_namespace(chain_id.as_u64());
    let (watch_tx, mut watch_rx) = mpsc::unbounded_channel();
    let (catchup_tx, mut catchup_rx) = mpsc::unbounded_channel();
    let control_stop = CancellationToken::new();
    let mut controls = JoinSet::new();
    let watch = broker
        .consumer(&Topic::namespaced(&namespace, routing::WATCH))
        .group("catchup-test-watch")
        .with_cancellation(control_stop.clone())
        .build()
        .unwrap();
    watch.ensure_topology().await.unwrap();
    controls.spawn(watch.run(AsyncHandlerPayloadClassified::new(
        move |command: WatchCommand| {
            let tx = watch_tx.clone();
            async move {
                let filters = command.into_filters();
                assert_eq!(filters.len(), 6);
                assert_eq!(
                    filters
                        .iter()
                        .filter(|f| f.filter_type
                            == Some(primitives::event::FilterType::Final))
                        .count(),
                    3
                );
                tx.send(filters[0].consumer_id.clone()).unwrap();
                Ok(())
            }
        },
    )));
    let replay_flow = if unfinalized {
        BlockFlow::Catchup
    } else {
        BlockFlow::FinalCatchup
    };
    let replay_routing = |id: String| {
        if unfinalized {
            routing::consumer_catchup_event_routing(id)
        } else {
            routing::consumer_final_catchup_event_routing(id)
        }
    };
    let catchup = broker
        .consumer(&Topic::namespaced(
            &namespace,
            if unfinalized {
                routing::FINAL_CATCHUP
            } else {
                routing::CATCHUP
            },
        ))
        .group("catchup-test-catchup")
        .with_cancellation(control_stop.clone())
        .build()
        .unwrap();
    catchup.ensure_topology().await.unwrap();
    controls.spawn(catchup.run(AsyncHandlerPayloadClassified::new(
        move |command: CatchupPayload| {
            let tx = catchup_tx.clone();
            async move {
                tx.send(command).unwrap();
                Ok(())
            }
        },
    )));
    let (request_tx, mut request_rx) = mpsc::unbounded_channel();
    let request_control = broker
        .consumer(&Topic::namespaced(
            &namespace,
            if unfinalized {
                routing::CATCHUP
            } else {
                routing::FINAL_CATCHUP
            },
        ))
        .group("catchup-test-final-catchup")
        .with_cancellation(control_stop.clone())
        .build()
        .unwrap();
    request_control.ensure_topology().await.unwrap();
    controls.spawn(request_control.run(AsyncHandlerPayloadClassified::new(
        move |command: CatchupPayload| {
            let tx = request_tx.clone();
            async move {
                tx.send(command).unwrap();
                Ok(())
            }
        },
    )));
    let run = tokio::spawn(run_consumer(config));
    let publisher = broker.publisher("").await.unwrap();
    let block = |number, flow, catchup_id| {
        event_block(chain_id.as_u64(), number, flow, catchup_id)
    };
    let first = watch_rx.recv().await.unwrap();
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(1000, BlockFlow::Live, None),
        )
        .await
        .unwrap();
    let request = request_rx.recv().await.unwrap();
    assert_eq!(
        (request.block_start, request.block_end),
        (900, if bounded { 950 } else { 999 })
    );
    assert_eq!(request.consumer_id, first);
    let catchup_id =
        request.catchup_id.expect("catchup request carries its id");
    // Live advances before historical delivery finishes.
    publisher
        .publish(
            &replay_routing(first.clone()),
            &block(900, replay_flow, Some(catchup_id)),
        )
        .await
        .unwrap();
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(1001, BlockFlow::Live, None),
        )
        .await
        .unwrap();
    wait_for_events(&pool, &[900, 1000, 1001]).await;
    let end = request.block_end;
    if bounded {
        // The same block delivered on the ordinary final flow must not
        // complete this catchup, even if it carries its ID.
        publisher
            .publish(
                &routing::consumer_final_event_routing(first.clone()),
                &block(end, BlockFlow::Final, Some(catchup_id)),
            )
            .await
            .unwrap();
        wait_for_events(&pool, &[900, end, 1000, 1001]).await;
        let status: String = sqlx::query_scalar(
            "SELECT status FROM host_consumer_catchups WHERE id = $1",
        )
        .bind(catchup_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(status, "ONGOING");
    }
    // Out-of-order chunks and duplicate deliveries must preserve both event sets.
    for number in [end, 925, 900, end] {
        publisher
            .publish(
                &replay_routing(first.clone()),
                &block(number, replay_flow, Some(catchup_id)),
            )
            .await
            .unwrap();
    }
    wait_for_events(&pool, &[900, 925, end, 1000, 1001]).await;
    loop {
        let status: String = sqlx::query_scalar(
            "SELECT status FROM host_consumer_catchups WHERE id = $1",
        )
        .bind(catchup_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        if status == "DONE" {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    if unfinalized {
        let finals: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id = $1 AND block_number IN (900, 925) AND block_status = 'finalized'")
            .bind(chain_id.as_i64()).fetch_one(&pool).await.unwrap();
        assert_eq!(
            finals, 0,
            "ordinary catchup must not finalize its historical blocks"
        );
    }
    assert!(
        !run.is_finished(),
        "bounded replay must not stop live consumption"
    );
    publisher
        .publish(
            &routing::consumer_new_event_routing(first.clone()),
            &block(1002, BlockFlow::Live, None),
        )
        .await
        .unwrap();
    wait_for_events(&pool, &[900, 925, end, 1000, 1001, 1002]).await;
    assert!(
        catchup_rx.try_recv().is_err(),
        "only the selected catchup flow is requested"
    );
    // Final delivery repairs an event missed by live ingestion and catchup.
    publisher
        .publish(
            &routing::consumer_final_event_routing(first.clone()),
            &block(910, BlockFlow::Final, None),
        )
        .await
        .unwrap();
    for _ in 0..2 {
        publisher
            .publish(
                &replay_routing(first.clone()),
                &block(900, replay_flow, Some(catchup_id)),
            )
            .await
            .unwrap();
    }
    wait_for_events(&pool, &[900, 910, 925, end, 1000, 1001, 1002]).await;
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let finals: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id = $1 AND block_number IN (900, 910, 925, $2) AND block_status = 'finalized'")
                .bind(chain_id.as_i64()).bind(end as i64).fetch_one(&pool).await.unwrap();
            if finals == if unfinalized { 2 } else { 4 } { break; }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    }).await.unwrap();
    // Only live delivery contributes timing rows. Historical replay (including
    // finalized replay when subscribed) must not pollute live timing statistics.
    let timed_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM host_chain_consumer_blocks WHERE chain_id = $1 ORDER BY block_number",
    ).bind(chain_id.as_i64()).fetch_all(&pool).await.unwrap();
    assert_eq!(timed_blocks, vec![1000, 1001, 1002]);
    // Public startup has no external stop handle. Each test owns its Tokio runtime,
    // which also drops the process services spawned by the consumer.
    run.abort();
    let _ = run.await;
    control_stop.cancel();
    while let Some(result) = controls.join_next().await {
        result.unwrap().unwrap();
    }
    instance.parent_token.cancel();
}

// Real ABI-encoded TFHE and ACL events exercise the same ingestion as RPC payloads.
fn event_block(
    chain_id: u64,
    number: u64,
    flow: BlockFlow,
    catchup_id: Option<uuid::Uuid>,
) -> BlockPayload {
    use alloy::{
        primitives::{Address, B256, U256},
        sol_types::SolEvent,
    };
    use primitives::event::{IndexedLog, TransactionPayload};
    let handle = B256::from(U256::from(number).to_be_bytes::<32>());
    let caller = Address::repeat_byte(2);
    let tfhe = host_listener::contracts::TfheContract::TrivialEncrypt {
        caller,
        pt: U256::from(number),
        toType: 4,
        result: handle,
    }
    .encode_log_data();
    let acl = host_listener::contracts::AclContract::Allowed {
        caller,
        account: caller,
        handle,
    }
    .encode_log_data();
    BlockPayload {
        flow,
        chain_id,
        block_number: number,
        block_hash: handle,
        parent_hash: B256::ZERO,
        timestamp: 1_700_000_000,
        catchup_id,
        transactions: vec![TransactionPayload {
            from: caller,
            to: Some(Address::repeat_byte(1)),
            hash: handle,
            transaction_index: 0,
            value: U256::ZERO,
            data: Default::default(),
            logs: [(Address::repeat_byte(1), tfhe), (Address::ZERO, acl)]
                .into_iter()
                .enumerate()
                .map(|(index, (address, data))| IndexedLog {
                    log_index: index as u64,
                    address,
                    topics: data.topics().to_vec(),
                    data: data.data,
                })
                .collect(),
        }],
    }
}

async fn event_sets(pool: &sqlx::PgPool) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let computations = sqlx::query_scalar(
        "SELECT output_handle FROM computations ORDER BY output_handle",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    let allowed = sqlx::query_scalar(
        "SELECT handle FROM allowed_handles ORDER BY handle",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    (computations, allowed)
}

fn expected_handles(blocks: &[u64]) -> Vec<Vec<u8>> {
    let mut expected: Vec<_> = blocks
        .iter()
        .map(|n| {
            alloy::primitives::U256::from(*n)
                .to_be_bytes::<32>()
                .to_vec()
        })
        .collect();
    expected.sort();
    expected
}

async fn assert_event_sets(pool: &sqlx::PgPool, blocks: &[u64]) {
    let expected = expected_handles(blocks);
    assert_eq!(event_sets(pool).await, (expected.clone(), expected));
}

async fn wait_for_events(pool: &sqlx::PgPool, blocks: &[u64]) {
    let expected = expected_handles(blocks);
    let result =
        tokio::time::timeout(std::time::Duration::from_secs(15), async {
            loop {
                let (computations, allowed) = event_sets(pool).await;
                if computations == expected && allowed == expected {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            }
        })
        .await;
    assert_event_sets(pool, blocks).await;
    result.unwrap();
}

fn test_config(
    broker_url: String,
    database_url: fhevm_engine_common::utils::DatabaseURL,
    chain_id: ChainId,
    manual_catchup: ManualCatchupArgs,
) -> ConsumerConfig {
    ConsumerConfig {
        catchup_from_block: manual_catchup.catchup_from_block,
        catchup_up_to_block: manual_catchup.catchup_up_to_block,
        catchup_unfinalized: manual_catchup.catchup_unfinalized,
        no_override_catchup: manual_catchup.no_override_catchup,
        url: broker_url,
        acl_address: Address::ZERO,
        tfhe_address: Address::repeat_byte(1),
        kms_generation_address: Some(Address::repeat_byte(2)),
        protocol_config_address: None,
        confidential_bridge_address: None,
        database_url,
        database_retry_interval: std::time::Duration::from_millis(10),
        migrate_from_service_name: None,
        health_port: 0,
        dependence_cache_size: 16,
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 1,
        chain_id: chain_id.as_u64().to_string(),
        gcs_mode: false,
        disable_synthetic_ops: true,
        canonical_protocol_config_chain_id: None,
    }
}
