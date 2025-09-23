use crate::{
    blockchain::{
        provider::{FillersWithoutNonceManagement, NonceManagedProvider},
        wallet::Wallet,
    },
    config::Config,
    db_manager::{DbManager, DbConnectorConfig, DecryptionRequestType},
    decryption::{
        EVENT_LISTENER_POLLING, init_public_decryption_response_listener,
        init_user_decryption_response_listener, public_decryption_burst, user_decryption_burst,
    },
};
use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{
        Identity, ProviderBuilder, RootProvider, WsConnect,
        fillers::{FillProvider, JoinFill, WalletFiller},
    },
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{self, DecryptionInstance};
use gateway_sdk::{FhevmSdk, FhevmSdkBuilder};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{Arc, Once};
use std::time::Duration;
use tokio::{
    task::JoinSet,
    time::{Instant, interval},
};
use tracing::{Instrument, info};

/// The provider used to interact with the Gateway.
type AppProvider = NonceManagedProvider<
    FillProvider<
        JoinFill<JoinFill<Identity, FillersWithoutNonceManagement>, WalletFiller<EthereumWallet>>,
        RootProvider,
    >,
>;

/// A struct used to perform the load/stress testing of the Gateway.
pub struct App {
    /// The decryption contract instance.
    decryption_contract: DecryptionInstance<AppProvider>,

    /// The configuration of the test session.
    config: Config,

    /// The wallet used to send the requests to the Gateway.
    wallet: Wallet,
    

    /// The fhevm rust sdk used to compute the EIP712 for UserDecryptions.
    sdk: Arc<FhevmSdk>,
}

impl App {
    /// Connects the tool to the Gateway.
    pub async fn connect(config: Config) -> anyhow::Result<Self> {
        INSTALL_CRYPTO_PROVIDER_ONCE.call_once(|| {
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .map_err(|e| anyhow!("Failed to install AWS-LC crypto provider: {e:?}"))
                .unwrap()
        });

        let wallet = Wallet::from_config(&config).await?;
        let gateway_url = config.gateway_url
            .as_ref()
            .ok_or_else(|| anyhow!("gateway_url is required for blockchain testing"))?;
        let provider = NonceManagedProvider::new(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .filler(FillersWithoutNonceManagement::default())
                .wallet(wallet.clone())
                .connect_ws(WsConnect::new(gateway_url))
                .await
                .map_err(|e| anyhow!("Failed to connect to Gateway at {gateway_url}: {e}"))?,
            wallet.address(),
        );
        info!("Successfully connected to the Gateway");
        let decryption_address = config.decryption_address
            .ok_or_else(|| anyhow!("decryption_address is required for blockchain testing"))?;
        let decryption_contract = Decryption::new(decryption_address, provider);

        let gateway_chain_id = config.gateway_chain_id
            .ok_or_else(|| anyhow!("gateway_chain_id is required for blockchain testing"))?;
        let host_chain_id = config.host_chain_id
            .ok_or_else(|| anyhow!("host_chain_id is required for blockchain testing"))?;
            
        let sdk = Arc::new(
            FhevmSdkBuilder::new()
                .with_gateway_chain_id(gateway_chain_id)
                .with_decryption_contract(&decryption_address.to_string())
                .with_acl_contract(&Address::ZERO.to_string())
                .with_input_verification_contract(&Address::ZERO.to_string())
                .with_host_chain_id(host_chain_id)
                .build()?,
        );

        Ok(Self {
            decryption_contract,
            config,
            wallet,
            sdk,
        })
    }

    /// Performs the public decryption stress test.
    pub async fn public_decryption_stress_test(&self) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();
        let response_listener =
            init_public_decryption_response_listener(self.decryption_contract.clone()).await?;
        tokio::time::sleep(EVENT_LISTENER_POLLING).await; // Sleep for listener to be ready

        let session_start = Instant::now();
        let mut interval = interval(self.config.tests_interval.unwrap_or(Duration::from_secs(1)));
        let mut burst_tasks = JoinSet::new();
        let mut burst_index = 1;
        loop {
            if !self.config.sequential {
                interval.tick().await;
            }

            if session_start.elapsed() > self.config.tests_duration.unwrap_or(Duration::from_secs(60)) {
                break;
            }

            let (requests_pb, responses_pb) =
                self.init_progress_bars(&progress_tracker, burst_index)?;

            burst_tasks.spawn(
                public_decryption_burst(
                    burst_index,
                    self.config.clone(),
                    self.decryption_contract.clone(),
                    response_listener.clone(),
                    requests_pb,
                    responses_pb,
                )
                .in_current_span(),
            );

            burst_index += 1;

            if self.config.sequential {
                burst_tasks.join_next().await;
            }
        }

        burst_tasks.join_all().await;
        let elapsed = session_start.elapsed().as_secs_f64();
        info!(
            "Handled all burst in {:.2}s. Throughput: {:.2} tps",
            elapsed,
            (self.config.parallel_requests.unwrap_or(1) * (burst_index - 1) as u32) as f64 / elapsed
        );
        Ok(())
    }

    /// Performs the user decryption stress test.
    pub async fn user_decryption_stress_test(&self) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();
        let response_listener =
            init_user_decryption_response_listener(self.decryption_contract.clone()).await?;
        tokio::time::sleep(EVENT_LISTENER_POLLING).await; // Sleep for listener to be ready

        let session_start = Instant::now();
        let mut interval = interval(self.config.tests_interval.unwrap_or(Duration::from_secs(1)));
        let mut burst_tasks = JoinSet::new();
        let mut burst_index = 1;
        loop {
            if !self.config.sequential {
                interval.tick().await;
            }

            if session_start.elapsed() > self.config.tests_duration.unwrap_or(Duration::from_secs(60)) {
                break;
            }

            let (requests_pb, responses_pb) =
                self.init_progress_bars(&progress_tracker, burst_index)?;

            burst_tasks.spawn(user_decryption_burst(
                burst_index,
                self.config.clone(),
                self.decryption_contract.clone(),
                Arc::clone(&self.sdk),
                self.wallet.address(),
                response_listener.clone(),
                requests_pb,
                responses_pb,
            ));

            burst_index += 1;

            if self.config.sequential {
                burst_tasks.join_next().await;
            }
        }

        burst_tasks.join_all().await;
        let elapsed = session_start.elapsed().as_secs_f64();
        info!(
            "Handled all burst in {:.2}s. Throughput: {:.2} tps",
            elapsed,
            (self.config.parallel_requests.unwrap_or(1) * (burst_index - 1) as u32) as f64 / elapsed
        );
        Ok(())
    }

    /// Run DB connector stress test using direct database insertions (standalone, no gateway connection)
    pub async fn db_connector_stress_test_standalone(
        config: Config,
        args: crate::cli::DbConnectorArgs,
    ) -> anyhow::Result<()> {
        use humantime::parse_duration;
        use std::time::{Duration, Instant};
        use tokio::time::interval;
        
        info!("Starting DB connector test");
        
        // Load config from specified file or use provided config
        let config = if let Some(config_path) = args.config {
            info!("Loading config from: {:?}", config_path);
            Config::from_env_and_file(Some(config_path))?
        } else {
            config
        };
        
        // Get DB configuration from config file or use defaults
        let db_config = config.db_connector.as_ref()
            .ok_or_else(|| anyhow!("Missing [db_connector] section in config file"))?;
        
        // Initialize DB manager with configured databases
        let mut configs = Vec::new();
        let database_urls = &db_config.database_urls;
        
        // Use -n to limit number of databases if specified
        let num_to_use = args.num_connectors.unwrap_or(database_urls.len());
        let urls_to_use = &database_urls[..num_to_use.min(database_urls.len())];
        
        for url in urls_to_use {
            let mut config = DbConnectorConfig::new(url.clone());
            config.pool_size = db_config.pool_size.map(|s| s as u32);
            config.connection_timeout = db_config.connection_timeout;
            configs.push(config);
        }
        
        info!("Using {} database(s) out of {} configured", configs.len(), database_urls.len());
        let db_manager = DbManager::new(configs).await?;
        
        // Health check
        info!("Performing health check...");
        let health_results = db_manager.health_check().await?;
        if !health_results[0] {
            return Err(anyhow!("Database is not healthy"));
        }
        info!("Database is healthy!");
        
        // Clear databases if requested
        if args.clear_db {
            info!("Clearing database tables before test...");
            db_manager.clear_databases().await?;
            info!("Database tables cleared");
        }
        
        // Use config defaults, override with CLI args if provided
        let request_type_str = args.request_type
            .unwrap_or_else(|| db_config.request_type.clone());
        
        let request_type = match request_type_str.as_str() {
            "public" => DecryptionRequestType::Public,
            "user" => DecryptionRequestType::User,
            "mixed" => DecryptionRequestType::Mixed,
            _ => return Err(anyhow!("Invalid request type: {}", request_type_str)),
        };
        
        // Use config durations, can be overridden by CLI
        let test_duration = if let Some(duration_str) = args.duration {
            parse_duration(&duration_str)?
        } else {
            db_config.duration
        };
        
        // Parse interval override
        let batch_interval = if let Some(interval_str) = args.interval {
            parse_duration(&interval_str)?
        } else {
            db_config.batch_interval
        };
        
        let batch_size = args.batch_size.unwrap_or(db_config.batch_size);
        
        // Create progress tracker
        let progress_tracker = MultiProgress::new();
        let style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )?;
        
        let progress_bar = progress_tracker.add(
            ProgressBar::new(100)
                .with_message("DB Connector Test Progress")
                .with_style(style),
        );
        
        info!("Starting test for {:?}", test_duration);
        info!("Inserting {} requests every {:?}", batch_size, batch_interval);
        
        let start_time = Instant::now();
        let mut interval = interval(batch_interval);
        let mut total_inserted = 0;
        
        // Main test loop
        while start_time.elapsed() < test_duration {
            interval.tick().await;
            
            // Insert batch of requests
            let request_ids = db_manager
                .insert_requests(request_type, batch_size)
                .await?;
            
            total_inserted += request_ids.len();
            let elapsed_percent = (start_time.elapsed().as_secs() as f64 / test_duration.as_secs() as f64 * 100.0) as u64;
            progress_bar.set_position(elapsed_percent.min(100));
            progress_bar.set_message(format!("Inserted {} requests", total_inserted));
            
            // Track responses if enabled
            if args.track_responses {
                db_manager.track_responses().await?;
                let stats = db_manager.get_stats().await;
                info!(
                    "Sync status - Fully synced: {}/{} ({:.1}%)",
                    stats.fully_synced,
                    stats.total_requests,
                    (stats.fully_synced as f64 / stats.total_requests.max(1) as f64) * 100.0
                );
            }
        }
        
        progress_bar.finish_with_message(format!("Test completed - {} requests inserted", total_inserted));
        
        // Final response tracking
        if args.track_responses {
            info!("Waiting 5 seconds for final responses...");
            tokio::time::sleep(Duration::from_secs(5)).await;
            
            db_manager.track_responses().await?;
            let stats = db_manager.get_stats().await;
            
            info!("\n=== Final Results ===");
            info!("Total requests inserted: {}", total_inserted);
            info!("Fully synced across all connectors: {}", stats.fully_synced);
            info!("Partially synced: {}", stats.partially_synced);
            info!("Missing from all: {}", stats.missing);
            
            for (i, count) in stats.responses_by_connector.iter().enumerate() {
                info!("Connector {}: {} responses", i, count);
            }
        }
        
        info!("DB Connector test completed successfully!");
        Ok(())
    }
    
    /// Create progress bars to track a burst of requests sent to the Gateway.
    ///
    /// - One is used to track when requests are received by the Gateway (tx receipt was received).
    /// - The other is used to track when responses are received by the Gateway (response event was
    ///   catched).
    fn init_progress_bars(
        &self,
        progress_tracker: &MultiProgress,
        burst_index: usize,
    ) -> anyhow::Result<(ProgressBar, ProgressBar)> {
        let style = ProgressStyle::with_template(
            "{prefix:32} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )?
        .progress_chars("##-");
        let prefix = format!("Burst #{burst_index}");

        let requests_pb = progress_tracker.add(
            ProgressBar::new(self.config.parallel_requests.unwrap_or(1).into())
                .with_prefix(prefix.clone())
                .with_message("Sending requests...")
                .with_style(style.clone()),
        );
        let responses_pb = progress_tracker.insert_after(
            &requests_pb,
            ProgressBar::new(self.config.parallel_requests.unwrap_or(1).into())
                .with_prefix(prefix)
                .with_message("Waiting responses...")
                .with_style(style),
        );

        Ok((requests_pb, responses_pb))
    }
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();
