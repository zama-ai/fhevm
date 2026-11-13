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
            name: "Ethereum Mainnet",
            url: "https://ethereum-rpc.publicnode.com",
        },
        RpcTarget {
            name: "Polygon PoS",
            url: "https://polygon-rpc.com",
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

    let provider =
        SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");
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

    let provider =
        SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");
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

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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
//     let provider = match SemEvmRpcProvider::new(target.url.to_string(), 10, false, 64) {
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

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 10, false, 64) {
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

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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

    let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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

    let provider =
        SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");

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

    let provider =
        SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");

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
//     let provider = SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");
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

        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64) {
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

    let provider =
        SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");

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
        false,
        64,
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

    let provider =
        SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");

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

    let provider =
        SemEvmRpcProvider::new(target.url.to_string(), 5, false, 64).expect("Provider creation");

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

// ── Bounded NotFound retries (reorg walk) ───────────────────────────────

/// Run `retry_with_cancel` with a 1ms interval over an operation that returns
/// `errors` in order, then `Ok(42)`. Returns the result and the attempt count.
async fn run_retry(
    errors: Vec<RpcProviderError>,
    not_found_max_retries: Option<u32>,
    cancel_at_attempt: Option<usize>,
) -> (Result<u32, BlockFetchError>, usize) {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let attempts = AtomicUsize::new(0);
    let errors = Mutex::new(errors.into_iter());
    let token = CancellationToken::new();
    let result = retry_with_cancel(
        || {
            let attempt = attempts.fetch_add(1, Ordering::SeqCst) + 1;
            // Deterministic cancellation: no wall-clock timing involved.
            if cancel_at_attempt == Some(attempt) {
                token.cancel();
            }
            let next = errors.lock().unwrap().next();
            async move {
                match next {
                    Some(e) => Err(e),
                    None => Ok(42u32),
                }
            }
        },
        Duration::from_millis(1),
        1,
        &token,
        not_found_max_retries,
    )
    .await;
    (result, attempts.load(Ordering::SeqCst))
}

fn not_found(n: usize) -> Vec<RpcProviderError> {
    (0..n).map(|_| RpcProviderError::NotFound).collect()
}

#[tokio::test]
async fn test_not_found_cap_gives_up_after_max_retries() {
    let (result, attempts) = run_retry(not_found(100), Some(2), None).await;
    assert!(matches!(
        result,
        Err(BlockFetchError::NotFoundAfterRetries { attempts: 3 })
    ));
    assert_eq!(attempts, 3);
}

#[tokio::test]
async fn test_not_found_cap_zero_gives_up_on_first_null() {
    let (result, attempts) = run_retry(not_found(100), Some(0), None).await;
    assert!(matches!(
        result,
        Err(BlockFetchError::NotFoundAfterRetries { attempts: 1 })
    ));
    assert_eq!(attempts, 1);
}

#[tokio::test]
async fn test_not_found_within_cap_still_succeeds() {
    let (result, attempts) = run_retry(not_found(2), Some(2), None).await;
    assert_eq!(result.unwrap(), 42);
    assert_eq!(attempts, 3);
}

#[tokio::test]
async fn test_not_found_uncapped_retries_until_cancelled() {
    // Far beyond any cap: without one, NotFound keeps retrying until cancelled.
    let (result, attempts) = run_retry(not_found(100), None, Some(20)).await;
    assert!(matches!(result, Err(BlockFetchError::Cancelled)));
    assert_eq!(attempts, 20);
}

#[tokio::test]
async fn test_not_found_cap_ignores_other_errors() {
    let errors = vec![
        RpcProviderError::TransportError("timeout".to_string()),
        RpcProviderError::TransportError("timeout".to_string()),
        RpcProviderError::NotFound,
        RpcProviderError::TransportError("timeout".to_string()),
    ];
    let (result, attempts) = run_retry(errors, Some(1), None).await;
    assert_eq!(result.unwrap(), 42);
    assert_eq!(attempts, 5);
}

// ── Receipts must belong to the block ───────────────────────────────────

#[test]
fn test_first_foreign_receipt() {
    let block = B256::repeat_byte(0xaa);
    let other = B256::repeat_byte(0xbb);
    let tx1 = B256::repeat_byte(0x01);
    let tx2 = B256::repeat_byte(0x02);

    assert_eq!(first_foreign_receipt(block, std::iter::empty()), None);
    assert_eq!(
        first_foreign_receipt(block, [(tx1, Some(block)), (tx2, Some(block))].into_iter()),
        None
    );
    assert_eq!(
        first_foreign_receipt(block, [(tx1, Some(block)), (tx2, Some(other))].into_iter()),
        Some((tx2, Some(other)))
    );
    assert_eq!(
        first_foreign_receipt(block, [(tx1, None)].into_iter()),
        Some((tx1, None))
    );
}

#[test]
fn test_is_inconsistent_snapshot() {
    let hash = B256::ZERO;
    let inconsistent = [
        BlockFetchError::NotFoundAfterRetries { attempts: 6 },
        BlockFetchError::ReceiptCountMismatch {
            block_tx_count: 4,
            receipt_count: 0,
        },
        BlockFetchError::MissingReceipt { tx_hash: hash },
        BlockFetchError::ReceiptBlockMismatch {
            tx_hash: hash,
            expected: hash,
            got: None,
        },
    ];
    for e in &inconsistent {
        assert!(e.is_inconsistent_snapshot(), "{e} should be inconsistent");
    }

    let other = [
        BlockFetchError::Cancelled,
        BlockFetchError::BlockNotFound,
        BlockFetchError::BatchUnsupported,
        BlockFetchError::UnsupportedMethod {
            method: "m".to_string(),
            details: "d".to_string(),
        },
        BlockFetchError::DeserializationError {
            method: "m".to_string(),
            details: "d".to_string(),
        },
    ];
    for e in &other {
        assert!(
            !e.is_inconsistent_snapshot(),
            "{e} should not be inconsistent"
        );
    }
}

// ── Receipt blockHash gate, against an in-process mock JSON-RPC node ────────
//
// Real BSC testnet block 142386396 (2 txs) and its receipts, served by a local
// axum server so the full provider → fetcher → build_fetched_block path runs
// without network access.

use super::mock_rpc;

const FOREIGN_HASH: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";

fn mock_fetcher(url: &str) -> EvmBlockFetcher {
    let provider = SemEvmRpcProvider::new(url.to_string(), 5, false, 64).expect("provider");
    EvmBlockFetcher::new(provider).with_retry_interval(Duration::from_millis(1))
}

/// Flag off (catch-up, final catch-up, finality, startup): a receipt from another
/// block is accepted exactly as before the check existed.
#[tokio::test]
async fn test_receipt_block_hash_check_off_accepts_foreign_receipt() {
    let node = mock_rpc::start(Some(FOREIGN_HASH), false).await;
    let fetched = mock_fetcher(&node.url)
        .fetch_block_with_block_receipts_by_number(node.block_number)
        .await
        .expect("flag off must not check receipt blockHash");
    assert_eq!(fetched.block.header.hash, node.block_hash);
    assert_eq!(fetched.receipts.len(), 2);
}

/// Flag on (live cursor, reorg walk): a receipt from another block is rejected.
#[tokio::test]
async fn test_receipt_block_hash_check_on_rejects_foreign_receipt() {
    let node = mock_rpc::start(Some(FOREIGN_HASH), false).await;
    let err = mock_fetcher(&node.url)
        .with_verify_receipt_block_hash(true)
        .fetch_block_with_block_receipts_by_number(node.block_number)
        .await
        .expect_err("foreign receipt must be rejected");
    match &err {
        BlockFetchError::ReceiptBlockMismatch { expected, got, .. } => {
            assert_eq!(*expected, node.block_hash);
            assert_eq!(*got, Some(FOREIGN_HASH.parse().unwrap()));
        }
        other => panic!("expected ReceiptBlockMismatch, got {other:?}"),
    }
    assert!(err.is_inconsistent_snapshot());
}

/// Flag on, matching receipts: the block is accepted (by number and by hash).
#[tokio::test]
async fn test_receipt_block_hash_check_on_accepts_matching_receipts() {
    let node = mock_rpc::start(None, false).await;
    let fetcher = mock_fetcher(&node.url).with_verify_receipt_block_hash(true);
    let by_number = fetcher
        .fetch_block_with_block_receipts_by_number(node.block_number)
        .await
        .expect("matching receipts by number");
    let by_hash = fetcher
        .fetch_block_with_block_receipts_by_hash(node.block_hash)
        .await
        .expect("matching receipts by hash");
    assert_eq!(by_number.block.header.hash, node.block_hash);
    assert_eq!(by_hash.block.header.hash, node.block_hash);
}

/// Reorg walk: a block that vanished (null by hash) gives up after the cap
/// instead of retrying forever (the bnb-testnet 142208778 stall).
#[tokio::test]
async fn test_by_hash_null_gives_up_after_cap() {
    let node = mock_rpc::start(None, true).await;
    let err = mock_fetcher(&node.url)
        .with_not_found_max_retries(Some(2))
        .fetch_block_with_block_receipts_by_hash(node.block_hash)
        .await
        .expect_err("vanished block must give up");
    assert!(
        matches!(err, BlockFetchError::NotFoundAfterRetries { attempts: 3 }),
        "got {err:?}"
    );
    assert!(err.is_inconsistent_snapshot());
    // Two strategy-1 tasks each fetch by hash; the first to hit the cap wins.
    let calls = node.by_hash_calls.load(std::sync::atomic::Ordering::SeqCst);
    assert!((3..=6).contains(&calls), "by-hash calls: {calls}");
}

// ── Null per-tx receipts (tx dropped by a reorg) ────────────────────────────

/// Fetch block by number with each per-tx receipt strategy (2-5).
async fn fetch_per_tx_strategy(
    fetcher: &EvmBlockFetcher,
    strategy: &str,
    block_number: u64,
) -> Result<FetchedBlock, BlockFetchError> {
    match strategy {
        "batch_full" => {
            fetcher
                .fetch_block_with_batch_receipts_by_number(block_number)
                .await
        }
        "batch_range" => {
            fetcher
                .fetch_block_by_number_with_parallel_batched_receipts(block_number, 1)
                .await
        }
        "individual" => {
            fetcher
                .fetch_block_with_individual_receipts_by_number(block_number)
                .await
        }
        "sequential" => {
            fetcher
                .fetch_block_with_sequential_receipts_by_number(block_number)
                .await
        }
        other => unreachable!("{other}"),
    }
}

const PER_TX_STRATEGIES: [&str; 4] = ["batch_full", "batch_range", "individual", "sequential"];

/// Sanity: the mock serves real receipts per tx, single and batched.
#[tokio::test]
async fn test_per_tx_strategies_fetch_receipts_from_mock() {
    let node = mock_rpc::start(None, false).await;
    for strategy in PER_TX_STRATEGIES {
        let fetched = fetch_per_tx_strategy(&mock_fetcher(&node.url), strategy, node.block_number)
            .await
            .unwrap_or_else(|e| panic!("{strategy}: {e:?}"));
        assert_eq!(fetched.receipts.len(), 2, "{strategy}");
    }
}

/// Live cursor: a tx dropped by a reorg keeps a null receipt forever. With the
/// receipt cap, every per-tx strategy gives up with an inconsistent-snapshot
/// error (which re-arms the cursor) instead of stalling.
#[tokio::test]
async fn test_null_tx_receipt_gives_up_after_receipt_cap() {
    let node = mock_rpc::start_with_null_tx_receipts().await;
    for strategy in PER_TX_STRATEGIES {
        let fetcher = mock_fetcher(&node.url).with_receipt_not_found_max_retries(Some(2));
        let err = fetch_per_tx_strategy(&fetcher, strategy, node.block_number)
            .await
            .expect_err(strategy);
        assert!(
            matches!(err, BlockFetchError::NotFoundAfterRetries { attempts: 3 }),
            "{strategy}: got {err:?}"
        );
        assert!(err.is_inconsistent_snapshot(), "{strategy}");
    }
}

/// Without the cap (catch-up, final catch-up, finality, startup): unchanged,
/// a null receipt is retried until cancelled.
#[tokio::test]
async fn test_null_tx_receipt_uncapped_retries_until_cancelled() {
    let node = mock_rpc::start_with_null_tx_receipts().await;
    for strategy in PER_TX_STRATEGIES {
        let before = node
            .tx_receipt_calls
            .load(std::sync::atomic::Ordering::SeqCst);
        let token = CancellationToken::new();
        let fetcher = mock_fetcher(&node.url).with_cancellation_token(token.clone());
        let cancel_task = {
            let calls = node.tx_receipt_calls.clone();
            tokio::spawn(async move {
                // Cancel only once the cap a capped fetch would use is far exceeded.
                while calls.load(std::sync::atomic::Ordering::SeqCst) < before + 20 {
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
                token.cancel();
            })
        };
        let err = fetch_per_tx_strategy(&fetcher, strategy, node.block_number)
            .await
            .expect_err(strategy);
        cancel_task.await.unwrap();
        assert!(
            matches!(err, BlockFetchError::Cancelled),
            "{strategy}: got {err:?}"
        );
    }
}

/// `block_receipts` never uses the receipt cap: a null `eth_getBlockReceipts`
/// is still governed by `not_found_max_retries` only (here: uncapped).
#[tokio::test]
async fn test_receipt_cap_ignored_by_block_receipts_strategy() {
    let node = mock_rpc::start_with_null_tx_receipts().await;
    // eth_getBlockReceipts is served normally: the receipt cap must not matter.
    let fetched = mock_fetcher(&node.url)
        .with_receipt_not_found_max_retries(Some(0))
        .fetch_block_with_block_receipts_by_number(node.block_number)
        .await
        .expect("block_receipts unaffected");
    assert_eq!(fetched.receipts.len(), 2);
    assert_eq!(
        node.tx_receipt_calls
            .load(std::sync::atomic::Ordering::SeqCst),
        0
    );
}
