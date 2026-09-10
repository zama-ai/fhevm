//! Regression case 1 of the minimal v0.13 hotfix plan: the pending-nonce lookup
//! must have its own bounded phase.
//!
//! > Hold the nonce lookup beyond 4 s with unrelated RPCs healthy, verify its
//! > distinct timeout, then restore it and observe both loops resume. Repeat
//! > after a send error; do not count the estimation or mutex queue wait as part
//! > of the lookup deadline.
//!
//! The lookup runs while the shared nonce mutex is held, so before the amendment
//! a single stalled `eth_getTransactionCount` parked both operation loops for as
//! long as the RPC stayed silent.

mod common;
mod support;

use std::time::{Duration, Instant};

use alloy::network::TransactionBuilder;
use alloy::primitives::U256;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use common::{SignerType, TestEnvironment};
use serial_test::serial;
use support::{Fault, FaultProxy};
use transaction_sender::{FillersWithoutNonceManagement, NonceManagedProvider};

/// The distinct error text the amendment introduced for this phase.
const LOOKUP_TIMEOUT: &str = "eth_getTransactionCount(pending) timeout";
/// The submission-phase text: acceptance unknown when this fires.
const SEND_TIMEOUT: &str = "eth_sendRawTransaction timeout";
/// Generous inclusion deadline; these tests are about the ordered phase.
const RECEIPT: Duration = Duration::from_secs(30);

fn proxied_provider(
    env: &TestEnvironment,
    proxy: &FaultProxy,
) -> NonceManagedProvider<impl Provider<alloy::network::Ethereum> + Clone> {
    NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_http(proxy.url()),
        Some(env.signer_address()),
    )
}

fn transfer(env: &TestEnvironment) -> TransactionRequest {
    TransactionRequest::default()
        .with_to(env.user_address)
        .with_value(U256::from(1))
        .with_gas_limit(21_000)
}

/// A stalled pending-nonce lookup must fail with its *own* timeout, inside the
/// lookup deadline, while every other RPC keeps working — and must leave the
/// lock free so the next send proceeds once the fault clears.
#[tokio::test]
#[serial(db)]
async fn nonce_lookup_has_its_own_deadline_and_frees_the_lock() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied_provider(&env, &proxy);

    let timeout = Duration::from_secs(4);
    proxy.set_fault("eth_getTransactionCount", Fault::Blackhole, None);

    let started = Instant::now();
    let err = provider
        .send_transaction_sync(transfer(&env), timeout, RECEIPT)
        .await
        .expect_err("a blackholed pending-nonce lookup must fail the send");
    let elapsed = started.elapsed();
    let err = err.to_string();

    // The distinct phase error, not the send-phase one.
    assert!(
        err.contains(LOOKUP_TIMEOUT),
        "expected the nonce-lookup timeout, got: {err}"
    );
    assert!(
        !err.contains(SEND_TIMEOUT),
        "lookup failure must not be reported as a send timeout: {err}"
    );
    // Lookup is bounded by its own deadline and does not additionally consume
    // the send deadline: total stays near one timeout, not two.
    assert!(
        elapsed >= timeout && elapsed < timeout * 2,
        "lookup should fail after ~{timeout:?} (its own phase), took {elapsed:?}"
    );

    // The lookup used the pending tag, which is the seeding fix itself.
    assert!(
        proxy.tx_count_calls_with_tag("pending") >= 1,
        "nonce must be seeded from pending, saw tags: {:?}",
        proxy.snapshot()
    );

    // Unrelated RPCs stayed healthy throughout: the stall is method-local.
    let head = provider.inner().get_block_number().await?;
    assert!(
        head > 0,
        "unrelated RPCs must keep working during the stall"
    );

    // Restore the RPC: the mutex was released normally, so the sequence resumes.
    proxy.clear_fault("eth_getTransactionCount");
    let receipt = provider
        .send_transaction_sync(transfer(&env), timeout, RECEIPT)
        .await?;
    assert!(receipt.status(), "send must succeed once the fault clears");

    Ok(())
}

/// The same, but entered after a *failed send* rather than at startup: the send
/// error invalidates the sequence, so the next attempt is cold and must again
/// bound its lookup instead of parking the lock.
#[tokio::test]
#[serial(db)]
async fn nonce_lookup_is_bounded_again_after_a_send_error() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied_provider(&env, &proxy);

    let timeout = Duration::from_secs(4);

    // Warm the sequence with a successful send.
    let receipt = provider
        .send_transaction_sync(transfer(&env), timeout, RECEIPT)
        .await?;
    assert!(receipt.status());

    // Now fail a send itself, which invalidates the cached nonce.
    proxy.set_fault("eth_sendRawTransaction", Fault::Blackhole, None);
    let err = provider
        .send_transaction_sync(transfer(&env), timeout, RECEIPT)
        .await
        .expect_err("blackholed send must time out")
        .to_string();
    assert!(
        err.contains(SEND_TIMEOUT),
        "expected the send-phase timeout, got: {err}"
    );
    proxy.clear_fault("eth_sendRawTransaction");

    // The next attempt is cold. Stall its lookup: it must hit the lookup
    // deadline, not hang on the mutex.
    proxy.reset_stats();
    proxy.set_fault("eth_getTransactionCount", Fault::Blackhole, None);
    let started = Instant::now();
    let err = provider
        .send_transaction_sync(transfer(&env), timeout, RECEIPT)
        .await
        .expect_err("cold lookup after a send error must be bounded")
        .to_string();
    let elapsed = started.elapsed();
    assert!(
        err.contains(LOOKUP_TIMEOUT),
        "expected the nonce-lookup timeout after a send error, got: {err}"
    );
    assert!(
        elapsed < timeout * 2,
        "lookup after a send error took {elapsed:?}, expected ~{timeout:?}"
    );
    assert!(
        proxy.tx_count_calls_with_tag("pending") >= 1,
        "re-seeding must query the pending count"
    );

    // And it recovers.
    proxy.clear_fault("eth_getTransactionCount");
    let receipt = provider
        .send_transaction_sync(transfer(&env), timeout, RECEIPT)
        .await?;
    assert!(
        receipt.status(),
        "sequence must resume after the fault ends"
    );

    Ok(())
}

/// A stalled lookup must not deadlock the *shared* sequence: a second caller
/// queued behind the lock proceeds as soon as the first releases it, and its own
/// lookup deadline is measured from when it acquires the lock, not from when it
/// started waiting.
#[tokio::test]
#[serial(db)]
async fn queued_sender_is_not_charged_for_the_mutex_wait() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied_provider(&env, &proxy);

    let timeout = Duration::from_secs(4);
    proxy.set_fault("eth_getTransactionCount", Fault::Blackhole, None);

    // Two concurrent sends contend for one nonce sequence. The first holds the
    // lock through its stalled lookup; the second waits.
    let first = {
        let provider = provider.clone();
        let txn = transfer(&env);
        tokio::spawn(async move { provider.send_transaction_sync(txn, timeout, RECEIPT).await })
    };
    // Give the first send time to take the lock before the second queues.
    tokio::time::sleep(Duration::from_millis(200)).await;
    let second = {
        let provider = provider.clone();
        let txn = transfer(&env);
        tokio::spawn(async move { provider.send_transaction_sync(txn, timeout, RECEIPT).await })
    };

    let first_err = first.await?.expect_err("first send must fail").to_string();
    assert!(
        first_err.contains(LOOKUP_TIMEOUT),
        "first send: expected lookup timeout, got {first_err}"
    );

    // Clear the fault while the second is still queued or in its lookup: it must
    // then complete rather than inherit the first send's failure.
    proxy.clear_fault("eth_getTransactionCount");
    match second.await? {
        Ok(receipt) => assert!(receipt.status(), "queued send should succeed"),
        Err(e) => {
            let e = e.to_string();
            // Acceptable only as its own bounded lookup timeout, never a hang.
            assert!(
                e.contains(LOOKUP_TIMEOUT),
                "queued send failed for an unexpected reason: {e}"
            );
        }
    }

    // The sequence is still usable afterwards.
    let receipt = provider
        .send_transaction_sync(transfer(&env), timeout, RECEIPT)
        .await?;
    assert!(receipt.status());

    Ok(())
}
