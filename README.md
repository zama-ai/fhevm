# How to run the new fhEVM

THis documentation is only temporary, it allows to run the last fhEVM with the fhe keys generated using kms-core dedicated binary. 

 # Key generation

 Please update `KEY_GEN` value in `.env`

| KEY_GEN | Purpose                                                                       |
|---------|-------------------------------------------------------------------------------|
| true    | FHE keys are generated on the fly in res/keys (requires at elast 15GB of RAM) |
| false   | FHE keys are copied from kms-service-dev image in res/keys                    |




# Fast run and test

Running a single async decrypt test for uint8 + a non trivial test

```bash
# Init node and copy or gen fhe keys
make init-ethermint-node
# Run fhEVM + full KMS components
make run-full
# Deploy ACL, Gateway ...
make prepare-e2e-test
# This test will fail (first event catch is buggy - we are on it)
make run-async-test
# This one is working
make run-async-test
# A non trivial test
make run-true-input-async-test
# Manual test
cd work_dir/fhevm & npx hardhat test --grep 'test async decrypt uint32'
```



<details><summary>Docker logs</summary>
<p>

```bash
# Check logs for Gateway
docker logs zama-kms-gateway-1 -f 

# On the second try you should see

# 2024-06-27T16:59:35.432399Z  INFO gateway::events::manager: ⭐ event_decryption: 1
# 2024-06-27T16:59:35.432410Z  INFO gateway::events::manager: Handled event decryption: 1
# 2024-06-27T16:59:35.432460Z  INFO gateway::blockchain::ciphertext_provider: Getting ciphertext for ct_handle: "aa9f8f90ebf0fa8e30caee92f0b97e158f1ec659b363101d07beac9b0cc90200"
# 2024-06-27T16:59:35.436144Z  INFO gateway::blockchain::handlers: 🚀 request_id: 1, fhe_type: euint8
# 2024-06-27T16:59:35.439802Z  INFO gateway::blockchain::kms_blockchain: 📦 Stored ciphertext, handle: 00008138b65173b5c57fc98d0fce54e5ff10635127e526144ffbe21d7099e3a1e1516574
# 2024-06-27T16:59:35.439813Z  INFO gateway::blockchain::kms_blockchain: 🍊 Decrypting ciphertext of size: 33080

# Check the logs for the node
docker logs zama-kms-validator-1 -f
```

</p>
</details>



<details><summary>Pre deployment</summary>
<p>
You should see the pre-processing steps, i.e. deployment of ACL, Gateway, KMSVerifier ...

```bash
Generating typings for: 22 artifacts in dir: types for target: ethers-v6
Successfully generated 50 typings!
Compiled 22 Solidity files successfully (evm target: cancun).
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
gatewayContractAddress written to gateway/.env.gateway successfully!
gateway/lib/PredeployAddress.sol file has been generated successfully.
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
ACL address 0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92 written successfully!
./lib/ACLAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
TFHE Executor address 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c written successfully!
./lib/FHEVMCoprocessorAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
KMS Verifier address 0x12B064FB845C1cc05e9493856a1D637a73e944bE written successfully!
./lib/KMSVerifierAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
ACL was deployed at address: 0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
TFHEExecutor was deployed at address: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
KMSVerifier was deployed at address: 0x12B064FB845C1cc05e9493856a1D637a73e944bE
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
privateKey 717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6
ownerAddress 0x305F1F471e9baCFF2b3549F9601f9A4BEafc94e1
GatewayContract was deployed at address:  0xc8c9303Cd7F337fab769686B593B87DC3403E0ce
Account 0x97F272ccfef4026A1F3f0e0E879d514627B84E69 was succesfully added as an gateway relayer

```

</p>
</details>

<br />


> [!NOTE]  
> If you get `override the existing name orchestrator [y/N]: `, just ^C and  run `make clean-node-storage` to remove the state.

# Init fhEVM

```bash
make init-ethermint-node 
```

This will initialize and generate the fhe keys or copy then based on `KEY_GEN` value in .env.

IMPORTANT: if KEY_GEN is `false`, ensure to have 15 GB of empty ram to generate the keys.

# Run fhEVM + KMS components

```bash
make run-full
# Check the logs for the node
docker logs zama-kms-validator-1 -f
# Check logs for Gateway
docker logs zama-kms-gateway-1 -f     
```

```
zama-kms-gateway-1		            ghcr.io/zama-ai/kms-blockchain-gateway-dev:aa90d98
zama-kms-connector-1		        ghcr.io/zama-ai/kms-blockchain-connector-dev:50872c4
zama-kms-validator-1		        ghcr.io/zama-ai/ethermint-node:v0.5.0
zama-kms-core-1		                ghcr.io/zama-ai/kms-service-dev:aa90d98
zama-kms-kv-store-1		            ghcr.io/zama-ai/kms-blockchain-gateway-dev:aa90d98
zama-kms-blockchain-validator-1		ghcr.io/zama-ai/kms-blockchain-asc-dev:50872c4
```

# Stop fhEVM

```bash
make stop-full
```

# Fresh start

```bash
make clean
```

Note: Fhe keys are in res/keys folder, delete them to regenerate new keys at ```make init-ethermint-node```


# Test using fhevm

```bash
# if not executed before
make init-ethermint-node 
# if not executed before
make run-full
# In new terminal
make run-e2e-test
```

or in one command 

```bash
make e2e-test
```



