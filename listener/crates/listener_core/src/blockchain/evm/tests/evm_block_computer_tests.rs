#[cfg(test)]
mod evm_block_computer_tests {
    use crate::blockchain::evm::{
        evm_block_computer::EvmBlockComputer,
        sem_evm_rpc_provider::{RpcProviderError, SemEvmRpcProvider, extract_tx_hashes},
    };
    use alloy::network::{AnyRpcBlock, AnyTransactionReceipt};
    use tracing::{Level, error, info, trace};
    use tracing_subscriber::FmtSubscriber;

    struct RpcTarget {
        name: &'static str,
        url: &'static str,
    }

    /// Fetch receipts with 3-tier fallback strategy.
    /// 1. eth_getBlockReceipts (most efficient, single call)
    /// 2. Batched eth_getTransactionReceipt (single HTTP call, multiple JSON-RPC requests)
    /// 3. Individual eth_getTransactionReceipt calls (least efficient)
    async fn fetch_receipts_with_fallback(
        provider: &SemEvmRpcProvider,
        block: &AnyRpcBlock,
    ) -> Result<Vec<AnyTransactionReceipt>, String> {
        // 1. Try eth_getBlockReceipts first (most efficient)
        match provider.get_block_receipts(block.header.number).await {
            Ok(receipts) => return Ok(receipts),
            Err(RpcProviderError::UnsupportedMethod { .. }) => {
                info!("eth_getBlockReceipts unsupported, trying batch...");
            }
            Err(e) => return Err(e.to_string()),
        }

        // 2. Try batched eth_getTransactionReceipt (single HTTP call)
        let tx_hashes = extract_tx_hashes(block);
        match provider.get_transaction_receipts_batch(&tx_hashes).await {
            Ok(receipts) => return Ok(receipts),
            Err(RpcProviderError::UnsupportedMethod { .. })
            | Err(RpcProviderError::BatchError(_))
            | Err(RpcProviderError::BatchUnsupported) => {
                info!("Batch receipts unsupported, falling back to individual calls...");
            }
            Err(e) => return Err(e.to_string()),
        }

        // 3. Final fallback: individual eth_getTransactionReceipt calls
        let mut receipts = Vec::new();
        for hash in block.transactions.hashes() {
            let receipt = provider
                .get_transaction_receipt(hash.to_string())
                .await
                .map_err(|e| format!("Receipt fetch failed for {}: {}", hash, e))?;
            receipts.push(receipt);
        }
        Ok(receipts)
    }

    struct VerificationReport {
        name: String,
        url: String,
        block_number: Result<u64, String>,
        tx_count: Result<usize, String>,
        receipt_count: Result<usize, String>,
        tx_root_verification: Result<(), String>,
        receipt_root_verification: Result<(), String>,
        block_hash_verification: Result<(), String>,
    }

    impl VerificationReport {
        fn fatal(target: &RpcTarget, reason: String) -> Self {
            Self {
                name: target.name.to_string(),
                url: target.url.to_string(),
                block_number: Err(reason),
                tx_count: Err("Skipped".to_string()),
                receipt_count: Err("Skipped".to_string()),
                tx_root_verification: Err("Skipped".to_string()),
                receipt_root_verification: Err("Skipped".to_string()),
                block_hash_verification: Err("Skipped".to_string()),
            }
        }
    }

    #[tokio::test]
    async fn report_block_verification() {
        // Initialize tracing
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber);

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
        println!("🔍 STARTING BLOCK VERIFICATION REPORT");
        println!("{}\n", "=".repeat(80));

        for target in targets {
            info!("Probing {}...", target.name);

            let report = probe_chain_verification(&target).await;

            print_verification_report(&report);
        }

        println!("{}", "=".repeat(80));
        println!("✅ VERIFICATION REPORT COMPLETE");
        println!("{}\n", "=".repeat(80));
    }

    async fn probe_chain_verification(target: &RpcTarget) -> VerificationReport {
        // 1. Create provider
        let provider = match SemEvmRpcProvider::new(target.url.to_string(), 5) {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to create provider: {}", e);
                return VerificationReport::fatal(
                    target,
                    format!("Provider creation failed: {}", e),
                );
            }
        };

        // 2. Get latest block number
        let height = match provider.get_block_number().await {
            Ok(h) => h,
            Err(e) => {
                error!("Failed to get block number: {}", e);
                return VerificationReport::fatal(
                    target,
                    format!("Block number call failed: {}", e),
                );
            }
        };

        let target_block = height - 10;
        info!("Block #{} fetched (height: {})", target_block, height);

        // 3. Fetch block
        trace!("Fetching block...");
        let block = match provider.get_block_by_number(target_block).await {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to fetch block: {}", e);
                return VerificationReport::fatal(target, format!("Block fetch failed: {}", e));
            }
        };

        let tx_count = block.transactions.len();
        info!("Block #{} fetched ({} txs)", target_block, tx_count);

        // 4. Fetch all receipts for that block with fallback
        trace!("Fetching receipts...");
        let receipts_result = fetch_receipts_with_fallback(&provider, &block).await;
        let (receipt_count, receipts) = match receipts_result {
            Ok(r) => {
                info!(
                    "Block #{} receipts fetched ({} receipts)",
                    target_block,
                    r.len()
                );
                (Ok(r.len()), Some(r))
            }
            Err(e) => {
                error!("Failed to fetch receipts: {}", e);
                (Err(format!("Receipt fetch failed: {}", e)), None)
            }
        };

        // 5. Run each verification separately to report granular results
        // Panic handling is now done in the main code, so we just call directly
        let tx_root = EvmBlockComputer::verify_transaction_root(&block).map_err(|e| {
            error!("Transaction root verification failed: {}", e);
            e.to_string()
        });

        let receipt_root = match &receipts {
            Some(r) => EvmBlockComputer::verify_receipt_root(&block, r).map_err(|e| {
                error!("Receipt root verification failed: {}", e);
                e.to_string()
            }),
            None => Err("Skipped (no receipts)".to_string()),
        };

        let block_hash = EvmBlockComputer::verify_block_hash(&block).map_err(|e| {
            error!("Block hash verification failed: {}", e);
            e.to_string()
        });

        // 6. Return report
        VerificationReport {
            name: target.name.to_string(),
            url: target.url.to_string(),
            block_number: Ok(target_block),
            tx_count: Ok(tx_count),
            receipt_count,
            tx_root_verification: tx_root,
            receipt_root_verification: receipt_root,
            block_hash_verification: block_hash,
        }
    }

    fn print_verification_report(r: &VerificationReport) {
        info!("CHAIN: {}", r.name);
        info!("URL:   {}", r.url);

        // Print block info
        match (&r.block_number, &r.tx_count, &r.receipt_count) {
            (Ok(block), Ok(txs), Ok(receipts)) => {
                info!("  Block #{} ({} txs, {} receipts)", block, txs, receipts);
            }
            (Ok(block), Ok(txs), Err(_)) => {
                info!("  Block #{} ({} txs, receipts unavailable)", block, txs);
            }
            (Err(e), _, _) => {
                info!("  [FAIL] Block fetch: {}", e);
            }
            _ => {}
        }

        // Print verification results
        match &r.tx_root_verification {
            Ok(()) => info!("  [OK]   Transaction Root        Verified ✓"),
            Err(e) => info!("  [FAIL] Transaction Root        {}", e),
        }

        match &r.receipt_root_verification {
            Ok(()) => info!("  [OK]   Receipt Root            Verified ✓"),
            Err(e) => info!("  [FAIL] Receipt Root            {}", e),
        }

        match &r.block_hash_verification {
            Ok(()) => info!("  [OK]   Block Hash              Verified ✓"),
            Err(e) => info!("  [FAIL] Block Hash              {}", e),
        }

        info!("{}\n", "-".repeat(40));
    }
}
