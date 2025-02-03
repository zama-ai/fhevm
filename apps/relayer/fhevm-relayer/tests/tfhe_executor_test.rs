#[cfg(test)]
mod integration_tests {
    use alloy::primitives::{Address, Bytes, Log as PrimitiveLog, LogData, B256, U256};
    use alloy::rpc::types::Log as RpcLog;
    use alloy_sol_types::{sol, SolEvent};

    use fhevm_relayer::event::processors::DecryptionOracleExecutor;
    use fhevm_relayer::EventRegistry;
    use std::str::FromStr;
    use std::sync::Arc;

    // Test helpers and setup
    struct TestContext {
        registry: Arc<EventRegistry>,
        contract_address: Address,
        executor: DecryptionOracleExecutor,
    }

    impl TestContext {
        fn new() -> Self {
            let contract_address =
                Address::from_str("0x67aa98a03CC4559E1e98e7b4Ed071C35c40b588d").unwrap();
            let executor = DecryptionOracleExecutor::new();
            let registry = Arc::new(EventRegistry::new());

            registry.register_contract(contract_address);
            registry.register_event(contract_address, executor.clone());

            Self {
                registry,
                contract_address,
                executor,
            }
        }

        fn create_log(
            &self,
            counter: u64,
            request_id: u64,
            ciphertexts: Vec<U256>,
            contract_caller: Address,
            callback_selector: [u8; 4],
        ) -> RpcLog {
            let mut data = Vec::new();

            // requestID
            data.extend_from_slice(&U256::from(request_id).to_be_bytes::<32>());

            // array offset
            data.extend_from_slice(&U256::from(128).to_be_bytes::<32>());

            // contract caller and callback selector
            data.extend_from_slice(&[0u8; 12]);
            data.extend_from_slice(contract_caller.as_slice());
            data.extend_from_slice(&callback_selector);
            data.extend_from_slice(&[0u8; 28]);

            // array length
            data.extend_from_slice(&U256::from(ciphertexts.len()).to_be_bytes::<32>());

            // array data
            for value in ciphertexts {
                data.extend_from_slice(&value.to_be_bytes::<32>());
            }

            RpcLog {
                inner: PrimitiveLog {
                    address: self.contract_address,
                    data: LogData::new_unchecked(
                        vec![
                            DecryptionRequest::SIGNATURE_HASH,
                            U256::from(counter).into(),
                        ],
                        Bytes::from(data),
                    ),
                },
                block_hash: Some(B256::ZERO),
                block_number: Some(1),
                block_timestamp: None,
                transaction_hash: Some(B256::ZERO),
                transaction_index: Some(0),
                log_index: Some(0),
                removed: false,
            }
        }
    }

    sol! {
        event DecryptionRequest(
            uint256 indexed counter,
            uint256 requestID,
            uint256[] cts,
            address contractCaller,
            bytes4 callbackSelector
        );
    }

    #[tokio::test]
    async fn test_basic_decryption_request() {
        let ctx = TestContext::new();
        let contract_caller =
            Address::from_str("0x3e6d37aab0d92109f8da5ad71eacb9240a9bf1e9").unwrap();

        let ciphertext = U256::from_str_radix(
            "975da42d26f90bc06a097700aea761388ab200afb31d9a07d98e5011c2000600",
            16,
        )
        .unwrap();

        let log = ctx.create_log(
            7, // counter
            0, // request_id
            vec![ciphertext],
            contract_caller,
            [0x2c, 0x14, 0x35, 0x63], // callback_selector
        );

        let topic = log.inner.data.topics()[0].to_string();
        let result = ctx
            .registry
            .process_event(ctx.contract_address, &topic, &log);
        assert!(result.is_ok(), "Basic decryption request should succeed");
    }

    // Test concurrent processing
    #[tokio::test]
    async fn test_concurrent_processing() {
        let ctx = Arc::new(TestContext::new());
        let mut handles = vec![];

        for i in 0..10 {
            let ctx = ctx.clone();
            let handle = tokio::spawn(async move {
                let contract_caller =
                    Address::from_str("0x3e6d37aab0d92109f8da5ad71eacb9240a9bf1e9").unwrap();

                let log = ctx.create_log(
                    i,
                    i,
                    vec![U256::from(i)],
                    contract_caller,
                    [0x2c, 0x14, 0x35, 0x63],
                );

                let topic = log.inner.data.topics()[0].to_string();
                ctx.registry
                    .process_event(ctx.contract_address, &topic, &log)
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok(), "Concurrent processing should succeed");
        }
    }
}
