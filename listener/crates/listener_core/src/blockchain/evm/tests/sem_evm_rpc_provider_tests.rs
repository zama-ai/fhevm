#[cfg(test)]
mod sem_evm_rpc_provider_tests {
    use tracing::{Level, info};
    use tracing_subscriber::FmtSubscriber;

    use crate::blockchain::evm::sem_evm_rpc_provider::{SemEvmRpcProvider, extract_tx_hashes};

    struct RpcTarget {
        name: &'static str,
        url: &'static str,
    }

    #[tokio::test]
    async fn report_provider_workability() {
        // 1. Force initialize tracing to stay informed during async calls
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);

        // 2. Hardcoded List
        let targets = vec![
            RpcTarget {
                name: "Ethereum Mainnet",
                url: "https://ethereum-rpc.publicnode.com",
            },
            // RpcTarget {
            //     name: "Binance Smart Chain Mainnet",
            //     url: "https://bsc-dataseed.bnbchain.org",
            // },
            // RpcTarget {
            //     name: "Polygon PoS",
            //     // url: "https://polygon-rpc.com",
            //     url: "https://polygon.drpc.org",
            // },
            // RpcTarget {
            //     name: "Avalanche",
            //     url: "https://api.avax.network/ext/bc/C/rpc",
            // },
            // RpcTarget {
            //     name: "Monad",
            //     url: "https://rpc.monad.xyz/",
            // },
            // RpcTarget {
            //     name: "Optimism",
            //     url: "https://mainnet.optimism.io",
            // },
            // RpcTarget {
            //     name: "Arbitrum One",
            //     url: "https://arb1.arbitrum.io/rpc",
            // },
            // RpcTarget {
            //     name: "Base",
            //     url: "https://mainnet.base.org",
            // },
            // RpcTarget {
            //     name: "Zama Gateway Testnet",
            //     url: "https://rpc-zama-testnet-0.t.conduit.xyz",
            // },
            // RpcTarget {
            //     name: "Polkadot mainnet",
            //     url: "https://eth-rpc.polkadot.io/",
            // },
            // RpcTarget {
            //     name: "Polkadot testnet",
            //     url: "https://services.polkadothub-rpc.com/testnet",
            // },
        ];

        println!("\n\n{}", "=".repeat(80));
        println!("🚀 STARTING RPC WORKABILITY REPORT");
        println!("{}\n", "=".repeat(80));

        for target in targets {
            info!("Probing {}...", target.name);

            let report = probe_chain(target).await;

            // PRINT IMMEDIATELY
            print_report_entry(report);
        }

        println!("{}", "=".repeat(80));
        println!("✅ REPORT COMPLETE");
        println!("{}\n", "=".repeat(80));
    }

    async fn probe_chain(target: RpcTarget) -> ChainReport {
        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
            Ok(p) => p,
            Err(e) => return ChainReport::fatal(target, format!("Creation failed: {}", e)),
        };

        let chain_id = provider.get_chain_id().await.map_err(|e| e.to_string());

        let height = match provider.get_block_number().await {
            Ok(h) => h,
            Err(e) => return ChainReport::fatal(target, format!("Height call failed: {}", e)),
        };

        // Get block by number (Verify Header Deserialization)
        let block_res = provider.get_block_by_number(height - 10).await;

        // NEW: Get block by hash (Verify Hash-based lookup and Deserialization)
        let mut block_hash_info = Err("No Hash available".to_string());
        if let Ok(ref block) = block_res {
            let hash = block.header.hash;
            block_hash_info = provider
                .get_block_by_hash(hash.to_string())
                .await
                .map(|b| b.header.hash == hash) // Verify we got the same block back
                .map_err(|e| e.to_string());
        }

        // Get receipt (Verify Log Deserialization)
        let mut nb_of_logs_for_tx_1 = Err("No Tx".to_string());
        if let Ok(ref block) = block_res {
            // info!(
            //     "Transaction hashes {:?}",
            //     block.transactions.hashes().collect::<Vec<_>>()
            // );
            if let Some(tx) = block.transactions.hashes().next() {
                // info!("Tested tx hash: {}", tx.clone().to_string());
                let receipt_info = provider
                    .get_transaction_receipt(tx.to_string())
                    .await
                    .map_err(|e| e.to_string());
                // info!(
                //     "Receipt for hash {}: {:?}",
                //     tx,
                //     receipt_info.clone().unwrap()
                // );

                let log = receipt_info
                    .map(|r| r.logs().len())
                    .map_err(|e| e.to_string());
                // info!("Number of logs for hash {}: {}", tx, log.clone().unwrap());
                nb_of_logs_for_tx_1 = log;
            }
        }

        // Get Block Receipts (Efficiency check)
        let block_receipts_info = provider
            .get_block_receipts(height - 10)
            .await
            .map(|r| r.len())
            .map_err(|e| e.to_string());

        // Test batch receipts - need to re-fetch block for tx hashes
        let batch_receipts_info = match provider.get_block_by_number(height - 10).await {
            Ok(block) => {
                let tx_hashes = extract_tx_hashes(&block);
                if tx_hashes.is_empty() {
                    Ok(0)
                } else {
                    // Only test with first 3 transactions to avoid overwhelming the node
                    // let len = tx_hashes.len();
                    // let test_hashes: Vec<_> =
                    // tx_hashes.into_iter().take(len).collect();
                    let test_hashes: Vec<_> = tx_hashes.into_iter().take(3).collect();
                    provider
                        .get_transaction_receipts_batch(&test_hashes)
                        .await
                        .map(|r| r.len())
                        .map_err(|e| e.to_string())
                }
            }
            Err(e) => Err(format!("Block fetch failed: {}", e)),
        };

        ChainReport {
            name: target.name.to_string(),
            url: target.url.to_string(),
            chain_id,
            block_number: Ok(height),
            get_block: block_res
                .map(|b| b.transactions.len())
                .map_err(|e| e.to_string()),
            get_block_hash: block_hash_info,
            get_receipt: nb_of_logs_for_tx_1,
            block_receipts: block_receipts_info,
            batch_receipts: batch_receipts_info,
        }
    }

    struct ChainReport {
        name: String,
        url: String,
        chain_id: Result<u64, String>,
        block_number: Result<u64, String>,
        get_block: Result<usize, String>,
        get_block_hash: Result<bool, String>,
        get_receipt: Result<usize, String>,
        block_receipts: Result<usize, String>,
        batch_receipts: Result<usize, String>,
    }

    impl ChainReport {
        fn fatal(target: RpcTarget, reason: String) -> Self {
            Self {
                name: target.name.to_string(),
                url: target.url.to_string(),
                chain_id: Err("Skipped".to_string()),
                block_number: Err(reason),
                get_block: Err("Skipped".to_string()),
                get_block_hash: Err("Skipped".to_string()),
                get_receipt: Err("Skipped".to_string()),
                block_receipts: Err("Skipped".to_string()),
                batch_receipts: Err("Skipped".to_string()),
            }
        }
    }

    fn print_report_entry(r: ChainReport) {
        info!("CHAIN: {}", r.name);
        info!("URL:   {}", r.url);

        let fmt_res = |res: &Result<usize, String>, label: &str| match res {
            Ok(val) => info!("  [OK]   {:<25} Count: {}", label, val),
            Err(e) => info!("  [FAIL] {:<25} Error: {}", label, e),
        };

        let fmt_res_for_log = |res: &Result<usize, String>, label: &str| match res {
            Ok(val) => info!("  [OK]   {:<25} Log Count for tx 1: {}", label, val),
            Err(e) => info!("  [FAIL] {:<25} Error: {}", label, e),
        };

        // Custom formatter for the boolean hash check
        let fmt_bool = |res: &Result<bool, String>, label: &str| match res {
            Ok(true) => info!("  [OK]   {:<25} Verified", label),
            Ok(false) => info!("  [FAIL] {:<25} Hash Mismatch", label),
            Err(e) => info!("  [FAIL] {:<25} Error: {}", label, e),
        };

        match r.chain_id {
            Ok(id) => info!("  [OK]   eth_chainId               Chain ID: {}", id),
            Err(e) => info!("  [FAIL] eth_chainId               Error: {}", e),
        }

        match r.block_number {
            Ok(h) => info!("  [OK]   eth_blockNumber           Height: {}", h),
            Err(e) => info!("  [FAIL] eth_blockNumber           Error: {}", e),
        }

        fmt_res(&r.get_block, "eth_getBlockByNumber");
        fmt_bool(&r.get_block_hash, "eth_getBlockByHash");
        fmt_res_for_log(&r.get_receipt, "eth_getTransactionReceipt");
        fmt_res(&r.block_receipts, "eth_getBlockReceipts");
        fmt_res(&r.batch_receipts, "batch receipts");

        info!("{}\n", "-".repeat(40));
    }
}
