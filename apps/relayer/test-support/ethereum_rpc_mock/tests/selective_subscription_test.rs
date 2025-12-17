//! Tests for selective subscription targeting
//!
//! Verifies that the mock server can send events to specific subscriptions
//! by index, enabling tests of event deduplication across multiple listeners.

use alloy::primitives::{Address, Bytes};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use ethereum_rpc_mock::test_utils::get_free_port;
use ethereum_rpc_mock::{MockConfig, MockServer, SubscriptionTarget};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

/// Test: Verify subscription count tracking
#[tokio::test]
async fn test_subscription_count_tracking() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    // Initially no subscriptions
    assert_eq!(server.get_log_subscription_count().await, 0);

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    let url = format!("ws://127.0.0.1:{}/ws", port);

    // Connect first subscriber
    let provider1 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();

    let _sub1 = provider1.subscribe_logs(&Filter::new()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(server.get_log_subscription_count().await, 1);

    // Connect second subscriber
    let provider2 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();

    let _sub2 = provider2.subscribe_logs(&Filter::new()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(server.get_log_subscription_count().await, 2);

    // Connect third subscriber
    let provider3 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();

    let _sub3 = provider3.subscribe_logs(&Filter::new()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(server.get_log_subscription_count().await, 3);

    handle.shutdown().await.unwrap();
}

/// Test: SubscriptionTarget::All sends to all subscriptions
#[tokio::test]
async fn test_target_all_sends_to_all_subscriptions() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    let url = format!("ws://127.0.0.1:{}/ws", port);

    // Create 2 subscriptions
    let provider0 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();
    let mut stream0 = provider0
        .subscribe_logs(&Filter::new())
        .await
        .unwrap()
        .into_stream();

    let provider1 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();
    let mut stream1 = provider1
        .subscribe_logs(&Filter::new())
        .await
        .unwrap()
        .into_stream();

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Emit with Target::All using ScheduledTransaction
    let test_log = alloy::primitives::Log {
        address: Address::repeat_byte(1),
        data: alloy::primitives::LogData::new_unchecked(vec![], Bytes::from(vec![1, 2, 3])),
    };

    // Use the existing transaction mechanism with SubscriptionTarget::All
    use ethereum_rpc_mock::ScheduledTransaction;
    server.blockchain_state().schedule_delayed_transaction(
        ScheduledTransaction::with_single_event(
            Duration::from_millis(100),
            Some(Address::repeat_byte(1)),
            test_log,
        ),
        server.log_subscriptions().clone(),
    );

    // Both should receive
    let result0 = timeout(Duration::from_secs(2), stream0.next()).await;
    assert!(
        result0.is_ok() && result0.unwrap().is_some(),
        "Subscription 0 should receive event"
    );

    let result1 = timeout(Duration::from_secs(2), stream1.next()).await;
    assert!(
        result1.is_ok() && result1.unwrap().is_some(),
        "Subscription 1 should receive event"
    );

    handle.shutdown().await.unwrap();
}

/// Test: SubscriptionTarget::Only sends to specific indices
#[tokio::test]
async fn test_target_only_sends_to_specific_indices() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    let url = format!("ws://127.0.0.1:{}/ws", port);

    // Create 3 subscriptions
    let provider0 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();
    let mut stream0 = provider0
        .subscribe_logs(&Filter::new())
        .await
        .unwrap()
        .into_stream();

    let provider1 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();
    let mut stream1 = provider1
        .subscribe_logs(&Filter::new())
        .await
        .unwrap()
        .into_stream();

    let provider2 = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();
    let mut stream2 = provider2
        .subscribe_logs(&Filter::new())
        .await
        .unwrap()
        .into_stream();

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Emit only to indices 0 and 2
    let test_log = alloy::primitives::Log {
        address: Address::repeat_byte(1),
        data: alloy::primitives::LogData::new_unchecked(vec![], Bytes::from(vec![1, 2, 3])),
    };

    use ethereum_rpc_mock::ScheduledTransaction;
    let transaction = ScheduledTransaction {
        target_address: Some(Address::repeat_byte(1)),
        response_events: vec![(
            Duration::from_millis(100),
            test_log,
            SubscriptionTarget::only(vec![0, 2]),
        )],
    };

    server
        .blockchain_state()
        .schedule_delayed_transaction(transaction, server.log_subscriptions().clone());

    // 0 and 2 should receive
    let result0 = timeout(Duration::from_secs(2), stream0.next()).await;
    assert!(
        result0.is_ok() && result0.unwrap().is_some(),
        "Subscription 0 should receive event"
    );

    let result2 = timeout(Duration::from_secs(2), stream2.next()).await;
    assert!(
        result2.is_ok() && result2.unwrap().is_some(),
        "Subscription 2 should receive event"
    );

    // 1 should NOT receive (timeout)
    let result1 = timeout(Duration::from_millis(500), stream1.next()).await;
    assert!(
        result1.is_err(),
        "Subscription 1 should NOT receive event (should timeout)"
    );

    handle.shutdown().await.unwrap();
}

/// Test: SubscriptionTarget::Only([]) sends to no one
#[tokio::test]
async fn test_target_empty_sends_to_none() {
    let port = get_free_port().unwrap();
    let server = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    let handle = server.clone().start().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    let url = format!("ws://127.0.0.1:{}/ws", port);

    let provider = ProviderBuilder::new()
        .network::<alloy::network::AnyNetwork>()
        .connect_ws(WsConnect::new(&url))
        .await
        .unwrap();

    let mut stream = provider
        .subscribe_logs(&Filter::new())
        .await
        .unwrap()
        .into_stream();

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Emit to NO indices
    let test_log = alloy::primitives::Log {
        address: Address::repeat_byte(1),
        data: alloy::primitives::LogData::new_unchecked(vec![], Bytes::from(vec![1, 2, 3])),
    };

    use ethereum_rpc_mock::ScheduledTransaction;
    let transaction = ScheduledTransaction {
        target_address: Some(Address::repeat_byte(1)),
        response_events: vec![(
            Duration::from_millis(100),
            test_log,
            SubscriptionTarget::only(vec![]),
        )],
    };

    server
        .blockchain_state()
        .schedule_delayed_transaction(transaction, server.log_subscriptions().clone());

    // Should NOT receive
    let result = timeout(Duration::from_millis(500), stream.next()).await;
    assert!(
        result.is_err(),
        "Should NOT receive event when emitting to empty indices"
    );

    handle.shutdown().await.unwrap();
}
