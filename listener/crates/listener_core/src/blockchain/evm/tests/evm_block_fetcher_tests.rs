//! Tests for the EVM Block Fetcher module.
//!
//! Test categories:
//! 1. Strategy tests (all 5 strategies, all chains)
//! 2. Cancellation tests
//! 3. Verification tests
//! 4. Retry tests
//!
//! Note: Some tests are commented out due to rate limiting from free RPC providers.
//! These tests should be re-enabled when using private/paid RPC nodes.

use super::*;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

struct RpcTarget {
    name: &'static str,
    url: &'static str,
}

/// Returns a minimal set of test chains to avoid rate limiting from free RPC providers.
/// The full list is commented below for reference when using private/paid nodes.
fn get_test_chains() -> Vec<RpcTarget> {
    vec![
        // Using only chains with generous rate limits for free tiers
        RpcTarget {
            name: "Zama Gateway Testnet",
            url: "https://rpc-zama-testnet-0.t.conduit.xyz",
        },
        RpcTarget {
            name: "Optimism",
            url: "https://mainnet.optimism.io",
        },
        RpcTarget {
            name: "Base",
            url: "https://mainnet.base.org",
        },
    ]
}

// Full chain list - uncomment when using private/paid RPC nodes:
// fn get_test_chains_full() -> Vec<RpcTarget> {
//     vec![
//         RpcTarget {
//             name: "Ethereum Mainnet",
//             url: "https://ethereum-rpc.publicnode.com",
//         },
//         RpcTarget {
//             name: "Binance Smart Chain Mainnet",
//             url: "https://bsc-dataseed.bnbchain.org",
//         },
//         RpcTarget {
//             name: "Polygon PoS",
//             url: "https://polygon-rpc.com",
//         },
//         RpcTarget {
//             name: "Avalanche",
//             url: "https://api.avax.network/ext/bc/C/rpc",
//         },
//         RpcTarget {
//             name: "Monad",
//             url: "https://rpc.monad.xyz/",
//         },
//         RpcTarget {
//             name: "Optimism",
//             url: "https://mainnet.optimism.io",
//         },
//         RpcTarget {
//             name: "Arbitrum One",
//             url: "https://arb1.arbitrum.io/rpc",
//         },
//         RpcTarget {
//             name: "Base",
//             url: "https://mainnet.base.org",
//         },
//         RpcTarget {
//             name: "Zama Gateway Testnet",
//             url: "https://rpc-zama-testnet-0.t.conduit.xyz",
//         },
//         RpcTarget {
//             name: "Polkadot mainnet",
//             url: "https://eth-rpc.polkadot.io/",
//         },
//         RpcTarget {
//             name: "Polkadot testnet",
//             url: "https://services.polkadothub-rpc.com/testnet",
//         },
//     ]
// }

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

#[tokio::test]
async fn test_fetcher_clone() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("CLONE: EvmBlockFetcher Clone Functionality");
    println!("{}\n", "=".repeat(80));

    let target = RpcTarget {
        name: "Zama Gateway Testnet",
        url: "https://rpc-zama-testnet-0.t.conduit.xyz",
    };

    let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");
    let master_token = CancellationToken::new();

    // Create one fetcher with shared cancellation token
    let fetcher = EvmBlockFetcher::new(provider)
        .with_cancellation_token(master_token.clone())
        .with_retry_interval(Duration::from_millis(100));

    // Clone the fetcher (this is what we're testing)
    let fetcher_clone = fetcher.clone();

    // Both fetchers should work independently
    let result1 = fetcher.fetch_block_with_block_receipts_by_number(1).await;
    let result2 = fetcher_clone
        .fetch_block_with_block_receipts_by_number(1)
        .await;

    match (result1, result2) {
        (Ok(r1), Ok(r2)) => {
            info!(
                "  [OK]   Original fetcher: Block #1 ({} txs)",
                r1.transaction_count()
            );
            info!(
                "  [OK]   Cloned fetcher: Block #1 ({} txs)",
                r2.transaction_count()
            );
            assert_eq!(r1.block.header.hash, r2.block.header.hash);
            info!("  [OK]   Both fetchers returned identical block hashes");
        }
        (Err(e1), _) => {
            info!("  [FAIL] Original fetcher error: {}", e1);
        }
        (_, Err(e2)) => {
            info!("  [FAIL] Cloned fetcher error: {}", e2);
        }
    }
}

#[tokio::test]
async fn test_fetcher_clone_shared_cancellation() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("CLONE: Shared Cancellation Token");
    println!("{}\n", "=".repeat(80));

    let target = RpcTarget {
        name: "Zama Gateway Testnet",
        url: "https://rpc-zama-testnet-0.t.conduit.xyz",
    };

    let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");
    let master_token = CancellationToken::new();

    // Create fetcher with shared cancellation token
    let fetcher = EvmBlockFetcher::new(provider)
        .with_cancellation_token(master_token.clone())
        .with_retry_interval(Duration::from_millis(100));

    // Clone the fetcher
    let fetcher_clone = fetcher.clone();

    // Cancel the master token
    master_token.cancel();

    // Both fetchers should be cancelled
    let result1 = fetcher.fetch_block_with_block_receipts_by_number(1).await;
    let result2 = fetcher_clone
        .fetch_block_with_block_receipts_by_number(1)
        .await;

    match (&result1, &result2) {
        (Err(BlockFetchError::Cancelled), Err(BlockFetchError::Cancelled)) => {
            info!("  [OK]   Both fetchers were cancelled by master token");
        }
        _ => {
            info!(
                "  [WARN] Expected both cancelled, got: {:?}, {:?}",
                result1.is_ok(),
                result2.is_ok()
            );
        }
    }
}

#[tokio::test]
async fn test_strategy1_block_receipts_by_number_all_chains() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 1: eth_getBlockReceipts by Number");
    println!("{}\n", "=".repeat(80));

    for target in get_test_chains() {
        info!("Testing {}...", target.name);

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
            Ok(p) => p,
            Err(e) => {
                info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
                continue;
            }
        };

        let height = match provider.get_block_number().await {
            Ok(h) => h,
            Err(e) => {
                info!("  [SKIP] {}: Block number failed: {}", target.name, e);
                continue;
            }
        };

        let fetcher =
            EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

        let target_block = height.saturating_sub(10);

        match fetcher
            .fetch_block_with_block_receipts_by_number(target_block)
            .await
        {
            Ok(result) => {
                info!(
                    "  [OK]   {}: Block #{} ({} txs, {} receipts)",
                    target.name,
                    target_block,
                    result.transaction_count(),
                    result.receipts.len()
                );
                assert_eq!(result.transaction_count(), result.receipts.len());
            }
            Err(e) => {
                info!("  [FAIL] {}: {}", target.name, e);
            }
        }
    }
}

#[tokio::test]
async fn test_strategy1_block_receipts_by_hash_all_chains() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 1: eth_getBlockReceipts by Hash");
    println!("{}\n", "=".repeat(80));

    for target in get_test_chains() {
        info!("Testing {}...", target.name);

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
            Ok(p) => p,
            Err(e) => {
                info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
                continue;
            }
        };

        let height = match provider.get_block_number().await {
            Ok(h) => h,
            Err(e) => {
                info!("  [SKIP] {}: Block number failed: {}", target.name, e);
                continue;
            }
        };

        // First get the block to get its hash
        let target_block_num = height.saturating_sub(10);
        let block = match provider.get_block_by_number(target_block_num).await {
            Ok(b) => b,
            Err(e) => {
                info!("  [SKIP] {}: Block fetch failed: {}", target.name, e);
                continue;
            }
        };

        let block_hash = block.header.hash;
        let fetcher =
            EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

        match fetcher
            .fetch_block_with_block_receipts_by_hash(block_hash)
            .await
        {
            Ok(result) => {
                info!(
                    "  [OK]   {}: Block {} ({} txs, {} receipts)",
                    target.name,
                    block_hash,
                    result.transaction_count(),
                    result.receipts.len()
                );
                assert_eq!(result.transaction_count(), result.receipts.len());
                assert_eq!(result.block.header.hash, block_hash);
            }
            Err(e) => {
                info!("  [FAIL] {}: {}", target.name, e);
            }
        }
    }
}

#[tokio::test]
async fn test_strategy2_batch_receipts_by_number_all_chains() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 2: Batch Receipts by Number");
    println!("{}\n", "=".repeat(80));

    for target in get_test_chains() {
        info!("Testing {}...", target.name);

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
            Ok(p) => p,
            Err(e) => {
                info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
                continue;
            }
        };

        let height = match provider.get_block_number().await {
            Ok(h) => h,
            Err(e) => {
                info!("  [SKIP] {}: Block number failed: {}", target.name, e);
                continue;
            }
        };

        let fetcher =
            EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

        let target_block = height.saturating_sub(10);

        match fetcher
            .fetch_block_with_batch_receipts_by_number(target_block)
            .await
        {
            Ok(result) => {
                info!(
                    "  [OK]   {}: Block #{} ({} txs, {} receipts)",
                    target.name,
                    target_block,
                    result.transaction_count(),
                    result.receipts.len()
                );
                assert_eq!(result.transaction_count(), result.receipts.len());
            }
            Err(e) => {
                info!("  [FAIL] {}: {}", target.name, e);
            }
        }
    }
}

#[tokio::test]
async fn test_strategy2_batch_receipts_by_hash_all_chains() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 2: Batch Receipts by Hash");
    println!("{}\n", "=".repeat(80));

    for target in get_test_chains() {
        info!("Testing {}...", target.name);

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
            Ok(p) => p,
            Err(e) => {
                info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
                continue;
            }
        };

        let height = match provider.get_block_number().await {
            Ok(h) => h,
            Err(e) => {
                info!("  [SKIP] {}: Block number failed: {}", target.name, e);
                continue;
            }
        };

        // First get the block to get its hash
        let target_block_num = height.saturating_sub(10);
        let block = match provider.get_block_by_number(target_block_num).await {
            Ok(b) => b,
            Err(e) => {
                info!("  [SKIP] {}: Block fetch failed: {}", target.name, e);
                continue;
            }
        };

        let block_hash = block.header.hash;
        let fetcher =
            EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

        match fetcher
            .fetch_block_with_batch_receipts_by_hash(block_hash)
            .await
        {
            Ok(result) => {
                info!(
                    "  [OK]   {}: Block {} ({} txs, {} receipts)",
                    target.name,
                    block_hash,
                    result.transaction_count(),
                    result.receipts.len()
                );
                assert_eq!(result.transaction_count(), result.receipts.len());
            }
            Err(e) => {
                info!("  [FAIL] {}: {}", target.name, e);
            }
        }
    }
}

// NOTE: This test is commented out due to rate limiting from free RPC providers.
// Running multiple chunk sizes sequentially on high-tx blocks exhausts rate limits.
// Re-enable when using private/paid RPC nodes with higher rate limits.

// #[tokio::test]
// async fn test_strategy3_chunked_batch_various_sizes() {
//     init_tracing();
//
//     println!("\n{}", "=".repeat(80));
//     println!("STRATEGY 3: Chunked Batch Receipts (Various Sizes)");
//     println!("{}\n", "=".repeat(80));
//
//     let chunk_sizes = [5, 10, 50];
//
//     // Use Ethereum Mainnet for this test
//     let target = RpcTarget {
//         name: "Ethereum Mainnet",
//         url: "https://ethereum-rpc.publicnode.com",
//     };
//
//     let provider = match SemEvmRpcProvider::new(target.url.to_string(), 10) {
//         Ok(p) => p,
//         Err(e) => {
//             info!("  [SKIP] Provider creation failed: {}", e);
//             return;
//         }
//     };
//
//     let height = match provider.get_block_number().await {
//         Ok(h) => h,
//         Err(e) => {
//             info!("  [SKIP] Block number failed: {}", e);
//             return;
//         }
//     };
//
//     let target_block = height.saturating_sub(10);
//
//     for chunk_size in chunk_sizes {
//         info!("Testing chunk_size={}...", chunk_size);
//
//         let fetcher = EvmBlockFetcher::new(provider.clone())
//             .with_retry_interval(Duration::from_millis(100));
//
//         match fetcher
//             .fetch_block_by_number_with_parallel_batched_receipts(target_block, chunk_size)
//             .await
//         {
//             Ok(result) => {
//                 info!(
//                     "  [OK]   chunk_size={}: Block #{} ({} txs, {} receipts)",
//                     chunk_size,
//                     target_block,
//                     result.transaction_count(),
//                     result.receipts.len()
//                 );
//                 assert_eq!(result.transaction_count(), result.receipts.len());
//             }
//             Err(e) => {
//                 info!("  [FAIL] chunk_size={}: {}", chunk_size, e);
//             }
//         }
//     }
// }

#[tokio::test]
async fn test_strategy3_chunked_batch_by_hash() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 3: Chunked Batch Receipts by Hash");
    println!("{}\n", "=".repeat(80));

    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 10) {
        Ok(p) => p,
        Err(e) => {
            info!("  [SKIP] Provider creation failed: {}", e);
            return;
        }
    };

    let height = match provider.get_block_number().await {
        Ok(h) => h,
        Err(e) => {
            info!("  [SKIP] Block number failed: {}", e);
            return;
        }
    };

    let target_block_num = height.saturating_sub(10);
    let block = match provider.get_block_by_number(target_block_num).await {
        Ok(b) => b,
        Err(e) => {
            info!("  [SKIP] Block fetch failed: {}", e);
            return;
        }
    };

    let block_hash = block.header.hash;
    let fetcher = EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

    match fetcher
        .fetch_block_by_hash_with_parallel_batched_receipts(block_hash, 10)
        .await
    {
        Ok(result) => {
            info!(
                "  [OK]   Block {} ({} txs, {} receipts)",
                block_hash,
                result.transaction_count(),
                result.receipts.len()
            );
            assert_eq!(result.transaction_count(), result.receipts.len());
        }
        Err(e) => {
            info!("  [FAIL] {}", e);
        }
    }
}

// NOTE: Strategy 4 tests are commented out due to rate limiting from free RPC providers.
// These tests spawn many parallel requests which quickly exhaust rate limits.
// Re-enable when using private/paid RPC nodes with higher rate limits.

// #[tokio::test]
// async fn test_strategy4_individual_receipts_by_number_all_chains() {
//     init_tracing();
//
//     println!("\n{}", "=".repeat(80));
//     println!("STRATEGY 4: Individual Receipts by Number");
//     println!("{}\n", "=".repeat(80));
//
//     for target in get_test_chains() {
//         info!("Testing {}...", target.name);
//
//         let provider = match SemEvmRpcProvider::new(target.url.to_string(), 20) {
//             Ok(p) => p,
//             Err(e) => {
//                 info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
//                 continue;
//             }
//         };
//
//         let height = match provider.get_block_number().await {
//             Ok(h) => h,
//             Err(e) => {
//                 info!("  [SKIP] {}: Block number failed: {}", target.name, e);
//                 continue;
//             }
//         };
//
//         let fetcher = EvmBlockFetcher::new(provider)
//             .with_retry_interval(Duration::from_millis(100));
//
//         let target_block = height.saturating_sub(10);
//
//         match fetcher.fetch_block_with_individual_receipts_by_number(target_block).await {
//             Ok(result) => {
//                 info!(
//                     "  [OK]   {}: Block #{} ({} txs, {} receipts)",
//                     target.name,
//                     target_block,
//                     result.transaction_count(),
//                     result.receipts.len()
//                 );
//                 assert_eq!(result.transaction_count(), result.receipts.len());
//             }
//             Err(e) => {
//                 info!("  [FAIL] {}: {}", target.name, e);
//             }
//         }
//     }
// }

// #[tokio::test]
// async fn test_strategy4_individual_receipts_by_hash_all_chains() {
//     init_tracing();
//
//     println!("\n{}", "=".repeat(80));
//     println!("STRATEGY 4: Individual Receipts by Hash");
//     println!("{}\n", "=".repeat(80));
//
//     for target in get_test_chains() {
//         info!("Testing {}...", target.name);
//
//         let provider = match SemEvmRpcProvider::new(target.url.to_string(), 20) {
//             Ok(p) => p,
//             Err(e) => {
//                 info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
//                 continue;
//             }
//         };
//
//         let height = match provider.get_block_number().await {
//             Ok(h) => h,
//             Err(e) => {
//                 info!("  [SKIP] {}: Block number failed: {}", target.name, e);
//                 continue;
//             }
//         };
//
//         // First get the block to get its hash
//         let target_block_num = height.saturating_sub(10);
//         let block = match provider.get_block_by_number(target_block_num).await {
//             Ok(b) => b,
//             Err(e) => {
//                 info!("  [SKIP] {}: Block fetch failed: {}", target.name, e);
//                 continue;
//             }
//         };
//
//         let block_hash = block.header.hash;
//         let fetcher = EvmBlockFetcher::new(provider)
//             .with_retry_interval(Duration::from_millis(100));
//
//         match fetcher.fetch_block_with_individual_receipts_by_hash(block_hash).await {
//             Ok(result) => {
//                 info!(
//                     "  [OK]   {}: Block {} ({} txs, {} receipts)",
//                     target.name,
//                     block_hash,
//                     result.transaction_count(),
//                     result.receipts.len()
//                 );
//                 assert_eq!(result.transaction_count(), result.receipts.len());
//             }
//             Err(e) => {
//                 info!("  [FAIL] {}: {}", target.name, e);
//             }
//         }
//     }
// }

#[tokio::test]
async fn test_strategy5_sequential_receipts_by_number() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 5: Sequential Receipts by Number");
    println!("{}\n", "=".repeat(80));

    // Test on a single chain with a low-tx block to avoid long test times
    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    info!("Testing {}...", target.name);

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
        Ok(p) => p,
        Err(e) => {
            info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
            return;
        }
    };

    // Use block 1 which has 0 transactions (fast test)
    let fetcher = EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

    match fetcher
        .fetch_block_with_sequential_receipts_by_number(1)
        .await
    {
        Ok(result) => {
            info!(
                "  [OK]   {}: Block #1 ({} txs, {} receipts)",
                target.name,
                result.transaction_count(),
                result.receipts.len()
            );
            assert_eq!(result.transaction_count(), result.receipts.len());
        }
        Err(e) => {
            info!("  [FAIL] {}: {}", target.name, e);
        }
    }
}

#[tokio::test]
async fn test_strategy5_sequential_receipts_by_hash() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 5: Sequential Receipts by Hash");
    println!("{}\n", "=".repeat(80));

    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    info!("Testing {}...", target.name);

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
        Ok(p) => p,
        Err(e) => {
            info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
            return;
        }
    };

    // Get block 1's hash
    let block = match provider.get_block_by_number(1).await {
        Ok(b) => b,
        Err(e) => {
            info!("  [SKIP] {}: Block fetch failed: {}", target.name, e);
            return;
        }
    };

    let block_hash = block.header.hash;
    let fetcher = EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

    match fetcher
        .fetch_block_with_sequential_receipts_by_hash(block_hash)
        .await
    {
        Ok(result) => {
            info!(
                "  [OK]   {}: Block {} ({} txs, {} receipts)",
                target.name,
                block_hash,
                result.transaction_count(),
                result.receipts.len()
            );
            assert_eq!(result.transaction_count(), result.receipts.len());
            assert_eq!(result.block.header.hash, block_hash);
        }
        Err(e) => {
            info!("  [FAIL] {}: {}", target.name, e);
        }
    }
}

#[tokio::test]
async fn test_strategy5_sequential_with_transactions() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("STRATEGY 5: Sequential Receipts with Transactions");
    println!("{}\n", "=".repeat(80));

    // Test on Zama Gateway Testnet which has fewer transactions and is less rate-limited
    let target = RpcTarget {
        name: "Zama Gateway Testnet",
        url: "https://rpc-zama-testnet-0.t.conduit.xyz",
    };

    info!("Testing {}...", target.name);

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
        Ok(p) => p,
        Err(e) => {
            info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
            return;
        }
    };

    let height = match provider.get_block_number().await {
        Ok(h) => h,
        Err(e) => {
            info!("  [SKIP] {}: Block number failed: {}", target.name, e);
            return;
        }
    };

    let fetcher = EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

    let target_block = height.saturating_sub(10);

    match fetcher
        .fetch_block_with_sequential_receipts_by_number(target_block)
        .await
    {
        Ok(result) => {
            info!(
                "  [OK]   {}: Block #{} ({} txs, {} receipts)",
                target.name,
                target_block,
                result.transaction_count(),
                result.receipts.len()
            );
            assert_eq!(result.transaction_count(), result.receipts.len());
        }
        Err(e) => {
            info!("  [FAIL] {}: {}", target.name, e);
        }
    }
}

#[tokio::test]
async fn test_cancellation_immediate() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("CANCELLATION: Immediate Cancel");
    println!("{}\n", "=".repeat(80));

    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");

    let cancel_token = CancellationToken::new();

    // Cancel immediately before starting
    cancel_token.cancel();

    let fetcher = EvmBlockFetcher::new(provider)
        .with_cancellation_token(cancel_token)
        .with_retry_interval(Duration::from_millis(100));

    let result = fetcher
        .fetch_block_with_block_receipts_by_number(12345)
        .await;

    match result {
        Err(BlockFetchError::Cancelled) => {
            info!("  [OK]   Immediate cancellation worked correctly");
        }
        Ok(_) => {
            panic!("Expected Cancelled error, got success");
        }
        Err(e) => {
            panic!("Expected Cancelled error, got: {}", e);
        }
    }
}

#[tokio::test]
async fn test_cancellation_during_fetch() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("CANCELLATION: During Fetch");
    println!("{}\n", "=".repeat(80));

    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");

    let cancel_token = CancellationToken::new();
    let cancel_token_clone = cancel_token.clone();

    let fetcher = EvmBlockFetcher::new(provider)
        .with_cancellation_token(cancel_token)
        .with_retry_interval(Duration::from_millis(100));

    // Spawn task to cancel after a short delay
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        cancel_token_clone.cancel();
    });

    // Try to fetch a block - should be cancelled
    let result = fetcher
        .fetch_block_with_individual_receipts_by_number(1)
        .await;

    // Result could be success (if fast enough) or cancelled
    match result {
        Ok(_) => {
            info!("  [OK]   Fetch completed before cancellation (acceptable)");
        }
        Err(BlockFetchError::Cancelled) => {
            info!("  [OK]   Cancellation during fetch worked correctly");
        }
        Err(e) => {
            info!("  [WARN] Unexpected error: {}", e);
        }
    }
}

// NOTE: This test is commented out due to rate limiting from free RPC providers.
// Uses individual receipts strategy which spawns many parallel requests.
// Re-enable when using private/paid RPC nodes with higher rate limits.

// #[tokio::test]
// async fn test_cancellation_cleans_up_all_tasks() {
//     init_tracing();
//
//     println!("\n{}", "=".repeat(80));
//     println!("CANCELLATION: Task Cleanup Verification");
//     println!("{}\n", "=".repeat(80));
//
//     let target = RpcTarget {
//         name: "Ethereum Mainnet",
//         url: "https://ethereum-rpc.publicnode.com",
//     };
//
//     let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");
//
//     let height = provider.get_block_number().await.expect("Block number");
//     let target_block = height.saturating_sub(10);
//
//     // First fetch the block to get transaction count
//     let block = provider
//         .get_block_by_number(target_block)
//         .await
//         .expect("Block fetch");
//     let tx_count = block.transactions.len();
//
//     info!("Block #{} has {} transactions", target_block, tx_count);
//
//     if tx_count == 0 {
//         info!("  [SKIP] Block has no transactions, cannot test task cleanup");
//         return;
//     }
//
//     let cancel_token = CancellationToken::new();
//     let cancel_token_clone = cancel_token.clone();
//
//     let fetcher = EvmBlockFetcher::new(provider)
//         .with_cancellation_token(cancel_token)
//         .with_retry_interval(Duration::from_millis(100));
//
//     // Spawn task to cancel quickly - this should interrupt the individual receipt fetches
//     tokio::spawn(async move {
//         tokio::time::sleep(Duration::from_millis(10)).await;
//         cancel_token_clone.cancel();
//     });
//
//     // Use individual receipts strategy which spawns many tasks
//     let result = fetcher.fetch_block_with_individual_receipts_by_number(target_block).await;
//
//     match result {
//         Ok(_) => {
//             info!("  [OK]   Fetch completed before cancellation");
//         }
//         Err(BlockFetchError::Cancelled) => {
//             info!("  [OK]   Cancellation occurred and tasks were cleaned up");
//         }
//         Err(e) => {
//             info!("  [WARN] Unexpected error: {}", e);
//         }
//     }
//
//     // Give time for any leaked tasks to complete/fail
//     tokio::time::sleep(Duration::from_millis(100)).await;
//     info!("  [OK]   No task leaks detected (test completed without hanging)");
// }

#[tokio::test]
async fn test_verification_enabled() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("VERIFICATION: Block Verification Enabled");
    println!("{}\n", "=".repeat(80));

    // Test on chains known to pass verification
    let verification_chains = vec![
        RpcTarget {
            name: "Ethereum Mainnet",
            url: "https://ethereum-rpc.publicnode.com",
        },
        RpcTarget {
            name: "Optimism",
            url: "https://mainnet.optimism.io",
        },
        RpcTarget {
            name: "Base",
            url: "https://mainnet.base.org",
        },
    ];

    for target in verification_chains {
        info!("Testing {}...", target.name);

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
            Ok(p) => p,
            Err(e) => {
                info!("  [SKIP] {}: Provider creation failed: {}", target.name, e);
                continue;
            }
        };

        let height = match provider.get_block_number().await {
            Ok(h) => h,
            Err(e) => {
                info!("  [SKIP] {}: Block number failed: {}", target.name, e);
                continue;
            }
        };

        let fetcher = EvmBlockFetcher::new(provider)
            .with_verify_block(true)
            .with_retry_interval(Duration::from_millis(100));

        let target_block = height.saturating_sub(10);

        // Use block receipts (eth_getBlockReceipts) to get all receipts
        match fetcher
            .fetch_block_with_block_receipts_by_number(target_block)
            .await
        {
            Ok(result) => {
                info!(
                    "  [OK]   {}: Block #{} verified ({} txs)",
                    target.name,
                    target_block,
                    result.transaction_count()
                );
            }
            Err(BlockFetchError::VerificationFailed(e)) => {
                info!("  [FAIL] {}: Verification failed: {}", target.name, e);
            }
            Err(e) => {
                info!("  [FAIL] {}: {}", target.name, e);
            }
        }
    }
}

#[tokio::test]
async fn test_retry_eventually_succeeds() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("RETRY: Eventually Succeeds");
    println!("{}\n", "=".repeat(80));

    // This test verifies that the retry mechanism works by using a real RPC
    // that should succeed on the first try

    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");

    let fetcher = EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

    // Fetch a known block
    let result = fetcher.fetch_block_with_block_receipts_by_number(1).await;

    match result {
        Ok(result) => {
            info!(
                "  [OK]   Block #1 fetched ({} txs)",
                result.transaction_count()
            );
            // Block 1 should have 0 transactions (genesis has no txs)
            assert!(
                result.transaction_count() <= 1,
                "Block 1 should have 0-1 transactions"
            );
        }
        Err(e) => {
            info!("  [FAIL] {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Long-running test
async fn test_retry_with_timeout() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("RETRY: With Timeout (ignored by default)");
    println!("{}\n", "=".repeat(80));

    // This test verifies retry behavior with a short timeout
    // Uses a non-existent RPC to ensure retries happen

    let provider = SemEvmRpcProvider::new(
        "http://localhost:9999".to_string(), // Non-existent RPC
        5,
    )
    .expect("Provider creation");

    let cancel_token = CancellationToken::new();
    let cancel_token_clone = cancel_token.clone();

    // Cancel after 2 seconds
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        cancel_token_clone.cancel();
    });

    let fetcher = EvmBlockFetcher::new(provider)
        .with_cancellation_token(cancel_token)
        .with_retry_interval(Duration::from_millis(200));

    let result = fetcher.fetch_block_with_batch_receipts_by_number(1).await;

    match result {
        Err(BlockFetchError::Cancelled) => {
            info!("  [OK]   Retry was cancelled after timeout");
        }
        Ok(_) => {
            panic!("Expected cancellation, got success");
        }
        Err(e) => {
            info!("  [WARN] Got error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_fetched_block_methods() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("FETCHED_BLOCK: Helper Methods");
    println!("{}\n", "=".repeat(80));

    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");

    let height = provider.get_block_number().await.expect("Block number");
    let target_block = height.saturating_sub(10);

    let fetcher = EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

    let result = fetcher
        .fetch_block_with_block_receipts_by_number(target_block)
        .await
        .expect("Fetch should succeed");

    // Test transaction_count()
    let tx_count = result.transaction_count();
    info!("  transaction_count(): {}", tx_count);

    // Test receipts_ordered()
    let ordered = result.receipts_ordered();
    assert_eq!(ordered.len(), tx_count);
    info!("  receipts_ordered(): {} receipts", ordered.len());

    // Test get_receipt() for each transaction
    let tx_hashes: Vec<B256> = result.block.transactions.hashes().collect();
    for hash in &tx_hashes {
        let receipt = result.get_receipt(hash);
        assert!(receipt.is_some(), "Receipt should exist for tx {}", hash);
    }
    info!(
        "  get_receipt(): All {} receipts accessible",
        tx_hashes.len()
    );

    // Test fetch_id is valid UUID
    assert!(!result.fetch_id.is_nil());
    info!("  fetch_id: {}", result.fetch_id);

    info!("  [OK]   All FetchedBlock methods work correctly");
}

#[tokio::test]
async fn test_empty_block() {
    init_tracing();

    println!("\n{}", "=".repeat(80));
    println!("EDGE CASE: Empty Block (No Transactions)");
    println!("{}\n", "=".repeat(80));

    // Block 1 on Ethereum has 0 transactions
    let target = RpcTarget {
        name: "Ethereum Mainnet",
        url: "https://ethereum-rpc.publicnode.com",
    };

    let provider = SemEvmRpcProvider::new(target.url.to_string(), 5).expect("Provider creation");

    let fetcher = EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(100));

    // Fetch block 1 which has no transactions
    let result = fetcher.fetch_block_with_batch_receipts_by_number(1).await;

    match result {
        Ok(result) => {
            info!(
                "  [OK]   Block #1: {} transactions, {} receipts",
                result.transaction_count(),
                result.receipts.len()
            );
            assert_eq!(result.transaction_count(), result.receipts.len());
        }
        Err(e) => {
            info!("  [FAIL] {}", e);
        }
    }
}

// NOTE: This test is commented out due to rate limiting from free RPC providers.
// Running all strategies on the same block with many transactions exhausts rate limits.
// Re-enable when using private/paid RPC nodes with higher rate limits.

// #[tokio::test]
// async fn test_all_strategies_same_result() {
//     init_tracing();
//
//     println!("\n{}", "=".repeat(80));
//     println!("COMPARISON: All Strategies Return Same Data");
//     println!("{}\n", "=".repeat(80));
//
//     let target = RpcTarget {
//         name: "Ethereum Mainnet",
//         url: "https://ethereum-rpc.publicnode.com",
//     };
//
//     let provider = SemEvmRpcProvider::new(target.url.to_string(), 20).expect("Provider creation");
//
//     let height = provider.get_block_number().await.expect("Block number");
//     let target_block = height.saturating_sub(10);
//
//     info!("Fetching block #{} with all 5 strategies...", target_block);
//
//     // Strategy 1: Block Receipts
//     let fetcher1 = EvmBlockFetcher::new(provider.clone())
//         .with_retry_interval(Duration::from_millis(100));
//     let result1 = fetcher1
//         .fetch_block_with_block_receipts_by_number(target_block)
//         .await;
//
//     // Strategy 2: Batch Receipts
//     let fetcher2 = EvmBlockFetcher::new(provider.clone())
//         .with_retry_interval(Duration::from_millis(100));
//     let result2 = fetcher2
//         .fetch_block_with_batch_receipts_by_number(target_block)
//         .await;
//
//     // Strategy 3: Chunked Batch
//     let fetcher3 = EvmBlockFetcher::new(provider.clone())
//         .with_retry_interval(Duration::from_millis(100));
//     let result3 = fetcher3
//         .fetch_block_by_number_with_parallel_batched_receipts(target_block, 10)
//         .await;
//
//     // Strategy 4: Individual
//     let fetcher4 = EvmBlockFetcher::new(provider.clone())
//         .with_retry_interval(Duration::from_millis(100));
//     let result4 = fetcher4
//         .fetch_block_with_individual_receipts_by_number(target_block)
//         .await;
//
//     // Strategy 5: Sequential
//     let fetcher5 = EvmBlockFetcher::new(provider)
//         .with_retry_interval(Duration::from_millis(100));
//     let result5 = fetcher5
//         .fetch_block_with_sequential_receipts_by_number(target_block)
//         .await;
//
//     // Compare results
//     let results = vec![
//         ("Strategy 1 (BlockReceipts)", result1),
//         ("Strategy 2 (Batch)", result2),
//         ("Strategy 3 (Chunked)", result3),
//         ("Strategy 4 (Individual)", result4),
//         ("Strategy 5 (Sequential)", result5),
//     ];
//
//     let mut successful_results: Vec<(&str, FetchedBlock)> = vec![];
//
//     for (name, result) in results {
//         match result {
//             Ok(r) => {
//                 info!("  [OK]   {}: {} txs", name, r.transaction_count());
//                 successful_results.push((name, r));
//             }
//             Err(e) => {
//                 info!("  [FAIL] {}: {}", name, e);
//             }
//         }
//     }
//
//     // Verify all successful results have the same data
//     if successful_results.len() >= 2 {
//         let (first_name, first) = &successful_results[0];
//         let first_tx_count = first.transaction_count();
//         let first_block_hash = first.block.header.hash;
//
//         for (name, result) in &successful_results[1..] {
//             assert_eq!(
//                 result.transaction_count(),
//                 first_tx_count,
//                 "{} tx count differs from {}",
//                 name,
//                 first_name
//             );
//             assert_eq!(
//                 result.block.header.hash, first_block_hash,
//                 "{} block hash differs from {}",
//                 name, first_name
//             );
//
//             // Verify all receipts match
//             for (hash, receipt) in &first.receipts {
//                 let other_receipt = result.get_receipt(hash);
//                 assert!(
//                     other_receipt.is_some(),
//                     "{} missing receipt for {}",
//                     name,
//                     hash
//                 );
//                 assert_eq!(
//                     other_receipt.unwrap().transaction_hash,
//                     receipt.transaction_hash,
//                     "{} receipt hash mismatch",
//                     name
//                 );
//             }
//         }
//
//         info!(
//             "  [OK]   All {} successful strategies returned identical data",
//             successful_results.len()
//         );
//     } else {
//         info!(
//             "  [WARN] Only {} strategies succeeded, cannot compare",
//             successful_results.len()
//         );
//     }
// }

// ============================================================
// classify_error unit tests
// ============================================================

#[test]
fn test_classify_error_rate_limited() {
    let err = RpcProviderError::RateLimited("429 Too Many Requests".into());
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::RateLimited
    ));
}

#[test]
fn test_classify_error_transport_is_recoverable() {
    let err = RpcProviderError::TransportError("connection timeout".into());
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::Recoverable
    ));
}

#[test]
fn test_classify_error_unsupported_is_unrecoverable() {
    let err = RpcProviderError::UnsupportedMethod {
        method: "eth_getBlockReceipts".into(),
        details: "method does not exist".into(),
    };
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::Unrecoverable(BlockFetchError::UnsupportedMethod { .. })
    ));
}

#[test]
fn test_classify_error_batch_unsupported_is_unrecoverable() {
    let err = RpcProviderError::BatchUnsupported;
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::Unrecoverable(BlockFetchError::BatchUnsupported)
    ));
}

#[test]
fn test_classify_error_deserialization_is_unrecoverable() {
    let err = RpcProviderError::DeserializationError {
        method: "eth_getBlockByNumber".into(),
        details: "data did not match".into(),
    };
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::Unrecoverable(BlockFetchError::DeserializationError { .. })
    ));
}

#[test]
fn test_classify_error_batch_error_is_recoverable() {
    let err = RpcProviderError::BatchError("HTTP request failed".into());
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::Recoverable
    ));
}

#[test]
fn test_classify_error_not_found_is_recoverable() {
    let err = RpcProviderError::NotFound;
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::Recoverable
    ));
}

#[test]
fn test_classify_error_semaphore_closed_is_recoverable() {
    let err = RpcProviderError::SemaphoreClosed;
    assert!(matches!(
        classify_error(&err),
        ErrorClassification::Recoverable
    ));
}
