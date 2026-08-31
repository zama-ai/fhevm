//! Proves the per-bucket `HEAD` ceiling in [`ciphertext_attestation::BoundedClient`]
//! actually binds: with several Coprocessor entries pointed at the same bucket, no more than the
//! configured cap of `HEAD` requests to that bucket are ever in flight at once.
//!
//! This is what stood between kms-connector and adopting this crate's client in place of its own
//! duplicate (see `shared/ciphertext-attestation/src/client/s3.rs`), so it is worth a test that
//! keeps the bound honest rather than merely documented.

// Exercises the `client` feature only — compiled out entirely when it's off, rather than failing
// to resolve `alloy`/`ciphertext_attestation::{BoundedClient, ...}`.
#![cfg(feature = "client")]

use alloy::{
    primitives::{Address, B256, U256},
    transports::http::Client,
};
use ciphertext_attestation::{
    BoundedClient, ConsensusCheckError, CoprocessorEntry, CoprocessorRegistrySnapshot,
    fetch_attestations_and_check_consensus,
};
use std::{
    num::NonZeroUsize,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
};

const HANDLE: B256 = B256::repeat_byte(0xAA);
const CONTEXT_ID: U256 = U256::ONE;

/// `CAP`-many permits shared by `ENTRIES` Coprocessor entries, all pointed at the same bucket, so
/// their `HEAD`s all race for the same ceiling. `ENTRIES` is larger than `CAP` so the ceiling
/// actually has to bind rather than happening to hold by coincidence.
const CAP: usize = 2;
const ENTRIES: u8 = 6;

/// A raw TCP bucket that answers every `HEAD` with 404 after `hold`, tracking the peak number of
/// requests it ever had open at once.
///
/// 404 (rather than a real attestation) means the round can never reach consensus, which matters
/// here: with a `threshold` of 1 (see the test below), the consensus tracker only reaches
/// `MissedThisRound` once every slot is filled, so every entry's `HEAD` actually gets issued instead
/// of the round exiting early and aborting whichever ones have not started yet.
async fn concurrency_tracking_bucket(hold: Duration) -> (String, Arc<AtomicUsize>, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let peak = Arc::new(AtomicUsize::new(0));
    let in_flight = Arc::new(AtomicUsize::new(0));

    let (peak_handle, in_flight_handle) = (Arc::clone(&peak), Arc::clone(&in_flight));
    let accept_loop = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let (peak, in_flight) = (Arc::clone(&peak_handle), Arc::clone(&in_flight_handle));
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf).await;

                let now = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                tokio::time::sleep(hold).await;
                in_flight.fetch_sub(1, Ordering::SeqCst);

                let _ = stream
                    .write_all(
                        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    )
                    .await;
                let _ = stream.shutdown().await;
            });
        }
    });

    (url, peak, accept_loop)
}

#[tokio::test]
async fn per_bucket_head_ceiling_is_never_exceeded() {
    let (bucket, peak, accept_loop) = concurrency_tracking_bucket(Duration::from_millis(50)).await;

    let coprocessors = (0..ENTRIES)
        .map(|i| CoprocessorEntry {
            tx_sender: Address::repeat_byte(0x70 + i),
            signer: Address::repeat_byte(0x80 + i),
            bucket: bucket.clone(),
        })
        .collect();
    // Threshold 1: every entry fails (404), so the round only turns `MissedThisRound` once every
    // entry has been accounted for. A higher threshold could make it unwinnable — and therefore
    // terminal — after the very first failure, aborting the rest before they even start.
    let registry = CoprocessorRegistrySnapshot::new(coprocessors, NonZeroUsize::new(1).unwrap());

    let client = BoundedClient::for_attestations_only(
        Client::new(),
        NonZeroUsize::new(CAP).unwrap(),
        Duration::from_secs(5),
        CONTEXT_ID,
    );
    let err = fetch_attestations_and_check_consensus(&client, HANDLE, &registry)
        .await
        .expect_err("every bucket answers 404, so consensus is never reached");
    assert!(matches!(
        err,
        ConsensusCheckError::MissedThisRound(ref round) if round.attested().is_empty()
    ));

    let observed = peak.load(Ordering::SeqCst);
    assert!(
        observed <= CAP,
        "observed {observed} concurrent HEADs to one bucket, expected at most {CAP}"
    );
    assert_eq!(
        observed, CAP,
        "expected the ceiling to actually bind with {ENTRIES} entries racing for {CAP} permits"
    );

    accept_loop.abort();
}
