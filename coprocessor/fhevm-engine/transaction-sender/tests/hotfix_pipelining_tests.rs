//! Validation for the ordered-broadcast / receipt-tracking change: the nonce
//! mutex covers nonce selection and submission acknowledgment only, and
//! inclusion is awaited outside it.
//!
//! Three properties are pinned here, matching the review's requirements 1-3:
//!
//! 1. Capacity is fixed at the mechanism, not inferred from throughput —
//!    several transactions from one signer are submitted before any receipt
//!    returns, and land in a single block with consecutive nonces.
//! 2. Submission uncertainty is handled: rejected, silently accepted, delayed
//!    past the client deadline, and a stale pending count. None may produce a
//!    persistent gap or a conflicting resubmission.
//! 3. Receipt uncertainty is handled *differently* from submission uncertainty:
//!    a lost receipt must not disturb nonces already handed to later
//!    transactions, and must not re-seed the sequence.

mod common;
mod support;

use std::time::Duration;

use alloy::consensus::Transaction;
use alloy::network::TransactionBuilder;
use alloy::primitives::{FixedBytes, U256};
use alloy::providers::ext::AnvilApi;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::TransactionRequest;
use common::{CiphertextCommits, SignerType, TestEnvironment};
use serial_test::serial;
use support::{Fault, FaultProxy};
use transaction_sender::{FillersWithoutNonceManagement, NonceManagedProvider};

const SUBMIT: Duration = Duration::from_secs(4);
const RECEIPT: Duration = Duration::from_secs(60);

fn transfer(env: &TestEnvironment) -> TransactionRequest {
    TransactionRequest::default()
        .with_to(env.user_address)
        .with_value(U256::from(1))
        .with_gas_limit(21_000)
}

fn proxied(
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

/// Nonces mined for this signer, in block order, with the block number.
async fn mined_nonces<P: Provider>(
    provider: &P,
    signer: alloy::primitives::Address,
) -> anyhow::Result<Vec<(u64, u64)>> {
    let head = provider.get_block_number().await?;
    let mut out = Vec::new();
    for number in 0..=head {
        let Some(block) = provider.get_block_by_number(number.into()).full().await? else {
            continue;
        };
        for txn in block.transactions.txns() {
            if txn.inner.signer() == signer {
                out.push((txn.inner.nonce(), number));
            }
        }
    }
    out.sort_unstable();
    Ok(out)
}

async fn pause_mining<P: Provider>(provider: &P) -> anyhow::Result<()> {
    provider.anvil_set_interval_mining(0).await?;
    provider.anvil_set_auto_mine(false).await?;
    Ok(())
}

/// Requirement 1: the capacity fix, demonstrated at the mechanism.
///
/// With mining paused, eight concurrent sends must all reach the node before any
/// receipt can possibly return — that is only possible if the nonce mutex is
/// released at submission acknowledgment. They then all mine in one block on
/// consecutive nonces, which is what the previous design made impossible: it
/// held the mutex until a receipt arrived, so the node was never offered the
/// second transaction until the first had been included.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn submissions_pipeline_ahead_of_inclusion() -> anyhow::Result<()> {
    const CONCURRENT: usize = 8;

    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    let control = ProviderBuilder::new()
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    pause_mining(&control).await?;

    let start_nonce = control.get_transaction_count(env.signer_address()).await?;

    let mut sends = tokio::task::JoinSet::new();
    for _ in 0..CONCURRENT {
        let provider = provider.clone();
        let txn = transfer(&env);
        sends.spawn(async move { provider.send_transaction_sync(txn, SUBMIT, RECEIPT).await });
    }

    // Every submission must be acknowledged while nothing can be mined.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while proxy.calls("eth_sendRawTransaction") < CONCURRENT {
        if tokio::time::Instant::now() >= deadline {
            anyhow::bail!(
                "only {} of {CONCURRENT} transactions were submitted before inclusion; \
                 the nonce mutex is still held across the receipt wait",
                proxy.calls("eth_sendRawTransaction")
            );
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Nothing has been mined, so no receipt has returned: submission really did
    // run ahead of inclusion rather than merely fast.
    assert_eq!(
        control.get_block_number().await?,
        0,
        "no block should have been mined yet"
    );
    let pending_count = control
        .get_transaction_count(env.signer_address())
        .pending()
        .await?;
    assert_eq!(
        pending_count - start_nonce,
        CONCURRENT as u64,
        "the node should be holding {CONCURRENT} pending transactions"
    );

    // One block is enough for all of them.
    control.anvil_mine(Some(1), None).await?;

    let mut ok = 0;
    while let Some(joined) = sends.join_next().await {
        if joined?.is_ok() {
            ok += 1;
        }
    }
    assert_eq!(
        ok, CONCURRENT,
        "every pipelined send should get its receipt"
    );

    let mined = mined_nonces(&control, env.signer_address()).await?;
    assert_eq!(mined.len(), CONCURRENT, "all transactions should be mined");
    let expected: Vec<u64> = (start_nonce..start_nonce + CONCURRENT as u64).collect();
    let got: Vec<u64> = mined.iter().map(|(n, _)| *n).collect();
    assert_eq!(got, expected, "nonces must be consecutive with no gap");
    let blocks: std::collections::BTreeSet<u64> = mined.iter().map(|(_, b)| *b).collect();
    assert_eq!(
        blocks.len(),
        1,
        "all {CONCURRENT} transactions should share one block, saw {blocks:?}"
    );

    // Exactly one pending lookup for the whole burst: the sequence was warm and
    // never re-seeded.
    assert_eq!(
        proxy.tx_count_calls_with_tag("pending"),
        1,
        "the sequence should be seeded once and then advanced locally"
    );

    Ok(())
}

/// Requirement 2a: rejected before acceptance. The nonce was never consumed, so
/// the next send must take it and leave no gap.
#[tokio::test]
#[serial(db)]
async fn rejected_submission_leaves_no_gap() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    let start_nonce = provider.get_transaction_count(env.signer_address()).await?;

    let doomed = TransactionRequest::default()
        .with_to(env.user_address)
        .with_value(U256::from(1))
        .with_gas_limit(u64::MAX / 2);
    provider
        .send_transaction_sync(doomed, SUBMIT, RECEIPT)
        .await
        .expect_err("a transaction over the block gas limit must be rejected");

    let receipt = provider
        .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
        .await?;
    assert!(receipt.status());

    let txn = provider
        .inner()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .expect("mined transaction is retrievable");
    assert_eq!(
        txn.inner.nonce(),
        start_nonce,
        "a rejected submission must not consume a nonce"
    );
    Ok(())
}

/// Requirement 2b/2c: accepted but the acknowledgment never arrives, and an
/// acknowledgment delayed past the client deadline. Acceptance is unknown to the
/// client in both cases, so the sequence must re-seed from `pending` — which
/// counts the transaction the node did accept — rather than reuse its nonce.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn unacknowledged_submission_does_not_reuse_its_nonce() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    let start_nonce = provider.get_transaction_count(env.signer_address()).await?;

    for (label, fault) in [
        ("suppressed acknowledgment", Fault::SuppressResponse),
        (
            "acknowledgment delayed past the deadline",
            Fault::DelayResponse(SUBMIT * 3),
        ),
    ] {
        proxy.set_fault("eth_sendRawTransaction", fault, Some(1));
        let err = provider
            .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
            .await
            .expect_err("the client must give up on the acknowledgment")
            .to_string();
        assert!(
            err.contains("eth_sendRawTransaction timeout"),
            "{label}: expected the submission-phase timeout, got: {err}"
        );
        proxy.clear_fault("eth_sendRawTransaction");

        // The node accepted it regardless, so the next send must move past it.
        let receipt = provider
            .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
            .await?;
        assert!(receipt.status(), "{label}: recovery send should succeed");
    }

    // Four transactions were accepted in total: two whose acknowledgment the
    // client lost, and two recoveries. No nonce was reused, so all four mined.
    let mined = mined_nonces(&provider.inner(), env.signer_address()).await?;
    let got: Vec<u64> = mined.iter().map(|(n, _)| *n).collect();
    let expected: Vec<u64> = (start_nonce..start_nonce + 4).collect();
    assert_eq!(
        got, expected,
        "expected four consecutive nonces with no reuse and no gap"
    );
    Ok(())
}

/// Requirement 2d: a stale pending count. The node reports a nonce that is
/// already spent, so the submission collides. That must surface as a submission
/// error and re-seed, not wedge the sequence.
#[tokio::test]
#[serial(db)]
async fn stale_pending_count_recovers() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    // Spend a few nonces normally.
    for _ in 0..2 {
        provider
            .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
            .await?;
    }
    let after_warmup = provider.get_transaction_count(env.signer_address()).await?;

    // Force a cold lookup that answers with a stale (already spent) value.
    proxy.set_fault("eth_sendRawTransaction", Fault::Blackhole, Some(1));
    let _ = provider
        .send_transaction_sync(transfer(&env), Duration::from_secs(1), RECEIPT)
        .await;
    proxy.clear_fault("eth_sendRawTransaction");
    proxy.set_fault(
        "eth_getTransactionCount",
        Fault::RespondWith(serde_json::json!("0x0")),
        Some(1),
    );

    // This attempt may fail; what matters is that the sequence recovers.
    let _ = provider
        .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
        .await;
    proxy.clear_fault("eth_getTransactionCount");

    let mut recovered = false;
    for _ in 0..5 {
        if provider
            .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
            .await
            .is_ok()
        {
            recovered = true;
            break;
        }
    }
    assert!(
        recovered,
        "the sequence must recover after a stale pending count"
    );
    let finished = provider.get_transaction_count(env.signer_address()).await?;
    assert!(
        finished > after_warmup,
        "the signer must make forward progress ({after_warmup} -> {finished})"
    );
    Ok(())
}

/// Requirement 3: receipt uncertainty must be isolated from the sequence.
///
/// Four transactions are acknowledged while mining is paused. The first is given
/// a deadline it cannot meet, so its receipt is lost. The other three hold
/// nonces above it and must keep them; the sequence must not re-seed, because
/// those nonces are legitimately outstanding rather than free.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn lost_receipt_does_not_disturb_later_nonces() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    let control = ProviderBuilder::new()
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    pause_mining(&control).await?;
    let start_nonce = control.get_transaction_count(env.signer_address()).await?;

    // The first send gets a receipt deadline that cannot be met while mining is
    // paused; the rest are patient.
    let doomed_receipt = {
        let provider = provider.clone();
        let txn = transfer(&env);
        tokio::spawn(async move {
            provider
                .send_transaction_sync(txn, SUBMIT, Duration::from_secs(2))
                .await
        })
    };
    // Let it take the first nonce before the others queue.
    tokio::time::sleep(Duration::from_millis(300)).await;

    let mut rest = tokio::task::JoinSet::new();
    for _ in 0..3 {
        let provider = provider.clone();
        let txn = transfer(&env);
        rest.spawn(async move { provider.send_transaction_sync(txn, SUBMIT, RECEIPT).await });
    }

    // The first attempt gives up on its receipt.
    let err = doomed_receipt
        .await?
        .expect_err("the receipt deadline must fire")
        .to_string();
    assert!(
        err.contains("receipt timeout"),
        "expected a receipt-phase timeout, got: {err}"
    );

    // All four are nonetheless accepted and hold consecutive nonces.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while control
        .get_transaction_count(env.signer_address())
        .pending()
        .await?
        - start_nonce
        < 4
    {
        if tokio::time::Instant::now() >= deadline {
            anyhow::bail!("not all four transactions were accepted");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    control.anvil_mine(Some(1), None).await?;
    let mut ok = 0;
    while let Some(joined) = rest.join_next().await {
        if joined?.is_ok() {
            ok += 1;
        }
    }
    assert_eq!(
        ok, 3,
        "the later transactions must still get their receipts"
    );

    let mined = mined_nonces(&control, env.signer_address()).await?;
    let got: Vec<u64> = mined.iter().map(|(n, _)| *n).collect();
    let expected: Vec<u64> = (start_nonce..start_nonce + 4).collect();
    assert_eq!(
        got, expected,
        "the lost receipt must not have released or duplicated its nonce"
    );

    // The decisive assertion: a receipt timeout must not invalidate the
    // sequence. Re-seeding would have produced a second pending lookup.
    assert_eq!(
        proxy.tx_count_calls_with_tag("pending"),
        1,
        "a receipt timeout must not re-seed the shared nonce sequence"
    );
    Ok(())
}

/// A genuinely failed receipt (`status = false`) is an inclusion outcome, not a
/// sequence problem: the nonce is spent and the sequence must keep going.
///
/// The revert comes from a contract that actually reverts, and the test asserts
/// `!receipt.status()` before checking nonce advancement. An earlier version
/// sent `0xfe` to an empty account, which is not executed as bytecode and
/// therefore succeeded - it asserted nothing about failed receipts at all.
#[tokio::test]
#[serial(db)]
async fn failed_receipt_does_not_reset_the_sequence() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    let deployer = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    // `already_added_revert = true`: every addCiphertextMaterial reverts.
    let reverting_contract = CiphertextCommits::deploy(&deployer, true).await?;

    let start_nonce = provider.get_transaction_count(env.signer_address()).await?;

    // Explicit gas limit, so the revert happens on chain rather than during
    // estimation: that is what produces a mined receipt with status = false.
    let reverting = reverting_contract
        .addCiphertextMaterial(
            FixedBytes([7u8; 32]),
            U256::from(1),
            FixedBytes([8u8; 32]),
            FixedBytes([9u8; 32]),
        )
        .into_transaction_request()
        .with_gas_limit(200_000);
    let receipt = provider
        .send_transaction_sync(reverting, SUBMIT, RECEIPT)
        .await?;
    assert!(
        !receipt.status(),
        "the transaction must actually revert for this test to mean anything"
    );

    let follow_up = provider
        .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
        .await?;
    assert!(follow_up.status());

    // The contract deployment also came from this signer, so count only what
    // this test sent.
    let mined = mined_nonces(&provider.inner(), env.signer_address()).await?;
    let got: Vec<u64> = mined
        .iter()
        .map(|(n, _)| *n)
        .filter(|n| *n >= start_nonce)
        .collect();
    assert_eq!(
        got,
        vec![start_nonce, start_nonce + 1],
        "the sequence must advance across an inclusion-level failure"
    );
    assert_eq!(
        proxy.tx_count_calls_with_tag("pending"),
        1,
        "an included-but-failed transaction must not re-seed the sequence"
    );
    Ok(())
}

/// The receipt phase must stay bounded even when receipt RPCs stop answering
/// entirely - through the watcher *and* the final direct lookup. An unbounded
/// fallback would park the attempt, and its admission permit, until the RPC
/// recovered, reintroducing the blocking hole the nonce-lookup amendment closed.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn blackholed_receipt_lookups_stay_bounded_and_recover() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    let receipt_budget = Duration::from_secs(4);
    // Both the watcher's polling and the final lookup go through this method.
    proxy.set_fault("eth_getTransactionReceipt", Fault::Blackhole, None);

    let started = std::time::Instant::now();
    let err = provider
        .send_transaction_sync(transfer(&env), SUBMIT, receipt_budget)
        .await
        .expect_err("a blackholed receipt lookup must not resolve")
        .to_string();
    let elapsed = started.elapsed();

    assert!(
        err.contains("receipt"),
        "expected a receipt-phase error, got: {err}"
    );
    // The final lookup is carved out of receipt_timeout rather than added to it,
    // so the phase must not run to twice the budget despite two lookups.
    assert!(
        elapsed < receipt_budget * 2,
        "the receipt phase took {elapsed:?}, which is not bounded by the \
         configured {receipt_budget:?}"
    );

    // The transaction itself was accepted, and the sequence keeps going.
    proxy.clear_fault("eth_getTransactionReceipt");
    let receipt = provider
        .send_transaction_sync(transfer(&env), SUBMIT, RECEIPT)
        .await?;
    assert!(
        receipt.status(),
        "the sender must recover once receipts answer"
    );
    Ok(())
}

/// A receipt deadline must also retire the heartbeat watcher if the hash will
/// never mine. Keep the provider alive: dropping it would hide the retention.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn dropped_transaction_watcher_stops_polling_after_receipt_deadline() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);
    provider
        .inner()
        .client()
        .set_poll_interval(Duration::from_millis(100));
    let control = ProviderBuilder::new().connect_http(env.http_endpoint_url());
    pause_mining(&control).await?;

    let attempt = provider.send_transaction_sync(transfer(&env), SUBMIT, Duration::from_secs(2));
    let remove = async {
        let hash = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let block = control
                    .get_block_by_number(alloy::eips::BlockNumberOrTag::Pending)
                    .await
                    .unwrap()
                    .unwrap();
                if let Some(hash) = block.transactions.hashes().next() {
                    break hash;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await?;
        assert_eq!(control.anvil_drop_transaction(hash).await?, Some(hash));
        anyhow::Ok(())
    };
    let (outcome, removal) = tokio::join!(attempt, remove);
    removal?;
    assert!(outcome.unwrap_err().to_string().contains("receipt"));
    assert!(
        proxy.calls("eth_blockNumber") > 0,
        "must actually exercise heartbeat polling"
    );
    // Allow watcher reaping and the last already-started poll to settle.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let polls = proxy.calls("eth_blockNumber");
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert_eq!(
        proxy.calls("eth_blockNumber"),
        polls,
        "an unmined hash must not keep the heartbeat polling indefinitely"
    );
    Ok(())
}

/// A transport failure during receipt observation must keep its type, so the
/// existing `is_backend_gone` classification still fires. Wrapping it in a
/// formatted string silently disabled the stop-and-restart path for the whole
/// receipt stage.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn backend_gone_during_receipt_watch_is_preserved() -> anyhow::Result<()> {
    let mut env = TestEnvironment::new(SignerType::PrivateKey).await?;

    // WebSocket, so dropping anvil produces a real BackendGone rather than an
    // HTTP connection error.
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.signer_address()),
    );

    let control = ProviderBuilder::new()
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    pause_mining(&control).await?;

    // Accepted, then left waiting for inclusion that will never come.
    let waiting = {
        let provider = provider.clone();
        let txn = transfer(&env);
        tokio::spawn(async move {
            provider
                .send_transaction_sync(txn, SUBMIT, Duration::from_secs(120))
                .await
        })
    };
    tokio::time::sleep(Duration::from_secs(2)).await;

    env.drop_anvil();

    let err = tokio::time::timeout(Duration::from_secs(60), waiting)
        .await
        .map_err(|_| {
            anyhow::anyhow!("the receipt wait did not return after the backend went away")
        })??
        .expect_err("the receipt wait must fail once the backend is gone");

    let as_anyhow = anyhow::Error::new(err);
    assert!(
        transaction_sender::is_backend_gone(&as_anyhow),
        "a receipt-stage backend failure must remain classifiable as \
         BackendGone, got: {as_anyhow}"
    );
    Ok(())
}

/// Acceptance delayed *before* the node sees it - the schedule the other
/// uncertainty tests miss, because they hold the response after forwarding.
///
/// The original request is held in the proxy, so the client times out while the
/// node has never seen it. The re-seed from `pending` cannot know about it and
/// hands out the same nonce, so two *different* logical operations end up
/// contending for it. One must lose.
///
/// The assertion is about work accounting, not chain invariants: nonce
/// uniqueness and contiguity are enforced by the chain regardless of what this
/// code does. What matters is that the losing operation stays retryable and
/// eventually completes, and that neither operation is applied twice. The two
/// operations are distinguishable by the value they transfer.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn delayed_acceptance_loses_no_logical_work() -> anyhow::Result<()> {
    // Distinct amounts, so each logical operation is identifiable on chain.
    const OP_A: u64 = 111;
    const OP_B: u64 = 222;

    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = proxied(&env, &proxy);

    let op = |value: u64| {
        TransactionRequest::default()
            .with_to(env.user_address)
            .with_value(U256::from(value))
            .with_gas_limit(21_000)
    };

    // A is held before forwarding: the node has not seen it when A gives up.
    proxy.set_fault("eth_sendRawTransaction", Fault::Delay(SUBMIT * 3), Some(1));
    let a_first = provider
        .send_transaction_sync(op(OP_A), SUBMIT, RECEIPT)
        .await;
    assert!(
        a_first.is_err(),
        "A must time out while its request is still held"
    );

    // B now takes the nonce A's re-seed cannot know is spoken for.
    let b_first = provider
        .send_transaction_sync(op(OP_B), SUBMIT, RECEIPT)
        .await;

    // Let the held original land, whatever the node makes of it.
    tokio::time::sleep(SUBMIT * 3).await;

    // Both operations must be able to finish. Retry each until it does, exactly
    // as the operation layer would with a row that has not reached a terminal
    // state.
    for (label, value, already_done) in [("A", OP_A, false), ("B", OP_B, b_first.is_ok())] {
        if already_done {
            continue;
        }
        let mut done = false;
        for _ in 0..8 {
            if provider
                .send_transaction_sync(op(value), SUBMIT, RECEIPT)
                .await
                .is_ok()
            {
                done = true;
                break;
            }
        }
        assert!(
            done,
            "operation {label} lost the nonce race and never completed; a losing \
             operation must stay retryable"
        );
    }

    // Work accounting: each logical operation applied exactly once. A duplicate
    // would mean the held original *and* a retry both landed.
    let head = provider.inner().get_block_number().await?;
    let mut applied_a = 0usize;
    let mut applied_b = 0usize;
    for number in 0..=head {
        let Some(block) = provider
            .inner()
            .get_block_by_number(number.into())
            .full()
            .await?
        else {
            continue;
        };
        for txn in block.transactions.txns() {
            if txn.inner.signer() != env.signer_address() {
                continue;
            }
            let v = txn.inner.value();
            if v == U256::from(OP_A) {
                applied_a += 1;
            } else if v == U256::from(OP_B) {
                applied_b += 1;
            }
        }
    }
    assert_eq!(
        (applied_a, applied_b),
        (1, 1),
        "each logical operation must be applied exactly once, saw A={applied_a} B={applied_b}"
    );
    eprintln!(
        "delayed acceptance: A retried={}, B first attempt {}",
        a_first.is_err(),
        if b_first.is_ok() { "won" } else { "lost" }
    );
    Ok(())
}

/// The admission semaphore bounds *active attempts*, not outstanding on-chain
/// transactions. Once a receipt deadline fires the permit is released while the
/// transaction may still be pending at the node, so outstanding work can exceed
/// the admission limit.
///
/// Routed through `send_sync_with_overprovision`, which is where the permit is
/// actually acquired - calling `send_transaction_sync` directly bypasses
/// admission control entirely and would pin nothing.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn outstanding_transactions_can_exceed_the_admission_limit() -> anyhow::Result<()> {
    const LIMIT: usize = 4;
    const WAVES: usize = 3;
    const SHORT_RECEIPT: Duration = Duration::from_secs(2);

    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_http(proxy.url()),
        Some(env.signer_address()),
    )
    .with_max_inflight(LIMIT);

    let control = ProviderBuilder::new()
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    pause_mining(&control).await?;
    let start_nonce = control.get_transaction_count(env.signer_address()).await?;

    // Each wave fills every permit, then abandons its receipts while mining is
    // paused. The permits are released; the transactions stay pending.
    for wave_index in 0..WAVES {
        let mut wave = tokio::task::JoinSet::new();
        for _ in 0..LIMIT {
            let provider = provider.clone();
            let txn = transfer(&env);
            wave.spawn(async move {
                // Gas is pre-set, so no estimation RPC; percent 100 keeps it as is.
                provider
                    .send_sync_with_overprovision(
                        txn,
                        100,
                        SUBMIT,
                        Duration::from_secs(20),
                        SHORT_RECEIPT,
                    )
                    .await
            });
        }
        let mut timed_out = 0;
        while let Some(joined) = wave.join_next().await {
            match joined? {
                Err(e) if e.to_string().contains("receipt") => timed_out += 1,
                Err(e) => anyhow::bail!("wave {wave_index}: unexpected failure: {e}"),
                Ok(_) => {
                    anyhow::bail!("wave {wave_index}: a receipt arrived while mining is paused")
                }
            }
        }
        assert_eq!(
            timed_out, LIMIT,
            "wave {wave_index}: every attempt should abandon its receipt"
        );
    }

    let outstanding = control
        .get_transaction_count(env.signer_address())
        .pending()
        .await?
        - start_nonce;
    assert_eq!(
        outstanding,
        (LIMIT * WAVES) as u64,
        "all {} submissions should still be pending",
        LIMIT * WAVES
    );
    assert!(
        outstanding > LIMIT as u64,
        "outstanding work ({outstanding}) must be able to exceed the admission \
         limit ({LIMIT}) once receipt deadlines release permits"
    );

    // They are all still valid and mine together when the chain resumes.
    control.anvil_mine(Some(1), None).await?;
    let mined = mined_nonces(&control, env.signer_address()).await?;
    let got: Vec<u64> = mined.iter().map(|(n, _)| *n).collect();
    let expected: Vec<u64> = (start_nonce..start_nonce + outstanding).collect();
    assert_eq!(
        got, expected,
        "abandoned receipts must not corrupt the sequence"
    );
    Ok(())
}
