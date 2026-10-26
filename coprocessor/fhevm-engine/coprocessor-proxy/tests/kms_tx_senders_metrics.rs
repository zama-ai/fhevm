//! Separate process so other followers cannot overwrite the gauges under test.

use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    rpc::types::Block,
    sol_types::SolCall,
    transports::mock::Asserter,
};
use coprocessor_proxy::kms_tx_senders::Follower;
use fhevm_host_bindings::i_protocol_config_base::IProtocolConfigBase::getCurrentKmsContextIdCounterCall;
use fhevm_sig_auth::RefreshableSigners;
use std::time::Duration;

fn block(number: u64, timestamp: u64) -> Block {
    let mut block: Block = Block::default();
    block.header.inner.number = number;
    block.header.inner.timestamp = timestamp;
    block
}

fn timestamp() -> i64 {
    prometheus::gather()
        .iter()
        .find(|metric| metric.name() == "coprocessor_proxy_kms_tx_senders_block_timestamp_seconds")
        .expect("timestamp series exists before the first successful read")
        .get_metric()[0]
        .get_gauge()
        .value() as i64
}

#[tokio::test(start_paused = true)]
async fn freshness_tracks_only_successfully_applied_finalized_blocks() {
    let asserter = Asserter::new();
    let no_context: alloy::primitives::Bytes =
        getCurrentKmsContextIdCounterCall::abi_encode_returns(&U256::ZERO).into();
    asserter.push_failure_msg("initial RPC failure");
    asserter.push_success(&block(100, 1_000));
    asserter.push_success(&no_context);
    // Even an inconsistent timestamp on the same height must not refresh the signal.
    asserter.push_success(&block(100, 2_000));
    asserter.push_success(&block(99, 990));
    asserter.push_success(&block(101, 1_012));
    asserter.push_success(&no_context);
    asserter.push_success(&block(102, 1_024));
    asserter.push_failure_msg("contract read failure");
    asserter.push_success(&block(102, 1_024));
    asserter.push_success(&no_context);
    asserter.push_failure_msg("RPC failure after loading");

    let interval = Duration::from_millis(20);
    let follower = Follower::new(
        ProviderBuilder::new().connect_mocked_client(asserter.clone()),
        Address::ZERO,
        RefreshableSigners::default(),
        interval,
    );
    follower
        .run(async {
            tokio::time::sleep(Duration::from_millis(1)).await;
            for (poll, expected) in [0, 1_000, 1_000, 1_000, 1_012, 1_012, 1_024, 1_024]
                .into_iter()
                .enumerate()
            {
                if poll > 0 {
                    tokio::time::sleep(interval).await;
                }
                assert_eq!(timestamp(), expected);
            }
        })
        .await;
    assert!(asserter.read_q().is_empty());
}
